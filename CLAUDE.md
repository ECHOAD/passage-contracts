# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build all WASM contracts
cargo wasm

# Build debug WASM
cargo wasm-debug

# Run all unit tests
cargo unit-test

# Test specific contract
cargo test --package registry

# Generate JSON schemas for a contract
cargo run --package pg721 --example schema

# Optimize WASM for deployment (requires Docker)
./scripts/optimize.sh          # amd64
./scripts/optimize-arm.sh      # ARM64
```

## Deployment Commands

```bash
# Store contract on-chain
passage tx wasm store artifacts/<contract>.wasm --from <address> --chain-id=passage-2 \
  --gas-prices 0.1upasg --gas auto --gas-adjustment 1.3

# Migrate existing contract
passage tx wasm migrate <contract_address> <code_id> '<migrate_msg>' --from <address> \
  --chain-id=passage-2 --gas-prices 0.1upasg --gas auto --gas-adjustment 1.3
```

## Architecture

This repository contains 24 CosmWasm smart contracts for the Passage Protocol NFT commerce, staking, and economic infrastructure.

### Contract Categories

**Core** (`contracts/core/`):
- `registry`: Source of truth for ecosystems, collections, and minter authorization
- `ecosystem-factory`: Governed flow for creating ecosystems
- `collection-factory`: Deploys pg721 collections within ecosystems
- `revenue-router` (aka split-router): Routes primary/secondary sale revenue with automatic fee distribution
- `multisig`: Proposal-based administrative control with signer rotation

**NFT** (`contracts/nft/`):
- `pg721`, `pg721-metadata-onchain`, `pg721-updatable`, `pg721-legacy`: NFT collection variants
- `minter-v2`, `minter-v2-metadata-onchain`: Primary sale minting with revenue router integration
- `marketplace-v3`: Multi-collection fixed-price sales, bids, and collection-wide offers
- `auction-english`: Reserve-style NFT auctions
- `whitelist`: Allowlist management for minting
- `royalty-group`: Multi-recipient split wallets for royalties

**Staking** (`contracts/staking/`):
- `nft-vault`: NFT staking with multiple reward accounts
- `vault-factory`: Creates vault instances
- `stake-rewards`: Distributes rewards to vault participants

**Relationship** (`contracts/relationship/`):
- `follow`, `friend`: Social mechanics

### Collection Creation Paths

**Path 1** (ecosystem management):
```
registry → ecosystem-factory → collection-factory → pg721
```

**Path 2** (primary drops):
```
minter-v2 → (deploys pg721) → register in registry → authorize minter
```

### Revenue Flow

Revenue router handles automatic distribution:
- `minter-v2` calls `RoutePrimarySale` on mint
- `marketplace-v3` calls `RouteSecondarySale` on sale
- `auction-english` calls `RouteAuctionSale` on settlement

## Code Patterns

### Contract Structure
```rust
pub mod contract;      // Entry points (instantiate, execute, query)
pub mod error;         // Custom ContractError using thiserror
pub mod msg;           // InstantiateMsg, ExecuteMsg, QueryMsg
pub mod state;         // State structures with cw-storage-plus
```

### Key Dependencies
- `cosmwasm-std = 2.1.3` with staking, stargate, cosmwasm_1_2 features
- `cw-storage-plus = 2.0` for advanced storage patterns
- `sylvia = 1.2.1` for contract framework
- `cw-multi-test = 2.1.1` for testing

### Testing
Tests use `cw-multi-test` for mock contract execution. Tests are typically in `src/contract/tests.rs` or `src/tests.rs`.

## Key Architectural Principles

From the Blockchain Smart Contract Engineer skill profile:

> **Passage** = the economic operating system (monetization framework, revenue routing, billing orchestrator)
>
> **PASG** = economic instrument within that system (payment token, governance token, staking token)

**Scope discipline**: Contracts must not absorb platform logic. Billing, subscription, and tier enforcement belong off-chain. On-chain contracts provide primitives that platform services invoke.

## Documentation

See `docs/` for comprehensive guides:
1. `01-end-to-end-setup.md` - Complete production setup
2. `02-method-reference.md` - Message signatures
3. `03-json-examples.md` - Ready-to-use payloads
4. `04-multisig-governance.md` - Admin patterns

Individual contracts have README.md files with access control matrices and migration notes.
