use cosmwasm_std::{
    from_json,
    testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
    to_json_binary, Addr, ContractResult, OwnedDeps, SystemError, SystemResult, Timestamp,
    Uint128, WasmQuery,
};

use crate::{
    contract::{execute, instantiate},
    error::ContractError,
    msg::{
        Cw721QueryMsg, ExecuteMsg, InstantiateMsg, OwnerOfResponse, RegistryCollection,
        RegistryCollectionResponse, RegistryQueryMsg, SessionStatus,
    },
    state::{
        Config, PlatformStats, StreamingSession, UserBalance, WorldConfig, CONFIG,
        PLATFORM_STATS, SESSION_COUNTER, SESSIONS, USER_BALANCES, USER_SESSIONS, WORLD_CONFIGS,
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
    let msg = InstantiateMsg {
        admin: addr(ADMIN).to_string(),
        split_router: addr(SPLIT_ROUTER).to_string(),
        registry: addr(REGISTRY).to_string(),
        backend_operator: backend_operator.map(|value| addr(value).to_string()),
        denom: "upasg".to_string(),
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
    SESSION_COUNTER.save(deps.as_mut().storage, &session_id).unwrap();
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
