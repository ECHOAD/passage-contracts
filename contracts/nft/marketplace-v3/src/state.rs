use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contract configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address
    pub admin: Addr,
    /// Minimum price for listings
    pub min_price: Uint128,
    /// Marketplace-wide trading fee in basis points (e.g., 250 = 2.5%)
    pub trading_fee_bps: u64,
    /// Fee collector address (legacy mode)
    pub fee_collector: Addr,
    /// Registry contract address (required for collection verification)
    pub registry: Option<Addr>,
    /// Operators who can update ask states
    pub operators: Vec<Addr>,
    /// Whether the contract is paused
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

// ========== Collection Configuration ==========

/// Per-collection configuration for the global marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionConfig {
    /// Collection address
    pub collection: Addr,
    /// Whether the collection is active on the marketplace
    pub active: bool,
    /// Settlement denom for this collection
    pub denom: String,
    /// Who registered this collection
    pub registered_by: Addr,
    /// When the collection was registered
    pub registered_at: u64,
    /// When the collection config was last updated
    pub updated_at: u64,
}

impl CollectionConfig {
    /// Get the configured denom for this collection
    pub fn get_denom(&self) -> String {
        self.denom.clone()
    }

    /// Check if collection can be traded
    pub fn can_trade(&self) -> bool {
        self.active
    }
}

/// Key: collection address
pub const COLLECTION_CONFIGS: Map<Addr, CollectionConfig> = Map::new("coll_configs");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CollectionRequestStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionRegistrationRequest {
    pub collection: Addr,
    pub requester: Addr,
    pub denom: String,
    pub note: Option<String>,
    pub status: CollectionRequestStatus,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionUpdateRequest {
    pub collection: Addr,
    pub requester: Addr,
    pub active: Option<bool>,
    pub denom: Option<String>,
    pub note: Option<String>,
    pub status: CollectionRequestStatus,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub const COLLECTION_REGISTRATION_REQUESTS: Map<Addr, CollectionRegistrationRequest> =
    Map::new("coll_reg_requests");
pub const COLLECTION_UPDATE_REQUESTS: Map<Addr, CollectionUpdateRequest> =
    Map::new("coll_update_requests");

pub type TokenId = String;

/// Ask (listing) on the marketplace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ask {
    pub collection: Addr,
    pub token_id: TokenId,
    pub seller: Addr,
    pub price: Coin,
    /// Optional recipient for funds (defaults to seller)
    pub funds_recipient: Option<Addr>,
    /// Ask creation timestamp
    pub created_at: u64,
    /// Whether the ask is active
    pub is_active: bool,
}

impl Ask {
    pub fn get_recipient(&self) -> Addr {
        self.funds_recipient.clone().unwrap_or(self.seller.clone())
    }
}

/// Primary key: (collection, token_id)
pub type AskKey = (Addr, TokenId);

/// Indices for asks
pub struct AskIndices<'a> {
    pub collection: MultiIndex<'a, Addr, Ask, AskKey>,
    pub seller: MultiIndex<'a, Addr, Ask, AskKey>,
    pub price: MultiIndex<'a, u128, Ask, AskKey>,
}

impl<'a> IndexList<Ask> for AskIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Ask>> + '_> {
        let v: Vec<&dyn Index<Ask>> = vec![&self.collection, &self.seller, &self.price];
        Box::new(v.into_iter())
    }
}

pub fn asks<'a>() -> IndexedMap<AskKey, Ask, AskIndices<'a>> {
    let indexes = AskIndices {
        collection: MultiIndex::new(
            |_pk: &[u8], d: &Ask| d.collection.clone(),
            "asks",
            "asks__collection",
        ),
        seller: MultiIndex::new(
            |_pk: &[u8], d: &Ask| d.seller.clone(),
            "asks",
            "asks__seller",
        ),
        price: MultiIndex::new(
            |_pk: &[u8], d: &Ask| d.price.amount.u128(),
            "asks",
            "asks__price",
        ),
    };
    IndexedMap::new("asks", indexes)
}

/// Bid (offer) on a specific NFT
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bid {
    pub collection: Addr,
    pub token_id: TokenId,
    pub bidder: Addr,
    pub price: Coin,
    /// Bid creation timestamp
    pub created_at: u64,
    /// Optional expiration timestamp
    pub expires_at: Option<u64>,
}

