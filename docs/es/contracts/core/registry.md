# registry

## Estado

Actual

## Proposito

Actua como ledger canonico para ecosystems, afiliacion de colecciones, provenance del creator y permisos de mint/trade.

## Instanciacion

Se instancia con control admin, configuracion de moderacion y recovery, y los defaults de politica para ecosystem/collection que definen el comportamiento canonico del registro.

Fuente real de payload: `contracts/core/registry/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Admin del protocolo, admins de ecosystems, miembros aprobados, creators, minters, marketplaces y consumidores de queries.

## Mensajes clave

- Registra ecosystems y colecciones.
- Gestiona membresia, deregistration y flujos de re-home.
- Autoriza minting y chequeos de tradeability usados por contratos de comercio aguas abajo.

## Relaciones

- Recibe escrituras canonicas desde `ecosystem-factory` y `collection-factory`.
- Entrega datos de permiso y afiliacion a `marketplace-v3`, `auction-english` y los flujos de mint.
- Almacena informacion de `nft_type` y provenance a nivel de coleccion usada por todo el protocolo.

## Ejemplo hipotetico

Flujo hipotetico: una coleccion se mueve de un ecosystem a otro; la direccion del contrato no cambia, pero registry actualiza su afiliacion de ecosystem preservando la provenance del creator.

## Referencias

- Codigo: `contracts/core/registry/src/msg.rs`
- Contexto local: `contracts/core/registry/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
