# Roadmap: Passage On-Chain Protocol Layer

## Overview

This roadmap starts from a brownfield CosmWasm workspace that already contains meaningful NFT, marketplace, routing, billing, admin-control, and NFT staking primitives. The journey is to harden those existing contracts, formalize PASG utility as the protocol center of gravity, add missing governance and staking mechanics, complete creator monetization and multi-economy settlement, and finish with audit-grade confidence.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions if brownfield realities require them later

- [x] **Phase 1: Protocol Hardening & Boundaries** - Close brownfield trust gaps and lock the on-chain/off-chain contract boundary.
- [x] **Phase 2: PASG Utility Surface** - Make PASG usage explicit, queryable, and enforceable across supported protocol flows.
- [x] **Phase 3: PASG Governance** - Replace admin-only control with PASG-scoped proposal and voting mechanics without repurposing the existing admin-owner multisig.
- [x] **Phase 4: PASG Validator Staking & Rewards** - Align PASG with native validator delegation, chain-level unbonding, validator fee participation, and token rewards.
- [x] **Phase 5: Creator Asset Contracts & Monetization** - Finish the typed NFT and revenue-bearing asset model around creators and worlds.
- [x] **Phase 6: Multi-Economy Settlement** - Let worlds run local economies that still settle against PASG without bypassing utility.
- [ ] **Phase 7: Audit Readiness & Launch Hardening** - Raise test coverage, fuzzing, docs, and migration discipline to audit grade.

## Phase Details

### Phase 1: Protocol Hardening & Boundaries
**Goal**: Remove the highest-risk brownfield defects and make the protocol boundary explicit before adding more PASG economics on top.
**Depends on**: Nothing (first phase)
**Requirements**: [ARCH-01, ARCH-02, ARCH-03, REV-03]
**Canonical refs**: [`.planning/codebase/CONCERNS.md`, `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`, `../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/nft/marketplace-v3/src/contract/query.rs`]
**Success Criteria** (what must be TRUE):
  1. Critical trust-boundary gaps in `streaming-billing` and other high-risk brownfield modules are closed or explicitly contained.
  2. The repo has a clear, enforced rule for what remains off-chain versus what the protocol guarantees on-chain.
  3. Every contract family in active scope has a documented pause, migrate, or cutover strategy.
**Plans**: 3 plans

Plans:
- [x] 01-01-PLAN.md - Harden streaming-billing authority, verified world ownership, and replay/bounds coverage
- [x] 01-02-PLAN.md - Correct marketplace/registry query semantics and factory reply-path behavior with tests
- [x] 01-03-PLAN.md - Publish the protocol boundary, safety matrix, and aligned streaming-billing operational docs

### Phase 2: PASG Utility Surface
**Goal**: Turn PASG from an assumed denom into an explicit protocol utility layer that contracts and services can integrate consistently.
**Depends on**: Phase 1
**Requirements**: [PASG-01, PASG-02, PASG-03]
**Canonical refs**: [`../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/split-router/src/msg.rs`, `README.md`]
**Success Criteria** (what must be TRUE):
  1. Supported flows can detect and apply PASG-specific fee treatment or utility policy on-chain.
  2. PASG integration no longer depends on scattered, contract-specific assumptions.
  3. Integrators have one unambiguous model for native `upasg` usage or any required adapter layer.
**Plans**: 3 plans

Plans:
- [x] 02-01: Define the PASG utility interface and native-denom or adapter strategy
- [x] 02-02: Integrate PASG fee-treatment hooks into target payment and routing flows
- [x] 02-03: Document and verify PASG utility semantics across contracts and service touchpoints

### Phase 3: PASG Governance
**Goal**: Deliver PASG-scoped governance for proposals, voting, delegation, quorum, and executable protocol changes as a separate layer around the existing admin-control primitives.
**Depends on**: Phase 2
**Requirements**: [GOV-01, GOV-02, GOV-03]
**Canonical refs**: [`contracts/core/multisig/src/msg.rs`, `contracts/core/multisig/src/contract.rs`, `docs/04-multisig-governance.md`, `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`]
**Success Criteria** (what must be TRUE):
  1. PASG holders can create, vote on, and execute protocol proposals on-chain.
  2. Voting weight, delegation, quorum, and approval rules are enforced by contract logic rather than process.
  3. Governance can adjust protocol parameters without overreaching into unrelated off-chain platform operations.
  4. `multisig` remains the stable admin-owner execution plane for `registry` and other protocol contracts; PASG governance is layered separately instead of replacing that contract's core role.
