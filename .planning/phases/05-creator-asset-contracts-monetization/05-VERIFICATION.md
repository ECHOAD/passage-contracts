# 05 Verification

## Final Stance

- `registry` is the canonical ledger for ecosystems and collection affiliation.
- Collections remain independent on-chain contracts identified by contract address.
- Ecosystem affiliation is mutable: a collection can be deregistered and later re-homed.
- Typed creator asset semantics are enforced in the shared `pg721` family.
- Monetization-bearing metadata stays on-chain where enforceable; runtime, rendering, and Unreal-specific behavior stay off-chain.

## Requirement Traceability

### NFT-01
Registry and ecosystem lifecycle now match the intended creator asset model.

Evidence:
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/state.rs`
- `contracts/core/registry/src/contract/execute.rs`
- `contracts/core/registry/src/contract/query.rs`
- `contracts/core/registry/src/tests/collections.rs`
- `.planning/phases/05-creator-asset-contracts-monetization/05-01-SUMMARY.md`

Coverage:
- direct collection registration by ecosystem admin or approved member
- `deregister` support
- `re-home` support
- unaffiliated collection query support

### NFT-02
Shared collection contracts now expose a complete typed asset story.

Evidence:
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721/src/contract/tests.rs`
- `contracts/nft/pg721-updatable/src/contract/tests.rs`
- `.planning/phases/05-creator-asset-contracts-monetization/05-02-SUMMARY.md`

Coverage:
- `component`
- `avatar`
- `companion`
- `world`
- `plugin`
- `achievement`
- `world_template`

### NFT-03
Integrator-facing docs and examples now describe the same ecosystem-centric collection model that the code enforces.

Evidence:
- `README.md`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`
- `contracts/core/registry/README.md`
- `contracts/core/collection-factory/README.md`
- `contracts/core/ecosystem-factory/README.md`
- `contracts/nft/pg721/README.md`
- `contracts/nft/pg721-updatable/README.md`

Coverage:
- collection deregister and re-home
- creator provenance preservation
- typed asset payload examples
- explicit off-chain boundary statements

### REV-01
Creator monetization remains explicit without adding NFT-type-specific routing contracts.

Evidence:
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/marketplace-v3/src/contract/helpers.rs`
- `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`
- `contracts/nft/auction-english/src/helpers.rs`
- `contracts/nft/auction-english/src/execute/tests.rs`

Coverage:
- collection `royalty_info`
- world `revenue_shares`
- generic `split-router` settlement
- no NFT-type-specific router branches

## Commands Run

- `cargo check -p registry -p collection-factory -p ecosystem-factory`
- `cargo test -p registry --lib`
- `cargo check -p pg721 -p pg721-updatable`
- `cargo test -p pg721 --lib`
- `cargo test -p pg721-updatable --lib`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p auction-english --lib`
- `cargo run --example schema -p pg721`
- `cargo run --example schema -p pg721-updatable`
- `cargo check --workspace`
- `cargo unit-test`

## Residual Boundaries

- Validator staking, delegations, undelegations, and rewards remain chain-native concerns.
- Runtime, rendering, and Unreal-specific asset behavior remain off-chain.
- Marketplace registration request flow still exists in `marketplace-v3` by separate phase scope; Phase 5 did not redesign marketplace onboarding.
