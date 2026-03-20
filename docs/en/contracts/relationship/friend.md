# friend

## Status

Current

## Purpose

Implements a friendship-style relationship primitive for Passage identities.

## Instantiation

Instantiate with the admin surface and any config needed for maintaining mutual or friend-like relationships.

Real payload source of truth: `contracts/relationship/friend/src/msg.rs` and `schema/` when available.

## Actors and permissions

Users requesting friendship, users accepting friendship, and any admin or moderation role.

## Key messages

- Creates and manages friendship-style social edges.
- Exposes social relationship queries.
- Remains separate from NFT ownership or economic policy.

## Relationships

- Lives beside `follow` as another relationship primitive.
- May support off-chain social UX.
- Does not replace registry or collection membership semantics.

## Hypothetical example

Hypothetical flow: an app queries `friend` to decide whether two players are mutual contacts before enabling a social feature.

## References

- Code: `contracts/relationship/friend/src/msg.rs`
- Local context: `contracts/relationship/friend/README.md`
- Shared guide: `docs/en/relationships.md`
