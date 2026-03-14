use cosmwasm_std::{Addr, Coin, Decimal, Empty, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MetadataMode {
    /// Only store token_uri on-chain. Full metadata is resolved off-chain.
    OffChain,
    /// Store token_uri and optional native asset references on-chain.
    OnChain,
}

impl Default for MetadataMode {
    fn default() -> Self {
        Self::OffChain
    }
}

impl MetadataMode {
    pub fn uses_onchain_metadata(&self) -> bool {
        matches!(self, Self::OnChain)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftType {
    Component,
    Avatar,
    Companion,
    World,
    Plugin,
    Achievement,
    WorldTemplate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NativeAsset {
    pub asset_id: String,
    pub name: String,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMetadata {
    pub nft_type: NftType,
    pub native_assets: Option<Vec<NativeAsset>>,
    pub extension: Option<Empty>,
}

pub type Extension = Option<TokenMetadata>;

/// Contract configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address with full control
    pub admin: Addr,
    /// The pg721 NFT collection address
    pub cw721_address: Addr,
    /// The pg721 code ID used for instantiation
    pub cw721_code_id: u64,
    /// Base URI for token metadata
    pub base_token_uri: String,
    /// Total number of tokens available for minting
    pub num_tokens: u32,
    /// Price per NFT
    pub unit_price: Coin,
    /// Maximum mints per address
    pub per_address_limit: u32,
    /// Minting start time
    pub start_time: Timestamp,
    /// Optional whitelist contract address
    pub whitelist: Option<Addr>,
    /// Registry contract address for verification
    pub registry: Option<Addr>,
    /// Optional destination for mint proceeds. If it is a contract, the minter
    /// executes `Split {}` with the mint funds; otherwise it sends the funds directly.
    pub collector_address: Option<Addr>,
    /// Metadata storage mode for minted tokens.
    pub metadata_mode: MetadataMode,
    /// Default native dependents included on each minted NFT.
    pub native_asset_template: Vec<NativeAsset>,
    /// Whether minting is paused
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Tracks number of mints per address
pub const MINTER_ADDRS: Map<&Addr, u32> = Map::new("minter_addrs");

/// Available token IDs for minting
pub const MINTABLE_TOKEN_IDS: Map<u32, bool> = Map::new("mintable_ids");

/// Optional token-specific native asset overrides.
/// If present for a token_id, this replaces the default template at mint time.
pub const TOKEN_NATIVE_ASSET_OVERRIDES: Map<u32, Vec<NativeAsset>> =
    Map::new("native_asset_overrides");

/// Counter for tracking total minted
pub const MINTABLE_NUM_TOKENS: Item<u32> = Item::new("mintable_num_tokens");

/// Minting statistics
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct MintStats {
    /// Total tokens minted
    pub total_minted: u32,
    /// Total revenue generated
    pub total_revenue: Uint128,
    /// Total revenue automatically forwarded out of the minter
    pub total_routed: Uint128,
    /// Number of unique minters
    pub unique_minters: u32,
}

pub const MINT_STATS: Item<MintStats> = Item::new("mint_stats");

/// Extension message for pg721 instantiation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pg721InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub nft_type: NftType,
    pub collection_info: CollectionInfoMsg<RoyaltyInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfoMsg<T> {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: Decimal,
}
