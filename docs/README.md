# Passage Commerce Docs

This folder documents the current Passage on-chain commerce flow from `registry` to either a fixed-price sale or an auction, plus the corrected PASG governance and validator staking boundaries.

Recommended reading order:

1. `01-end-to-end-setup.md`
2. `02-method-reference.md`
3. `03-json-examples.md`
4. `04-multisig-governance.md`

## Quick map

- `registry`: source of truth for ecosystems, collections, authorized minters, and optional Passage validator metadata for staking UX.
- `ecosystem-factory`: governed flow for creating ecosystems and deploying their `collection-factory`.
- `collection-factory`: deploys `pg721` collections inside an ecosystem and registers them in `registry`.
- `pg721`: base NFT collection contract.
- `split-router`: routes creator-side mint and royalty proceeds.
- `marketplace-v3`: fixed-price secondary sales, bids, and collection bids.
- `auction-english`: reserve-style NFT auctions.
- `minter-v2`: optional primary sale flow; deploys its own `pg721`.
- PASG validator staking: chain-native delegation to Passage validators, documented here but not implemented as a duplicate staking vault in this repo.

## Staking path

For PASG staking, use the docs in this order:

1. `01-end-to-end-setup.md` for the high-level boundary and operational note.
2. `02-method-reference.md` for the native staking model and optional `registry` metadata surface.
3. `03-json-examples.md` for conceptual CLI examples, reward-state queries, undelegation queries, and `registry` validator metadata payloads.
4. `04-multisig-governance.md` for the narrow governance boundary around staking metadata and policy edges.

What these docs assume:

- Delegation, undelegation, redelegation, reward-state, and unbonding remain chain-native Passage staking behavior.
- `registry` may hold curated validator metadata for UX and policy purposes only.
- `contracts/staking/*` remain NFT staking contracts unless a later scoped adapter is explicitly documented.

## Naming note

The contract package and its physical directory now use the same name:

- `contracts/core/split-router`

## Two collection paths

There are two ways to end up with an operable collection:

1. `registry` -> `ecosystem-factory` / `collection-factory` -> `pg721`
   Recommended when you already control minting or want curated collections inside an ecosystem.

2. `minter-v2` -> `pg721`
   Recommended for primary drops. In this path the minter deploys its own collection and that collection must then be registered in `registry`.

## Final outcome

With the current stack you can end up with two secondary sale channels:

- `marketplace-v3` for fixed-price sales, bids, and collection bids.
- `auction-english` for reserve auctions.

If you want primary sales, the correct contract is `minter-v2`, not `marketplace-v3` or `auction-english`.

## Ready-to-use payloads

For concrete `instantiate`, `execute`, and selected `query` payloads, see:

- `03-json-examples.md`

For the recommended multisig governance flow and proposal examples against `registry`, see:

- `04-multisig-governance.md`
