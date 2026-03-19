use cosmwasm_std::{
    coins, from_json,
    testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
    to_json_binary, Addr, BankMsg, ContractResult, CosmosMsg, OwnedDeps, SystemError, SystemResult,
    Timestamp, Uint128, WasmMsg, WasmQuery,
};

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
    msg::{
        ConfigResponse, ConversionRateResponse, Cw721QueryMsg, ExecuteMsg, InstantiateMsg,
        OwnerOfResponse, PasgBusinessBoundary, PasgCompatibilityShimKind, PasgSettlementKind,
        PasgUtilityExecuteRoute, PasgUtilityQueryRoute, PasgUtilityResponse, QueryMsg,
        RegistryCollection, RegistryCollectionResponse, RegistryQueryMsg, SessionStatus,
        UserBalanceResponse,
    },
    state::{
        Config, PendingRevenue, PlatformStats, StreamingSession, UserBalance, WorldConfig, CONFIG,
        PENDING_REVENUE, PLATFORM_STATS, SESSIONS, SESSION_COUNTER, USER_BALANCES, USER_SESSIONS,
        WORLD_CONFIGS,
    },
};

type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

const ADMIN: &str = "admin";
const BACKEND: &str = "backend";
const FIAT_ORACLE: &str = "fiat-oracle";
const REGISTRY: &str = "registry";
const SPLIT_ROUTER: &str = "split-router";
const WORLD_COLLECTION: &str = "world-collection";
const OTHER_COLLECTION: &str = "other-collection";
const WORLD_ID: &str = "world-1";
const USER: &str = "user";

fn addr(label: &str) -> Addr {
    MockApi::default().addr_make(label)
}

fn env_at(seconds: u64) -> cosmwasm_std::Env {
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(seconds);
    env
}

fn instantiate_contract(
    deps: &mut TestDeps,
    backend_operator: Option<&str>,
    fiat_oracle: Option<&str>,
) {
    instantiate_contract_with_denom(deps, backend_operator, fiat_oracle, "upasg");
}

fn instantiate_contract_with_denom(
    deps: &mut TestDeps,
    backend_operator: Option<&str>,
    fiat_oracle: Option<&str>,
    denom: &str,
) {
    let msg = InstantiateMsg {
        admin: addr(ADMIN).to_string(),
        split_router: addr(SPLIT_ROUTER).to_string(),
        registry: addr(REGISTRY).to_string(),
        backend_operator: backend_operator.map(|value| addr(value).to_string()),
        denom: denom.to_string(),
        points_per_denom: Uint128::new(100),
        fiat_oracle: fiat_oracle.map(|value| addr(value).to_string()),
        stripe_webhook_validator: None,
    };

    instantiate(
        deps.as_mut(),
        mock_env(),
        message_info(&addr("deployer"), &[]),
        msg,
    )
    .unwrap();
}

fn seed_pending_revenue(
    deps: &mut TestDeps,
    world_nft_id: &str,
    pending_points: u128,
    pending_pasg: u128,
    last_distribution: Option<Timestamp>,
) {
    PENDING_REVENUE
        .save(
            deps.as_mut().storage,
            world_nft_id,
            &PendingRevenue {
                world_nft_id: world_nft_id.to_string(),
                pending_points: Uint128::new(pending_points),
                pending_pasg: Uint128::new(pending_pasg),
                last_distribution,
            },
        )
        .unwrap();
}

fn seed_user_balance(deps: &mut TestDeps, env: &cosmwasm_std::Env, user: &str, points: u128) {
    USER_BALANCES
        .save(
            deps.as_mut().storage,
            &addr(user),
            &UserBalance {
                user: addr(user),
                points_balance: Uint128::new(points),
                total_deposited_pasg: Uint128::zero(),
                total_spent_points: Uint128::zero(),
                total_sessions: 0,
                last_activity: env.block.time,
            },
        )
        .unwrap();
}

