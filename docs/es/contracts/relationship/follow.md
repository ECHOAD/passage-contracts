# follow

## Estado

Actual

## Proposito

Implementa una primitiva social de follow para cuentas o identidades Passage.

## Instanciacion

Se instancia con la superficie admin y cualquier config necesaria para mantener el grafo de follow.

Fuente real de payload: `contracts/relationship/follow/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Usuarios que siguen, usuarios seguidos y admins/operators si existe moderacion.

## Mensajes clave

- Registra relaciones sociales tipo follow.
- Expone queries alrededor de esas relaciones.
- Se mantiene fuera de la politica de creator-commerce, governance y staking.

## Relaciones

- Pertenece a la familia relationship y no al stack economico.
- Puede complementar features sociales o de perfil off-chain.
- No dirige semantica de registry, marketplace ni utilidad PASG.

## Ejemplo hipotetico

Flujo hipotetico: una capa de perfil consulta `follow` para mostrar a quien sigue una cuenta creator, mientras los contratos de comercio no cambian.

## Referencias

- Codigo: `contracts/relationship/follow/src/msg.rs`
- Contexto local: `contracts/relationship/follow/README.md`
- Guia compartida: `docs/es/relationships.md`
