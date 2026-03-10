# Passage Commerce Docs

Esta carpeta documenta el flujo on-chain actual de Passage desde `registry` hasta una venta por precio fijo o por subasta.

Orden recomendado de lectura:

1. `01-end-to-end-setup.md`
2. `02-method-reference.md`
3. `03-json-examples.md`

## Mapa rapido

- `registry`: fuente de verdad para ecosystems, collections y minters autorizados.
- `ecosystem-factory`: flujo gobernado para crear ecosystems y desplegar su `collection-factory`.
- `collection-factory`: despliega colecciones `pg721` dentro de un ecosystem y las registra en `registry`.
- `pg721`: contrato NFT base de la coleccion.
- `split-router`: distribuye fondos creator-side para mint y royalties.
- `marketplace-v3`: ventas secundarias por precio fijo, bids y collection bids.
- `auction-english`: subastas reserve-style por NFT.
- `minter-v2`: flujo opcional de venta primaria; despliega su propio `pg721`.

## Nota de naming

El contrato se llama `split-router`, pero el directorio fisico aun es:

- `contracts/core/revenue-router`

La documentacion usa el nombre funcional `split-router`, porque asi esta expuesto hoy el paquete y la integracion.

## Dos caminos de coleccion

Hay dos formas de llegar a una coleccion operable:

1. `registry` -> `ecosystem-factory` / `collection-factory` -> `pg721`
   Uso recomendado cuando ya controlas la emision o quieres colecciones curadas dentro de un ecosystem.

2. `minter-v2` -> `pg721`
   Uso recomendado para drops primarios. En este camino el minter crea su propia coleccion y luego debe registrarse en `registry`.

## Resultado final

Con el stack actual puedes terminar en dos canales de venta secundaria:

- `marketplace-v3` para fixed price sale, bids y collection bids.
- `auction-english` para reserve auction.

Si quieres venta primaria, el contrato correcto es `minter-v2`, no `marketplace-v3` ni `auction-english`.

## Ejemplos listos para usar

Si quieres payloads concretos de `instantiate`, `execute` y algunas `query`, revisa:

- `03-json-examples.md`
