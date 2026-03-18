# Architecture

**Analysis Date:** 2026-03-18

## Pattern Overview

**Overall:** Domain-segmented CosmWasm workspace with contract-local state machines and a small set of shared protocol control points.

**Key Characteristics:**
- The root `Cargo.toml` defines a single Rust workspace with four contract domains under `contracts/core/*`, `contracts/nft/*`, `contracts/relationship/*`, and `contracts/staking/*`.
- `contracts/core/registry` is the closest thing to a protocol source of truth: it owns ecosystem, collection, moderation, and recovery state, and other contracts query it instead of duplicating policy.
- The codebase uses multiple internal module styles on purpose. Split-handler crates keep lifecycle code in `src/contract/*.rs` such as `contracts/core/registry/src/contract/execute.rs` and `contracts/nft/marketplace-v3/src/contract/query.rs`, while older contracts keep direct files like `contracts/nft/marketplace-legacy/src/execute.rs` and Sylvia-based staking contracts centralize logic in `src/contract.rs` such as `contracts/staking/nft-vault/src/contract.rs`.

## Layers

**Protocol Control Plane:**
- Purpose: Own registry state, factory orchestration, governance, routing, and protocol-level billing decisions.
- Contains: `contracts/core/registry`, `contracts/core/collection-factory`, `contracts/core/ecosystem-factory`, `contracts/core/multisig`, `contracts/core/split-router`, `contracts/core/streaming-billing`
- Depends on: `cw-storage-plus`, CosmWasm entry points, and cross-contract messages defined in files such as `contracts/core/registry/src/msg.rs` and `contracts/core/collection-factory/src/msg.rs`.
- Used by: NFT contracts, staking contracts, and off-chain callers that treat the registry as the canonical catalog.

**NFT Commerce Layer:**
- Purpose: Handle collection primitives, minting, trading, royalties, allowlists, and metadata variants.
- Contains: `contracts/nft/pg721`, `contracts/nft/pg721-updatable`, `contracts/nft/pg721-metadata-onchain`, `contracts/nft/minter`, `contracts/nft/minter-v2`, `contracts/nft/minter-v2-metadata-onchain`, `contracts/nft/marketplace-legacy`, `contracts/nft/marketplace-v2`, `contracts/nft/marketplace-v3`, `contracts/nft/auction-english`, `contracts/nft/royalty-group`, `contracts/nft/whitelist`
- Depends on: contract-local config and state, registry queries from helpers such as `contracts/nft/minter-v2/src/contract/helpers.rs`, and CW721-compatible messages or queries.
- Used by: creators, marketplaces, minters, and factory contracts.

**Staking Layer:**
- Purpose: Manage NFT vaults, reward distribution, and vault deployment.
- Contains: `contracts/staking/nft-vault`, `contracts/staking/stake-rewards`, `contracts/staking/vault-factory`
- Depends on: Sylvia-generated entry points, local `state.rs` / `error.rs` modules, and workspace dependencies declared in `Cargo.toml`.
- Used by: vault creation flows and any caller staking approved collections.

**Relationship Layer:**
- Purpose: Model social graph edges and emit hook notifications on relationship changes.
- Contains: `contracts/relationship/follow`, `contracts/relationship/friend`
- Depends on: local `msg.rs`, `state.rs`, `hooks.rs`, and helper modules in each crate.
- Used by: contracts or indexers that subscribe to follow/friend events.

**Contract-Local State Machine Layer:**
- Purpose: Keep each contract self-contained around messages, entry points, validation, state, and replies.
- Contains: `src/lib.rs`, `src/msg.rs`, `src/state.rs`, `src/error.rs`, plus either `src/contract.rs`, `src/contract/*.rs`, or direct top-level `src/instantiate.rs` / `src/execute.rs` / `src/query.rs`.
- Depends on: CosmWasm APIs such as `Deps`, `DepsMut`, `Env`, `MessageInfo`, `Response`, and `StdResult`.
- Used by: every deployable crate in the workspace.

## Data Flow

