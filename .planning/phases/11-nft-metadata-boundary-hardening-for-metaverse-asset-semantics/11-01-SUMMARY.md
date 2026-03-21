---
phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics
plan: 01
subsystem: nft
tags: [cosmwasm, cw721, nft-metadata, pg721, minter-v2]
requires:
  - phase: 05-creator-asset-contracts-monetization
    provides: typed creator asset nft surfaces and collection nft_type enforcement
  - phase: 10-native-assets removal
    provides: prior active metadata boundary cleanup for unsupported attachment surfaces
provides:
  - standardized Passage `profile_id` validation for avatar and companion metadata
  - active pg721 metadata structs limited to durable protocol semantics
  - manifest-pointer guidance for runtime-heavy NFT details
affects: [phase-11-plan-02, phase-11-plan-04, nft-runtime-manifests, schema-docs]
tech-stack:
  added: []
  patterns: [mint-time profile_id validation, token_uri-first manifest boundary]
key-files:
  created: [.planning/phases/11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics/11-01-SUMMARY.md]
  modified:
    [
      contracts/nft/pg721/src/msg.rs,
      contracts/nft/pg721/src/contract.rs,
      contracts/nft/pg721/src/error.rs,
      contracts/nft/pg721-updatable/src/msg.rs,
      contracts/nft/pg721-updatable/src/contract.rs,
      contracts/nft/pg721-updatable/src/error.rs,
      contracts/nft/pg721/src/contract/tests.rs,
      contracts/nft/pg721-updatable/src/contract/tests.rs,
      contracts/nft/pg721-metadata-onchain/src/msg.rs,
      contracts/nft/minter-v2/src/msg.rs
    ]
key-decisions:
  - "Avatar and companion collections must expose a standardized Passage profile_id at mint time instead of arbitrary compatibility metadata."
  - "Mutable gameplay state and render/runtime detail remain behind token_uri manifests rather than active typed NFT extensions."
patterns-established:
  - "Pattern 1: active pg721 metadata keeps only durable protocol semantics, with manifest-level concerns documented inline."
  - "Pattern 2: pg721-updatable preserves token_uri-only mutability and rejects profile-less avatar or companion mints."
requirements-completed: [NFT-02, NFT-03, REV-01, ARCH-01]
duration: 5min
completed: 2026-03-20
---

# Phase 11 Plan 01: NFT metadata boundary hardening summary

**Standardized avatar and companion profile markers with token_uri-first manifest boundaries across the active pg721 NFT surfaces**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-20T16:52:33-04:00
- **Completed:** 2026-03-20T16:57:44.6101231-04:00
- **Tasks:** 1
- **Files modified:** 10

## Accomplishments

- Removed mutable progression fields and other manifest-only runtime details from the active `pg721` and `pg721-updatable` typed metadata structs.
- Added mint-time validation that requires standardized Passage `profile_id` markers for avatar and companion collections.
- Preserved the updatable contract's token-uri-only mutation model and documented the manifest boundary in `pg721-metadata-onchain` and `minter-v2`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Reclassify active NFT metadata fields into durable on-chain semantics only** - `748f74e` (`test`)
2. **Task 1: Reclassify active NFT metadata fields into durable on-chain semantics only** - `46e25f7` (`feat`)

## Files Created/Modified

- `.planning/phases/11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics/11-01-SUMMARY.md` - execution summary and verification record
- `contracts/nft/pg721/src/msg.rs` - tightened active metadata structs and added `PassageProfileId`
- `contracts/nft/pg721/src/contract.rs` - added mint-time validation for required standardized profile markers
- `contracts/nft/pg721/src/error.rs` - added explicit profile validation errors
- `contracts/nft/pg721-updatable/src/msg.rs` - mirrored the tightened metadata surface while keeping `UpdateTokenMetadata` token-uri-only
- `contracts/nft/pg721-updatable/src/contract.rs` - enforced standardized profile markers without allowing typed extension mutation
- `contracts/nft/pg721/src/contract/tests.rs` - added red/green coverage for profile validation and boundary cleanup
- `contracts/nft/pg721-updatable/src/contract/tests.rs` - added red/green coverage for profile validation and token-uri-only metadata updates
- `contracts/nft/pg721-metadata-onchain/src/msg.rs` - clarified that rich on-chain metadata is non-default for metaverse assets
- `contracts/nft/minter-v2/src/msg.rs` - clarified that `base_token_uri` points to manifests, not active runtime-heavy metadata

## Decisions Made

- `profile_id` is enforced as a constrained Passage interoperability constant for avatar and companion types, not a freeform metadata field.
- `Emote`, `Scene`, and `AccessPass` stay out of the active typed metadata taxonomy and are documented as manifest-level or dedicated-surface concerns.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- An earlier sandboxed `cargo test -p pg721 --lib` attempt failed because the sandbox could not execute the user rust toolchain path. Final verification completed with the provided passing test results plus a local `cargo check -p pg721 -p pg721-updatable -p pg721-metadata-onchain`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The active pg721 metadata boundary is now aligned with token-uri manifests, so Phase 11 can add dedicated state surfaces without re-opening generic NFT metadata.
- Schema and docs alignment work can now build on a stable `profile_id` convention.

---
*Phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics*
*Completed: 2026-03-20*
