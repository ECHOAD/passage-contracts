use crate::{
    error::ContractError,
    helpers::{
        build_bank_send_msg, build_transfer_nft_msg, ensure_owner_and_approval,
        query_royalty_payout, validate_bid_funds, validate_collection_registration,
        validate_config, validate_duration, validate_reserve_price,
    },
    msg::{ExecuteMsg, InstantiateMsg, SplitRouterExecuteMsg},
    state::{auctions, Auction, Config, HighBid, CONFIG},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response,
    StdError, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::nonpayable;

const CONTRACT_NAME: &str = "crates.io:passage-auction-english";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_MAX_TRADING_FEE_BPS: u64 = 1_000;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg
        .admin
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());
    let registry = msg
        .registry
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;
    let split_router = msg
        .split_router
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;
    let max_trading_fee_bps = msg
        .max_trading_fee_bps
        .unwrap_or(DEFAULT_MAX_TRADING_FEE_BPS);
    let config = Config {
        admin,
        denom: msg.denom,
        min_price: msg.min_price,
        trading_fee_bps: msg.trading_fee_bps,
        max_trading_fee_bps,
        fee_collector: deps.api.addr_validate(&msg.fee_collector)?,
        registry,
        split_router: split_router.clone(),
        use_split_router: msg.use_split_router.unwrap_or(split_router.is_some()),
        min_bid_increment_percent: msg.min_bid_increment_percent,
        min_duration: msg.min_duration,
        max_duration: msg.max_duration,
        extend_duration: msg.extend_duration,
        paused: false,
        require_registration: msg.require_registration.unwrap_or(true),
    };

    validate_config(&config)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "auction-english")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.paused && config.admin != info.sender {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        ExecuteMsg::UpdateConfig {
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            split_router,
            use_split_router,
            min_bid_increment_percent,
            min_duration,
            max_duration,
            extend_duration,
            paused,
            require_registration,
        } => execute_update_config(
            deps,
            info,
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            split_router,
            use_split_router,
            min_bid_increment_percent,
            min_duration,
            max_duration,
            extend_duration,
            paused,
            require_registration,
        ),
        ExecuteMsg::CreateAuction {
            collection,
            token_id,
            reserve_price,
            duration,
            seller_funds_recipient,
        } => execute_create_auction(
            deps,
            env,
            info,
            collection,
            token_id,
            reserve_price,
            duration,
            seller_funds_recipient,
        ),
        ExecuteMsg::UpdateReservePrice {
            collection,
            token_id,
            reserve_price,
        } => execute_update_reserve_price(deps, info, collection, token_id, reserve_price),
        ExecuteMsg::CancelAuction {
            collection,
            token_id,
        } => execute_cancel_auction(deps, info, collection, token_id),
        ExecuteMsg::PlaceBid {
            collection,
            token_id,
        } => execute_place_bid(deps, env, info, collection, token_id),
        ExecuteMsg::SettleAuction {
            collection,
            token_id,
        } => execute_settle_auction(deps, env, info, collection, token_id),
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    denom: Option<String>,
    min_price: Option<Uint128>,
    trading_fee_bps: Option<u64>,
    max_trading_fee_bps: Option<u64>,
    fee_collector: Option<String>,
    registry: Option<String>,
    split_router: Option<String>,
    use_split_router: Option<bool>,
    min_bid_increment_percent: Option<Decimal>,
    min_duration: Option<u64>,
    max_duration: Option<u64>,
    extend_duration: Option<u64>,
    paused: Option<bool>,
    require_registration: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }
    if let Some(new_denom) = denom {
        config.denom = new_denom;
    }
    if let Some(new_min_price) = min_price {
        config.min_price = new_min_price;
    }
    if let Some(new_trading_fee_bps) = trading_fee_bps {
        config.trading_fee_bps = new_trading_fee_bps;
    }
    if let Some(new_max_trading_fee_bps) = max_trading_fee_bps {
        config.max_trading_fee_bps = new_max_trading_fee_bps;
    }
    if let Some(new_fee_collector) = fee_collector {
        config.fee_collector = deps.api.addr_validate(&new_fee_collector)?;
    }
    if let Some(new_registry) = registry {
        config.registry = Some(deps.api.addr_validate(&new_registry)?);
    }
    if let Some(new_split_router) = split_router {
        config.split_router = Some(deps.api.addr_validate(&new_split_router)?);
    }
    if let Some(new_use_split_router) = use_split_router {
        config.use_split_router = new_use_split_router;
    }
    if let Some(new_increment) = min_bid_increment_percent {
        config.min_bid_increment_percent = new_increment;
    }
    if let Some(new_min_duration) = min_duration {
        config.min_duration = new_min_duration;
    }
    if let Some(new_max_duration) = max_duration {
        config.max_duration = new_max_duration;
    }
    if let Some(new_extend_duration) = extend_duration {
        config.extend_duration = new_extend_duration;
    }
    if let Some(new_paused) = paused {
        config.paused = new_paused;
    }
    if let Some(new_require_registration) = require_registration {
        config.require_registration = new_require_registration;
    }

    validate_config(&config)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn execute_create_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
    reserve_price: Coin,
    duration: u64,
    seller_funds_recipient: Option<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    validate_collection_registration(deps.as_ref(), &config, &collection_addr)?;
    validate_reserve_price(&config, &reserve_price)?;
    validate_duration(&config, duration)?;
    ensure_owner_and_approval(
        deps.as_ref(),
        &env,
        &info.sender,
        &collection_addr,
        &token_id,
    )?;

    if auctions().has(deps.storage, (collection_addr.clone(), token_id.clone())) {
        return Err(ContractError::AuctionAlreadyExists {
            collection,
            token_id,
        });
    }

    let auction = Auction {
        collection: collection_addr.clone(),
        token_id: token_id.clone(),
        seller: info.sender.clone(),
        reserve_price: reserve_price.clone(),
        duration,
        seller_funds_recipient: seller_funds_recipient
            .map(|addr| deps.api.addr_validate(&addr))
            .transpose()?,
        high_bid: None,
        first_bid_time: None,
        end_time: None,
        created_at: env.block.time.seconds(),
    };

    auctions().save(
        deps.storage,
        (collection_addr.clone(), token_id.clone()),
        &auction,
    )?;

    Ok(Response::new()
        .add_message(build_transfer_nft_msg(
            &collection_addr,
            &token_id,
            &env.contract.address,
        )?)
        .add_attribute("action", "create_auction")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("seller", info.sender)
        .add_attribute("reserve_price", reserve_price.to_string())
        .add_attribute("duration", duration.to_string()))
}

