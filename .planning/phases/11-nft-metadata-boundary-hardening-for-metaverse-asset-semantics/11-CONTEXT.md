# Phase 11: NFT metadata boundary hardening for metaverse asset semantics - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Harden the Passage NFT metadata boundary so metaverse assets keep only durable protocol semantics on-chain, move rendering/runtime/detail-heavy metadata into content-addressed manifests, and relocate mutable protocol-relevant state into dedicated state surfaces instead of generic NFT metadata.

This phase does not redesign Phase 5 collection identity, `registry`, or the broader on-chain/off-chain architecture. It clarifies and corrects the token-level metadata boundary for metaverse-facing NFT types.

</domain>

<decisions>
## Implementation Decisions

### Metadata boundary classes
- Properties used for rendering, client behavior, indexer enrichment, presentation, or runtime consumption default to off-chain manifests referenced by `token_uri`.
- Properties that affect monetization, transferability, royalties, revenue routing, or other enforceable protocol rights must stay on-chain.
- Mutable facts with protocol effect must not live in generic NFT metadata; if they need on-chain trust, they belong in dedicated state/contract surfaces.

### Mutable state and progression
- The base NFT identifies the asset and its durable semantics; mutable state is stored in domain-specific state modules rather than inside generic NFT metadata.
- Passage should prefer separate state modules by domain instead of a single generic overlay module.
- Progression is optional and should exist only where a real protocol/product case exists.
- UI should present the NFT plus its state module as a single composed asset to end users.
- `Avatar` and `Companion` may both have progression, but not in generic NFT metadata.
- Progression is primarily in-game and world-specific rather than globally standardized by Passage.
- XP/level progression is calculated off-chain inside the game/runtime and persisted on-chain at save points rather than per action.
- Passage should standardize how progression snapshots are persisted, not impose one global gameplay formula for XP or leveling.
- The NFT itself should not change on every level-up; exceptional evolution flows may use explicit NFT operations such as burn-and-mint replacement.

### Plugin rights and assignment
- Plugins are not consumables and should not be modeled with usage counters.
- A plugin NFT represents durable ownership or licensing semantics rather than limited-use consumption.
- A world may receive a durable right to use a plugin through an explicit assignment/installation relationship separate from simple NFT ownership.
- If a plugin is later sold, a world that already received a durable assignment keeps that right.
- Plugin-to-world assignment must be visible and queryable on-chain as an explicit protocol relationship.
- Technical plugin deployment, binaries, runtime permissions, and installation mechanics remain off-chain.

### Compatibility and interoperability
- Compatibility signals kept on-chain must be minimal.
- Passage should use standardized, versioned `profile_id` conventions for shared interoperability promises rather than arbitrary creator-defined strings.
- `profile_id` is a Passage compatibility constant that indicates conformance with a known standard, not freeform metadata.
- `Avatar` and `Companion` should carry shared Passage compatibility profiles because they are expected to work across all Passage worlds.
- Detailed technical compatibility rules remain in off-chain manifests.
- Worlds may define additional world-specific compatibility rules off-chain.
- Assets that do not need meaningful interoperability may omit on-chain compatibility profiles.

### NFT type taxonomy
- Only asset classes with distinct protocol semantics should remain first-class `NftType` variants.
- `Avatar` and `Companion` remain first-class on-chain types.
- `World` and `Plugin` remain first-class on-chain types because they carry distinct rights and monetization semantics.
- `Emote` should remain a manifest-level category unless it later gains distinct protocol semantics or entitlement behavior.
- `Scene` should remain a manifest-level category or world-template concern unless it becomes an independently owned and monetized protocol asset.
- `AccessPass` should not be solved by metadata alone; if Passage needs expiry, redemption, or gating semantics, it needs dedicated state and may justify a more explicit type/surface later.

### Claude's Discretion
- Exact state-surface layout for progression, entitlement, and plugin assignment modules.
- Whether Passage should store only `profile_id` on-chain or pair it with an optional verification hash later.
- Exact naming and query surfaces for plugin-to-world assignment records.
- Whether future specialized types like `Emote`, `Scene`, or `AccessPass` should become first-class enums in a later phase once their semantics are clearer.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project boundary and requirements
- `.planning/PROJECT.md` - core on-chain/off-chain boundaries and protocol scope
- `.planning/REQUIREMENTS.md` - governing requirements for NFT assets, monetization, and off-chain runtime boundaries
- `.planning/STATE.md` - recent roadmap evolution and prior architectural decisions
- `.planning/ROADMAP.md` - Phase 11 placement and relationship to Phase 5 and Phase 10

