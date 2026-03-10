use crate::state::{
    Ask, Bid, CollectionBid, CollectionConfig, CollectionStats, Config, MarketStats, TokenId,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

/// Migration message from marketplace-v2 to v3
#[cw_serde]
pub struct MigrateMsg {
    /// The collection address from marketplace-v2 (single collection)
    /// This is required to migrate asks/bids to multi-collection format
    pub collection: String,
    /// Optional: Registry contract address
    pub registry: Option<String>,
    /// Optional: Split Router address for creator-side royalty distribution
    pub split_router: Option<String>,
    /// Whether to use Split Router (default: true if split_router is set)
    pub use_split_router: Option<bool>,
    /// Additional collections to support (besides the migrated one)
    pub additional_collections: Option<Vec<String>>,
    /// Optional per-collection denom overrides
    pub collection_denoms: Option<Vec<CollectionDenomInput>>,
}

#[cw_serde]
pub struct CollectionDenomInput {
    pub collection: String,
    pub denom: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// Admin address
    pub admin: Option<String>,
    /// Token denom for payments
    pub denom: String,
    /// Minimum listing price
    pub min_price: Uint128,
    /// Default trading fee in basis points (e.g., 250 = 2.5%)
    pub trading_fee_bps: u64,
    /// Maximum allowed trading fee in basis points (e.g., 1000 = 10%)
    pub max_trading_fee_bps: Option<u64>,
    /// Fee collector address (legacy mode)
    pub fee_collector: String,
    /// Registry contract address (for collection verification)
    pub registry: Option<String>,
    /// Split Router address
    pub split_router: Option<String>,
    /// Whether to use Split Router
    pub use_split_router: Option<bool>,
    /// Operator addresses
    pub operators: Option<Vec<String>>,
    /// Whether to require collection registration (default: true)
    pub require_registration: Option<bool>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ========== Admin Operations ==========
    /// Update contract configuration
    UpdateConfig {
        admin: Option<String>,
        denom: Option<String>,
        min_price: Option<Uint128>,
        trading_fee_bps: Option<u64>,
        max_trading_fee_bps: Option<u64>,
        fee_collector: Option<String>,
        registry: Option<String>,
        split_router: Option<String>,
        use_split_router: Option<bool>,
        operators: Option<Vec<String>>,
        paused: Option<bool>,
        require_registration: Option<bool>,
    },

    // ========== Collection Registration ==========
    /// Register a new collection on the marketplace
    /// Can be called by admin, operators, or registry (if configured)
    RegisterCollection {
        collection: String,
        /// Custom trading fee for this collection (None = use default)
        trading_fee_bps: Option<u64>,
        /// Custom denom for this collection (None = use default)
        denom: Option<String>,
    },
    /// Update a registered collection's configuration
    UpdateCollectionConfig {
        collection: String,
        /// Enable/disable the collection
        active: Option<bool>,
        /// Custom trading fee (None to clear override and use default)
        trading_fee_bps: Option<u64>,
        /// Custom denom (None to clear override and use default)
        denom: Option<String>,
    },
    /// Deactivate a collection (soft removal, keeps data)
    DeactivateCollection {
        collection: String,
        reason: Option<String>,
    },
    /// Reactivate a previously deactivated collection
    ReactivateCollection { collection: String },

    // ========== Listing Operations ==========
    /// Create a listing (ask) for an NFT
    SetAsk {
        collection: String,
        token_id: TokenId,
        price: Coin,
        funds_recipient: Option<String>,
    },
    /// Update an existing listing
    UpdateAsk {
        collection: String,
        token_id: TokenId,
        price: Coin,
    },
    /// Remove a listing
    RemoveAsk {
        collection: String,
        token_id: TokenId,
    },
    /// Buy a listed NFT (must send exact price)
    BuyNow {
        collection: String,
        token_id: TokenId,
    },

    // ========== Bid Operations ==========
    /// Place a bid on a specific NFT
    SetBid {
        collection: String,
        token_id: TokenId,
        price: Coin,
        expires_at: Option<u64>,
    },
    /// Remove a bid
    RemoveBid {
        collection: String,
        token_id: TokenId,
    },
    /// Accept a bid (seller action)
    AcceptBid {
        collection: String,
        token_id: TokenId,
        bidder: String,
    },

    // ========== Collection Bid Operations ==========
    /// Place a collection-wide bid
    SetCollectionBid {
        collection: String,
        units: u32,
        price: Coin,
        expires_at: Option<u64>,
    },
    /// Remove a collection bid
    RemoveCollectionBid { collection: String },
    /// Accept a collection bid (seller action)
    AcceptCollectionBid {
        collection: String,
        token_id: TokenId,
        bidder: String,
    },

    // ========== Operator Operations ==========
    /// Sync ask state (deactivate if NFT transferred)
    SyncAsk {
        collection: String,
        token_id: TokenId,
    },
    /// Batch sync asks
    BatchSyncAsks { asks: Vec<(String, TokenId)> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},

    // ========== Collection Queries ==========
    /// Get a collection's configuration
    #[returns(CollectionConfigResponse)]
    CollectionConfig { collection: String },
    /// List all registered collections
    #[returns(CollectionConfigsResponse)]
    CollectionConfigs {
        start_after: Option<String>,
        limit: Option<u32>,
        /// Filter by active status
        active_only: Option<bool>,
    },
    /// Check if a collection is registered and can trade
    #[returns(CanTradeResponse)]
    CanTrade { collection: String },
    /// Get effective denom for a specific collection
    #[returns(CollectionDenomResponse)]
    CollectionDenom { collection: String },
    /// Get effective trading fee for a specific collection
    #[returns(CollectionFeeResponse)]
    CollectionFee { collection: String },

    // ========== Ask Queries ==========
    /// Get a specific ask
    #[returns(AskResponse)]
    Ask {
        collection: String,
        token_id: TokenId,
    },
    /// List asks by collection
    #[returns(AsksResponse)]
    AsksByCollection {
        collection: String,
        start_after: Option<TokenId>,
        limit: Option<u32>,
    },
    /// List asks by seller
    #[returns(AsksResponse)]
    AsksBySeller {
        seller: String,
        start_after: Option<(String, TokenId)>,
        limit: Option<u32>,
    },
    /// List asks sorted by price
    #[returns(AsksResponse)]
    AsksByPrice {
        collection: Option<String>,
        start_after: Option<u128>,
        limit: Option<u32>,
        descending: Option<bool>,
    },
    /// Count of active asks
    #[returns(CountResponse)]
    AskCount { collection: Option<String> },

    // ========== Bid Queries ==========
    /// Get a specific bid
    #[returns(BidResponse)]
    Bid {
        collection: String,
        token_id: TokenId,
        bidder: String,
    },
    /// List bids for a token
    #[returns(BidsResponse)]
    BidsByToken {
        collection: String,
        token_id: TokenId,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// List bids by bidder
    #[returns(BidsResponse)]
    BidsByBidder {
        bidder: String,
        start_after: Option<(String, TokenId)>,
        limit: Option<u32>,
    },

    // ========== Collection Bid Queries ==========
    /// Get a collection bid
    #[returns(CollectionBidResponse)]
    CollectionBid { collection: String, bidder: String },
    /// List collection bids
    #[returns(CollectionBidsResponse)]
    CollectionBidsByCollection {
        collection: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// List collection bids by bidder
    #[returns(CollectionBidsResponse)]
    CollectionBidsByBidder {
        bidder: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    // ========== Statistics ==========
    /// Get marketplace statistics
    #[returns(MarketStatsResponse)]
    MarketStats {},
    /// Get collection statistics
    #[returns(CollectionStatsResponse)]
    CollectionStats { collection: String },

    // ========== Utilities ==========
    /// Calculate sale proceeds (preview)
    #[returns(SalePreviewResponse)]
    PreviewSale { collection: String, price: Uint128 },
}

// ========== Response Types ==========

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub denom: String,
    pub min_price: Uint128,
    pub trading_fee_bps: u64,
    pub max_trading_fee_bps: u64,
    pub fee_collector: String,
    pub registry: Option<String>,
    pub split_router: Option<String>,
    pub use_split_router: bool,
    pub operators: Vec<String>,
    pub paused: bool,
    pub require_registration: bool,
}

impl From<Config> for ConfigResponse {
    fn from(c: Config) -> Self {
        ConfigResponse {
            admin: c.admin.to_string(),
            denom: c.denom,
            min_price: c.min_price,
            trading_fee_bps: c.trading_fee_bps,
            max_trading_fee_bps: c.max_trading_fee_bps,
            fee_collector: c.fee_collector.to_string(),
            registry: c.registry.map(|a| a.to_string()),
            split_router: c.split_router.map(|a| a.to_string()),
            use_split_router: c.use_split_router,
            operators: c.operators.iter().map(|a| a.to_string()).collect(),
            paused: c.paused,
            require_registration: c.require_registration,
        }
    }
}

#[cw_serde]
pub struct CollectionConfigResponse {
    pub config: Option<CollectionConfig>,
}

#[cw_serde]
pub struct CollectionConfigsResponse {
    pub configs: Vec<CollectionConfig>,
}

#[cw_serde]
pub struct CanTradeResponse {
    pub can_trade: bool,
    pub reason: Option<String>,
}

#[cw_serde]
pub struct CollectionDenomResponse {
    pub collection: String,
    pub denom: String,
    /// True when this denom is a collection-specific override.
    pub is_override: bool,
}

#[cw_serde]
pub struct CollectionFeeResponse {
    pub collection: String,
    pub trading_fee_bps: u64,
    /// True when this fee is a collection-specific override.
    pub is_override: bool,
}

#[cw_serde]
pub struct AskResponse {
    pub ask: Option<Ask>,
}

#[cw_serde]
pub struct AsksResponse {
    pub asks: Vec<Ask>,
}

#[cw_serde]
pub struct BidResponse {
    pub bid: Option<Bid>,
}

#[cw_serde]
pub struct BidsResponse {
    pub bids: Vec<Bid>,
}

#[cw_serde]
pub struct CollectionBidResponse {
    pub bid: Option<CollectionBid>,
}

#[cw_serde]
pub struct CollectionBidsResponse {
    pub bids: Vec<CollectionBid>,
}

#[cw_serde]
pub struct CountResponse {
    pub count: u64,
}

#[cw_serde]
pub struct MarketStatsResponse {
    pub stats: MarketStats,
}

#[cw_serde]
pub struct CollectionStatsResponse {
    pub stats: CollectionStats,
}

#[cw_serde]
pub struct SalePreviewResponse {
    pub sale_price: Uint128,
    pub trading_fee: Uint128,
    pub royalty: Uint128,
    pub seller_proceeds: Uint128,
}

// ========== External Messages ==========

#[cw_serde]
pub enum SplitRouterExecuteMsg {
    RouteSecondaryRoyalty { collection: String },
}

#[cw_serde]
pub enum Cw721ExecuteMsg {
    TransferNft { recipient: String, token_id: String },
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
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct Approval {
    pub spender: String,
    pub expires: Expiration,
}

#[cw_serde]
pub enum Expiration {
    AtHeight(u64),
    AtTime(cosmwasm_std::Timestamp),
    Never {},
}

/// Query pg721 for collection info including royalties
#[cw_serde]
pub enum Pg721QueryMsg {
    CollectionInfo {},
}

#[cw_serde]
pub struct CollectionInfoResponse {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<RoyaltyInfoResponse>,
}

#[cw_serde]
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: String, // Decimal as string
}

#[cw_serde]
pub enum RegistryQueryMsg {
    CanTradeCollection { address: String },
}

#[cw_serde]
pub struct RegistryApprovalStatusResponse {
    pub approved: bool,
}
