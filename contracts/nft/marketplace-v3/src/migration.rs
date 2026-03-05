//! Migration module for marketplace-v2 to marketplace-v3
//!
//! This module handles the state migration from the single-collection marketplace-v2
//! to the multi-collection marketplace-v3.

use cosmwasm_std::{Addr, Coin, Decimal, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{
    asks, bids, collection_bids, Ask as AskV3, Bid as BidV3, CollectionBid as CollectionBidV3,
    CollectionStats, Config, MarketStats, COLLECTION_DENOMS, COLLECTION_STATS, CONFIG,
    MARKET_STATS,
};

// ============================================================================
// V2 State Types (for reading old state)
// ============================================================================

/// Config from marketplace-v2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigV2 {
    pub cw721_address: Addr,
    pub denom: String,
    pub collector_address: Addr,
    pub trading_fee_percent: Decimal,
    pub operators: Vec<Addr>,
    pub min_price: Uint128,
}

/// Ask from marketplace-v2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AskV2 {
    pub token_id: String,
    pub seller: Addr,
    pub price: Coin,
    pub funds_recipient: Option<Addr>,
}

/// Bid from marketplace-v2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidV2 {
    pub token_id: String,
    pub bidder: Addr,
    pub price: Coin,
}

/// CollectionBid from marketplace-v2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionBidV2 {
    pub bidder: Addr,
    pub units: u32,
    pub price: Coin,
}

// ============================================================================
// V2 Storage Keys (must match exactly what v2 uses)
// ============================================================================

const CONFIG_V2: Item<ConfigV2> = Item::new("config");

type AskKeyV2 = String; // token_id

pub struct AskIndicesV2<'a> {
    pub price: MultiIndex<'a, u128, AskV2, AskKeyV2>,
    pub seller: MultiIndex<'a, Addr, AskV2, AskKeyV2>,
}

impl<'a> IndexList<AskV2> for AskIndicesV2<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<AskV2>> + '_> {
        let v: Vec<&dyn Index<AskV2>> = vec![&self.price, &self.seller];
        Box::new(v.into_iter())
    }
}

pub fn asks_v2<'a>() -> IndexedMap<AskKeyV2, AskV2, AskIndicesV2<'a>> {
    let indexes = AskIndicesV2 {
        price: MultiIndex::new(
            |_pk: &[u8], d: &AskV2| d.price.amount.u128(),
            "asks",
            "asks__price",
        ),
        seller: MultiIndex::new(
            |_pk: &[u8], d: &AskV2| d.seller.clone(),
            "asks",
            "asks__seller",
        ),
    };
    IndexedMap::new("asks", indexes)
}

type BidKeyV2 = (Addr, String); // (bidder, token_id)

pub struct BidIndicesV2<'a> {
    pub token_price: MultiIndex<'a, (String, u128), BidV2, BidKeyV2>,
}

impl<'a> IndexList<BidV2> for BidIndicesV2<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<BidV2>> + '_> {
        let v: Vec<&dyn Index<BidV2>> = vec![&self.token_price];
        Box::new(v.into_iter())
    }
}

pub fn bids_v2<'a>() -> IndexedMap<BidKeyV2, BidV2, BidIndicesV2<'a>> {
    let indexes = BidIndicesV2 {
        token_price: MultiIndex::new(
            |_pk: &[u8], d: &BidV2| (d.token_id.clone(), d.price.amount.u128()),
            "bids",
            "bids__token_price",
        ),
    };
    IndexedMap::new("bids", indexes)
}

type CollectionBidKeyV2 = Addr; // bidder

pub struct CollectionBidIndicesV2<'a> {
    pub price: MultiIndex<'a, u128, CollectionBidV2, CollectionBidKeyV2>,
}

impl<'a> IndexList<CollectionBidV2> for CollectionBidIndicesV2<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<CollectionBidV2>> + '_> {
        let v: Vec<&dyn Index<CollectionBidV2>> = vec![&self.price];
        Box::new(v.into_iter())
    }
}

pub fn collection_bids_v2<'a>(
) -> IndexedMap<CollectionBidKeyV2, CollectionBidV2, CollectionBidIndicesV2<'a>> {
    let indexes = CollectionBidIndicesV2 {
        price: MultiIndex::new(
            |_pk: &[u8], d: &CollectionBidV2| d.price.amount.u128(),
            "col_bids",
            "col_bids__price",
        ),
    };
    IndexedMap::new("col_bids", indexes)
}

// ============================================================================
// Migration Logic
// ============================================================================

