use super::*;
use crate::msg::RoyaltyInfoResponse;
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
) {
    let contract_addr = contract_addr.to_string();
    let royalty_recipient = royalty_recipient.to_string();
    let royalty_share = royalty_share.to_string();

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
        WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: "unknown".to_string(),
        }),
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
        split_router: None,
        use_split_router: false,
        operators: vec![],
        paused: false,
        require_registration: false,
    };
    let royalty_recipient = deps.api.addr_make("royalty-wallet");

    mock_collection_info(&mut deps, "collection", royalty_recipient.as_str(), "0.1");

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
