# collection-factory

`collection-factory` deploys `pg721` collections inside one ecosystem context and registers them in `registry`.

## Role

- Ecosystem-scoped factory, one instance per ecosystem.
- Enforces local creator allowlist only when `enforce_local_allowlist = true`.
- Always relies on `registry.CanCreateCollectionInEcosystem` for the final authorization check.
- Registers newly instantiated collections through `registry.RegisterCollectionFromFactory`.

## Model

- Ecosystem admin and approved ecosystem members can create collections directly.
- Collection request or approval flow does not live here.
- A collection is an independent on-chain contract. `registry` tracks its current ecosystem affiliation.

## Instantiate

```json
{
  "admin": "passage1ecosystemadmin...",
  "operators": ["passage1operator..."],
  "registry": "passage1registry...",
  "ecosystem_id": "cyberpunk",
  "collection_code_id": 301,
  "enforce_local_allowlist": false,
  "approved_creators": null
}
```

## Main Execute Messages

- `UpdateConfig`
- `ApproveCreator`
- `RevokeCreator`
- `CreateCollection`

## CreateCollection

`CreateCollection` instantiates a `pg721` contract and forwards the resulting address to `registry`.

```json
{
  "create_collection": {
    "name": "Cyberpunk Wearables",
    "symbol": "CYBER",
    "minter": "passage1creator...",
    "collection_info": {
      "creator": "passage1creator...",
      "description": "Wearables for the Cyberpunk ecosystem",
      "image": "ipfs://bafy.../cover.png",
      "external_link": "https://example.com/cyber",
      "royalty_info": {
        "payment_address": "passage1royalty...",
        "share": "0.05"
      }
    },
    "label": "cyber-wearables"
  }
}
```

## Queries

- `Config`
- `IsCreatorApproved`
- `ApprovedCreators`
- `Collection`
- `Collections`
- `CollectionsByCreator`

## Notes

- Runtime or rendering payloads stay off-chain.
- Typed asset semantics live in the shared `pg721` family, not in `collection-factory`.
