# Architecture

**Analysis Date:** 2026-03-17

## Pattern Overview

**Overall:** Domain-segmented CosmWasm workspace with a registry-centered control plane and contract-local state machines.

**Key Characteristics:**
- The workspace root `Cargo.toml` groups deployable contracts by domain under `contracts/core/*`, `contracts/nft/*`, `contracts/staking/*`, and `contracts/relationship/*`; each child crate compiles to its own Wasm artifact.
- Global policy and catalog state live in `contracts/core/registry`, while downstream contracts such as `contracts/core/collection-factory`, `contracts/core/ecosystem-factory`, `contracts/nft/minter-v2`, and `contracts/nft/marketplace-v3` query the registry instead of duplicating allowlist or moderation rules.
- Internal crate structure is mixed on purpose: newer crates split entry points into `src/contract/*.rs` modules (`contracts/core/registry/src/contract/*.rs`, `contracts/nft/marketplace-v3/src/contract/*.rs`), older crates keep direct `src/contract.rs` or `src/execute.rs` / `src/query.rs` files (`contracts/nft/marketplace-legacy/src/execute.rs`, `contracts/relationship/follow/src/execute.rs`), and staking crates use Sylvia-generated entry points (`contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`).

## Layers

**Control Plane Contracts:**
- Purpose: Own protocol-wide registration, moderation, governance, routing, and platform billing decisions.
- Location: `contracts/core/registry`, `contracts/core/collection-factory`, `contracts/core/ecosystem-factory`, `contracts/core/multisig`, `contracts/core/split-router`, `contracts/core/streaming-billing`
- Contains: ecosystem and collection registries, factory request queues, governance proposals, payment routing, billing sessions, and platform stats.
- Depends on: `cw-storage-plus` state, CosmWasm entry points, and cross-contract messages such as `RegistryQueryMsg` / `RegistryExecuteMsg` in `contracts/core/registry/src/msg.rs`.
- Used by: NFT contracts in `contracts/nft/*`, staking and billing contracts that need downstream addresses, and off-chain clients that treat the registry as the source of truth.

**NFT Asset and Commerce Contracts:**
- Purpose: Define collection primitives and monetization flows for Passage assets.
- Location: `contracts/nft/pg721`, `contracts/nft/pg721-updatable`, `contracts/nft/pg721-metadata-onchain`, `contracts/nft/minter*`, `contracts/nft/marketplace-*`, `contracts/nft/auction-english`, `contracts/nft/royalty-group`, `contracts/nft/whitelist`
- Contains: CW721-derived collection contracts, minters that instantiate or mint into collections, marketplace order books, auctions, royalty routing, and sale allowlisting.
- Depends on: collection metadata in `contracts/nft/pg721/src/state.rs`, registry lookups from helpers such as `contracts/nft/minter-v2/src/contract/helpers.rs` and `contracts/nft/marketplace-v3/src/contract/helpers.rs`, plus direct CW721 queries.
- Used by: creators, factory contracts, and user-facing mint/trade flows.

**Staking and Rewards Contracts:**
- Purpose: Manage NFT staking vaults and reward distribution accounts.
- Location: `contracts/staking/nft-vault`, `contracts/staking/stake-rewards`, `contracts/staking/vault-factory`
- Contains: Sylvia contracts with instantiate/exec/query methods, indexed staking state, instantiate2 flows, and reward-account orchestration.
- Depends on: local workspace crate references declared in the root `Cargo.toml`, plus shared libraries like `uju-cw2-common`, `uju-cw2-nft`, and `uju-index-query`.
- Used by: the staking factory in `contracts/staking/vault-factory/src/contract.rs` and any caller creating staking products around approved NFT collections.

