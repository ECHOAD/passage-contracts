# Coding Conventions

**Analysis Date:** 2026-03-18

## Naming Patterns

**Files:**
- Keep the standard contract surface in `src/lib.rs`, `src/error.rs`, `src/msg.rs`, and `src/state.rs`, then use either a monolithic `src/contract.rs` or split `src/contract/*.rs` modules depending on the crate layout. Representative examples include `contracts/core/split-router/src/contract.rs`, `contracts/core/multisig/src/contract.rs`, and `contracts/nft/minter-v2/src/contract.rs`.
- Put handler-adjacent tests next to the code they exercise when the crate already uses that style, such as `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/core/ecosystem-factory/src/contract/execute/tests.rs`, `contracts/nft/minter-v2/src/contract/helpers/tests.rs`, and `contracts/nft/marketplace-v3/src/contract/query/tests.rs`.
- Use `src/tests/*.rs` for crate-level suites where that pattern already exists, as in `contracts/core/multisig/src/tests/governance.rs`, or `src/contract_tests.rs` for broader crate-local scenarios like `contracts/nft/minter/src/contract_tests.rs` and `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`.
- Use `multitest.rs` for multi-contract scenario suites in crates that already rely on `cw-multi-test`, such as `contracts/relationship/follow/src/multitest.rs` and `contracts/nft/marketplace-v2/src/multitest.rs`.
- Keep schema helpers in `examples/schema.rs` or `src/bin/schema.rs` where those entrypoints already exist.

**Functions:**
- Use `snake_case` for all functions. CosmWasm entrypoints should remain named `instantiate`, `execute`, `query`, and `migrate` when the crate exposes those handlers.
- Prefix helper functions with their purpose, such as `execute_update_config`, `execute_split`, `query_world_stats`, `setup_contract`, `save_base_state`, and `mock_queries`.
- Prefer behavior-driven test names in new suites, for example `split_sends_remainder_to_last_recipient_and_records_event` in `contracts/core/split-router/src/contract/execute/tests.rs` and `asks_by_price_respects_collection_filter` in `contracts/nft/marketplace-v3/src/contract/query/tests.rs`.
- Legacy suites still contain older names like `proper_initialization`, `test_claim_tokens_with_all_released_claims`, and `test_instantiate_follow_contract`; keep those unchanged unless you are actively rewriting the file.

**Variables:**
- Use `snake_case` for locals, parameters, and storage-derived values such as `world_nft_id`, `user_balance`, `collection_deltas`, and `reward_accounts`.
- Use `SCREAMING_SNAKE_CASE` for constants and storage singletons such as `CONTRACT_NAME`, `CONTRACT_VERSION`, `DEFAULT_LIMIT`, `MAX_LIMIT`, `CONFIG`, and `MINT_STATS`.
- Short temporary names like `res`, `err`, `msg`, and `ctx` are acceptable inside tight scopes, but broader helpers should stay explicit.

**Types:**
- Use `UpperCamelCase` for structs, enums, and errors: `ContractError`, `InstantiateMsg`, `QueryMsg`, `ConfigResponse`, `MintStats`.
- Suffix wire-format types with `Msg` or `Response` where it makes the API shape clearer.
- Keep storage models short and domain-named, for example `Config`, `Collection`, `PendingRevenue`, and `WorldStats`.
- Serialize external-facing enums in `snake_case` through `#[serde(rename_all = "snake_case")]` or `#[cw_serde]`, as seen in `contracts/core/registry/src/state.rs` and `contracts/nft/minter-v2/src/msg.rs`.

## Code Style

**Formatting:**
- Use `rustfmt`; `rust-toolchain.toml` already pins `rustfmt` and `clippy`, and there is no root `rustfmt.toml` override.
- Keep long imports, struct literals, and response builders vertically formatted with trailing commas, matching files like `contracts/core/multisig/src/contract.rs` and `contracts/nft/minter-v2/src/contract.rs`.
- Prefer one chained response call per line for stateful handlers, and keep section-banner comments in larger files where they already organize the module, such as `contracts/core/streaming-billing/src/contract.rs`.

**Linting:**
- There is no repository-wide `clippy.toml` or CI clippy step. Treat warning-free local changes as the practical bar.
- Avoid introducing new production `unwrap()` calls or unchecked conversions. Test code still uses them freely, but contract code generally prefers typed errors.

## Import Organization

**Order:**
1. In split-operation modules under `src/contract/`, start with `use super::*;` so the leaf file consumes the shared prelude from the parent `contract.rs`. See `contracts/core/split-router/src/contract/execute.rs` and `contracts/nft/minter-v2/src/contract/execute.rs`.
2. Add targeted `crate::...` imports only for items not already exposed by that prelude, as in `contracts/nft/minter-v2/src/contract/helpers/tests.rs` and `contracts/nft/marketplace-v3/src/contract/query/tests.rs`.
3. In monolithic files, group framework imports before crate imports, matching `contracts/core/streaming-billing/src/contract.rs` and `contracts/relationship/follow/src/execute.rs`.

