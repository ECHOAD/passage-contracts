use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coins, entry_point, to_json_binary, Addr, BankMsg, Binary, Decimal, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Response, StdResult, Uint128, WasmMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{
    ConfigResponse, ConversionRateResponse, Cw721QueryMsg, ExecuteMsg, InstantiateMsg,
    OwnerOfResponse, PasgBusinessBoundary, PasgCompatibilityRouterExecuteRoute,
    PasgCompatibilityRouterResponse, PasgScopeBoundaryResponse, PasgSettlementKind,
    PasgUtilityExecuteRoute, PasgUtilityMetadata, PasgUtilityQueryRoute, PasgUtilityResponse,
    PendingRevenueResponse, PlatformStatsResponse, PurchaseHistoryResponse, PurchaseRecord,
    PurchaseType, QueryMsg, RegistryCollectionResponse, RegistryQueryMsg, SessionResponse,
    SessionStatus, UserBalanceResponse, UserSessionsResponse, WorldConfigResponse,
    WorldStatsResponse,
};
use crate::state::{
    Config, PendingRevenue, PlatformStats, Purchase, StreamingSession, UserBalance, WorldConfig,
    WorldStats, CONFIG, FIAT_PURCHASE_TX_IDS, PENDING_REVENUE, PLATFORM_STATS, PURCHASES,
    PURCHASE_COUNTER, SESSIONS, SESSION_COUNTER, USER_BALANCES, USER_SESSIONS, WORLD_CONFIGS,
    WORLD_STATS, WORLD_USERS,
};

const CONTRACT_NAME: &str = "crates.io:streaming-billing";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_FIAT_REPORT_AGE_SECONDS: u64 = 900;
const MAX_SESSION_DURATION_SECONDS: u64 = 86_400;
const PASG_COMPATIBILITY_NOTE: &str =
    "Compatibility shims must forward to native upasg settlement and must not redefine PASG economics.";

#[cw_serde]
struct SplitRouterRouteWorldRevenueExecuteMsg {
    route_world_revenue: RouteWorldRevenuePayload,
}

#[cw_serde]
struct RouteWorldRevenuePayload {
    world_nft_id: String,
    world_collection: String,
}

fn default_pasg_utility_metadata() -> PasgUtilityMetadata {
    PasgUtilityMetadata {
        settlement_kind: PasgSettlementKind::NativeDenom,
        compatibility_shim: None,
        compatibility_note: PASG_COMPATIBILITY_NOTE.to_string(),
    }
}

fn default_pasg_execute_routes() -> Vec<PasgUtilityExecuteRoute> {
    vec![
        PasgUtilityExecuteRoute::DepositCrypto,
        PasgUtilityExecuteRoute::ReportFiatPurchase,
        PasgUtilityExecuteRoute::WithdrawPoints,
        PasgUtilityExecuteRoute::DistributeWorldRevenue,
        PasgUtilityExecuteRoute::BatchDistributeRevenue,
    ]
}

fn pasg_per_point(points_per_pasg: Uint128) -> Decimal {
    Decimal::one()
        .checked_div(Decimal::from_ratio(points_per_pasg, Uint128::one()))
        .unwrap_or(Decimal::zero())
}

