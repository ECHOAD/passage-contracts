# ecosystem-factory

## Status

Current

## Purpose

Implements the governed workflow for creating ecosystems and their initial collection-factory setup.

## Instantiation

Instantiate with registry linkage, the child collection-factory code path or deployment configuration, and the admin/operator surface that approves ecosystem creation.

Real payload source of truth: `contracts/core/ecosystem-factory/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Protocol admin, ecosystem applicants, registry, and the collection-factory created for each approved ecosystem.

## Key messages

- Receives ecosystem creation requests or governed create flows.
- Registers approved ecosystems in registry.
- Deploys or links the per-ecosystem collection-factory.

## Relationships

- Feeds canonical ecosystem state into `registry`.
- Creates the environment that `collection-factory` uses for collection deployment.
- Sits under the admin plane rather than replacing registry.

## Hypothetical example

Hypothetical flow: a new creator universe is approved through `ecosystem-factory`, which registers the ecosystem and prepares the collection deployment surface the team will use next.

## References

- Code: `contracts/core/ecosystem-factory/src/msg.rs`
- Local context: `contracts/core/ecosystem-factory/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
