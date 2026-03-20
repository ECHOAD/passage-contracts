# streaming-billing

## Status

Current

## Purpose

Defines the canonical PASG utility interface and the bounded billing or local-economy surfaces that remain on-chain in this repo.

## Instantiation

Instantiate with registry and billing-related links, operator/admin permissions, PASG assumptions, and any world-scoped billing configuration this service-owned contract surface requires.

Real payload source of truth: `contracts/core/streaming-billing/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Backend operators, worlds or services that trigger billing events, PASG-aware integrators, and query consumers.

## Key messages

- Exposes `PasgUtility` and supplemental PASG-aware queries.
- Handles bounded deposit, reporting, withdrawal, and revenue-distribution surfaces.
- Can preview optional world-local economy semantics without replacing core creator commerce.

## Relationships

- Acts as the source of truth for PASG semantics consumed by the rest of the repo.
- Connects to `split-router` for revenue routing while preserving generic routing boundaries.
- Must stay within the on-chain/off-chain boundary fixed in earlier phases.

## Hypothetical example

Hypothetical flow: an integrator queries `PasgUtility` to confirm the native PASG denom and then uses billing queries to preview a bounded points-to-PASG world settlement path.

## References

- Code: `contracts/core/streaming-billing/src/msg.rs`
- Local context: `contracts/core/streaming-billing/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
