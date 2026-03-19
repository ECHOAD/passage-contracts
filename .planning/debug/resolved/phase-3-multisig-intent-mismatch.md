---
status: resolved
trigger: "Investigate issue: phase-3-multisig-intent-mismatch"
created: 2026-03-18T00:00:00-04:00
updated: 2026-03-18T23:24:59.5664080-04:00
---

## Current Focus

hypothesis: The rollback and planning correction resolved the architectural mismatch by restoring `multisig` as the admin-owner control plane and separating future PASG governance work.
test: Archive the confirmed fix, preserve the investigation record, and update the knowledge base with the resolved pattern.
expecting: The session can be closed cleanly with the resolved debug record and knowledge base entry matching the verified root cause.
next_action: archive the resolved session and sync debug documentation

## Symptoms

expected: `contracts/core/multisig` should primarily exist to own `registry` and other protocol contracts through CosmWasm admin control, executing actions as the admin control plane. Governance votes should be a separate layer, more like DAO DAO modules built around voting/proposal semantics rather than replacing the base admin multisig itself.
actual: Phase 3 was planned and implemented as if `multisig` itself should become the PASG-holder governance contract with deposited `upasg`, delegation, quorum, approval thresholds, and scoped execution.
errors: Potential architectural mismatch between the original purpose of `multisig` and the implemented Phase 3 direction.
reproduction: Review the existing `contracts/core/multisig` role, `docs/04-multisig-governance.md`, prior admin-owner usage around `registry`, and the executed Phase 3 artifacts/commits (`4f259ac`, `cc55457`). Determine whether Phase 3 reinterpreted the contract incorrectly.
started: Issue surfaced immediately after Phase 3 completion when comparing the result against the intended design of multisig-as-owner/admin plus separate governance voting.

## Eliminated

## Evidence

- timestamp: 2026-03-18T00:05:00-04:00
  checked: knowledge base
  found: The knowledge base exists but only contains an unrelated workspace compile issue; there is no prior resolved pattern for multisig intent mismatch.
  implication: This issue needed first-principles investigation rather than a known fix.
- timestamp: 2026-03-18T00:06:00-04:00
  checked: current `contracts/core/multisig` contract and messages before rollback
  found: `InstantiateMsg` required `pasg_denom`, quorum, approval, proposal threshold, voting period, and execution targets, while `ExecuteMsg` included deposit, withdrawal, delegation, proposal, vote, execute, and close flows.
  implication: The Phase 3 implementation had turned `multisig` into a token-holder governance module.
- timestamp: 2026-03-18T00:07:00-04:00
  checked: current `docs/04-multisig-governance.md` before rollback
  found: The doc explicitly said `multisig` was no longer a fixed-signer administrative shell and was now the PASG-holder governance primitive for protocol-scoped decisions.
  implication: The reinterpretation was intentional and documented, not an isolated code drift.
- timestamp: 2026-03-18T00:12:00-04:00
  checked: repo-wide references to `multisig` and `registry`
  found: `docs/01-end-to-end-setup.md` still instructed operators to deploy `multisig` first and use it as both `registry` admin and the CosmWasm instance admin, while `docs/02-method-reference.md` described `multisig` as a proposal-based admin contract.
  implication: The broader repo still framed `multisig` as the protocol admin-owner surface.
- timestamp: 2026-03-18T00:14:00-04:00
  checked: Phase 3 commit summaries
  found: Commit `4f259ac` was titled `feat(03): implement PASG governance multisig` and rewrote `contracts/core/multisig` plus its docs. Commit `cc55457` then marked the PASG governance phase as complete in planning state and summaries.
  implication: The reinterpretation was the declared Phase 3 deliverable and was codified as completed project state.
- timestamp: 2026-03-18T00:20:00-04:00
  checked: pre-Phase-3 `contracts/core/multisig` API and README from `4f259ac^`
  found: Before Phase 3, `InstantiateMsg` used `{ members, threshold, max_voting_period_secs }`, `ExecuteMsg` exposed proposal, vote, execute, close, and self-call `UpdateMembers`, and the README described `multisig` as a proposal-based administrative multisig for `registry`, `ecosystem-factory`, `marketplace-v3`, and `auction-english`.
  implication: The original contract role was clearly an admin-owner multisig with stable-address signer rotation.
