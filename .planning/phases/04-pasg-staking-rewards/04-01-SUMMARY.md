---
phase: 04-pasg-staking-rewards
plan: 01
subsystem: native-validator-staking-model
tags: [pasg, staking, validators, docs]
provides:
  - native PASG validator delegation model
  - explicit staking boundary against NFT staking contracts
  - chain-level unbonding and validator-selection guidance
affects: [docs, planning]
key-files:
  modified:
    - .planning/phases/04-pasg-staking-rewards/04-RESEARCH.md
    - README.md
    - docs/01-end-to-end-setup.md
    - docs/02-method-reference.md
completed: 2026-03-19
---

# Phase 04 Plan 01 Summary

Plan 04-01 locked the staking architecture around Passage validator delegation instead of a repo-local PASG staking vault.

Delivered outcomes:
- expanded `04-RESEARCH.md` with a native responsibility map and explicit delegate, undelegate, redelegate action model
- updated `README.md` to define PASG staking as chain-native staking with validator delegation and to mark NFT staking crates as non-target for PASG
- added staking guidance to `docs/01-end-to-end-setup.md` so integrators see the validator flow and the repo boundary in the main setup walkthrough
- added a dedicated `PASG native staking model` section to `docs/02-method-reference.md` covering source of truth, action surface, unbonding, and adapter constraints

Verification recorded for this plan:
- doc-level validation against the 04-01 acceptance criteria
- no contract tests required in this wave because the plan only locked architecture and documentation boundaries
