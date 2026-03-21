# Phase 11: NFT metadata boundary hardening for metaverse asset semantics - Research

**Researched:** 2026-03-20
**Domain:** Passage NFT metadata boundary hardening for metaverse asset semantics
**Confidence:** MEDIUM-HIGH

## Summary

Current active NFT surfaces confirm the core corrective need for Phase 11:
- `pg721` typed extensions are effectively mint-time static.
- `pg721-updatable` only updates `token_uri`, not typed extension payloads.
- Mutable progression fields such as companion `level` and `experience` should move out of generic NFT metadata into dedicated progression state surfaces.
- Durable plugin-to-world rights should be modeled in a separate assignment surface, not inside NFT metadata.
- `profile_id` should be a minimal Passage-standard compatibility constant, while detailed compatibility remains in off-chain manifests.
- `WorldExtension.revenue_shares` is protocol-significant and should remain on-chain.
- Docs, schemas, and examples still advertise stale metadata assumptions and need explicit alignment work.

## Standard Stack

- `contracts/nft/pg721` and `contracts/nft/pg721-updatable` remain the active typed NFT collection surfaces.
- `token_uri` should remain the canonical manifest pointer for metaverse-oriented assets.
- Dedicated state modules should be introduced for mutable protocol-relevant facts instead of extending generic NFT metadata.
- Existing registry and collection identity decisions from Phase 5 remain valid and are not the target of this corrective phase.

## Architecture Patterns

### Pattern 1: Keep durable protocol semantics on-chain
Keep collection identity, creator provenance, royalties, transfer restrictions, collection `nft_type`, and world revenue routing on-chain.

### Pattern 2: Push runtime/detail-heavy metadata into manifests
Keep Unreal/runtime/render/indexer/presentation details in content-addressed manifests referenced by `token_uri`.

### Pattern 3: Model mutable facts in dedicated state surfaces
Progression snapshots, plugin-world durable assignments, entitlement-like state, expiry, and similar mutable facts should live in domain-specific state modules.

### Pattern 4: Use minimal standardized compatibility constants
A Passage-standard `profile_id` should represent compatibility conformance; detailed compatibility matrices remain off-chain.

## Don't Hand-Roll

- Do not add mutable gameplay state back into generic NFT extension structs.
- Do not model durable plugin assignment as ownership-only semantics.
- Do not put Unreal-specific runtime vocabulary on-chain.
- Do not use rich metadata-on-chain surfaces as the default path for metaverse assets.

## Common Pitfalls

- Treating typed NFT metadata as mutable state even though active contracts only enforce it at mint time.
- Encoding plugin installation or world rights inside NFT metadata instead of an assignment surface.
- Expanding compatibility into engine-specific taxonomies on-chain instead of a minimal constant plus manifest detail.
- Forgetting docs/schema drift after contract-surface corrections.
- Ignoring version skew between older `pg721*` dependencies and newer `minter-v2` workspace dependencies when planning changes.

## Validation Architecture

### Test Framework
- Rust unit/integration tests with existing crate-level suites.
- Schema/doc verification for public metadata surfaces.

### Required verification focus
- Prove `pg721-updatable` mutates `token_uri` only.
- Prove retained on-chain fields are protocol-significant and not runtime-only detail.
- Prove mutable progression and plugin assignment semantics are not left half-modeled in generic metadata.
- Prove docs and schemas no longer advertise stale assumptions.

## Corrective Planning Guidance

Recommended plan decomposition:
1. Reclassify and tighten active typed metadata fields across `pg721*` and related shared surfaces.
2. Define or scaffold dedicated state surfaces for progression snapshots and plugin-world durable assignments.
3. Align docs, schemas, examples, and tests with the corrected boundary.

## Confidence

- High confidence that typed NFT extensions are currently mint-time static.
- High confidence that plugin-world durable rights and progression need separate state surfaces.
- Medium confidence on the exact migration shape because it depends on how much compatibility standardization Phase 11 chooses to formalize.
