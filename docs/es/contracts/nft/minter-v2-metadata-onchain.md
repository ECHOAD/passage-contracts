# minter-v2-metadata-onchain

## Estado

Actual / Compatibilidad

## Proposito

Provee una variante de venta primaria con metadata on-chain alineada con la superficie de minting v2.

## Instanciacion

Se instancia con los mismos supuestos de venta primaria que `minter-v2`, mas la variante de coleccion o expectativas de metadata necesarias para metadata on-chain.

Fuente real de payload: `contracts/nft/minter-v2-metadata-onchain/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin del drop, buyers, coleccion desplegada, registry y cualquier ruta de payout usada en la venta.

## Mensajes clave

- Ejecuta un flujo de mint estilo v2 con supuestos de metadata on-chain.
- Emite solo metadata tipada reducida; ya no expone APIs de template ni overrides para native assets.
- Conecta la capa de venta primaria con el contrato de coleccion.
- Mantiene compatibilidad con el registro y trading posteriores.

## Relaciones

- Se empareja de forma natural con `pg721-metadata-onchain`.
- Sigue el mismo lifecycle amplio que `minter-v2`.
- Debe leerse junto con la pagina de la coleccion para entender la semantica exacta de metadata.

## Ejemplo hipotetico

Flujo hipotetico: un creator quiere metadata completamente on-chain para un drop primario y elige la ruta del minter v2 metadata-onchain antes de la reventa posterior.

## Referencias

- Codigo: `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`
- Contexto local: `contracts/nft/minter-v2-metadata-onchain/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
