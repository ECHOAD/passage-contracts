# marketplace-v3

## Estado

Actual

## Proposito

Provee el marketplace secundario actual multi-coleccion para asks, bids, collection bids, fees y reventa con conocimiento de royalties.

## Instanciacion

Se instancia con admin, registry, fee collector, precio minimo, fee global de trading y cualquier operador que pueda moderar el comportamiento del marketplace.

Fuente real de payload: `contracts/nft/marketplace-v3/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Owners de colecciones, buyers, bidders, admin del marketplace, registry, recipients de royalty y fee collector.

## Mensajes clave

- Registra colecciones y gestiona el denom de settlement por coleccion.
- Soporta asks, compras directas, bids por token y collection bids.
- Calcula fee del marketplace y preview/settlement de venta con awareness de royalties.

## Relaciones

- Depende de la moderacion de registry y la tradeability de la coleccion.
- Consume semantica PASG de forma indirecta via streaming-billing cuando el integrador necesita la vista canonica PASG.
- Coexiste con auction-english como la via de venta secundaria a precio fijo.

## Ejemplo hipotetico

Flujo hipotetico: una coleccion registrada lista el token 1 en `upasg`, un buyer usa `buy_now` y el contrato aplica el fee global del marketplace mas la distribucion de royalties.

## Referencias

- Codigo: `contracts/nft/marketplace-v3/src/msg.rs`
- Contexto local: `contracts/nft/marketplace-v3/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
