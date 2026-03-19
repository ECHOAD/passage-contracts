# Phase 3 Intent Correction

**Date:** 2026-03-18
**Source:** `.planning/debug/phase-3-multisig-intent-mismatch.md`

## Correction

`contracts/core/multisig` is an existing admin-owner control plane. It should keep the stable-address signer, proposal, and execution role used to own `registry` and other protocol contracts through CosmWasm admin control.

Phase 3 must not replace that contract with PASG-holder voting logic.

## Required Invariants

- Preserve `multisig` as the admin-owner shell for `registry` and similar protocol contracts.
- Treat PASG governance as a separate voting/proposal layer, closer to DAO-style modules than to a rewrite of the base admin multisig.
- Keep governance scoped to PASG and protocol parameters only.
- Make the relationship between PASG governance and admin execution explicit in the replanned phase before implementation resumes.

## Planning Effect

The prior Phase 3 research and plans were based on the wrong architectural premise and are superseded.

Next action: replan Phase 3 against this corrected intent before reopening implementation work.