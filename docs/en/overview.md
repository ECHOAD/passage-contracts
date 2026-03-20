# Overview

## Goal

Passage uses this workspace for on-chain contracts covering registration, NFT collections, secondary commerce, revenue routing, governance, and supporting primitives. Platform UX, rendering, streaming, search, and analytics remain off-chain.

## Active families

- `core`: `registry`, `ecosystem-factory`, `collection-factory`, `multisig`, `pasg-governance`, `split-router`, `streaming-billing`
- `nft`: `pg721`, `pg721-updatable`, `pg721-metadata-onchain`, `minter-v2`, `marketplace-v3`, `auction-english`, `royalty-group`, `whitelist`, plus compatibility surfaces
- `relationship`: `follow`, `friend`
- `staking`: `nft-vault`, `stake-rewards`, `vault-factory`

## Historical material

Explicit legacy contracts live under `contracts/legacy` in this documentation library. They are preserved as historical or compatibility reference, not as the preferred target for new integrations.

## Reading rule

- To understand registration and ecosystem organization, start with `contracts/core/registry.md`.
- To understand collections and minting, continue with `contracts/nft/pg721.md` and `contracts/nft/minter-v2.md`.
- To understand secondary commerce, read `contracts/nft/marketplace-v3.md` and `contracts/nft/auction-english.md`.
- To understand admin and governance, read `contracts/core/multisig.md` and `contracts/core/pasg-governance.md`.
