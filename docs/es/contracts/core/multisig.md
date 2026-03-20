# multisig

## Estado

Actual

## Proposito

Funciona como el plano estable de ejecucion admin-owner para contratos del protocolo como registry y las superficies de marketplace.

## Instanciacion

Se instancia con el conjunto de signers/admins, thresholds de propuesta, reglas temporales y las relaciones iniciales con contratos bajo su control administrativo.

Fuente real de payload: `contracts/core/multisig/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Miembros del multisig, contratos del protocolo administrados por el multisig y PASG governance cuando entrega acciones administrativas ratificadas.

## Mensajes clave

- Crea, vota y ejecuta propuestas administrativas.
- Es el paso final de ejecucion para cambios admin del protocolo.
- Actua como destino del handoff acotado desde PASG governance.

## Relaciones

- Es owner de contratos como `registry` y puede ejecutar updates sobre ellos.
- Recibe acciones tipadas ratificadas desde `pasg-governance`.
- No debe confundirse con la capa de votacion PASG en si.

## Ejemplo hipotetico

Flujo hipotetico: PASG governance ratifica un cambio de configuracion del marketplace y luego el multisig refleja esa accion en una propuesta admin normal y ejecuta el cambio on-chain.

## Referencias

- Codigo: `contracts/core/multisig/src/msg.rs`
- Contexto local: `contracts/core/multisig/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
