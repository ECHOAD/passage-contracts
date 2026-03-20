# friend

## Estado

Actual

## Proposito

Implementa una primitiva de amistad para identidades Passage.

## Instanciacion

Se instancia con la superficie admin y cualquier config necesaria para mantener relaciones mutuas o de amistad.

Fuente real de payload: `contracts/relationship/friend/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Usuarios que solicitan amistad, usuarios que la aceptan y cualquier rol admin o de moderacion.

## Mensajes clave

- Crea y gestiona edges sociales tipo amistad.
- Expone queries de relacion social.
- Permanece separado del ownership NFT y la politica economica.

## Relaciones

- Vive junto a `follow` como otra primitiva de relationship.
- Puede soportar UX social off-chain.
- No reemplaza semanticas de registry ni membresia de coleccion.

## Ejemplo hipotetico

Flujo hipotetico: una app consulta `friend` para decidir si dos jugadores son contactos mutuos antes de habilitar una feature social.

## Referencias

- Codigo: `contracts/relationship/friend/src/msg.rs`
- Contexto local: `contracts/relationship/friend/README.md`
- Guia compartida: `docs/es/relationships.md`
