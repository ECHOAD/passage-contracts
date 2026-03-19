# Phase 5: Creator Asset Contracts & Monetization - Validation Strategy

**Defined:** 2026-03-19
**Scope:** validation guardrails for ecosystem-centric collection registration, typed NFT completion, and creator monetization
**Status:** updated-for-replan

## Validation Guardrails

- `registry` must remain the canonical source of truth for ecosystems and registered collection affiliation.
- Collection contracts must remain independent on-chain contracts; ecosystem affiliation changes must never imply contract destruction.
- Collection creator provenance must survive deregistration and any later ecosystem re-homing.
- Collection authorization should be membership-based, not request-based.
- Collection identity must remain the collection contract address; synthetic collection IDs must not be introduced.
- `NftType` support must not drift between `pg721`, `pg721-updatable`, and any retained pg721 variant Phase 5 still treats as active.
- Monetization rules must remain on-chain only where the protocol can actually enforce or query them; rendering/runtime/platform UX concerns stay off-chain.
- Revenue routing must remain generic in routing contracts. Type-specific monetization belongs in asset metadata, collection info, registry semantics, or integration docs.

## Wave 0 Checks For Plan

- The replanned phase covers all four Phase 5 requirement IDs: `NFT-01`, `NFT-02`, `NFT-03`, `REV-01`.
- At least one plan explicitly removes or deprecates collection request/approval surfaces in favor of ecosystem membership authorization.
- At least one plan explicitly introduces deregistration and re-home semantics for collections while preserving creator provenance.
- At least one plan explicitly closes the missing `Plugin`, `Achievement`, and `WorldTemplate` extension drift.
- At least one plan explicitly aligns docs/schemas/examples to the new registry/ecosystem/collection model.

## Automated Verification To Keep

- `cargo check -p registry -p collection-factory -p ecosystem-factory`
- `cargo check -p pg721 -p pg721-updatable`
- `cargo test -p registry --lib`
- `cargo test -p pg721 --lib`
- `cargo test -p pg721-updatable --lib`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p auction-english --lib`
- `cargo check --workspace`
- `cargo unit-test`

These remain the regression floor. Phase 5 should also rely on plan-level grep checks proving that request-based collection flows are gone or deprecated, membership-based creation is explicit, deregistration/re-home surfaces exist, and typed extension variants are present where expected.