// ========================================
// INSTANTIATE
// ========================================

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.points_per_denom.is_zero() {
        return Err(ContractError::InvalidConversionRate {});
    }

    let admin = deps.api.addr_validate(&msg.admin)?;
    let split_router = deps.api.addr_validate(&msg.split_router)?;
    let registry = deps.api.addr_validate(&msg.registry)?;
    let backend_operator = msg
        .backend_operator
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let fiat_oracle = msg
        .fiat_oracle
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let stripe_webhook_validator = msg
        .stripe_webhook_validator
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let config = Config {
        admin,
        split_router,
        registry,
        backend_operator,
        pasg_denom: msg.denom,
        points_per_pasg: msg.points_per_denom,
        pasg_utility: default_pasg_utility_metadata(),
        fiat_oracle,
        stripe_webhook_validator,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;

    // Initialize platform stats
    let platform_stats = PlatformStats {
        total_users: 0,
        total_sessions: 0,
        total_points_issued: Uint128::zero(),
        total_pasg_volume: Uint128::zero(),
        active_sessions: 0,
    };
    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    SESSION_COUNTER.save(deps.storage, &0)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

// ========================================
// EXECUTE
// ========================================

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.paused && !matches!(msg, ExecuteMsg::UpdateConfig { .. }) {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        ExecuteMsg::UpdateConfig {
            admin,
            split_router,
            backend_operator,
            fiat_oracle,
            stripe_webhook_validator,
            paused,
        } => execute_update_config(
            deps,
            info,
            admin,
            split_router,
            backend_operator,
            fiat_oracle,
            stripe_webhook_validator,
            paused,
        ),
        ExecuteMsg::DepositCrypto {} => execute_deposit_crypto(deps, env, info),
        ExecuteMsg::ReportFiatPurchase {
            user,
            fiat_amount_usd,
            pasg_amount,
            points_awarded,
            transaction_id,
            timestamp,
        } => execute_report_fiat_purchase(
            deps,
            env,
            info,
            user,
            fiat_amount_usd,
            pasg_amount,
            points_awarded,
            transaction_id,
            timestamp,
        ),
        ExecuteMsg::WithdrawPoints { points } => execute_withdraw_points(deps, env, info, points),
        ExecuteMsg::StartSession {
            user,
            world_nft_id,
            world_collection,
        } => execute_start_session(deps, env, info, user, world_nft_id, world_collection),
        ExecuteMsg::StopSession {
            session_id,
            duration_seconds,
        } => execute_stop_session(deps, env, info, session_id, duration_seconds),
        ExecuteMsg::ForceStopSession { session_id } => {
            execute_force_stop_session(deps, env, info, session_id)
        }
        ExecuteMsg::SetWorldRate {
            world_nft_id,
            world_collection,
            points_per_hour,
        } => execute_set_world_rate(
            deps,
            env,
            info,
            world_nft_id,
            world_collection,
            points_per_hour,
        ),
        ExecuteMsg::UpdateWorldRate {
            world_nft_id,
            points_per_hour,
        } => execute_update_world_rate(deps, info, world_nft_id, points_per_hour),
        ExecuteMsg::DistributeWorldRevenue { world_nft_id } => {
            execute_distribute_world_revenue(deps, env, info, world_nft_id)
        }
        ExecuteMsg::BatchDistributeRevenue { world_nft_ids } => {
            execute_batch_distribute_revenue(deps, env, info, world_nft_ids)
        }
    }
}

// ========================================
// EXECUTE HANDLERS
// ========================================

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    split_router: Option<String>,
    backend_operator: Option<String>,
    fiat_oracle: Option<String>,
    stripe_webhook_validator: Option<String>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(admin) = admin {
        config.admin = deps.api.addr_validate(&admin)?;
    }
    if let Some(split_router) = split_router {
        config.split_router = deps.api.addr_validate(&split_router)?;
    }
    if let Some(backend_operator) = backend_operator {
        config.backend_operator = Some(deps.api.addr_validate(&backend_operator)?);
    }
    if let Some(fiat_oracle) = fiat_oracle {
        config.fiat_oracle = Some(deps.api.addr_validate(&fiat_oracle)?);
    }
    if let Some(validator) = stripe_webhook_validator {
        config.stripe_webhook_validator = Some(deps.api.addr_validate(&validator)?);
    }
    if let Some(paused) = paused {
        config.paused = paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn ensure_backend_operator(config: &Config, sender: &Addr) -> Result<(), ContractError> {
    if *sender == config.admin || config.backend_operator.as_ref() == Some(sender) {
        return Ok(());
    }

    Err(ContractError::Unauthorized {})
}

fn query_registered_world_collection(
    deps: Deps,
    registry: &Addr,
    world_collection: &str,
) -> Result<Addr, ContractError> {
    let world_collection_addr = deps.api.addr_validate(world_collection)?;
    let response: RegistryCollectionResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: registry.to_string(),
            msg: to_json_binary(&RegistryQueryMsg::Collection {
                address: world_collection_addr.to_string(),
            })?,
        }))
        .map_err(|_| ContractError::InvalidWorldCollection {
            reason: format!(
                "collection {} could not be resolved from registry",
                world_collection_addr
            ),
        })?;

    let collection = response
        .collection
        .ok_or_else(|| ContractError::InvalidWorldCollection {
            reason: format!("collection {} is not registered", world_collection_addr),
        })?;

    if collection.address != world_collection_addr {
        return Err(ContractError::InvalidWorldCollection {
            reason: format!(
                "registry returned {} for requested collection {}",
                collection.address, world_collection_addr
            ),
        });
    }

    Ok(collection.address)
}

