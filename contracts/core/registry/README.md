# Passage Registry Contract

The Registry contract is the on-chain source of truth for Passage marketplace structure:

- `Ecosystem`: top-level ownership/admin domain.
- `Collection`: pg721 collection associated directly to one ecosystem.

`World` is no longer modeled on-chain in this contract.

## Overview

Hierarchy:

```
Ecosystem
  -> Collection (pg721)
```

## Features

### Ecosystem management

- Register approved ecosystems from `ecosystem-factory`.
- Update ecosystem metadata/admin.
- Approve/revoke ecosystem members (team) who can add collections.
- Apply creator, ecosystem, and collection moderation.

### Collection registration

- Register new collections in an ecosystem.
- Register existing collections retroactively (`admin` only).
- Update collection metadata and linked runtime contracts (`minter`, `marketplace`).
- Transfer collection ownership.

### Minter authorization

- Authorize/revoke minter contracts per collection.
- Query authorization status.

## Messages

### Instantiate

```json
{
  "admin": "passage1...",
  "operators": ["passage1..."],
  "recovery_council": ["passage1..."],
  "ecosystem_factory": null
}
```

After deployment, wire the authorized factory with `UpdateConfig { ecosystem_factory }`. Direct ecosystem creation is not supported.

### Execute

- `UpdateConfig`
- `UpdateCreatorModeration`
- `UpdateEcosystemModeration`
- `UpdateCollectionModeration`
- `SetEcosystemRecoveryPolicy`
- `SetCollectionRecoveryPolicy`
- `RegisterEcosystemFromFactory`
- `UpdateEcosystem`
- `ApproveEcosystemMember { ecosystem_id, member }`
- `RevokeEcosystemMember { ecosystem_id, member }`
- `RegisterCollection { address, ecosystem_id, name }`
- `RegisterExistingCollection { address, ecosystem_id, name, creator }`
- `UpdateCollection`
- `TransferCollectionOwnership`
- `AuthorizeMinter`
- `RevokeMinter`
- `UpdateRecoveryConfig`
- `OpenRecoveryCase`
- `ContestRecoveryCase`
- `ResolveRecoveryCase`

### Query

```rust
Config {}

Ecosystem { id }
Ecosystems { start_after, limit }
EcosystemsByAdmin { admin, start_after, limit }
CanCreateEcosystem { creator }
CreatorModeration { creator }
EcosystemModeration { ecosystem_id }
EcosystemRecoveryPolicy { ecosystem_id }
IsEcosystemMember { ecosystem_id, member }

Collection { address }
Collections { start_after, limit }
CollectionsByEcosystem { ecosystem_id, start_after, limit }
CollectionsByCreator { creator, start_after, limit }
IsCollectionVerified { address }
CanMintCollection { address }
CanTradeCollection { address }
CollectionModeration { address }
CollectionRecoveryPolicy { address }

IsMinterAuthorized { collection_address, minter_address }
AuthorizedMinters { collection_address, start_after, limit }

RecoveryConfig {}
RecoveryCase { case_id }
RecoveryCases { status, start_after, limit }
```

## Access control

| Action | Who can execute |
|--------|-----------------|
| Register Ecosystem | Configured `ecosystem-factory` only |
| Update Ecosystem | Ecosystem admin or contract admin |
| Approve Ecosystem Member | Ecosystem admin or contract admin |
| Register Collection | Ecosystem admin, approved ecosystem member, or contract admin |
| Register Existing Collection | Contract admin only |
| Update Collection | Collection creator or contract admin |
| Set verified status | Contract admin only |
| Authorize/Revoke minter | Collection creator or contract admin |
| Resolve Recovery Case | Recovery council or contract admin |

## Migration strategy

`registry` has no `migrate` entrypoint. Use deploy-and-backfill.
See [`MIGRATION.md`](./MIGRATION.md).

## State structure

```rust
Config {
    admin: Addr,
    operators: Vec<Addr>,
    recovery_council: Vec<Addr>,
    ecosystem_factory: Option<Addr>,
    paused: bool,
}

Ecosystem {
    id: String,
    name: String,
    admin: Addr,
    description: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
    created_at: u64,
    updated_at: u64,
}

Collection {
    address: Addr,
    ecosystem_id: String,
    name: String,
    creator: Addr,
    verified: bool,
    minter: Option<Addr>,
    marketplace: Option<Addr>,
    created_at: u64,
    updated_at: u64,
}
```

## Events

All execute messages emit attributes for indexers:

- `action`
- entity identifiers such as `ecosystem_id` and `collection`
- actor addresses such as `admin` and `creator`

## License

Apache-2.0
