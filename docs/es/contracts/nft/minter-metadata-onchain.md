# minter-metadata-onchain

## Estado

Compatibilidad

## Proposito

Representa la superficie antigua de venta primaria con metadata on-chain retenida por compatibilidad y referencia.

## Instanciacion

Se instancia con los supuestos antiguos de coleccion metadata-onchain y la configuracion de venta usada por esta generacion de minter.

Fuente real de payload: `contracts/nft/minter-metadata-onchain/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin del drop, buyers, coleccion desplegada y la configuracion de payout asociada a la ruta antigua metadata-onchain.

## Mensajes clave

- Ejecuta una ruta antigua de venta primaria para colecciones con metadata on-chain.
- Conecta la logica de venta con el despliegue de la coleccion correspondiente.
- Importa sobre todo para compatibilidad, migracion o contexto historico.

## Relaciones

- Se empareja conceptualmente con elecciones antiguas de coleccion metadata-onchain.
- Debe compararse con `minter-v2-metadata-onchain` al documentar rutas actuales versus antiguas.
- Aun encaja en el lifecycle amplio de coleccion y reventa.

## Ejemplo hipotetico

Flujo hipotetico: un despliegue antiguo mantenia metadata completamente on-chain en la ruta de venta primaria y aun documenta este contrato por motivos de compatibilidad.

## Referencias

- Codigo: `contracts/nft/minter-metadata-onchain/src/msg.rs`
- Contexto local: `contracts/nft/minter-metadata-onchain/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