fn query_world_owner(
    deps: Deps,
    world_collection: &Addr,
    world_nft_id: &str,
) -> Result<Addr, ContractError> {
    let response: OwnerOfResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: world_collection.to_string(),
            msg: to_json_binary(&Cw721QueryMsg::OwnerOf {
                token_id: world_nft_id.to_string(),
                include_expired: None,
            })?,
        }))
        .map_err(|_| ContractError::InvalidWorldOwner {})?;

    deps.api
        .addr_validate(&response.owner)
        .map_err(ContractError::from)
}

/// User deposits PASG directly to buy streaming points
fn execute_deposit_crypto(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Verify payment
    let payment = info
        .funds
        .iter()
        .find(|coin| coin.denom == config.pasg_denom)
        .ok_or(ContractError::NoPayment {})?;

    let pasg_amount = payment.amount;

    // Calculate points to award
    let points_awarded = pasg_amount
        .checked_mul(config.points_per_pasg)
        .map_err(|_| ContractError::InvalidConversionRate {})?;

    // Update user balance
    let mut user_balance = USER_BALANCES
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(UserBalance {
            user: info.sender.clone(),
            points_balance: Uint128::zero(),
            total_deposited_pasg: Uint128::zero(),
            total_spent_points: Uint128::zero(),
            total_sessions: 0,
            last_activity: env.block.time,
        });

    user_balance.points_balance += points_awarded;
    user_balance.total_deposited_pasg += pasg_amount;
    user_balance.last_activity = env.block.time;

    USER_BALANCES.save(deps.storage, &info.sender, &user_balance)?;

    // Record purchase
    let purchase_id = PURCHASE_COUNTER
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(0)
        + 1;

    PURCHASE_COUNTER.save(deps.storage, &info.sender, &purchase_id)?;

    let purchase = Purchase {
        id: purchase_id,
        user: info.sender.clone(),
        timestamp: env.block.time,
        purchase_type: PurchaseType::CryptoDirect,
        amount_usd: None,
        amount_pasg: pasg_amount,
        points_received: points_awarded,
        transaction_id: None,
    };

    PURCHASES.save(deps.storage, (&info.sender, purchase_id), &purchase)?;

    // Update platform stats
    let mut platform_stats = PLATFORM_STATS.load(deps.storage)?;
    platform_stats.total_points_issued += points_awarded;
    platform_stats.total_pasg_volume += pasg_amount;

    if purchase_id == 1 {
        platform_stats.total_users += 1;
    }

    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    Ok(Response::new()
        .add_attribute("action", "deposit_crypto")
        .add_attribute("user", info.sender)
        .add_attribute("pasg_amount", pasg_amount)
        .add_attribute("points_awarded", points_awarded))
}

/// Off-chain oracle reports fiat purchase (after Stripe payment)
fn execute_report_fiat_purchase(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
    fiat_amount_usd: Uint128,
    pasg_amount: Uint128,
    points_awarded: Uint128,
    transaction_id: String,
    timestamp: cosmwasm_std::Timestamp,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only fiat oracle can call this
    if Some(info.sender) != config.fiat_oracle {
        return Err(ContractError::Unauthorized {});
    }

    let block_time = env.block.time.seconds();
    let report_time = timestamp.seconds();
    if report_time > block_time {
        return Err(ContractError::InvalidFiatPurchase {
            reason: "fiat report timestamp cannot be in the future".to_string(),
        });
    }
    if block_time.saturating_sub(report_time) > MAX_FIAT_REPORT_AGE_SECONDS {
        return Err(ContractError::InvalidFiatPurchase {
            reason: "fiat report timestamp is older than allowed".to_string(),
        });
    }

    let user_addr = deps.api.addr_validate(&user)?;

    if FIAT_PURCHASE_TX_IDS.has(deps.storage, transaction_id.as_str()) {
        return Err(ContractError::DuplicateTransaction { transaction_id });
    }

    FIAT_PURCHASE_TX_IDS.save(deps.storage, transaction_id.as_str(), &true)?;

    // Update user balance
    let mut user_balance =
        USER_BALANCES
            .may_load(deps.storage, &user_addr)?
            .unwrap_or(UserBalance {
                user: user_addr.clone(),
                points_balance: Uint128::zero(),
                total_deposited_pasg: Uint128::zero(),
                total_spent_points: Uint128::zero(),
                total_sessions: 0,
                last_activity: env.block.time,
            });

    user_balance.points_balance += points_awarded;
    user_balance.total_deposited_pasg += pasg_amount;
    user_balance.last_activity = env.block.time;

    USER_BALANCES.save(deps.storage, &user_addr, &user_balance)?;

    // Record purchase
    let purchase_id = PURCHASE_COUNTER
        .may_load(deps.storage, &user_addr)?
        .unwrap_or(0)
        + 1;

    PURCHASE_COUNTER.save(deps.storage, &user_addr, &purchase_id)?;

    let purchase = Purchase {
        id: purchase_id,
        user: user_addr.clone(),
        timestamp,
        purchase_type: PurchaseType::FiatConverted,
        amount_usd: Some(fiat_amount_usd),
        amount_pasg: pasg_amount,
        points_received: points_awarded,
        transaction_id: Some(transaction_id.clone()),
    };

    PURCHASES.save(deps.storage, (&user_addr, purchase_id), &purchase)?;

    // Update platform stats
    let mut platform_stats = PLATFORM_STATS.load(deps.storage)?;
    platform_stats.total_points_issued += points_awarded;
    platform_stats.total_pasg_volume += pasg_amount;

    if purchase_id == 1 {
        platform_stats.total_users += 1;
    }

    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    Ok(Response::new()
        .add_attribute("action", "report_fiat_purchase")
        .add_attribute("user", user)
        .add_attribute("fiat_amount_usd", fiat_amount_usd)
        .add_attribute("pasg_amount", pasg_amount)
        .add_attribute("points_awarded", points_awarded)
        .add_attribute("transaction_id", transaction_id))
}

