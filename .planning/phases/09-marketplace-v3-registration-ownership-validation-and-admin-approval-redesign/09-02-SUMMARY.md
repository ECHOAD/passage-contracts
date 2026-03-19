---
phase: 09-marketplace-v3-registration-ownership-validation-and-admin-approval-redesign
plan: 02
subsystem: owner-request-admin-approval
tags: [marketplace-v3, ownership, approvals, requests]
provides:
  - owner-only registration requests
  - owner-only update requests
  - admin direct register and update bypass
affects: [marketplace-v3]
key-files:
  modified:
    - contracts/nft/marketplace-v3/src/error.rs
    - contracts/nft/marketplace-v3/src/msg.rs
    - contracts/nft/marketplace-v3/src/state.rs
    - contracts/nft/marketplace-v3/src/contract/execute.rs
    - contracts/nft/marketplace-v3/src/contract/helpers.rs
    - contracts/nft/marketplace-v3/src/contract/execute/tests.rs
completed: 2026-03-19
---

# Phase 09 Plan 02 Summary

Plan 09-02 added the request-and-approval flow that the redesign needed instead of letting collection management remain an operator-only admin shortcut.

Implementation commit:
- `209ca46` `feat(09): redesign marketplace-v3 registration model`

Delivered outcomes:
- `SubmitCollectionRegistrationRequest` now requires the actual collection creator
- `ResolveCollectionRegistrationRequest` lets admin approve or reject and optionally override denom on approval
- `SubmitCollectionUpdateRequest` now requires the collection creator and at least one real change
- `ResolveCollectionUpdateRequest` lets admin approve or reject queued updates
- admin still keeps direct `RegisterCollection` and `UpdateCollectionConfig` paths without a request round-trip

Verification recorded for this plan:
- `cargo test -p marketplace-v3 --lib`
- `cargo check -p marketplace-v3`
