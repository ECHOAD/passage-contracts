# Relaciones entre contratos

## Mapa principal

- `ecosystem-factory` crea el flujo aprobado para nuevos ecosystems.
- `registry` registra ecosystems, membresia y afiliacion canonica de colecciones.
- `collection-factory` despliega colecciones `pg721*` dentro de un ecosystem y las registra.
- `minter-v2*` tambien puede desplegar su propia coleccion y luego esa coleccion debe registrarse.
- `marketplace-v3` y `auction-english` dependen de la coleccion y de las reglas de `registry` para tradear.
- `royalty-group` y `split-router` ayudan a distribuir revenue y royalties.
- `multisig` sigue siendo el plano de ejecucion admin-owner.
- `pasg-governance` ratifica decisiones PASG y entrega acciones tipadas hacia `multisig`.
- `streaming-billing` expone la superficie utilitaria canonica de PASG.

## Relacion administrativa

1. `multisig` administra contratos del protocolo.
2. `pasg-governance` decide propuestas PASG.
3. `multisig` ejecuta los cambios administrativos aprobados cuando aplican.

## Relacion de assets

1. Un `ecosystem` es el contexto padre.
2. `collection-factory` o un registro manual introducen una coleccion.
3. `registry` guarda la afiliacion y la capacidad de minteo/trading.
4. `pg721*` mantiene la logica de coleccion NFT.
5. `minter*` representa la venta primaria cuando se usa.
6. `marketplace-v3` y `auction-english` cubren reventas y subastas.

## Relacion economica

- `streaming-billing` define la lectura canonica de utilidad PASG.
- `split-router` rutea fondos de forma generica.
- `royalty-group` puede repartir royalties a multiples destinatarios.
- `marketplace-v3` y `auction-english` ejecutan settlement sobre ventas secundarias.

## Relacion de staking

- `nft-vault`, `stake-rewards`, y `vault-factory` son para staking de NFT.
- No representan el staking nativo de PASG hacia validators.

## Regla de interpretacion

Cuando haya conflicto entre narrativa y codigo, `msg.rs`, `schema/` y el contrato real ganan.
