# Coding Conventions

**Analysis Date:** 2026-03-17

## Naming Patterns

**Files:**
- Keep the standard contract surface in `src/lib.rs`, `src/error.rs`, `src/msg.rs`, `src/state.rs`, and either `src/contract.rs` or `src/contract/*.rs`. Follow the local crate layout instead of inventing a new one.
- In split-operation crates, put handlers in responsibility-named files such as `contracts/core/registry/src/contract/instantiate.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/core/registry/src/contract/query.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, and `contracts/nft/marketplace-v3/src/contract/migrate.rs`.
- Keep test files adjacent to the code they exercise when the crate already does that: `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/nft/minter-v2/src/contract/helpers/tests.rs`, `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`.
- Use `multitest.rs` for scenario suites in crates that already use `cw-multi-test`: `contracts/relationship/follow/src/multitest.rs`, `contracts/relationship/friend/src/multitest.rs`, `contracts/nft/marketplace-v2/src/multitest.rs`.
- Use `contract_tests.rs` for large end-to-end style crate-local suites where that pattern already exists: `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`.

**Functions:**
- Use `snake_case` for all functions and keep public entrypoints named `instantiate`, `execute`, `query`, or `migrate` when they expose CosmWasm handlers. See `contracts/core/registry/src/contract.rs`, `contracts/nft/minter-v2/src/contract/execute.rs`, and `contracts/core/streaming-billing/src/contract.rs`.
- Prefix handler helpers with the operation they serve: `execute_update_config`, `execute_mint`, `query_world_stats`, `instantiate_default`, `setup_contract`, `mock_queries`.
- Name tests as executable behavior statements when touching the modular suites, for example `collections_by_nft_type_returns_only_matching_collections` in `contracts/core/registry/src/tests/collections.rs` and `split_sends_remainder_to_last_recipient_and_records_event` in `contracts/core/split-router/src/contract/execute/tests.rs`.
- Preserve the crate-local naming style in suites that still use `proper_initialization`, `test_claim_tokens_with_all_released_claims`, or `test_instantiate_follow_contract` in `contracts/nft/pg721/src/contract/tests.rs`, `contracts/staking/nft-vault/src/claim/test.rs`, and `contracts/relationship/follow/src/multitest.rs`.

**Variables:**
- Use `snake_case` for locals, function parameters, and storage-derived values: `world_nft_id`, `user_balance`, `reward_accounts`, `collection_deltas`.
- Use `SCREAMING_SNAKE_CASE` for constants and storage singletons: `CONTRACT_NAME`, `CONTRACT_VERSION`, `DEFAULT_LIMIT`, `MAX_LIMIT`, `CONFIG`, `RECOVERY_CASES`, `MINT_STATS`.
- Use short, domain-specific temporary names only inside tight scopes, usually `res`, `err`, `msg`, `ctx`, or `env`. Keep broader helpers explicit, as in `collection_nft_type` in `contracts/nft/minter-v2/src/contract/execute.rs`.

**Types:**
- Use `UpperCamelCase` for structs and enums: `ContractError`, `InstantiateMsg`, `QueryMsg`, `MintStats`, `RecoveryCase`, `ConfigEvent`.
- Suffix wire-format types with `Msg` or `Response`: `ExecuteMsg`, `WhitelistQueryMsg`, `ConfigResponse`, `MintableNumTokensResponse`.
- Keep storage models short and domain-named: `Config`, `Collection`, `AuthorizedMinter`, `UserReward`, `PendingRevenue`.
- Serialize API enums in `snake_case`. This is done explicitly with `#[serde(rename_all = "snake_case")]` in `contracts/core/registry/src/state.rs` and via `#[cw_serde]` message definitions in `contracts/nft/minter-v2/src/msg.rs`.

## Code Style

**Formatting:**
- Use `rustfmt`; the toolchain in `rust-toolchain.toml` includes `rustfmt`, and no root `rustfmt.toml` overrides are present.
- Keep long imports and struct literals vertically aligned with trailing commas, matching `contracts/core/registry/src/contract.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/staking/nft-vault/src/contract.rs`.
- Split long chained responses and storage writes across lines instead of compressing them into single lines. The common pattern is `Response::new()` followed by one chained call per line.
- Preserve section-banner comments in large files where they already organize the file, such as `contracts/core/streaming-billing/src/contract.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/core/registry/src/contract/execute.rs`.

**Linting:**
- `clippy` is installed through `rust-toolchain.toml`, but no root `clippy.toml` and no CI clippy step are present. Treat warning-free local changes as the expected bar even though the repo does not enforce it automatically.
- Do not add new unchecked conversions or new production `unwrap()` calls. Most crates return typed errors; the remaining production `unwrap()` usage is localized outlier behavior in files such as `contracts/staking/nft-vault/src/contract.rs` and `contracts/relationship/follow/src/execute.rs`.

## Import Organization

**Order:**
1. In split-operation modules under `src/contract/`, start with `use super::*;` so the file consumes the shared prelude from the parent `contract.rs`. See `contracts/core/registry/src/contract/execute.rs`, `contracts/core/split-router/src/contract/query.rs`, and `contracts/nft/minter-v2/src/contract/execute.rs`.
2. Add targeted `crate::...` imports only for types not already exposed by that shared prelude, such as `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs` and `contracts/nft/minter-v2/src/contract/helpers/tests.rs`.
3. In monolithic files without a shared prelude, group framework imports first and crate imports after them, matching `contracts/core/streaming-billing/src/contract.rs` and `contracts/relationship/follow/src/execute.rs`.

**Path Aliases:**
- No custom path aliases are used. Import local code with `crate::...` or `super::*`.
- For split contracts, centralize commonly reused imports in the parent `contract.rs` and re-export them as `pub(super) use ...`, as done in `contracts/core/registry/src/contract.rs` and `contracts/core/split-router/src/contract.rs`.

