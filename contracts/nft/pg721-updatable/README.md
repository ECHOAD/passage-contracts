# pg721-updatable

`pg721-updatable` is the Passage collection contract for typed creator assets whose token URI must remain editable until the creator freezes it.

## Supported Collection Types

- `component`
- `avatar`
- `companion`
- `world`
- `plugin`
- `achievement`
- `world_template`

## Typed Metadata

It shares the same typed asset surface as `pg721`:

- `Component(ComponentExtension)`
- `Avatar(AvatarExtension)`
- `Companion(CompanionExtension)`
- `World(WorldExtension)`
- `Plugin(PluginExtension)`
- `Achievement(AchievementExtension)`
- `WorldTemplate(WorldTemplateExtension)`

The contract validates that token metadata and typed extensions match the collection `nft_type`.

## Additional Capabilities

- `UpdateTokenMetadata`
- `FreezeTokenMetadata`
- `FrozenTokenMetadata`

Those controls affect mutable token URI behavior only. Asset runtime/rendering payloads still stay off-chain.

## Monetization Surface

- Collection-level `royalty_info` remains queryable through `CollectionInfo`.
- World tokens can carry `revenue_shares` in `WorldExtension`.
- Secondary sale contracts continue to use generic split routing and royalty reads.
