---
phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics
plan: 03
subsystem: nft
tags: [cosmwasm, cw721, plugin-assignment, relationships, schema]
requires:
  - phase: 11-01
    provides: tightened pg721 metadata semantics for world and plugin asset classes
provides:
  - dedicated world-plugin-assignment relationship contract crate
  - durable plugin-to-world rights that remain queryable after NFT resale
  - live cw721 owner-or-approval authorization checks plus crate-local schema generation
affects: [11-04, 11-05, nft-metadata-boundary, composed-assets]
tech-stack:
  added: [world-plugin-assignment]
  patterns: [durable relationship persistence, live cw721 authority checks, crate-local schema generation]
key-files:
  created: [contracts/relationship/world-plugin-assignment/Cargo.toml, contracts/relationship/world-plugin-assignment/README.md, contracts/relationship/world-plugin-assignment/examples/schema.rs]
  modified: [contracts/relationship/world-plugin-assignment/src/contract.rs, contracts/relationship/world-plugin-assignment/src/helpers.rs, contracts/relationship/world-plugin-assignment/src/state.rs, contracts/relationship/world-plugin-assignment/src/tests.rs]
key-decisions:
  - "Plugin-to-world durable rights are keyed by plugin collection plus token_id and world collection plus token_id instead of plain NFT ownership."
  - "Assignment authorization resolves against live cw721 owner or approval state for both the plugin NFT and the world NFT before rights are granted."
  - "Tests use a local mock NFT authority contract so the relationship crate stays on a single CosmWasm 2.x stack during verification."
patterns-established:
  - "Pattern 1: durable metaverse usage rights live in dedicated relationship contracts queried alongside NFT metadata instead of inside typed token extensions."
  - "Pattern 2: relationship crates can use local mock cw721 authority contracts in tests when legacy workspace crates would otherwise force incompatible CosmWasm versions."
requirements-completed: [NFT-03, REV-01, ARCH-01, QUAL-01]
duration: 1h 6m
completed: 2026-03-20
---

# Phase 11 Plan 03: Durable plugin-to-world assignment rights Summary

**Dedicated `world-plugin-assignment` persistence for plugin usage rights with live cw721 authority checks, resale-safe assignment retention, and crate-local schema generation**

## Performance

- **Duration:** 1h 6m
- **Started:** 2026-03-20T20:30:00Z
- **Completed:** 2026-03-20T21:36:28Z
- **Tasks:** 1
- **Files modified:** 10

## Accomplishments
- Added a dedicated `contracts/relationship/world-plugin-assignment` crate with instantiate, execute, and query surfaces for durable plugin-to-world assignments.
- Enforced assignment writes against live cw721 owner or approval state for both the plugin NFT and the target world NFT.
- Verified deterministic tests for unauthorized callers, approved operators, indexed queries, and assignment persistence after later plugin resale.

## Task Commits

Task commit history was not recorded in this closeout-only turn because execution resumed after an interrupted implementation session.

**Plan metadata:** pending

## Files Created/Modified
- `contracts/relationship/world-plugin-assignment/Cargo.toml` - workspace manifest for the new relationship contract crate.
- `contracts/relationship/world-plugin-assignment/README.md` - contract boundary and durable-rights semantics.
- `contracts/relationship/world-plugin-assignment/examples/schema.rs` - crate-local schema exporter.
- `contracts/relationship/world-plugin-assignment/src/msg.rs` - instantiate, execute, query, and response types for plugin-to-world assignment records.
- `contracts/relationship/world-plugin-assignment/src/state.rs` - durable assignment record storage keyed by plugin and world asset identity.
- `contracts/relationship/world-plugin-assignment/src/helpers.rs` - live cw721 owner and approval authority resolution.
- `contracts/relationship/world-plugin-assignment/src/contract.rs` - instantiate, assign, remove, and query logic.
- `contracts/relationship/world-plugin-assignment/src/tests.rs` - deterministic mock-cw721 tests for authorization, indexing, and resale persistence.

## Decisions Made
- Durable plugin rights are modeled as an explicit relationship record instead of metadata on the NFT itself.
- Assignment persistence does not depend on future plugin ownership; resale only changes the current owner query, not an existing granted relationship.
- The crate test harness uses a local mock NFT authority contract because pulling `pg721` into this new relationship crate would reintroduce incompatible CosmWasm versions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Replaced the initial `pg721`-based test harness with a local mock cw721 authority contract**
- **Found during:** Task 1 (RED/GREEN verification)
- **Issue:** The first harness pulled an incompatible `pg721` and `cosmwasm-std` stack into the new relationship crate tests.
- **Fix:** Removed the direct `pg721` test dependency and implemented a local mock NFT authority contract exposing owner and operator authorization paths.
- **Files modified:** `contracts/relationship/world-plugin-assignment/Cargo.toml`, `contracts/relationship/world-plugin-assignment/src/tests.rs`
- **Verification:** `cargo test -p world-plugin-assignment --lib` passed after the harness swap.
- **Committed in:** pending

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary test-harness correction only. No scope creep beyond making the planned crate verifiable on the active workspace stack.

## Issues Encountered
- Windows sandbox execution intermittently blocked Rust test runs, so final cargo verification required running outside the sandbox.
- The original closeout path was interrupted before task commits were created, so this summary records the verified state and metadata closeout only.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- `11-04` can now reference a concrete on-chain relationship surface for durable plugin-world rights instead of describing those rights abstractly.
- The crate’s schema output and query shapes are ready for the Phase 11 doc and example alignment work.

## Self-Check: PASSED

---
*Phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics*
*Completed: 2026-03-20*
