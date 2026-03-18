# Codebase Structure

**Analysis Date:** 2026-03-18

## Directory Layout

```text
passage-contracts/
├── Cargo.toml                  # Workspace manifest, shared deps, release profiles
├── Cargo.lock                  # Locked dependency versions
├── README.md                   # High-level project overview and deploy examples
├── CLAUDE.md                   # Agent instructions for this repository
├── rust-toolchain.toml         # Rust toolchain pin
├── contract-template/          # Starter layout for a direct CosmWasm contract
├── contracts/                  # Deployable contract crates grouped by domain
│   ├── core/                   # Registry, factories, governance, routing, billing
│   ├── nft/                    # Collection, minting, marketplace, and royalty contracts
│   ├── relationship/          # Social graph contracts
│   └── staking/                # Sylvia-based staking and reward contracts
├── docs/                       # End-to-end setup and method-reference docs
├── scripts/                    # Wasm optimization helpers
├── artifacts/                  # Built Wasm binaries and checksums
├── schema/                     # Root-level generated JSON schema output
└── .planning/                  # Project state and generated codebase maps
    └── codebase/               # Architecture and structure reference docs
```

## Directory Purposes

**`contracts/core`:**
- Purpose: Hold protocol control-plane contracts and contract-to-contract orchestration.
- Contains: `collection-factory`, `ecosystem-factory`, `multisig`, `registry`, `split-router`, `streaming-billing`
- Key files: `contracts/core/registry/src/contract.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/split-router/src/contract.rs`, `contracts/core/multisig/src/contract.rs`
- Subdirectories: Each crate contains its own `src/`, `examples/`, and sometimes `src/tests/` or `src/contract/` tree.

**`contracts/nft`:**
- Purpose: Hold NFT collection primitives and commerce contracts.
- Contains: collection contracts such as `pg721`, minter variants such as `minter-v2`, trading contracts such as `marketplace-v3` and `auction-english`, and support contracts like `royalty-group` and `whitelist`
- Key files: `contracts/nft/pg721/src/contract.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`, `contracts/nft/auction-english/src/execute.rs`, `contracts/nft/whitelist/src/contract.rs`
- Subdirectories: The crate layouts vary by age, with direct `src/*.rs`, split `src/contract/*.rs`, and `src/contract.rs` styles all present.

**`contracts/relationship`:**
- Purpose: Hold relationship graph contracts with hook-based side effects.
- Contains: `follow`, `friend`
- Key files: `contracts/relationship/follow/src/execute.rs`, `contracts/relationship/follow/src/hooks.rs`, `contracts/relationship/follow/src/multitest.rs`, `contracts/relationship/friend/src/query.rs`
- Subdirectories: Both crates keep direct top-level `src/*.rs` modules and supporting helpers or hooks.

**`contracts/staking`:**
- Purpose: Hold staking and reward contracts implemented with Sylvia.
- Contains: `nft-vault`, `stake-rewards`, `vault-factory`
- Key files: `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/nft-vault/src/claim.rs`, `contracts/staking/nft-vault/src/bin/schema.rs`, `contracts/staking/stake-rewards/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`
- Subdirectories: These crates expose a `src/contract.rs` entry point, plus support modules and schema generators under `src/bin/` or `examples/`.

**`contract-template`:**
- Purpose: Provide the baseline file layout for a new direct CosmWasm contract.
- Contains: `src/lib.rs`, `src/instantiate.rs`, `src/execute.rs`, `src/query.rs`, `src/state.rs`, `src/helpers.rs`, `src/error.rs`, `examples/schema.rs`
- Key files: `contract-template/src/lib.rs`, `contract-template/src/execute.rs`, `contract-template/src/query.rs`
- Subdirectories: `src/` for contract modules, `examples/` for schema generation, `.cargo/` for local cargo config.

**`docs`:**
- Purpose: Keep developer-facing documentation and setup notes.
- Contains: end-to-end setup, JSON examples, method reference, and multisig governance docs.
- Key files: `docs/README.md`, `docs/01-end-to-end-setup.md`, `docs/02-method-reference.md`, `docs/03-json-examples.md`, `docs/04-multisig-governance.md`
- Subdirectories: None.

**`scripts`:**
- Purpose: Hold repeatable build helpers for Wasm optimization.
- Contains: shell wrappers around the CosmWasm optimizer workflow.
- Key files: `scripts/optimize.sh`, `scripts/optimize-arm.sh`
- Subdirectories: None.

**`artifacts`:**
- Purpose: Store built `.wasm` binaries and checksum output.
- Contains: compiled contract artifacts and `checksums.txt`
- Key files: the generated Wasm files under `artifacts/` and checksum metadata
- Subdirectories: None.

**`schema`:**
- Purpose: Store root-level schema output for the currently generated contract set.
- Contains: `execute_msg.json`, `instantiate_msg.json`, `migrate_msg.json`, `query_msg.json`
- Key files: `schema/execute_msg.json`, `schema/query_msg.json`
- Subdirectories: None.

