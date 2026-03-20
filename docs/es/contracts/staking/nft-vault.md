# nft-vault

## Estado

Actual

## Proposito

Provee staking de NFT con tiempos de unstake/claim e integracion con reward accounts.

## Instanciacion

Se instancia con colecciones aprobadas, code o enlaces a reward accounts y configuracion de duracion de unstake.

Fuente real de payload: `contracts/staking/nft-vault/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Stakers de NFT, admin del vault, reward accounts y claimers.

## Mensajes clave

- Acepta flujos de staking y unstaking de NFT.
- Gestiona ventanas de claim para NFTs unstaked.
- Coordina con modulos de rewards para reclamo de recompensas.

## Relaciones

- Trabaja con `stake-rewards` y `vault-factory`.
- Pertenece al staking de NFT, no al staking nativo PASG hacia validators.
- Puede adjuntarse a colecciones NFT aprobadas.

## Ejemplo hipotetico

Flujo hipotetico: un usuario stakea NFTs de una coleccion aprobada en `nft-vault`, espera el periodo de unstake y luego reclama sus NFTs.

## Referencias

- Codigo: `contracts/staking/nft-vault/src/msg.rs`
- Contexto local: `contracts/staking/nft-vault/README.md`
- Guia compartida: `docs/es/relationships.md`
