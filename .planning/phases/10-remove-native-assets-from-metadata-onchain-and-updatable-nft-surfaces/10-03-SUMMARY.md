# 10-03 Summary

## Outcome

Aligned the published contract surface after the cleanup by regenerating schemas and updating the public NFT/minter docs.

## Key Changes

- Regenerated schema outputs for `pg721-updatable`, `pg721-metadata-onchain`, and `minter-v2-metadata-onchain`.
- Removed retired native-asset template/query references from `docs/02-method-reference.md`.
- Updated English and Spanish contract pages to describe the reduced typed metadata surface without attachment APIs.

## Verification

- `cargo run --example schema -p pg721-updatable`
- `cargo run --example schema -p pg721-metadata-onchain`
- `cargo run --example schema -p minter-v2-metadata-onchain`
- `cargo check -p pg721-updatable -p pg721-metadata-onchain -p minter-v2-metadata-onchain`
- `rg -n "native_assets|native_asset_template|TokenNativeAssets|SetNativeAssetTemplate|SetTokenNativeAssetOverride|ClearTokenNativeAssetOverride|NativeAssetTemplate" docs contracts/nft schema -g "*.md" -g "*.json"`
