# Phase 5: Creator Asset Contracts & Monetization - Context

**Gathered:** 2026-03-19
**Status:** Ready for replanning
**Source:** User discussion during `$gsd-discuss-phase 5`

<domain>
## Phase Boundary

Complete the creator-asset registration and monetization model around ecosystems, collections, typed Passage NFT assets, and revenue-bearing flows.

This phase clarifies how creator assets are organized and registered:
- `registry` is the canonical source of truth for registered ecosystems and registered collections
- an `ecosystem` is the parent context for collections
- a collection is an independent on-chain contract that can be affiliated, de-affiliated, and later re-homed across ecosystems without losing its creator identity
- typed asset and monetization semantics remain on-chain, while rendering/runtime behavior remains off-chain

</domain>

<decisions>
## Implementation Decisions

### Registry as canonical registry
- `registry` is the canonical ledger for ecosystems and for collections that are officially affiliated with an ecosystem.
- Ecosystem creation resolves into canonical registration in `registry`; workflow contracts are not the final source of truth.
- Collection affiliation to an ecosystem is an administrative relationship recorded by `registry`, not an intrinsic property of the collection contract forever.

### Ecosystem and collection relationship
- An `ecosystem` is the parent context, for example a world or universe such as `Cyberpunk Universe`.
- Multiple collections can exist inside one ecosystem.
- A collection remains an independent on-chain contract even when registered to an ecosystem.
- If a collection is removed from an ecosystem, it must continue to exist on-chain with its history intact.
- A collection removed from an ecosystem becomes a loose collection with no current ecosystem affiliation.
- A loose collection may later be accepted by another ecosystem.
- When a collection is re-homed into another ecosystem, it keeps its original creator; only the ecosystem affiliation changes.

### Collection authorization model
- Collection creation/registration should not rely on a per-collection request/approval workflow.
- The ecosystem admin can create or register collections directly.
- Approved ecosystem members/operators can also create or register collections directly.
- The correct control mechanism is ecosystem membership and membership revocation, not collection-by-collection approval.
- If access must be removed, the ecosystem admin revokes the member's access.

### Collection removal and re-homing
- The protocol should support administrative removal of a collection from an ecosystem registry entry without destroying the collection contract.
- Removal only detaches the collection from the ecosystem in `registry`.
- Another ecosystem may later adopt that collection into its own registry domain.
- Re-homing must not rewrite original creator ownership semantics.

### Identity model
- Collection contract address should be treated as the canonical collection identifier.
- Artificial collection IDs are unnecessary when the contract address already exists.
- Ecosystem string IDs should be reconsidered and minimized where a stable contract address already exists.
- If ecosystem identity can be represented canonically by an existing ecosystem-bound contract address, that address should be preferred over a synthetic ID; human-readable names/slugs should remain metadata.

### Claude's Discretion
- Whether Phase 5 fully removes `collection creation request` surfaces now or deprecates them first with migration notes.
- Whether ecosystem identity should become `collection_factory` address directly or another ecosystem-bound address already present in the model.
- Exact execute/query names for collection deregistration and ecosystem re-homing flows.
- Whether loose collections remain queryable through a dedicated `unaffiliated collections` surface or through a nullable ecosystem field.

</decisions>

<specifics>
## Specific Ideas

- `registry` should act as the place "where everything is registered" canonically.
- The ecosystem team model matters more than per-collection approvals.
- Collections should be able to move between ecosystems administratively without losing creator provenance.
- The user explicitly does not want unwanted collections solved through request approvals; they should be handled by membership control and deregistration.
- The user explicitly prefers real contract addresses over synthetic IDs wherever possible.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Product and architecture
- `../context/product/architecture/NFT_COLLECTION_STRATEGY.md` - creator asset hierarchy, ecosystem and collection intent
- `../context/product/architecture/IMPLEMENTATION_REVIEW.md` - current drift between intended model and existing implementation
- `../context/product/architecture/NFT_METADATA_DESIGN.md` - typed asset metadata expectations and asset semantics
- `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md` - corrected Passage NFT model and contract boundary expectations

### Registry and ecosystem registration
- `contracts/core/registry/src/msg.rs` - canonical execute/query surface for ecosystem and collection registration today
- `contracts/core/registry/src/state.rs` - current ecosystem, collection, membership, moderation, and request storage model
- `contracts/core/registry/src/contract/helpers.rs` - current access-control and ecosystem policy behavior
- `contracts/core/registry/src/contract/query.rs` - current ecosystem/collection query surface that Phase 5 will likely reshape

### Ecosystem and collection factories
- `contracts/core/ecosystem-factory/src/msg.rs` - current ecosystem creation request workflow and downstream registry registration path
- `contracts/core/collection-factory/src/msg.rs` - current collection creation interface and registry registration path
- `contracts/core/collection-factory/src/state.rs` - current collection records and ecosystem-linked creation state

### Typed NFT contracts
- `contracts/nft/pg721/src/msg.rs` - base typed collection metadata surface
- `contracts/nft/pg721-updatable/src/msg.rs` - updatable typed collection metadata surface
- `contracts/nft/pg721-metadata-onchain/src/msg.rs` - on-chain metadata collection variant that may need alignment

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `contracts/core/registry`: already stores ecosystems, collections, ecosystem members, creator moderation, and collection moderation; it is the natural place to remain canonical.
- `contracts/core/ecosystem-factory`: already implements ecosystem request/approval workflow and downstream registration into `registry`.
- `contracts/core/collection-factory`: already creates collections inside an ecosystem context and registers them into `registry`.
- `contracts/nft/pg721*`: already provide the collection contracts whose addresses can serve as canonical collection identity.

### Established Patterns
- Ecosystem creation already follows a workflow-contract -> registry-registration model.
- Collection creation currently assumes ecosystem affiliation and pushes canonical registration through `RegisterCollectionFromFactory`.
- Membership and moderation already exist in `registry`, so access control can move toward team-based authorization rather than request-based approval.
- Current collection creation request surfaces in `registry` exist, but they now conflict with the newly clarified direction.

### Integration Points
- Phase 5 should review `registry` ecosystem and collection state first, because affiliation, deregistration, and re-homing all converge there.
- `collection-factory` and `ecosystem-factory` will need to align with the final identity and authorization model chosen in `registry`.
- Typed NFT and monetization work still depends on `pg721*`, `minter-v2*`, and sale-path helpers, but the registration model now has to be replanned around the clarified ecosystem rules.

</code_context>

<deferred>
## Deferred Ideas

- Whether ecosystem identities should be fully migrated from string IDs to contract-address keys in this phase, or whether that should be staged for a follow-up migration-focused phase.
- Whether ecosystem adoption of an already-existing loose collection should require an explicit collection-owner acknowledgement in a later phase.

</deferred>

---

*Phase: 05-creator-asset-contracts-monetization*
*Context gathered: 2026-03-19 via direct user decisions*
