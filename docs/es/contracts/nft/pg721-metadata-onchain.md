# pg721-metadata-onchain

## Estado

Actual / Compatibilidad

## Proposito

Representa la variante de coleccion donde la metadata vive on-chain en lugar de depender principalmente de referencias off-chain.

## Instanciacion

Se instancia con ownership, modelo de metadata tipada, supuestos de royalty y la estructura de metadata on-chain que la coleccion persistira.

Fuente real de payload: `contracts/nft/pg721-metadata-onchain/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin de la coleccion, minters, token holders y cualquier flujo que prefiera persistencia on-chain de metadata.

## Mensajes clave

- Almacena el comportamiento de coleccion para NFTs con metadata on-chain.
- Funciona con flujos de venta primaria y secundaria que necesitan esta variante de metadata.
- Preserva las mismas relaciones amplias de registro y tradeability que otras variantes pg721.

## Relaciones

- Se empareja de forma natural con variantes de minter metadata-onchain.
- Aun puede registrarse, tradearse y moderarse por las mismas superficies generales del protocolo.
- Debe contrastarse con `pg721` y `pg721-updatable` al explicar elecciones de coleccion.

## Ejemplo hipotetico

Flujo hipotetico: un creator quiere que la metadata de coleccion y token viva directamente on-chain y elige esta variante antes de conectarla con registry y comercio.

## Referencias

- Codigo: `contracts/nft/pg721-metadata-onchain/src/msg.rs`
- Contexto local: `contracts/nft/pg721-metadata-onchain/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
