# End-to-End Setup

Esta guia describe el flujo recomendado desde la instanciacion de `registry` hasta dejar una coleccion lista para vender por `marketplace-v3` o por `auction-english`.

Payloads JSON listos para usar:

- `03-json-examples.md`

## 1. Instanciar `registry`

`registry` es el primer contrato que debe existir.

Campos importantes de `InstantiateMsg`:

- `admin`: admin global del registry.
- `operators`: operadores globales opcionales.
- `ecosystem_factory`: direccion opcional del `ecosystem-factory` autorizado para registrar ecosystems aprobados.

Uso:

- Si vas a manejar ecosystems manualmente, puedes instanciar `registry` sin `ecosystem_factory`.
- Si vas a usar el flujo gobernado, despliega `ecosystem-factory` y luego conecta su direccion con `UpdateConfig`.

## 2. Elegir como crear el ecosystem

Hay dos caminos.

### Camino A: directo en `registry`

Usa este camino si el equipo admin controla todo el alta.

Mensajes relevantes:

- `ApproveEcosystemCreator`
- `RegisterEcosystem`
- `UpdateEcosystem`

Flujo:

1. El admin aprueba al creador con `ApproveEcosystemCreator { creator }`, si aplica.
2. El creador o admin llama `RegisterEcosystem`.
3. Si aun no existe un `collection-factory` para ese ecosystem, debes desplegarlo manualmente y luego guardar su direccion con `UpdateEcosystem { collection_factory }`.

### Camino B: gobernado con `ecosystem-factory`

Usa este camino si quieres requests y aprobacion.

`ecosystem-factory` se instancia con:

- `admin`
- `operators`
- `registry`
- `collection_factory_code_id`
- `collection_code_id`

Flujo:

1. Un creador llama `SubmitEcosystemCreationRequest`.
2. Admin u operador llama `ResolveEcosystemCreationRequest`.
3. Si se aprueba, `ecosystem-factory` despliega un `collection-factory` dedicado.
4. En el `reply`, el factory llama `registry.RegisterEcosystemFromFactory`.

Resultado esperado:

- El ecosystem queda registrado en `registry`.
- El ecosystem queda enlazado a un `collection-factory`.

## 3. Instanciar o validar `collection-factory`

Si el ecosystem vino desde `ecosystem-factory`, este paso ya queda hecho.

Si el ecosystem fue creado directo, debes instanciar `collection-factory` manualmente con:

- `admin`
- `operators`
- `registry`
- `ecosystem_id`
- `collection_code_id`
- `enforce_local_allowlist`
- `approved_creators`

Notas:

- `enforce_local_allowlist = true` obliga a aprobar wallets localmente con `ApproveCreator`.
- Aun si desactivas la allowlist local, `collection-factory` sigue consultando `registry.CanCreateCollectionInEcosystem`.

## 4. Crear la coleccion `pg721`

La coleccion se crea desde `collection-factory`.

Mensaje principal:

- `CreateCollection { name, symbol, minter, collection_info, label }`

`collection_info` incluye:

- `description`
- `image`
- `external_link`
- `royalty_info`

Puntos importantes:

- `minter` es quien podra hacer `Mint` dentro del `pg721`.
- Si quieres mintear manualmente, usa una wallet o contrato que controles.
- Si quieres un drop primario con `minter-v2`, no uses este camino para esa coleccion; `minter-v2` despliega su propio `pg721`.

Que pasa despues:

1. `collection-factory` instancia `pg721`.
2. En `reply`, registra la nueva coleccion en `registry` via `RegisterCollectionFromFactory`.

Verificaciones recomendadas:

- `registry.Collection { address }`
- `registry.CollectionsByEcosystem { ecosystem_id }`

## 5. Instanciar `split-router`

`split-router` reparte ingresos del creator y royalties.

Se instancia con:

- `admin`
- `registry`

Despues debes crear una regla por coleccion:

- `SetDistributionRule { collection, creator, creator_share, collaborators }`

Uso recomendado:

- Configuralo antes de habilitar ventas.
- Si usaras `marketplace-v3` o `auction-english` con `use_split_router = true`, la coleccion debe tener regla cargada.

Queries utiles:

- `DistributionRule { collection }`
- `PreviewDistribution { collection, amount, event_type }`

## 6. Camino de fixed price sale con `marketplace-v3`

`marketplace-v3` es para secondary sales. No crea ni custodia colecciones.

### 6.1 Instanciar el marketplace

Campos importantes:

- `admin`
- `denom`
- `min_price`
- `trading_fee_bps`
- `max_trading_fee_bps`
- `fee_collector`
- `registry`
- `split_router`
- `use_split_router`
- `operators`
- `require_registration`

Uso recomendado:

- `require_registration = true`
- `registry = <registry>`
- `split_router = <split-router>`
- `use_split_router = true`

### 6.2 Registrar la coleccion en el marketplace

Mensaje:

- `RegisterCollection { collection, trading_fee_bps, denom }`

Notas:

