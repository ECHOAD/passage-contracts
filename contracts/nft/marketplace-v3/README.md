# Passage Marketplace v3

Marketplace v3 is a comprehensive NFT trading contract with **multi-collection support** and **automatic Revenue Router integration**. It provides a unified secondary market for all Passage NFT collections.

## Overview

Key improvements over marketplace-v2:
- **Multi-Collection Support**: Single marketplace instance for multiple collections
- **Revenue Router Integration**: Automatic fee and royalty distribution
- **Collection-Wide Bids**: Place offers across entire collections
- **Better Statistics**: Track sales, volume, and floor prices
- **Operator System**: Automated ask state synchronization

## Features

### Listings (Asks)
- Create listings with custom prices
- Update listing prices
- Optional custom funds recipient
- Automatic deactivation on NFT transfer

### Bids
- Place bids on specific NFTs
- Bid expiration support
- Automatic refund on bid removal
- Accept highest bid as seller

### Collection Bids
- Place bids across an entire collection
- Specify number of NFTs to buy
- Price per NFT with total escrow
- Any holder can accept your bid

### Revenue Distribution
- **Revenue Router Mode**: Full integration with Revenue Router
- **Legacy Mode**: Direct fee distribution (backwards compatible)
- Automatic royalty handling

## Architecture

```
[Buyer] ─── BuyNow/AcceptBid ──► [Marketplace v3]
                                      │
                                      ├── Transfer NFT ──► [pg721]
                                      │
                                      └── Route Payment ──► [Revenue Router]
                                                                │
                                                                ├── Platform Fee
                                                                ├── Royalty
                                                                └── Seller
```

## Messages

### Instantiate

```json
{
  "admin": "passage1...",
  "denom": "upasg",
  "min_price": "1000000",
  "trading_fee_bps": 250,
  "fee_collector": "passage1treasury...",
  "supported_collections": ["passage1coll1...", "passage1coll2..."],
  "allow_any_collection": true,
  "registry": "passage1registry...",
  "revenue_router": "passage1router...",
  "use_revenue_router": true,
  "operators": ["passage1operator..."]
}
```

### Execute Messages

#### Admin Operations
```rust
UpdateConfig {
    admin: Option<String>,
    denom: Option<String>,
    min_price: Option<Uint128>,
    trading_fee_bps: Option<u64>,
    fee_collector: Option<String>,
    registry: Option<String>,
    revenue_router: Option<String>,
    use_revenue_router: Option<bool>,
    operators: Option<Vec<String>>,
    paused: Option<bool>,
}

AddCollection { collection: String }
RemoveCollection { collection: String }
```

#### Listing Operations
```rust
// Create a listing
SetAsk {
    collection: String,
    token_id: String,
    price: Coin,
    funds_recipient: Option<String>,
}

// Update listing price
UpdateAsk {
    collection: String,
    token_id: String,
    price: Coin,
}

// Remove listing
RemoveAsk {
    collection: String,
    token_id: String,
}

// Buy listed NFT (send exact price)
BuyNow {
    collection: String,
    token_id: String,
}
```

#### Bid Operations
```rust
// Place bid on specific NFT (send bid amount)
SetBid {
    collection: String,
    token_id: String,
    price: Coin,
    expires_at: Option<u64>,
}

// Remove bid and get refund
RemoveBid {
    collection: String,
    token_id: String,
}

// Accept a bid as seller
AcceptBid {
    collection: String,
    token_id: String,
    bidder: String,
}
```

#### Collection Bid Operations
```rust
// Place collection-wide bid (send total amount)
SetCollectionBid {
    collection: String,
    units: u32,
    price: Coin,           // Price per NFT
    expires_at: Option<u64>,
}

// Remove collection bid and get refund
RemoveCollectionBid { collection: String }

// Accept a collection bid as any holder
AcceptCollectionBid {
    collection: String,
    token_id: String,
    bidder: String,
}
```

#### Operator Operations
```rust
// Sync ask state (deactivate if NFT transferred)
SyncAsk {
    collection: String,
    token_id: String,
}

// Batch sync multiple asks
BatchSyncAsks {
    asks: Vec<(String, String)>,
}
```

### Query Messages

```rust
// Configuration
Config {}

// Asks
Ask { collection, token_id }
AsksByCollection { collection, start_after, limit }
AsksBySeller { seller, start_after, limit }
AsksByPrice { collection, start_after, limit, descending }
AskCount { collection }

// Bids
Bid { collection, token_id, bidder }
BidsByToken { collection, token_id, start_after, limit }
BidsByBidder { bidder, start_after, limit }

// Collection Bids
CollectionBid { collection, bidder }
CollectionBidsByCollection { collection, start_after, limit }
CollectionBidsByBidder { bidder, start_after, limit }

// Statistics
MarketStats {}
CollectionStats { collection }

// Preview sale proceeds
PreviewSale { collection, price }
```

## State Structure

