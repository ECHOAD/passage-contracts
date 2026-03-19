# Passage On-Chain Protocol Layer

## What This Is

This project defines and hardens the on-chain protocol layer that gives Passage real economic and ownership guarantees. It covers PASG utility, governance, staking, creator asset contracts, revenue execution, and multi-economy settlement for world builders, users, and the Passage platform without collapsing platform business logic into smart contracts.

The codebase is brownfield: core NFT, marketplace, routing, billing, admin-control, and staking primitives already exist in `passage-contracts`, while the broader product architecture lives in `../context/product/architecture/`. The work now is to reconcile product promises, current contracts, and audit-grade protocol delivery.

## Core Value

PASG must provide real on-chain utility for payments, governance, staking, and creator monetization while keeping platform UX, streaming infrastructure, and other off-chain business workflows out of protocol contracts.

## Requirements

### Validated

- [x] Creator collection and asset primitives already exist through `registry`, `collection-factory`, `ecosystem-factory`, `pg721*`, `minter-v2*`, `marketplace-v3`, and `auction-english` - existing
- [x] Revenue-routing primitives already exist through `split-router`, with a hybrid streaming-payment prototype in `streaming-billing` - existing
- [x] Admin governance and operational safety primitives already exist through `multisig`, admin/operator patterns, pause flags, and selected migration flows - existing
- [x] NFT staking infrastructure already exists through `nft-vault`, `stake-rewards`, and `vault-factory` - existing
- [x] Relationship ownership primitives already exist through `follow` and `friend`, supporting the broader Relationship Protocol direction - existing

### Active

- [ ] Deliver a PASG utility surface that supports payment utility, fee treatment, and token-level protocol integrations promised by product and whitepaper materials.
- [ ] Replace admin-only governance with PASG-scoped on-chain governance for proposals, voting, delegation, quorum, execution, and SIG-compatible parameter control while preserving `multisig` as the admin-owner control plane.
- [ ] Implement PASG staking with 21-day unbonding, scheduled emissions, and the later transition to fee-backed rewards.
- [ ] Complete and harden creator NFT/world/plugin/template monetization flows so royalties, splits, licensing, and asset ownership behave as documented.
- [ ] Define a multi-economy framework so worlds can run their own token or point systems while still settling against PASG.
- [ ] Standardize upgrade, migration, pause, and version-management strategy across long-lived contracts.
- [ ] Raise the protocol to audit-ready quality with high branch coverage, fuzzing/invariant testing, and documentation that matches actual contract behavior.
- [ ] Close brownfield security and correctness gaps in critical contracts before new protocol economics build on top of them.

### Out of Scope

- Pixel streaming infrastructure, GPU orchestration, signaling, and event/session backend workflows - these are platform services and remain off-chain by architecture
- Search, discovery, analytics, recommendations, and content moderation - these are off-chain, high-throughput, or subjective systems
- Avatar rendering, equipment composition runtime, and world-entry UX - these remain off-chain even when ownership and hashes are verified on-chain
- PASG governance over general Passage platform operations - governance in this project is limited to PASG and protocol parameters, not all business workflows
- Treating PASG as platform equity or embedding subscription/business-tier logic directly into token contracts - architecturally and legally out of bounds

## Context

Passage serves three primary constituencies that this protocol layer must support:
- World creators who mint collections, monetize worlds and assets, receive royalties, and need programmable revenue distribution
- Users and players who arrive for the social and 3D experience first, then interact with ownership, payments, governance, and staking
- Passage as a platform, which needs dependable on-chain primitives for payments, splits, fee policy, and PASG utility without moving all business logic on-chain

Key product and architecture inputs were loaded from `../context/product/architecture/`, especially:
- `ONCHAIN_OFFCHAIN_BOUNDARIES.md`
- `FIAT_TO_CRYPTO_PAYMENT_FLOW.md`
- `NFT_COLLECTION_STRATEGY.md`
- `IMPLEMENTATION_REVIEW.md`
- `NFT_METADATA_DESIGN.md`
- `NFT_DESIGN_CORRECTIONS.md`
- `PIXEL_STREAMING_PHASED_APPROACH.md`

Those documents reinforce several program-level truths:
- On-chain owns ownership, payments, governance, revenue rules, and economic guarantees
- Off-chain owns UX, infra orchestration, rendering, search, moderation, and compute-heavy systems
- Hybrid flows are required for streaming billing, fiat bridges, and creator monetization
- The existing repo already contains meaningful brownfield capability, but some docs are more aspirational than the actual implementation and some current contracts have critical trust-boundary gaps

## Constraints

- **Tech stack**: CosmWasm contracts in Rust on the Passage chain - new work should align with existing repo conventions and deployment assumptions around `upasg`
- **Architecture**: Preserve the on-chain/off-chain boundary documented in `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`
- **Priority**: PASG utility is the highest-priority outcome if tradeoffs arise
- **Scope**: All eight deliverable areas discussed with PASG stakeholders remain in active scope; none are intentionally deferred at initialization time
- **Brownfield reality**: Existing contracts must be reviewed, hardened, and evolved in place where safe; this is not a clean-slate rewrite
- **Governance scope**: On-chain governance must stay scoped to PASG/protocol parameters rather than full platform operations
- **Quality bar**: The intended finish line is audit-ready delivery with strong coverage, fuzzing, and migration discipline

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Treat this as a full protocol program, not a narrow feature spike | User explicitly scoped all eight deliverables into the active milestone set | - Pending |
| Prioritize PASG utility over all other sequencing tradeoffs | PASG utility is the single most important thing that must work first | - Pending |
| Preserve strict on-chain/off-chain boundaries | Product architecture repeatedly states that contracts should provide economic guarantees while platform services provide UX and infrastructure | - Pending |
| Use the existing repo as the foundation | The workspace already implements NFT commerce, routing, admin control, and staking primitives worth preserving and hardening | - Pending |
| Scope governance to PASG/protocol parameters only | Prevents architecture creep, legal ambiguity, and accidental control over unrelated off-chain operations | - Pending |
| Model world/asset monetization as hybrid | Product docs and current code both point to on-chain revenue rules invoked by off-chain services rather than fully autonomous billing | - Pending |

---
*Last updated: 2026-03-17 after initialization*
