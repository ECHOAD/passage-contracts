# auction-english

## Estado

Actual

## Proposito

Ejecuta subastas NFT con precio de reserva para colecciones registradas y maneja competencia de bids y settlement final.

## Instanciacion

Se instancia con controles admin, superficie de fees, enlace a registry y las reglas de subasta necesarias para ventas con reserva.

Fuente real de payload: `contracts/nft/auction-english/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Owner del NFT, bidders, admin/operators, registry, recipients de royalty y recipients del settlement.

## Mensajes clave

- Crea y gestiona subastas con reserva.
- Acepta bids y maneja de forma refund-safe a bidders desplazados o perdedores.
- Liquida la subasta ganadora hacia seller proceeds, fee y royalties.

## Relaciones

- Depende del ownership y estado de approval de la coleccion.
- Usa supuestos de trading basados en registry para colecciones validas.
- Se superpone con marketplace-v3 como otra via de comercio secundario, pero para subastas y no para precio fijo.

## Ejemplo hipotetico

Flujo hipotetico: un NFT world propiedad del creator entra en una subasta inglesa, los bidders compiten y el ganador liquida mientras el contrato rutea correctamente fee y royalties.

## Referencias

- Codigo: `contracts/nft/auction-english/src/msg.rs`
- Contexto local: `contracts/nft/auction-english/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
