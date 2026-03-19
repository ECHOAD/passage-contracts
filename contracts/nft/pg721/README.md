# pg721

`pg721` is the shared Passage collection contract for typed creator assets.

## Supported Collection Types

- `component`
- `avatar`
- `companion`
- `world`
- `plugin`
- `achievement`
- `world_template`

## Typed Metadata

The contract enforces that token metadata matches the collection `nft_type`.

- `Component(ComponentExtension)`
- `Avatar(AvatarExtension)`
- `Companion(CompanionExtension)`
- `World(WorldExtension)`
- `Plugin(PluginExtension)`
- `Achievement(AchievementExtension)`
- `WorldTemplate(WorldTemplateExtension)`

These extensions are intentionally compact and contract-facing. Runtime payloads, Unreal assets, rendering graphs, and similar execution details stay off-chain.

## Monetization Surface

- Collection-level `royalty_info` is queryable through `CollectionInfo`.
- World tokens can carry `revenue_shares` in `WorldExtension`.
- Sale paths such as `marketplace-v3` and `auction-english` keep routing generic and read royalty semantics from collection metadata.

## Main Queries

- `CollectionInfo`
- `NftInfo`
- `AllNftInfo`
- `OwnerOf`
- `Tokens`
- `AllTokens`

## Notes

- The collection contract address is the canonical collection identity in `registry`.
- `pg721` does not model runtime behavior. It models enforceable ownership, approvals, royalties, and typed creator-asset metadata.
