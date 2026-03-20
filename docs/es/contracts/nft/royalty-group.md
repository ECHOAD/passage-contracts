# royalty-group

## Estado

Actual / Soporte

## Proposito

Provee una superficie de recipient grupal para que los royalties puedan repartirse entre multiples partes.

## Instanciacion

Se instancia con la configuracion de payouts por miembro y cualquier permiso admin necesario para mantener la definicion del grupo de royalties.

Fuente real de payload: `contracts/nft/royalty-group/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Recipients de royalty, contratos de coleccion o venta que envian royalties aqui y el admin que gestiona la definicion del grupo.

## Mensajes clave

- Define un grupo receptor de royalties.
- Recibe fondos de royalty y los redistribuye.
- Permite que contratos de comercio NFT apunten a un contrato recipient unico en vez de muchas direcciones directas.

## Relaciones

- Puede ubicarse detras del payout de royalties en marketplace o auction.
- Complementa la configuracion de royalties a nivel de coleccion.
- Suele trabajar junto con split-router cuando se necesita un payout mas complejo.

## Ejemplo hipotetico

Flujo hipotetico: marketplace-v3 envia fondos de royalty a `royalty-group`, que luego redistribuye el monto entre un equipo de creators.

## Referencias

- Codigo: `contracts/nft/royalty-group/src/msg.rs`
- Contexto local: `contracts/nft/royalty-group/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
