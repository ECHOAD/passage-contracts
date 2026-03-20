# vault-factory

## Status

Current

## Purpose

Deploys NFT vault instances and related reward infrastructure for NFT staking setups.

## Instantiation

Instantiate with vault code IDs, reward-module deployment parameters, and the admin/operator surface allowed to create new vaults.

Real payload source of truth: `contracts/staking/vault-factory/src/msg.rs` and `schema/` when available.

## Actors and permissions

Admin/operator, deployed vaults, reward accounts, and users who later stake into those vaults.

## Key messages

- Creates vault instances.
- Coordinates deployment of supporting reward modules.
- Standardizes NFT staking setup across collections or programs.

## Relationships

- Sits upstream from `nft-vault` and `stake-rewards`.
- Exists only inside the NFT staking family.
- Does not overlap with validator delegation for PASG.

## Hypothetical example

Hypothetical flow: an operator uses `vault-factory` to deploy a new NFT staking vault program instead of instantiating every staking component by hand.

## References

- Code: `contracts/staking/vault-factory/src/msg.rs`
- Local context: `contracts/staking/vault-factory/README.md`
- Shared guide: `docs/en/relationships.md`
