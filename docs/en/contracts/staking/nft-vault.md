# nft-vault

## Status

Current

## Purpose

Provides NFT staking with unstake/claim timing and reward-account integration.

## Instantiation

Instantiate with approved collections, reward-account code or links, and unstaking duration settings.

Real payload source of truth: `contracts/staking/nft-vault/src/msg.rs` and `schema/` when available.

## Actors and permissions

NFT stakers, vault admin, reward accounts, and claimers.

## Key messages

- Accepts NFT staking and unstaking flows.
- Tracks claim windows for unstaked NFTs.
- Coordinates with reward modules for reward claiming.

## Relationships

- Works with `stake-rewards` and `vault-factory`.
- Belongs to NFT staking, not chain-native PASG validator staking.
- Can attach to approved NFT collections.

## Hypothetical example

Hypothetical flow: a user stakes NFTs from an approved collection into `nft-vault`, waits through the unstake period, and later claims the NFTs back.

## References

- Code: `contracts/staking/nft-vault/src/msg.rs`
- Local context: `contracts/staking/nft-vault/README.md`
- Shared guide: `docs/en/relationships.md`