/// Primary key: (collection, token_id, bidder)
pub type BidKey = (Addr, TokenId, Addr);

/// Indices for bids
pub struct BidIndices<'a> {
    pub token: MultiIndex<'a, (Addr, String), Bid, BidKey>,
    pub bidder: MultiIndex<'a, Addr, Bid, BidKey>,
    pub price: MultiIndex<'a, u128, Bid, BidKey>,
}

impl<'a> IndexList<Bid> for BidIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Bid>> + '_> {
        let v: Vec<&dyn Index<Bid>> = vec![&self.token, &self.bidder, &self.price];
        Box::new(v.into_iter())
    }
}

pub fn bids<'a>() -> IndexedMap<BidKey, Bid, BidIndices<'a>> {
    let indexes = BidIndices {
        token: MultiIndex::new(
            |_pk: &[u8], d: &Bid| (d.collection.clone(), d.token_id.clone()),
            "bids",
            "bids__token",
        ),
        bidder: MultiIndex::new(
            |_pk: &[u8], d: &Bid| d.bidder.clone(),
            "bids",
            "bids__bidder",
        ),
        price: MultiIndex::new(
            |_pk: &[u8], d: &Bid| d.price.amount.u128(),
            "bids",
            "bids__price",
        ),
    };
    IndexedMap::new("bids", indexes)
}

/// Collection-wide bid (offer to buy any NFT from collection)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionBid {
    pub collection: Addr,
    pub bidder: Addr,
    /// Number of NFTs to buy
    pub units: u32,
    /// Price per NFT
    pub price: Coin,
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

impl CollectionBid {
    pub fn total_cost(&self) -> Uint128 {
        self.price.amount * Uint128::from(self.units)
    }
}

/// Primary key: (collection, bidder)
pub type CollectionBidKey = (Addr, Addr);

/// Indices for collection bids
pub struct CollectionBidIndices<'a> {
    pub collection: MultiIndex<'a, Addr, CollectionBid, CollectionBidKey>,
    pub bidder: MultiIndex<'a, Addr, CollectionBid, CollectionBidKey>,
    pub price: MultiIndex<'a, u128, CollectionBid, CollectionBidKey>,
}

impl<'a> IndexList<CollectionBid> for CollectionBidIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CollectionBid>> + '_> {
        let v: Vec<&dyn Index<CollectionBid>> = vec![&self.collection, &self.bidder, &self.price];
        Box::new(v.into_iter())
    }
}

pub fn collection_bids<'a>() -> IndexedMap<CollectionBidKey, CollectionBid, CollectionBidIndices<'a>>
{
    let indexes = CollectionBidIndices {
        collection: MultiIndex::new(
            |_pk: &[u8], d: &CollectionBid| d.collection.clone(),
            "col_bids",
            "col_bids__collection",
        ),
        bidder: MultiIndex::new(
            |_pk: &[u8], d: &CollectionBid| d.bidder.clone(),
            "col_bids",
            "col_bids__bidder",
        ),
        price: MultiIndex::new(
            |_pk: &[u8], d: &CollectionBid| d.price.amount.u128(),
            "col_bids",
            "col_bids__price",
        ),
    };
    IndexedMap::new("col_bids", indexes)
}

/// Cached royalty info for a collection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CachedRoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
    pub cached_at: u64,
}

/// Key: collection address
pub const ROYALTY_CACHE: Map<Addr, CachedRoyaltyInfo> = Map::new("royalty_cache");

/// Marketplace statistics
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct MarketStats {
    pub total_sales: u64,
    pub total_volume: Uint128,
    pub total_trading_fees: Uint128,
    pub total_royalties: Uint128,
}

pub const MARKET_STATS: Item<MarketStats> = Item::new("market_stats");

/// Per-collection statistics
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct CollectionStats {
    pub total_sales: u64,
    pub total_volume: Uint128,
    pub floor_price: Option<Uint128>,
    pub highest_sale: Uint128,
}

pub const COLLECTION_STATS: Map<Addr, CollectionStats> = Map::new("coll_stats");