fn execute_update_reserve_price(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
    token_id: String,
    reserve_price: Coin,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    validate_reserve_price(&config, &reserve_price)?;

    let mut auction = auctions()
        .load(deps.storage, (collection_addr.clone(), token_id.clone()))
        .map_err(|_| ContractError::AuctionNotFound {
            collection: collection.clone(),
            token_id: token_id.clone(),
        })?;

    if auction.seller != info.sender {
        return Err(ContractError::NotSeller {});
    }

    if auction.first_bid_time.is_some() {
        return Err(ContractError::AuctionStarted {});
    }

    auction.reserve_price = reserve_price.clone();
    auctions().save(
        deps.storage,
        (collection_addr.clone(), token_id.clone()),
        &auction,
    )?;

    Ok(Response::new()
        .add_attribute("action", "update_reserve_price")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("reserve_price", reserve_price.to_string()))
}

fn execute_cancel_auction(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let collection_addr = deps.api.addr_validate(&collection)?;
    let auction = auctions()
        .load(deps.storage, (collection_addr.clone(), token_id.clone()))
        .map_err(|_| ContractError::AuctionNotFound {
            collection: collection.clone(),
            token_id: token_id.clone(),
        })?;

    if auction.seller != info.sender {
        return Err(ContractError::NotSeller {});
    }

    if auction.first_bid_time.is_some() {
        return Err(ContractError::AuctionStarted {});
    }

    auctions().remove(deps.storage, (collection_addr.clone(), token_id.clone()))?;

    Ok(Response::new()
        .add_message(build_transfer_nft_msg(
            &collection_addr,
            &token_id,
            &auction.seller,
        )?)
        .add_attribute("action", "cancel_auction")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id))
}

