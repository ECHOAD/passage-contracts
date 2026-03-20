# pg721-updatable

## Status

Current

## Purpose

Extends the typed Passage collection model with controlled manifest-pointer updates.

## Instantiation

Instantiate with collection ownership, royalty and NFT-type metadata, plus the permissions that control later `token_uri` updates.

Real payload source of truth: `contracts/nft/pg721-updatable/src/msg.rs` and `schema/` when available.

## Actors and permissions

Collection admin, token holders, marketplaces, registry, and any flow that needs a mutable manifest pointer without mutating typed NFT semantics.

## Key messages

- Supports the typed collection baseline with controlled token URI updates.
- Keeps token metadata scoped to `nft_type` plus typed Passage extension data; it no longer advertises the retired attachment array.
- Keeps the same creator-asset classification model as the base collection.
- Remains compatible with registration and secondary-commerce flows.
- `UpdateTokenMetadata` changes `token_uri` only; typed extension fields remain frozen at mint-time semantics.
- Rendering/runtime payloads, detailed compatibility rules, and world-specific manifests stay off-chain behind `token_uri`.

## Relationships

- Often appears where Passage wants mutable metadata without abandoning the main collection model.
- Shares the same surrounding ecosystem, registry, and marketplace relationships as `pg721`.
- Should be documented alongside `pg721` to explain why an updatable variant exists.
- Uses dedicated surfaces like `asset-progression` or `world-plugin-assignment` when mutable protocol state must remain queryable on-chain.

## Hypothetical example

Hypothetical flow: a creator updates the manifest referenced by `token_uri` after refreshing avatar visuals, while ownership semantics and typed protocol fields stay unchanged on-chain.

## References

- Code: `contracts/nft/pg721-updatable/src/msg.rs`
- Local context: `contracts/nft/pg721-updatable/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
