# Marketplace v3 Migration Guide

This guide covers in-place migration from marketplace-v2 to marketplace-v3.

## Source and Target

- Source: marketplace-v2 state (single collection)
- Target: marketplace-v3 state (multi-collection)

## Migrate Message

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
- `collection`

Optional:
- `registry`
- `revenue_router`
- `use_revenue_router` (defaults to `true` when router is present)
- `additional_collections`
- `collection_denoms`

## Pre-Migration Checklist

1. Freeze writes from frontend/indexer workers.
2. Confirm current contract is marketplace-v2.
3. Decide source `collection` and optional additional collections.
4. Prepare per-collection denom overrides if needed.
5. Snapshot asks/bids for post-check.

## Execute Migration

```bash
wasmd tx wasm migrate <marketplace_addr> <new_code_id> '{
  "collection":"passage1legacycoll...",
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

## What Gets Migrated

1. v2 config -> v3 config.
2. asks/bids/collection_bids key format conversion.
3. `MarketStats` and `CollectionStats` initialization.
4. migrated collection denom override persistence.
5. contract version update.

## Post-Migration Verification

1. Query `Config {}`.
2. Query `AskCount`.
3. Query sample `BidsByToken` and `CollectionBidsByCollection`.
4. Query `CollectionDenom { collection }` for override collections.
5. Confirm migrate event attributes:
   - `asks_migrated`
   - `bids_migrated`
   - `collection_bids_migrated`
6. Run a low-value test sale.