**`.planning/codebase`:**
- Purpose: Generated repository reference docs used by GSD planning and execution commands.
- Contains: `ARCHITECTURE.md` and `STRUCTURE.md`
- Key files: the two mapping docs in this directory
- Subdirectories: None.

## Key File Locations

**Entry Points:**
- `Cargo.toml`: workspace root for crate membership and shared dependency versions
- `README.md`: project overview, diagrams, and deploy commands
- `contracts/core/registry/src/contract.rs`: registry contract entry surface
- `contracts/nft/marketplace-v3/src/contract.rs`: split-handler marketplace entry surface
- `contracts/staking/nft-vault/src/contract.rs`: Sylvia contract entry surface
- `contracts/relationship/follow/src/instantiate.rs`: direct entry-point style for relationship contracts

**Configuration:**
- `Cargo.toml`: workspace dependencies, release profiles, and local library crate wiring
- `rust-toolchain.toml`: Rust version pin
- `contract-template/.cargo/config`: template-local cargo configuration
- `contracts/*/*/Cargo.toml`: per-crate package metadata, feature flags, and schema binaries or examples

**Core Logic:**
- `contracts/core/registry/src/contract/*.rs`: registry execution, query, helpers, and instantiate logic
- `contracts/core/collection-factory/src/contract/*.rs`: collection creation orchestration and reply handling
- `contracts/core/ecosystem-factory/src/contract/*.rs`: ecosystem creation orchestration and reply handling
- `contracts/nft/minter-v2/src/contract/*.rs`: mint lifecycle, reply handling, migration, and helpers
- `contracts/nft/marketplace-v3/src/contract/*.rs`: trade registration, ask/bid execution, and query helpers
- `contracts/staking/nft-vault/src/contract.rs`: staking state machine and reward-account orchestration

**Testing:**
- `contracts/core/registry/src/tests/`: registry integration-style tests
- `contracts/core/multisig/src/tests/`: governance tests
- `contracts/core/split-router/src/contract/execute/tests.rs`: submodule-local tests
- `contracts/nft/auction-english/src/execute/tests.rs`: execute-path tests
- `contracts/nft/minter-v2/src/contract/helpers/tests.rs`: helper-path tests
- `contracts/relationship/follow/src/multitest.rs`: multi-test integration coverage
- `contracts/staking/nft-vault/src/contract/tests.rs`: staking contract tests

**Documentation:**
- `README.md`: user-facing overview and deploy examples
- `docs/README.md`: documentation index
- `.planning/codebase/ARCHITECTURE.md`: conceptual codebase map
- `.planning/codebase/STRUCTURE.md`: physical codebase map
- `contracts/*/README.md`: crate-specific contract docs and usage notes

## Naming Conventions

**Files:**
- Core contract modules use snake_case filenames such as `contract.rs`, `state.rs`, `msg.rs`, `error.rs`, `helpers.rs`, `reply.rs`, and `migration.rs`.
- Split-handler crates place lifecycle files inside `src/contract/`, for example `contracts/core/registry/src/contract/execute.rs` and `contracts/nft/marketplace-v3/src/contract/query.rs`.
- Schema generators are named `examples/schema.rs` in direct-entry-point crates and `src/bin/schema.rs` in Sylvia crates such as `contracts/staking/nft-vault/src/bin/schema.rs`.

**Directories:**
- Contract crate directories use kebab-case names such as `collection-factory`, `marketplace-v3`, `nft-vault`, and `streaming-billing`.
- Internal module directories mirror lifecycle concepts: `src/contract/`, `src/tests/`, `src/bin/`, and `examples/`.
- Domain grouping is fixed at the top level: add new crates under `contracts/core`, `contracts/nft`, `contracts/staking`, or `contracts/relationship`, not at the repository root.

**Special Patterns:**
- `src/lib.rs` exports the crate module surface and shared error type.
- `src/contract.rs` is common in Sylvia crates and some modern direct contracts.
- `examples/schema.rs` or `src/bin/schema.rs` generates contract JSON schema.

## Where to Add New Code

**New Feature:**
- Primary code: extend the existing crate inside its current style. Use `src/contract/*.rs` in split-handler crates such as `contracts/core/registry` or `contracts/nft/marketplace-v3`; use direct `src/*.rs` files in older crates such as `contracts/relationship/follow` or `contracts/nft/marketplace-legacy`.
- Tests: keep tests next to the style already used by that crate. Examples are `contracts/core/registry/src/tests/`, `contracts/nft/auction-english/src/execute/tests.rs`, and `contracts/relationship/follow/src/multitest.rs`.

**New Component/Module:**
- Implementation: create a new sibling crate under the correct domain directory with its own `Cargo.toml`, `src/lib.rs`, `src/msg.rs`, `src/state.rs`, `src/error.rs`, and entry-point implementation. Use `contract-template/` as the baseline for direct CosmWasm crates, or mirror `contracts/staking/nft-vault` when the new contract should use Sylvia.
- Tests: place crate-local coverage next to the implementation style the new crate chooses.

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

*Structure analysis: 2026-03-18*