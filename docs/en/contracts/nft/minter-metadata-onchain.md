# minter-metadata-onchain

## Status

Compatibility

## Purpose

Represents the older metadata-onchain primary-sale surface retained for compatibility and reference.

## Instantiation

Instantiate with the older metadata-onchain collection assumptions and sale configuration used by this minter generation.

Real payload source of truth: `contracts/nft/minter-metadata-onchain/src/msg.rs` and `schema/` when available.

## Actors and permissions

Drop admin, buyers, deployed collection, and the payout configuration associated with the older metadata-onchain path.

## Key messages

- Runs an older primary-sale path for on-chain metadata collections.
- Bridges sale logic into the corresponding collection deployment.
- Mostly matters for compatibility, migration understanding, or historical context.

## Relationships

- Pairs conceptually with older metadata-onchain collection choices.
- Should be compared to `minter-v2-metadata-onchain` when documenting current versus older paths.
- Still fits into the broader collection and resale lifecycle.

## Hypothetical example

Hypothetical flow: an older deployment kept fully on-chain metadata in the primary sale path and still documents this contract for compatibility reasons.

## References

- Code: `contracts/nft/minter-metadata-onchain/src/msg.rs`
- Local context: `contracts/nft/minter-metadata-onchain/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