**Relationship Contracts:**
- Purpose: Model social graph edges and emit hook notifications on changes.
- Location: `contracts/relationship/follow`, `contracts/relationship/friend`
- Contains: follow/friend state, add/remove hook management, and query surfaces backed by indexed maps.
- Depends on: contract-local `msg.rs`, `state.rs`, and hook helpers in `contracts/relationship/follow/src/hooks.rs` and `contracts/relationship/friend/src/hooks.rs`.
- Used by: other contracts or indexers that subscribe to follow/unfollow or friend/unfriend events.

**Intra-Contract State Machine Layer:**
- Purpose: Keep every contract self-contained around messages, entry points, helpers, and persistent state.
- Location: `src/lib.rs`, `src/msg.rs`, `src/state.rs`, `src/error.rs`, and either `src/contract.rs` or `src/contract/*.rs` inside each crate.
- Contains: message enums, entry-point dispatch, helper functions, reply handlers, migrations, and `Item` / `Map` / `IndexedMap` definitions.
- Depends on: CosmWasm APIs (`Deps`, `DepsMut`, `Env`, `MessageInfo`) and `cw-storage-plus`.
- Used by: all crates; this is the stable implementation pattern to follow when extending an existing contract.

## Data Flow

**Ecosystem Provisioning Flow:**

1. `contracts/core/ecosystem-factory/src/contract/execute.rs` accepts `SubmitEcosystemCreationRequest`, validates the creator against `contracts/core/registry` through `RegistryQueryMsg::CanCreateEcosystem`, and stores the request in `REQUESTS` and `PENDING_REQUEST_BY_ID`.
2. The same file resolves approved requests by instantiating a per-ecosystem collection factory with `SubMsg::reply_on_success`, storing `PendingEcosystemCreation` in `PENDING_ECOSYSTEM_CREATIONS`.
3. `contracts/core/ecosystem-factory/src/contract/reply.rs` parses the instantiate reply, extracts the new collection-factory address, and registers the ecosystem back into `contracts/core/registry` via `RegistryExecuteMsg::RegisterEcosystemFromFactory`.

**Collection Provisioning Flow:**

1. `contracts/core/collection-factory/src/contract/execute.rs` validates creator permissions by querying `contracts/core/registry` for ecosystem membership, moderation, and cross-ecosystem admin status.
2. It persists `PendingCreation` in `PENDING_CREATIONS`, instantiates a `pg721` collection with `WasmMsg::Instantiate`, and binds the reply ID to the pending request.
3. `contracts/core/collection-factory/src/contract/reply.rs` resolves the new collection address, records it in local `COLLECTIONS`, and calls `RegistryExecuteMsg::RegisterCollectionFromFactory` so the registry becomes the canonical catalog entry.

**Mint Authorization Flow:**

1. A minter entry point such as `contracts/nft/minter-v2/src/contract/execute.rs` delegates gating to `contracts/nft/minter-v2/src/contract/helpers.rs`.
2. Helper code queries `contracts/core/registry` to verify the collection is registered, minting is enabled, and the minter contract is authorized for that collection.
3. The minter optionally queries `contracts/nft/whitelist` for active whitelist configuration, updates local mint counters in `MINTER_ADDRS`, `MINTABLE_NUM_TOKENS`, and `MINT_STATS`, then issues a CW721 mint message against the configured collection address.

**Marketplace Sale Flow:**

1. `contracts/nft/marketplace-v3/src/contract/execute.rs` validates ask/bid actions and uses `contracts/nft/marketplace-v3/src/contract/helpers.rs` to decide whether a collection can trade.
2. Helper code checks local collection configuration in `COLLECTION_CONFIGS` and, when configured, queries `contracts/core/registry` through `RegistryQueryMsg::CanTradeCollection`.
3. Sale execution computes trading fees and optional royalties, routes royalty payouts either directly or through `contracts/core/split-router`, transfers the NFT, and updates `MARKET_STATS` plus `COLLECTION_STATS`.

**Staking Vault Flow:**

