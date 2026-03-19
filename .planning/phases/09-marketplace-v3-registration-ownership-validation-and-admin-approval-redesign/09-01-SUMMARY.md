---
phase: 09-marketplace-v3-registration-ownership-validation-and-admin-approval-redesign
plan: 01
subsystem: marketplace-config-simplification
tags: [marketplace-v3, registration, fees, denom]
provides:
  - marketplace-global trading fee model
  - collection-scoped settlement denom model
  - mandatory registration before marketplace interaction
affects: [marketplace-v3]
key-files:
  modified:
    - contracts/nft/marketplace-v3/src/msg.rs
    - contracts/nft/marketplace-v3/src/state.rs
    - contracts/nft/marketplace-v3/src/contract/instantiate.rs
    - contracts/nft/marketplace-v3/src/contract/execute.rs
    - contracts/nft/marketplace-v3/src/contract/helpers.rs
    - contracts/nft/marketplace-v3/src/contract/query.rs
    - contracts/nft/marketplace-v3/src/migration.rs
completed: 2026-03-19
---

# Phase 09 Plan 01 Summary

Plan 09-01 removed the old mixed model where marketplace-wide defaults and per-collection overrides competed for control.

Implementation commit:
- `209ca46` `feat(09): redesign marketplace-v3 registration model`

Delivered outcomes:
- removed instantiate-time `denom`, `max_trading_fee_bps`, and `require_registration`
- kept `trading_fee_bps` as a marketplace-global value
- made settlement denom mandatory at collection registration time
- removed per-collection fee override behavior from config, query, and settlement helpers
- made trading fail for collections that are not explicitly registered

Verification recorded for this plan:
- `cargo test -p marketplace-v3 --lib`
- `cargo check -p marketplace-v3`
