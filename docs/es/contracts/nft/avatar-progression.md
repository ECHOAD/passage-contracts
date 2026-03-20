# avatar-progression

## Estado

Actual

## Proposito

Documenta la superficie dedicada de estado de progresion de Passage implementada por el crate `asset-progression`.

Este contrato mantiene los snapshots mutables de progresion de avatar y companion fuera de la metadata NFT generica. El NFT sigue siendo la identidad duradera y la superficie de derechos, mientras la progresion se persiste por separado en puntos de guardado.

## Instanciacion

Se instancia con:

- `InstantiateMsg { admin }`

Fuente real de payload: `contracts/nft/asset-progression/src/msg.rs` y `contracts/nft/asset-progression/schema/`.

## Actores y permisos

- El admin puede rotar la administracion del contrato.
- Quien escribe snapshots debe ser el owner NFT vigente o una approval activa de CW721 para el asset.
- Solo se soportan assets de Passage de tipo avatar y companion.

## Mensajes clave

- `SaveSnapshot`: persiste un snapshot de progresion por asset y por world.
- `UpdateAdmin`: rota el admin del contrato.

## Queries clave

- `Config`
- `Snapshot`
- `SnapshotsByAsset`
- `SnapshotsByWorld`

## Notas de boundary

- La progresion vive en una superficie de estado dedicada, no en metadata NFT generica.
- La persistencia en save points ocurre on-chain; formulas de gameplay y logica runtime siguen definidas por el world y off-chain.
- Detalles de render, matrices de compatibilidad y otros payloads pesados siguen detras de `token_uri`.

## Relaciones

- Resuelve ownership y approvals contra la coleccion fuente `pg721` o `pg721-updatable`.
- Complementa colecciones de avatar y companion sin expandir su boundary de metadata tipada.
- Puede componerse con lecturas del manifiesto del cliente para que el usuario vea un solo activo compuesto con su ultimo snapshot de progresion.

## Ejemplo hipotetico

Flujo hipotetico: un world guarda el checkpoint de un avatar al terminar una sesion, y luego clientes e indexadores leen el NFT para identidad duradera y `avatar-progression` para el ultimo snapshot por world.

## Referencias

- Codigo: `contracts/nft/asset-progression/src/msg.rs`
- Contexto local: `contracts/nft/asset-progression/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
