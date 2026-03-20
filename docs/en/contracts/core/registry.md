# registry

## Status

Current

## Purpose

Acts as the canonical ledger for ecosystems, collection affiliation, creator provenance, and minter/trade permissions.

## Instantiation

Instantiate with admin control, moderation and recovery configuration, and the ecosystem/collection policy defaults that define canonical registry behavior.

Real payload source of truth: `contracts/core/registry/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Protocol admin, ecosystem admins, approved ecosystem members, creators, minters, marketplaces, and query consumers.

## Key messages

- Registers ecosystems and collections.
- Tracks membership, deregistration, and re-homing flows.
- Authorizes minting and tradeability checks that downstream commerce contracts rely on.

## Relationships

- Receives canonical writes from `ecosystem-factory` and `collection-factory`.
- Feeds permission and affiliation data into `marketplace-v3`, `auction-english`, and mint flows.
- Stores collection-level NFT type and provenance information used across the protocol.

## Hypothetical example

Hypothetical flow: a collection is re-homed from one ecosystem to another; the contract address stays the same, but registry updates its ecosystem affiliation while preserving creator provenance.

## References

- Code: `contracts/core/registry/src/msg.rs`
- Local context: `contracts/core/registry/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
