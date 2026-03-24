# Passage Commerce Docs

This folder now has two documentation layers:

1. The original operational guides:
   - `01-end-to-end-setup.md`
   - `02-method-reference.md`
   - `03-json-examples.md`
   - `04-multisig-governance.md`
2. The mirrored bilingual contract library:
   - `docs/es/`
   - `docs/en/`

## Bilingual contract library

Use the new mirrored trees when you want contract-by-contract documentation, contract relationships, instantiate guidance, and hypothetical end-to-end examples.

- Spanish: `docs/es/README.md`
- English: `docs/en/README.md`

## What the bilingual tree covers

- One page per contract crate in the workspace
- Shared relationship and protocol flow explanations
- Instantiation guidance and deployment ordering notes
- Historical/reference treatment for explicit legacy contracts

## Existing quick map

- `registry`: source of truth for ecosystems, collections, and authorized minters.
- `ecosystem-factory`: governed flow for creating ecosystems and deploying their `collection-factory`.
- `collection-factory`: deploys `pg721` collections inside an ecosystem and registers them in `registry`.
- `pg721`: base NFT collection contract.
- `split-router`: routes creator-side mint and royalty proceeds.
- `marketplace-v3`: fixed-price secondary sales, bids, and collection bids.
- `auction-english`: reserve-style NFT auctions.
- `minter-v2`: optional primary sale flow for an already deployed `pg721`.
