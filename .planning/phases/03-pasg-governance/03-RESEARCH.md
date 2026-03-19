# Phase 3: PASG Governance - Research

**Researched:** 2026-03-18
**Updated:** 2026-03-18 after intent-mismatch debug
**Domain:** PASG governance architecture
**Confidence:** HIGH

## Summary

The repository already had an established governance/admin primitive before Phase 3 execution: `contracts/core/multisig` was a fixed-member administrative multisig used as the stable owner/admin surface for `registry` and other protocol contracts. The Phase 3 mismatch came from treating that existing contract as the target for PASG-holder governance, which collapsed two different roles into one contract.

The corrected research conclusion is:

- `multisig` must remain the admin-owner control plane.
- PASG governance must be introduced as a separate voting/proposal layer.
- Phase 3 must be replanned before implementation resumes.

## What We Know

- Pre-Phase-3 `multisig` used `{ members, threshold, max_voting_period_secs }` and self-call `UpdateMembers` semantics.
- `docs/01-end-to-end-setup.md`, `docs/02-method-reference.md`, and the pre-Phase-3 `docs/04-multisig-governance.md` all describe `multisig` as the admin address for `registry` and other critical protocol contracts.
- `.planning/PROJECT.md` already recorded that admin governance and operational safety primitives existed through `multisig` before PASG governance work started.

## Corrected Direction

Phase 3 should add PASG governance without overwriting the admin multisig primitive. That means the next plan must define a separate governance layer and an explicit relationship between that layer and the existing admin execution plane.

The exact decomposition remains open. Acceptable directions may include a dedicated PASG governance contract family or DAO-style voting/proposal modules, but the base `multisig` contract should not be repurposed into deposited-balance governance.

## Open Questions For Replan

1. What contract or module family will own PASG proposal and voting semantics?
2. How does a passed governance decision authorize execution through the admin-owner plane?
3. What emergency or operator path, if any, remains with the base multisig once PASG governance exists?
4. Which protocol contracts stay directly owned by `multisig`, and which, if any, should be owned by a new governance executor?

## Primary Sources

- `.planning/debug/phase-3-multisig-intent-mismatch.md`
- `.planning/PROJECT.md`
- `contracts/core/multisig/src/msg.rs` at `4f259ac^`
- `contracts/core/multisig/README.md` at `4f259ac^`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/04-multisig-governance.md` at `4f259ac^`
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`