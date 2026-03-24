# minter-v2

## Estado

Actual

## Proposito

Gestiona el flujo actual de minting en venta primaria para una coleccion ya existente.

## Instanciacion

Se instancia con la direccion de la coleccion objetivo, configuracion de venta, controles admin/operator y cualquier dependencia de payout o registry que requiera el drop.

Fuente real de payload: `contracts/nft/minter-v2/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin del drop, creator, buyers, la coleccion existente, registry y rutas de payout.

## Mensajes clave

- Gestiona una venta primaria por mint para una coleccion concreta.
- Controla ventanas de mint y configuracion de venta.
- Rutea revenue de venta primaria manteniendo compatibilidad con registry y marketplace despues.

## Relaciones

- Apunta a una coleccion `pg721` o a una variante relacionada ya desplegada.
- Puede enviar proceeds a rutas de split o payout aware de royalties.
- La coleccion resultante aun debe ser visible para registry dentro del lifecycle general del protocolo.

## Ejemplo hipotetico

Flujo hipotetico: un creator o admin de ecosistema primero registra una coleccion, autoriza un `minter-v2` para esa coleccion en `registry`, y luego los buyers mintean en venta primaria antes de que la coleccion entre en venta secundaria por marketplace-v3.

## Referencias

- Codigo: `contracts/nft/minter-v2/src/msg.rs`
- Contexto local: `contracts/nft/minter-v2/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
