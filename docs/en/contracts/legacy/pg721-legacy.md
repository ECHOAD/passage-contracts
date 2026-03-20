# pg721-legacy

## Status

Historical / Reference

## Purpose

Preserves the legacy collection model as historical/reference material.

## Instantiation

Check `contracts/nft/pg721-legacy/src/msg.rs` and `schema/` if you need the exact historical payload. This page does not recommend the contract as the default path for new work.

## Actors and permissions

Actors depend on the historical model of this contract. Read it as compatibility, migration, or historical reference.

## Key messages

- Historical surface preserved for reference.
- May still matter for older deployments or compatibility paths.
- Not the recommended default for new integrations.

## Relationships

- Lives under `docs/en/contracts/legacy` because it should be read comparatively and historically.
- Should be contrasted against the current equivalent surface.
- Prefer `pg721` or `pg721-updatable` for current collection work.

## Hypothetical example

Hypothetical flow: an integrator maintains an older deployment and needs to understand how `pg721-legacy` behaved before moving to the current recommended path.

## References

- Code: `contracts/nft/pg721-legacy/src/msg.rs`
- Local context: `contracts/nft/pg721-legacy/README.md`
- Current related contract: check the current docs under `docs/en/contracts/nft/`.