/// User withdraws unused points back to PASG
fn execute_withdraw_points(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    points: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mut user_balance = USER_BALANCES
        .load(deps.storage, &info.sender)
        .map_err(|_| ContractError::UserBalanceNotFound {
            user: info.sender.to_string(),
        })?;

    // Check sufficient balance
    if user_balance.points_balance < points {
        return Err(ContractError::InsufficientPoints {
            have: user_balance.points_balance.u128(),
            need: points.u128(),
        });
    }

    // Calculate PASG to return (with 2% withdrawal fee)
    let pasg_amount = points
        .checked_div(config.points_per_pasg)
        .map_err(|_| ContractError::InvalidConversionRate {})?;

    let fee = pasg_amount.multiply_ratio(2u128, 100u128); // 2% fee
    let amount_to_return = pasg_amount
        .checked_sub(fee)
        .map_err(|_| ContractError::WithdrawalTooSmall {})?;

    if amount_to_return.is_zero() {
        return Err(ContractError::WithdrawalTooSmall {});
    }

    // Update balance
    user_balance.points_balance -= points;
    user_balance.last_activity = env.block.time;
    USER_BALANCES.save(deps.storage, &info.sender, &user_balance)?;

    // Send PASG back to user
    let send_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(amount_to_return.u128(), &config.pasg_denom),
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", "withdraw_points")
        .add_attribute("user", info.sender)
        .add_attribute("points_withdrawn", points)
        .add_attribute("pasg_returned", amount_to_return)
        .add_attribute("fee", fee))
}

/// Start a streaming session
fn execute_start_session(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
    world_nft_id: String,
    world_collection: String,
) -> Result<Response, ContractError> {
    let user_addr = deps.api.addr_validate(&user)?;
    let config = CONFIG.load(deps.storage)?;
    ensure_backend_operator(&config, &info.sender)?;

    // Load world config
    let world_config = WORLD_CONFIGS
        .load(deps.storage, &world_nft_id)
        .map_err(|_| ContractError::WorldConfigNotFound {
            world_nft_id: world_nft_id.clone(),
        })?;

    if !world_config.active {
        return Err(ContractError::WorldNotActive { world_nft_id });
    }

    if !world_collection.is_empty() {
        let hinted_collection = deps.api.addr_validate(&world_collection)?;
        if hinted_collection != world_config.world_collection {
            return Err(ContractError::InvalidWorldCollection {
                reason: format!(
                    "expected {}, got {}",
                    world_config.world_collection, hinted_collection
                ),
            });
        }
    }

    // Verify user has balance
    let user_balance = USER_BALANCES.load(deps.storage, &user_addr).map_err(|_| {
        ContractError::UserBalanceNotFound {
            user: user_addr.to_string(),
        }
    })?;

    if user_balance.points_balance.is_zero() {
        return Err(ContractError::InsufficientPoints {
            have: 0,
            need: world_config.points_per_hour.u128(),
        });
    }

    // Create session
    let session_id = SESSION_COUNTER.load(deps.storage)? + 1;
    SESSION_COUNTER.save(deps.storage, &session_id)?;

    let session = StreamingSession {
        session_id,
        user: user_addr.clone(),
        world_nft_id: world_nft_id.clone(),
        world_collection: world_config.world_collection.clone(),
        start_time: env.block.time,
        end_time: None,
        points_rate_per_hour: world_config.points_per_hour,
        points_charged: Uint128::zero(),
        status: SessionStatus::Active,
    };

    SESSIONS.save(deps.storage, session_id, &session)?;
    USER_SESSIONS.save(deps.storage, (&user_addr, session_id), &())?;

    // Update platform stats
    let mut platform_stats = PLATFORM_STATS.load(deps.storage)?;
    platform_stats.active_sessions += 1;
    platform_stats.total_sessions += 1;
    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    Ok(Response::new()
        .add_attribute("action", "start_session")
        .add_attribute("session_id", session_id.to_string())
        .add_attribute("user", user)
        .add_attribute("world_nft_id", world_nft_id))
}

