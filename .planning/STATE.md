---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: execution_complete
stopped_at: Phase 10 added; Phase 7 remains current mainline
last_updated: "2026-03-20T03:25:00.000Z"
last_activity: 2026-03-19 - Added Phase 10 for removing native_assets from metadata-onchain and updatable NFT surfaces while Phase 7 remains the current mainline focus
progress:
  total_phases: 10
  completed_phases: 8
  total_plans: 27
  completed_plans: 25
  percent: 80
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-19)

**Core value:** PASG must provide real on-chain utility for payments, governance, staking, and creator monetization while keeping platform UX and infrastructure concerns off-chain.
**Current focus:** Phase 7 - Audit Readiness & Launch Hardening

## Current Position

Phase: 7 of 10 (Audit Readiness & Launch Hardening)
Plan: 0 of 2 in current phase
Status: Phase 10 added out of sequence; Phase 7 not started
Last activity: 2026-03-19 - Added Phase 10 for removing native_assets from metadata-onchain and updatable NFT surfaces while Phase 7 remains the current mainline focus

Progress: [########--] 80%

## Performance Metrics

**Velocity:**
- Total plans completed: 25
- Average duration: 52 min
- Total execution time: 10.4 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Protocol Hardening & Boundaries | 3 | 170 min | 57 min |
| 2. PASG Utility Surface | 3 | 222 min | 74 min |
| 3. PASG Governance | 3 | 160 min | 53 min |
| 8. Bilingual Contract Documentation | 4 | docs-heavy | n/a |
| 9. Marketplace-v3 Registration Redesign | 3 | 70 min | 23 min |

**Recent Trend:**
- Last 5 plans: 08-01, 08-02, 08-03, 08-04, 09-03
- Trend: Stable documentation expansion with mirrored bilingual coverage added out of mainline sequence

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
- [Phase 2]: streaming-billing owns the canonical PASG utility interface via QueryMsg::PasgUtility
- [Phase 2]: upasg remains the only in-repo PASG settlement model; wrappers are compatibility-only shims
- [Phase 2]: split-router stays denom-agnostic while billing and subscription policy remain off-chain
- [Phase 2]: split-router accepts streaming-billing route_world_revenue calls via a generic compatibility alias
- [Phase 2]: marketplace, minter-v2, and auction-english now emit canonical PASG utility reference attributes on fee-bearing flows
- [Phase 2]: contract docs and JSON examples now require query-first PASG verification instead of contract-local assumptions
- [Phase 2]: minter-v2 schema outputs are checked in and split-router schema now exposes inspectable routing metadata
- [Debug 2026-03-18]: multisig must remain the admin-owner control plane; PASG governance must be designed as a separate voting/proposal layer instead of replacing that contract
- [Phase 3]: PASG governance now lives in contracts/core/pasg-governance, while multisig remains the owner-admin executor with typed handoff through ratified_admin_action
- [Phase 9]: marketplace-v3 now enforces mandatory collection registration, a marketplace-global fee, collection-scoped denoms, and owner-request/admin-approval flows
- [Debug 2026-03-19]: Phase 4 PASG staking means chain-native delegation to Passage validators; existing contracts/staking/* remain NFT staking primitives, not the default PASG staking target
- [Phase 4]: validator discovery, rewards, and undelegation stay chain-native; no registry or governance validator metadata adapter remains in scope
- [Phase 5]: registry now owns mutable collection affiliation with deregister and re-home, while pg721 and pg721-updatable expose the complete typed creator asset surface
- [Phase 6]: streaming-billing now exposes bounded WorldLocalEconomy and PreviewWorldSettlement queries while local economies stay auxiliary to collection-level creator monetization
- [Phase 8]: docs/es and docs/en now provide mirrored one-file-per-contract documentation plus shared relationship, instantiate, and flow guides; explicit legacy contracts are documented as historical/reference material

### Roadmap Evolution

- Phase 8 added: Documentacion completa bilingue de contratos en docs
- Phase 8 completed out of sequence while Phase 7 remains pending
- Phase 9 added: marketplace-v3 registration, ownership validation, and admin approval redesign
- Phase 9 completed: marketplace-v3 registration redesign executed out of the mainline sequence
- Phase 10 added: remove native_assets from metadata-onchain and updatable nft surfaces

### Pending Todos

None yet.

### Blockers/Concerns

- Several active core contracts still have no migrate entrypoint, so future upgrades will require explicit redeploy-and-cutover planning.
- Phase 7 remains the current mainline gate before the milestone is truly audit-ready.

## Session Continuity

Last session: 2026-03-19T21:13:35.839Z
Stopped at: Phase 10 added; Phase 7 remains current mainline
Resume file: .planning/ROADMAP.md

