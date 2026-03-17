# Testing Patterns

**Analysis Date:** 2026-03-17

## Test Framework

**Runner:**
- Rust's built-in test harness through Cargo.
- Workspace alias config: `.cargo/config.toml`
- CI entrypoint: `.github/workflows/main.yaml`
- Main contract testing libraries:
  - `cw-multi-test` in crate manifests such as `contracts/core/multisig/Cargo.toml`, `contracts/core/split-router/Cargo.toml`, `contracts/nft/minter/Cargo.toml`, and `contracts/nft/marketplace-v2/Cargo.toml`
  - `sylvia` multitest support in `contracts/staking/nft-vault/Cargo.toml`

**Assertion Library:**
- Standard Rust assertions: `assert_eq!`, `assert!`, and targeted `panic!` branches for message matching.

**Run Commands:**
```bash
cargo unit-test                              # Workspace library tests via alias in `.cargo/config.toml`
cargo test -p <crate> --lib                 # Focus one crate while iterating
cargo test -p <crate> --lib <test_name> -- --exact --nocapture
```

Current workspace state:
- `cargo unit-test` is wired into CI, but it does not currently complete because `contracts/core/streaming-billing/src/contract.rs` fails to compile. The failing areas include missing `serde_json`, incorrect `Bound` usage, arithmetic error conversion, and response message typing.

## Test File Organization

