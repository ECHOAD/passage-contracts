# Phase 3: PASG Governance - Validation Strategy

**Defined:** 2026-03-18
**Updated:** 2026-03-18 after intent-mismatch debug
**Scope:** validation guardrails for the corrected Phase 3 architecture
**Status:** replan-required

## Validation Guardrails

The next Phase 3 plan must satisfy these guardrails before implementation work resumes:

- No task may replace `contracts/core/multisig` member-threshold admin semantics with PASG-holder voting semantics.
- PASG governance voting and proposal logic must live in a separate layer from the base admin-owner multisig.
- Public docs must keep one consistent story about `multisig` as the admin-owner surface for `registry` and similar protocol contracts.
- Governance scope must stay limited to PASG and protocol parameters, not general off-chain platform systems.

## Wave 0 Checks For Replan

- The replanned artifact set identifies a separate governance contract or module family.
- The replanned integration path explains how passed governance decisions reach the admin execution plane.
- The replanned docs do not instruct contributors to rewrite `contracts/core/multisig` into deposited-balance governance.

## Automated Verification To Keep

- `cargo test -p multisig --lib --tests`
- `cargo check -p multisig`

These commands remain relevant as regression protection for the restored admin multisig, but the detailed Phase 3 task map must be regenerated after replanning.