fn seed_world_config(
    deps: &mut TestDeps,
    env: &cosmwasm_std::Env,
    world_nft_id: &str,
    world_collection: &str,
    owner: &str,
    points_per_hour: u128,
) {
    WORLD_CONFIGS
        .save(
            deps.as_mut().storage,
            world_nft_id,
            &WorldConfig {
                world_nft_id: world_nft_id.to_string(),
                world_collection: addr(world_collection),
                owner: addr(owner),
                points_per_hour: Uint128::new(points_per_hour),
                active: true,
                created_at: env.block.time,
            },
        )
        .unwrap();
}

fn seed_active_session(
    deps: &mut TestDeps,
    env: &cosmwasm_std::Env,
    session_id: u64,
    user: &str,
    world_nft_id: &str,
    world_collection: &str,
    points_rate_per_hour: u128,
) {
    let user_addr = addr(user);

    SESSIONS
        .save(
            deps.as_mut().storage,
            session_id,
            &StreamingSession {
                session_id,
                user: user_addr.clone(),
                world_nft_id: world_nft_id.to_string(),
                world_collection: addr(world_collection),
                start_time: env.block.time,
                end_time: None,
                points_rate_per_hour: Uint128::new(points_rate_per_hour),
                points_charged: Uint128::zero(),
                status: SessionStatus::Active,
            },
        )
        .unwrap();

    USER_SESSIONS
        .save(deps.as_mut().storage, (&user_addr, session_id), &())
        .unwrap();
    SESSION_COUNTER
        .save(deps.as_mut().storage, &session_id)
        .unwrap();
}

fn mock_world_queries(
    deps: &mut TestDeps,
    registry_addr: &str,
    collection_addr: &str,
    collection_registered: bool,
    owner: &str,
) {
    let registry_addr = registry_addr.to_string();
    let collection_addr = collection_addr.to_string();
    let owner = owner.to_string();

    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart { contract_addr, msg } if contract_addr == &registry_addr => {
            let parsed: RegistryQueryMsg = from_json(msg).unwrap();
            match parsed {
                RegistryQueryMsg::Collection { address } => {
                    assert_eq!(address, collection_addr);
                    SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&RegistryCollectionResponse {
                            collection: collection_registered.then(|| RegistryCollection {
                                address: Addr::unchecked(collection_addr.clone()),
                            }),
                        })
                        .unwrap(),
                    ))
                }
            }
        }
        WasmQuery::Smart { contract_addr, msg } if contract_addr == &collection_addr => {
            let parsed: Cw721QueryMsg = from_json(msg).unwrap();
            match parsed {
                Cw721QueryMsg::OwnerOf {
                    token_id,
                    include_expired,
                } => {
                    assert_eq!(token_id, WORLD_ID);
                    assert_eq!(include_expired, None);
                    SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&OwnerOfResponse {
                            owner: owner.clone(),
                        })
                        .unwrap(),
                    ))
                }
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
fn start_session_rejects_public_sender() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), None);
    seed_user_balance(&mut deps, &env, USER, 500);
    seed_world_config(&mut deps, &env, WORLD_ID, WORLD_COLLECTION, "creator", 120);

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr("public"), &[]),
        ExecuteMsg::StartSession {
            user: addr(USER).to_string(),
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(WORLD_COLLECTION).to_string(),
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn stop_session_rejects_public_sender() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), None);
    seed_user_balance(&mut deps, &env, USER, 500);
    seed_active_session(&mut deps, &env, 1, USER, WORLD_ID, WORLD_COLLECTION, 100);

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr("public"), &[]),
        ExecuteMsg::StopSession {
            session_id: 1,
            duration_seconds: 3_600,
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn set_world_rate_requires_token_owner_or_admin() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), None);
    mock_world_queries(
        &mut deps,
        addr(REGISTRY).as_str(),
        addr(WORLD_COLLECTION).as_str(),
        true,
        addr("world-owner").as_str(),
    );

    let err = execute(
        deps.as_mut(),
        env.clone(),
        message_info(&addr("intruder"), &[]),
        ExecuteMsg::SetWorldRate {
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(WORLD_COLLECTION).to_string(),
            points_per_hour: Uint128::new(200),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&addr(ADMIN), &[]),
        ExecuteMsg::SetWorldRate {
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(WORLD_COLLECTION).to_string(),
            points_per_hour: Uint128::new(200),
        },
    )
    .unwrap();

    let stored = WORLD_CONFIGS.load(deps.as_ref().storage, WORLD_ID).unwrap();
    assert_eq!(stored.owner, addr("world-owner"));
    assert_eq!(stored.world_collection, addr(WORLD_COLLECTION));

    execute(
        deps.as_mut(),
        env,
        message_info(&addr("world-owner"), &[]),
        ExecuteMsg::SetWorldRate {
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(WORLD_COLLECTION).to_string(),
            points_per_hour: Uint128::new(250),
        },
    )
    .unwrap();
}