- Esto habilita la coleccion dentro del marketplace.
- No sustituye el registro en `registry`; son dos registros distintos.

Opcional pero recomendado:

- Guardar el puntero runtime en `registry` con `UpdateCollection { marketplace: Some(...) }`

### 6.3 Aprobar el marketplace en `pg721`

Antes de listar, el owner debe aprobar el marketplace en la coleccion `pg721`.

Metodos relevantes del `pg721`:

- `Approve { spender, token_id, expires }`
- `ApproveAll { operator, expires }`

Sin esa aprobacion, no podra completarse la venta cuando llegue el comprador.

### 6.4 Crear la venta

Mensaje:

- `SetAsk { collection, token_id, price, funds_recipient }`

Luego tienes tres salidas:

- compra directa con `BuyNow`
- oferta puntual con `SetBid` y aceptacion con `AcceptBid`
- oferta sobre toda la coleccion con `SetCollectionBid` y aceptacion con `AcceptCollectionBid`

### 6.5 Como liquida la venta

En el modelo actual:

- `marketplace-v3` cobra `trading_fee_bps`
- el seller recibe `sale_price - trading_fee - royalty`
- si `use_split_router = true`, el royalty se manda a `split-router`
- si `use_split_router = false`, el royalty se paga directo al `payment_address` de `pg721`

Queries utiles:

- `CanTrade { collection }`
- `PreviewSale { collection, price }`
- `Ask { collection, token_id }`
- `CollectionStats { collection }`

## 7. Camino de auction con `auction-english`

`auction-english` tambien es secondary sale, pero custodial.

### 7.1 Instanciar el auction

Campos importantes:

- `admin`
- `denom`
- `min_price`
- `trading_fee_bps`
- `max_trading_fee_bps`
- `fee_collector`
- `registry`
- `split_router`
- `use_split_router`
- `min_bid_increment_percent`
- `min_duration`
- `max_duration`
- `extend_duration`
- `require_registration`

Uso recomendado:

- `require_registration = true`
- `registry = <registry>`
- `split_router = <split-router>`
- `use_split_router = true`

### 7.2 Aprobar el auction en `pg721`

El owner debe aprobar el contrato de auction en la coleccion `pg721`.

Metodos relevantes:

- `Approve`
- `ApproveAll`

### 7.3 Crear la subasta

Mensaje:

- `CreateAuction { collection, token_id, reserve_price, duration, seller_funds_recipient }`

Importante:

- el NFT se transfiere al contrato de auction al crear la subasta
- antes del primer bid, el seller todavia puede:
  - `UpdateReservePrice`
  - `CancelAuction`

### 7.4 Recibir bids

Mensaje:

- `PlaceBid { collection, token_id }`

Comportamiento:

- el primer bid inicia el reloj de cierre
- bids posteriores deben respetar `min_bid_increment_percent`
- si faltan pocos segundos, la subasta se extiende usando `extend_duration`

### 7.5 Cerrar y liquidar

Mensaje:

- `SettleAuction { collection, token_id }`

Comportamiento:

- cualquiera puede ejecutar el settle despues del cierre
- se paga `trading_fee` al `fee_collector`
- se paga seller proceeds al seller o a `seller_funds_recipient`
- el royalty va a `split-router` si esta activado
- el NFT se transfiere al ganador

Queries utiles:

- `CanTrade { collection }`
- `Auction { collection, token_id }`
- `AuctionsByCollection`
- `AuctionsBySeller`
- `AuctionsByEndTime`

## 8. Camino opcional de venta primaria con `minter-v2`

Este camino es distinto. `minter-v2` despliega su propia coleccion `pg721`.

Flujo recomendado:

1. Instancia `minter-v2`.
2. Espera el `reply` o consulta `Config {}` para obtener `cw721_address`.
3. Registra esa coleccion en `registry` con `RegisterExistingCollection`.
4. Autoriza el minter con `AuthorizeMinter { collection_address, minter_address }`.
5. Opcionalmente guarda el puntero con `UpdateCollection { minter: Some(...) }`.
6. Crea la regla de `split-router` para esa coleccion.
7. Abre mint con `start_time` y luego usa `Mint` o `BatchMint`.

Sin los pasos 3 y 4, `minter-v2` puede quedar bloqueado si `registry` esta configurado, porque valida:

- que la coleccion exista en `registry`
- que el minter este autorizado en `registry`

## 9. Checklist minima para salir a produccion

### Fixed price sale

1. `registry` operativo
2. coleccion registrada en `registry`
3. `split-router` con regla para la coleccion
4. `marketplace-v3` instanciado
5. coleccion registrada en `marketplace-v3`
6. owner aprobando el marketplace en `pg721`
7. `SetAsk`

### Auction

1. `registry` operativo
2. coleccion registrada en `registry`
3. `split-router` con regla para la coleccion
4. `auction-english` instanciado
5. owner aprobando el auction en `pg721`
6. `CreateAuction`
7. bids
8. `SettleAuction`

Para mensajes exactos en JSON, usa tambien:

- `03-json-examples.md`
