# marketplace-v3

## Status

Current

## Purpose

Provides the current multi-collection secondary marketplace for asks, bids, collection bids, fees, and royalty-aware resale.

## Instantiation

Instantiate with admin, registry, fee collector, min price, global trading fee, and any operator accounts that can moderate marketplace behavior.

Real payload source of truth: `contracts/nft/marketplace-v3/src/msg.rs` and `schema/` when available.

## Actors and permissions

Collection owners, buyers, bidders, marketplace admin, registry, royalty recipients, and fee collector.

## Key messages

- Registers collections and tracks collection-scoped settlement denom.
- Supports asks, direct purchases, token bids, and collection bids.
- Calculates marketplace fee and royalty-aware sale preview and settlement.

## Relationships

- Depends on registry moderation and collection tradeability.
- Consumes PASG semantics indirectly through streaming-billing when integrators need the canonical PASG view.
- Coexists with auction-english as the fixed-price secondary-sale path.

## Hypothetical example

Hypothetical flow: a registered collection lists token 1 for `upasg`, a buyer uses `buy_now`, and the contract enforces global marketplace fee plus royalty distribution.

## References

- Code: `contracts/nft/marketplace-v3/src/msg.rs`
- Local context: `contracts/nft/marketplace-v3/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
