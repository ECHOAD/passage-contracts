# pg721-legacy

## Estado

Historica / Referencia

## Proposito

Preserva el modelo legacy de coleccion como material historico/de referencia.

## Instanciacion

Consulta `contracts/nft/pg721-legacy/src/msg.rs` y `schema/` si necesitas entender el payload historico exacto. Esta pagina no recomienda usar este contrato como default para trabajo nuevo.

## Actores y permisos

Los actores dependen del modelo historico de este contrato. Debe leerse como referencia para compatibilidad, migracion o comprension del pasado del repo.

## Mensajes clave

- Superficie historica conservada para referencia.
- Puede seguir siendo relevante para despliegues antiguos o rutas de compatibilidad.
- No es la ruta recomendada para nuevas integraciones.

## Relaciones

- Vive en `docs/es/contracts/legacy` porque su lectura correcta es historica y comparativa.
- Debe contrastarse con la superficie actual equivalente.
- Prefiere `pg721` o `pg721-updatable` para trabajo actual de colecciones.

## Ejemplo hipotetico

Flujo hipotetico: un integrador mantiene una implementacion antigua y necesita entender como funcionaba `pg721-legacy` antes de migrar al camino actual recomendado.

## Referencias

- Codigo: `contracts/nft/pg721-legacy/src/msg.rs`
- Contexto local: `contracts/nft/pg721-legacy/README.md`
- Contrato actual relacionado: revisa la documentacion actual en `docs/es/contracts/nft/`.
