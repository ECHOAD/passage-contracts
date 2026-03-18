---
phase: 01-protocol-hardening-boundaries
plan: 02
subsystem: api
tags: [cosmwasm, marketplace-v3, registry, collection-factory, ecosystem-factory, pagination, reply]
requires: []
provides:
  - honest query pagination and collection-scoped filters
  - registry-authorized minter pagination inside collection prefixes
  - reply-data parsing tests for factory instantiate flows
affects: [01-03, phase-02-pasg-utility-surface, marketplace-v3, registry]
tech-stack:
  added: []
  patterns:
    - storage-aware pagination using prefix-bound ranges
    - reply parsing anchored to instantiate response data
key-files:
  created:
    - contracts/nft/marketplace-v3/src/contract/query/tests.rs
    - contracts/core/registry/src/tests/minters.rs
    - contracts/core/collection-factory/src/contract/reply/tests.rs
    - contracts/core/ecosystem-factory/src/contract/reply/tests.rs
  modified:
    - contracts/nft/marketplace-v3/src/contract/query.rs
    - contracts/core/registry/src/contract/query.rs
    - contracts/core/collection-factory/src/contract/helpers.rs
    - contracts/core/collection-factory/src/contract/reply.rs
    - contracts/core/ecosystem-factory/src/contract/helpers.rs
    - contracts/core/ecosystem-factory/src/contract/reply.rs
key-decisions:
  - "Public query surfaces must honor the pagination and scoping they advertise instead of silently ignoring parameters."
  - "Factory reply handling should trust instantiate reply data, not event scraping assumptions."
patterns-established:
  - "Collection-scoped scans should use prefixed storage ranges before any in-memory filtering."
  - "Reply-path regressions need dedicated tests at the reply module boundary."
requirements-completed:
  - ARCH-02
duration: 55min
completed: 2026-03-18
---

# Phase 1: Protocol Hardening & Boundaries Summary

**Marketplace, registry, and factory reply paths now honor their public contract surfaces instead of relying on brownfield shortcuts.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-03-18T09:05:00-04:00
- **Completed:** 2026-03-18T10:00:00-04:00
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments

- Fixed `marketplace-v3` query handlers so collection filters, cursor pagination, and scoped ask counts behave as declared.
- Fixed registry authorized-minter pagination to page inside the collection prefix rather than scanning the full map.
- Normalized collection and ecosystem factory reply handling around instantiate response data and added regression tests.

## Task Commits

Each task was consolidated into a single plan commit:

1. **Plan 02 query and reply normalization** - `1a79847` (fix)

**Plan metadata:** included in the phase documentation commit.

## Files Created/Modified

- `contracts/nft/marketplace-v3/src/contract/query.rs` - Implemented real cursor handling and collection-aware query logic.
- `contracts/nft/marketplace-v3/src/contract/query/tests.rs` - Added direct coverage for the corrected public query surface.
- `contracts/core/registry/src/contract/query.rs` - Scoped authorized-minter pagination to the collection prefix.
- `contracts/core/registry/src/tests/minters.rs` - Added regression coverage for the authorized-minter query fix.
- `contracts/core/collection-factory/src/contract/helpers.rs` - Added local reply-data parsing helper for instantiate responses.
- `contracts/core/collection-factory/src/contract/reply.rs` - Wired the reply module to use instantiate reply data and enabled tests.
- `contracts/core/ecosystem-factory/src/contract/helpers.rs` - Added reply-data extraction helper.
- `contracts/core/ecosystem-factory/src/contract/reply.rs` - Switched ecosystem reply parsing to the helper and enabled tests.

## Decisions Made

- Query correctness took precedence over keeping placeholder APIs compatible with broken behavior.
- Reply paths now depend on `msg_responses.first().value`, which is closer to the actual instantiate contract boundary than event scraping.

## Deviations from Plan

### Auto-fixed Issues

**1. [Blocking] Used local helper for collection-factory instantiate reply parsing**
- **Found during:** Task 3
- **Issue:** the plan needed reply-data parsing but adding a new dependency was unnecessary in the owned edit scope.
- **Fix:** implemented a local helper with the same behavior and covered it with reply tests.
- **Files modified:** `contracts/core/collection-factory/src/contract/helpers.rs`
- **Verification:** `cargo test -p collection-factory --lib`
- **Committed in:** `1a79847`

---

**Total deviations:** 1 auto-fixed
**Impact on plan:** No functional scope change. The plan outcome stayed the same while keeping the patch local and reviewable.

## Issues Encountered

- None beyond the local helper decision noted above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The brownfield query/reply surface is now stable enough to document in the safety matrix.
- Later PASG work can rely on declared query behavior and reply semantics without carrying Phase 1 ambiguity forward.

---
*Phase: 01-protocol-hardening-boundaries*
*Completed: 2026-03-18*