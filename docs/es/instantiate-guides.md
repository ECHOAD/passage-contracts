# Guias de Instanciacion

## Orden tipico de despliegue

1. `registry`
2. `multisig`
3. `ecosystem-factory`
4. `collection-factory` por ecosystem
5. contratos `pg721*` o `minter*`
6. `marketplace-v3` y/o `auction-english`
7. `split-router`, `royalty-group`, `streaming-billing`, segun el flujo
8. `pasg-governance` si se activa governance PASG

## Regla general

Cada contrato define su `InstantiateMsg` en `src/msg.rs` o en el schema generado. Esta guia explica dependencias y orden, no reemplaza los campos reales.

## Campos que se repiten

- `admin`
- direcciones de contratos dependientes como `registry`, `multisig`, `split-router`, `collection`, `stake`
- `code_id` cuando una factory despliega otros contratos
- parametros economicos como `trading_fee_bps`, `min_price`, duraciones, o denoms

## Ejemplos de dependencia

- `collection-factory` necesita conocer `registry` y el code id del contrato de coleccion que desplegara.
- `marketplace-v3` necesita `registry`, `fee_collector`, operadores y su fee global.
- `pasg-governance` necesita conocer el `admin_multisig` al que entregara acciones ratificadas.
- `stake-rewards` necesita la direccion del contrato de stake autorizado.

## Recomendacion

Antes de instanciar, revisa la pagina especifica del contrato y luego confirma el payload real en `msg.rs` o `schema/`.
