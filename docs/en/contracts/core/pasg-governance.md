# pasg-governance

## Status

Current

## Purpose

Provides the PASG-holder proposal, voting, delegation, and ratification layer without replacing multisig as the admin executor.

## Instantiation

Instantiate with the PASG-denom assumptions, governance thresholds, voting windows, delegation rules, and the target admin multisig for ratified protocol actions.

Real payload source of truth: `contracts/core/pasg-governance/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

PASG holders, delegates, proposers, and the admin multisig that consumes ratified actions.

## Key messages

- Accepts PASG deposits for voting power.
- Tracks proposals, votes, delegation, quorum, and pass thresholds.
- Publishes typed ratified admin actions for multisig execution.

## Relationships

- Depends on `multisig` for final protocol admin execution.
- Touches PASG utility and parameter surfaces, not general off-chain business systems.
- Can point into contracts such as `streaming-billing`, `marketplace-v3`, and `auction-english` through typed actions.

## Hypothetical example

Hypothetical flow: PASG holders pass a proposal to update marketplace parameters; governance records the ratified action and multisig executes the target contract update.

## References

- Code: `contracts/core/pasg-governance/src/msg.rs`
- Local context: `contracts/core/pasg-governance/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
