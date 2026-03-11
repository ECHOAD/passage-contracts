use super::*;
use crate::msg::{
    Approval, CollectionInfoResponse, Cw721QueryMsg, Expiration, OwnerOfResponse, Pg721QueryMsg,
    RoyaltyInfoResponse, SplitRouterExecuteMsg,
};
use cosmwasm_std::{
    from_json,
    testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
    to_json_binary, Coin, ContractResult, OwnedDeps, SystemError, SystemResult, WasmQuery,
};

fn base_config() -> Config {
    let api = MockApi::default();
    Config {
        admin: api.addr_make("admin"),
        denom: "upasg".to_string(),
        min_price: Uint128::new(100),
        trading_fee_bps: 250,
        max_trading_fee_bps: 1000,
        fee_collector: api.addr_make("treasury"),
        registry: None,
        split_router: Some(api.addr_make("split-router")),
        use_split_router: true,
        min_bid_increment_percent: Decimal::percent(5),
        min_duration: 60,
        max_duration: 3_600,
        extend_duration: 120,
        paused: false,
        require_registration: false,
    }
}

fn mock_collection_queries(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>,
    collection: &str,
    owner: &str,
    approved_spender: Option<&str>,
    royalty_recipient: Option<&str>,
    royalty_share: Option<&str>,
) {
    let collection = collection.to_string();
    let owner = owner.to_string();
    let approved_spender = approved_spender.map(str::to_string);
    let royalty_recipient = royalty_recipient.map(str::to_string);
    let royalty_share = royalty_share.map(str::to_string);
    let creator = MockApi::default().addr_make("creator").to_string();

    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart { contract_addr, msg } if contract_addr == &collection => {
            if let Ok(parsed) = from_json::<Cw721QueryMsg>(msg) {
                match parsed {
                    Cw721QueryMsg::OwnerOf { .. } => {
                        let approvals = approved_spender
                            .clone()
                            .into_iter()
                            .map(|spender| Approval {
                                spender,
                                expires: Expiration::Never {},
                            })
                            .collect::<Vec<_>>();
                        return SystemResult::Ok(ContractResult::Ok(
                            to_json_binary(&OwnerOfResponse {
                                owner: owner.clone(),
                                approvals,
                            })
                            .unwrap(),
                        ));
                    }
                }
            }

            if let Ok(parsed) = from_json::<Pg721QueryMsg>(msg) {
                match parsed {
                    Pg721QueryMsg::CollectionInfo {} => {
                        let royalty_info = royalty_recipient
                            .clone()
                            .zip(royalty_share.clone())
                            .map(|(payment_address, share)| RoyaltyInfoResponse {
                                payment_address,
                                share,
                            });
                        return SystemResult::Ok(ContractResult::Ok(
                            to_json_binary(&CollectionInfoResponse {
                                creator: creator.clone(),
                                description: "desc".to_string(),
                                image: "image".to_string(),
                                external_link: None,
                                royalty_info,
                            })
                            .unwrap(),
                        ));
                    }
                }
            }

            SystemResult::Err(SystemError::InvalidRequest {
                error: "unexpected collection query".to_string(),
                request: msg.clone(),
            })
        }
        WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: "unknown".to_string(),
        }),
        _ => SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "unsupported wasm query".to_string(),
        }),
    });
}

#[test]
fn create_auction_moves_nft_into_contract_custody() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let seller = deps.api.addr_make("seller");
    let collection = deps.api.addr_make("collection");

    CONFIG.save(deps.as_mut().storage, &base_config()).unwrap();
    mock_collection_queries(
        &mut deps,
        collection.as_str(),
        seller.as_str(),
        Some(env.contract.address.as_str()),
        None,
        None,
    );

    let response = execute_create_auction(
        deps.as_mut(),
        env.clone(),
        message_info(&seller, &[]),
        collection.to_string(),
        "1".to_string(),
        Coin::new(500u128, "upasg"),
        300,
        None,
    )
    .unwrap();

    assert!(auctions().has(deps.as_ref().storage, (collection.clone(), "1".to_string())));
    assert_eq!(response.messages.len(), 1);
    match &response.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, collection.as_str());
        }
        _ => panic!("expected nft transfer message"),
    }
}