/// Migrate state from marketplace-v2 to marketplace-v3
pub fn migrate_state(
    storage: &mut dyn Storage,
    collection: Addr,
    registry: Option<Addr>,
    revenue_router: Option<Addr>,
    use_revenue_router: bool,
    additional_collections: Vec<Addr>,
    current_time: u64,
) -> StdResult<MigrationStats> {
    let mut stats = MigrationStats::default();

    // 1. Read and migrate config
    let config_v2 = CONFIG_V2.load(storage)?;

    let mut supported_collections = vec![collection.clone()];
    for candidate in additional_collections {
        if !supported_collections.contains(&candidate) {
            supported_collections.push(candidate);
        }
    }

    // Convert trading_fee_percent (Decimal) to basis points
    // e.g., 0.025 (2.5%) -> 250 bps
    let trading_fee_bps = (config_v2.trading_fee_percent * Decimal::from_ratio(10000u128, 1u128))
        .to_uint_floor()
        .u128() as u64;

    let config_v3 = Config {
        admin: config_v2.collector_address.clone(), // Use collector as admin initially
        supported_collections,
        allow_any_collection: false,
        denom: config_v2.denom,
        min_price: config_v2.min_price,
        trading_fee_bps,
        fee_collector: config_v2.collector_address,
        registry,
        revenue_router: revenue_router.clone(),
        use_revenue_router,
        operators: config_v2.operators,
        paused: false,
    };

    CONFIG.save(storage, &config_v3)?;
    // Keep migrated collection denom explicit to preserve historical behavior even
    // if default config denom is updated later.
    COLLECTION_DENOMS.save(storage, collection.clone(), &config_v3.denom)?;

    // 2. Migrate asks
    let asks_list: Vec<(AskKeyV2, AskV2)> = asks_v2()
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    for (token_id, ask_v2) in asks_list {
        // Remove legacy v2 entry (and its indices) to avoid mixed key formats.
        asks_v2().remove(storage, token_id.clone())?;

        let ask_v3 = AskV3 {
            collection: collection.clone(),
            token_id: token_id.clone(),
            seller: ask_v2.seller,
            price: ask_v2.price,
            funds_recipient: ask_v2.funds_recipient,
            created_at: current_time,
            is_active: true,
        };

        let ask_key_v3 = (collection.clone(), token_id);
        asks().save(storage, ask_key_v3, &ask_v3)?;
        stats.asks_migrated += 1;
    }

    // 3. Migrate bids
    let bids_list: Vec<(BidKeyV2, BidV2)> = bids_v2()
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    for ((bidder, token_id), bid_v2) in bids_list {
        // Remove legacy v2 entry (and its indices) to avoid mixed key formats.
        bids_v2().remove(storage, (bidder.clone(), token_id.clone()))?;

        let bid_v3 = BidV3 {
            collection: collection.clone(),
            token_id: token_id.clone(),
            bidder: bidder.clone(),
            price: bid_v2.price,
            created_at: current_time,
            expires_at: None, // V2 didn't have expiration
        };

        let bid_key_v3 = (collection.clone(), token_id, bidder);
        bids().save(storage, bid_key_v3, &bid_v3)?;
        stats.bids_migrated += 1;
    }

    // 4. Migrate collection bids
    let col_bids_list: Vec<(CollectionBidKeyV2, CollectionBidV2)> = collection_bids_v2()
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    for (bidder, col_bid_v2) in col_bids_list {
        // Remove legacy v2 entry (and its indices) to avoid mixed key formats.
        collection_bids_v2().remove(storage, bidder.clone())?;

        let col_bid_v3 = CollectionBidV3 {
            collection: collection.clone(),
            bidder: bidder.clone(),
            units: col_bid_v2.units,
            price: col_bid_v2.price,
            created_at: current_time,
            expires_at: None,
        };

        let col_bid_key_v3 = (collection.clone(), bidder);
        collection_bids().save(storage, col_bid_key_v3, &col_bid_v3)?;
        stats.collection_bids_migrated += 1;
    }

    // 5. Initialize new state
    MARKET_STATS.save(storage, &MarketStats::default())?;
    for coll in config_v3.supported_collections {
        COLLECTION_STATS.save(storage, coll, &CollectionStats::default())?;
    }

    Ok(stats)
}

/// Statistics from migration
#[derive(Default)]
pub struct MigrationStats {
    pub asks_migrated: u32,
    pub bids_migrated: u32,
    pub collection_bids_migrated: u32,
}
