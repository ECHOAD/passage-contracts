# Contract Relationships

## Primary map

- `ecosystem-factory` drives the approved workflow for new ecosystems.
- `registry` records ecosystems, membership, and canonical collection affiliation.
- `collection-factory` deploys `pg721*` collections inside an ecosystem and registers them.
- `minter-v2*` can also deploy its own collection, which must then be registered.
- `marketplace-v3` and `auction-english` depend on collection state and `registry` trade rules.
- `royalty-group` and `split-router` help distribute revenue and royalties.
- `multisig` remains the admin-owner execution plane.
- `pasg-governance` ratifies PASG decisions and hands typed actions to `multisig`.
- `streaming-billing` exposes the canonical PASG utility surface.

## Administrative relationship

1. `multisig` administers protocol contracts.
2. `pasg-governance` makes PASG-scoped decisions.
3. `multisig` executes approved admin changes when required.

## Asset relationship

1. An `ecosystem` is the parent context.
2. `collection-factory` or manual registration introduces a collection.
3. `registry` stores affiliation and mint/trade authorization.
4. `pg721*` maintains NFT collection logic.
5. `minter*` covers primary sale flows when used.
6. `marketplace-v3` and `auction-english` cover resale and auction flows.

## Economic relationship

- `streaming-billing` defines the canonical PASG utility read surface.
- `split-router` routes funds generically.
- `royalty-group` can distribute royalties to multiple recipients.
- `marketplace-v3` and `auction-english` execute secondary-sale settlement.

## Staking relationship

- `nft-vault`, `stake-rewards`, and `vault-factory` are NFT staking contracts.
- They are not the native PASG validator staking system.

## Interpretation rule

When narrative and code disagree, `msg.rs`, `schema/`, and the contract implementation win.
