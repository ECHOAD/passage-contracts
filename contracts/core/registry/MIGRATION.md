# Registry Upgrade Guide

`registry` currently has no `migrate` entrypoint. Upgrades are deploy-and-backfill.

## Upgrade Steps

1. Deploy a new registry instance.
2. Deploy a new `ecosystem-factory` pointing at the new registry with the correct `collection_factory_code_id` and `collection_code_id`.
3. Wire the new factory into the registry with `UpdateConfig { ecosystem_factory }`.
4. Recreate ecosystems by approving requests in `ecosystem-factory`, which will deploy a dedicated `collection-factory` and call `RegisterEcosystemFromFactory`.
5. Approve ecosystem team members with `ApproveEcosystemMember` when they must register collections.
6. Backfill existing collections with `RegisterExistingCollection` (now requires `ecosystem_id`).
7. Re-link runtime metadata per collection with `UpdateCollection`:
   - `minter`
   - `marketplace`
8. Re-apply minter permissions with `AuthorizeMinter`.
9. Switch indexer/backend read path to new registry.

## Verification Queries

- `Ecosystems`
- `CollectionsByEcosystem`
- `IsMinterAuthorized`
- `IsCollectionVerified`

## Operational Notes

- Maintain both old and new registry addresses in backend during cutover.
- Do not revoke old permissions until all producers are repointed and verified.
