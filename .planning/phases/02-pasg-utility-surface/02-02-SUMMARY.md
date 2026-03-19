---
phase: 02-pasg-utility-surface
plan: 02
subsystem: payments
tags: [pasg, upasg, streaming-billing, split-router, marketplace-v3, minter-v2, auction-english]
requires:
  - phase: 02-pasg-utility-surface
    provides: canonical PASG utility query surface and native `upasg` integration stance from 02-01
provides:
  - native `upasg` fee treatment enforced in streaming-billing
  - generic split-router compatibility hooks for PASG-owned routing calls
  - consumer payment flows stamped with the canonical PASG utility reference instead of hidden local assumptions
affects: [streaming-billing, split-router, marketplace-v3, minter-v2, auction-english]
tech-stack:
  added: []
  patterns: [native-pasg enforcement, generic routing metadata, shared PASG utility reference attributes]
key-files:
  created:
    - contracts/nft/marketplace-v3/src/contract/execute/tests.rs
    - contracts/nft/minter-v2/src/contract/execute/tests.rs
  modified:
    - contracts/core/streaming-billing/src/contract.rs
    - contracts/core/streaming-billing/src/msg.rs
    - contracts/core/streaming-billing/src/state.rs
    - contracts/core/streaming-billing/src/tests.rs
    - contracts/core/streaming-billing/README.md
    - contracts/core/split-router/src/msg.rs
    - contracts/core/split-router/src/contract.rs
    - contracts/core/split-router/src/contract/execute.rs
    - contracts/core/split-router/src/contract/query.rs
    - contracts/core/split-router/src/contract/execute/tests.rs
    - contracts/core/split-router/src/contract/query/tests.rs
    - contracts/nft/marketplace-v3/Cargo.toml
    - contracts/nft/marketplace-v3/src/msg.rs
    - contracts/nft/marketplace-v3/src/contract/execute.rs
    - contracts/nft/minter-v2/Cargo.toml
    - contracts/nft/minter-v2/src/msg.rs
    - contracts/nft/minter-v2/src/contract/execute.rs
    - contracts/nft/auction-english/Cargo.toml
    - contracts/nft/auction-english/src/execute.rs
    - contracts/nft/auction-english/src/execute/tests.rs
    - Cargo.lock
    - .planning/phases/02-pasg-utility-surface/deferred-items.md
key-decisions:
  - "streaming-billing canonicalizes PASG settlement to native upasg and treats non-native denom config as compatibility-only metadata"
  - "split-router closes the route_world_revenue integration gap with a generic compatibility alias and generic routing metadata instead of PASG policy logic"
  - "consumer payment flows expose the canonical PASG utility query route via shared attributes rather than silently re-encoding PASG assumptions per contract"
patterns-established:
  - "Canonical reference attributes: marketplace, mint, and auction payment flows now emit the shared PASG utility query route and native-denom flag"
  - "Generic routing compatibility: split-router accepts streaming-billing revenue routing without learning PASG economics"
requirements-completed: [PASG-01]
duration: 201 min
completed: 2026-03-18
---

# Phase 02 Plan 02: PASG Fee-Treatment Hooks Summary

**PASG fee treatment now has one native-denom owner, one generic routing compatibility layer, and explicit consumer-flow references to the canonical utility surface**

## Performance

- **Duration:** 201 min
- **Started:** 2026-03-18T20:56:28Z
- **Completed:** 2026-03-19T00:17:10Z
- **Tasks:** 3
- **Files modified:** 23

## Accomplishments
- Enforced native `upasg` PASG handling in `streaming-billing`, including compatibility-only metadata for non-native denom config and regression coverage for native and non-native deposit paths.
- Kept `split-router` generic while adding a bounded `RouteWorldRevenue` compatibility alias and a generic routing metadata query that confirms passthrough behavior without encoding PASG policy.
- Rebound marketplace, minter, and auction payment flows to the shared PASG utility surface by importing the canonical PASG query reference from `streaming-billing` and emitting standardized PASG utility attributes on fee-bearing paths.

## Task Commits

Each task was committed atomically:

1. **Task 1: Enforce PASG fee treatment in streaming-billing** - `f7ec752` (feat)
2. **Task 2: Keep split-router generic while exposing bounded PASG-aware hooks** - `92e9bd8` (feat)
3. **Task 3: Rebind consumer payment flows to the shared PASG surface** - `1b38874` (feat)

