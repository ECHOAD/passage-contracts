# Passage Revenue Router Contract

The Revenue Router is the **financial brain** of the Passage ecosystem. It handles automatic, configurable revenue distribution for all economic activity including minting (primary sales), marketplace transactions (secondary sales), and auctions.

## Overview

The Revenue Router replaces the fragmented revenue handling in the existing contracts:
- **Minter**: Previously accumulated funds for manual admin withdrawal
- **Marketplace**: Previously hardcoded direct fee/royalty distribution

With the Revenue Router, all revenue flows through a single, configurable contract that:
- Applies platform fees automatically
- Distributes to creators and collaborators
- Tracks all financial events for auditing
- Supports split wallets for complex multi-party distributions

## Features

### Distribution Rules
- Configure per-collection distribution rules
- Set custom platform fee overrides
- Define creator and collaborator shares
- Link to royalty pools for secondary sales

### Revenue Routing
- Route primary sale revenue (minting)
- Route secondary sale revenue (marketplace)
- Route auction sale revenue
- Automatic platform fee deduction
- Automatic collaborator payment

### Split Wallets
- Create multi-recipient split wallets (evolved from royalty-group)
- Weight-based distribution
- Admin-controlled recipient management

### Analytics & Auditing
- Track all revenue events
- Per-collection statistics
- Total volume tracking
- Platform fee tracking

## Architecture

```
[Minter v2] ──────┐
                  │
[Marketplace v3] ─┼──► [Revenue Router] ──► [Platform Treasury]
                  │           │
[Auction]  ───────┘           ├──► [Creator]
                              ├──► [Collaborators]
                              └──► [Split Wallets]
```

## Messages

### Instantiate

```json
{
  "admin": "passage1...",
  "platform_fee_collector": "passage1treasury...",
  "default_platform_fee": "0.025",
  "registry": "passage1registry..."
}
```

### Execute Messages

#### Admin Operations
```rust
UpdateConfig {
    admin: Option<String>,
    platform_fee_collector: Option<String>,
    default_platform_fee: Option<Decimal>,
    registry: Option<String>,
    paused: Option<bool>,
}
```

#### Distribution Rules
```rust
// Set distribution rule for a collection
SetDistributionRule {
    collection: String,
    creator: String,
    creator_share: Decimal,        // e.g., "0.95" for 95%
    platform_fee: Option<Decimal>, // Override default
    collaborators: Option<Vec<CollaboratorInput>>,
    royalty_pool: Option<String>,
}

// Update existing rule
UpdateDistributionRule {
    collection: String,
    creator: Option<String>,
    creator_share: Option<Decimal>,
    platform_fee: Option<Decimal>,
    collaborators: Option<Vec<CollaboratorInput>>,
    royalty_pool: Option<String>,
    active: Option<bool>,
}

// Remove rule
RemoveDistributionRule { collection: String }

// Optional ecosystem treasury metadata (no ecosystem fee at this stage)
SetEcosystemConfig {
    ecosystem_id: String,
    treasury: Option<String>,
}
```

#### Revenue Routing
```rust
// Called by Minter v2 on each mint
RoutePrimarySale { collection: String }

// Called by Marketplace v3 on each sale
RouteSecondarySale {
    collection: String,
    seller: String,
    royalty_amount: Uint128,
}

// Called by Auction contract
RouteAuctionSale {
    collection: String,
    seller: String,
    royalty_amount: Uint128,
}
```

#### Split Wallets
```rust
CreateSplitWallet {
    id: String,
    recipients: Vec<SplitRecipientInput>,
}

UpdateSplitWallet {
    id: String,
    recipients: Vec<SplitRecipientInput>,
}

DistributeSplitWallet { id: String }

RemoveSplitWallet { id: String }
```

### Query Messages

```rust
// Configuration
Config {}

// Distribution rules
DistributionRule { collection: String }
DistributionRules { start_after, limit }

// Ecosystem config
EcosystemConfig { ecosystem_id: String }

// Preview distribution without executing
PreviewDistribution {
    collection: String,
    amount: Uint128,
    event_type: RevenueEventType,
}

// Statistics
CollectionStats { collection: String }
RevenueEvents { collection, start_after, limit }

// Split wallets
SplitWallet { id: String }
SplitWallets { start_after, limit }
```

## Distribution Flow