1. `contracts/staking/vault-factory/src/contract.rs` stores code IDs and creates vault instances with `WasmMsg::Instantiate2`, predicting the address before execution.
2. `contracts/staking/nft-vault/src/contract.rs` accepts stake and unstake actions, persists staked NFT records in an `IndexedMap`, and emits submessages to update associated reward contracts.
3. Reward accounts are created from the vault itself with another `Instantiate2` flow into `contracts/staking/stake-rewards`, keeping vault and reward account linkage on-chain.

**State Management:**
- Persistent state is crate-local and explicit. Examples include `CONFIG` and `ECOSYSTEMS` in `contracts/core/registry/src/state.rs`, `COLLECTION_CONFIGS`, `asks()`, and `bids()` in `contracts/nft/marketplace-v3/src/state.rs`, and `users_staked_nfts` in `contracts/staking/nft-vault/src/contract.rs`.
- Query-heavy datasets use `IndexedMap` and `MultiIndex` instead of scanning raw maps. Representative patterns live in `contracts/core/registry/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`, and `contracts/relationship/follow/src/state.rs`.
- Multi-step workflows store pending state keyed by reply IDs before sending submessages. This pattern appears in `contracts/core/ecosystem-factory/src/state.rs`, `contracts/core/collection-factory/src/state.rs`, `contracts/nft/minter-v2/src/contract/instantiate.rs`, and `contracts/nft/minter-v2-metadata-onchain/src/contract/instantiate.rs`.

## Key Abstractions

**Registry Catalog and Policy Surface:**
- Purpose: Provide the canonical record for ecosystems, collections, minter authorization, moderation, and ownership recovery.
- Examples: `contracts/core/registry/src/state.rs`, `contracts/core/registry/src/msg.rs`, `contracts/core/registry/src/contract/helpers.rs`
- Pattern: One contract owns protocol-wide data, while other contracts consume it through `query_wasm_smart` and explicit execute messages.

**Pending Creation Queues:**
- Purpose: Bridge async contract instantiation to later registration logic.
- Examples: `contracts/core/ecosystem-factory/src/state.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/state.rs`, `contracts/core/collection-factory/src/contract/reply.rs`
- Pattern: Save request metadata before `SubMsg::reply_on_success`, then finalize local state and downstream registration in the reply handler.

**Per-Contract Config Object:**
- Purpose: Centralize admin addresses, code IDs, pause flags, pricing, and contract references.
- Examples: `contracts/core/split-router/src/state.rs`, `contracts/nft/minter-v2/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`, `contracts/core/streaming-billing/src/state.rs`
- Pattern: A `Config` struct in `state.rs` plus a top-level `CONFIG: Item<Config>` gatekeeps privileged actions and integration endpoints.

**Indexed Storage Models:**
- Purpose: Support paginated queries and alternate lookup dimensions without off-chain joins.
- Examples: `collections()` in `contracts/core/registry/src/state.rs`, `asks()` / `bids()` / `collection_bids()` in `contracts/nft/marketplace-v3/src/state.rs`, `follows()` in `contracts/relationship/follow/src/state.rs`
- Pattern: `IndexedMap` exposes secondary indexes for query handlers; add new query dimensions here before adding query handlers that need them.

**Library-Style Contract Exports:**
- Purpose: Allow one contract crate to depend on another crate's message types or Sylvia-generated interfaces.
- Examples: root `Cargo.toml` workspace dependencies for `nft-vault`, `stake-rewards`, `pg721`, and `pg721-updatable`; imports in `contracts/staking/vault-factory/src/contract.rs`
- Pattern: Reusable contract crates expose library interfaces through `crate-type = ["cdylib", "rlib"]`, then sibling crates import instantiate/query message types instead of duplicating them.

## Entry Points

**Workspace Build Entry Point:**
- Location: `Cargo.toml`
- Triggers: `cargo check`, `cargo test`, `cargo run --example schema`, optimizer scripts
- Responsibilities: declare workspace members, pin shared dependencies, and define release profiles for each contract crate.

