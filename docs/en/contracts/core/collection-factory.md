# collection-factory

## Status

Current

## Purpose

Deploys collection contracts for an ecosystem and pushes the canonical registration flow into registry.

## Instantiation

Instantiate with the registry address, the ecosystem context, the collection code ID, and the admin/operator surface that is allowed to deploy child collections.

Real payload source of truth: `contracts/core/collection-factory/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Ecosystem admin, approved ecosystem members, registry, and the deployed collection contract.

## Key messages

- Creates collections inside an ecosystem context.
- Registers the resulting collection in registry.
- Tracks deployment and reply-path lifecycle for child contracts.

## Relationships

- Depends on `registry` as the canonical ledger.
- Works downstream from `ecosystem-factory` because ecosystems must exist first.
- Usually deploys `pg721` or related collection variants.

## Hypothetical example

Hypothetical flow: a Cyberpunk Universe admin uses `collection-factory` to deploy a world collection, and the new collection address is registered immediately so trading and mint permissions can be checked later.

## References

- Code: `contracts/core/collection-factory/src/msg.rs`
- Local context: `contracts/core/collection-factory/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
