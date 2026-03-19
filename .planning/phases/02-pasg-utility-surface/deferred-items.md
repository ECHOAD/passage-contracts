# Deferred Items

## 2026-03-18 - Workspace verification blocker outside 02-01 scope

- `cargo unit-test` fails in `contracts/core/ecosystem-factory`.
- `cargo check --workspace` fails in `contracts/core/ecosystem-factory`.
- Both commands report the same missing symbol: `REQUESTS_BY_CREATOR` is not imported in:
  - `contracts/core/ecosystem-factory/src/contract/execute.rs:188`
  - `contracts/core/ecosystem-factory/src/contract/query.rs:84`
- Scope decision: left untouched during `02-01` because this plan owns PASG utility surface and docs, not `ecosystem-factory`.
- Impact: targeted PASG docs checks passed, but full workspace verification remains blocked until `ecosystem-factory` is repaired.

## 2026-03-18 - Additional workspace verification blocker observed during 02-02

- `cargo check --workspace` also fails in `contracts/nft/minter-metadata-onchain`.
- The failure is `missing field nft_type in initializer of pg721_metadata_onchain::msg::InstantiateMsg` at `contracts/nft/minter-metadata-onchain/src/contract.rs:102`.
- Scope decision: left untouched during `02-02` because this plan owns PASG utility surface integration, not legacy metadata-minter constructor drift.
- Impact: targeted PASG package tests passed, but workspace-wide verification now has two known blockers outside the phase-owned files.