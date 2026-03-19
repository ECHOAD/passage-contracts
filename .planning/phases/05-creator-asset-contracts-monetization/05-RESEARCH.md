# Phase 05 Research

**Phase:** 05 - Creator Asset Contracts & Monetization
**Date:** 2026-03-19
**Status:** Updated for replan

## Goal

Complete the creator-asset contract model around ecosystems, collections, creators, typed NFT assets, and monetization-bearing flows using `registry` as the canonical affiliation layer.

## Inputs Reviewed

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/05-creator-asset-contracts-monetization/05-CONTEXT.md`
- `../context/product/architecture/NFT_COLLECTION_STRATEGY.md`
- `../context/product/architecture/IMPLEMENTATION_REVIEW.md`
- `../context/product/architecture/NFT_METADATA_DESIGN.md`
- `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md`
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/state.rs`
- `contracts/core/registry/src/contract/execute.rs`
- `contracts/core/collection-factory/src/msg.rs`
- `contracts/core/collection-factory/src/state.rs`
- `contracts/core/ecosystem-factory/src/msg.rs`
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- `contracts/core/split-router/src/msg.rs`
- `contracts/nft/minter-v2/src/contract/helpers.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`

## Confirmed Current State

1. `registry` already acts as the canonical store for ecosystems and collections.
2. `ecosystem-factory` already uses a workflow path that resolves into `RegisterEcosystemFromFactory`, so the final source of truth is already `registry`.
3. `collection-factory` already creates one CW721-style contract per collection and registers that contract into `registry`.
4. Collection contract address is already the primary collection key in `registry`, which matches the newly clarified identity model.
5. `registry` still carries a collection-creation request workflow and a fixed `ecosystem_id`-centric collection affiliation model that no longer matches the desired Phase 5 behavior.
6. Typed NFT drift still exists: the repo advertises seven collection-level `NftType` variants, but `NftTypeExtension` in the pg721 family still trails behind.
7. Royalties already exist at collection level, and downstream sale paths already read royalty data from collection info; routing primitives remain generic.

## Main Gaps

### Gap 1: Collection affiliation is modeled as fixed membership, not mutable affiliation

The user clarified that a collection:
- is its own on-chain contract
- can belong to an ecosystem today
- can later be removed from that ecosystem without being destroyed
- can later be adopted by another ecosystem
- must keep its original creator during re-homing

The current `registry` collection model stores `ecosystem_id` as a required field and does not expose an explicit detach/re-home flow.

### Gap 2: Collection request/approval flow is the wrong authorization model

The desired model is:
- ecosystem admin can create/register collections directly
- approved ecosystem members can create/register collections directly
- access control is handled by team membership plus revocation
- unwanted collections are handled by deregistration, not request review

Current `registry` still exposes `SubmitCollectionCreationRequest` and `ResolveCollectionCreationRequest`, and helper logic still assumes `ApprovalRequired` as a first-class collection workflow.

### Gap 3: Ecosystem identity is still overly string-ID-centric

Collection identity is already correctly based on contract address, but ecosystem surfaces still assume synthetic `ecosystem_id` strings everywhere. The user direction is to minimize artificial IDs when an address can serve as canonical identity. The repo needs an explicit stance for this phase:
- either keep string ecosystem IDs for now but treat them as legacy metadata while introducing stronger address-based identity
- or partially refactor ecosystem surfaces toward address-backed identity in a staged way

### Gap 4: Typed asset drift still exists in the pg721 family

The collection-level enum includes:
- `Component`
- `Avatar`
- `Companion`
- `World`
- `Plugin`
- `Achievement`
- `WorldTemplate`

But `NftTypeExtension` in `pg721` and `pg721-updatable` still only supports the original subset. Phase 5 still needs to close that drift.

### Gap 5: Monetization semantics need to be explicit without polluting routers

`REV-01` still requires a clear on-chain story for creator/platform/partner revenue routing. The routing layer should remain generic, while the creator asset and registry model should make monetization-bearing collections queryable and coherent.

## Recommended Phase 5 Direction

### Direction 1: Recenter the phase on `registry` as the canonical affiliation ledger

Phase 5 is no longer just "finish more NFT metadata". It now needs to define how ecosystems and collections relate canonically:
- ecosystem registered in `registry`
- collection contract registered in `registry`
- ecosystem affiliation mutable in `registry`
- collection creator provenance preserved across affiliation changes

