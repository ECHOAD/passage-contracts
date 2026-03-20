# minter-v2

## Estado

Actual

## Proposito

Gestiona el flujo actual de minting en venta primaria y puede desplegar su propia coleccion para lanzamientos tipo drop.

## Instanciacion

Se instancia con parametros de despliegue de coleccion, configuracion de venta, controles admin/operator y cualquier dependencia de payout o registry que requiera el drop.

Fuente real de payload: `contracts/nft/minter-v2/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin del drop, creator, buyers, la coleccion desplegada, registry y rutas de payout.

## Mensajes clave

- Despliega o gestiona una coleccion para ventas primarias por mint.
- Controla ventanas de mint y configuracion de venta.
- Rutea revenue de venta primaria manteniendo compatibilidad con registry y marketplace despues.

## Relaciones

- Normalmente apunta a `pg721` o a una variante relacionada de coleccion.
- Puede enviar proceeds a rutas de split o payout aware de royalties.
- La coleccion resultante aun debe ser visible para registry dentro del lifecycle general del protocolo.

## Ejemplo hipotetico

Flujo hipotetico: un creator lanza un nuevo drop por `minter-v2`, buyers mintean en venta primaria y luego la coleccion entra en venta secundaria por marketplace-v3.

## Referencias

- Codigo: `contracts/nft/minter-v2/src/msg.rs`
- Contexto local: `contracts/nft/minter-v2/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
