# minter

## Estado

Compatibilidad

## Proposito

Representa la superficie antigua de minter para venta primaria que aun existe en el workspace por compatibilidad y referencia.

## Instanciacion

Se instancia con el despliegue de coleccion y configuracion de venta propios de esta generacion antigua de minter.

Fuente real de payload: `contracts/nft/minter/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin del drop, buyers, la coleccion desplegada y cualquier ruta de payout usada por el flujo de venta antiguo.

## Mensajes clave

- Ejecuta el flujo mas antiguo de venta primaria.
- Despliega o coordina una coleccion para minting.
- Sigue siendo util sobre todo para compatibilidad o comprension historica frente a v2.

## Relaciones

- Precede a `minter-v2` en la evolucion de herramientas de venta primaria.
- Sigue girando alrededor del despliegue de coleccion y control de la venta.
- Debe leerse junto con `minter-v2` para entender la ruta recomendada actual.

## Ejemplo hipotetico

Flujo hipotetico: una integracion mantenida desde un despliegue anterior de Passage aun referencia `minter`, aunque el trabajo nuevo normalmente deberia mirar primero `minter-v2`.

## Referencias

- Codigo: `contracts/nft/minter/src/msg.rs`
- Contexto local: `contracts/nft/minter/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