**Path Aliases:**
- No custom path aliases are used. Import local code with `crate::...` or `super::*`.
- Split contracts commonly centralize shared imports in the parent `contract.rs` via `pub(super) use ...`, as in `contracts/core/split-router/src/contract.rs` and `contracts/nft/minter-v2/src/contract.rs`.

## Error Handling

**Patterns:**
- Return `Result<Response, ContractError>` from execute and instantiate paths, and `StdResult<T>` from pure query and storage helpers.
- Define one crate-local `ContractError` enum with `thiserror::Error`. Most crates include a `Std(#[from] StdError)` variant and then add domain-specific errors for unauthorized access, invalid input, or missing state.
- Validate input early with `deps.api.addr_validate`, `must_pay`, `nonpayable`, `maybe_addr`, or `map(...).transpose()?` when the handler depends on validated addresses or optional fields.
- Use `ensure!` and `ensure_eq!` in Sylvia crates such as `contracts/staking/nft-vault/src/contract.rs`; use explicit `if` checks with `return Err(...)` in non-Sylvia crates such as `contracts/core/streaming-billing/src/contract.rs` and `contracts/nft/minter-v2/src/contract/execute.rs`.
- Map foreign errors into domain errors when the caller needs a contract-specific failure surface. `contracts/core/streaming-billing/src/contract.rs` uses errors such as `UserBalanceNotFound`, `WorldConfigNotFound`, and `InvalidConversionRate`.

## Logging

**Framework:** Response attributes and CosmWasm `Event`

**Patterns:**
- In non-Sylvia contracts, build responses with `Response::new().add_attribute(...)` and include an `action` attribute for stateful operations.
- In Sylvia contracts, prefer typed event wrappers that convert into `Event`, as in `contracts/staking/stake-rewards/src/events.rs` and `contracts/staking/vault-factory/src/events.rs`.
- Include actor identifiers and key business fields in emitted metadata, such as `sender`, `recipient`, `token_id`, `world_nft_id`, `points_awarded`, and `follow_timestamp`.
- No off-chain logger abstraction is present in contract code.

## Comments

**When to Comment:**
- Comment state-layout intent, migration constraints, and non-obvious business rules. Good examples appear in `contracts/core/registry/src/state.rs`, `contracts/nft/minter-v2/src/migration.rs`, and `contracts/nft/marketplace-v3/src/migration.rs`.
- Use short inline comments for boundaries or irreversible transitions when the code would otherwise be ambiguous.
- Keep comments sparse in routine CRUD-style handlers; type names and helper names should carry most of the meaning.

**JSDoc/TSDoc:**
- Use Rust doc comments (`///`) for public state and message types, and regular `//` comments for section banners or narrow implementation notes.

## Function Design

**Size:** Keep entrypoints thin and push real logic into `execute_*`, `query_*`, helper, reply, or migrate functions. That pattern is consistent across `contracts/core/split-router`, `contracts/core/ecosystem-factory`, `contracts/nft/minter-v2`, and `contracts/nft/marketplace-v3`.

**Parameters:** Pass CosmWasm context types directly (`DepsMut`, `Deps`, `Env`, `MessageInfo`) and unpack message fields at the dispatcher boundary. Sylvia crates use `InstantiateCtx`, `ExecCtx`, and `QueryCtx` in the same role.

**Return Values:** Return typed query DTOs and assemble a full `Response` for execute paths, including messages, submessages, and emitted attributes or events.

## Module Design

**Exports:** Keep `src/lib.rs` small: declare modules, re-export `ContractError`, and only attach `#[cfg(test)] mod tests;` or `mod multitest;` when the crate already uses that structure.

**Barrel Files:** There are no broad cross-crate barrel modules. The closest pattern is the contract-local prelude in files such as `contracts/core/split-router/src/contract.rs`, `contracts/core/multisig/src/contract.rs`, and `contracts/nft/minter-v2/src/contract.rs`.

**Editing Guidance:**
- When changing a split-operation crate, add new logic in the existing operation file instead of growing `src/contract.rs`.
- When changing a monolithic crate like `contracts/core/streaming-billing/src/contract.rs` or `contracts/relationship/follow/src/execute.rs`, match the local style instead of force-migrating the file in an unrelated change.
- Do not mix Sylvia patterns into non-Sylvia crates unless the crate already depends on Sylvia.

---
*Convention analysis: 2026-03-18*
