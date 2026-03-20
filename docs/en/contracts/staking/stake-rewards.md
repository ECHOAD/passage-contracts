# stake-rewards

## Status

Current

## Purpose

Distributes external rewards to users based on stake state reported by an authorized stake contract.

## Instantiation

Instantiate with the authorized stake contract, reward denom, and reward-duration parameters.

Real payload source of truth: `contracts/staking/stake-rewards/src/msg.rs` and `schema/` when available.

## Actors and permissions

Authorized stake contract, stakers, reward claimers, and admin/config managers.

## Key messages

- Updates reward accounting when stake changes.
- Lets users claim accumulated rewards.
- Depends on stake state from another contract instead of owning stake itself.

## Relationships

- Typically sits behind `nft-vault`.
- Is part of the NFT staking family.
- Should not be confused with chain-native PASG staking rewards.

## Hypothetical example

Hypothetical flow: `nft-vault` reports stake changes into `stake-rewards`, and users later claim reward tokens calculated from that stake history.

## References

- Code: `contracts/staking/stake-rewards/src/msg.rs`
- Local context: `contracts/staking/stake-rewards/README.md`
- Shared guide: `docs/en/relationships.md`
