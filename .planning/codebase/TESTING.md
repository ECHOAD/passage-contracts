# Testing Patterns

**Analysis Date:** 2026-03-18

## Test Framework

**Runner:**
- Rust's built-in test harness through Cargo.
- Workspace alias: `.cargo/config.toml` defines `cargo unit-test` as `cargo test --lib`.
- CI entrypoint: `.github/workflows/main.yaml` runs `cargo unit-test` after building wasm.
- Main contract testing libraries:
  - `cw-multi-test` in crates such as `contracts/core/multisig`, `contracts/core/split-router`, `contracts/nft/minter`, and `contracts/nft/marketplace-v2`
  - `sylvia::multitest` in `contracts/staking/nft-vault`

**Assertion Library:**
- Standard Rust assertions: `assert_eq!`, `assert!`, and targeted `panic!` branches for message matching.

**Run Commands:**
```bash
cargo unit-test
cargo test -p <crate> --lib
cargo test -p <crate> --lib <test_name> -- --exact --nocapture
```

## Test File Organization

**Location:**
- The workspace mixes co-located tests and crate-level suites; follow the crate's existing style rather than normalizing it.
- Co-located unit tests live beside the code under test: `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/core/split-router/src/contract/query/tests.rs`, `contracts/nft/minter-v2/src/contract/helpers/tests.rs`, `contracts/nft/marketplace-v3/src/contract/query/tests.rs`, and `contracts/staking/nft-vault/src/contract/tests.rs`.
- Crate-level suites live under `src/tests/` or in standalone files like `src/tests.rs`: `contracts/core/multisig/src/tests/governance.rs`, `contracts/core/streaming-billing/src/tests.rs`, and `contracts/nft/pg721-legacy/src/tests.rs`.
- Larger scenario suites use `contract_tests.rs` or `multitest.rs`, such as `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, `contracts/relationship/follow/src/multitest.rs`, and `contracts/nft/marketplace-v2/src/multitest.rs`.
- Pure helper-style tests also appear in nested files like `contracts/staking/nft-vault/src/claim/test.rs`.

**Naming:**
- Common filenames are `tests.rs`, `test.rs`, `multitest.rs`, `contract_tests.rs`, and `src/tests/*.rs`.
- Test names are usually behavior-driven snake_case in newer suites, but older modules still use names like `proper_initialization`, `test_claim_tokens_with_all_released_claims`, and `test_instantiate_follow_contract`.

**Structure:**
```text
contracts/
├── core/split-router/src/contract/*/tests.rs
├── core/multisig/src/tests/governance.rs
├── nft/minter/src/contract_tests.rs
├── nft/marketplace-v2/src/multitest.rs
└── staking/nft-vault/src/claim/test.rs
```

## Test Structure

**Suite Organization:**
```rust
fn save_base_state(deps: DepsMut) {
    CONFIG.save(deps.storage, &Config { ... }).unwrap();
}

#[test]
fn update_split_requires_admin() {
    let mut deps = mock_dependencies();
    save_base_state(deps.as_mut());

    let err = execute_update_split(...).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}
```

That pattern appears in `contracts/core/split-router/src/contract/execute/tests.rs` and similar suites.

**Patterns:**
- Create one narrow setup helper per suite, usually `setup_contract`, `instantiate_default`, `save_base_state`, `sample_config`, or `mock_queries`.
- Exercise the lowest practical boundary. Many unit tests call helper or handler functions directly instead of routing through the outer dispatcher.
- Assert on both response shape and storage side effects. Typical checks validate `Response` attributes, message counts, saved state, and query output in the same test.
- Mutate `mock_env()` when block time or height matters, as in `contracts/core/multisig/src/tests/governance.rs`, `contracts/core/ecosystem-factory/src/contract/execute/tests.rs`, and `contracts/staking/nft-vault/src/claim/test.rs`.

## Mocking

**Framework:** `cosmwasm_std::testing`, `cw-multi-test`, and `sylvia::multitest`

**Patterns:**
```rust
deps.querier.update_wasm(move |query| match query {
    WasmQuery::Smart { contract_addr, msg } if contract_addr == whitelist_addr => {
        let parsed: WhitelistQueryMsg = from_json(msg).unwrap();
        SystemResult::Ok(ContractResult::Ok(to_json_binary(&response).unwrap()))
    }
    _ => SystemResult::Err(SystemError::UnsupportedRequest { kind: ... }),
});
```

That pattern is used in `contracts/nft/minter-v2/src/contract/helpers/tests.rs` and `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`.

**What to Mock:**
- Mock cross-contract smart queries with `deps.querier.update_wasm(...)` when the code under test depends on external contracts such as whitelist, registry, pg721, or split-router.
- Mock block time and height explicitly when expiry, voting windows, or release conditions are part of the behavior.
- Use `cw_multi_test::ContractWrapper` and `AppBuilder` when the scenario depends on real message dispatch between multiple contracts, as in `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, and `contracts/nft/marketplace-v2/src/multitest.rs`.
- Use `sylvia::multitest::App` only in Sylvia crates that already expose that pattern, such as `contracts/staking/nft-vault/src/contract/tests.rs`.

