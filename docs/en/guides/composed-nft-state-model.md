# Composed NFT + State Model

## Purpose

Passage integrators should treat a user-facing asset as one composed asset built from a base NFT plus any dedicated state module that carries mutable protocol state.

The base NFT keeps durable identity, ownership, approvals, royalties, asset type, and the `token_uri` pointer used for manifests. A state module keeps mutable facts that must remain queryable on-chain without being reduced to generic NFT metadata.

## Why Passage uses a composed asset model

Users see one asset. Protocol surfaces may still be split:

- The base NFT proves ownership and durable rights.
- The manifest behind `token_uri` carries rendering payloads, runtime descriptors, and detailed compatibility data.
- A state module persists mutable protocol facts when those facts need on-chain trust.

This keeps generic NFT metadata small and durable while still letting product teams present one composed asset in wallets, world UIs, and indexers.

## Base NFT responsibilities

The base NFT is the canonical source for:

- Asset identity and collection membership
- Ownership and approvals
- Typed Passage semantics such as `nft_type`
- Durable monetization or transfer rights
- Minimal compatibility markers such as `profile_id`

The base NFT is not where Passage stores mutable progression, installation state, or other change-heavy world state.

## State module responsibilities

A state module exists when mutable facts need protocol visibility.

Examples in Phase 11:

- [`avatar-progression`](../contracts/nft/avatar-progression.md) stores world-scoped progression save points for avatars or companions.
- [`world-plugin-assignment`](../contracts/relationship/world-plugin-assignment.md) stores durable plugin-to-world assignment rights.

Both are contract-backed state surfaces. They are not generic metadata fields on the NFT itself.

## Progression as composed state

For avatars and companions, the NFT remains the durable asset identity. Progression snapshots are saved separately in `avatar-progression` at save points.

This means:

- gameplay formulas stay world-defined and off-chain
- users still experience one avatar asset
- indexers and clients can compose the base NFT with the latest progression snapshot

## Plugin rights as composed state

For plugins, the plugin NFT proves ownership or licensing semantics. A world that receives a durable assignment records that right in `world-plugin-assignment`.

This means:

- the base NFT does not need install-state fields
- a world can keep its durable assignment after later plugin resale
- integrators can query enforceable assignment state directly instead of inferring it from metadata

## Integration guidance

When Passage apps present one composed asset, they should:

1. Read the base NFT for identity, ownership, and durable semantics.
2. Resolve the manifest behind `token_uri` for rendering and runtime detail.
3. Query any relevant state module for mutable protocol facts.

If no dedicated state module exists, the asset should be treated as NFT-only rather than pushing mutable facts into generic metadata.
