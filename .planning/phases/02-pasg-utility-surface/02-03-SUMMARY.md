---
phase: 02-pasg-utility-surface
plan: 03
subsystem: docs-and-schema
tags: [pasg, upasg, docs, schema, streaming-billing, split-router, marketplace-v3, minter-v2, auction-english]
requires:
  - phase: 02-pasg-utility-surface
    provides: canonical PASG utility semantics, generic routing compatibility, and PASG response attributes from 02-01 and 02-02
provides:
  - aligned PASG-native documentation across billing, routing, marketplace, minter, and auction docs
  - concrete JSON examples for PASG queries, settlement paths, and execute-response verification
  - refreshed schema artifacts for split-router and checked-in schema outputs for minter-v2
affects: [streaming-billing, split-router, marketplace-v3, minter-v2, auction-english, docs]
tech-stack:
  added: []
  patterns: [docs-as-contract, query-first verification, schema-backed integration examples]
key-files:
  created:
    - .planning/phases/02-pasg-utility-surface/02-03-SUMMARY.md
    - .planning/phases/02-pasg-utility-surface/02-VERIFICATION.md
    - contracts/nft/minter-v2/schema/instantiate_msg.json
    - contracts/nft/minter-v2/schema/execute_msg.json
    - contracts/nft/minter-v2/schema/query_msg.json
    - contracts/nft/minter-v2/schema/migrate_msg.json
  modified:
    - contracts/core/split-router/README.md
    - contracts/nft/marketplace-v3/README.md
    - contracts/nft/minter-v2/README.md
    - contracts/nft/auction-english/README.md
    - docs/03-json-examples.md
    - contracts/core/split-router/schema/execute_msg.json
    - contracts/core/split-router/schema/query_msg.json
key-decisions:
  - "repo docs now describe PASG as native-upasg-first and point integrators back to streaming-billing for canonical PASG semantics"
  - "split-router documentation stays generic and uses RoutingMetadata plus PreviewSplit as the inspectable routing surface"
  - "minter-v2 now commits generated schema outputs so downstream consumers can inspect mint query and execute payloads without rerunning schema locally"
patterns-established:
  - "Query-first verification: integrators inspect PasgUtility, RoutingMetadata, CollectionDenom/CollectionFee, MintPrice, and Auction/Config before sending funds"
  - "Response-attribute verification: fee-bearing marketplace, mint, and auction flows publish canonical PASG reference attributes after execution"
requirements-completed: [PASG-01, PASG-02, PASG-03]
duration: 15 min
completed: 2026-03-18
---

# Phase 02 Plan 03: PASG Docs, Examples, and Schema Summary

**Phase 2 now has one human-readable and machine-readable PASG story centered on native `upasg`, canonical `streaming-billing` queries, and explicit consumer verification paths.**

## Performance

- **Duration:** 15 min
- **Started:** 2026-03-19T00:29:16Z
- **Completed:** 2026-03-19T00:43:57Z
- **Tasks:** 3
- **Files modified:** 9
- **Files created:** 6

## Accomplishments
- Rewrote the contract-facing READMEs for `split-router`, `marketplace-v3`, `minter-v2`, and `auction-english` so they describe the current native-coin settlement model instead of stale revenue-router fields or generic placeholders.
- Replaced the Phase 2 portion of `docs/03-json-examples.md` with concrete PASG examples for `streaming-billing`, `split-router`, `marketplace-v3`, `auction-english`, and `minter-v2`, including the exact query and execute names integrators need.
- Regenerated `split-router` schema to expose `routing_metadata` and `route_world_revenue`, and checked in the generated `minter-v2/schema` directory so downstream tooling can inspect the current payload surface directly.
- Completed a phase-level wording audit and package-level verification pass for the Phase 2 PASG surfaces.

## Task Commits

Each task was committed atomically:

1. **Task 1: Align PASG contract narratives** - `bf8e099` (docs)
2. **Task 2: Publish PASG examples and schema surfaces** - `db415c8` (docs)

**Plan metadata and phase verification:** captured in the final completion commit.

## Files Created/Modified
- `contracts/core/split-router/README.md` - Reframed the router as a generic native-fund splitter and documented `RoutingMetadata` plus `RouteWorldRevenue`.
- `contracts/nft/marketplace-v3/README.md` - Replaced stale revenue-router guidance with current denom, fee, preview, and PASG attribute verification paths.
- `contracts/nft/minter-v2/README.md` - Documented the actual mint, withdraw, and query surfaces with native `upasg` PASG guidance.
- `contracts/nft/auction-english/README.md` - Added real instantiate, query, settlement, and royalty-routing guidance.
- `docs/03-json-examples.md` - Added canonical PASG examples for `streaming-billing` and updated all downstream Phase 2 examples to current message shapes.
- `contracts/core/split-router/schema/execute_msg.json` - Now includes `route_world_revenue`.
- `contracts/core/split-router/schema/query_msg.json` - Now includes `routing_metadata`.
- `contracts/nft/minter-v2/schema/*.json` - New generated schema outputs for minter instantiate, execute, query, and migrate messages.
- `.planning/phases/02-pasg-utility-surface/02-VERIFICATION.md` - Records Phase 2 goal verification and the remaining out-of-scope workspace blockers.

## Issues Encountered
- `cargo unit-test` initially failed inside the sandbox with `Access is denied (os error 5)` while spawning `rustc`. An escalated rerun confirmed the real repo-level blocker is still `ecosystem-factory`, not the PASG phase files.
- `cargo unit-test` still fails in `contracts/core/ecosystem-factory` because `REQUESTS_BY_CREATOR` is missing from `src/contract/execute.rs:188` and `src/contract/query.rs:84`.
- `cargo check --workspace` still fails in the same `ecosystem-factory` locations and also in `contracts/nft/minter-metadata-onchain/src/contract.rs:102`, where `Pg721InstantiateMsg` initialization is missing `nft_type`.
- Those blockers remain outside Phase 2 ownership and were already logged in `deferred-items.md`.

## Verification
- `rg -n "native \`upasg\`|canonical PASG|adapter|compatibility-only|service-invoked|fee treatment|query" README.md CLAUDE.md docs/03-json-examples.md contracts/core/streaming-billing/README.md contracts/core/split-router/README.md contracts/nft/marketplace-v3/README.md contracts/nft/minter-v2/README.md contracts/nft/auction-english/README.md`
- `cargo test -p split-router --lib`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p minter-v2 --lib --tests`
- `cargo test -p auction-english --lib`
- `cargo check -p streaming-billing`
- `cargo unit-test` (fails outside Phase 2 scope in `ecosystem-factory`)
- `cargo check --workspace` (fails outside Phase 2 scope in `ecosystem-factory` and `minter-metadata-onchain`)

## Next Phase Readiness
- Phase 2 is complete. The PASG utility layer is now explicit in code, docs, examples, and schema outputs.
- Phase 3 can start planning PASG-scoped governance on top of a documented native `upasg` foundation.
- Workspace-wide green verification still depends on fixing the previously logged `ecosystem-factory` and `minter-metadata-onchain` compile drift.

## Self-Check: PASSED
- Found `.planning/phases/02-pasg-utility-surface/02-03-SUMMARY.md` on disk.
- Verified task commits `bf8e099` and `db415c8` exist in git history.
- Confirmed Phase 2 docs now point integrators to concrete PASG queries before settlement.
