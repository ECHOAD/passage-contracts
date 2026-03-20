# Modelo compuesto de NFT y estado

## Proposito

Los integradores de Passage deben tratar un asset visible para el usuario como un activo compuesto construido desde un NFT base mas cualquier modulo de estado dedicado que lleve estado mutable relevante para el protocolo.

El NFT base conserva identidad duradera, ownership, approvals, royalties, tipo de asset y el puntero `token_uri` para manifiestos. Un modulo de estado conserva hechos mutables que deben seguir siendo consultables on-chain sin convertirlos en metadata NFT generica.

## Por que Passage usa un modelo de activo compuesto

El usuario ve un solo asset, aunque la superficie del protocolo este dividida:

- El NFT base prueba ownership y derechos duraderos.
- El manifiesto detras de `token_uri` lleva payloads de render, descriptores runtime y compatibilidad detallada.
- Un modulo de estado persiste hechos mutables cuando esos hechos requieren confianza on-chain.

Esto mantiene la metadata NFT generica pequena y durable, mientras wallets, worlds e indexadores pueden presentar un solo activo compuesto.

## Responsabilidades del NFT base

El NFT base es la fuente canonica para:

- identidad del asset y pertenencia a la coleccion
- ownership y approvals
- semanticas tipadas de Passage como `nft_type`
- derechos duraderos de monetizacion o transferencia
- marcadores minimos de compatibilidad como `profile_id`

El NFT base no es donde Passage guarda progresion mutable, estado de instalacion u otro estado cambiante del world.

## Responsabilidades del modulo de estado

Un modulo de estado existe cuando los hechos mutables necesitan visibilidad de protocolo.

Ejemplos en Phase 11:

- [`avatar-progression`](../contracts/nft/avatar-progression.md) guarda save points de progresion por world para avatars o companions.
- [`world-plugin-assignment`](../contracts/relationship/world-plugin-assignment.md) guarda derechos duraderos de asignacion plugin-a-world.

Ambos son superficies de estado respaldadas por contrato. No son campos de metadata generica en el NFT.

## Progresion como estado compuesto

Para avatars y companions, el NFT sigue siendo la identidad duradera del asset. Los snapshots de progresion se guardan aparte en `avatar-progression` en save points.

Esto significa que:

- las formulas de gameplay siguen definidas por cada world y off-chain
- el usuario sigue experimentando un solo asset de avatar
- indexadores y clientes pueden componer el NFT base con el snapshot de progresion mas reciente

## Derechos de plugin como estado compuesto

Para plugins, el NFT del plugin prueba ownership o semanticas de licencia. Un world que recibe una asignacion duradera registra ese derecho en `world-plugin-assignment`.

Esto significa que:

- el NFT base no necesita campos de estado de instalacion
- un world puede conservar su asignacion duradera despues de una reventa posterior del plugin
- los integradores pueden consultar estado de asignacion exigible directamente sin inferirlo desde metadata

## Guia de integracion

Cuando una app de Passage presente un solo activo compuesto, debe:

1. Leer el NFT base para identidad, ownership y semanticas duraderas.
2. Resolver el manifiesto detras de `token_uri` para render y detalle runtime.
3. Consultar el modulo de estado relevante para hechos mutables del protocolo.

Si no existe un modulo de estado dedicado, el asset debe tratarse como solo NFT en vez de empujar hechos mutables a metadata generica.
