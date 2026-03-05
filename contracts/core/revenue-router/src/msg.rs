use crate::state::{
    CollectionStats, Config, DistributionRule, EcosystemConfig, RevenueEvent, RevenueEventType,
    SplitWallet,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    /// Admin address
    pub admin: Option<String>,
    /// Platform fee collector address
    pub platform_fee_collector: String,
    /// Default platform fee (e.g., "0.025" for 2.5%)
    pub default_platform_fee: Decimal,
    /// Registry contract address
    pub registry: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ========== Admin Operations ==========
    /// Update contract configuration
    UpdateConfig {
        admin: Option<String>,
        platform_fee_collector: Option<String>,
        default_platform_fee: Option<Decimal>,
        registry: Option<String>,
        paused: Option<bool>,
    },

    // ========== Distribution Rules ==========
    /// Set distribution rule for a collection
    SetDistributionRule {
        collection: String,
        creator: String,
        creator_share: Decimal,
        platform_fee: Option<Decimal>,
        collaborators: Option<Vec<CollaboratorInput>>,
        royalty_pool: Option<String>,
    },
    /// Update existing distribution rule
    UpdateDistributionRule {
        collection: String,
        creator: Option<String>,
        creator_share: Option<Decimal>,
        platform_fee: Option<Decimal>,
        collaborators: Option<Vec<CollaboratorInput>>,
        royalty_pool: Option<String>,
        active: Option<bool>,
    },
    /// Remove distribution rule
    RemoveDistributionRule { collection: String },

    // ========== Ecosystem Configuration ==========
    /// Set ecosystem-level configuration
    SetEcosystemConfig {
        ecosystem_id: String,
        treasury: Option<String>,
    },

    // ========== Revenue Routing ==========
    /// Route revenue for a primary sale (minting)
    /// Funds must be sent with this message
    RoutePrimarySale { collection: String },
    /// Route revenue for a secondary sale (marketplace)
    /// Funds must be sent with this message
    RouteSecondarySale {
        collection: String,
        seller: String,
        royalty_amount: Uint128,
    },
    /// Route revenue for an auction sale
    RouteAuctionSale {
        collection: String,
        seller: String,
        royalty_amount: Uint128,
    },
    /// Generic route funds with custom type
    RouteRevenue {
        collection: String,
        event_type: RevenueEventType,
        custom_recipients: Option<Vec<RecipientInput>>,
    },

    // ========== Split Wallets ==========
    /// Create a new split wallet
    CreateSplitWallet {
        id: String,
        recipients: Vec<SplitRecipientInput>,
    },
    /// Update split wallet recipients
    UpdateSplitWallet {
        id: String,
        recipients: Vec<SplitRecipientInput>,
    },
    /// Distribute funds to split wallet recipients
    DistributeSplitWallet { id: String },
    /// Remove a split wallet
    RemoveSplitWallet { id: String },
}

#[cw_serde]
pub struct CollaboratorInput {
    pub address: String,
    pub share: Decimal,
    pub name: Option<String>,
}

#[cw_serde]
pub struct RecipientInput {
    pub address: String,
    pub share: Decimal,
}

#[cw_serde]
pub struct SplitRecipientInput {
    pub address: String,
    pub weight: u64,
    pub name: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},

    /// Get distribution rule for a collection
    #[returns(DistributionRuleResponse)]
    DistributionRule { collection: String },

    /// List all distribution rules
    #[returns(DistributionRulesResponse)]
    DistributionRules {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Get ecosystem configuration
    #[returns(EcosystemConfigResponse)]
    EcosystemConfig { ecosystem_id: String },

    /// Calculate distribution preview (without executing)
    #[returns(DistributionPreviewResponse)]
    PreviewDistribution {
        collection: String,
        amount: Uint128,
        event_type: RevenueEventType,
    },

    /// Get collection statistics
    #[returns(CollectionStatsResponse)]
    CollectionStats { collection: String },

    /// Get recent revenue events
    #[returns(RevenueEventsResponse)]
    RevenueEvents {
        collection: Option<String>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },

    /// Get split wallet
    #[returns(SplitWalletResponse)]
    SplitWallet { id: String },

    /// List all split wallets
    #[returns(SplitWalletsResponse)]
    SplitWallets {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

// ========== Response Types ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct DistributionRuleResponse {
    pub rule: Option<DistributionRule>,
}

#[cw_serde]
pub struct DistributionRulesResponse {
    pub rules: Vec<DistributionRule>,
}

#[cw_serde]
pub struct EcosystemConfigResponse {
    pub config: Option<EcosystemConfig>,
}

#[cw_serde]
pub struct DistributionPreviewResponse {
    pub total_amount: Uint128,
    pub platform_fee: Uint128,
    pub creator_amount: Uint128,
    pub collaborator_amounts: Vec<(Addr, Uint128)>,
    pub royalty_amount: Option<Uint128>,
}

#[cw_serde]
pub struct CollectionStatsResponse {
    pub stats: CollectionStats,
}

#[cw_serde]
pub struct RevenueEventsResponse {
    pub events: Vec<RevenueEvent>,
}

#[cw_serde]
pub struct SplitWalletResponse {
    pub wallet: Option<SplitWallet>,
}

#[cw_serde]
pub struct SplitWalletsResponse {
    pub wallets: Vec<SplitWallet>,
}
