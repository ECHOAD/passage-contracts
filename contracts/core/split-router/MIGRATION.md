# Split Router Upgrade Guide

`split-router` currently has no `migrate` entrypoint. Upgrades are deploy-and-cutover.

## Upgrade Steps

1. Deploy new router code as a new contract instance.
2. Recreate distribution rules with `SetDistributionRule`.
3. Recreate split wallets if used.
4. Repoint producer contracts:
   - `marketplace-v3` -> `UpdateConfig { split_router, use_split_router }`
   - `minter-v2` -> `UpdateConfig { split_router, use_split_router }`
5. Execute low-value primary + secondary sale tests.
6. After validation, retire old router references.

## Operational Notes

- Any funds already routed to the previous router remain there.
- Settle/distribute outstanding balances before full decommission.
- Keep indexer/event consumers pointed to both routers during cutover window.