/// Stop session and charge user
fn execute_stop_session(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    session_id: u64,
    duration_seconds: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    ensure_backend_operator(&config, &info.sender)?;

    let mut session = SESSIONS
        .load(deps.storage, session_id)
        .map_err(|_| ContractError::SessionNotFound { session_id })?;

    if session.status != SessionStatus::Active {
        return Err(ContractError::SessionAlreadyStopped { session_id });
    }

    if duration_seconds == 0 || duration_seconds > MAX_SESSION_DURATION_SECONDS {
        return Err(ContractError::InvalidDuration { duration_seconds });
    }

    // Calculate points to charge
    let hours = Decimal::from_ratio(duration_seconds as u128, 3600u128);
    let points_to_charge = session
        .points_rate_per_hour
        .checked_mul_floor(hours)
        .map_err(|_| ContractError::InvalidConversionRate {})?;

    // Load user balance
    let mut user_balance = USER_BALANCES.load(deps.storage, &session.user)?;

    // Check sufficient balance
    if user_balance.points_balance < points_to_charge {
        // Charge what they have
        session.points_charged = user_balance.points_balance;
    } else {
        session.points_charged = points_to_charge;
    }

    // Deduct points from user
    user_balance.points_balance -= session.points_charged;
    user_balance.total_spent_points += session.points_charged;
    user_balance.total_sessions += 1;
    user_balance.last_activity = env.block.time;
    USER_BALANCES.save(deps.storage, &session.user, &user_balance)?;

    // Update session
    session.end_time = Some(env.block.time);
    session.status = SessionStatus::Completed;
    SESSIONS.save(deps.storage, session_id, &session)?;

    // Convert points to PASG for revenue distribution
    let pasg_earned = session
        .points_charged
        .checked_div(config.points_per_pasg)
        .unwrap_or(Uint128::zero());

    // Update world stats
    let mut world_stats = WORLD_STATS
        .may_load(deps.storage, &session.world_nft_id)?
        .unwrap_or(WorldStats {
            world_nft_id: session.world_nft_id.clone(),
            total_sessions: 0,
            total_points_earned: Uint128::zero(),
            total_pasg_earned: Uint128::zero(),
            unique_users: 0,
            total_duration_seconds: 0,
        });

    world_stats.total_sessions += 1;
    world_stats.total_points_earned += session.points_charged;
    world_stats.total_pasg_earned += pasg_earned;
    world_stats.total_duration_seconds += duration_seconds;

    // Track unique users
    if !WORLD_USERS.has(deps.storage, (&session.world_nft_id, &session.user)) {
        WORLD_USERS.save(deps.storage, (&session.world_nft_id, &session.user), &())?;
        world_stats.unique_users += 1;
    }

    WORLD_STATS.save(deps.storage, &session.world_nft_id, &world_stats)?;

    // Add to pending revenue
    let mut pending = PENDING_REVENUE
        .may_load(deps.storage, &session.world_nft_id)?
        .unwrap_or(PendingRevenue {
            world_nft_id: session.world_nft_id.clone(),
            pending_points: Uint128::zero(),
            pending_pasg: Uint128::zero(),
            last_distribution: None,
        });

    pending.pending_points += session.points_charged;
    pending.pending_pasg += pasg_earned;
    PENDING_REVENUE.save(deps.storage, &session.world_nft_id, &pending)?;

    // Update platform stats
    let mut platform_stats = PLATFORM_STATS.load(deps.storage)?;
    platform_stats.active_sessions = platform_stats.active_sessions.saturating_sub(1);
    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    Ok(Response::new()
        .add_attribute("action", "stop_session")
        .add_attribute("session_id", session_id.to_string())
        .add_attribute("points_charged", session.points_charged)
        .add_attribute("pasg_earned", pasg_earned)
        .add_attribute("duration_seconds", duration_seconds.to_string()))
}

