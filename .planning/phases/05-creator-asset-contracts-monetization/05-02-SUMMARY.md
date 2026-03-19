# 05-02 Summary

## Outcome

Wave 2 completed the shared typed creator asset surface and kept creator monetization on generic routing paths.

## What Changed

- Added first-class typed metadata support for `plugin`, `achievement`, and `world_template` in `pg721` and `pg721-updatable`.
- Added deterministic mint tests for the new variants in both collection contracts.
- Kept royalty and revenue semantics contract-facing through collection metadata instead of introducing NFT-type-specific router branches.
- Left runtime and rendering payloads off-chain.

## Verification

- `cargo check -p pg721 -p pg721-updatable`
- `cargo test -p pg721 --lib`
- `cargo test -p pg721-updatable --lib`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p auction-english --lib`

## Notes

- `WorldExtension` remains the place for `revenue_shares`.
- Marketplace and auction settlement continue to route royalties generically through collection metadata and `Split {}` where needed.
