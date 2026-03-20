# Requirements: Passage On-Chain Protocol Layer

**Defined:** 2026-03-17
**Core Value:** PASG must provide real on-chain utility for payments, governance, staking, and creator monetization while keeping platform UX and infrastructure concerns off-chain.

## v1 Requirements

### Protocol Boundary & Safety

- [x] **ARCH-01**: Protocol contracts keep ownership, payments, governance, and economic policy on-chain while leaving streaming infrastructure, rendering, search, analytics, and similar UX systems off-chain.
- [x] **ARCH-02**: Critical brownfield auth, query, and reply-path defects are fixed before new PASG economics depend on those contracts.
- [x] **ARCH-03**: Every long-lived contract in scope has an explicit pause, migration, or deploy-and-cutover strategy that preserves active economic flows.

### PASG Utility

- [x] **PASG-01**: User can pay supported protocol-facing fees with PASG and receive on-chain-verifiable fee treatment where configured.
- [x] **PASG-02**: Protocol contracts expose a canonical PASG utility interface instead of duplicating fee or discount logic contract by contract.
- [x] **PASG-03**: PASG utility works with the existing `upasg` settlement model or a documented wrapper/adapter path, with no ambiguity for integrators.

### Governance

- [x] **GOV-01**: PASG holder can create protocol proposals, vote on them, and execute passed actions on-chain.
- [x] **GOV-02**: Governance enforces token-weighted voting, delegation, quorum, and majority rules on-chain.
- [x] **GOV-03**: Governance is limited to PASG and protocol parameters and cannot directly control general off-chain platform operations.

### Staking

- [x] **STAK-01**: PASG holder can delegate and undelegate stake to one or more Passage validators through the chain-native staking flow, with the expected 21-day unbonding period when that remains the chain-level staking rule.
- [x] **STAK-02**: Delegated PASG participates in validator fee sharing and PASG token rewards defined by the chain/token program, without assuming a separate default CosmWasm emission vault.
- [x] **STAK-03**: Delegation balances, validator assignments, undelegation state, and reward-state transitions are queryable and testable through chain-native or documented adapter surfaces without manual bookkeeping.

### NFT Assets & Creator Monetization

- [x] **NFT-01**: Creator can create typed Passage NFT collections through factories with correct collection metadata, royalties, and registration behavior.
- [x] **NFT-02**: Required world, plugin, achievement, avatar, companion, and component extensions are implemented or explicitly retired in docs and code with no silent drift.
- [x] **NFT-03**: Cross-world usage and monetization rules for Passage assets are enforced through on-chain primitives plus documented off-chain coordination.

### Revenue Execution

- [x] **REV-01**: Platform services can execute creator/platform/partner revenue splits for world, marketplace, and asset flows from on-chain rules.
- [x] **REV-02**: Revenue execution supports session-based billing, escrow, and refund-safe settlement where the product architecture requires it.
- [x] **REV-03**: Fiat-assisted purchase flows into PASG-denominated accounting are protected by explicit oracle, replay, and freshness controls.

### Multi-Economy

- [x] **ECON-01**: World operators can define local token or points economies that settle against PASG through a standard contract interface.
- [x] **ECON-02**: Hybrid payment paths that combine PASG with world-specific units are queryable, enforceable, and do not bypass PASG utility.

### Quality & Audit

- [ ] **QUAL-01**: Critical economic contracts have deterministic tests for authorization, accounting, pagination/filter semantics, and reply flows.
- [ ] **QUAL-02**: In-scope contracts reach >95% branch coverage with fuzzing or invariants on economic logic.
- [ ] **QUAL-03**: Audit-ready documentation, migration guides, and schema outputs match actual contract behavior.

### Documentation

- [x] **DOC-01**: Every workspace contract has mirrored English and Spanish documentation that explains purpose, instantiation, key messages, actors, permissions, and dependencies.
- [x] **DOC-02**: Documentation includes cross-contract relationship guides and hypothetical business/technical examples that explain how the protocol works end to end.
- [x] **DOC-03**: Explicitly legacy or historical contracts are documented as reference material and clearly separated from the recommended current integration path.

## v2 Requirements

None currently. The active program intentionally keeps all eight stakeholder-requested deliverable areas in scope for this initialization cycle.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Pixel streaming GPU orchestration, signaling servers, and event backend UX | Product architecture explicitly places these off-chain |
| Search, discovery, recommendations, and analytics | High-throughput and low-latency systems do not belong in protocol contracts |
| Avatar rendering, real-time equipment composition, and Unreal runtime logic | Ownership and hashes may be verified on-chain, but rendering remains off-chain |
| PASG governance over general Passage business operations | Governance in this project is scoped to PASG and protocol parameters only |
| Embedding subscriptions, billing tiers, or broad platform business rules inside PASG token contracts | Violates the architectural boundary between protocol primitives and platform services |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ARCH-01 | Phase 1 | Complete |
| ARCH-02 | Phase 1 | Complete |
| ARCH-03 | Phase 1 | Complete |
| REV-03 | Phase 1 | Complete |
| PASG-01 | Phase 2 | Complete |
| PASG-02 | Phase 2 | Complete |
| PASG-03 | Phase 2 | Complete |
| GOV-01 | Phase 3 | Complete |
| GOV-02 | Phase 3 | Complete |
| GOV-03 | Phase 3 | Complete |
| STAK-01 | Phase 4 | Complete |
| STAK-02 | Phase 4 | Complete |
| STAK-03 | Phase 4 | Complete |
| NFT-01 | Phase 5 | Complete |
| NFT-02 | Phase 5 | Complete |
| NFT-03 | Phase 5 | Complete |
| REV-01 | Phase 5 | Complete |
| ECON-01 | Phase 6 | Complete |
| ECON-02 | Phase 6 | Complete |
| REV-02 | Phase 6 | Complete |
| QUAL-01 | Phase 7 | Pending |
| QUAL-02 | Phase 7 | Pending |
| QUAL-03 | Phase 7 | Pending |
| DOC-01 | Phase 8 | Complete |
| DOC-02 | Phase 8 | Complete |
| DOC-03 | Phase 8 | Complete |

**Coverage:**
- v1 requirements: 26 total
- Mapped to phases: 26
- Unmapped: 0

---
*Requirements defined: 2026-03-17*
*Last updated: 2026-03-19 after Phase 8 planning*









