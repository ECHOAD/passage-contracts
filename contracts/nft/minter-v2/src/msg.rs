use crate::state::{Config, MintStats, Pg721InstantiateMsg};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Timestamp};

pub const CANONICAL_PASG_NATIVE_DENOM: &str = streaming_billing::msg::CANONICAL_PASG_DENOM;
pub const CANONICAL_PASG_UTILITY_QUERY: &str =
    streaming_billing::msg::CANONICAL_PASG_UTILITY_QUERY_ROUTE;

/// Migration message from minter v1 to minter-v2
#[cw_serde]
pub struct MigrateMsg {
    /// Registry contract address for collection verification
    pub registry: Option<String>,
    /// Required only for `passage-minter-metadata-onchain` migrations
    /// because that legacy state does not store a base token URI.
    pub base_token_uri: Option<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// Base URI for token metadata (e.g., "ipfs://...")
    pub base_token_uri: String,
    /// Total number of tokens available
    pub num_tokens: u32,
    /// Code ID for pg721 contract
    pub cw721_code_id: u64,
    /// Instantiate message for pg721
    pub cw721_instantiate_msg: Pg721InstantiateMsg,
    /// Minting start time
    pub start_time: Timestamp,
    /// Maximum mints per address
    pub per_address_limit: u32,
    /// Price per NFT
    pub unit_price: Coin,
    /// Optional whitelist contract address
    pub whitelist: Option<String>,
    /// Registry contract address
    pub registry: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ========== Minting ==========
    /// Mint a random token
    Mint {},
    /// Mint to a specific recipient (admin only)
    MintTo { recipient: String },
    /// Mint a specific token ID to a recipient (admin only)
    MintFor { token_id: u32, recipient: String },
    /// Batch mint multiple tokens
    BatchMint { count: u32 },

    // ========== Admin Operations ==========
    /// Update minting configuration
    UpdateConfig {
        admin: Option<String>,
        per_address_limit: Option<u32>,
        unit_price: Option<Coin>,
        whitelist: Option<String>,
        registry: Option<String>,
        paused: Option<bool>,
    },
    /// Update minting start time
    UpdateStartTime { start_time: Timestamp },
    /// Set whitelist contract
    SetWhitelist { whitelist: String },
    /// Remove whitelist
    RemoveWhitelist {},

    // ========== Legacy Compatibility ==========
    /// Withdraw accumulated funds held by the minter
    Withdraw {},
    /// Withdraw to specific address
    WithdrawTo { recipient: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},

    /// Get number of tokens still available for minting
    #[returns(MintableNumTokensResponse)]
    MintableNumTokens {},

    /// Get minting start time
    #[returns(StartTimeResponse)]
    StartTime {},

    /// Get current mint price (considers whitelist pricing)
    #[returns(MintPriceResponse)]
    MintPrice {},

    /// Get mint count for a specific address
    #[returns(MintCountResponse)]
    MintCount { address: String },

    /// Check if address can mint
    #[returns(CanMintResponse)]
    CanMint { address: String },

    /// Get minting statistics
    #[returns(MintStatsResponse)]
    MintStats {},

    /// Check if minting is active
    #[returns(IsMintingActiveResponse)]
    IsMintingActive {},
}

// ========== Response Types ==========

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub base_token_uri: String,
    pub num_tokens: u32,
    pub per_address_limit: u32,
    pub cw721_address: String,
    pub cw721_code_id: u64,
    pub start_time: Timestamp,
    pub unit_price: Coin,
    pub whitelist: Option<String>,
    pub registry: Option<String>,
    pub paused: bool,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> Self {
        ConfigResponse {
            admin: config.admin.to_string(),
            base_token_uri: config.base_token_uri,
            num_tokens: config.num_tokens,
            per_address_limit: config.per_address_limit,
            cw721_address: config.cw721_address.to_string(),
            cw721_code_id: config.cw721_code_id,
            start_time: config.start_time,
            unit_price: config.unit_price,
            whitelist: config.whitelist.map(|w| w.to_string()),
            registry: config.registry.map(|r| r.to_string()),
            paused: config.paused,
        }
    }
}

#[cw_serde]
pub struct MintableNumTokensResponse {
    pub count: u32,
}

#[cw_serde]
pub struct StartTimeResponse {
    pub start_time: Timestamp,
}

#[cw_serde]
pub struct MintPriceResponse {
    pub public_price: Coin,
    pub whitelist_price: Option<Coin>,
    pub current_price: Coin,
}

#[cw_serde]
pub struct MintCountResponse {
    pub address: String,
    pub count: u32,
}

#[cw_serde]
pub struct CanMintResponse {
    pub can_mint: bool,
    pub reason: Option<String>,
}

#[cw_serde]
pub struct MintStatsResponse {
    pub stats: MintStats,
}

#[cw_serde]
pub struct IsMintingActiveResponse {
    pub is_active: bool,
    pub reason: Option<String>,
}

// ========== Whitelist Query ==========

#[cw_serde]
pub enum WhitelistQueryMsg {
    Config {},
    HasMember { member: String },
}

#[cw_serde]
pub struct WhitelistConfigResponse {
    pub per_address_limit: u32,
    pub member_limit: u32,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub unit_price: Coin,
    pub is_active: bool,
}

#[cw_serde]
pub struct HasMemberResponse {
    pub has_member: bool,
}

// ========== Registry Query ==========

#[cw_serde]
pub enum RegistryQueryMsg {
    Collection {
        address: String,
    },
    CanMintCollection {
        address: String,
    },
    IsMinterAuthorized {
        collection_address: String,
        minter_address: String,
    },
}

#[cw_serde]
pub struct RegistryCollectionResponse {
    pub collection: Option<RegistryCollection>,
}

#[cw_serde]
pub struct RegistryCollection {
    pub creator: String,
}

#[cw_serde]
pub struct RegistryMinterAuthorizedResponse {
    pub is_authorized: bool,
}

#[cw_serde]
pub struct RegistryApprovalStatusResponse {
    pub approved: bool,
}
