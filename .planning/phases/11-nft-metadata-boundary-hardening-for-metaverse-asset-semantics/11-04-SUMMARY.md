---
phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics
plan: 04
subsystem: docs
tags: [schemas, docs, metadata-boundary, progression, plugin-assignment]
requires:
  - phase: 11-01
    provides: tightened pg721 and pg721-updatable metadata semantics
  - phase: 11-02
    provides: dedicated progression contract surface and schema entrypoint
  - phase: 11-03
    provides: dedicated plugin assignment contract surface and schema entrypoint
provides:
  - aligned English contract docs for the corrected metadata boundary
  - refreshed method reference coverage for asset-progression and world-plugin-assignment
  - schema regeneration verification for active Phase 11 contract surfaces
affects: [11-05, docs, nft-metadata-boundary, composed-assets]
tech-stack:
  added: []
  patterns: [schema regeneration, contract-surface documentation, boundary documentation]
key-files:
  modified: [docs/02-method-reference.md, docs/en/contracts/nft/pg721.md, docs/en/contracts/nft/pg721-updatable.md, docs/en/contracts/nft/avatar-progression.md, docs/en/contracts/relationship/world-plugin-assignment.md]
key-decisions:
  - "Public docs now state explicitly that manifests behind token_uri carry runtime and render detail while mutable protocol state moves to dedicated surfaces."
  - "The progression surface remains documented under the public avatar-progression name even though the backing crate is asset-progression."
  - "Wave verification uses the real asset-progression crate name to avoid stale validation drift."
patterns-established:
  - "Pattern 1: contract docs describe protocol semantics and dedicated state boundaries without implying gameplay state lives in generic NFT metadata."
  - "Pattern 2: schema/doc closeout can verify public surfaces even when historical plan text still uses an older public label."
requirements-completed: [NFT-02, NFT-03]
duration: 26 min
completed: 2026-03-20
---

# Phase 11 Plan 04: Schema and contract-surface alignment Summary

**English contract docs, method references, and schema verification now match the hardened NFT metadata boundary and the new dedicated state surfaces.**

## Performance

- **Duration:** 26 min
- **Tasks:** 1
- **Files modified:** 5

## Accomplishments
- Updated the public method reference to describe `token_uri` manifests, standardized `profile_id`, `asset-progression`, and `world-plugin-assignment` as the active Phase 11 boundary.
- Aligned the English contract docs for `pg721`, `pg721-updatable`, `avatar-progression`, and `world-plugin-assignment` with the corrected semantics.
- Re-ran schema examples for `pg721`, `pg721-updatable`, `asset-progression`, and `world-plugin-assignment` as the plan-level verification pass.

## Task Commits

Implementation commit already existed before administrative closeout:

1. **Task 1: Regenerate schemas and update contract-reference docs** - `b4387ce`

## Files Created/Modified
- `docs/02-method-reference.md` - boundary rules, progression save-point semantics, and durable plugin assignment coverage.
- `docs/en/contracts/nft/pg721.md` - clarified manifest-vs-metadata boundary and standardized `profile_id`.
- `docs/en/contracts/nft/pg721-updatable.md` - clarified that mutable protocol state lives in dedicated surfaces, not token metadata updates.
- `docs/en/contracts/nft/avatar-progression.md` - documented the dedicated progression surface backed by the `asset-progression` crate.
- `docs/en/contracts/relationship/world-plugin-assignment.md` - documented the durable plugin-to-world rights surface.

## Decisions Made
- Runtime/render payloads remain described as manifest content behind `token_uri`.
- Generic NFT metadata no longer advertises mutable progression semantics.
- Public docs preserve the user-facing `avatar-progression` contract label while referencing the real `asset-progression` source of truth.

## Verification
- `cargo run --example schema -p pg721`
- `cargo run --example schema -p pg721-updatable`
- `cargo run --example schema -p asset-progression`
- `cargo run --example schema -p world-plugin-assignment`
- `rg -n "profile_id|manifest|assignment|save point|progression" docs/en/contracts docs/02-method-reference.md`
- `rg -n "level|experience" docs/en/contracts docs/02-method-reference.md` returned no matches, confirming stale generic-NFT progression claims were removed.

## Issues Encountered
- The historical plan and validation docs still used `avatar-progression` in a few verification strings even though the actual crate name is `asset-progression`; this closeout corrects the validation artifact.
- Example schema runners still write into the workspace schema directory for some crates, so verification was treated as a smoke pass rather than a file-inventory audit.

## Next Phase Readiness
- `11-05` can now point to stable English contract docs and method references when publishing the bilingual composed-asset guides.

## Self-Check: PASSED

---
*Phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics*
*Completed: 2026-03-20*