# royalty-group

## Status

Current / Supporting

## Purpose

Provides a grouped royalty recipient surface so royalties can be split across multiple parties.

## Instantiation

Instantiate with the member payout configuration and any admin permissions needed to maintain the royalty group definition.

Real payload source of truth: `contracts/nft/royalty-group/src/msg.rs` and `schema/` when available.

## Actors and permissions

Royalty recipients, collection or sale contracts that send royalties here, and the admin who manages the group definition.

## Key messages

- Defines a royalty recipient group.
- Receives royalty funds and redistributes them.
- Lets NFT commerce contracts point to one recipient contract instead of many direct addresses.

## Relationships

- Can sit behind marketplace or auction royalty payout.
- Complements collection-level royalty configuration.
- Often works alongside split-router when more complex payout routing is needed.

## Hypothetical example

Hypothetical flow: marketplace-v3 sends royalty funds to `royalty-group`, which then redistributes the amount across a creator team.

## References

- Code: `contracts/nft/royalty-group/src/msg.rs`
- Local context: `contracts/nft/royalty-group/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