**What NOT to Mock:**
- Do not mock internal storage reads or writes. Let real storage mutate and assert on the saved values directly.
- Do not introduce a new test harness for a crate that already has an established one.
- Do not stub response attributes when the contract already emits them; assert on the returned `Response` instead.

## Fixtures and Factories

**Test Data:**
```rust
struct TestAddrs {
    deployer: Addr,
    alice: Addr,
    bob: Addr,
}

fn instantiate_default(deps: DepsMut) -> TestAddrs {
    instantiate(...).unwrap();
    addrs
}
```

This pattern is used in `contracts/core/multisig/src/tests/governance.rs`.

Additional fixture styles:
- Literal config factories like `sample_config()` in `contracts/nft/minter-v2/src/contract/helpers/tests.rs`
- Setup helpers like `setup_contract(...)` in `contracts/nft/pg721-updatable/src/contract/tests.rs`
- Reusable state builders like `save_base_state(...)` in `contracts/core/split-router/src/contract/execute/tests.rs`

**Location:**
- Fixtures are local to each test file or test module. No shared workspace-level fixture crate is present.

## Coverage

**Requirements:** None enforced. The detected CI quality gate is `cargo unit-test` in `.github/workflows/main.yaml`.

**View Coverage:**
```bash
# Not configured in-repo
```

Practical guidance:
- Prefer per-crate test runs while working in a single package.
- Keep new work inside already-tested crates unless the change explicitly includes adding missing tests for an untested package.

## Test Types

**Unit Tests:**
- Most common pattern. Use `mock_dependencies`, `mock_env`, `mock_info`, or `message_info` to test storage mutations and response construction in-process.
- Representative files: `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/core/multisig/src/tests/governance.rs`, `contracts/nft/royalty-group/src/contract/tests.rs`, `contracts/staking/nft-vault/src/claim/test.rs`.

**Integration Tests:**
- Multi-contract and stateful flows use `cw-multi-test` or Sylvia multitest.
- Representative files: `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, `contracts/nft/marketplace-v2/src/multitest.rs`, `contracts/relationship/follow/src/multitest.rs`, `contracts/staking/nft-vault/src/contract/tests.rs`.

**E2E Tests:**
- Not detected. No external chain, RPC, or deployment-level test harness is present in the repo.

Detected test gaps in the current scan:
- No `*test*.rs` files were found in `contracts/nft/marketplace-legacy`, `contracts/nft/minter-v2-metadata-onchain`, `contracts/staking/stake-rewards`, or `contracts/staking/vault-factory`.

## Common Patterns

**Async Testing:**
```rust
// Not used
```

The workspace test suites are synchronous Rust tests. No `tokio`, `async-std`, or async contract test harness appears in contract code.

**Error Testing:**
```rust
let err = execute(...).unwrap_err();
assert_eq!(err, ContractError::Unauthorized {});
```

Use direct `ContractError` equality when the enum derives `PartialEq`. String-based comparisons appear where equality is less convenient, such as in `contracts/nft/pg721-updatable/src/contract/tests.rs`.

**Message Assertions:**
```rust
match &res.messages[0].msg {
    CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => { ... }
    _ => panic!("expected bank send"),
}
```

Use this for payout and routing checks in files like `contracts/core/split-router/src/contract/execute/tests.rs` and `contracts/nft/marketplace-v3/src/contract/query/tests.rs`.

---
*Testing analysis: 2026-03-18*
