---
phase: 02-pasg-utility-surface
plan: 01
subsystem: payments
tags: [pasg, upasg, cosmwasm, streaming-billing, documentation]
requires:
  - phase: 01-protocol-hardening-boundaries
    provides: verified streaming-billing authority boundaries and protocol/off-chain scope guidance
provides:
  - canonical PASG utility query surface in streaming-billing
  - native-upasg-first integrator guidance across repo docs
  - explicit split-router boundary as a generic PASG consumer
affects: [02-02, streaming-billing, split-router, marketplace-v3, minter-v2, auction-english]
tech-stack:
  added: []
  patterns: [native-denom-first PASG utility surface, query-first PASG consumer integration, generic router boundary]
key-files:
  created: [.planning/phases/02-pasg-utility-surface/deferred-items.md]
  modified: [contracts/core/streaming-billing/src/msg.rs, contracts/core/streaming-billing/src/contract.rs, contracts/core/streaming-billing/src/state.rs, contracts/core/streaming-billing/src/lib.rs, contracts/core/streaming-billing/src/tests.rs, contracts/core/streaming-billing/README.md, contracts/core/split-router/README.md, README.md, CLAUDE.md]
key-decisions:
  - "streaming-billing owns the canonical PASG utility interface via QueryMsg::PasgUtility"
  - "upasg remains the only in-repo PASG settlement model; wrappers are compatibility-only shims"
  - "split-router stays denom-agnostic while billing and subscription policy remain off-chain"
patterns-established:
  - "Canonical query first: consumers should read PasgUtility instead of duplicating PASG assumptions"
  - "Native-denom first: PASG semantics are expressed through upasg, not an in-repo token contract"
requirements-completed: [PASG-02, PASG-03]
duration: 6 min
completed: 2026-03-18
---

# Phase 02 Plan 01: PASG Utility Surface Summary

**Canonical PASG utility metadata in `streaming-billing`, backed by native `upasg` guidance and a generic `split-router` boundary**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-18T20:35:38Z
- **Completed:** 2026-03-18T20:42:08Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Added a canonical `PasgUtility` query surface in `streaming-billing` for denom, conversion, compatibility, and scope metadata.
- Locked the native-denom-first PASG interface with package tests and a non-zero conversion-rate guard.
- Aligned repo docs so `upasg` is the unambiguous settlement model and `split-router` remains generic.

## Task Commits

Each task was committed atomically:

1. **Task 1: Canonicalize the PASG utility surface in streaming-billing** - `eceb5c9` (feat)
2. **Task 2: Publish the repo-wide native-denom stance and consumer guidance** - `93747af` (docs)

**Plan metadata:** Pending final docs commit

## Files Created/Modified
- `contracts/core/streaming-billing/src/msg.rs` - Added canonical PASG utility metadata, routes, and query response types.
- `contracts/core/streaming-billing/src/contract.rs` - Persisted PASG utility stance, exposed `PasgUtility`, and validated zero conversion rates.
- `contracts/core/streaming-billing/src/state.rs` - Stored the canonical PASG utility metadata alongside config.
- `contracts/core/streaming-billing/src/tests.rs` - Added regression coverage for query shape, native-denom stance, and split-router boundary drift.
- `contracts/core/streaming-billing/README.md` - Documented canonical entrypoints, scope boundaries, and native `upasg` semantics.
- `contracts/core/split-router/README.md` - Documented split-router as a denom-agnostic PASG consumer rather than a policy engine.
- `README.md` - Published repo-wide native `upasg` guidance and integrator entrypoints.
- `CLAUDE.md` - Added repository instructions for native-denom PASG semantics and split-router scope.
- `.planning/phases/02-pasg-utility-surface/deferred-items.md` - Logged the unrelated workspace verification blocker in `ecosystem-factory`.

## Decisions Made
- `streaming-billing` is the only source-of-truth PASG contract surface in this plan; consumers should query it instead of inventing local PASG policy.
- Native `upasg` remains the canonical settlement model in this workspace. Any future wrapper must be compatibility-only and forward to native semantics.
- `split-router` remains a generic routing primitive and does not absorb PASG fee treatment, platform billing, or subscription logic.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Rejected zero PASG conversion rates at instantiation**
- **Found during:** Task 1 (Canonicalize the PASG utility surface in streaming-billing)
- **Issue:** The contract allowed `points_per_denom = 0`, even though the canonical PASG conversion surface assumes valid point-to-PASG math.
- **Fix:** Added an instantiate-time `InvalidConversionRate` guard and covered it with a regression test.
- **Files modified:** `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/tests.rs`
- **Verification:** `cargo test -p streaming-billing --lib`; `cargo check -p streaming-billing`
- **Committed in:** `eceb5c9`

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** The auto-fix was required to make the canonical PASG utility surface internally consistent. No scope creep was introduced.

## Issues Encountered
- `cargo unit-test` and `cargo check --workspace` both fail in `contracts/core/ecosystem-factory` because `REQUESTS_BY_CREATOR` is referenced without being imported in `src/contract/execute.rs:188` and `src/contract/query.rs:84`.
- This blocker predates `02-01`, is outside the plan-owned PASG surface, and was logged in `.planning/phases/02-pasg-utility-surface/deferred-items.md` instead of being fixed inline.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 2 now has one canonical PASG utility surface and one repo-wide native-denom story for integrators.
- `02-02` can integrate fee-treatment hooks against `streaming-billing::PasgUtility` without redefining PASG policy in each consumer contract.
- Full workspace verification remains blocked until the unrelated `ecosystem-factory` import error is repaired.

## Self-Check: PASSED
- Found `.planning/phases/02-pasg-utility-surface/02-01-SUMMARY.md` on disk.
- Verified task commits `eceb5c9` and `93747af` exist in git history.