fn execute_force_stop_session(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    session_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let mut session = SESSIONS
        .load(deps.storage, session_id)
        .map_err(|_| ContractError::SessionNotFound { session_id })?;

    if session.status != SessionStatus::Active {
        return Err(ContractError::SessionAlreadyStopped { session_id });
    }

    session.end_time = Some(env.block.time);
    session.status = SessionStatus::ForceStopped;
    SESSIONS.save(deps.storage, session_id, &session)?;

    // Update platform stats
    let mut platform_stats = PLATFORM_STATS.load(deps.storage)?;
    platform_stats.active_sessions = platform_stats.active_sessions.saturating_sub(1);
    PLATFORM_STATS.save(deps.storage, &platform_stats)?;

    Ok(Response::new()
        .add_attribute("action", "force_stop_session")
        .add_attribute("session_id", session_id.to_string()))
}

fn execute_set_world_rate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    world_nft_id: String,
    world_collection: String,
    points_per_hour: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let world_collection_addr =
        query_registered_world_collection(deps.as_ref(), &config.registry, &world_collection)?;
    let verified_owner = query_world_owner(deps.as_ref(), &world_collection_addr, &world_nft_id)?;

    if info.sender != config.admin && info.sender != verified_owner {
        return Err(ContractError::Unauthorized {});
    }

    let world_config = WorldConfig {
        world_nft_id: world_nft_id.clone(),
        world_collection: world_collection_addr,
        owner: verified_owner,
        points_per_hour,
        active: true,
        created_at: env.block.time,
    };

    WORLD_CONFIGS.save(deps.storage, &world_nft_id, &world_config)?;

    Ok(Response::new()
        .add_attribute("action", "set_world_rate")
        .add_attribute("world_nft_id", world_nft_id)
        .add_attribute("points_per_hour", points_per_hour))
}

fn execute_update_world_rate(
    deps: DepsMut,
    info: MessageInfo,
    world_nft_id: String,
    points_per_hour: Uint128,
) -> Result<Response, ContractError> {
    let mut world_config = WORLD_CONFIGS
        .load(deps.storage, &world_nft_id)
        .map_err(|_| ContractError::WorldConfigNotFound {
            world_nft_id: world_nft_id.clone(),
        })?;

    // Only owner can update
    if info.sender != world_config.owner {
        return Err(ContractError::InvalidWorldOwner {});
    }

    world_config.points_per_hour = points_per_hour;
    WORLD_CONFIGS.save(deps.storage, &world_nft_id, &world_config)?;

    Ok(Response::new()
        .add_attribute("action", "update_world_rate")
        .add_attribute("world_nft_id", world_nft_id)
        .add_attribute("points_per_hour", points_per_hour))
}

/// Distribute accumulated revenue to world owner via split-router
fn execute_distribute_world_revenue(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    world_nft_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mut pending = PENDING_REVENUE
        .load(deps.storage, &world_nft_id)
        .map_err(|_| ContractError::NoPendingRevenue {
            world_nft_id: world_nft_id.clone(),
        })?;

    if pending.pending_pasg.is_zero() {
        return Err(ContractError::NoPendingRevenue { world_nft_id });
    }

    let world_config = WORLD_CONFIGS.load(deps.storage, &world_nft_id)?;

    // Call split-router to distribute revenue
    // Split-router will read revenue_shares from World NFT and distribute
    let distribute_msg = WasmMsg::Execute {
        contract_addr: config.split_router.to_string(),
        msg: to_json_binary(&SplitRouterRouteWorldRevenueExecuteMsg {
            route_world_revenue: RouteWorldRevenuePayload {
                world_nft_id: world_nft_id.clone(),
                world_collection: world_config.world_collection.to_string(),
            },
        })?,
        funds: coins(pending.pending_pasg.u128(), &config.pasg_denom),
    };

    // Reset pending revenue
    let distributed_pasg = pending.pending_pasg;
    pending.pending_points = Uint128::zero();
    pending.pending_pasg = Uint128::zero();
    pending.last_distribution = Some(env.block.time);
    PENDING_REVENUE.save(deps.storage, &world_nft_id, &pending)?;

    Ok(Response::new()
        .add_message(distribute_msg)
        .add_attribute("action", "distribute_world_revenue")
        .add_attribute("world_nft_id", world_nft_id)
        .add_attribute("pasg_distributed", distributed_pasg))
}