**Registry-Gated Creation Flow:**

1. A factory contract such as `contracts/core/ecosystem-factory/src/contract/execute.rs` or `contracts/core/collection-factory/src/contract/execute.rs` receives a creation request.
2. It validates permissions against `contracts/core/registry` using query messages defined in `contracts/core/registry/src/msg.rs`.
3. It stores pending request state in crate-local storage, then sends a submessage to instantiate a downstream contract.
4. The reply handler, such as `contracts/core/ecosystem-factory/src/contract/reply.rs` or `contracts/core/collection-factory/src/contract/reply.rs`, finalizes local state and registers the new object back with the registry.

**Mint and Trade Flow:**

1. A minter or marketplace entry point such as `contracts/nft/minter-v2/src/contract/execute.rs` or `contracts/nft/marketplace-v3/src/contract/execute.rs` receives a user action.
2. Helper code checks local config and registry-backed authorization before any mutation.
3. The contract updates its own state maps and emits downstream CW721, bank, or split-router messages.
4. Query handlers expose the resulting state through typed response structs from `msg.rs`.

**Staking Flow:**

1. `contracts/staking/vault-factory/src/contract.rs` instantiates vault contracts with predicted addresses.
2. `contracts/staking/nft-vault/src/contract.rs` records staked NFTs and orchestrates reward-account linkage.
3. `contracts/staking/stake-rewards/src/contract.rs` maintains reward distribution state tied to the vault lifecycle.

**State Management:**
- Persistent state is explicit and crate-local. Examples include `CONFIG` and collection policy maps in `contracts/core/registry/src/state.rs`, `COLLECTION_CONFIGS` and order-book storage in `contracts/nft/marketplace-v3/src/state.rs`, and staking records in `contracts/staking/nft-vault/src/state.rs`.
- Query-heavy data uses `IndexedMap`, `MultiIndex`, or other keyed collections instead of raw scans when a contract needs pagination or alternate lookup dimensions.
- Reply-driven flows keep pending metadata in storage before dispatching `SubMsg::reply_on_success`, which appears in factory crates and in minter instantiation flows.

## Key Abstractions

**Registry Catalog and Policy Surface:**
- Purpose: Hold the canonical protocol record for ecosystems, collections, minter authorization, moderation, and recovery.
- Examples: `contracts/core/registry/src/state.rs`, `contracts/core/registry/src/msg.rs`, `contracts/core/registry/src/contract/helpers.rs`
- Pattern: One contract owns protocol-wide data, while other contracts consume it through queries and explicit execute messages.

**Pending Creation Queue:**
- Purpose: Bridge asynchronous instantiation with later registration and bookkeeping.
- Examples: `contracts/core/ecosystem-factory/src/state.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/state.rs`, `contracts/core/collection-factory/src/contract/reply.rs`
- Pattern: Save request metadata before sending a submessage, then reconcile reply data back into permanent state.

**Per-Contract Config Object:**
- Purpose: Centralize admin addresses, code IDs, pricing, and protocol references.
- Examples: `contracts/core/split-router/src/state.rs`, `contracts/core/streaming-billing/src/state.rs`, `contracts/nft/minter-v2/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`
- Pattern: A `Config` struct plus a top-level `CONFIG: Item<Config>` controls privileged paths and integration points.

**Indexed Storage Model:**
- Purpose: Support paginated queries and alternate lookup dimensions without off-chain joins.
- Examples: `contracts/core/registry/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`, `contracts/relationship/follow/src/state.rs`
- Pattern: Secondary indexes are added where query handlers need them, rather than creating separate caches.

**Library-Style Contract Exports:**
- Purpose: Allow one contract crate to depend on another crate's message types or generated interfaces.
- Examples: workspace dependencies for `nft-vault`, `stake-rewards`, `pg721`, and `pg721-updatable` in `Cargo.toml`
- Pattern: Reusable crates expose library interfaces, then sibling crates import message types instead of duplicating them.

## Entry Points