**Plans**: 3 plans

Plans:
- [x] 03-01-PLAN.md - Build the separate PASG governance contract and governance-owned parameter surface
- [x] 03-02-PLAN.md - Add deposited voting power, delegation, quorum, and lock enforcement
- [x] 03-03-PLAN.md - Add scoped multisig handoff, corrected docs, and governance guardrails

### Phase 4: PASG Validator Staking & Rewards
**Goal**: Align PASG staking with chain-native staking through native Passage validator delegation, chain-level unbonding, validator fee participation, and PASG rewards without inventing a duplicate CosmWasm staking vault by default.
**Depends on**: Phase 3
**Requirements**: [STAK-01, STAK-02, STAK-03]
**Canonical refs**: [`../context/whitepaper-tokenomics.html`, `.planning/phases/04-pasg-staking-rewards/04-INTENT-CORRECTION.md`, `.planning/phases/04-pasg-staking-rewards/04-RESEARCH.md`, `.planning/codebase/ARCHITECTURE.md`]
**Success Criteria** (what must be TRUE):
  1. PASG holders can delegate to one or more Passage validators, initiate undelegation, and complete exit after the chain-level unbonding window, including an expected 21-day wait when that remains the chain rule.
  2. Staking rewards reflect validator fee participation and PASG token rewards from the chain/token program rather than a default contract-local emission vault.
  3. Delegations, validator assignments, undelegation state, and reward surfaces can be queried and verified through chain-native or explicitly documented adapter surfaces without inventing a duplicate staking ledger.
**Plans**: 3 plans

Plans:
- [x] 04-01: Define the native validator delegation model, validator selection rules, and required chain interfaces
- [x] 04-02: Align reward, fee-participation, and query surfaces with the chain staking flow
- [x] 04-03: Document migration, dependencies, and the explicit no-adapter closeout for the native model

### Phase 5: Creator Asset Contracts & Monetization
**Goal**: Complete the asset-contract model around creators, collections, worlds, plugins, and monetization-bearing NFT types.
**Depends on**: Phase 4
**Requirements**: [NFT-01, NFT-02, NFT-03, REV-01]
**Canonical refs**: [`../context/product/architecture/NFT_COLLECTION_STRATEGY.md`, `../context/product/architecture/IMPLEMENTATION_REVIEW.md`, `../context/product/architecture/NFT_METADATA_DESIGN.md`, `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md`, `contracts/nft/pg721/src/msg.rs`]
**Success Criteria** (what must be TRUE):
  1. Creators can instantiate correctly typed collections with the required metadata and royalty behavior.
  2. Missing or drifting NFT-type extensions are resolved in code and documentation.
  3. Revenue-bearing asset flows route creator, collaborator, and platform shares from on-chain rules.
**Plans**: 3 plans

Plans:
- [x] 05-01: Rebuild ecosystem-centric registry affiliation and direct collection lifecycle
- [x] 05-02: Complete typed creator asset metadata and keep monetization routing generic
- [x] 05-03: Align docs, examples, and verification to the ecosystem-centric creator asset model