#[test]
fn first_bid_starts_auction_and_sets_end_time() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection = deps.api.addr_make("collection");
    let bidder = deps.api.addr_make("bidder");
    let seller = deps.api.addr_make("seller");

    CONFIG.save(deps.as_mut().storage, &base_config()).unwrap();
    auctions()
        .save(
            deps.as_mut().storage,
            (collection.clone(), "1".to_string()),
            &Auction {
                collection: collection.clone(),
                token_id: "1".to_string(),
                seller,
                reserve_price: Coin::new(500u128, "upasg"),
                duration: 300,
                seller_funds_recipient: None,
                high_bid: None,
                first_bid_time: None,
                end_time: None,
                created_at: env.block.time.seconds(),
            },
        )
        .unwrap();

    let response = execute_place_bid(
        deps.as_mut(),
        env.clone(),
        message_info(&bidder, &[Coin::new(500u128, "upasg")]),
        collection.to_string(),
        "1".to_string(),
    )
    .unwrap();

    let auction = auctions()
        .load(deps.as_ref().storage, (collection, "1".to_string()))
        .unwrap();
    assert_eq!(auction.first_bid_time, Some(env.block.time));
    assert_eq!(auction.end_time, Some(env.block.time.plus_seconds(300)));
    assert_eq!(auction.high_bid.unwrap().bidder, bidder);
    assert_eq!(
        response
            .attributes
            .iter()
            .find(|attr| attr.key == "first_bid")
            .unwrap()
            .value,
        "true"
    );
}

#[test]
fn settle_auction_routes_trading_fee_seller_and_royalty() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    let collection = deps.api.addr_make("collection");
    let seller = deps.api.addr_make("seller");
    let bidder = deps.api.addr_make("bidder");
    let royalty_recipient = deps.api.addr_make("creator-wallet");
    let seller_recipient = deps.api.addr_make("seller-recipient");
    let fee_collector = deps.api.addr_make("treasury");
    let split_router = deps.api.addr_make("split-router");
    let caller = deps.api.addr_make("anyone");

    CONFIG.save(deps.as_mut().storage, &base_config()).unwrap();
    mock_collection_queries(
        &mut deps,
        collection.as_str(),
        seller.as_str(),
        None,
        Some(royalty_recipient.as_str()),
        Some("0.1"),
    );

    let ended_at = env.block.time.minus_seconds(1);
    auctions()
        .save(
            deps.as_mut().storage,
            (collection.clone(), "7".to_string()),
            &Auction {
                collection: collection.clone(),
                token_id: "7".to_string(),
                seller: seller.clone(),
                reserve_price: Coin::new(500u128, "upasg"),
                duration: 300,
                seller_funds_recipient: Some(seller_recipient.clone()),
                high_bid: Some(HighBid {
                    bidder: bidder.clone(),
                    coin: Coin::new(1_000u128, "upasg"),
                    placed_at: ended_at.minus_seconds(10),
                }),
                first_bid_time: Some(ended_at.minus_seconds(300)),
                end_time: Some(ended_at),
                created_at: ended_at.minus_seconds(400).seconds(),
            },
        )
        .unwrap();

    env.block.time = env.block.time.plus_seconds(1);

    let response = execute_settle_auction(
        deps.as_mut(),
        env,
        message_info(&caller, &[]),
        collection.to_string(),
        "7".to_string(),
    )
    .unwrap();

    assert!(!auctions().has(deps.as_ref().storage, (collection.clone(), "7".to_string())));
    assert_eq!(response.messages.len(), 4);

    match &response.messages[0].msg {
        CosmosMsg::Bank(cosmwasm_std::BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, fee_collector.as_str());
            assert_eq!(amount[0].amount, Uint128::new(25));
        }
        _ => panic!("expected trading fee bank send"),
    }

    match &response.messages[1].msg {
        CosmosMsg::Bank(cosmwasm_std::BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, seller_recipient.as_str());
            assert_eq!(amount[0].amount, Uint128::new(875));
        }
        _ => panic!("expected seller payout bank send"),
    }

    match &response.messages[2].msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        }) => {
            assert_eq!(contract_addr, split_router.as_str());
            let parsed: SplitRouterExecuteMsg = from_json(msg).unwrap();
            assert_eq!(
                parsed,
                SplitRouterExecuteMsg::Split {
                    key: collection.to_string(),
                }
            );
            assert_eq!(funds[0].amount, Uint128::new(100));
        }
        _ => panic!("expected split router royalty routing"),
    }

    match &response.messages[3].msg {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, collection.as_str());
        }
        _ => panic!("expected nft transfer"),
    }
}