### Direction 2: Replace collection request flow with membership-based authorization

The strongest architecture match is:
- ecosystem admin and approved ecosystem members can create or register directly
- collection request maps become dead weight and should be removed or explicitly deprecated during migration
- collection removal from ecosystem should be an admin operation in `registry`

### Direction 3: Treat collection contract address as the only collection identity that matters

The repo already does this in storage. Phase 5 should make the surrounding interfaces, docs, and flows fully consistent with that fact.

### Direction 4: Keep typed NFT completion as a major workstream, but after affiliation rules are corrected

The prior plan made typed metadata the first wave. With the new context, the registry/ecosystem relationship needs to be fixed first because it changes the lifecycle that factories, docs, and monetization depend on.

### Direction 5: Keep routing generic and move meaning into asset metadata plus canonical registry data

No NFT-type-specific router branches should be invented. Revenue-bearing behavior should stay driven by collection metadata, registry affiliation, royalty info, and existing generic split execution.

## Likely Work Breakdown

### 05-01
Rework ecosystem and collection affiliation rules in `registry`, `collection-factory`, and `ecosystem-factory`.

Likely files:
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/state.rs`
- `contracts/core/registry/src/contract/execute.rs`
- `contracts/core/registry/src/contract/query.rs`
- `contracts/core/registry/src/contract/helpers.rs`
- `contracts/core/collection-factory/src/msg.rs`
- `contracts/core/collection-factory/src/state.rs`
- `contracts/core/collection-factory/src/contract/execute.rs`
- `contracts/core/collection-factory/src/contract/reply.rs`
- `contracts/core/ecosystem-factory/src/msg.rs`
- `contracts/core/ecosystem-factory/src/contract/execute.rs`

### 05-02
Complete typed asset support and align monetization-bearing asset semantics.

Likely files:
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721/src/contract.rs`
- `contracts/nft/pg721/src/contract/tests.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721-updatable/src/contract.rs`
- `contracts/nft/pg721-updatable/src/contract/tests.rs`
- `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- `contracts/nft/minter-v2/src/contract/helpers.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`
- `contracts/nft/marketplace-v3/src/contract/helpers.rs`
- `contracts/nft/auction-english/src/helpers.rs`

### 05-03
Align docs, schemas, examples, and verification with the final ecosystem/collection/typed-asset model.

Likely files:
- `README.md`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`
- `contracts/core/registry/README.md`
- `contracts/core/collection-factory/README.md`
- `contracts/core/ecosystem-factory/README.md`
- `contracts/nft/pg721/README.md`
- `contracts/nft/pg721-updatable/README.md`
- `contracts/nft/pg721/examples/schema.rs`
- `contracts/nft/pg721-updatable/examples/schema.rs`

## Planning Risks

1. `registry` currently threads `ecosystem_id` through state, queries, moderation, and recovery logic. Reworking affiliation can ripple widely.
2. The user wants to minimize synthetic IDs, but ecosystem identity is currently heavily string-based. This phase must avoid a half-migration that breaks recoverability or query ergonomics.
3. Removing collection request flows must not accidentally remove legitimate moderation controls that should stay at the team-membership layer.
4. Re-homing collections must preserve creator provenance and avoid silently changing collection ownership semantics.
5. Typed NFT expansion still has to remain compact and contract-facing; Phase 5 must not drag Unreal/runtime concerns on-chain.

## Recommended Validation Strategy

- Verify request-based collection creation surfaces are removed or explicitly deprecated from the public registry/factory interfaces.
- Verify admin/member authorization remains deterministic after removing collection requests.
- Verify collections can be detached from an ecosystem without destroying the collection record or creator provenance.
- Verify any re-home flow changes only affiliation and not creator ownership.
- Verify every advertised `NftType` variant ends with a coherent extension or explicit contract stance.
- Keep `cargo check --workspace` and `cargo unit-test` in the final verification floor.

## Research Conclusion

Phase 5 should now be planned as a three-wave realignment phase:
1. redefine ecosystems and collection affiliation canonically in `registry`
2. complete typed NFT and monetization-bearing collection semantics on top of that lifecycle
3. align docs, schemas, examples, and verification so the repo tells one coherent ecosystem-centric creator asset story
