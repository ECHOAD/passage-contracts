# Passage Commerce Docs

This folder documents the current Passage on-chain commerce flow from `registry` to either a fixed-price sale or an auction.

Recommended reading order:

1. `01-end-to-end-setup.md`
2. `02-method-reference.md`
3. `03-json-examples.md`
4. `04-multisig-governance.md`

## Quick map

- `registry`: source of truth for ecosystems, collections, and authorized minters.
- `ecosystem-factory`: governed flow for creating ecosystems and deploying their `collection-factory`.
- `collection-factory`: deploys `pg721` collections inside an ecosystem and registers them in `registry`.
- `pg721`: base NFT collection contract.
- `split-router`: routes creator-side mint and royalty proceeds.
- `marketplace-v3`: fixed-price secondary sales, bids, and collection bids.
- `auction-english`: reserve-style NFT auctions.
- `minter-v2`: optional primary sale flow; deploys its own `pg721`.

## Naming note

The contract is named `split-router`, but its physical directory is still:

- `contracts/core/revenue-router`

This documentation uses the functional name `split-router`, because that is how the package and integrations are exposed today.

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
