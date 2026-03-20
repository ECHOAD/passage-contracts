# pg721-updatable

## Estado

Actual

## Proposito

Extiende el modelo de coleccion tipada de Passage con semanticas de metadata actualizable bajo control.

## Instanciacion

Se instancia con ownership de coleccion, royalties y metadata de tipo NFT, mas los permisos de update que definen quien puede cambiar campos mutables despues.

Fuente real de payload: `contracts/nft/pg721-updatable/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin de la coleccion, token holders, marketplaces, registry y cualquier flujo que necesite metadata mutable de coleccion o token.

## Mensajes clave

- Soporta la linea base de coleccion tipada con updates controlados de metadata.
- Mantiene la metadata del token limitada a `nft_type` mas la extension tipada de Passage; ya no expone el arreglo retirado de attachments.
- Mantiene el mismo modelo de clasificacion creator-asset que la coleccion base.
- Sigue siendo compatible con registration y comercio secundario.

## Relaciones

- Aparece cuando Passage necesita metadata mutable sin abandonar el modelo principal de coleccion.
- Comparte el mismo contexto de ecosystem, registry y marketplace que `pg721`.
- Debe documentarse junto a `pg721` para explicar por que existe una variante actualizable.

## Ejemplo hipotetico

Flujo hipotetico: una coleccion necesita updates controlados de metadata despues del mint, por lo que el ecosystem despliega `pg721-updatable` en vez del contrato base completamente estatico.

## Referencias

- Codigo: `contracts/nft/pg721-updatable/src/msg.rs`
- Contexto local: `contracts/nft/pg721-updatable/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
