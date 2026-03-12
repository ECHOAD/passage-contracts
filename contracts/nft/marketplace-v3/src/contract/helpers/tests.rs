use super::*;
use crate::msg::{RoyaltyInfoResponse, SplitRouterExecuteMsg};
use cosmwasm_std::ContractInfoResponse;
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    ContractResult, OwnedDeps, SystemError, SystemResult, WasmQuery,
};

fn mock_collection_info(
    deps: &mut OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
    >,
    contract_addr: &str,
    royalty_recipient: &str,
    royalty_share: &str,
    royalty_is_contract: bool,
) {
    let contract_addr = contract_addr.to_string();
    let royalty_recipient = royalty_recipient.to_string();
    let royalty_share = royalty_share.to_string();
    let contract_recipient = royalty_recipient.clone();

    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart {
            contract_addr: addr,
            msg,
        } if addr == &contract_addr => {
            let parsed: Pg721QueryMsg = from_json(msg).unwrap();
            match parsed {
                Pg721QueryMsg::CollectionInfo {} => SystemResult::Ok(ContractResult::Ok(
                    to_json_binary(&CollectionInfoResponse {
                        creator: "creator".to_string(),
                        description: "desc".to_string(),
                        image: "ipfs://image".to_string(),
                        external_link: None,
                        royalty_info: Some(RoyaltyInfoResponse {
                            payment_address: royalty_recipient.clone(),
                            share: royalty_share.clone(),
                        }),
                    })
                    .unwrap(),
                )),
            }
        }
        WasmQuery::ContractInfo { contract_addr }
            if royalty_is_contract && contract_addr == &contract_recipient =>
        {
            SystemResult::Ok(ContractResult::Ok(
                to_json_binary(&ContractInfoResponse::new(
                    1,
                    Addr::unchecked("creator"),
                    None::<Addr>,
                    false,
                    None::<String>,
                ))
                .unwrap(),
            ))
        }
        WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: "unknown".to_string(),
        }),
        WasmQuery::ContractInfo { contract_addr } => {
            SystemResult::Err(SystemError::NoSuchContract {
                addr: contract_addr.clone(),
            })
        }
        _ => SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "unsupported wasm query".to_string(),
        }),
    });
}

#[test]
fn legacy_sale_sends_royalty_payment() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection = Addr::unchecked("collection");
    let buyer = Addr::unchecked("buyer");
    let seller = Addr::unchecked("seller");
    let recipient = Addr::unchecked("seller_payout");
    let config = Config {
        admin: Addr::unchecked("admin"),
        denom: "upasg".to_string(),
        min_price: Uint128::new(1),
        trading_fee_bps: 250,
        max_trading_fee_bps: 1000,
        fee_collector: Addr::unchecked("treasury"),
        registry: None,
        operators: vec![],
        paused: false,
        require_registration: false,
    };
    let royalty_recipient = deps.api.addr_make("royalty-wallet");

    mock_collection_info(
        &mut deps,
        "collection",
        royalty_recipient.as_str(),
        "0.1",
        false,
    );

    let (messages, sale_info) = execute_sale(
        &deps.as_mut(),
        &env,
        &config,
        &collection,
        "1",
        &seller,
        &buyer,
        &recipient,
        "upasg",
        Uint128::new(1_000),
    )
    .unwrap();

    assert_eq!(sale_info.trading_fee, Uint128::new(25));
    assert_eq!(sale_info.royalty, Uint128::new(100));
    assert_eq!(messages.len(), 4);

    let royalty_msg = &messages[2];
    match royalty_msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, royalty_recipient.as_str());
            assert_eq!(amount[0].amount, Uint128::new(100));
        }
        _ => panic!("expected royalty bank send"),
    }
}

#[test]
fn split_mode_routes_royalty_to_collection_splitter() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection = Addr::unchecked("collection");
    let buyer = Addr::unchecked("buyer");
    let seller = Addr::unchecked("seller");
    let recipient = Addr::unchecked("seller_payout");
    let config = Config {
        admin: Addr::unchecked("admin"),
        denom: "upasg".to_string(),
        min_price: Uint128::new(1),
        trading_fee_bps: 250,
        max_trading_fee_bps: 1000,
        fee_collector: Addr::unchecked("treasury"),
        registry: None,
        operators: vec![],
        paused: false,
        require_registration: false,
    };
    let split_contract = deps.api.addr_make("royalty-split");

    mock_collection_info(
        &mut deps,
        "collection",
        split_contract.as_str(),
        "0.1",
        true,
    );

    let (messages, sale_info) = execute_sale(
        &deps.as_mut(),
        &env,
        &config,
        &collection,
        "1",
        &seller,
        &buyer,
        &recipient,
        "upasg",
        Uint128::new(1_000),
    )
    .unwrap();

    assert_eq!(sale_info.trading_fee, Uint128::new(25));
    assert_eq!(sale_info.royalty, Uint128::new(100));
    assert_eq!(messages.len(), 4);

    let royalty_msg = &messages[2];
    match royalty_msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        }) => {
            assert_eq!(contract_addr, split_contract.as_str());
            let parsed: SplitRouterExecuteMsg = from_json(msg).unwrap();
            assert_eq!(parsed, SplitRouterExecuteMsg::Split {});
            assert_eq!(funds, &vec![Coin::new(100u128, "upasg")]);
        }
        _ => panic!("expected split execute"),
    }
}
