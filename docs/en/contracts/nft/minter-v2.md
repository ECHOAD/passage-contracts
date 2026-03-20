# minter-v2

## Status

Current

## Purpose

Handles the current primary-sale minting flow and can deploy its own collection for drop-style launches.

## Instantiation

Instantiate with collection deployment parameters, sale configuration, admin/operator controls, and any linked payout or registry dependencies required by the drop flow.

Real payload source of truth: `contracts/nft/minter-v2/src/msg.rs` and `schema/` when available.

## Actors and permissions

Drop admin, creator, buyers, the deployed collection contract, registry, and payout routes.

## Key messages

- Deploys or manages a collection for primary mint sales.
- Controls mint windows and sale configuration.
- Routes primary-sale revenue while staying compatible with later registry and marketplace flows.

## Relationships

- Usually points into `pg721` or a related collection variant.
- May feed proceeds into split or royalty-aware payout paths.
- The resulting collection should still be visible to registry for the broader protocol lifecycle.

## Hypothetical example

Hypothetical flow: a creator launches a new drop through `minter-v2`, buyers mint in primary sale, and the resulting collection later enters secondary sale through marketplace-v3.

## References

- Code: `contracts/nft/minter-v2/src/msg.rs`
- Local context: `contracts/nft/minter-v2/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
