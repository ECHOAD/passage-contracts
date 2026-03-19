---
phase: 04-pasg-staking-rewards
plan: 02
subsystem: staking-metadata-and-query-surface
tags: [pasg, staking, registry, governance, docs]
provides:
  - optional registry validator metadata surface
  - governance handoff for staking metadata only
  - chain-native reward and undelegation query examples
affects: [registry, pasg-governance, docs]
key-files:
  modified:
    - contracts/core/registry/src/msg.rs
    - contracts/core/registry/src/contract/execute.rs
    - contracts/core/registry/src/contract/query.rs
    - contracts/core/registry/src/tests/staking.rs
    - contracts/core/pasg-governance/src/msg.rs
    - contracts/core/pasg-governance/src/tests/admin_handoff.rs
    - docs/03-json-examples.md
    - docs/04-multisig-governance.md
completed: 2026-03-19
---

# Phase 04 Plan 02 Summary

Plan 04-02 added the minimum repo-local staking integration surface without creating a second staking ledger.

Delivered outcomes:
- added `registry` execute and query support for optional Passage validator metadata through `UpsertStakingValidator`, `RemoveStakingValidator`, `StakingValidator`, and `StakingValidators`
- kept the registry surface metadata-only: no delegated balances, unbond queues, reward-per-token accounting, or emission-rate state
- extended `pasg-governance` with typed admin actions for `RegistryUpsertStakingValidator` and `RegistryRemoveStakingValidator`, keeping staking governance limited to metadata and policy edges
- added `registry` and `pasg-governance` tests proving the staking metadata surface stays queryable and does not dispatch arbitrary staking custody logic
- aligned `docs/03-json-examples.md` and `docs/04-multisig-governance.md` with chain-native reward-state queries, undelegation reads, and the optional metadata flow

Verification recorded for this plan:
- `cargo check -p registry -p pasg-governance`
- `cargo test -p registry --lib`
- `cargo test -p pasg-governance --lib`

Notes:
- `cargo test -p pasg-governance --lib` passed with existing `mock_info` deprecation warnings in tests; no functional failures remained.
## Deviations from Plan

None - the plan stayed within the intended native-staking boundary and only added the optional validator metadata surface that the docs now reference.

## Auth Gates

None.

## Self-Check: PASSED

- Found `.planning/phases/04-pasg-staking-rewards/04-02-SUMMARY.md`
- Found commit `821230c`
- Found commit `4918ddc`
- Found commit `5b724be`


