# pasg-governance

## Estado

Actual

## Proposito

Provee la capa de propuestas, votacion, delegacion y ratificacion para holders de PASG sin reemplazar al multisig como ejecutor administrativo.

## Instanciacion

Se instancia con los supuestos del denom PASG, thresholds de governance, ventanas de votacion, reglas de delegacion y el multisig admin destino para acciones ratificadas.

Fuente real de payload: `contracts/core/pasg-governance/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Holders de PASG, delegados, proponentes y el multisig admin que consume acciones ratificadas.

## Mensajes clave

- Acepta depositos PASG para poder de voto.
- Gestiona propuestas, votos, delegacion, quorum y thresholds de aprobacion.
- Publica acciones administrativas tipadas ratificadas para ejecucion por multisig.

## Relaciones

- Depende de `multisig` para la ejecucion admin final del protocolo.
- Toca superficies PASG y parametros del protocolo, no sistemas generales off-chain.
- Puede apuntar a contratos como `streaming-billing`, `marketplace-v3` y `auction-english` mediante acciones tipadas.

## Ejemplo hipotetico

Flujo hipotetico: los holders PASG aprueban una propuesta para actualizar parametros del marketplace; governance registra la accion ratificada y multisig ejecuta el update del contrato destino.

## Referencias

- Codigo: `contracts/core/pasg-governance/src/msg.rs`
- Contexto local: `contracts/core/pasg-governance/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
