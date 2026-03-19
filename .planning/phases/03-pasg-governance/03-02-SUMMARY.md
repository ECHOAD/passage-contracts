---
phase: 03-pasg-governance
plan: 02
subsystem: token-weighted-voting
tags: [pasg, governance, delegation, quorum, voting]
provides:
  - delegated deposited-power voting
  - quorum and pass-threshold enforcement from snapshot state
  - proposal lock rules preventing withdrawal of active voting balances
affects: [pasg-governance]
key-files:
  modified:
    - contracts/core/pasg-governance/src/contract.rs
    - contracts/core/pasg-governance/src/error.rs
    - contracts/core/pasg-governance/src/msg.rs
    - contracts/core/pasg-governance/src/state.rs
    - contracts/core/pasg-governance/src/tests/mod.rs
    - contracts/core/pasg-governance/src/tests/voting.rs
requirements-completed: [GOV-02]
completed: 2026-03-19
---

# Phase 03 Plan 02 Summary

Plan 03-02 moved PASG governance from a basic lifecycle to contract-enforced weighted voting.

Implementation commit:
- `18bd6d1` `feat(03): complete PASG governance split architecture`

Delivered outcomes:
- `Delegate` and `Undelegate` support over deposited native `upasg`
- `ProposalSnapshot`, `LockedBalance`, and delegated-power accounting in contract state
- `resolve_effective_power`, `snapshot_total_power`, quorum checks, and pass-threshold checks inside `contract.rs`
- withdrawal guards so balances participating in open proposals cannot be withdrawn early
- deterministic tests for delegation, cycle rejection, quorum failure, pass-threshold failure, and lock enforcement

Verification recorded for this plan:
- `cargo check -p pasg-governance`
- `cargo test -p pasg-governance --lib`
