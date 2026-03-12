use super::*;
use crate::msg::RegistryCollection;
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    ContractResult, OwnedDeps, SystemError, SystemResult, Timestamp, WasmQuery,
};

fn mock_queries(
    deps: &mut OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
    >,
    whitelist_addr: &'static str,
    registry_addr: &'static str,
    whitelist_active: bool,
    has_member: bool,
    collection_registered: bool,
    minter_authorized: bool,
) {
    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart { contract_addr, msg } if contract_addr == whitelist_addr => {
            let parsed: WhitelistQueryMsg = from_json(msg).unwrap();
            let bin = match parsed {
                WhitelistQueryMsg::Config {} => to_json_binary(&WhitelistConfigResponse {
                    per_address_limit: 1,
                    member_limit: 10,
                    start_time: Timestamp::from_seconds(100),
                    end_time: Timestamp::from_seconds(200),
                    unit_price: Coin::new(50u128, "upasg"),
                    is_active: whitelist_active,
                })
                .unwrap(),
                WhitelistQueryMsg::HasMember { .. } => {
                    to_json_binary(&HasMemberResponse { has_member }).unwrap()
                }
            };

            SystemResult::Ok(ContractResult::Ok(bin))
        }
        WasmQuery::Smart { contract_addr, msg } if contract_addr == registry_addr => {
            let parsed: RegistryQueryMsg = from_json(msg).unwrap();
            let bin = match parsed {
                RegistryQueryMsg::Collection { .. } => {
                    to_json_binary(&RegistryCollectionResponse {
                        collection: collection_registered.then(|| RegistryCollection {
                            creator: "creator".to_string(),
                        }),
                    })
                    .unwrap()
                }
                RegistryQueryMsg::CanMintCollection { .. } => {
                    to_json_binary(&RegistryApprovalStatusResponse { approved: true }).unwrap()
                }
                RegistryQueryMsg::IsMinterAuthorized { .. } => {
                    to_json_binary(&RegistryMinterAuthorizedResponse {
                        is_authorized: minter_authorized,
                    })
                    .unwrap()
                }
            };

            SystemResult::Ok(ContractResult::Ok(bin))
        }
        WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: "unknown".to_string(),
        }),
        _ => SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "unsupported wasm query".to_string(),
        }),
    });
}

fn sample_config() -> Config {
    Config {
        admin: Addr::unchecked("admin"),
        cw721_address: Addr::unchecked("collection"),
        cw721_code_id: 1,
        base_token_uri: "ipfs://base".to_string(),
        num_tokens: 10,
        unit_price: Coin::new(100u128, "upasg"),
        per_address_limit: 5,
        start_time: Timestamp::from_seconds(1_000),
        whitelist: Some(Addr::unchecked("whitelist")),
        registry: None,
        paused: false,
    }
}

#[test]
fn whitelist_context_applies_before_public_start() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("buyer");
    let config = sample_config();

    MINTABLE_NUM_TOKENS
        .save(deps.as_mut().storage, &10)
        .unwrap();
    mock_queries(&mut deps, "whitelist", "registry", true, true, true, true);

    let ctx =
        resolve_mint_context(deps.as_ref(), &env, &sender, &config, &env.contract.address).unwrap();

    assert!(ctx.is_whitelist);
    assert_eq!(ctx.price, Coin::new(50u128, "upasg"));
    assert_eq!(ctx.per_address_limit, 1);
}

#[test]
fn active_whitelist_requires_membership() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("buyer");
    let config = sample_config();

    MINTABLE_NUM_TOKENS
        .save(deps.as_mut().storage, &10)
        .unwrap();
    mock_queries(&mut deps, "whitelist", "registry", true, false, true, true);

    let err = resolve_mint_context(deps.as_ref(), &env, &sender, &config, &env.contract.address)
        .unwrap_err();

    assert_eq!(err, ContractError::NotWhitelisted {});
}

#[test]
fn registry_authorization_is_enforced_when_configured() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("buyer");
    let mut config = sample_config();
    config.whitelist = None;
    config.registry = Some(Addr::unchecked("registry"));
    config.start_time = Timestamp::from_seconds(0);

    MINTABLE_NUM_TOKENS
        .save(deps.as_mut().storage, &10)
        .unwrap();
    mock_queries(&mut deps, "whitelist", "registry", false, true, true, false);

    let err = resolve_mint_context(deps.as_ref(), &env, &sender, &config, &env.contract.address)
        .unwrap_err();

    assert_eq!(err, ContractError::MinterNotAuthorized {});
}