### Primary Sale (Minting)
```
Total Payment
    │
    ├── Platform Fee (2.5% default) ──► Platform Treasury
    │
    └── Remaining (97.5%)
            │
            ├── Collaborator 1 (10% of remaining) ──► Collaborator
            ├── Collaborator 2 (5% of remaining)  ──► Collaborator
            │
            └── Creator (remaining) ──► Creator Wallet
```

### Secondary Sale (Marketplace)
```
Sale Price
    │
    ├── Platform Fee (2.5%) ──► Platform Treasury
    │
    ├── Seller Amount (after fees & royalty) ──► Seller
    │
    └── Royalty Amount
            │
            ├── Collaborator shares ──► Collaborators
            └── Creator share ──► Creator
```

## State Structure

```rust
// Configuration
Config {
    admin: Addr,
    platform_fee_collector: Addr,
    default_platform_fee: Decimal,
    registry: Option<Addr>,
    paused: bool,
}

// Distribution Rule (per collection)
DistributionRule {
    collection: Addr,
    platform_fee: Option<Decimal>,
    creator: Addr,
    creator_share: Decimal,
    collaborators: Vec<Collaborator>,
    royalty_pool: Option<Addr>,
    active: bool,
    created_at: u64,
    updated_at: u64,
}

// Collaborator
Collaborator {
    address: Addr,
    share: Decimal,
    name: Option<String>,
}

// Revenue Event (for auditing)
RevenueEvent {
    id: u64,
    collection: Addr,
    event_type: RevenueEventType,
    total_amount: Uint128,
    denom: String,
    platform_fee: Uint128,
    creator_amount: Uint128,
    collaborator_amounts: Vec<(Addr, Uint128)>,
    timestamp: u64,
    tx_sender: Addr,
}

// Collection Statistics
CollectionStats {
    total_primary_volume: Uint128,
    total_secondary_volume: Uint128,
    total_platform_fees: Uint128,
    total_creator_earnings: Uint128,
    total_royalties: Uint128,
    event_count: u64,
}
```

## Integration Guide

### For Minter v2

```rust
// On mint, call Revenue Router with payment
let route_msg = RevenueRouterExecuteMsg::RoutePrimarySale {
    collection: collection_address.to_string(),
};

let route_submsg = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: revenue_router_address.to_string(),
    msg: to_json_binary(&route_msg)?,
    funds: vec![payment_coin],
});
```

### For Marketplace v3

```rust
// On sale, call Revenue Router with payment
let route_msg = RevenueRouterExecuteMsg::RouteSecondarySale {
    collection: collection_address.to_string(),
    seller: seller_address.to_string(),
    royalty_amount: calculated_royalty,
};

let route_submsg = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: revenue_router_address.to_string(),
    msg: to_json_binary(&route_msg)?,
    funds: vec![sale_amount_coin],
});
```

## Migration and Upgrade Notes

`revenue-router` does not currently expose a `migrate` entrypoint. Upgrade strategy is deploy-and-cutover:
See also: [`MIGRATION.md`](./MIGRATION.md).

1. Deploy a new router instance.
2. Recreate distribution rules (`SetDistributionRule`) in the new instance.
3. Recreate split wallets if used.
4. Update all producers to point to the new router:
   - `marketplace-v3` via `UpdateConfig { revenue_router, use_revenue_router }`
   - `minter-v2` via `UpdateConfig { revenue_router, use_revenue_router }`
   - set optional ecosystem treasury via `SetEcosystemConfig`
5. Verify with low-value primary and secondary test flows.

Operational note:
- Funds already sent to the previous router remain there. Distribute/settle them before decommissioning old routing paths.

## Access Control

| Action | Who Can Execute |
|--------|-----------------|
| Update Config | Contract Admin |
| Set Distribution Rule | Contract Admin |
| Update Distribution Rule | Contract Admin or Collection Creator (via Registry) |
| Route Revenue | Any contract (Minter, Marketplace, Auction) |
| Create Split Wallet | Anyone |
| Modify Split Wallet | Split Wallet Admin |
| Distribute Split Wallet | Anyone (with funds) |

## Events

All routing operations emit events for indexer consumption:
- `action`: The operation type
- `collection`: Collection address
- `event_type`: PrimarySale, SecondarySale, Auction, Royalty
- `total_amount`: Total payment received
- `platform_fee`: Platform fee deducted
- `creator_amount`: Amount sent to creator
- `event_id`: Unique event identifier

## Constraints

- Maximum platform fee: 10%
- Creator share must be > 0 and <= 100%
- Total collaborator shares must be <= 100%
- Split wallet total weight must be > 0
- Maximum stored events: 1000 (older events pruned)

## License

Apache-2.0