#[test]
fn start_session_uses_configured_world_collection() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), None);
    seed_user_balance(&mut deps, &env, USER, 500);
    seed_world_config(&mut deps, &env, WORLD_ID, WORLD_COLLECTION, "creator", 120);

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&addr(BACKEND), &[]),
        ExecuteMsg::StartSession {
            user: addr(USER).to_string(),
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(WORLD_COLLECTION).to_string(),
        },
    )
    .unwrap();

    let session = SESSIONS.load(deps.as_ref().storage, 1).unwrap();
    assert_eq!(session.world_collection, addr(WORLD_COLLECTION));

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr(BACKEND), &[]),
        ExecuteMsg::StartSession {
            user: addr(USER).to_string(),
            world_nft_id: WORLD_ID.to_string(),
            world_collection: addr(OTHER_COLLECTION).to_string(),
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::InvalidWorldCollection {
            reason: format!(
                "expected {}, got {}",
                addr(WORLD_COLLECTION),
                addr(OTHER_COLLECTION)
            ),
        }
    );
}

#[test]
fn report_fiat_purchase_rejects_duplicate_transaction_id_globally() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, None, Some(FIAT_ORACLE));

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&addr(FIAT_ORACLE), &[]),
        ExecuteMsg::ReportFiatPurchase {
            user: addr(USER).to_string(),
            fiat_amount_usd: Uint128::new(1_000),
            pasg_amount: Uint128::new(10),
            points_awarded: Uint128::new(1_000),
            transaction_id: "tx-1".to_string(),
            timestamp: env.block.time,
        },
    )
    .unwrap();

    let err = execute(
        deps.as_mut(),
        env.clone(),
        message_info(&addr(FIAT_ORACLE), &[]),
        ExecuteMsg::ReportFiatPurchase {
            user: addr("another-user").to_string(),
            fiat_amount_usd: Uint128::new(1_000),
            pasg_amount: Uint128::new(10),
            points_awarded: Uint128::new(1_000),
            transaction_id: "tx-1".to_string(),
            timestamp: env.block.time,
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::DuplicateTransaction {
            transaction_id: "tx-1".to_string(),
        }
    );
}

#[test]
fn report_fiat_purchase_rejects_stale_timestamp() {
    let mut deps = mock_dependencies();
    let env = env_at(2_000);

    instantiate_contract(&mut deps, None, Some(FIAT_ORACLE));

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr(FIAT_ORACLE), &[]),
        ExecuteMsg::ReportFiatPurchase {
            user: addr(USER).to_string(),
            fiat_amount_usd: Uint128::new(1_000),
            pasg_amount: Uint128::new(10),
            points_awarded: Uint128::new(1_000),
            transaction_id: "stale-tx".to_string(),
            timestamp: Timestamp::from_seconds(1_099),
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::InvalidFiatPurchase {
            reason: "fiat report timestamp is older than allowed".to_string(),
        }
    );
}

#[test]
fn stop_session_rejects_duration_above_max() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), None);
    seed_user_balance(&mut deps, &env, USER, 500);
    seed_active_session(&mut deps, &env, 1, USER, WORLD_ID, WORLD_COLLECTION, 100);

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr(BACKEND), &[]),
        ExecuteMsg::StopSession {
            session_id: 1,
            duration_seconds: 86_401,
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::InvalidDuration {
            duration_seconds: 86_401,
        }
    );
}

