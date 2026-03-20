# pg721-updatable

## Estado

Actual

## Proposito

Extiende el modelo de coleccion tipada de Passage con updates controlados del puntero de manifiesto.

## Instanciacion

Se instancia con ownership de coleccion, royalties, metadata de tipo NFT y los permisos que controlan updates posteriores de `token_uri`.

Fuente real de payload: `contracts/nft/pg721-updatable/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin de la coleccion, token holders, marketplaces, registry y cualquier flujo que necesite un manifiesto mutable sin alterar la semantica tipada del NFT.

## Mensajes clave

- Soporta la linea base de coleccion tipada con updates controlados de metadata.
- Mantiene la metadata del token limitada a `nft_type` mas la extension tipada de Passage; ya no expone el arreglo retirado de attachments.
- Mantiene el mismo modelo de clasificacion creator-asset que la coleccion base.
- Sigue siendo compatible con registration y comercio secundario.
- `UpdateTokenMetadata` cambia solo `token_uri`; los campos tipados quedan congelados con la semantica del mint.
- Render, reglas de compatibilidad detalladas y payloads runtime siguen off-chain detras de `token_uri`.

## Relaciones

- Aparece cuando Passage necesita metadata mutable sin abandonar el modelo principal de coleccion.
- Comparte el mismo contexto de ecosystem, registry y marketplace que `pg721`.
- Debe documentarse junto a `pg721` para explicar por que existe una variante actualizable.
- Usa superficies dedicadas como `avatar-progression` o `world-plugin-assignment` cuando el protocolo necesita estado mutable consultable on-chain.

## Ejemplo hipotetico

Flujo hipotetico: un creator actualiza el manifiesto referido por `token_uri` despues de refrescar visuales de avatar, mientras ownership y semanticas tipadas permanecen intactas on-chain.

## Referencias

- Codigo: `contracts/nft/pg721-updatable/src/msg.rs`
- Contexto local: `contracts/nft/pg721-updatable/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
