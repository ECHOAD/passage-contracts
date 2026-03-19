# Phase 5: Creator Asset Contracts & Monetization - Validation Strategy

**Defined:** 2026-03-19
**Scope:** validation guardrails for typed NFT asset completion and creator monetization
**Status:** ready-to-plan

## Validation Guardrails

- No task may break the repo-wide rule that each collection is its own CW721-style contract with a collection-level `nft_type`.
- `NftType` support must not drift between `pg721`, `pg721-updatable`, and any retained pg721 variant that Phase 5 still treats as active.
- Every NFT type promised by the repo must end Phase 5 in one of three states: implemented in contract interfaces, explicitly retired, or explicitly documented as metadata-only/out-of-scope.
- Monetization rules must remain on-chain only where the protocol can actually enforce or query them; rendering/runtime/platform UX concerns stay off-chain.
- Revenue routing must remain generic in routing contracts. Type-specific monetization belongs in asset metadata, collection info, or explicit integration docs, not hardcoded route families.

## Wave 0 Checks For Plan

- The plan covers all four Phase 5 requirement IDs: `NFT-01`, `NFT-02`, `NFT-03`, `REV-01`.
- At least one plan explicitly closes the missing `Plugin`, `Achievement`, and `WorldTemplate` extension drift.
- At least one plan explicitly addresses creator/world/plugin/template monetization semantics and routing boundaries.
- At least one plan explicitly aligns docs/schemas/tests after the contract changes.

## Automated Verification To Keep

- `cargo check --workspace`
- `cargo unit-test`

These remain the regression floor. Phase 5 should also rely on plan-level grep checks proving that typed extension variants, revenue semantics, and documentation alignment exist where expected.
