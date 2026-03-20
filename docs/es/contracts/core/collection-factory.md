# collection-factory

## Estado

Actual

## Proposito

Despliega contratos de coleccion para un ecosystem y empuja el flujo canonico de registro hacia registry.

## Instanciacion

Se instancia con la direccion de registry, el contexto del ecosystem, el code ID de la coleccion y la superficie admin/operator que puede desplegar colecciones hijas.

Fuente real de payload: `contracts/core/collection-factory/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Admin del ecosystem, miembros aprobados, registry y el contrato de coleccion desplegado.

## Mensajes clave

- Crea colecciones dentro del contexto de un ecosystem.
- Registra la coleccion resultante en registry.
- Gestiona el lifecycle de despliegue y reply-path de contratos hijos.

## Relaciones

- Depende de `registry` como ledger canonico.
- Trabaja aguas abajo de `ecosystem-factory` porque el ecosystem debe existir primero.
- Normalmente despliega `pg721` o variantes relacionadas de coleccion.

## Ejemplo hipotetico

Flujo hipotetico: un admin de Cyberpunk Universe usa `collection-factory` para desplegar una coleccion de worlds y la nueva direccion queda registrada de inmediato para poder validar luego permisos de mint y trading.

## Referencias

- Codigo: `contracts/core/collection-factory/src/msg.rs`
- Contexto local: `contracts/core/collection-factory/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
