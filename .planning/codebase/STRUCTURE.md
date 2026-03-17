# Codebase Structure

**Analysis Date:** 2026-03-17

## Directory Layout

```text
passage-contracts/
??? Cargo.toml                  # Workspace manifest and shared dependency/release config
??? contracts/                  # Deployable contract crates grouped by business domain
?   ??? core/                   # Registry, factories, governance, routing, billing
?   ??? nft/                    # Collection primitives, minters, marketplaces, auctions
?   ??? relationship/           # Social graph contracts
?   ??? staking/                # Sylvia-based vault and reward contracts
??? contract-template/          # Scaffold for new direct-entry-point contracts
??? scripts/                    # Wasm optimization helpers
??? artifacts/                  # Built Wasm binaries and checksum output
??? schema/                     # Root-level JSON schema output
??? docs/                       # Narrative documentation and design notes
??? .planning/codebase/         # Mapper-generated repository reference docs
```

## Directory Purposes

**`contracts/core`:**
- Purpose: Hold protocol control-plane contracts and contract-to-contract orchestration.
- Contains: `collection-factory`, `ecosystem-factory`, `multisig`, `registry`, `split-router`, `streaming-billing`
- Key files: `contracts/core/registry/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/streaming-billing/src/contract.rs`

**`contracts/nft`:**
- Purpose: Hold NFT collection primitives and commerce contracts.
- Contains: collection contracts such as `pg721`, minter variants such as `minter-v2`, trading contracts such as `marketplace-v3` and `auction-english`, and support contracts like `royalty-group` and `whitelist`
- Key files: `contracts/nft/pg721/src/contract.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`

**`contracts/staking`:**
- Purpose: Hold staking and reward contracts implemented with Sylvia.
- Contains: `nft-vault`, `stake-rewards`, `vault-factory`
- Key files: `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/stake-rewards/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`

**`contracts/relationship`:**
- Purpose: Hold relationship graph contracts with hook-based side effects.
- Contains: `follow`, `friend`
- Key files: `contracts/relationship/follow/src/execute.rs`, `contracts/relationship/follow/src/state.rs`, `contracts/relationship/friend/src/query.rs`

**`contract-template`:**
- Purpose: Provide the baseline file layout for a new direct CosmWasm contract.
- Contains: `src/lib.rs`, `src/instantiate.rs`, `src/execute.rs`, `src/query.rs`, `src/state.rs`, `examples/schema.rs`
- Key files: `contract-template/src/lib.rs`, `contract-template/src/instantiate.rs`

**`scripts`:**
- Purpose: Hold repeatable build helpers for Wasm optimization.
- Contains: shell wrappers around `cosmwasm/optimizer`
- Key files: `scripts/optimize.sh`, `scripts/optimize-arm.sh`

**`artifacts`:**
- Purpose: Store built `.wasm` binaries and checksums.
- Contains: one Wasm per compiled contract plus `checksums.txt`
- Key files: `artifacts/registry.wasm`, `artifacts/marketplace_v3.wasm`, `artifacts/checksums.txt`

**`schema`:**
- Purpose: Store root-level schema output for the currently generated contract set.
- Contains: `execute_msg.json`, `instantiate_msg.json`, `migrate_msg.json`, `query_msg.json`
- Key files: `schema/execute_msg.json`, `schema/query_msg.json`

**`docs`:**
- Purpose: Keep prose design notes that explain business intent and contract interactions.
- Contains: architecture summaries, redesign notes, and topic-specific explainers
- Key files: `ARCHITECTURE_SUMMARY.md`, `ECOSYSTEM_FEE_PATTERN.md`, `REVENUE_ROUTER_EXPLAINED.md`

## Key File Locations

**Entry Points:**
- `Cargo.toml`: workspace root for all crate membership and shared dependency versions
- `contracts/core/registry/src/lib.rs`: exports registry modules and test module
- `contracts/core/registry/src/contract/instantiate.rs`: split entry-point style instantiate handler
- `contracts/nft/marketplace-v3/src/contract/execute.rs`: split entry-point style execute handler
- `contracts/nft/marketplace-legacy/src/execute.rs`: legacy direct execute entry point kept outside a `contract/` subdirectory
- `contracts/staking/nft-vault/src/contract.rs`: Sylvia contract implementation that generates Wasm entry points from annotated methods
- `contracts/relationship/follow/src/instantiate.rs`: simple direct instantiate entry point for relationship contracts

**Configuration:**
- `Cargo.toml`: workspace dependencies, release profiles, and local library crate wiring
- `rust-toolchain.toml`: Rust toolchain pinning
- `.github/workflows/main.yaml`: CI entry point
- `scripts/optimize.sh`: canonical optimizer invocation for release artifacts
- `contracts/*/*/Cargo.toml`: per-crate package metadata, feature flags, and schema binaries/examples

**Core Logic:**
- `contracts/core/registry/src/contract/*.rs`: ecosystem, collection, moderation, and recovery logic
- `contracts/core/collection-factory/src/contract/*.rs`: collection creation orchestration and reply handling
- `contracts/nft/minter-v2/src/contract/*.rs`: minting lifecycle, reply handling, and migration
- `contracts/nft/marketplace-v3/src/contract/*.rs`: trade registration, ask/bid execution, and query helpers
- `contracts/staking/nft-vault/src/contract.rs`: staking state machine and reward-account orchestration

