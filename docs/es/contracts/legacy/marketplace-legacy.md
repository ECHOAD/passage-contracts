# marketplace-legacy

## Estado

Historica / Referencia

## Proposito

Preserva la superficie antigua de marketplace como material historico/de referencia.

## Instanciacion

Consulta `contracts/nft/marketplace-legacy/src/msg.rs` y `schema/` si necesitas entender el payload historico exacto. Esta pagina no recomienda usar este contrato como default para trabajo nuevo.

## Actores y permisos

Los actores dependen del modelo historico de este contrato. Debe leerse como referencia para compatibilidad, migracion o comprension del pasado del repo.

## Mensajes clave

- Superficie historica conservada para referencia.
- Puede seguir siendo relevante para despliegues antiguos o rutas de compatibilidad.
- No es la ruta recomendada para nuevas integraciones.

## Relaciones

- Vive en `docs/es/contracts/legacy` porque su lectura correcta es historica y comparativa.
- Debe contrastarse con la superficie actual equivalente.
- Prefiere `marketplace-v3` para integraciones actuales.

## Ejemplo hipotetico

Flujo hipotetico: un integrador mantiene una implementacion antigua y necesita entender como funcionaba `marketplace-legacy` antes de migrar al camino actual recomendado.

## Referencias

- Codigo: `contracts/nft/marketplace-legacy/src/msg.rs`
- Contexto local: `contracts/nft/marketplace-legacy/README.md`
- Contrato actual relacionado: revisa la documentacion actual en `docs/es/contracts/nft/`.
