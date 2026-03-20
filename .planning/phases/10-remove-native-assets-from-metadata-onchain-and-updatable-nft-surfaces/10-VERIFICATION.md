---
phase: 10
slug: remove-native-assets-from-metadata-onchain-and-updatable-nft-surfaces
status: passed
created: 2026-03-19
updated: 2026-03-19
---

# Phase 10 Verification

## Goal

Retire the unused `native_assets` attachment model from active metadata-onchain and updatable NFT surfaces so Passage NFT typing stays focused on supported typed extensions and mint flows.

## Result

Passed.

## Must-Have Verification

1. Active Passage NFT metadata surfaces no longer expose `native_assets` or `NativeAsset` fields/types where they are not part of supported typed extension behavior.
   - Verified in `contracts/nft/pg721/src/msg.rs`, `contracts/nft/pg721-updatable/src/msg.rs`, `contracts/nft/pg721-metadata-onchain/src/msg.rs`, and `contracts/nft/minter-v2/src/state.rs`.
2. `minter-v2-metadata-onchain` no longer stores template/override state or execute/query APIs for native asset attachments, while minting still emits valid metadata for the target collection contracts.
   - Verified in `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`, `contracts/nft/minter-v2-metadata-onchain/src/state.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/execute.rs`, and `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`.
3. Schemas, tests, and docs no longer advertise native asset attachment support on those active surfaces.
   - Verified via regenerated `schema/` outputs, `docs/02-method-reference.md`, and the English/Spanish NFT contract pages.

## Automated Verification

- `cargo test -p pg721-updatable --lib`
- `cargo check -p pg721-updatable -p pg721-metadata-onchain`
- `cargo test -p minter-v2-metadata-onchain --lib`
- `cargo check -p minter-v2-metadata-onchain`
- `cargo run --example schema -p pg721-updatable`
- `cargo run --example schema -p pg721-metadata-onchain`
- `cargo run --example schema -p minter-v2-metadata-onchain`
- `cargo check -p pg721-updatable -p pg721-metadata-onchain -p minter-v2-metadata-onchain`
- `rg -n "native_assets|native_asset_template|TokenNativeAssets|SetNativeAssetTemplate|SetTokenNativeAssetOverride|ClearTokenNativeAssetOverride|NativeAssetTemplate" docs contracts/nft schema -g "*.md" -g "*.json"`

## Human Verification

None required.

## Gaps

None.