### Phase 6: Multi-Economy Settlement
**Goal**: Let worlds run local economies that settle against PASG and support hybrid payment paths safely.
**Depends on**: Phase 5
**Requirements**: [ECON-01, ECON-02, REV-02]
**Canonical refs**: [`../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`, `contracts/core/split-router/src/contract.rs`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/registry/src/msg.rs`]
**Success Criteria** (what must be TRUE):
  1. A world operator can define a local token or points model that still settles through PASG-aware protocol rules.
  2. Hybrid payment paths can be queried and enforced without bypassing PASG utility.
  3. Session-based, escrow, and refund-safe settlement rules exist where the product architecture requires them.
**Plans**: 3 plans

Plans:
- [x] 06-01: Define the multi-economy contract interfaces and settlement primitives
- [x] 06-02: Implement hybrid payment routing, escrow, and refund-safe execution
- [x] 06-03: Integrate world-economy settlement with existing registry and routing surfaces

### Phase 7: Audit Readiness & Launch Hardening
**Goal**: Bring the in-scope protocol to audit-ready quality with aligned docs, coverage, fuzzing, and migration confidence.
**Depends on**: Phase 6
**Requirements**: [QUAL-01, QUAL-02, QUAL-03]
**Canonical refs**: [`.planning/codebase/TESTING.md`, `.planning/codebase/CONCERNS.md`, `contracts/core/registry/src/tests/mod.rs`, `contracts/core/multisig/src/tests/governance.rs`]
**Success Criteria** (what must be TRUE):
  1. Critical economic paths have deterministic tests for authorization, accounting, and state transitions.
  2. Coverage and fuzz/invariant suites meet the program quality bar for in-scope contracts.
  3. Audit-facing docs, schema outputs, and migration guides match real behavior and upgrade paths.
**Plans**: 2 plans

Plans:
- [ ] 07-01: Expand deterministic, fuzz, and invariant testing across economic flows
- [ ] 07-02: Assemble audit-ready documentation, migration guides, and verification artifacts

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Protocol Hardening & Boundaries | 3/3 | Complete | 2026-03-18 |
| 2. PASG Utility Surface | 3/3 | Complete | 2026-03-18 |
| 3. PASG Governance | 3/3 | Complete | 2026-03-19 |
| 4. PASG Validator Staking & Rewards | 3/3 | Complete | 2026-03-19 |
| 5. Creator Asset Contracts & Monetization | 3/3 | Complete | 2026-03-19 |
| 6. Multi-Economy Settlement | 3/3 | Complete | 2026-03-19 |
| 7. Audit Readiness & Launch Hardening | 0/2 | Not started | - |
| 8. Documentacion completa bilingue de contratos en docs | 4/4 | Complete | 2026-03-19 |
| 9. marketplace-v3 registration, ownership validation, and admin approval redesign | 3/3 | Complete | 2026-03-19 |



### Phase 8: Documentacion completa bilingue de contratos en docs

**Goal:** Publish complete bilingual documentation for all workspace contracts, including per-contract instantiate and interaction guidance, cross-contract relationship explanations, and hypothetical business/technical flows in English and Spanish.
**Requirements**: [DOC-01, DOC-02, DOC-03]
**Depends on:** Phase 7
**Canonical refs**: [`docs/README.md`, `.planning/phases/08-documentacion-completa-bilingue-de-contratos-en-docs/08-CONTEXT.md`, `.planning/codebase/STRUCTURE.md`, `.planning/codebase/INTEGRATIONS.md`, `contracts/core/`, `contracts/nft/`, `contracts/relationship/`, `contracts/staking/`]
**Success Criteria** (what must be TRUE):
  1. Every contract crate in the workspace has a mirrored English and Spanish documentation page.
  2. The docs explain instantiation, actors, key messages, dependencies, and cross-contract relationships without inventing unsupported protocol behavior.
  3. Hypothetical business and technical examples make the major protocol flows understandable end to end.
  4. Explicit legacy contracts are documented as historical/reference material rather than recommended default integration targets.
**Plans**: 4 plans

Plans:
- [x] 08-01: Build the bilingual documentation scaffold, indexes, relationship map, instantiate guides, and flow guides
- [x] 08-02: Document the core contract family bilingually with one file per contract
- [x] 08-03: Document the NFT contract family bilingually with one file per contract
- [x] 08-04: Document relationship, staking, and explicit legacy contracts bilingually and close tracking artifacts


### Phase 9: marketplace-v3 registration, ownership validation, and admin approval redesign

**Goal:** Remove the confusing mixed registration/config model from `marketplace-v3` and replace it with owner-validated requests plus admin-controlled approvals.
**Requirements**: [QUAL-01]
**Depends on:** Phase 8
**Plans:** 3 plans

Plans:
- [x] 09-01-PLAN.md - Simplify marketplace config to global fee plus collection-scoped denom with mandatory registration
- [x] 09-02-PLAN.md - Add owner validation plus request/approval flows for register and update
- [x] 09-03-PLAN.md - Align queries, schema, and docs to the redesigned public surface












### Phase 10: remove native_assets from metadata-onchain and updatable nft surfaces

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 9
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 10 to break down)
