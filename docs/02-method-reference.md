# Method Reference

Esta referencia resume para que sirve cada smart contract y cuales son sus metodos mas importantes.

Si necesitas ejemplos exactos de payload, revisa tambien:

- `03-json-examples.md`

## `registry`

Rol:

- fuente de verdad para ecosystems, collections y minters autorizados

Instantiate:

- `InstantiateMsg { admin, operators, ecosystem_factory }`

Execute mas importantes:

- `UpdateConfig`
- `ApproveEcosystemCreator`
- `RevokeEcosystemCreator`
- `RegisterEcosystem`
- `RegisterEcosystemFromFactory`
- `SubmitEcosystemCreationRequest`
- `ResolveEcosystemCreationRequest`
- `UpdateEcosystem`
- `ApproveEcosystemMember`
- `RevokeEcosystemMember`
- `SubmitCollectionCreationRequest`
- `ResolveCollectionCreationRequest`
- `RegisterCollection`
- `RegisterCollectionFromFactory`
- `RegisterExistingCollection`
- `UpdateCollection`
- `TransferCollectionOwnership`
- `AuthorizeMinter`
- `RevokeMinter`

Cuando usar cada uno:

- `RegisterEcosystem`: alta directa del ecosystem
- `RegisterEcosystemFromFactory`: callback del ecosystem-factory
- `RegisterCollectionFromFactory`: callback del collection-factory
- `RegisterExistingCollection`: alta manual de una coleccion ya desplegada
- `UpdateCollection`: guardar runtime pointers como `minter` y `marketplace`
- `AuthorizeMinter`: obligatorio si un `minter-v2` va a operar con `registry` habilitado

Queries mas utiles:

- `Config`
- `Ecosystem`
- `Ecosystems`
- `CanCreateCollectionInEcosystem`
- `Collection`
- `CollectionsByEcosystem`
- `IsCollectionVerified`
- `IsMinterAuthorized`
- `AuthorizedMinters`

## `ecosystem-factory`

Rol:

- flujo gobernado para crear ecosystems y desplegar su `collection-factory`

Instantiate:

- `InstantiateMsg { admin, operators, registry, collection_factory_code_id, collection_code_id }`

Execute:

- `UpdateConfig`
- `SubmitEcosystemCreationRequest`
- `ResolveEcosystemCreationRequest`

Queries:

- `Config`
- `EcosystemCreationRequest`
- `EcosystemCreationRequests`
- `PendingRequestById`
- `IsAdminOrOperator`

Nota:

- al aprobar un request, este contrato despliega el `collection-factory` y registra el ecosystem en `registry`

## `collection-factory`

Rol:

- desplegar `pg721` dentro de un ecosystem y registrarlo en `registry`

Instantiate:

- `InstantiateMsg { admin, operators, registry, ecosystem_id, collection_code_id, enforce_local_allowlist, approved_creators }`

Execute:

- `UpdateConfig`
- `ApproveCreator`
- `RevokeCreator`
- `CreateCollection`

`CreateCollection` recibe:

- `name`
- `symbol`
- `minter`
- `collection_info`
- `label`

Queries:

- `Config`
- `IsCreatorApproved`
- `ApprovedCreators`
- `Collection`
- `Collections`
- `CollectionsByCreator`

Notas:

- si `enforce_local_allowlist` esta activo, la wallet debe estar aprobada localmente
- ademas siempre se consulta `registry.CanCreateCollectionInEcosystem`
- en el `reply`, el factory llama `registry.RegisterCollectionFromFactory`

## `pg721`

Rol:

- contrato NFT base de la coleccion

Instantiate:

- `InstantiateMsg { name, symbol, minter, collection_info }`

Execute relevantes:

- `Mint`
- `TransferNft`
- `SendNft`
- `Approve`
- `ApproveAll`
- `Revoke`
- `RevokeAll`
- `Burn`

Queries relevantes:

- `OwnerOf`
- `Approval`
- `Approvals`
- `AllOperators`
- `NftInfo`
- `AllNftInfo`
- `Tokens`
- `AllTokens`
- `Minter`
- `CollectionInfo`

Uso en comercio:

- `Approve` o `ApproveAll` para `marketplace-v3` y `auction-english`
- `CollectionInfo` para resolver royalties

## `split-router`

Rol:

- repartir fondos creator-side para mint y royalties

Instantiate:

- `InstantiateMsg { admin, registry }`

Execute:

- `UpdateConfig`
- `SetDistributionRule`
- `UpdateDistributionRule`
- `RemoveDistributionRule`
- `SetEcosystemConfig`
- `RoutePrimarySale`
- `RouteSecondaryRoyalty`
- `RouteAuctionRoyalty`
- `RouteRevenue`
- `CreateSplitWallet`
- `UpdateSplitWallet`
- `DistributeSplitWallet`
- `RemoveSplitWallet`

Cuando usar cada uno:

