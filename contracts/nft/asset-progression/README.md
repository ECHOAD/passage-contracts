# asset-progression

`asset-progression` is the dedicated Passage state surface for world-scoped avatar and companion progression snapshots.

It persists save-point snapshots keyed by real asset identity:

- `collection`
- `token_id`
- `asset_kind`
- `world`

The NFT remains the durable identity and rights surface. Mutable gameplay progression is stored here instead of being written back into generic NFT metadata.

## Authorization Model

- Snapshot writes resolve against the live NFT collection contract.
- The sender must be the current owner or an active CW721 approval for the token.
- The token metadata must identify the asset as an `avatar` or `companion`.

## Scope

- The contract stores snapshots such as `level`, `xp`, and opaque checkpoint references.
- Gameplay formulas remain world-defined and off-chain.
- The contract standardizes save-point persistence, not per-action progression logic.
