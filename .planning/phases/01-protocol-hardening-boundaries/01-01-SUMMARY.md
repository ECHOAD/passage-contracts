---
phase: 01-protocol-hardening-boundaries
plan: 01
subsystem: payments
tags: [cosmwasm, streaming-billing, auth, billing, registry, cw721]
requires: []
provides:
  - backend-operator-gated streaming session lifecycle
  - registry-backed world ownership verification for billing config
  - global fiat replay protection and bounded session settlement
affects: [01-03, phase-02-pasg-utility-surface, streaming-billing]
tech-stack:
  added: []
  patterns:
    - explicit operator gating for service-invoked billing flows
    - canonical world metadata derived from registry and cw721 ownership
key-files:
  created:
    - contracts/core/streaming-billing/src/tests.rs
  modified:
    - contracts/core/streaming-billing/src/contract.rs
    - contracts/core/streaming-billing/src/msg.rs
    - contracts/core/streaming-billing/src/state.rs
    - contracts/core/streaming-billing/src/lib.rs
    - contracts/core/streaming-billing/src/error.rs
key-decisions:
  - "Session start and stop remain service-invoked paths and now require admin or the configured backend operator."
  - "World rate configuration is anchored to registry validation plus cw721 owner-of, not caller-supplied metadata."
  - "Fiat purchase replay protection is global and timestamp-bounded to keep off-chain payment reporting honest."
patterns-established:
  - "Hybrid billing paths must model an explicit backend/operator role in contract state."
  - "Caller-provided world collection values are compatibility hints only; stored config remains canonical."
requirements-completed:
  - ARCH-01
  - REV-03
duration: 75min
completed: 2026-03-18
---

# Phase 1: Protocol Hardening & Boundaries Summary

**Streaming billing now enforces backend-operated session lifecycle, verified world ownership, and global fiat replay bounds.**

## Performance

- **Duration:** 75 min
- **Started:** 2026-03-18T09:00:00-04:00
- **Completed:** 2026-03-18T10:15:00-04:00
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments

- Added `backend_operator` configuration and enforced it on `StartSession` and `StopSession`.
- Reworked world-rate setup so registry registration and cw721 ownership, not caller input, define the canonical billing world.
- Added crate-local regression coverage for auth, replay protection, timestamp freshness, and maximum session duration.

## Task Commits

Each task was consolidated into a single plan commit:

1. **Plan 01 hardening and regression coverage** - `3b5e970` (fix)

**Plan metadata:** included in the phase documentation commit.

## Files Created/Modified

- `contracts/core/streaming-billing/src/contract.rs` - Added operator gating, registry/cw721 ownership checks, replay index usage, and duration bounds.
- `contracts/core/streaming-billing/src/msg.rs` - Added backend operator config and minimal registry/cw721 query message types.
- `contracts/core/streaming-billing/src/state.rs` - Added backend operator state and global fiat transaction index.
- `contracts/core/streaming-billing/src/tests.rs` - Added regression tests for auth, ownership, replay, and billing bounds.
- `contracts/core/streaming-billing/src/lib.rs` - Registered the new test module.
- `contracts/core/streaming-billing/src/error.rs` - Added explicit world-collection mismatch handling.

## Decisions Made

- Session lifecycle stays hybrid and service-invoked; public callers are no longer trusted.
- `SetWorldRate` allows admin override, but persisted ownership always comes from the verified token owner.
- Fiat purchase freshness is enforced with a 15-minute max age and future timestamps are rejected.

## Deviations from Plan

### Auto-fixed Issues

**1. [Blocking] Fixed pre-existing compile errors inside `streaming-billing`**
- **Found during:** verification
- **Issue:** the crate still had a stale `Bound` reference, invalid `Uint128` math, and incorrect submessage handling.
- **Fix:** imported `cw_storage_plus::Bound`, switched to `checked_mul_floor`, mapped withdrawal overflow explicitly, and used `add_submessages`.
- **Files modified:** `contracts/core/streaming-billing/src/contract.rs`
- **Verification:** `cargo test -p streaming-billing --lib`, `cargo check -p streaming-billing`
- **Committed in:** `3b5e970`

---

**Total deviations:** 1 auto-fixed
**Impact on plan:** Required for the planned hardening to compile and verify. No scope creep beyond crate correctness.

## Issues Encountered

- The crate had no local test harness and also contained dormant compile issues. Both were resolved before verification.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `streaming-billing` now has explicit service-boundary primitives that Phase 1 documentation can reference directly.
- Later PASG utility work can build against a verified operator, oracle, and world-ownership model instead of comments.

---
*Phase: 01-protocol-hardening-boundaries*
*Completed: 2026-03-18*