- `SetDistributionRule`: crear regla por coleccion
- `RoutePrimarySale`: lo llama `minter-v2`
- `RouteSecondaryRoyalty`: lo llama `marketplace-v3`
- `RouteAuctionRoyalty`: lo llama `auction-english`

Queries:

- `Config`
- `DistributionRule`
- `DistributionRules`
- `EcosystemConfig`
- `PreviewDistribution`
- `CollectionStats`
- `RevenueEvents`
- `SplitWallet`
- `SplitWallets`

## `marketplace-v3`

Rol:

- fixed price sale, token bids y collection bids

Instantiate:

- `InstantiateMsg { admin, denom, min_price, trading_fee_bps, max_trading_fee_bps, fee_collector, registry, split_router, use_split_router, operators, require_registration }`

Admin execute:

- `UpdateConfig`
- `RegisterCollection`
- `UpdateCollectionConfig`
- `DeactivateCollection`
- `ReactivateCollection`
- `BlacklistCollection`
- `UnblacklistCollection`

Trading execute:

- `SetAsk`
- `UpdateAsk`
- `RemoveAsk`
- `BuyNow`
- `SetBid`
- `RemoveBid`
- `AcceptBid`
- `SetCollectionBid`
- `RemoveCollectionBid`
- `AcceptCollectionBid`
- `SyncAsk`
- `BatchSyncAsks`

Cuando usar cada uno:

- `SetAsk`: publicar una venta fija
- `BuyNow`: comprar una venta fija
- `SetBid` / `AcceptBid`: oferta por token
- `SetCollectionBid` / `AcceptCollectionBid`: oferta por cualquier NFT de la coleccion
- `RegisterCollection`: habilitar una coleccion para trading

Queries:

- `Config`
- `CollectionConfig`
- `CollectionConfigs`
- `CanTrade`
- `CollectionDenom`
- `CollectionFee`
- `Ask`
- `AsksByCollection`
- `AsksBySeller`
- `AsksByPrice`
- `AskCount`
- `Bid`
- `BidsByToken`
- `BidsByBidder`
- `CollectionBid`
- `CollectionBidsByCollection`
- `CollectionBidsByBidder`
- `MarketStats`
- `CollectionStats`
- `PreviewSale`

## `auction-english`

Rol:

- reserve auction custodial por NFT

Instantiate:

- `InstantiateMsg { admin, denom, min_price, trading_fee_bps, max_trading_fee_bps, fee_collector, registry, split_router, use_split_router, min_bid_increment_percent, min_duration, max_duration, extend_duration, require_registration }`

Execute:

- `UpdateConfig`
- `CreateAuction`
- `UpdateReservePrice`
- `CancelAuction`
- `PlaceBid`
- `SettleAuction`

Reglas principales:

- `CreateAuction` mueve el NFT al contrato
- `UpdateReservePrice` solo antes del primer bid
- `CancelAuction` solo antes del primer bid
- `PlaceBid` inicia o sube la subasta
- `SettleAuction` liquida y entrega el NFT al ganador

Queries:

- `Config`
- `CanTrade`
- `Auction`
- `AuctionsByCollection`
- `AuctionsBySeller`
- `AuctionsByEndTime`

## `minter-v2`

Rol:

- venta primaria y despliegue de su propio `pg721`

Instantiate:

- `InstantiateMsg { base_token_uri, num_tokens, cw721_code_id, cw721_instantiate_msg, start_time, per_address_limit, unit_price, whitelist, registry, split_router, use_split_router, metadata_mode, native_asset_template }`

Execute:

- `Mint`
- `MintTo`
- `MintFor`
- `BatchMint`
- `UpdateConfig`
- `UpdateStartTime`
- `SetWhitelist`
- `RemoveWhitelist`
- `Withdraw`
- `WithdrawTo`
- `SetNativeAssetTemplate`
- `SetTokenNativeAssetOverride`
- `ClearTokenNativeAssetOverride`

Notas operativas:

- `Mint` y `BatchMint` usan `split-router` si esta activo
- si `registry` esta configurado, el minter exige que:
  - la coleccion exista en `registry`
  - el minter este autorizado via `AuthorizeMinter`
- `Withdraw` solo aplica si `use_split_router = false`

Queries:

- `Config`
- `MintableNumTokens`
- `StartTime`
- `MintPrice`
- `MintCount`
- `CanMint`
- `MintStats`
- `IsMintingActive`
- `NativeAssetTemplate`
- `TokenNativeAssets`

## Relacion entre contratos

Relaciones practicas mas importantes:

- `collection-factory` depende de `registry`
- `marketplace-v3` consulta `pg721` y opcionalmente `registry` y `split-router`
- `auction-english` consulta `pg721` y opcionalmente `registry` y `split-router`
- `minter-v2` despliega `pg721` y opcionalmente usa `registry` y `split-router`
- `split-router` puede usar `registry` para validar creator-side config

Si el objetivo es una venta secundaria, el camino minimo suele ser:

1. `registry`
2. `collection-factory` + `pg721`
3. `split-router`
4. `marketplace-v3` o `auction-english`
