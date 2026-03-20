# split-router

## Estado

Actual

## Proposito

Rutea revenue de creators y revenue dirigido por contratos de forma generica sin ser due?o de la politica PASG.

## Instanciacion

Se instancia con la configuracion de payout o routing que requiera el flujo, ademas de los permisos admin/operator para actualizar ese comportamiento.

Fuente real de payload: `contracts/core/split-router/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Contratos llamadores como minters o marketplaces, destinatarios de payout y cualquier admin/operator que pueda actualizar la configuracion de routing.

## Mensajes clave

- Acepta instrucciones genericas de split o routing.
- Distribuye fondos adjuntos entre recipients.
- Se mantiene denom-agnostico incluso cuando lo llaman contratos PASG-aware.

## Relaciones

- Es usado por flujos de comercio creator-side que necesitan distribuir revenue.
- Puede ser invocado por ventas NFT o flujos de royalty.
- Debe leerse junto con `streaming-billing` cuando importa la semantica PASG, porque la politica PASG no vive aqui.

## Ejemplo hipotetico

Flujo hipotetico: un contrato de venta envia fondos a `split-router` para que creator, colaboradores y treasury reciban sus porciones usando una superficie generica de routing.

## Referencias

- Codigo: `contracts/core/split-router/src/msg.rs`
- Contexto local: `contracts/core/split-router/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
