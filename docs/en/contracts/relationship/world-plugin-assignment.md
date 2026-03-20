# world-plugin-assignment

## Status

Current

## Purpose

Defines the dedicated Passage relationship surface for durable plugin-to-world assignment rights.

This contract keeps plugin ownership separate from world usage rights. The plugin NFT proves ownership, the world NFT proves world authority, and this relationship surface records the durable assignment that survives later resale.

## Instantiation

Instantiate with:

- `InstantiateMsg {}`

Real payload source of truth: `contracts/relationship/world-plugin-assignment/src/msg.rs` and `contracts/relationship/world-plugin-assignment/schema/`.

## Actors and permissions

- The plugin owner or approved operator can create an assignment.
- The world owner or approved operator must also authorize the assignment target.
- Anyone can query current assignments.

## Key messages

- `Assign`: persist a durable plugin-to-world assignment record.
- `Remove`: remove an existing assignment record.

## Key queries

- `Assignment`
- `AssignmentsByWorld`
- `AssignmentsByPlugin`

## Boundary notes

- Durable assignment state is on-chain and queryable in this contract, not embedded in generic NFT metadata.
- Plugin binaries, deployment steps, runtime permissions, and installation mechanics remain off-chain.
- Manifest data behind `token_uri` can describe plugin details, but the enforceable assignment relationship lives here.

## Relationships

- Reads plugin ownership from the relevant plugin NFT collection.
- Reads world authority from the relevant world NFT collection.
- Complements `pg721` and `pg721-updatable` without expanding their typed metadata model.

## Hypothetical example

Hypothetical flow: a creator assigns a plugin asset to a world, later transfers the plugin NFT to a new owner, and integrators still query this contract to confirm the world retained its durable assignment right.

## References

- Code: `contracts/relationship/world-plugin-assignment/src/msg.rs`
- Local context: `contracts/relationship/world-plugin-assignment/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
