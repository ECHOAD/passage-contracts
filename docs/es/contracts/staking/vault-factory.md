# vault-factory

## Estado

Actual

## Proposito

Despliega instancias de NFT vault y la infraestructura de rewards relacionada para configuraciones de staking NFT.

## Instanciacion

Se instancia con code IDs de vault, parametros para desplegar modulos de rewards y la superficie admin/operator autorizada para crear nuevos vaults.

Fuente real de payload: `contracts/staking/vault-factory/src/msg.rs` y `schema/` cuando exista.

## Actores y permisos

Admin/operator, vaults desplegados, reward accounts y usuarios que luego hacen stake en esos vaults.

## Mensajes clave

- Crea instancias de vault.
- Coordina el despliegue de modulos de rewards de soporte.
- Estandariza la configuracion de staking NFT entre colecciones o programas.

## Relaciones

- Se ubica aguas arriba de `nft-vault` y `stake-rewards`.
- Existe solo dentro de la familia de staking NFT.
- No se superpone con la delegacion a validators para PASG.

## Ejemplo hipotetico

Flujo hipotetico: un operador usa `vault-factory` para desplegar un nuevo programa de staking NFT en vez de instanciar a mano cada componente.

## Referencias

- Codigo: `contracts/staking/vault-factory/src/msg.rs`
- Contexto local: `contracts/staking/vault-factory/README.md`
- Guia compartida: `docs/es/relationships.md`
