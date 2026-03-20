# split-router

## Status

Current

## Purpose

Routes creator-side and contract-directed revenue generically without owning PASG policy itself.

## Instantiation

Instantiate with the payout or routing configuration your flow needs, plus any admin/operator permissions for updating routing behavior.

Real payload source of truth: `contracts/core/split-router/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Calling contracts such as minters or marketplaces, payout recipients, and any admin/operator that can update routing config.

## Key messages

- Accepts generic split or route instructions.
- Distributes attached funds across recipients.
- Remains denom-agnostic even when PASG-aware contracts call into it.

## Relationships

- Used by creator-commerce flows that need revenue distribution.
- Can be called by NFT sale or royalty flows.
- Should be read together with `streaming-billing` when PASG semantics matter, because PASG policy does not live here.

## Hypothetical example

Hypothetical flow: a sale contract sends funds into `split-router` so creator, collaborators, and treasury can receive their portions using one generic route surface.

## References

- Code: `contracts/core/split-router/src/msg.rs`
- Local context: `contracts/core/split-router/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