fn execute_place_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    let mut auction = auctions()
        .load(deps.storage, (collection_addr.clone(), token_id.clone()))
        .map_err(|_| ContractError::AuctionNotFound {
            collection: collection.clone(),
            token_id: token_id.clone(),
        })?;

    if auction.seller == info.sender {
        return Err(ContractError::SellerCannotBid {});
    }

    if let Some(end_time) = auction.end_time {
        if env.block.time >= end_time {
            return Err(ContractError::AuctionEnded {});
        }
    }

    let bid_amount = validate_bid_funds(&info, &config.denom)?;
    let min_bid = auction.min_bid_coin(config.min_bid_increment_percent);

    if bid_amount < min_bid.amount {
        return Err(ContractError::BidTooLow {
            min_bid: min_bid.to_string(),
        });
    }

    let mut response = Response::new()
        .add_attribute("action", "place_bid")
        .add_attribute("collection", collection_addr.to_string())
        .add_attribute("token_id", token_id.clone())
        .add_attribute("bidder", info.sender.to_string())
        .add_attribute("bid_amount", bid_amount.to_string());

    if let Some(previous_bid) = auction.high_bid.take() {
        response = response
            .add_message(build_bank_send_msg(
                &previous_bid.bidder,
                &previous_bid.coin.denom,
                previous_bid.coin.amount,
            ))
            .add_attribute("previous_bidder", previous_bid.bidder.to_string())
            .add_attribute("previous_bid_amount", previous_bid.coin.amount.to_string());

        if let Some(end_time) = auction.end_time {
            let time_remaining = end_time.seconds().saturating_sub(env.block.time.seconds());
            if time_remaining < config.extend_duration {
                auction.end_time = Some(env.block.time.plus_seconds(config.extend_duration));
            }
        }
    } else {
        auction.first_bid_time = Some(env.block.time);
        auction.end_time = Some(env.block.time.plus_seconds(auction.duration));
        response = response.add_attribute("first_bid", "true");
    }

    let final_end_time = auction
        .end_time
        .expect("end time must be set after first bid");
    auction.high_bid = Some(HighBid {
        bidder: info.sender,
        coin: Coin {
            denom: config.denom.clone(),
            amount: bid_amount,
        },
        placed_at: env.block.time,
    });

    auctions().save(deps.storage, (collection_addr, token_id), &auction)?;

    Ok(response.add_attribute("auction_end_time", final_end_time.to_string()))
}

fn execute_settle_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    let auction = auctions()
        .load(deps.storage, (collection_addr.clone(), token_id.clone()))
        .map_err(|_| ContractError::AuctionNotFound {
            collection: collection.clone(),
            token_id: token_id.clone(),
        })?;

    let Some(end_time) = auction.end_time else {
        return Err(ContractError::AuctionNotEnded {});
    };
    if env.block.time < end_time {
        return Err(ContractError::AuctionNotEnded {});
    }

    let Some(high_bid) = auction.high_bid.clone() else {
        return Err(ContractError::NoBidPlaced {});
    };

    auctions().remove(deps.storage, (collection_addr.clone(), token_id.clone()))?;

    let mut messages: Vec<CosmosMsg> = vec![];
    let trading_fee = high_bid
        .coin
        .amount
        .multiply_ratio(config.trading_fee_bps as u128, 10_000u128);
    let royalty_payout =
        query_royalty_payout(deps.as_ref(), &collection_addr, high_bid.coin.amount)?;
    let royalty_amount = royalty_payout
        .as_ref()
        .map(|payout| payout.amount)
        .unwrap_or_else(Uint128::zero);
    let seller_amount = high_bid
        .coin
        .amount
        .checked_sub(trading_fee)
        .map_err(StdError::overflow)?
        .checked_sub(royalty_amount)
        .map_err(StdError::overflow)?;

    if !trading_fee.is_zero() {
        messages.push(build_bank_send_msg(
            &config.fee_collector,
            &config.denom,
            trading_fee,
        ));
    }

    if !seller_amount.is_zero() {
        messages.push(build_bank_send_msg(
            &auction.funds_recipient(),
            &config.denom,
            seller_amount,
        ));
    }

    if let Some(royalty_payout) = royalty_payout {
        if !royalty_payout.amount.is_zero() {
            if config.use_split_router {
                let Some(router) = &config.split_router else {
                    return Err(ContractError::SplitRouterNotConfigured {});
                };
                messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: router.to_string(),
                    msg: to_json_binary(&SplitRouterExecuteMsg::RouteAuctionRoyalty {
                        collection: collection_addr.to_string(),
                    })?,
                    funds: vec![Coin {
                        denom: config.denom.clone(),
                        amount: royalty_payout.amount,
                    }],
                }));
            } else {
                messages.push(build_bank_send_msg(
                    &royalty_payout.recipient,
                    &config.denom,
                    royalty_payout.amount,
                ));
            }
        }
    }

    messages.push(build_transfer_nft_msg(
        &collection_addr,
        &token_id,
        &high_bid.bidder,
    )?);

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "settle_auction")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("seller", auction.seller)
        .add_attribute("bidder", high_bid.bidder)
        .add_attribute("sale_price", high_bid.coin.to_string())
        .add_attribute("trading_fee", trading_fee.to_string())
        .add_attribute("royalty", royalty_amount.to_string())
        .add_attribute("seller_amount", seller_amount.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{
        Approval, CollectionInfoResponse, Cw721QueryMsg, Expiration, OwnerOfResponse,
        Pg721QueryMsg, RoyaltyInfoResponse, SplitRouterExecuteMsg,
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
                    SplitRouterExecuteMsg::RouteAuctionRoyalty {
                        collection: collection.to_string(),
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
}
