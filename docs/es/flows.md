# Flujos Hipoteticos

## Flujo 1: ecosystem -> collection -> mint -> resale

1. Un admin crea un ecosystem mediante `ecosystem-factory`.
2. `registry` registra el ecosystem y su membresia.
3. El admin o un miembro aprobado usa `collection-factory` para desplegar una coleccion `pg721`.
4. La coleccion queda registrada en `registry`.
5. Un `minter-v2` opcional vende tokens en primaria.
6. Mas tarde, `marketplace-v3` permite la reventa y aplica fee global + royalties.

## Flujo 2: subasta

1. El owner del NFT aprueba `auction-english`.
2. Crea una subasta con precio de reserva.
3. Los compradores envian bids.
4. El ganador liquida la subasta y el contrato distribuye seller proceeds, fee y royalties.

## Flujo 3: governance PASG

1. Holders depositan `upasg` en `pasg-governance`.
2. Se crea una propuesta.
3. La propuesta pasa por quorum y threshold.
4. Si la accion es administrativa, `pasg-governance` publica una accion ratificada.
5. `multisig` ejecuta la accion sobre el contrato objetivo.

## Flujo 4: economia local auxiliar

1. Un world operator usa `streaming-billing` para exponer una economia local basada en puntos.
2. Esa economia sigue siendo auxiliar a la monetizacion principal.
3. La venta inicial y las reventas siguen viviendo en las colecciones y mercados NFT.

## Flujo tecnico corto

- Consultar `streaming-billing` con `PasgUtility` para semantica PASG.
- Consultar `registry` para afiliacion, mint y trade permissions.
- Ejecutar mercado o subasta solo sobre colecciones registradas y tradeables.
