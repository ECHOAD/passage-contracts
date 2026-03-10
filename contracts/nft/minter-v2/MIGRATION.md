# Minter v2 Migration Guide

This guide covers in-place migration to minter-v2 from legacy minter contracts.

## Supported Sources

- `crates.io:passage-minter`
- `crates.io:passage-minter-metadata-onchain`

## Migrate Message

```json
{
  "registry": "passage1registry...",
  "revenue_router": "passage1router...",
  "use_revenue_router": true,
  "base_token_uri": "ipfs://QmCollectionBase/"
}
```

Notes:
- `base_token_uri` is required only for `passage-minter-metadata-onchain`.
- `use_revenue_router` defaults to `true` when router is provided.

## Pre-Migration Checklist

1. Pause mint UI and background jobs.
2. Verify source contract kind/version.
3. For metadata-onchain source, prepare `base_token_uri`.
4. Prepare router/registry addresses.
5. Snapshot mintable supply and sample minter counters.

## Execute Migration

From `passage-minter`:

```bash
wasmd tx wasm migrate <minter_addr> <new_code_id> '{
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

From `passage-minter-metadata-onchain`:

```bash
wasmd tx wasm migrate <minter_addr> <new_code_id> '{
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true,
  "base_token_uri":"ipfs://QmCollectionBase/"
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

## What Gets Migrated

1. Existing `cw721_address` and core config.
2. Mintable token set and remaining supply.
3. Per-address mint counters.
4. Mint stats (`total_minted`, `total_revenue`, `unique_minters`).
5. contract version update.

## Post-Migration Verification

1. Query `Config {}`.
2. Query `MintableNumTokens {}`.
3. Query `MintStats {}`.
4. Confirm migrate event attributes:
   - `source_state`
   - `tokens_migrated`
   - `mintable_remaining`
   - `unique_minters`
   - `revenue_router_enabled`
5. Run a low-value `Mint {}` test.

