use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::msg::{PurchaseType, SessionStatus};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub split_router: Addr,
    pub registry: Addr,
    pub backend_operator: Option<Addr>,
    pub pasg_denom: String,
    pub points_per_pasg: Uint128,
    pub fiat_oracle: Option<Addr>,
    pub stripe_webhook_validator: Option<Addr>,
    pub paused: bool,
}

#[cw_serde]
pub struct UserBalance {
    pub user: Addr,
    pub points_balance: Uint128,
    pub total_deposited_pasg: Uint128,
    pub total_spent_points: Uint128,
    pub total_sessions: u64,
    pub last_activity: Timestamp,
}

#[cw_serde]
pub struct Purchase {
    pub id: u64,
    pub user: Addr,
    pub timestamp: Timestamp,
    pub purchase_type: PurchaseType,
    pub amount_usd: Option<Uint128>,
    pub amount_pasg: Uint128,
    pub points_received: Uint128,
    pub transaction_id: Option<String>,
}

#[cw_serde]
pub struct StreamingSession {
    pub session_id: u64,
    pub user: Addr,
    pub world_nft_id: String,
    pub world_collection: Addr,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub points_rate_per_hour: Uint128,
    pub points_charged: Uint128,
    pub status: SessionStatus,
}

#[cw_serde]
pub struct WorldConfig {
    pub world_nft_id: String,
    pub world_collection: Addr,
    pub owner: Addr,
    pub points_per_hour: Uint128,
    pub active: bool,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct WorldStats {
    pub world_nft_id: String,
    pub total_sessions: u64,
    pub total_points_earned: Uint128,
    pub total_pasg_earned: Uint128,
    pub unique_users: u64,
    pub total_duration_seconds: u64,
}

#[cw_serde]
pub struct PendingRevenue {
    pub world_nft_id: String,
    pub pending_points: Uint128,
    pub pending_pasg: Uint128,
    pub last_distribution: Option<Timestamp>,
}

#[cw_serde]
pub struct PlatformStats {
    pub total_users: u64,
    pub total_sessions: u64,
    pub total_points_issued: Uint128,
    pub total_pasg_volume: Uint128,
    pub active_sessions: u64,
}

// ========================================
// STORAGE
// ========================================

pub const CONFIG: Item<Config> = Item::new("config");

/// User balances: user_addr -> UserBalance
pub const USER_BALANCES: Map<&Addr, UserBalance> = Map::new("user_balances");

/// Purchase history: (user_addr, purchase_id) -> Purchase
pub const PURCHASES: Map<(&Addr, u64), Purchase> = Map::new("purchases");

/// Purchase counter per user: user_addr -> count
pub const PURCHASE_COUNTER: Map<&Addr, u64> = Map::new("purchase_counter");

/// Global fiat purchase transaction index: transaction_id -> seen
pub const FIAT_PURCHASE_TX_IDS: Map<&str, bool> = Map::new("fiat_purchase_tx_ids");

/// Active streaming sessions: session_id -> StreamingSession
pub const SESSIONS: Map<u64, StreamingSession> = Map::new("sessions");

/// Session counter (global)
pub const SESSION_COUNTER: Item<u64> = Item::new("session_counter");

/// User sessions index: (user_addr, session_id) -> ()
pub const USER_SESSIONS: Map<(&Addr, u64), ()> = Map::new("user_sessions");

/// World streaming configuration: world_nft_id -> WorldConfig
pub const WORLD_CONFIGS: Map<&str, WorldConfig> = Map::new("world_configs");

/// World statistics: world_nft_id -> WorldStats
pub const WORLD_STATS: Map<&str, WorldStats> = Map::new("world_stats");

/// Pending revenue for distribution: world_nft_id -> PendingRevenue
pub const PENDING_REVENUE: Map<&str, PendingRevenue> = Map::new("pending_revenue");

/// Platform-wide statistics
pub const PLATFORM_STATS: Item<PlatformStats> = Item::new("platform_stats");

/// Unique users per world: (world_nft_id, user_addr) -> ()
pub const WORLD_USERS: Map<(&str, &Addr), ()> = Map::new("world_users");