**Testing:**
- `contracts/core/registry/src/tests/`: registry integration-style test modules
- `contracts/core/multisig/src/tests/`: multisig governance tests
- `contracts/core/split-router/src/contract/execute/tests.rs`: submodule-local tests
- `contracts/nft/auction-english/src/execute/tests.rs`: feature-local execute tests
- `contracts/nft/minter/src/contract_tests.rs`: crate-level test file for older minter layout
- `contracts/relationship/follow/src/multitest.rs`: contract integration tests using multi-test

## Naming Conventions

**Files:**
- Core contract modules use snake_case filenames such as `contract.rs`, `state.rs`, `msg.rs`, `error.rs`, `helpers.rs`, `reply.rs`, and `migration.rs`.
- Split-handler crates place lifecycle files inside `src/contract/`, for example `contracts/core/registry/src/contract/execute.rs` and `contracts/nft/marketplace-v3/src/contract/query.rs`.
- Schema generators are named `examples/schema.rs` in direct-entry-point crates and `src/bin/schema.rs` in Sylvia crates such as `contracts/staking/nft-vault/src/bin/schema.rs`.

**Directories:**
- Contract crate directories use kebab-case names such as `collection-factory`, `marketplace-v3`, `nft-vault`, and `streaming-billing`.
- Internal module directories mirror lifecycle concepts: `src/contract/`, `src/tests/`, `src/claim/`, and `examples/`.
- Domain grouping is fixed at the top level: add new crates under `contracts/core`, `contracts/nft`, `contracts/staking`, or `contracts/relationship`, not at the repository root.

## Where to Add New Code

**New Feature:**
- Primary code: extend the existing crate inside its current style. Use `src/contract/*.rs` in split-handler crates such as `contracts/core/registry` or `contracts/nft/marketplace-v3`; use `src/contract.rs` or `src/execute.rs` / `src/query.rs` in legacy crates such as `contracts/nft/marketplace-legacy` or `contracts/relationship/follow`.
- Tests: keep tests next to the style already used by that crate. Examples are `contracts/core/registry/src/tests/`, `contracts/nft/auction-english/src/execute/tests.rs`, and `contracts/relationship/follow/src/multitest.rs`.

**New Component/Module:**
- Implementation: create a new sibling crate under the correct domain directory with its own `Cargo.toml`, `src/lib.rs`, `src/msg.rs`, `src/state.rs`, `src/error.rs`, and entry-point implementation. Use `contract-template/` as the baseline for direct CosmWasm crates, or mirror `contracts/staking/nft-vault` when the new contract should use Sylvia.

**Utilities:**
- Shared helpers: prefer crate-local `helpers.rs` or `src/contract/helpers.rs` first. This repo does not use a global shared `src/common` crate.
- Cross-crate reuse: expose reusable logic as a real workspace crate only when another contract must import it as a dependency. Current examples are `nft-vault`, `stake-rewards`, `pg721`, and `pg721-updatable` referenced from the workspace `Cargo.toml`.

## Special Directories

**`artifacts/`:**
- Purpose: optimized Wasm build output and checksums
- Generated: Yes
- Committed: Yes

**`schema/`:**
- Purpose: root-level JSON schemas exported by schema generation commands
- Generated: Yes
- Committed: Yes

**`contracts/*/*/schema/`:**
- Purpose: contract-local JSON schema directories generated from `examples/schema.rs` or `src/bin/schema.rs`
- Generated: Yes
- Committed: Yes

**`target/`:**
- Purpose: Cargo compilation output
- Generated: Yes
- Committed: No

**`contract-template/`:**
- Purpose: starter layout for new contract crates
- Generated: No
- Committed: Yes

**`.planning/codebase/`:**
- Purpose: generated reference docs for later GSD planning and execution commands
- Generated: Yes
- Committed: Usually yes when refreshing the codebase map

## Contract Crate Layout Patterns

**Split-handler crate pattern:**
- Shape: `src/lib.rs` + `src/msg.rs` + `src/state.rs` + `src/error.rs` + `src/contract/{instantiate,execute,query,reply,migrate,helpers}.rs`
- Use it when: extending crates such as `contracts/core/registry`, `contracts/core/collection-factory`, `contracts/nft/minter-v2`, or `contracts/nft/marketplace-v3`
- Example paths: `contracts/core/registry/src/contract/execute.rs`, `contracts/nft/minter-v2/src/contract/migrate.rs`

**Legacy direct-entry-point pattern:**
- Shape: `src/lib.rs` plus top-level `src/instantiate.rs`, `src/execute.rs`, `src/query.rs`, or a single `src/contract.rs`
- Use it when: modifying crates that already follow this layout; do not refactor them into `src/contract/` just to add one feature
- Example paths: `contracts/relationship/follow/src/execute.rs`, `contracts/nft/marketplace-legacy/src/query.rs`, `contracts/core/streaming-billing/src/contract.rs`

**Sylvia contract pattern:**
- Shape: a single annotated `src/contract.rs` plus supporting modules, often with `src/bin/schema.rs`
- Use it when: working in `contracts/staking/*`
- Example paths: `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`, `contracts/staking/stake-rewards/src/bin/schema.rs`

---

*Structure analysis: 2026-03-17*
