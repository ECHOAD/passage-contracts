---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: in_progress
stopped_at: Completed Phase 4 closeout; Phase 5 planning is next
last_updated: "2026-03-19T20:30:00Z"
last_activity: 2026-03-19 - Reverted validator metadata overreach and closed Phase 4 as chain-native staking only
progress:
  total_phases: 9
  completed_phases: 5
  total_plans: 23
  completed_plans: 15
  percent: 65
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-19)

**Core value:** PASG must provide real on-chain utility for payments, governance, staking, and creator monetization while keeping platform UX and infrastructure concerns off-chain.
**Current focus:** Phase 5 - Creator Asset Contracts & Monetization

## Current Position

Phase: 5 of 9 (Creator Asset Contracts & Monetization)
Plan: 0 of 3 in current phase
Status: Ready to plan
Last activity: 2026-03-19 - Reverted validator metadata overreach and closed Phase 4 as chain-native staking only

Progress: [#######---] 65%

## Performance Metrics

**Velocity:**
- Total plans completed: 14
- Average duration: 52 min
- Total execution time: 10.4 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Protocol Hardening & Boundaries | 3 | 170 min | 57 min |
| 2. PASG Utility Surface | 3 | 222 min | 74 min |
| 3. PASG Governance | 3 | 160 min | 53 min |
| 9. Marketplace-v3 Registration Redesign | 3 | 70 min | 23 min |

**Recent Trend:**
- Last 5 plans: 09-02, 09-03, 04-01, 04-02, 04-03
- Trend: Stable with docs-only closeout after reverting the validator metadata overreach

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
- [Debug 2026-03-18]: `multisig` must remain the admin-owner control plane; PASG governance must be designed as a separate voting/proposal layer instead of replacing that contract
- [Phase 3]: PASG governance now lives in `contracts/core/pasg-governance`, while `multisig` remains the owner-admin executor with typed handoff through `ratified_admin_action`
- [Phase 9]: `marketplace-v3` now enforces mandatory collection registration, a marketplace-global fee, collection-scoped denoms, and owner-request/admin-approval flows
- [Debug 2026-03-19]: Phase 4 PASG staking means chain-native delegation to Passage validators; existing `contracts/staking/*` remain NFT staking primitives, not the default PASG staking target
- [Phase 4]: 04-01 locked README and contract docs to the native validator delegation model and chain-owned unbonding boundary
- [Phase 04]: contracts/staking/* remain NFT staking primitives and are not PASG validator staking contracts
- [Phase 4]: validator discovery, rewards, and undelegation stay chain-native; no registry or governance validator metadata adapter remains in scope

### Roadmap Evolution

- Phase 8 added: Documentacion completa bilingue de contratos en docs
- Phase 9 added: marketplace-v3 registration, ownership validation, and admin approval redesign
- Phase 9 completed: marketplace-v3 registration redesign executed out of the mainline sequence

### Pending Todos

None yet.

### Blockers/Concerns

- Several active core contracts still have no migrate entrypoint, so future upgrades will require explicit redeploy-and-cutover planning.

## Session Continuity

Last session: 2026-03-19 20:30 UTC
Stopped at: Completed Phase 4 closeout; Phase 5 planning is next
Resume file: .planning/phases/04-pasg-staking-rewards/04-03-PLAN.md