### Prior NFT design decisions
- `.planning/phases/05-creator-asset-contracts-monetization/05-CONTEXT.md` - prior locked decisions about collection identity, registry, and typed asset semantics
- `.planning/phases/05-creator-asset-contracts-monetization/05-RESEARCH.md` - corrective research identifying token-level metadata drift and the need for this phase

### Product and architecture
- `../context/product/architecture/NFT_COLLECTION_STRATEGY.md` - asset hierarchy and collection intent
- `../context/product/architecture/IMPLEMENTATION_REVIEW.md` - current drift between intended model and repo behavior
- `../context/product/architecture/NFT_METADATA_DESIGN.md` - original NFT metadata intent and areas later needing correction
- `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md` - corrected Passage NFT model and contract-boundary expectations
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md` - authoritative boundary between protocol semantics and runtime/platform concerns
- `../context/product/architecture/PIXEL_STREAMING_PHASED_APPROACH.md` - confirms Pixel Streaming is product/runtime infrastructure rather than protocol metadata

### Current NFT contract surfaces
- `contracts/nft/pg721/src/msg.rs` - typed NFT metadata structure and collection/token semantics
- `contracts/nft/pg721/src/contract.rs` - enforcement of collection type and mint-time metadata validation
- `contracts/nft/pg721-updatable/src/msg.rs` - current updatable surface, including the fact that `token_uri` is mutable while typed extension fields are not
- `contracts/nft/pg721-updatable/src/contract.rs` - actual update behavior for token metadata
- `contracts/nft/pg721-metadata-onchain/src/msg.rs` - richer metadata-on-chain variant that should not be the default metaverse path
- `contracts/nft/minter-v2/src/msg.rs` - current manifest-pointer minting flow
- `contracts/nft/minter-v2-metadata-onchain/src/msg.rs` - current metadata-on-chain minting variant for comparison

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `contracts/nft/pg721`: already provides collection-level `nft_type` enforcement and compact typed token metadata.
- `contracts/nft/pg721-updatable`: already establishes the key boundary that `token_uri` can change without mutating typed extension fields.
- `contracts/nft/minter-v2`: already fits a manifest-pointer model through `base_token_uri` and collection-targeted minting.
- `contracts/core/registry`: remains the canonical affiliation layer and does not need redesign in this phase.

### Established Patterns
- Passage already treats rendering/runtime concerns as off-chain at the architecture level.
- Collection identity and monetization rights already sit on-chain and should remain there.
- Token metadata today is validated strongly at mint time, but mutable typed token state is not supported by the active `pg721` family.
- Prior work already removed unsupported attachment surfaces such as `native_assets`, reinforcing the move toward a tighter metadata boundary.

### Integration Points
- This phase should primarily reshape `contracts/nft/pg721/src/msg.rs`, `contracts/nft/pg721-updatable/src/msg.rs`, related shared typed metadata semantics, and docs/tests that still imply the wrong boundary.
- If plugin assignment or progression snapshots need protocol surfaces, planning should define separate contract/state modules or explicit extensions rather than extending generic NFT metadata.
- Any compatibility profile convention introduced here must align with off-chain manifest consumers in Passage runtime services and world integrations.

</code_context>

<specifics>
## Specific Ideas

- The user wants the final model aligned to real metaverse usage with Unreal and Pixel Streaming while keeping those runtime concerns off-chain.
- The user explicitly wants plugin rights to survive NFT resale once a world has already received a durable assignment.
- The user clarified that avatar and companion progression happens in-game, not by modifying NFT metadata every time gameplay changes.
- The user relayed CEO guidance that progression should persist at save points, not on every gameplay event, to avoid constant wallet friction and on-chain costs.
- `profile_id` should behave like a Passage convention/standard constant rather than arbitrary creator metadata.

</specifics>

<deferred>
## Deferred Ideas

- Whether future asset classes such as `Emote`, `Scene`, or `AccessPass` should become first-class `NftType` variants in a later phase once their protocol semantics are fully defined.
- Whether Passage should add optional `profile_hash` verification alongside standardized `profile_id` conventions in a later hardening step.
- Whether progression-capable asset types should eventually share one reusable persistence module or only a common interface/spec while keeping separate implementations by domain.

</deferred>

---

*Phase: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics*
*Context gathered: 2026-03-20*
