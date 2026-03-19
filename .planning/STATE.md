---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: in_progress
stopped_at: Completed 02-03-PLAN.md
last_updated: "2026-03-19T00:43:57Z"
last_activity: 2026-03-18 - Phase 2 completed; PASG utility docs, examples, and schema surfaces verified
progress:
  total_phases: 7
  completed_phases: 2
  total_plans: 20
  completed_plans: 6
  percent: 30
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-17)

**Core value:** PASG must provide real on-chain utility for payments, governance, staking, and creator monetization while keeping platform UX and infrastructure concerns off-chain.
**Current focus:** Phase 3 - PASG Governance

## Current Position

Phase: 3 of 7 (PASG Governance)
Plan: 0 of 3 in current phase
Status: Ready to plan
Last activity: 2026-03-18 - Phase 2 completed; PASG utility docs, examples, and schema surfaces verified

Progress: [###-------] 30%

## Performance Metrics

**Velocity:**
- Total plans completed: 6
- Average duration: 65 min
- Total execution time: 6.5 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Protocol Hardening & Boundaries | 3 | 170 min | 57 min |
| 2. PASG Utility Surface | 3 | 222 min | 74 min |

**Recent Trend:**
- Last 5 plans: 01-02, 01-03, 02-01, 02-02, 02-03
- Trend: Stable

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Init]: Treat the repo as brownfield hardening plus completion, not a greenfield rewrite
- [Init]: PASG utility is the priority driver when sequencing tradeoffs appear
- [Init]: Governance is scoped to PASG/protocol parameters, not all off-chain platform operations
- [Phase 1]: Streaming billing remains service-invoked and now requires explicit backend-operator authority
- [Phase 1]: Registry plus cw721 ownership is the canonical source of truth for world billing ownership
- [Phase 1]: Safety planning documents the current pause, migrate, and cutover reality rather than inventing upgrade paths
- [Phase 2]: streaming-billing owns the canonical PASG utility interface via `QueryMsg::PasgUtility`
- [Phase 2]: `upasg` remains the only in-repo PASG settlement model; wrappers are compatibility-only shims
- [Phase 2]: split-router stays denom-agnostic while billing and subscription policy remain off-chain
- [Phase 2]: split-router accepts streaming-billing `route_world_revenue` calls via a generic compatibility alias
- [Phase 2]: marketplace, minter-v2, and auction-english now emit canonical PASG utility reference attributes on fee-bearing flows
- [Phase 2]: contract docs and JSON examples now require query-first PASG verification instead of contract-local assumptions
- [Phase 2]: minter-v2 schema outputs are checked in and split-router schema now exposes inspectable routing metadata

### Pending Todos

None yet.

### Blockers/Concerns

- `cargo unit-test` and `cargo check --workspace` remain blocked by `ecosystem-factory`: missing `REQUESTS_BY_CREATOR` imports in `src/contract/execute.rs:188` and `src/contract/query.rs:84`.
- `cargo check --workspace` remains additionally blocked by `minter-metadata-onchain`: `Pg721InstantiateMsg` initialization is missing `nft_type` in `src/contract.rs:102`.
- Several active core contracts still have no migrate entrypoint, so future upgrades will require explicit redeploy-and-cutover planning.

## Session Continuity

Last session: 2026-03-19 00:43 UTC
Stopped at: Completed Phase 2 PASG Utility Surface
Resume file: .planning/ROADMAP.md
