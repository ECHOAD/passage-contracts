---
phase: 04-pasg-staking-rewards
plan: 02
subsystem: native-reward-and-query-surface
tags: [pasg, staking, docs, governance]
provides:
  - chain-native reward and undelegation guidance
  - explicit no-adapter stance for validator discovery and rewards
  - governance boundary alignment for native staking
affects: [docs, pasg-governance]
key-files:
  modified:
    - docs/02-method-reference.md
    - docs/03-json-examples.md
    - docs/04-multisig-governance.md
completed: 2026-03-19
---

# Phase 04 Plan 02 Summary

Plan 04-02 closed the reward and query model around chain-native staking without keeping any validator metadata surface in repo-local contracts.

Delivered outcomes:
- aligned `docs/02-method-reference.md` to treat validator discovery, delegations, undelegations, and rewards as chain-native reads
- aligned `docs/03-json-examples.md` to show staking and distribution CLI/query examples only, with no `registry` validator metadata path
- tightened `docs/04-multisig-governance.md` so PASG governance does not stage validator metadata actions through `multisig`
- removed the temporary `registry` and `pasg-governance` validator metadata overreach after review confirmed the node endpoint already covers validator discovery

Verification recorded for this plan:
- `cargo check -p registry -p pasg-governance`
- `cargo test -p registry --lib`
- `cargo test -p pasg-governance --lib`

## Deviations from Plan

The earlier adapter interpretation was reverted. Final phase outcome is the stricter no-adapter stance: validator metadata, rewards, and validator discovery stay chain-native.

## Auth Gates

None.

## Self-Check: PASSED

- Overreach removed from `registry`
- Overreach removed from `pasg-governance`
- Staking docs now reference chain-native surfaces only
