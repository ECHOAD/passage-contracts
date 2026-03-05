# Registry Upgrade Guide

`registry` currently has no `migrate` entrypoint. Upgrades are deploy-and-backfill.

## Upgrade Steps

1. Deploy a new registry instance.
2. Approve ecosystem creators (if non-admin teams will register ecosystems) with `ApproveEcosystemCreator`.
3. Recreate ecosystems with `RegisterEcosystem` using required metadata:
   - `name`
   - `detail`
   - `image_urls` (at least one image URL)
4. Approve ecosystem team members with `ApproveEcosystemMember` when they must register collections.
5. Backfill existing collections with `RegisterExistingCollection` (now requires `ecosystem_id`).
6. Re-link runtime metadata per collection with `UpdateCollection`:
   - `minter`
   - `marketplace`
7. Re-apply minter permissions with `AuthorizeMinter`.
8. Switch indexer/backend read path to new registry.

## Verification Queries

- `Ecosystems`
- `CollectionsByEcosystem`
- `IsMinterAuthorized`
- `IsCollectionVerified`

## Operational Notes

- Maintain both old and new registry addresses in backend during cutover.
- Do not revoke old permissions until all producers are repointed and verified.
