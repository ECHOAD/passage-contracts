# 10-01 Summary

## Outcome

Removed `native_assets` from the active NFT metadata message surfaces and aligned the shared compatibility structs.

## Key Changes

- `pg721-updatable::TokenMetadata` now carries only `nft_type` and typed `extension`.
- `pg721-metadata-onchain::Metadata` no longer exposes the retired attachment array.
- Dead `NativeAsset` / `native_assets` compatibility fields were removed from `pg721` and `minter-v2` shared state types.
- `pg721-updatable` tests were updated to mint against the reduced metadata shape.

## Verification

- `cargo test -p pg721-updatable --lib`
- `cargo check -p pg721-updatable -p pg721-metadata-onchain`
