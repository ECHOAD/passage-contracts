# whitelist

## Estado

Actual / Soporte

## Proposito

Provee una superficie tipo allowlist para control de acceso en ventas o mint NFT.

## Instanciacion

Se instancia con la superficie admin, configuracion de allowlist y cualquier supuesto de tiempo o membresia requerido por el flujo de venta que la use.

Fuente real de payload: `contracts/nft/whitelist/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin de la venta, buyers incluidos en la lista y el contrato de venta o mint que consulta el estado de allowlist.

## Mensajes clave

- Agrega o gestiona direcciones allowlisted.
- Expone chequeos de elegibilidad de la lista.
- Soporta flujos de venta que necesitan acceso gated antes del mint o la compra publica.

## Relaciones

- Puede ser usado por flujos de minter o de venta.
- No reemplaza la logica de coleccion o marketplace; solo restringe quien puede participar.
- Debe leerse como una primitiva auxiliar de control de acceso.

## Ejemplo hipotetico

Flujo hipotetico: un creator drop usa `whitelist` para que solo buyers preaprobados puedan mintear durante la ventana de acceso temprano.

## Referencias

- Codigo: `contracts/nft/whitelist/src/msg.rs`
- Contexto local: `contracts/nft/whitelist/README.md`
- Guia compartida: `docs/es/relationships.md`
- Flujos: `docs/es/flows.md`
