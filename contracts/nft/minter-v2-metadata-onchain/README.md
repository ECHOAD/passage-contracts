# Passage Minter v2

Minter v2 is an upgraded NFT minting contract with **automatic Revenue Router integration**. It replaces the legacy manual withdrawal pattern with instant, configurable revenue distribution.

## Overview

Key improvements over the original minter:
- **Automatic Revenue Routing**: Funds are sent to the Revenue Router on each mint
- **No Manual Withdrawals**: Eliminates admin withdrawal bottleneck
- **Registry Awareness**: Can verify authorization with the Registry
- **Batch Minting**: Support for minting multiple tokens in one transaction
- **Better Statistics**: Tracks minting metrics and revenue

## Features

### Minting
- Random token ID selection
- Per-address mint limits
- Whitelist support with special pricing
- Timed minting start
- Batch minting support

### Revenue Handling
- **Revenue Router Mode** (recommended): Funds automatically routed on each mint
- **Legacy Mode**: Funds accumulate for admin withdrawal (backwards compatible)

### Admin Controls
- Pause/unpause minting
- Update pricing and limits
- Set/remove whitelist
- Update start time

## Architecture

```
User Payment
    │
    ▼
[Minter v2] ─── mint NFT ──► [pg721 Collection]
    │
    └── route payment ──► [Revenue Router]
                              │
                              ├──► Platform Fee
                              └──► Creator/Collaborators
```

## Messages

### Instantiate

```json
{
  "base_token_uri": "ipfs://QmXxx.../",
  "num_tokens": 10000,
  "cw721_code_id": 123,
  "cw721_instantiate_msg": {
    "name": "My Collection",
    "symbol": "MYCOL",
    "minter": "...",
    "collection_info": {
      "creator": "passage1...",
      "description": "...",
      "image": "ipfs://...",
      "royalty_info": {
        "payment_address": "passage1...",
        "share": "0.05"
      }
    }
  },
  "start_time": "1709251200000000000",
  "per_address_limit": 5,
  "unit_price": { "denom": "upasg", "amount": "1000000" },
  "whitelist": "passage1whitelist...",
  "registry": "passage1registry...",
  "revenue_router": "passage1router...",
  "use_revenue_router": true
}
```

### Execute Messages

#### Minting
```rust
// Mint a random token (payment required)
Mint {}

// Admin: Mint to a specific recipient (no payment)
MintTo { recipient: String }

// Admin: Mint specific token ID to recipient
MintFor { token_id: u32, recipient: String }

// Mint multiple tokens at once
BatchMint { count: u32 }
```

#### Configuration
```rust
UpdateConfig {
    admin: Option<String>,
    per_address_limit: Option<u32>,
    unit_price: Option<Coin>,
    whitelist: Option<String>,
    registry: Option<String>,
    revenue_router: Option<String>,
    use_revenue_router: Option<bool>,
    paused: Option<bool>,
}

UpdateStartTime { start_time: Timestamp }

SetWhitelist { whitelist: String }

RemoveWhitelist {}
```

#### Legacy Operations
```rust
// Only works if use_revenue_router is false
Withdraw {}
WithdrawTo { recipient: String }
```

### Query Messages

```rust
// Get contract configuration
Config {}

// Get remaining mintable tokens
MintableNumTokens {}

// Get minting start time
StartTime {}

// Get current mint price (considers whitelist)
MintPrice {}

// Get mint count for an address
MintCount { address: String }

// Check if address can mint
CanMint { address: String }

// Get minting statistics
MintStats {}

// Check if minting is currently active
IsMintingActive {}
```

## Revenue Router Integration

### How It Works

1. User sends payment with `Mint {}` message
2. Minter validates payment and mints NFT
3. Minter calls Revenue Router's `RoutePrimarySale`
4. Revenue Router distributes funds according to collection rules

### Configuration

Set `revenue_router` and `use_revenue_router: true` during instantiation:

```json
{
  "revenue_router": "passage1router...",
  "use_revenue_router": true
}
```

### Revenue Router Setup

Before minting, ensure the Revenue Router has a distribution rule for this collection:

```json
{
  "set_distribution_rule": {
    "collection": "passage1collection...",
    "creator": "passage1creator...",
    "creator_share": "0.95",
    "platform_fee": "0.025",
    "collaborators": [
      { "address": "passage1collab...", "share": "0.10" }
    ]
  }
}
```

