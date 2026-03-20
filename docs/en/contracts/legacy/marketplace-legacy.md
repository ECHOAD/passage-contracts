# marketplace-legacy

## Status

Historical / Reference

## Purpose

Preserves the older marketplace surface as historical/reference material.

## Instantiation

Check `contracts/nft/marketplace-legacy/src/msg.rs` and `schema/` if you need the exact historical payload. This page does not recommend the contract as the default path for new work.

## Actors and permissions

Actors depend on the historical model of this contract. Read it as compatibility, migration, or historical reference.

## Key messages

- Historical surface preserved for reference.
- May still matter for older deployments or compatibility paths.
- Not the recommended default for new integrations.

## Relationships

- Lives under `docs/en/contracts/legacy` because it should be read comparatively and historically.
- Should be contrasted against the current equivalent surface.
- Prefer `marketplace-v3` for current integrations.

## Hypothetical example

Hypothetical flow: an integrator maintains an older deployment and needs to understand how `marketplace-legacy` behaved before moving to the current recommended path.

## References

- Code: `contracts/nft/marketplace-legacy/src/msg.rs`
- Local context: `contracts/nft/marketplace-legacy/README.md`
- Current related contract: check the current docs under `docs/en/contracts/nft/`.
