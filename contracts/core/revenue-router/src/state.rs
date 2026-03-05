use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contract configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address with full control
    pub admin: Addr,
    /// Platform fee collector address (Passage treasury)
    pub platform_fee_collector: Addr,
    /// Default platform fee percentage (e.g., 0.025 = 2.5%)
    pub default_platform_fee: Decimal,
    /// Registry contract address for verification
    pub registry: Option<Addr>,
    /// Whether the contract is paused
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Revenue distribution rule for a specific collection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DistributionRule {
    /// Collection address this rule applies to
    pub collection: Addr,
    /// Platform fee override (if None, use default)
    pub platform_fee: Option<Decimal>,
    /// Creator/primary recipient address
    pub creator: Addr,
    /// Creator's share after platform fee (e.g., 0.95 = 95%)
    pub creator_share: Decimal,
    /// Optional collaborator splits (shares from creator's portion)
    pub collaborators: Vec<Collaborator>,
    /// Optional royalty pool address for secondary sales
    pub royalty_pool: Option<Addr>,
    /// Whether this rule is active
    pub active: bool,
    /// Created timestamp
    pub created_at: u64,
    /// Updated timestamp
    pub updated_at: u64,
}

/// Collaborator receiving a portion of creator's share
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collaborator {
    pub address: Addr,
    /// Share of the creator's portion (e.g., 0.10 = 10% of creator share)
    pub share: Decimal,
    pub name: Option<String>,
}

/// Key: collection address
pub const DISTRIBUTION_RULES: Map<Addr, DistributionRule> = Map::new("dist_rules");

/// Ecosystem-level default distribution settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EcosystemConfig {
    pub ecosystem_id: String,
    /// Ecosystem treasury address
    pub treasury: Option<Addr>,
}

/// Key: ecosystem_id
pub const ECOSYSTEM_CONFIGS: Map<String, EcosystemConfig> = Map::new("ecosystem_configs");

/// Represents a revenue event for tracking/auditing
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RevenueEvent {
    pub id: u64,
    pub collection: Addr,
    pub event_type: RevenueEventType,
    pub total_amount: Uint128,
    pub denom: String,
    pub platform_fee: Uint128,
    pub creator_amount: Uint128,
    pub collaborator_amounts: Vec<(Addr, Uint128)>,
    pub timestamp: u64,
    pub tx_sender: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum RevenueEventType {
    PrimarySale,   // Minting
    SecondarySale, // Marketplace
    Auction,       // Auction sale
    Royalty,       // Royalty payment
    Other,
}

/// Counter for revenue events
pub const REVENUE_EVENT_COUNT: Item<u64> = Item::new("rev_event_count");

/// Recent revenue events (limited storage)
pub const REVENUE_EVENTS: Map<u64, RevenueEvent> = Map::new("rev_events");

/// Accumulated statistics per collection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct CollectionStats {
    pub total_primary_volume: Uint128,
    pub total_secondary_volume: Uint128,
    pub total_platform_fees: Uint128,
    pub total_creator_earnings: Uint128,
    pub total_royalties: Uint128,
    pub event_count: u64,
}

/// Key: collection address
pub const COLLECTION_STATS: Map<Addr, CollectionStats> = Map::new("coll_stats");

/// Split wallet configuration (evolved from royalty-group)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitWallet {
    pub id: String,
    pub admin: Addr,
    pub recipients: Vec<SplitRecipient>,
    pub total_weight: u64,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitRecipient {
    pub address: Addr,
    pub weight: u64,
    pub name: Option<String>,
}

/// Key: split wallet id
pub const SPLIT_WALLETS: Map<String, SplitWallet> = Map::new("split_wallets");
