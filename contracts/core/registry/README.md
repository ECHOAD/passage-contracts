# registry

`registry` is the canonical Passage ledger for ecosystems, collection affiliation, creator provenance, and authorized minters.

## Core Model

- An ecosystem is an admin/team context, for example `Cyberpunk Universe`.
- A collection is its own on-chain contract.
- `registry` tracks which ecosystem, if any, currently hosts that collection.
- A collection can be deregistered from an ecosystem and remain alive on-chain.
- A deregistered collection can later be re-homed into a different ecosystem.
- Re-homing changes affiliation, not creator provenance.

## Main Execute Messages

- `UpdateConfig`
- `UpdateCreatorModeration`
- `UpdateEcosystemModeration`
- `UpdateCollectionModeration`
- `SetEcosystemRecoveryPolicy`
- `SetCollectionRecoveryPolicy`
- `RegisterEcosystemFromFactory`
- `UpdateEcosystem`
- `ApproveEcosystemMember`
- `RevokeEcosystemMember`
- `RegisterCollection`
- `RegisterCollectionFromFactory`
- `RegisterExistingCollection`
- `DeregisterCollection`
- `RehomeCollection`
- `UpdateCollection`
- `TransferCollectionOwnership`
- `AuthorizeMinter`
- `RevokeMinter`
- `UpdateRecoveryConfig`
- `OpenRecoveryCase`
- `ContestRecoveryCase`
- `ResolveRecoveryCase`

## Main Queries

- `Config`
- `Ecosystem`
- `Ecosystems`
- `EcosystemsByAdmin`
- `CanCreateEcosystem`
- `IsEcosystemMember`
- `CanCreateCollectionInEcosystem`
- `Collection`
- `Collections`
- `CollectionsByEcosystem`
- `UnaffiliatedCollections`
- `CollectionsByCreator`
- `CollectionsByNftType`
- `CanMintCollection`
- `CanTradeCollection`
- `IsMinterAuthorized`
- `AuthorizedMinters`

## Authorization Model

- `ecosystem-factory` is the only ecosystem creation path.
- Ecosystem admin and approved ecosystem members can add collections directly.
- Collection request and approval flow is not part of the current model.
- Unwanted collections are handled by membership revocation or by `DeregisterCollection`.

## Collection Lifecycle

1. Ecosystem is approved and registered.
2. Ecosystem admin or approved member creates a collection through the dedicated `collection-factory`, or admin manually registers an existing collection.
3. `registry` stores the collection address as the canonical collection identity.
4. The collection can later be:
   - updated
   - deregistered from its ecosystem
   - re-homed into another ecosystem
5. The original collection creator remains the creator of record unless `TransferCollectionOwnership` is used explicitly.

## Typed Asset Scope

`registry` stores `nft_type` at collection level so integrators can reason about creator asset classes without querying runtime or rendering systems. Runtime payloads, Unreal data, and rendering behavior stay off-chain.
