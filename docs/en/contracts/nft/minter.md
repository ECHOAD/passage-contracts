# minter

## Status

Compatibility

## Purpose

Represents the older primary-sale minter surface that still exists in the workspace for compatibility and reference.

## Instantiation

Instantiate with the older collection deployment and sale configuration expected by this minter generation.

Real payload source of truth: `contracts/nft/minter/src/msg.rs` and `schema/` when available.

## Actors and permissions

Drop admin, buyers, the deployed collection, and any payout route used by the older sale flow.

## Key messages

- Runs the earlier primary-sale flow.
- Deploys or coordinates with a collection for minting.
- Remains useful mainly for compatibility or historical understanding compared to v2.

## Relationships

- Precedes `minter-v2` in the evolution of primary-sale tooling.
- Still sits around collection deployment and sale control.
- Should be read together with `minter-v2` to understand the current recommended path.

## Hypothetical example

Hypothetical flow: an integration maintained from an older Passage deployment still references `minter`, even though new work should usually look at `minter-v2` first.

## References

- Code: `contracts/nft/minter/src/msg.rs`
- Local context: `contracts/nft/minter/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