fn execute_batch_distribute_revenue(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    world_nft_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_attribute("action", "batch_distribute_revenue");

    for world_nft_id in world_nft_ids {
        match execute_distribute_world_revenue(
            deps.branch(),
            env.clone(),
            info.clone(),
            world_nft_id.clone(),
        ) {
            Ok(res) => {
                response = response.add_attributes(res.attributes);
                response = response.add_submessages(res.messages);
            }
            Err(_) => {
                // Skip worlds with no pending revenue
                continue;
            }
        }
    }

    Ok(response)
}

// ========================================
// QUERY
// ========================================

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::UserBalance { user } => to_json_binary(&query_user_balance(deps, user)?),
        QueryMsg::PurchaseHistory {
            user,
            start_after,
            limit,
        } => to_json_binary(&query_purchase_history(deps, user, start_after, limit)?),
        QueryMsg::Session { session_id } => to_json_binary(&query_session(deps, session_id)?),
        QueryMsg::UserSessions { user, active_only } => {
            to_json_binary(&query_user_sessions(deps, user, active_only)?)
        }
        QueryMsg::WorldConfig { world_nft_id } => {
            to_json_binary(&query_world_config(deps, world_nft_id)?)
        }
        QueryMsg::WorldStats { world_nft_id } => {
            to_json_binary(&query_world_stats(deps, world_nft_id)?)
        }
        QueryMsg::PendingRevenue { world_nft_id } => {
            to_json_binary(&query_pending_revenue(deps, world_nft_id)?)
        }
        QueryMsg::ConversionRate {} => to_json_binary(&query_conversion_rate(deps)?),
        QueryMsg::PasgUtility {} => to_json_binary(&query_pasg_utility(deps)?),
        QueryMsg::PlatformStats {} => to_json_binary(&query_platform_stats(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admin: config.admin,
        split_router: config.split_router,
        registry: config.registry,
        backend_operator: config.backend_operator,
        pasg_denom: config.pasg_denom,
        points_per_pasg: config.points_per_pasg,
        pasg_utility: config.pasg_utility,
        fiat_oracle: config.fiat_oracle,
        stripe_webhook_validator: config.stripe_webhook_validator,
        paused: config.paused,
    })
}

fn query_user_balance(deps: Deps, user: String) -> StdResult<UserBalanceResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let balance = USER_BALANCES.load(deps.storage, &user_addr)?;

    Ok(UserBalanceResponse {
        user: balance.user,
        points_balance: balance.points_balance,
        total_deposited_pasg: balance.total_deposited_pasg,
        total_spent_points: balance.total_spent_points,
        total_sessions: balance.total_sessions,
    })
}

fn query_purchase_history(
    deps: Deps,
    user: String,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<PurchaseHistoryResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let limit = limit.unwrap_or(50).min(100) as usize;

    let purchases: Vec<PurchaseRecord> = PURCHASES
        .prefix(&user_addr)
        .range(
            deps.storage,
            start_after.map(Bound::exclusive),
            None,
            cosmwasm_std::Order::Ascending,
        )
        .take(limit)
        .filter_map(|item| {
            item.ok().map(|(_, purchase)| PurchaseRecord {
                id: purchase.id,
                timestamp: purchase.timestamp,
                purchase_type: purchase.purchase_type,
                amount_usd: purchase.amount_usd,
                amount_pasg: purchase.amount_pasg,
                points_received: purchase.points_received,
                transaction_id: purchase.transaction_id,
            })
        })
        .collect();

    Ok(PurchaseHistoryResponse { purchases })
}

fn query_session(deps: Deps, session_id: u64) -> StdResult<SessionResponse> {
    let session = SESSIONS.load(deps.storage, session_id)?;
    Ok(SessionResponse {
        session_id: session.session_id,
        user: session.user,
        world_nft_id: session.world_nft_id,
        world_collection: session.world_collection,
        start_time: session.start_time,
        end_time: session.end_time,
        points_rate_per_hour: session.points_rate_per_hour,
        points_charged: session.points_charged,
        status: session.status,
    })
}

