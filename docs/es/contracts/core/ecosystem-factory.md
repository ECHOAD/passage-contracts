# ecosystem-factory

## Estado

Actual

## Proposito

Implementa el flujo gobernado para crear ecosystems y su configuracion inicial de collection-factory.

## Instanciacion

Se instancia con enlace a registry, la configuracion de despliegue del collection-factory hijo y la superficie admin/operator que aprueba la creacion de ecosystems.

Fuente real de payload: `contracts/core/ecosystem-factory/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Admin del protocolo, solicitantes de ecosystem, registry y el collection-factory creado para cada ecosystem aprobado.

## Mensajes clave

- Recibe solicitudes de creacion de ecosystem o flujos gobernados de alta.
- Registra ecosystems aprobados en registry.
- Despliega o vincula el collection-factory por ecosystem.

## Relaciones

- Alimenta el estado canonico del ecosystem dentro de `registry`.
- Crea el entorno que `collection-factory` usa para desplegar colecciones.
- Se ubica debajo del plano administrativo y no reemplaza a registry.

## Ejemplo hipotetico

Flujo hipotetico: un nuevo universo de creators es aprobado por `ecosystem-factory`, que registra el ecosystem y prepara la superficie de despliegue de colecciones que el equipo usara despues.

## Referencias

- Codigo: `contracts/core/ecosystem-factory/src/msg.rs`
- Contexto local: `contracts/core/ecosystem-factory/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