```rust
Config {
    admin: Addr,
    supported_collections: Vec<Addr>,
    allow_any_collection: bool,
    denom: String,
    min_price: Uint128,
    trading_fee_bps: u64,
    fee_collector: Addr,
    registry: Option<Addr>,
    revenue_router: Option<Addr>,
    use_revenue_router: bool,
    operators: Vec<Addr>,
    paused: bool,
}

Ask {
    collection: Addr,
    token_id: String,
    seller: Addr,
    price: Coin,
    funds_recipient: Option<Addr>,
    created_at: u64,
    is_active: bool,
}

Bid {
    collection: Addr,
    token_id: String,
    bidder: Addr,
    price: Coin,
    created_at: u64,
    expires_at: Option<u64>,
}

CollectionBid {
    collection: Addr,
    bidder: Addr,
    units: u32,
    price: Coin,
    created_at: u64,
    expires_at: Option<u64>,
}

MarketStats {
    total_sales: u64,
    total_volume: Uint128,
    total_fees: Uint128,
    total_royalties: Uint128,
}

CollectionStats {
    total_sales: u64,
    total_volume: Uint128,
    floor_price: Option<Uint128>,
    highest_sale: Uint128,
}
```

## Revenue Router Integration

### How Sales Work

1. Buyer calls `BuyNow` with exact payment
2. Marketplace validates payment and ownership
3. Marketplace calls Revenue Router's `RouteSecondarySale`:
   ```json
   {
     "route_secondary_sale": {
       "collection": "passage1coll...",
       "seller": "passage1seller...",
       "royalty_amount": "50000"
     }
   }
   ```
4. Revenue Router distributes:
   - Platform fee to treasury
   - Royalty to creator/collaborators
   - Remainder to seller
5. Marketplace transfers NFT to buyer

### Fee Structure

```
Sale Price: 100 PASG
├── Platform Fee (2.5%): 2.5 PASG → Platform Treasury
├── Royalty (5%):        5.0 PASG → Creator/Collaborators
└── Seller Proceeds:    92.5 PASG → Seller
```

## Migration from Marketplace v2

Marketplace v3 supports in-place state migration via `migrate`.
See also: [`MIGRATION.md`](./MIGRATION.md).

### Migrate Message

```json
{
  "collection": "passage1legacycoll...",
  "registry": "passage1registry...",
  "revenue_router": "passage1router...",
  "use_revenue_router": true,
  "additional_collections": [
    "passage1coll2...",
    "passage1coll3..."
  ],
  "collection_denoms": [
    { "collection": "passage1legacycoll...", "denom": "upasg" },
    { "collection": "passage1coll3...", "denom": "uion" }
  ]
}
```

Required:
- `collection`: v2 only had one collection; this sets the source collection for all migrated asks/bids.

Optional:
- `registry`, `revenue_router`, `use_revenue_router`
- `additional_collections`: extra collections enabled after migration
- `collection_denoms`: per-collection payment denom overrides

### What Migration Changes

1. Converts v2 single-collection config to v3 multi-collection config.
2. Migrates asks, bids, and collection bids into v3 key format.
3. Initializes v3 stats (`MarketStats`, `CollectionStats`).
4. Persists denom override for the migrated source collection.
5. Updates contract version metadata.

### Pre-Migration Checklist

1. Pause frontend write operations to avoid concurrent list/bid writes.
2. Confirm current contract is a marketplace-v2 instance.
3. Decide the canonical migrated `collection` address.
4. Prepare router/registry addresses and denom overrides.
5. Snapshot indexer state for verification.

### Example CLI

```bash
wasmd tx wasm migrate <marketplace_addr> <new_code_id> '{
  "collection":"passage1legacycoll...",
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

### Post-Migration Verification

1. Query `Config {}` and validate `supported_collections`, router flags, and admin values.
2. Query `AskCount` and compare with pre-migration snapshots.
3. Query `BidsByToken` / `BidsByBidder` samples and compare with snapshots.
4. Query `CollectionBidsByCollection` samples and compare with snapshots.
5. Query `CollectionDenom { collection }` for collections with overrides.
6. Validate the migrate event attributes:
   - `asks_migrated`
   - `bids_migrated`
   - `collection_bids_migrated`
7. Execute a low-value test sale to confirm routing and settlement behavior.

## Operator System

Operators are trusted addresses that can synchronize ask states:

```rust
// After NFT transfer, operator syncs the ask
SyncAsk { collection: "passage1...", token_id: "123" }
```

This deactivates asks for NFTs that have been transferred outside the marketplace, preventing stale listings.

## Access Control

| Action | Who Can Execute |
|--------|-----------------|
| SetAsk | Token owner |
| UpdateAsk | Ask seller |
| RemoveAsk | Ask seller or Admin |
| BuyNow | Anyone (with payment) |
| SetBid | Anyone (with payment) |
| RemoveBid | Bidder |
| AcceptBid | Token owner |
| SyncAsk | Operators or Admin |
| UpdateConfig | Admin |

## Events

All operations emit events for indexer consumption:
- `action`: Operation type
- `collection`: Collection address
- `token_id`: Token identifier
- `seller` / `buyer` / `bidder`: Participant addresses
- `price`: Transaction price

## License

Apache-2.0
