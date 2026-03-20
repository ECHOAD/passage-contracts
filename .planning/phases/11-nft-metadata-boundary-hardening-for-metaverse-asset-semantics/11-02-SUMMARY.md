---
phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics
plan: 02
subsystem: nft
tags: [cosmwasm, cw721, progression, schema, metadata]
requires:
  - phase: 11-01
    provides: tightened pg721 metadata semantics and standardized avatar/companion identity types
provides:
  - dedicated asset progression contract crate for avatar and companion save-point snapshots
  - live cw721 owner or approval authorization for progression writes
  - crate-local schema outputs and deterministic progression tests
affects: [11-03, 11-04, nft-metadata-boundary, composed-assets]
tech-stack:
  added: [asset-progression]
  patterns: [world-scoped snapshot persistence, live cw721 authorization checks, crate-local schema generation]
key-files:
  created: [contracts/nft/asset-progression/Cargo.toml, contracts/nft/asset-progression/README.md, contracts/nft/asset-progression/schema/execute_msg.json]
  modified: [contracts/nft/asset-progression/src/contract.rs, contracts/nft/asset-progression/src/helpers.rs, contracts/nft/asset-progression/src/tests.rs, contracts/relationship/world-plugin-assignment/Cargo.toml]
key-decisions:
  - "Progression snapshots are keyed by collection, token_id, and world while the NFT remains the canonical durable asset identity."
  - "Save access is authorized against live cw721 owner or approval state instead of mint-flow shortcuts or metadata mutation."
  - "Schema generation writes into the crate-local schema directory to avoid clobbering workspace-root schema artifacts."
patterns-established:
  - "Pattern 1: mutable gameplay state lives in a dedicated contract queried alongside pg721 metadata instead of inside token extension fields."
  - "Pattern 2: progression-capable asset contracts validate supported asset kinds from live pg721 token metadata before persisting state."
requirements-completed: [NFT-02, NFT-03, ARCH-01, QUAL-01]
duration: 12 min
completed: 2026-03-20
---

# Phase 11 Plan 02: Dedicated progression snapshots bound to live avatar and companion ownership Summary

**Dedicated `asset-progression` save-point persistence for avatar and companion NFTs with live cw721 authorization, world-scoped snapshots, and crate-local schema output**

## Performance

- **Duration:** 12 min
- **Started:** 2026-03-20T21:20:48Z
- **Completed:** 2026-03-20T21:33:12Z
- **Tasks:** 1
- **Files modified:** 13

## Accomplishments
- Added a dedicated `contracts/nft/asset-progression` crate with instantiate, execute, and query surfaces for progression snapshots.
- Enforced snapshot writes against live cw721 owner or active approval state and validated `avatar` or `companion` token kinds from pg721 metadata.
- Generated crate-local JSON schemas and deterministic tests covering save, overwrite, approval, and rejection paths.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define progression snapshot state in a dedicated asset-lifecycle contract** - `a483cad` (test)
2. **Task 1: Define progression snapshot state in a dedicated asset-lifecycle contract** - `80ab2ef` (feat)

**Plan metadata:** pending

_Note: TDD task used separate RED and GREEN commits._

## Files Created/Modified
- `contracts/nft/asset-progression/Cargo.toml` - workspace crate manifest for the dedicated progression contract.
- `contracts/nft/asset-progression/README.md` - contract boundary and authorization semantics for save-point snapshots.
- `contracts/nft/asset-progression/src/msg.rs` - instantiate, execute, query, and response types for progression persistence.
- `contracts/nft/asset-progression/src/state.rs` - world-scoped snapshot records keyed by collection, token, and world.
- `contracts/nft/asset-progression/src/helpers.rs` - live cw721 owner and approval queries plus token-kind validation.
- `contracts/nft/asset-progression/src/contract.rs` - instantiate, admin update, save snapshot, and query logic.
- `contracts/nft/asset-progression/src/tests.rs` - deterministic CosmWasm 2.x-safe tests for authorization and state transitions.
- `contracts/nft/asset-progression/examples/schema.rs` - crate-local schema exporter.
- `contracts/nft/asset-progression/schema/*.json` - generated public schema outputs for the new contract.
- `contracts/relationship/world-plugin-assignment/Cargo.toml` - removed unsupported `backtraces` feature that blocked workspace cargo resolution.

## Decisions Made
- Progression persistence stays world-specific and stores save-point snapshots only; gameplay formulas remain off-chain.
- Authorization reads the current cw721 owner or approval state from the target collection at write time.
- Schema generation is isolated to the contract crate to avoid overwriting unrelated workspace root schemas.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Removed unsupported `cosmwasm-std/backtraces` feature from `world-plugin-assignment`**
- **Found during:** Task 1 (RED verification)
- **Issue:** `cargo test -p asset-progression --lib` failed before compiling the new crate because `world-plugin-assignment` declared a non-existent workspace feature.
- **Fix:** Changed `backtraces` to an empty feature flag in `contracts/relationship/world-plugin-assignment/Cargo.toml`.
- **Files modified:** `contracts/relationship/world-plugin-assignment/Cargo.toml`
- **Verification:** `cargo test -p asset-progression --lib` progressed to compile and execute the new crate tests after the fix.
- **Committed in:** `a483cad` (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary workspace unblock only. No scope creep beyond enabling the planned crate verification.

## Issues Encountered
- Rust toolchain execution for schema generation and `cargo check` required running outside the sandbox on Windows.
- Root `schema/` artifacts were briefly overwritten by the example runner; the exporter was corrected to write to `contracts/nft/asset-progression/schema/` and the root schema directory was restored.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 11 now has a dedicated mutable progression surface that later composed-asset docs and plugin-assignment work can reference.
- `11-03` can follow the same pattern for plugin-to-world rights with live identity checks and crate-local schemas.

## Self-Check: PASSED

---
*Phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics*
*Completed: 2026-03-20*

