# auction-english

## Status

Current

## Purpose

Runs reserve-style NFT auctions for registered collections and handles bid competition and final settlement.

## Instantiation

Instantiate with admin controls, fee surface, registry linkage, and the auction rules needed for reserve-style sales.

Real payload source of truth: `contracts/nft/auction-english/src/msg.rs` and `schema/` when available.

## Actors and permissions

NFT owner, bidders, admin/operators, registry, royalty recipients, and settlement recipients.

## Key messages

- Creates and manages reserve auctions.
- Accepts bids and handles displaced or losing bidder refund-safe behavior.
- Settles the winning auction into seller proceeds, fee, and royalty outputs.

## Relationships

- Depends on collection ownership and approval state.
- Uses registry-facing trade assumptions for valid collections.
- Overlaps with marketplace-v3 as another secondary-commerce path, but for auctions rather than fixed-price sale.

## Hypothetical example

Hypothetical flow: a creator-owned world NFT is listed in an English auction, bidders compete, and the winner settles while the contract routes fee and royalties correctly.

## References

- Code: `contracts/nft/auction-english/src/msg.rs`
- Local context: `contracts/nft/auction-english/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