#[test]
fn update_config_persists_backend_operator() {
    let mut deps = mock_dependencies();

    instantiate_contract(&mut deps, None, None);

    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addr(ADMIN), &[]),
        ExecuteMsg::UpdateConfig {
            admin: None,
            split_router: None,
            backend_operator: Some(addr(BACKEND).to_string()),
            fiat_oracle: None,
            stripe_webhook_validator: None,
            paused: None,
        },
    )
    .unwrap();

    let config: Config = CONFIG.load(deps.as_ref().storage).unwrap();
    assert_eq!(config.backend_operator, Some(addr(BACKEND)));

    let stats: PlatformStats = PLATFORM_STATS.load(deps.as_ref().storage).unwrap();
    assert_eq!(stats.total_sessions, 0);
}

#[test]
fn instantiate_rejects_zero_pasg_conversion_rate() {
    let mut deps = mock_dependencies();

    let err = instantiate(
        deps.as_mut(),
        mock_env(),
        message_info(&addr("deployer"), &[]),
        InstantiateMsg {
            admin: addr(ADMIN).to_string(),
            split_router: addr(SPLIT_ROUTER).to_string(),
            registry: addr(REGISTRY).to_string(),
            backend_operator: None,
            denom: "upasg".to_string(),
            points_per_denom: Uint128::zero(),
            fiat_oracle: None,
            stripe_webhook_validator: None,
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::InvalidConversionRate {});
}

#[test]
fn instantiate_marks_non_native_denom_as_compatibility_only() {
    let mut deps = mock_dependencies();

    instantiate_contract_with_denom(&mut deps, Some(BACKEND), Some(FIAT_ORACLE), "legacy-pasg");

    let response: ConfigResponse =
        from_json(query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();

    assert_eq!(response.pasg_denom, "upasg");
    let shim = response
        .pasg_utility
        .compatibility_shim
        .expect("legacy denom should expose compatibility shim metadata");
    assert_eq!(shim.kind, PasgCompatibilityShimKind::NativeDenomAdapter);
    assert!(shim.forwards_to_native_denom);
    assert!(response
        .pasg_utility
        .compatibility_note
        .contains("native upasg settlement"));
}

#[test]
fn deposit_crypto_accepts_native_upasg_and_updates_balance() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));

    let response = execute(
        deps.as_mut(),
        env,
        message_info(&addr(USER), &coins(12, "upasg")),
        ExecuteMsg::DepositCrypto {},
    )
    .unwrap();

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "deposit_crypto"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_amount" && attr.value == "12"));

    let balance: UserBalanceResponse = from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserBalance {
                user: addr(USER).to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(balance.points_balance, Uint128::new(1_200));
    assert_eq!(balance.total_deposited_pasg, Uint128::new(12));
}

#[test]
fn deposit_crypto_rejects_non_pasg_funds() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&addr(USER), &coins(12, "ujuno")),
        ExecuteMsg::DepositCrypto {},
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::InvalidPayment {
            expected: "upasg".to_string(),
            denom: "ujuno".to_string(),
        }
    );
}

#[test]
fn withdraw_points_returns_native_upasg() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));
    seed_user_balance(&mut deps, &env, USER, 10_000);

    let response = execute(
        deps.as_mut(),
        env,
        message_info(&addr(USER), &[]),
        ExecuteMsg::WithdrawPoints {
            points: Uint128::new(10_000),
        },
    )
    .unwrap();

    assert_eq!(response.messages.len(), 1);
    match &response.messages[0].msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, &addr(USER).to_string());
            assert_eq!(amount, &coins(98, "upasg"));
        }
        other => panic!("expected bank send, got {other:?}"),
    }

    let balance: UserBalanceResponse = from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserBalance {
                user: addr(USER).to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(balance.points_balance, Uint128::zero());
}