**Plan metadata:** Pending final docs commit

## Files Created/Modified
- `contracts/core/streaming-billing/src/contract.rs` - Canonicalized native `upasg` enforcement and compatibility-only denom metadata.
- `contracts/core/streaming-billing/src/msg.rs` - Exported canonical PASG constants used by downstream consumers.
- `contracts/core/split-router/src/msg.rs` - Added generic routing metadata and the `RouteWorldRevenue` compatibility alias.
- `contracts/core/split-router/src/contract/execute.rs` - Delegated compatibility routing through the existing split path while preserving denom-agnostic behavior.
- `contracts/core/split-router/src/contract/query.rs` - Added routing metadata query surface.
- `contracts/nft/marketplace-v3/src/contract/execute.rs` - Added canonical PASG utility reference attributes to sale settlement flows.
- `contracts/nft/minter-v2/src/contract/execute.rs` - Added canonical PASG utility reference attributes to mint payment and withdrawal flows.
- `contracts/nft/auction-english/src/execute.rs` - Added canonical PASG utility reference attributes to bid and settlement flows.
- `contracts/nft/marketplace-v3/src/contract/execute/tests.rs` - Added marketplace PASG utility attribute coverage.
- `contracts/nft/minter-v2/src/contract/execute/tests.rs` - Added minter PASG utility attribute coverage.
- `contracts/nft/auction-english/src/execute/tests.rs` - Extended auction flow tests to assert canonical PASG utility attributes.
- `.planning/phases/02-pasg-utility-surface/deferred-items.md` - Logged the additional out-of-scope workspace failure discovered during workspace verification.

## Decisions Made
- Native `upasg` remains the only canonical PASG settlement path in-repo; non-native config is compatibility metadata, not a second PASG model.
- `split-router` now accepts the `RouteWorldRevenue` execute shape required by `streaming-billing`, but it still behaves as a generic passthrough router rather than a PASG policy engine.
- Consumer contracts now point back to the shared PASG utility surface explicitly through imported constants and response attributes instead of leaving that contract boundary implicit.

## Deviations from Plan

### Manual containment

**1. Workspace-wide verification exposed unrelated legacy blockers outside 02-02 ownership**
- **Found during:** Task 3 verification
- **Issue:** `cargo check --workspace` still fails outside the plan-owned PASG files.
- **Containment:** Logged both failures in `.planning/phases/02-pasg-utility-surface/deferred-items.md` instead of fixing unrelated contracts inline.
- **Verification impact:** Targeted PASG package tests passed; workspace sign-off remains blocked until the unrelated contracts are repaired.

---

**Total deviations:** 1 containment note
**Impact on plan:** No PASG scope expansion. The plan-owned contracts are complete, but full workspace verification is still blocked by unrelated packages.

## Issues Encountered
- `cargo check --workspace` still fails in `contracts/core/ecosystem-factory` because `REQUESTS_BY_CREATOR` is missing from `src/contract/execute.rs:188` and `src/contract/query.rs:84`.
- `cargo check --workspace` also fails in `contracts/nft/minter-metadata-onchain/src/contract.rs:102` because `Pg721InstantiateMsg` initialization is missing the required `nft_type` field.
- Both blockers are outside `02-02` ownership and were logged to `.planning/phases/02-pasg-utility-surface/deferred-items.md`.

## User Setup Required

None - no external service configuration required.

## Verification
- `cargo test -p streaming-billing --lib`
- `cargo check -p streaming-billing`
- `cargo test -p split-router --lib`
- `cargo check -p split-router`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p minter-v2 --lib --tests`
- `cargo test -p auction-english --lib`
- `cargo check --workspace` (fails only on the unrelated blockers recorded above)

## Next Phase Readiness
- `02-03` can now document concrete, queryable PASG semantics across billing, routing, marketplace, minter, and auction surfaces.
- The canonical PASG utility story is now visible in code and events, not just in repo prose.
- Final workspace verification still depends on resolving the out-of-scope `ecosystem-factory` and `minter-metadata-onchain` failures.

## Self-Check: PASSED
- Found `.planning/phases/02-pasg-utility-surface/02-02-SUMMARY.md` on disk.
- Verified task commits `f7ec752`, `92e9bd8`, and `1b38874` exist in git history.