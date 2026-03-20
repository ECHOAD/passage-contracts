# avatar-progression

## Status

Current

## Purpose

Documents the dedicated Passage progression-state surface implemented by the `asset-progression` crate.

This contract keeps mutable avatar and companion progression snapshots out of generic NFT metadata. The NFT remains the durable identity and rights surface, while progression snapshots are persisted separately at save points.

## Instantiation

Instantiate with:

- `InstantiateMsg { admin }`

Real payload source of truth: `contracts/nft/asset-progression/src/msg.rs` and `contracts/nft/asset-progression/schema/`.

## Actors and permissions

- Admin can rotate contract administration.
- Snapshot writers must be the live NFT owner or an active CW721 approval for the asset.
- Supported assets are Passage avatars and companions only.

## Key messages

- `SaveSnapshot`: persist a world-scoped progression snapshot for one asset.
- `UpdateAdmin`: rotate the contract admin.

## Key queries

- `Config`
- `Snapshot`
- `SnapshotsByAsset`
- `SnapshotsByWorld`

## Boundary notes

- Progression is a dedicated state surface, not generic NFT metadata.
- Save-point persistence is on-chain; gameplay formulas and runtime logic remain world-defined and off-chain.
- Rendering details, compatibility matrices, and other manifest-heavy payloads still live behind the NFT `token_uri`.

## Relationships

- Resolves ownership and approvals against the source `pg721` or `pg721-updatable` collection.
- Complements avatar and companion collections without expanding their typed metadata boundary.
- Can be composed with client-side manifest reads so users see one asset plus its latest progression snapshot.

## Hypothetical example

Hypothetical flow: a world saves an avatar checkpoint after a gameplay session, then indexers and clients read the NFT for durable identity and `asset-progression` for the latest world-scoped snapshot.

## References

- Code: `contracts/nft/asset-progression/src/msg.rs`
- Local context: `contracts/nft/asset-progression/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
