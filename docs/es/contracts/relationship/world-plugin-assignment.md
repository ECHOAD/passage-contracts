# world-plugin-assignment

## Estado

Actual

## Proposito

Define la superficie dedicada de relacion de Passage para derechos duraderos de asignacion plugin-a-world.

Este contrato mantiene separadas la propiedad del plugin y los derechos de uso del world. El NFT del plugin prueba ownership, el NFT del world prueba autoridad sobre el world, y esta superficie registra la asignacion duradera que sobrevive a una reventa posterior.

## Instanciacion

Se instancia con:

- `InstantiateMsg {}`

Fuente real de payload: `contracts/relationship/world-plugin-assignment/src/msg.rs` y `contracts/relationship/world-plugin-assignment/schema/`.

## Actores y permisos

- El owner del plugin o un operador aprobado puede crear una asignacion.
- El owner del world o un operador aprobado tambien debe autorizar el destino de la asignacion.
- Cualquiera puede consultar las asignaciones actuales.

## Mensajes clave

- `Assign`: persiste un registro duradero de asignacion plugin-a-world.
- `Remove`: elimina un registro de asignacion existente.

## Queries clave

- `Assignment`
- `AssignmentsByWorld`
- `AssignmentsByPlugin`

## Notas de boundary

- El estado duradero de asignacion vive on-chain y es consultable en este contrato, no incrustado en metadata NFT generica.
- Binarios del plugin, pasos de despliegue, permisos runtime y mecanicas de instalacion siguen off-chain.
- Los datos del manifiesto detras de `token_uri` pueden describir el plugin, pero la relacion exigible de asignacion vive aqui.

## Relaciones

- Lee la propiedad del plugin desde la coleccion NFT de plugins correspondiente.
- Lee la autoridad del world desde la coleccion NFT de worlds correspondiente.
- Complementa `pg721` y `pg721-updatable` sin expandir su modelo de metadata tipada.

## Ejemplo hipotetico

Flujo hipotetico: un creator asigna un plugin a un world, luego transfiere el NFT del plugin a otro owner, y los integradores siguen consultando este contrato para confirmar que el world conserva su derecho duradero de asignacion.

## Referencias

- Codigo: `contracts/relationship/world-plugin-assignment/src/msg.rs`
- Contexto local: `contracts/relationship/world-plugin-assignment/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
