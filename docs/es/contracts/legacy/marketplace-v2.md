# marketplace-v2

## Estado

Historica / Referencia

## Proposito

Preserva la superficie del marketplace v2 como material historico/de referencia y contexto de compatibilidad.

## Instanciacion

Consulta `contracts/nft/marketplace-v2/src/msg.rs` y `schema/` si necesitas entender el payload historico exacto. Esta pagina no recomienda usar este contrato como default para trabajo nuevo.

## Actores y permisos

Los actores dependen del modelo historico de este contrato. Debe leerse como referencia para compatibilidad, migracion o comprension del pasado del repo.

## Mensajes clave

- Superficie historica conservada para referencia.
- Puede seguir siendo relevante para despliegues antiguos o rutas de compatibilidad.
- No es la ruta recomendada para nuevas integraciones.

## Relaciones

- Vive en `docs/es/contracts/legacy` porque su lectura correcta es historica y comparativa.
- Debe contrastarse con la superficie actual equivalente.
- Prefiere `marketplace-v3` para el modelo actual de marketplace.

## Ejemplo hipotetico

Flujo hipotetico: un integrador mantiene una implementacion antigua y necesita entender como funcionaba `marketplace-v2` antes de migrar al camino actual recomendado.

## Referencias

- Codigo: `contracts/nft/marketplace-v2/src/msg.rs`
- Contexto local: `contracts/nft/marketplace-v2/README.md`
- Contrato actual relacionado: revisa la documentacion actual en `docs/es/contracts/nft/`.
