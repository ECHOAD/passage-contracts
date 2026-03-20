# pg721-updatable

## Status

Current

## Purpose

Extends the typed Passage collection model with controlled updatable metadata semantics.

## Instantiation

Instantiate with collection ownership, royalty and NFT-type metadata, plus the update permissions that define who may change mutable fields later.

Real payload source of truth: `contracts/nft/pg721-updatable/src/msg.rs` and `schema/` when available.

## Actors and permissions

Collection admin, token holders, marketplaces, registry, and any flow that needs mutable collection or token metadata.

## Key messages

- Supports the typed collection baseline with controlled metadata updates.
- Keeps token metadata scoped to `nft_type` plus typed Passage extension data; it no longer advertises the retired attachment array.
- Keeps the same creator-asset classification model as the base collection.
- Remains compatible with registration and secondary-commerce flows.

## Relationships

- Often appears where Passage wants mutable metadata without abandoning the main collection model.
- Shares the same surrounding ecosystem, registry, and marketplace relationships as `pg721`.
- Should be documented alongside `pg721` to explain why an updatable variant exists.

## Hypothetical example

Hypothetical flow: a collection needs controlled metadata updates after mint, so the ecosystem deploys `pg721-updatable` rather than the fully static base contract.

## References

- Code: `contracts/nft/pg721-updatable/src/msg.rs`
- Local context: `contracts/nft/pg721-updatable/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
