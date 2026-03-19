---
phase: 03-pasg-governance
plan: 01
subsystem: governance-core
tags: [pasg, governance, multisig, upasg, proposal-lifecycle, scoped-execution]
requires:
  - phase: 02-pasg-utility-surface
    provides: native `upasg` PASG stance and protocol/off-chain scope boundary
provides:
  - PASG-holder governance lifecycle in `multisig`
  - deposited `upasg` voting-power accounting
  - scoped governance action surface for protocol contracts
affects: [multisig, docs/04-multisig-governance.md]
tech-stack:
  added: []
  patterns: [native governance deposits, protocol-only action allowlist, queryable proposal status]
key-files:
  modified:
    - contracts/core/multisig/src/msg.rs
    - contracts/core/multisig/src/contract.rs
    - contracts/core/multisig/src/state.rs
    - contracts/core/multisig/src/error.rs
    - contracts/core/multisig/src/tests/governance.rs
key-decisions:
  - "multisig evolves from fixed signers to PASG-holder governance backed by deposited native upasg"
  - "proposal payloads are bounded to protocol actions and allowlisted wasm execute targets"
patterns-established:
  - "Deposited governance power: proposal eligibility and voting come from contract-controlled balances"
  - "Scope in code: off-chain platform operations are blocked by the governance action surface"
requirements-completed: [GOV-01, GOV-03]
completed: 2026-03-18
---

# Phase 03 Plan 01 Summary

Implemented the governance-core rewrite in `contracts/core/multisig`: deposits, withdrawals, proposal lifecycle, scoped proposal actions, execution-target allowlist, and the first PASG-holder tests. This replaced the old fixed-member administrative model with a protocol-scoped PASG governance base.

## Highlights
- Added `DepositVotingPower`, `WithdrawVotingPower`, `Propose`, `Vote`, `Execute`, and `Close` around native `upasg` balances.
- Replaced arbitrary admin-style proposal payloads with `ProposalAction` variants that keep governance inside protocol scope.
- Added regression coverage for threshold checks, allowed-target execution, and governance-power locking while proposals are open.

## Verification
- `cargo test -p multisig --lib --tests`
- `cargo check -p multisig`

## Self-Check: PASSED
- New governance surface is in `contracts/core/multisig/src/msg.rs` and `contracts/core/multisig/src/contract.rs`.
- PASG-holder lifecycle tests pass in `contracts/core/multisig/src/tests/governance.rs`.