## Error Handling

**Patterns:**
- Return `Result<Response, ContractError>` from execute and instantiate paths, and `StdResult<T>` from pure query and storage helpers. See `contracts/core/registry/src/contract/execute.rs`, `contracts/nft/minter-v2/src/contract/execute.rs`, and `contracts/core/streaming-billing/src/contract.rs`.
- Define one crate-local `ContractError` enum with `thiserror::Error`. The minimum common variant is `Std(#[from] StdError)`, as in `contracts/core/registry/src/error.rs`, `contracts/core/multisig/src/error.rs`, and `contracts/core/streaming-billing/src/error.rs`.
- In Sylvia-based staking crates, also convert framework and arithmetic errors directly in `ContractError`. See `contracts/staking/stake-rewards/src/error.rs`.
- Validate user input early with `deps.api.addr_validate`, `must_pay`, `nonpayable`, `maybe_addr`, and `map(...).transpose()?`, as shown in `contracts/core/streaming-billing/src/contract.rs` and `contracts/staking/nft-vault/src/contract.rs`.
- Use `ensure!` and `ensure_eq!` in Sylvia contracts such as `contracts/staking/stake-rewards/src/contract.rs` and `contracts/staking/nft-vault/src/contract.rs`. Use explicit `if` checks with `return Err(...)` in non-Sylvia crates such as `contracts/nft/minter-v2/src/contract/execute.rs` and `contracts/core/streaming-billing/src/contract.rs`.
- Map foreign errors into domain errors when the caller needs a contract-specific failure surface. Examples: `UserBalanceNotFound`, `WorldConfigNotFound`, `InvalidConversionRate` in `contracts/core/streaming-billing/src/contract.rs`.
- Use `StdError::generic_err(...)` only in migration or compatibility code where the failure is not part of the public execute surface, as in `contracts/nft/minter-v2/src/migration.rs` and `contracts/nft/minter-v2-metadata-onchain/src/migration.rs`.

## Logging

**Framework:** Response attributes and CosmWasm `Event`

**Patterns:**
- In non-Sylvia contracts, build responses with `Response::new().add_attribute(...)` and include an `action` attribute whenever the contract performs a stateful action. See `contracts/nft/whitelist/src/contract.rs`, `contracts/nft/minter-v2/src/contract/execute.rs`, and `contracts/core/streaming-billing/src/contract.rs`.
- In Sylvia contracts, prefer typed event wrappers that implement `From<...> for Event`, as in `contracts/staking/stake-rewards/src/events.rs` and `contracts/staking/vault-factory/src/events.rs`.
- Include actor identifiers and key business fields in emitted metadata. Examples include `sender`, `recipient`, `token_id`, `world_nft_id`, `points_awarded`, and `follow_timestamp`.
- No `tracing`, `log`, or off-chain logger abstraction is present in contract code.

## Comments

**When to Comment:**
- Add comments for state-layout intent, migration constraints, and non-obvious business rules. Good examples are in `contracts/core/registry/src/state.rs`, `contracts/nft/marketplace-v3/src/migration.rs`, and `contracts/nft/minter-v2/src/migration.rs`.
- Use short inline comments for setup boundaries or irreversible state transitions only when the code would otherwise be ambiguous. Examples appear in `contracts/nft/minter-v2/src/contract/execute.rs` and `contracts/core/streaming-billing/src/contract.rs`.
- Keep comments sparse in straightforward CRUD-style handlers. Most behavior is communicated through type names and helper names rather than prose.

**JSDoc/TSDoc:**
- Not applicable. Use Rust doc comments (`///`) for public state and message types and regular `//` comments for section banners or narrow implementation notes.

## Function Design

**Size:** Keep entrypoints thin and push real logic into `execute_*`, `query_*`, helper, or migration functions. This is the dominant pattern in `contracts/core/registry`, `contracts/core/split-router`, `contracts/nft/minter-v2`, and `contracts/nft/marketplace-v3`.

**Parameters:** Pass CosmWasm context types directly (`DepsMut`, `Deps`, `Env`, `MessageInfo`) and unpack message fields at the dispatcher boundary. Sylvia crates use `InstantiateCtx`, `ExecCtx`, and `QueryCtx` in the same role.

**Return Values:** Return typed query DTOs and convert storage models into response models with `From` impls where useful, as in `contracts/nft/minter-v2/src/msg.rs`. For execute paths, return a fully assembled `Response` including messages, submessages, and emitted attributes or events.

## Module Design

**Exports:** Keep `src/lib.rs` small: declare modules, re-export `ContractError`, and attach `#[cfg(test)] mod tests;` or `mod multitest;` only when the crate already uses that structure. See `contracts/core/registry/src/lib.rs`, `contracts/relationship/follow/src/lib.rs`, and `contracts/nft/minter-v2/src/lib.rs`.

**Barrel Files:** There are no broad cross-crate barrel modules. The closest pattern is the contract-local prelude in files such as `contracts/core/registry/src/contract.rs` and `contracts/core/split-router/src/contract.rs`, which expose shared imports, constants, and top-level handler re-exports.

**Editing Guidance:**
- When changing a split-operation crate, add new logic in the existing operation file instead of growing `src/contract.rs`.
- When changing a monolithic crate like `contracts/core/streaming-billing/src/contract.rs` or `contracts/relationship/follow/src/execute.rs`, match the local style instead of force-migrating the file in an unrelated change.
- Do not mix Sylvia patterns into non-Sylvia crates unless the crate already depends on Sylvia. The workspace uses both styles and the package boundary is the correct seam.

---

*Convention analysis: 2026-03-17*
