use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub split_router: String,
    pub registry: String,
    pub backend_operator: Option<String>,

    // Payment configuration
    pub denom: String,  // e.g., "upasg"
    pub points_per_denom: Uint128,  // e.g., 100 points = 1 PASG

    // Fiat integration
    pub fiat_oracle: Option<String>,  // Off-chain service that reports fiat → crypto conversions
    pub stripe_webhook_validator: Option<String>,  // Contract that validates Stripe webhooks
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Update contract configuration (admin only)
    UpdateConfig {
        admin: Option<String>,
        split_router: Option<String>,
        backend_operator: Option<String>,
        fiat_oracle: Option<String>,
        stripe_webhook_validator: Option<String>,
        paused: Option<bool>,
    },

    // ========================================
    // CREDIT/POINTS MANAGEMENT
    // ========================================

    /// User deposits PASG directly to buy streaming points
    /// Payment comes in msg.funds
    DepositCrypto {},

    /// Off-chain service reports fiat purchase (Stripe payment completed)
    /// Only callable by fiat_oracle
    ReportFiatPurchase {
        user: String,
        fiat_amount_usd: Uint128,  // Amount paid in USD cents (e.g., 1000 = $10.00)
        pasg_amount: Uint128,       // Equivalent PASG amount (after conversion)
        points_awarded: Uint128,    // Points to credit
        transaction_id: String,     // Stripe payment intent ID
        timestamp: Timestamp,
    },

    /// User withdraws unused points back to PASG (with small fee)
    WithdrawPoints {
        points: Uint128,
    },

    // ========================================
    // STREAMING SESSIONS
    // ========================================

    /// Start a streaming session (called by backend)
    StartSession {
        user: String,
        world_nft_id: String,
        world_collection: String,
    },

    /// Stop session and charge user
    StopSession {
        session_id: u64,
        duration_seconds: u64,
    },

    /// Force stop session (admin only, for emergencies)
    ForceStopSession {
        session_id: u64,
    },

    // ========================================
    // WORLD CONFIGURATION
    // ========================================

    /// Set streaming rate for a world (called by world owner or admin)
    SetWorldRate {
        world_nft_id: String,
        world_collection: String,
        points_per_hour: Uint128,
    },

    /// Update world streaming rate
    UpdateWorldRate {
        world_nft_id: String,
        points_per_hour: Uint128,
    },

    // ========================================
    // REVENUE DISTRIBUTION
    // ========================================

    /// Distribute accumulated revenue for a world (anyone can call)
    DistributeWorldRevenue {
        world_nft_id: String,
    },

    /// Batch distribute for multiple worlds
    BatchDistributeRevenue {
        world_nft_ids: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},

    /// Get user's streaming points balance
    #[returns(UserBalanceResponse)]
    UserBalance { user: String },

    /// Get user's purchase history
    #[returns(PurchaseHistoryResponse)]
    PurchaseHistory {
        user: String,
        start_after: Option<u64>,
        limit: Option<u32>,
    },

    /// Get active streaming session
    #[returns(SessionResponse)]
    Session { session_id: u64 },

    /// Get all active sessions for a user
    #[returns(UserSessionsResponse)]
    UserSessions {
        user: String,
        active_only: Option<bool>,
    },

    /// Get world streaming configuration
    #[returns(WorldConfigResponse)]
    WorldConfig { world_nft_id: String },

    /// Get world revenue statistics
    #[returns(WorldStatsResponse)]
    WorldStats { world_nft_id: String },

    /// Get accumulated revenue pending distribution
    #[returns(PendingRevenueResponse)]
    PendingRevenue { world_nft_id: String },

    /// Get conversion rate (points per PASG)
    #[returns(ConversionRateResponse)]
    ConversionRate {},

    /// Get total platform statistics
    #[returns(PlatformStatsResponse)]
    PlatformStats {},
}

// ========================================
// RESPONSE TYPES
// ========================================

#[cw_serde]
pub struct ConfigResponse {
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
pub struct UserBalanceResponse {
    pub user: Addr,
    pub points_balance: Uint128,
    pub total_deposited_pasg: Uint128,
    pub total_spent_points: Uint128,
    pub total_sessions: u64,
}

#[cw_serde]
pub struct PurchaseRecord {
    pub id: u64,
    pub timestamp: Timestamp,
    pub purchase_type: PurchaseType,
    pub amount_usd: Option<Uint128>,   // If fiat purchase
    pub amount_pasg: Uint128,           // Crypto amount
    pub points_received: Uint128,
    pub transaction_id: Option<String>, // Stripe ID or tx hash
}

#[cw_serde]
pub enum PurchaseType {
    CryptoDirect,      // User deposited PASG directly
    FiatConverted,     // User paid with credit card → converted to PASG
}

#[cw_serde]
pub struct PurchaseHistoryResponse {
    pub purchases: Vec<PurchaseRecord>,
}

#[cw_serde]
pub struct SessionResponse {
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
pub enum SessionStatus {
    Active,
    Completed,
    ForceStopped,
}

#[cw_serde]
pub struct UserSessionsResponse {
    pub sessions: Vec<SessionResponse>,
}

#[cw_serde]
pub struct WorldConfigResponse {
    pub world_nft_id: String,
    pub world_collection: Addr,
    pub owner: Addr,
    pub points_per_hour: Uint128,
    pub pasg_per_hour: Uint128,  // Calculated from points
    pub active: bool,
}

#[cw_serde]
pub struct WorldStatsResponse {
    pub world_nft_id: String,
    pub total_sessions: u64,
    pub total_points_earned: Uint128,
    pub total_pasg_earned: Uint128,
    pub unique_users: u64,
    pub average_session_duration_seconds: u64,
}

#[cw_serde]
pub struct PendingRevenueResponse {
    pub world_nft_id: String,
    pub pending_points: Uint128,
    pub pending_pasg: Uint128,
    pub last_distribution: Option<Timestamp>,
}

#[cw_serde]
pub struct ConversionRateResponse {
    pub points_per_pasg: Uint128,
    pub pasg_per_point: Decimal,
    pub pasg_denom: String,
}

#[cw_serde]
pub struct PlatformStatsResponse {
    pub total_users: u64,
    pub total_sessions: u64,
    pub total_points_issued: Uint128,
    pub total_pasg_volume: Uint128,
    pub active_sessions: u64,
}

#[cw_serde]
pub enum RegistryQueryMsg {
    Collection { address: String },
}

#[cw_serde]
pub struct RegistryCollectionResponse {
    pub collection: Option<RegistryCollection>,
}

#[cw_serde]
pub struct RegistryCollection {
    pub address: Addr,
}

#[cw_serde]
pub enum Cw721QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
}

#[cw_serde]
pub struct OwnerOfResponse {
    pub owner: String,
}

// ========================================
// MIGRATE
// ========================================

#[cw_serde]
pub struct MigrateMsg {}
