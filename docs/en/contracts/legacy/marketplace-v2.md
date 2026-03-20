# marketplace-v2

## Status

Historical / Reference

## Purpose

Preserves the v2 marketplace surface as historical/reference material and compatibility context.

## Instantiation

Check `contracts/nft/marketplace-v2/src/msg.rs` and `schema/` if you need the exact historical payload. This page does not recommend the contract as the default path for new work.

## Actors and permissions

Actors depend on the historical model of this contract. Read it as compatibility, migration, or historical reference.

## Key messages

- Historical surface preserved for reference.
- May still matter for older deployments or compatibility paths.
- Not the recommended default for new integrations.

## Relationships

- Lives under `docs/en/contracts/legacy` because it should be read comparatively and historically.
- Should be contrasted against the current equivalent surface.
- Prefer `marketplace-v3` for the current marketplace model.

## Hypothetical example

Hypothetical flow: an integrator maintains an older deployment and needs to understand how `marketplace-v2` behaved before moving to the current recommended path.

## References

- Code: `contracts/nft/marketplace-v2/src/msg.rs`
- Local context: `contracts/nft/marketplace-v2/README.md`
- Current related contract: check the current docs under `docs/en/contracts/nft/`.