fn query_user_sessions(
    deps: Deps,
    user: String,
    active_only: Option<bool>,
) -> StdResult<UserSessionsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let active_only = active_only.unwrap_or(false);

    let sessions: Vec<SessionResponse> = USER_SESSIONS
        .prefix(&user_addr)
        .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
        .filter_map(|item| item.ok())
        .filter_map(|(session_id, _)| SESSIONS.load(deps.storage, session_id).ok())
        .filter(|session| {
            if active_only {
                session.status == SessionStatus::Active
            } else {
                true
            }
        })
        .map(|session| SessionResponse {
            session_id: session.session_id,
            user: session.user,
            world_nft_id: session.world_nft_id,
            world_collection: session.world_collection,
            start_time: session.start_time,
            end_time: session.end_time,
            points_rate_per_hour: session.points_rate_per_hour,
            points_charged: session.points_charged,
            status: session.status,
        })
        .collect();

    Ok(UserSessionsResponse { sessions })
}

fn query_world_config(deps: Deps, world_nft_id: String) -> StdResult<WorldConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let world_config = WORLD_CONFIGS.load(deps.storage, &world_nft_id)?;

    let pasg_per_hour = world_config
        .points_per_hour
        .checked_div(config.points_per_pasg)
        .unwrap_or(Uint128::zero());

    Ok(WorldConfigResponse {
        world_nft_id: world_config.world_nft_id,
        world_collection: world_config.world_collection,
        owner: world_config.owner,
        points_per_hour: world_config.points_per_hour,
        pasg_per_hour,
        active: world_config.active,
    })
}

fn query_world_stats(deps: Deps, world_nft_id: String) -> StdResult<WorldStatsResponse> {
    let stats = WORLD_STATS
        .may_load(deps.storage, &world_nft_id)?
        .unwrap_or(WorldStats {
            world_nft_id: world_nft_id.clone(),
            total_sessions: 0,
            total_points_earned: Uint128::zero(),
            total_pasg_earned: Uint128::zero(),
            unique_users: 0,
            total_duration_seconds: 0,
        });

    let avg_duration = if stats.total_sessions > 0 {
        stats.total_duration_seconds / stats.total_sessions
    } else {
        0
    };

    Ok(WorldStatsResponse {
        world_nft_id: stats.world_nft_id,
        total_sessions: stats.total_sessions,
        total_points_earned: stats.total_points_earned,
        total_pasg_earned: stats.total_pasg_earned,
        unique_users: stats.unique_users,
        average_session_duration_seconds: avg_duration,
    })
}

fn query_pending_revenue(deps: Deps, world_nft_id: String) -> StdResult<PendingRevenueResponse> {
    let pending = PENDING_REVENUE
        .may_load(deps.storage, &world_nft_id)?
        .unwrap_or(PendingRevenue {
            world_nft_id: world_nft_id.clone(),
            pending_points: Uint128::zero(),
            pending_pasg: Uint128::zero(),
            last_distribution: None,
        });

    Ok(PendingRevenueResponse {
        world_nft_id: pending.world_nft_id,
        pending_points: pending.pending_points,
        pending_pasg: pending.pending_pasg,
        last_distribution: pending.last_distribution,
    })
}

fn query_conversion_rate(deps: Deps) -> StdResult<ConversionRateResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConversionRateResponse {
        points_per_pasg: config.points_per_pasg,
        pasg_per_point: pasg_per_point(config.points_per_pasg),
        pasg_denom: config.pasg_denom,
    })
}

fn query_pasg_utility(deps: Deps) -> StdResult<PasgUtilityResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(PasgUtilityResponse {
        canonical_denom: config.pasg_denom.clone(),
        points_per_pasg: config.points_per_pasg,
        pasg_per_point: pasg_per_point(config.points_per_pasg),
        metadata: config.pasg_utility,
        canonical_query: PasgUtilityQueryRoute::PasgUtility,
        canonical_execute: default_pasg_execute_routes(),
        compatibility_router: PasgCompatibilityRouterResponse {
            contract: config.split_router,
            forwards_native_denom: true,
            execute_route: PasgCompatibilityRouterExecuteRoute::RouteWorldRevenue,
        },
        scope_boundary: PasgScopeBoundaryResponse {
            settlement: PasgBusinessBoundary::OnChainUtilitySurface,
            platform_billing: PasgBusinessBoundary::OffChainService,
            subscriptions: PasgBusinessBoundary::OffChainService,
        },
    })
}

fn query_platform_stats(deps: Deps) -> StdResult<PlatformStatsResponse> {
    let stats = PLATFORM_STATS.load(deps.storage)?;

    Ok(PlatformStatsResponse {
        total_users: stats.total_users,
        total_sessions: stats.total_sessions,
        total_points_issued: stats.total_points_issued,
        total_pasg_volume: stats.total_pasg_volume,
        active_sessions: stats.active_sessions,
    })
}
