# Phase 05 Research

**Phase:** 05 - Creator Asset Contracts & Monetization
**Date:** 2026-03-19
**Status:** Complete

## Goal

Complete the asset-contract model around creators, collections, worlds, plugins, achievements, avatars, companions, and monetization-bearing NFT types.

## Inputs Reviewed

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `../context/product/architecture/NFT_COLLECTION_STRATEGY.md`
- `../context/product/architecture/IMPLEMENTATION_REVIEW.md`
- `../context/product/architecture/NFT_METADATA_DESIGN.md`
- `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md`
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- `contracts/core/collection-factory/src/msg.rs`
- `contracts/core/registry/src/msg.rs`
- `contracts/core/split-router/src/msg.rs`
- `contracts/nft/minter-v2/src/contract/helpers.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`

## Confirmed Current State

1. The repo already follows the approved collection model: one CW721-style contract per collection, with `nft_type` set at collection instantiate time.
2. `pg721`, `pg721-updatable`, `collection-factory`, `registry`, and the minter families already carry `NftType` through collection creation and registration flows.
3. `pg721` and `pg721-updatable` validate that token metadata `nft_type` matches the collection `nft_type`, which is the right base safety property.
4. Royalty info is already present at collection level and downstream sale flows such as `marketplace-v3` and `auction-english` already read royalty data from collection info.
5. `split-router` is already generic and can route creator-side revenue without becoming NFT-type-specific.

## Main Gaps

### Gap 1: `NftType` enum is ahead of `NftTypeExtension`

The collection-level enum includes:
- `Component`
- `Avatar`
- `Companion`
- `World`
- `Plugin`
- `Achievement`
- `WorldTemplate`

But `NftTypeExtension` in `pg721` and `pg721-updatable` only supports:
- `Component`
- `Avatar`
- `Companion`
- `World`

This is the clearest Phase 5 drift. The repo advertises seven NFT types but only four have typed token-extension support.

### Gap 2: The pg721 family is not fully aligned across variants

The repo has multiple related collection contracts:
- `pg721`
- `pg721-updatable`
- `pg721-metadata-onchain`

Phase 5 should avoid introducing new type drift between those families. The typed extension story, collection queries, and mismatch validation need a consistent stance across the collection contracts that remain in scope.

### Gap 3: Product docs describe richer type semantics than the contracts currently expose

The architecture docs describe semantics such as:
- plugin permissions and licensing
- achievement/soulbound behavior
- world-template branding requirements
- avatar equipment references
- companion progression behavior
- world revenue-share semantics

Some of these are represented today only as architecture notes, not enforceable contract interfaces.

### Gap 4: Monetization boundaries need to stay explicit

`REV-01` is about executing creator/platform/partner revenue splits from on-chain rules. The repo already has the pieces:
- collection-level royalties in the pg721 family
- generic split execution in `split-router`
- world revenue routing entrypoints in `split-router` and `streaming-billing`

What is still needed is a clear contract-level model for which NFT types may carry monetization rules, where the revenue-share data lives, and how factories/registry/docs describe those flows without leaking off-chain product logic into contracts.

## Recommended Phase 5 Direction

### Direction 1: Extend the existing shared collection contracts, not a new contract per NFT type

The architecture docs originally discuss conceptual per-type models, but the actual repo standard is shared collection contracts with typed metadata. Phase 5 should preserve that:
- keep `pg721` and `pg721-updatable` as the collection primitives
- complete missing typed extension variants there
- keep factory and registry interfaces centered on `nft_type`

### Direction 2: Complete the missing typed extension variants first

The highest-value first move is to add and validate:
- `PluginExtension`
- `AchievementExtension`
- `WorldTemplateExtension`

This closes the most obvious architectural drift and directly advances `NFT-02`.

### Direction 3: Keep off-chain vs on-chain boundaries strict

Phase 5 should not try to force Unreal/rendering/runtime systems on-chain.

Safe on-chain responsibilities:
- collection `nft_type`
- token extension discriminants and compact typed fields
- royalty/revenue-share configuration needed for automated settlement
- transfer restrictions only where the product explicitly requires them, such as soulbound achievements

Off-chain responsibilities that should stay off-chain:
- rendering assets and composition logic
- full avatar equipment rendering state
- plugin execution runtime
- analytics, search, and rich content indexing

### Direction 4: Make monetization rules type-aware but keep routing generic

The routing layer should stay generic. Phase 5 should instead make the asset contracts and docs explicit about:
- which types can carry `revenue_shares`
- when royalty-only is enough versus when split-router-backed routing is expected
- how registry/factory documentation explains creator monetization-bearing assets

## Likely Work Breakdown

### 05-01
Reconcile typed collection contracts and missing extension models.

Likely files:
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721/src/contract.rs`
- `contracts/nft/pg721/src/error.rs`
- `contracts/nft/pg721/src/contract/tests.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721-updatable/src/contract.rs`
- `contracts/nft/pg721-updatable/src/error.rs`
- `contracts/nft/pg721-updatable/src/contract/tests.rs`
- possibly `contracts/nft/pg721-metadata-onchain/*` if retained in the same compatibility story

### 05-02
Harden monetization-bearing asset semantics and explicit revenue routing boundaries.

Likely files:
- `contracts/core/collection-factory/src/msg.rs`
- `contracts/core/collection-factory/src/contract/execute.rs`
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/state.rs`
- `contracts/core/split-router/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- market/auction helper tests where royalty and split-router assumptions are asserted

### 05-03
Align docs, schemas, and verification with the final type model.

Likely files:
- `README.md`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`
- package READMEs and schema examples for affected contracts

## Planning Risks

1. There are three pg721-like contract families, so Phase 5 can easily reintroduce drift if it updates only one.
2. The product docs describe ambitious plugin/achievement/template behavior; the phase must keep only the enforceable contract pieces on-chain.
3. Soulbound achievements are likely the only place where transfer semantics may need type-specific enforcement. That needs an explicit design choice rather than implicit metadata-only documentation.
4. Revenue-share semantics should not duplicate marketplace fee logic or re-embed off-chain billing logic into NFT contracts.

## Recommended Validation Strategy

- Verify every `NftType` variant has a coherent contract story: supported token extension, explicitly retired, or explicitly documented as metadata-only.
- Add deterministic tests for type mismatch validation and newly added type variants.
- Add deterministic tests or grep checks proving monetization-bearing data is queryable from the intended contract surfaces.
- Keep workspace-level checks and unit tests in the final verification loop.

## Research Conclusion

Phase 5 should be planned as a three-wave alignment phase:
1. complete and normalize the typed NFT extension surface
2. harden creator monetization and revenue-bearing asset rules without breaking the off-chain boundary
3. align factory, registry, docs, and verification so the repo tells one coherent story for creator assets
