# whitelist

## Status

Current / Supporting

## Purpose

Provides an allowlist-style surface for NFT sale or mint access control.

## Instantiation

Instantiate with the admin surface, allowlist configuration, and any timing or membership assumptions required by the sale flow that uses it.

Real payload source of truth: `contracts/nft/whitelist/src/msg.rs` and `schema/` when available.

## Actors and permissions

Sale admin, whitelisted buyers, and the sale or mint contract that checks allowlist state.

## Key messages

- Adds or manages allowlisted addresses.
- Exposes allowlist eligibility checks.
- Supports sale flows that need gated access before public mint or purchase.

## Relationships

- Can be used by minter or sale flows.
- Does not replace collection or marketplace logic; it only constrains who may participate.
- Should be read as an auxiliary access-control primitive.

## Hypothetical example

Hypothetical flow: a creator drop uses `whitelist` so only pre-approved buyers can mint during the early access window.

## References

- Code: `contracts/nft/whitelist/src/msg.rs`
- Local context: `contracts/nft/whitelist/README.md`
- Shared guide: `docs/en/relationships.md`
- Flows: `docs/en/flows.md`
