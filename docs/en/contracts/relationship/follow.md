# follow

## Status

Current

## Purpose

Implements a social follow primitive for Passage accounts or identities.

## Instantiation

Instantiate with the admin surface and any config required for maintaining the follow graph.

Real payload source of truth: `contracts/relationship/follow/src/msg.rs` and `schema/` when available.

## Actors and permissions

Users who follow, users being followed, and admins/operators if moderation exists.

## Key messages

- Records follow-style social relationships.
- Exposes query surfaces around those relationships.
- Stays outside creator-commerce, governance, and staking policy.

## Relationships

- Belongs to the relationship family rather than the economic stack.
- Can complement off-chain profile or social features.
- Does not drive registry, marketplace, or PASG utility semantics.

## Hypothetical example

Hypothetical flow: a profile layer queries `follow` to show who a creator account follows, while commerce contracts remain unaffected.

## References

- Code: `contracts/relationship/follow/src/msg.rs`
- Local context: `contracts/relationship/follow/README.md`
- Shared guide: `docs/en/relationships.md`
