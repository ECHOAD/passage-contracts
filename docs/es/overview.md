# Overview

## Objetivo

Passage usa este workspace para contratos on-chain de registro, colecciones NFT, comercio secundario, revenue routing, governance y primitivas auxiliares. La experiencia de plataforma, rendering, streaming, search y analytics siguen fuera de cadena.

## Familias activas

- `core`: `registry`, `ecosystem-factory`, `collection-factory`, `multisig`, `pasg-governance`, `split-router`, `streaming-billing`
- `nft`: `pg721`, `pg721-updatable`, `pg721-metadata-onchain`, `minter-v2`, `marketplace-v3`, `auction-english`, `royalty-group`, `whitelist`, y superficies de compatibilidad
- `relationship`: `follow`, `friend`
- `staking`: `nft-vault`, `stake-rewards`, `vault-factory`

## Material historico

Los contratos explicitamente legacy viven en `contracts/legacy` dentro de esta libreria documental. Se mantienen como referencia historica o compatibilidad, no como recomendacion principal para nuevas integraciones.

## Regla de lectura

- Si quieres entender como se registran y se organizan los assets: empieza por `contracts/core/registry.md`.
- Si quieres entender minting y colecciones: sigue por `contracts/nft/pg721.md` y `contracts/nft/minter-v2.md`.
- Si quieres entender mercado secundario: lee `contracts/nft/marketplace-v3.md` y `contracts/nft/auction-english.md`.
- Si quieres entender admin y governance: lee `contracts/core/multisig.md` y `contracts/core/pasg-governance.md`.
