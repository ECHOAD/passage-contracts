# minter-v2

## Status

Current

## Purpose

Handles the current primary-sale minting flow for an already-existing collection.

## Instantiation

Instantiate with the target collection address, sale configuration, admin/operator controls, and any linked payout or registry dependencies required by the drop flow.

Real payload source of truth: `contracts/nft/minter-v2/src/msg.rs` and `schema/` when available.

## Actors and permissions

Drop admin, creator, buyers, the existing collection contract, registry, and payout routes.

## Key messages

- Manages a collection-specific primary mint sale.
- Controls mint windows and sale configuration.
- Routes primary-sale revenue while staying compatible with later registry and marketplace flows.

## Relationships

- Points into an existing `pg721` or a related collection variant.
- May feed proceeds into split or royalty-aware payout paths.
- The resulting collection should still be visible to registry for the broader protocol lifecycle.

## Hypothetical example

Hypothetical flow: a creator or ecosystem admin first registers a collection, authorizes a `minter-v2` instance for it in `registry`, and buyers then mint in primary sale before the collection later enters secondary sale through marketplace-v3.

## References

- Code: `contracts/nft/minter-v2/src/msg.rs`
- Local context: `contracts/nft/minter-v2/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