**Workspace Build Entry Point:**
- Location: `Cargo.toml`
- Triggers: `cargo check`, `cargo test`, schema generation, and optimizer scripts
- Responsibilities: declare workspace members, pin shared dependencies, and define release profiles for each contract crate.

**Registry Contract Entry Points:**
- Location: `contracts/core/registry/src/contract.rs` and the split modules under `contracts/core/registry/src/contract/*.rs`
- Triggers: Wasm `instantiate`, `execute`, `query`, and tests in `contracts/core/registry/src/tests/`
- Responsibilities: initialize global config, apply moderation and recovery actions, register ecosystems and collections, and answer policy queries.

**Factory and Minter Reply Entry Points:**
- Location: `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/nft/minter-v2/src/contract/instantiate.rs`
- Triggers: `SubMsg::reply_on_success` after contract instantiation
- Responsibilities: parse instantiate replies, turn pending state into permanent records, and wire newly created addresses into local or registry state.

**Marketplace and Minter Entry Points:**
- Location: `contracts/nft/marketplace-v3/src/contract/*.rs`, `contracts/nft/minter-v2/src/contract/*.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/*.rs`
- Triggers: mint, list, bid, buy, migrate, and query messages
- Responsibilities: enforce local and registry-backed rules, mutate order books and mint counters, and emit downstream CW721 or bank messages.

**Sylvia Contract Entry Points:**
- Location: `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/stake-rewards/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`
- Triggers: Sylvia-generated Wasm entry points
- Responsibilities: hide boilerplate while exposing typed instantiate, execute, and query methods for staking and reward flows.

**Schema Generation Entry Points:**
- Location: `contracts/*/*/examples/schema.rs`, `contracts/staking/*/src/bin/schema.rs`
- Triggers: `cargo run --example schema` or `cargo run --bin schema`
- Responsibilities: generate JSON schema directories committed alongside each contract crate and, for the root workspace, in `schema/`.

## Error Handling

**Strategy:** Typed per-crate errors plus early validation. Queries return `StdResult`, and execute paths convert failures into contract-specific `ContractError` variants.

**Patterns:**
- Authorization and pause checks happen before state mutation in contracts such as `contracts/core/registry/src/contract/execute.rs`, `contracts/core/split-router/src/contract/execute.rs`, and `contracts/nft/marketplace-v3/src/contract/execute.rs`.
- Boundary validation uses `deps.api.addr_validate`, `nonpayable`, URL parsing, and helper validators in files such as `contracts/core/registry/src/contract/helpers.rs` and `contracts/nft/pg721/src/contract.rs`.
- Reply-based workflows protect themselves with pending-state lookups and explicit parse errors in `contracts/core/ecosystem-factory/src/contract/reply.rs` and `contracts/core/collection-factory/src/contract/reply.rs`.
- Helper code often converts failed contract calls into domain decisions instead of panicking, as seen in registry gating and trade checks for minters and marketplaces.

## Cross-Cutting Concerns

**Logging:** Contracts emit `Response` attributes and `Event`s instead of using a shared logger. Examples: `contracts/core/streaming-billing/src/contract.rs`, `contracts/staking/nft-vault/src/contract.rs`, `contracts/relationship/follow/src/execute.rs`.

**Validation:** Inputs are validated at message boundaries with typed parsers and helper functions in `contracts/core/registry/src/contract/helpers.rs`, `contracts/nft/pg721/src/contract.rs`, `contracts/core/ecosystem-factory/src/contract/helpers.rs`, and Sylvia utilities such as `nonpayable` and `only_contract_admin`.

**Authentication:** Admin and operator patterns dominate direct CosmWasm contracts such as `contracts/core/collection-factory/src/state.rs` and `contracts/nft/marketplace-v3/src/state.rs`, while staking uses `only_contract_admin` and ownership checks in `contracts/staking/nft-vault/src/contract.rs`; relationship contracts authenticate via sender-owned hook lists in `contracts/relationship/follow/src/execute.rs`.

---

*Architecture analysis: 2026-03-18*