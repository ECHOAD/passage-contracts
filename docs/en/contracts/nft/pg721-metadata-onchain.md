# pg721-metadata-onchain

## Status

Current / Compatibility

## Purpose

Represents the collection variant where metadata lives on-chain rather than being primarily off-chain referenced.

## Instantiation

Instantiate with ownership, typed metadata model, royalty assumptions, and the on-chain metadata structure the collection will persist.

Real payload source of truth: `contracts/nft/pg721-metadata-onchain/src/msg.rs` and `schema/` when available.

## Actors and permissions

Collection admin, minters, token holders, and any flow that prefers on-chain metadata persistence.

## Key messages

- Stores collection behavior for metadata-onchain NFTs.
- Works with primary and secondary sale flows that need this metadata flavor.
- Preserves the same broader registration and tradeability relationships as other pg721 variants.

## Relationships

- Pairs naturally with metadata-onchain minter variants.
- Can still be registered, traded, and moderated through the same broader protocol surfaces.
- Should be contrasted with `pg721` and `pg721-updatable` when explaining collection choices.

## Hypothetical example

Hypothetical flow: a creator wants collection and token metadata stored directly on-chain and chooses this collection variant before plugging into registry and commerce.

## References

- Code: `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- Local context: `contracts/nft/pg721-metadata-onchain/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
