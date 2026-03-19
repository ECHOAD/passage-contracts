# Phase 8: Documentacion completa bilingue de contratos en docs - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** User discussion during `$gsd-discuss-phase 8`

<domain>
## Phase Boundary

Create complete bilingual contract documentation under `docs/` for the full repository surface, covering what each contract does, how it is instantiated, how it relates to other contracts, and how the main protocol flows work.

This phase is documentation-only:
- it does not add new protocol capabilities
- it does not redefine already-set contract boundaries
- it translates current implementation and architecture into clear English and Spanish docs
- it must explain both per-contract behavior and cross-contract relationships with hypothetical examples

</domain>

<decisions>
## Implementation Decisions

### Full repository coverage
- The documentation must cover all contract families in the repo.
- Coverage includes `core`, `nft`, `relationship`, and `staking` contracts.
- Legacy contracts must also be documented, but treated as historical/reference material rather than the recommended path for new integrations.

### Bilingual directory structure
- Documentation must live in two language-specific directories under `docs/`.
- One directory must be for Spanish and one for English.
- The two trees should mirror each other structurally so that a reader can move between languages easily.

### Per-contract granularity
- Documentation should be organized as one file per contract.
- Each contract file should explain at least:
  - purpose
  - instantiate surface
  - important execute/query messages
  - actors and permissions
  - dependencies on other contracts
  - where it sits in the overall protocol
- This phase should avoid only family-level summaries; the user explicitly wants contract-by-contract documentation.

### Relationship and flow documentation
- The docs must explain how contracts relate to each other, not just describe them in isolation.
- The docs must include explicit relationship views across the protocol, including administrative, registration, minting, trading, routing, and governance flows.
- The docs must include hypothetical examples and end-to-end narratives so a reader can understand how the system behaves in practice.

### Hypothetical examples
- Examples must include both business-style flows and technical integration flows.
- Business-style examples should describe scenarios such as ecosystem creation, collection registration, minting, sales, royalties, governance, and PASG utility usage.
- Technical examples should show instantiate expectations and representative execute/query usage where helpful.
- Examples are explanatory artifacts, not a commitment to add new contract behavior.

### Legacy treatment
- Legacy contracts should be documented as reference or historical compatibility surfaces.
- Legacy docs should make clear that they are not the preferred target for new work unless explicitly required.
- The main documentation path should prioritize current contracts first, then place legacy material in a clearly labeled reference section.

### Claude's Discretion
- Exact file naming and numbering within `docs/es/` and `docs/en/`, as long as both trees stay structurally aligned.
- Whether overview/index pages are added per family in addition to the required one-file-per-contract pages.
- How much instantiate/execute/query detail belongs inline in each contract page versus linked from shared flow pages.
- Whether relationship diagrams are rendered as markdown tables, ASCII flow maps, or both.

</decisions>

<specifics>
## Specific Ideas

- The user wants documentation for all contracts, not only active or core ones.
- The user explicitly wants separate Spanish and English directories.
- The user explicitly wants the docs to explain how contracts relate to each other.
- The user explicitly wants hypothetical examples that explain how the protocol works in practice.
- The user explicitly wants one file per contract, not only grouped family summaries.
- Legacy contracts should still be present in the docs, but as historical reference rather than recommended usage.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Existing protocol docs
- `README.md` - top-level protocol framing and current contract inventory
- `docs/README.md` - current docs index and organization baseline
- `docs/01-end-to-end-setup.md` - current setup, instantiate, and environment narrative
- `docs/02-method-reference.md` - current method-level contract notes and public interfaces
- `docs/03-json-examples.md` - current example payloads and integration examples
- `docs/04-multisig-governance.md` - current governance/admin relationship explanation

### Planning and architecture context
- `.planning/ROADMAP.md` - Phase 8 definition and project ordering
- `.planning/REQUIREMENTS.md` - current requirement set and in-scope protocol surface
- `.planning/STATE.md` - current project state and already-fixed architectural decisions
- `.planning/codebase/ARCHITECTURE.md` - current repo architecture and boundaries
- `.planning/codebase/STRUCTURE.md` - current workspace and contract-family layout
- `.planning/codebase/INTEGRATIONS.md` - cross-contract and external integration notes

### Contract inventory roots
- `contracts/core/` - admin, registry, routing, PASG utility, and billing surfaces
- `contracts/nft/` - collections, minters, marketplaces, auctions, royalties, and legacy NFT surfaces
- `contracts/relationship/` - follow/friend relationship contracts
- `contracts/staking/` - NFT staking-related contracts and supporting reward modules

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- The repo already has partial protocol docs in `docs/`, but they are not yet complete, bilingual, or one-file-per-contract.
- Many active contracts already contain local `README.md` files that can seed contract-level pages.
- Schema directories and `msg.rs` files provide the best source of instantiate/execute/query truth for contract pages.
- Prior phases already clarified important architectural boundaries that the docs must preserve, especially around PASG, governance, staking, registry, and marketplace behavior.

### Established Patterns
- Current docs already mix conceptual narrative, method reference, and JSON examples; Phase 8 can reorganize this into a clearer bilingual contract library.
- The workspace is naturally grouped by contract family, which supports mirrored `docs/es/` and `docs/en/` trees with family indexes plus per-contract pages.
- Several contracts are clearly legacy or superseded, so the docs should separate current vs historical guidance explicitly.

### Integration Points
- Contract-level docs should derive instantiate and interface details from each crate's `msg.rs`, schema, and README where present.
- Relationship/flow docs should synthesize behavior across `registry`, `ecosystem-factory`, `collection-factory`, `pg721*`, `minter*`, `marketplace-v3`, `auction-english`, `split-router`, `streaming-billing`, `multisig`, and `pasg-governance`.
- The documentation structure should remain aligned with the actual contract families under `contracts/` so future maintenance stays straightforward.

</code_context>

<deferred>
## Deferred Ideas

- Whether future docs should include rendered diagrams or image assets beyond markdown-native explanations.
- Whether bilingual documentation should later gain automated translation checks or generation support.
- Whether public-facing docs outside the repo should later be generated from this same source tree.

</deferred>

---

*Phase: 08-documentacion-completa-bilingue-de-contratos-en-docs*
*Context gathered: 2026-03-19 via direct user decisions*
