# stake-rewards

## Estado

Actual

## Proposito

Distribuye rewards externos a usuarios segun el estado de stake reportado por un contrato de stake autorizado.

## Instanciacion

Se instancia con el contrato de stake autorizado, el denom de rewards y los parametros de duracion.

Fuente real de payload: `contracts/staking/stake-rewards/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Contrato de stake autorizado, stakers, claimers de rewards y administradores de configuracion.

## Mensajes clave

- Actualiza la contabilidad de rewards cuando cambia el stake.
- Permite que usuarios reclamen rewards acumulados.
- Depende del estado de stake de otro contrato en vez de poseer el stake directamente.

## Relaciones

- Normalmente se ubica detras de `nft-vault`.
- Es parte de la familia de staking de NFT.
- No debe confundirse con los rewards del staking nativo PASG.

## Ejemplo hipotetico

Flujo hipotetico: `nft-vault` reporta cambios de stake a `stake-rewards`, y los usuarios luego reclaman tokens de recompensa calculados desde ese historial.

## Referencias

- Codigo: `contracts/staking/stake-rewards/src/msg.rs`
- Contexto local: `contracts/staking/stake-rewards/README.md`
- Guia compartida: `docs/es/relationships.md`
