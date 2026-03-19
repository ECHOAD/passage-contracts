# Phase 8 Research: Complete Bilingual Contract Documentation in docs

**Phase:** 8
**Date:** 2026-03-19
**Status:** Complete

## Research Objective

Determine how to plan a bilingual documentation phase that covers every contract in the Passage workspace, explains inter-contract relationships, and provides hypothetical business and technical examples without changing protocol scope.

## Workspace Inventory

Current workspace members under `contracts/`:

- `core` (7): `collection-factory`, `ecosystem-factory`, `multisig`, `pasg-governance`, `registry`, `split-router`, `streaming-billing`
- `nft` (14): `auction-english`, `marketplace-legacy`, `marketplace-v2`, `marketplace-v3`, `minter`, `minter-metadata-onchain`, `minter-v2`, `minter-v2-metadata-onchain`, `pg721`, `pg721-legacy`, `pg721-metadata-onchain`, `pg721-updatable`, `royalty-group`, `whitelist`
- `relationship` (2): `follow`, `friend`
- `staking` (3): `nft-vault`, `stake-rewards`, `vault-factory`

Total documented contract crates required by user decision: **26**.

## Existing Documentation Sources

Primary reusable material already in repo:

- Top-level docs: `docs/README.md`, `docs/01-end-to-end-setup.md`, `docs/02-method-reference.md`, `docs/03-json-examples.md`, `docs/04-multisig-governance.md`
- Contract-local READMEs exist for most active crates and several legacy/compatibility crates
- `msg.rs`, `contract.rs`, `lib.rs`, and checked-in `schema/` directories provide the authoritative instantiate/execute/query surface
- `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`, and `.planning/codebase/INTEGRATIONS.md` already describe architectural boundaries and cross-contract relationships

## Constraints from User Context

Locked by `08-CONTEXT.md`:

- documentation must cover all contracts, not just active/current ones
- output must live in mirrored `docs/es/` and `docs/en/` trees
- the granularity must be **one file per contract**
- the docs must explain relationships between contracts, not just isolated APIs
- hypothetical examples must include both business flows and technical usage patterns
- legacy contracts must be treated as historical/reference material rather than the recommended path for new integrations

## Planning Implications

### 1. This phase is large enough to require multiple waves
The documentation count is too high for a single-plan closeout if quality and cross-linking are to remain coherent. The plan should separate:
- bilingual scaffold and shared relationship docs
- core contract pages
- NFT contract pages
- smaller families plus legacy/reference closeout

### 2. The docs need mirrored structure, not ad hoc translation
A stable mirrored tree should be planned up front so execution does not create mismatched Spanish/English coverage. The simplest durable structure is:
- `docs/es/...`
- `docs/en/...`
with matching subfolders per contract family and matching page names.

### 3. One-file-per-contract still benefits from family indexes
The user wants per-contract files, but the volume means family-level landing pages and a top-level index will still be useful. These should complement, not replace, contract pages.

### 4. Relationship docs should be separate first-class artifacts
Cross-contract explanation should not be buried inside contract pages alone. The phase should also produce shared docs for:
- contract relationship map
- instantiate guidance
- hypothetical end-to-end flows

### 5. Legacy classification must stay conservative
Only explicitly legacy/superseded surfaces should be labeled as historical/reference by default. Named legacy crates are obvious (`marketplace-legacy`, `pg721-legacy`); additional historical or compatibility framing should be justified from local README/context, not guessed recklessly.

## Recommended Documentation Structure

Recommended mirrored structure for planning:

- `docs/es/README.md`
- `docs/en/README.md`
- `docs/es/overview.md` / `docs/en/overview.md`
- `docs/es/relationships.md` / `docs/en/relationships.md`
- `docs/es/instantiate-guides.md` / `docs/en/instantiate-guides.md`
- `docs/es/flows.md` / `docs/en/flows.md`
- `docs/es/contracts/core/*.md` / `docs/en/contracts/core/*.md`
- `docs/es/contracts/nft/*.md` / `docs/en/contracts/nft/*.md`
- `docs/es/contracts/relationship/*.md` / `docs/en/contracts/relationship/*.md`
- `docs/es/contracts/staking/*.md` / `docs/en/contracts/staking/*.md`
- `docs/es/contracts/legacy/*.md` / `docs/en/contracts/legacy/*.md`

## Verification Architecture

This is a documentation-heavy phase, so validation should focus on deterministic file coverage and wording checks rather than cargo execution.

Recommended verification floor:

- Every contract crate has a corresponding page in both `docs/es` and `docs/en`
- Shared docs exist in both languages: overview, relationships, instantiate guides, flows
- Contract pages contain the minimum required sections: purpose, instantiation, actors/permissions, key messages, dependencies/relationships
- Legacy/reference docs explicitly say they are historical/reference rather than default integration guidance
- Top-level `docs/README.md` links to both language trees

## Risks to Watch During Planning

- Drift between English and Spanish trees if naming and structure are not fixed in plan text
- Superficial contract pages that only restate crate names without relationships or instantiate guidance
- Overwriting or fragmenting current `docs/` files without a clear migration strategy
- Treating every older crate as legacy without evidence from naming or existing docs

## Planning Recommendation

Plan Phase 8 in **4 plans / 4 waves**:

1. Bilingual scaffold, top-level indexes, relationship map, instantiate guides, and flow narrative shells
2. Core contract pages in both languages
3. NFT contract pages in both languages
4. Relationship/staking/legacy pages, final cross-links, and tracker closeout

This split keeps execution bounded while still satisfying the user's requirement for complete and mirrored documentation coverage.

---

*Research complete for Phase 8.*