**Location:**
- Mix co-located and crate-level organization; follow the existing crate style instead of normalizing it.
- Co-located unit-style modules live beside the code under test: `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/core/split-router/src/contract/query/tests.rs`, `contracts/nft/minter-v2/src/contract/helpers/tests.rs`, `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`.
- Crate-level suites live under dedicated `src/tests/`: `contracts/core/registry/src/tests/mod.rs`, `contracts/core/registry/src/tests/collections.rs`, `contracts/core/registry/src/tests/recovery.rs`, `contracts/core/multisig/src/tests/governance.rs`.
- Large scenario suites live in standalone crate files: `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, `contracts/nft/pg721-legacy/src/tests.rs`.
- Multi-contract integration suites often use `multitest.rs`: `contracts/relationship/follow/src/multitest.rs`, `contracts/relationship/friend/src/multitest.rs`, `contracts/nft/marketplace-v2/src/multitest.rs`.

**Naming:**
- Common filenames are `tests.rs`, `test.rs`, `multitest.rs`, `contract_tests.rs`, and `src/tests/mod.rs`.
- Test names are a mix of behavior-driven snake_case (`collections_by_nft_type_returns_only_matching_collections`) and legacy names (`proper_initialization`, `test_claim_tokens_with_all_released_claims`).

**Structure:**
```text
contracts/
├── core/registry/src/tests/                 # crate-level focused suites
├── core/split-router/src/contract/*/tests.rs # handler-adjacent unit tests
├── nft/minter/src/contract_tests.rs         # large end-to-end scenario suite
├── nft/marketplace-v2/src/multitest.rs      # cw-multi-test scenario suite
└── staking/nft-vault/src/claim/test.rs      # pure module tests
```

Detected suite hotspots:
- `contracts/nft/minter-metadata-onchain/src/contract_tests.rs` contains 18 `#[test]` cases.
- `contracts/nft/minter/src/contract_tests.rs` contains 14 `#[test]` cases.
- `contracts/staking/nft-vault/src/claim/test.rs` contains 13 `#[test]` cases.
- `contracts/nft/whitelist/src/contract/tests.rs` contains 10 `#[test]` cases.
- `contracts/nft/royalty-group/src/contract/tests.rs` contains 9 `#[test]` cases.

## Test Structure

**Suite Organization:**
```rust
fn save_base_state(deps: DepsMut) {
    CONFIG.save(deps.storage, &Config { ... }).unwrap();
    SPLIT_CONFIG.save(deps.storage, &SplitConfig { ... }).unwrap();
}

#[test]
fn update_split_requires_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    save_base_state(deps.as_mut());

    let err = execute_update_split(...).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});
}
```

This pattern appears directly in `contracts/core/split-router/src/contract/execute/tests.rs`.

**Patterns:**
- Create one narrow setup helper per suite, usually `setup_contract`, `instantiate_default`, `save_base_state`, `sample_config`, or `mock_queries`.
- Exercise the lowest practical boundary. Unit suites often call helper functions or handler functions directly instead of routing through the outer dispatcher.
- Assert on both response shape and storage side effects. Typical checks validate `Response` attributes, message counts, saved state, and query output in the same test.
- Mutate `mock_env()` when block time or height matters, as in `contracts/core/multisig/src/tests/governance.rs`, `contracts/core/registry/src/tests/recovery.rs`, and `contracts/staking/nft-vault/src/claim/test.rs`.

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
- Use `cw_multi_test::ContractWrapper` and `AppBuilder` when the scenario depends on real message dispatch between multiple contracts. See `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, and `contracts/nft/marketplace-v2/src/multitest.rs`.
- Use `sylvia::multitest::App` only in Sylvia crates that already expose that pattern, such as `contracts/staking/nft-vault/src/contract/tests.rs`.

**What NOT to Mock:**
- Do not mock internal storage reads or writes. The common pattern is to let real storage mutate and assert on the saved values directly.
- Do not introduce a new test harness for a crate that already has an established one. Reuse `mock_dependencies`, `cw-multi-test`, or Sylvia multitest according to the package's existing style.
- Do not stub response attributes when the contract already emits them; assert on the returned `Response` instead.

## Fixtures and Factories

**Test Data:**
```rust
struct TestAddrs {
    deployer: Addr,
    alice: Addr,
    bob: Addr,
    carol: Addr,
}

fn instantiate_default(deps: DepsMut) -> TestAddrs {
    instantiate(deps, mock_env(), message_info(&addrs.deployer, &[]), InstantiateMsg { ... }).unwrap();
    addrs
}
```

This pattern is used in `contracts/core/multisig/src/tests/governance.rs`.

Additional fixture styles:
- Literal config factories like `sample_config()` in `contracts/nft/minter-v2/src/contract/helpers/tests.rs`
- Setup helpers like `setup_contract(...)` in `contracts/nft/whitelist/src/contract/tests.rs` and `contracts/nft/pg721/src/contract/tests.rs`
- Reusable NFT generators like `get_test_nfts(...)` in `contracts/staking/nft-vault/src/claim/test.rs`

**Location:**
- Fixtures are local to each test file or test module. No shared workspace-level fixture crate or helper module is present.

## Coverage

**Requirements:** None enforced. The only detected CI quality gate is `cargo unit-test` in `.github/workflows/main.yaml`.

**View Coverage:**
```bash
# Not configured in-repo
```

Practical guidance:
- Prefer per-crate test runs while working in a single package, especially because the workspace-wide alias is currently blocked by `contracts/core/streaming-billing/src/contract.rs`.
- Keep new work inside already-tested crates unless the change explicitly includes adding missing tests for an untested package.

## Test Types

**Unit Tests:**
- Most common pattern. Use `mock_dependencies`, `mock_env`, `mock_info`, or `message_info` to test storage mutations and response construction in-process.
- Representative files: `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/core/registry/src/tests/collections.rs`, `contracts/nft/royalty-group/src/contract/tests.rs`, `contracts/staking/nft-vault/src/claim/test.rs`.

**Integration Tests:**
- Multi-contract and stateful flows use `cw-multi-test` or Sylvia multitest.
- Representative files: `contracts/nft/minter/src/contract_tests.rs`, `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`, `contracts/nft/marketplace-v2/src/multitest.rs`, `contracts/relationship/follow/src/multitest.rs`, `contracts/staking/nft-vault/src/contract/tests.rs`.

**E2E Tests:**
- Not detected. No external chain, RPC, or deployment-level test harness is present in the repo.

Untested or effectively untested packages:
- No test files were detected in `contracts/core/collection-factory`, `contracts/core/ecosystem-factory`, `contracts/core/streaming-billing`, `contracts/nft/marketplace-legacy`, `contracts/nft/minter-v2-metadata-onchain`, `contracts/staking/stake-rewards`, and `contracts/staking/vault-factory`.

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

Use direct `ContractError` equality when the error enum derives `PartialEq`, as in `contracts/core/multisig/src/tests/governance.rs`, `contracts/core/split-router/src/contract/execute/tests.rs`, and `contracts/nft/minter-v2/src/contract/helpers/tests.rs`.

String-based comparisons exist where equality is less convenient:
```rust
assert_eq!(err.to_string(), ContractError::Unauthorized {}.to_string());
```

That pattern appears in `contracts/nft/pg721-updatable/src/contract/tests.rs`.

Message assertion pattern:
```rust
match &res.messages[0].msg {
    CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => { ... }
    _ => panic!("expected bank send"),
}
```

Use this for payout and routing checks in `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/nft/auction-english/src/execute/tests.rs`, and `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`.

---

*Testing analysis: 2026-03-17*