- timestamp: 2026-03-18T00:22:00-04:00
  checked: current public setup and method docs
  found: `docs/01-end-to-end-setup.md` and `docs/02-method-reference.md` still matched the old member-threshold admin-owner model even while Phase 3 had changed the contract itself.
  implication: The repo had contradictory governance stories after Phase 3 completion.
- timestamp: 2026-03-18T00:24:00-04:00
  checked: `.planning/phases/03-pasg-governance/03-RESEARCH.md` before correction
  found: The research explicitly recommended evolving the existing `multisig` contract family into a PASG governance module instead of introducing a separate governance layer.
  implication: The architectural reinterpretation originated in Phase 3 planning and research.
- timestamp: 2026-03-18T00:30:00-04:00
  checked: repo-level project framing and pre-Phase-3 guidance
  found: `.planning/PROJECT.md` already said admin governance and operational safety primitives existed through `multisig`, while the pre-Phase-3 `CLAUDE.md`, `README.md`, and `docs/04-multisig-governance.md` all described `multisig` as proposal-based administrative control with signer rotation.
  implication: Phase 3 erased an existing architectural primitive instead of filling an unmet governance gap.
- timestamp: 2026-03-18T00:45:00-04:00
  checked: rollback and planning correction
  found: Reverting commits `cc55457` and `4f259ac` restored the member-threshold `multisig` contract, the admin-owner docs, and reopened Phase 3 planning; additional planning edits now explicitly say `multisig` must remain the admin-owner control plane and that PASG governance must be replanned as a separate layer.
  implication: The code, docs, and planning state now align with the intended architecture again.
- timestamp: 2026-03-18T01:05:00-04:00
  checked: self-verification commands
  found: `cargo test -p multisig --lib --tests` passed with 3 tests, `cargo check -p multisig` passed, a positive `rg` check found the restored member/admin multisig language, and a negative `rg` check found no remaining PASG-holder governance surface in the restored `multisig` contract/docs.
  implication: The rollback is mechanically sound and the repo no longer presents `multisig` as deposited-balance governance.
- timestamp: 2026-03-18T23:24:59.5664080-04:00
  checked: human verification response
  found: The user confirmed the original issue is fixed in the intended workflow/environment.
  implication: The fix is verified end-to-end and the debug session can be archived.

## Resolution

root_cause: Phase 3 planning and implementation treated the existing `contracts/core/multisig` admin-owner contract as the place to build PASG-holder governance, even though the repo already depended on that contract as the stable admin/control plane for `registry` and other protocol contracts.
fix: Reverted the Phase 3 `multisig` reinterpretation, restored the pre-Phase-3 admin multisig code and docs, reopened Phase 3 in roadmap/state/requirements terms, and corrected the Phase 3 planning artifacts so they explicitly preserve `multisig` and require PASG governance to be replanned as a separate layer.
verification: `cargo test -p multisig --lib --tests` passed; `cargo check -p multisig` passed; repo text checks confirm the restored member-threshold admin multisig surface and no remaining PASG-holder governance API in the reverted contract/docs; the user confirmed on 2026-03-18 that the original issue is fixed in the real workflow/environment.
files_changed:
  - .planning/PROJECT.md
  - .planning/ROADMAP.md
  - .planning/STATE.md
  - .planning/phases/03-pasg-governance/03-01-PLAN.md
  - .planning/phases/03-pasg-governance/03-02-PLAN.md
  - .planning/phases/03-pasg-governance/03-03-PLAN.md
  - .planning/phases/03-pasg-governance/03-INTENT-CORRECTION.md
  - .planning/phases/03-pasg-governance/03-RESEARCH.md
  - .planning/phases/03-pasg-governance/03-VALIDATION.md
  - CLAUDE.md
  - README.md
  - contracts/core/multisig/README.md
  - contracts/core/multisig/src/contract.rs
  - contracts/core/multisig/src/error.rs
  - contracts/core/multisig/src/msg.rs
  - contracts/core/multisig/src/state.rs
  - contracts/core/multisig/src/tests/governance.rs
  - docs/04-multisig-governance.md