## State Structure

```rust
Config {
    admin: Addr,
    cw721_address: Addr,
    cw721_code_id: u64,
    base_token_uri: String,
    num_tokens: u32,
    unit_price: Coin,
    per_address_limit: u32,
    start_time: Timestamp,
    whitelist: Option<Addr>,
    registry: Option<Addr>,
    revenue_router: Option<Addr>,
    use_revenue_router: bool,
    paused: bool,
}

MintStats {
    total_minted: u32,
    total_revenue: Uint128,
    total_routed: Uint128,  // Amount sent to Revenue Router
    unique_minters: u32,
}
```

## Events

All minting operations emit events:
- `action`: mint, mint_to, mint_for, batch_mint
- `token_id`: The minted token ID(s)
- `minter` / `recipient`: Who received the NFT
- `price`: Payment amount
- `is_whitelist`: Whether whitelist pricing was used

## Migration from Minter v1

Minter v2 supports in-place migration via `migrate` from:
- `crates.io:passage-minter`
- `crates.io:passage-minter-metadata-onchain`
See also: [`MIGRATION.md`](./MIGRATION.md).

### Migrate Message

```json
{
  "registry": "passage1registry...",
  "revenue_router": "passage1router...",
  "use_revenue_router": true,
  "base_token_uri": "ipfs://QmCollectionBase/"
}
```

Notes:
- `base_token_uri` is required only when migrating from `passage-minter-metadata-onchain`.
- `use_revenue_router` defaults to `true` when `revenue_router` is provided.

### What Migration Preserves

1. Existing `cw721_address` and minting config.
2. Mintable token set and remaining supply.
3. Per-address mint counters.
4. Mint stats (`total_minted`, `total_revenue`, `unique_minters`), with `total_routed` initialized to zero for migrated state.
5. Contract version metadata (`cw2`).

### Pre-Migration Checklist

1. Pause mint UI to avoid race conditions while migrating.
2. Confirm the current contract version/source contract.
3. If source is `metadata-onchain`, prepare a valid `base_token_uri`.
4. Pre-create distribution rule in Revenue Router for the collection.
5. Snapshot mintable count and a sample of minter counters.

### Example CLI

```bash
wasmd tx wasm migrate <minter_addr> <new_code_id> '{
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

Example from `metadata-onchain` source:

```bash
wasmd tx wasm migrate <minter_addr> <new_code_id> '{
  "registry":"passage1registry...",
  "revenue_router":"passage1router...",
  "use_revenue_router":true,
  "base_token_uri":"ipfs://QmCollectionBase/"
}' --from <admin> --gas auto --gas-adjustment 1.3 -y
```

### Post-Migration Verification

1. Query `Config {}` and confirm router + registry fields.
2. Query `MintableNumTokens {}` and compare to pre-migration snapshot.
3. Query `MintStats {}` and validate totals.
4. Validate migrate event attributes:
   - `source_state`
   - `tokens_migrated`
   - `mintable_remaining`
   - `unique_minters`
   - `revenue_router_enabled`
5. Execute a low-value `Mint {}` test transaction.

### Configuration Differences

| Feature | Minter v1 | Minter v2 |
|---------|-----------|-----------|
| Revenue Handling | Manual Withdraw | Auto-route to Revenue Router |
| Batch Mint | No | Yes |
| Registry Integration | No | Yes |
| Pause/Unpause | No | Yes |
| Minting Stats | No | Yes |

## Access Control

| Action | Who Can Execute |
|--------|-----------------|
| Mint | Anyone (with payment) |
| MintTo | Admin only |
| MintFor | Admin only |
| BatchMint | Anyone (with payment) |
| UpdateConfig | Admin only |
| Withdraw | Admin only (legacy mode only) |

## Error Handling

Common errors:
- `MintingPaused`: Minting is currently paused
- `MintingNotStarted`: Current time is before start_time
- `SoldOut`: No tokens remaining
- `InvalidPayment`: Wrong payment amount
- `MaxMintLimitReached`: Address exceeded per_address_limit
- `CannotWithdrawWithRevenueRouter`: Trying to withdraw in router mode

## License

Apache-2.0
