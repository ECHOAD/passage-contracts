# streaming-billing

## Estado

Actual

## Proposito

Define la interfaz canonica de utilidad PASG y las superficies acotadas de billing o economia local que permanecen on-chain en este repo.

## Instanciacion

Se instancia con enlaces a registry y al contexto de billing, permisos operator/admin, supuestos PASG y cualquier configuracion de billing por world que esta superficie service-owned necesite.

Fuente real de payload: `contracts/core/streaming-billing/src/msg.rs` y, cuando exista, `schema/` del crate.

## Actores y permisos

Operadores backend, worlds o servicios que disparan eventos de billing, integradores PASG-aware y consumidores de queries.

## Mensajes clave

- Expone `PasgUtility` y queries suplementarias PASG-aware.
- Gestiona superficies acotadas de deposito, reportes, retiros y distribucion de revenue.
- Puede previsualizar semanticas opcionales de economia local por world sin reemplazar el comercio principal del creator.

## Relaciones

- Actua como fuente de verdad para la semantica PASG consumida por el resto del repo.
- Se conecta con `split-router` para revenue routing preservando el limite de routing generico.
- Debe mantenerse dentro del boundary on-chain/off-chain fijado en fases previas.

## Ejemplo hipotetico

Flujo hipotetico: un integrador consulta `PasgUtility` para confirmar el denom PASG nativo y luego usa queries de billing para previsualizar un flujo acotado de settlement de puntos hacia PASG.

## Referencias

- Codigo: `contracts/core/streaming-billing/src/msg.rs`
- Contexto local: `contracts/core/streaming-billing/README.md`
- Guia compartida: `docs/es/relationships.md`
- Guia de instanciacion: `docs/es/instantiate-guides.md`
