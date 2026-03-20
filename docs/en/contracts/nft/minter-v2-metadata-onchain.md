# minter-v2-metadata-onchain

## Status

Current / Compatibility

## Purpose

Provides a metadata-onchain primary sale variant aligned with the v2 minting surface.

## Instantiation

Instantiate with the same primary-sale assumptions as `minter-v2`, plus the collection variant or metadata expectations needed for on-chain metadata.

Real payload source of truth: `contracts/nft/minter-v2-metadata-onchain/src/msg.rs` and `schema/` when available.

## Actors and permissions

Drop admin, buyers, deployed collection, registry, and any payout route used in the sale.

## Key messages

- Runs a v2-style mint flow with on-chain metadata assumptions.
- Bridges the primary-sale layer into the collection contract.
- Keeps compatibility with later registration and trading flows.

## Relationships

- Pairs naturally with `pg721-metadata-onchain`.
- Follows the same broader lifecycle as `minter-v2`.
- Should be read together with the collection page for exact metadata semantics.

## Hypothetical example

Hypothetical flow: a creator wants fully on-chain metadata for a primary drop and chooses the v2 metadata-onchain minter path before later resale.

## References

- Code: `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`
- Local context: `contracts/nft/minter-v2-metadata-onchain/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