#[test]
fn distribute_world_revenue_uses_native_upasg_funds() {
    let mut deps = mock_dependencies();
    let env = env_at(1_000);

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));
    seed_world_config(&mut deps, &env, WORLD_ID, WORLD_COLLECTION, "creator", 120);
    seed_pending_revenue(&mut deps, WORLD_ID, 300, 3, None);

    let response = execute(
        deps.as_mut(),
        env,
        message_info(&addr(USER), &[]),
        ExecuteMsg::DistributeWorldRevenue {
            world_nft_id: WORLD_ID.to_string(),
        },
    )
    .unwrap();

    assert_eq!(response.messages.len(), 1);
    match &response.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            funds,
            ..
        }) => {
            assert_eq!(contract_addr, &addr(SPLIT_ROUTER).to_string());
            assert_eq!(funds, &coins(3, "upasg"));
        }
        other => panic!("expected wasm execute, got {other:?}"),
    }

    let pending = PENDING_REVENUE
        .load(deps.as_ref().storage, WORLD_ID)
        .unwrap();
    assert_eq!(pending.pending_points, Uint128::zero());
    assert_eq!(pending.pending_pasg, Uint128::zero());
    assert!(pending.last_distribution.is_some());
}

#[test]
fn config_query_exposes_pasg_utility_metadata() {
    let mut deps = mock_dependencies();

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));

    let response: ConfigResponse =
        from_json(query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();

    assert_eq!(response.pasg_denom, "upasg");
    assert_eq!(response.points_per_pasg, Uint128::new(100));
    assert_eq!(
        response.pasg_utility.settlement_kind,
        PasgSettlementKind::NativeDenom
    );
    assert!(response.pasg_utility.compatibility_shim.is_none());
    assert!(response
        .pasg_utility
        .compatibility_note
        .contains("forward to native upasg settlement"));
}

#[test]
fn pasg_utility_query_is_native_first_source_of_truth() {
    let mut deps = mock_dependencies();

    instantiate_contract(&mut deps, Some(BACKEND), Some(FIAT_ORACLE));

    let utility: PasgUtilityResponse =
        from_json(query(deps.as_ref(), mock_env(), QueryMsg::PasgUtility {}).unwrap()).unwrap();
    let conversion_rate: ConversionRateResponse =
        from_json(query(deps.as_ref(), mock_env(), QueryMsg::ConversionRate {}).unwrap()).unwrap();

    assert_eq!(utility.canonical_denom, "upasg");
    assert_eq!(utility.points_per_pasg, Uint128::new(100));
    assert_eq!(utility.pasg_per_point, conversion_rate.pasg_per_point);
    assert_eq!(utility.canonical_denom, conversion_rate.pasg_denom);
    assert_eq!(
        utility.metadata.settlement_kind,
        PasgSettlementKind::NativeDenom
    );
    assert!(utility.metadata.compatibility_shim.is_none());
    assert_eq!(utility.canonical_query, PasgUtilityQueryRoute::PasgUtility);
    assert_eq!(
        utility.canonical_execute,
        vec![
            PasgUtilityExecuteRoute::DepositCrypto,
            PasgUtilityExecuteRoute::ReportFiatPurchase,
            PasgUtilityExecuteRoute::WithdrawPoints,
            PasgUtilityExecuteRoute::DistributeWorldRevenue,
            PasgUtilityExecuteRoute::BatchDistributeRevenue,
        ]
    );
    assert_eq!(utility.compatibility_router.contract, addr(SPLIT_ROUTER));
    assert!(utility.compatibility_router.forwards_native_denom);
    assert_eq!(
        utility.scope_boundary.settlement,
        PasgBusinessBoundary::OnChainUtilitySurface
    );
    assert_eq!(
        utility.scope_boundary.platform_billing,
        PasgBusinessBoundary::OffChainService
    );
    assert_eq!(
        utility.scope_boundary.subscriptions,
        PasgBusinessBoundary::OffChainService
    );
}

#[test]
fn canonical_pasg_query_shape_is_stable() {
    let binary = to_json_binary(&QueryMsg::PasgUtility {}).unwrap();
    assert_eq!(
        String::from_utf8(binary.to_vec()).unwrap(),
        r#"{"pasg_utility":{}}"#
    );
}

#[test]
fn split_router_message_surface_stays_generic_for_pasg_policy() {
    let split_router_msg_source =
        include_str!("../../split-router/src/msg.rs").to_ascii_lowercase();

    assert!(!split_router_msg_source.contains("pasg"));
    assert!(!split_router_msg_source.contains("upasg"));
}
