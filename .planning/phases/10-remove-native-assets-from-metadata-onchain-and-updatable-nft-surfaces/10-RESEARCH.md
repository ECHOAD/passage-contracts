# Phase 10: Remove Native Assets From Metadata-Onchain and Updatable NFT Surfaces - Research

**Researched:** 2026-03-19
**Domain:** active NFT metadata surface cleanup
**Confidence:** HIGH

## Summary

`native_assets` is not part of the active `pg721` base token metadata surface, but it still appears in the active metadata-bearing variants:
- `pg721-updatable::TokenMetadata`
- `pg721-metadata-onchain::Metadata`
- `minter-v2-metadata-onchain` instantiate/config/query/execute/state

The current code treats `native_assets` as descriptive attachment metadata only. It is not used by registry affiliation, marketplace settlement, royalty routing, governance, or collection authorization. The only meaningful behavior left is inside `minter-v2-metadata-onchain`, where admins can set a default template or per-token override and the mint helper injects those values into token metadata when metadata-onchain mode is active.

That makes the field a drift surface rather than a protocol primitive. Removing it is technically feasible, but it must be done coherently across NFT message types, minter state/query/execute APIs, migrations, schema outputs, tests, and docs.

## Where The Drift Lives

### Active NFT metadata surfaces
- `contracts/nft/pg721-updatable/src/msg.rs`
  - defines `NativeAsset`
  - includes `native_assets: Option<Vec<NativeAsset>>` in `TokenMetadata`
- `contracts/nft/pg721-metadata-onchain/src/msg.rs`
  - reuses `pg721::msg::NativeAsset`
  - includes `native_assets: Option<Vec<NativeAsset>>` in `Metadata`

### Metadata-onchain minter surfaces
- `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`
  - `InstantiateMsg.native_asset_template`
  - execute messages: `SetNativeAssetTemplate`, `SetTokenNativeAssetOverride`, `ClearTokenNativeAssetOverride`
  - query messages: `NativeAssetTemplate`, `TokenNativeAssets`
  - response shapes: `NativeAssetTemplateResponse`, `TokenNativeAssetsResponse`
- `contracts/nft/minter-v2-metadata-onchain/src/state.rs`
  - `NativeAsset`
  - `Config.native_asset_template`
  - `TOKEN_NATIVE_ASSET_OVERRIDES`
  - `TokenMetadata.native_assets`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`
  - resolves template/override state and injects `native_assets` into mint payloads
- `contracts/nft/minter-v2-metadata-onchain/src/contract/execute.rs`
  - validates and mutates template/override state
- `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`
  - exposes template/override query results
- `contracts/nft/minter-v2-metadata-onchain/src/migration.rs`
  - carries template state across migrations

### Shared compatibility leftovers
- `contracts/nft/minter-v2/src/state.rs`
  - still defines `NativeAsset` and `TokenMetadata.native_assets`
- `contracts/nft/pg721/src/msg.rs`
  - still defines `NativeAsset` even though base `TokenMetadata` no longer uses it

## What The Code Actually Does Today

1. `pg721-updatable` and `pg721-metadata-onchain` do not enforce any protocol rules around `native_assets`.
2. Validation in those collection contracts only checks NFT typing and typed extension consistency.
3. `minter-v2-metadata-onchain` is the only active module that materially uses the field:
   - stores a default template
   - stores per-token overrides
   - validates those values
   - exposes dedicated query APIs
   - injects them into mint metadata
4. If the field is removed, minting still needs to keep valid `nft_type` / typed extension payloads for the target collection contract; the removal must not break metadata-onchain mint compatibility.

## Planning Implications

1. The cleanup should start at the NFT metadata shape layer so downstream code has a stable target.
2. The minter cleanup is larger because it includes public APIs, storage, migrations, helper behavior, and tests.
3. Schema regeneration and docs alignment need a dedicated closeout step because this field appears in examples and contract reference docs.
4. Regression coverage matters because this is a public message-surface change, even if the runtime behavior was low-value.

## Recommended Phase Split

1. Remove `native_assets` from active `pg721-updatable` / `pg721-metadata-onchain` metadata types and any now-dead shared compatibility structs.
2. Remove `native_asset_template` and per-token override mechanics from `minter-v2-metadata-onchain`, preserving valid mint metadata output.
3. Regenerate schemas and align tests/docs so the retired surface disappears from public contract references.

## Primary Sources

- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `contracts/nft/pg721-updatable/src/contract.rs`
- `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- `contracts/nft/pg721-metadata-onchain/src/contract.rs`
- `contracts/nft/minter-v2/src/state.rs`
- `contracts/nft/minter-v2/src/contract/helpers.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/state.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/execute.rs`
- `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`
- `docs/02-method-reference.md`