**Registry Contract Entry Points:**
- Location: `contracts/core/registry/src/contract/instantiate.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/core/registry/src/contract/query.rs`
- Triggers: Wasm `instantiate`, `execute`, and `query`
- Responsibilities: initialize global config, apply moderation and recovery actions, register ecosystems and collections, and answer policy queries consumed by other contracts.

**Factory Reply Entry Points:**
- Location: `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/nft/minter-v2/src/contract/instantiate.rs`
- Triggers: `SubMsg::reply_on_success` after child contract instantiation
- Responsibilities: parse instantiate replies, turn pending state into permanent records, and wire newly created addresses into registry or config state.

**Marketplace and Minter Entry Points:**
- Location: `contracts/nft/marketplace-v3/src/contract/*.rs`, `contracts/nft/minter-v2/src/contract/*.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/*.rs`
- Triggers: user mint, list, bid, buy, migrate, and query messages
- Responsibilities: enforce local and registry-backed rules, mutate order books and mint counters, and emit downstream CW721 or bank messages.

**Sylvia Contract Entry Points:**
- Location: `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/stake-rewards/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`
- Triggers: Sylvia-generated Wasm entry points from `#[contract]` and `#[sv::msg(...)]`
- Responsibilities: hide raw entry-point boilerplate while exposing typed instantiate/exec/query methods for staking and reward flows.

**Schema Generation Entry Points:**
- Location: `contracts/*/*/examples/schema.rs`, `contracts/staking/*/src/bin/schema.rs`
- Triggers: `cargo run --example schema` or `cargo run --bin schema --features schema`
- Responsibilities: generate JSON schema directories committed alongside each contract crate and, for the root, in `schema/`.

## Error Handling

**Strategy:** Per-crate typed errors plus early validation, with queries returning `StdResult` and execute paths converting failures into domain-specific `ContractError` variants.

**Patterns:**
- Authorization and pause checks happen before state mutation. Representative implementations are in `contracts/core/registry/src/contract/execute.rs`, `contracts/core/split-router/src/contract/execute.rs`, and `contracts/nft/marketplace-v3/src/contract/execute.rs`.
- Address parsing and structural validation happen at the boundary using `deps.api.addr_validate`, `nonpayable`, URL parsing, or custom helper validators in files such as `contracts/core/registry/src/contract/helpers.rs` and `contracts/nft/pg721/src/contract.rs`.
- Reply-based workflows protect themselves with pending-state lookups and explicit parse errors in `contracts/core/ecosystem-factory/src/contract/reply.rs` and `contracts/core/collection-factory/src/contract/reply.rs`.
- Query helpers often downgrade contract-call failures into domain decisions. Examples include registry gating in `contracts/nft/minter-v2/src/contract/helpers.rs` and trade checks in `contracts/nft/marketplace-v3/src/contract/helpers.rs`.

## Cross-Cutting Concerns

**Logging:** Contracts emit `Response` attributes and `Event`s instead of using a shared logger. Examples: `contracts/core/streaming-billing/src/contract.rs`, `contracts/staking/nft-vault/src/contract.rs`, `contracts/relationship/follow/src/execute.rs`.
**Validation:** Inputs are validated at message boundaries with helpers and typed parsers in `contracts/core/registry/src/contract/helpers.rs`, `contracts/nft/pg721/src/contract.rs`, `contracts/core/ecosystem-factory/src/contract/helpers.rs`, and Sylvia utilities like `nonpayable` and `only_contract_admin`.
**Authentication:** Admin and operator patterns dominate direct CosmWasm contracts (`contracts/core/collection-factory/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`), while staking uses `only_contract_admin` and ownership checks in `contracts/staking/nft-vault/src/contract.rs`; relationship contracts authenticate via sender-owned hook lists in `contracts/relationship/follow/src/execute.rs`.

---

*Architecture analysis: 2026-03-17*
