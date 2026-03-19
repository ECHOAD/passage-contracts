---
phase: 09-marketplace-v3-registration-ownership-validation-and-admin-approval-redesign
plan: 03
subsystem: public-surface-alignment
tags: [marketplace-v3, docs, schema, queries]
provides:
  - request-aware query surface
  - corrected README and integration docs
  - regenerated schema matching the redesign
affects: [marketplace-v3, docs]
key-files:
  modified:
    - contracts/nft/marketplace-v3/README.md
    - contracts/nft/marketplace-v3/schema/execute_msg.json
    - contracts/nft/marketplace-v3/schema/instantiate_msg.json
    - contracts/nft/marketplace-v3/schema/query_msg.json
    - contracts/nft/marketplace-v3/src/contract/query.rs
    - contracts/nft/marketplace-v3/src/contract/query/tests.rs
    - docs/01-end-to-end-setup.md
    - docs/02-method-reference.md
    - docs/03-json-examples.md
completed: 2026-03-19
---

# Phase 09 Plan 03 Summary

Plan 09-03 aligned the public contract surface with the new registration model so integrators no longer read stale semantics from docs or schema files.

Implementation commit:
- `209ca46` `feat(09): redesign marketplace-v3 registration model`

Delivered outcomes:
- added request inspection queries for registration and update queues
- made `CollectionFee` explicitly marketplace-global with `is_override = false`
- rewrote the marketplace README around mandatory registration, owner request flow, and admin direct flow
- updated end-to-end setup, method reference, and JSON examples to remove the old instantiate-time denom and optional-registration story
- regenerated checked-in schema outputs for the new wire surface

Verification recorded for this plan:
- `cargo run --example schema -p marketplace-v3`
- `cargo test -p marketplace-v3 --lib`
- `cargo check -p marketplace-v3`
