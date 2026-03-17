# Phase 1: Protocol Hardening & Boundaries - Research

**Researched:** 2026-03-17
**Domain:** Brownfield CosmWasm hardening, contract/service boundaries, and protocol safety normalization
**Confidence:** HIGH

## Summary

Phase 1 should not try to "modernize everything." The codebase already has usable Passage primitives for collections, factories, marketplace flows, revenue routing, and some pause/migrate behavior. The right move is to harden the brownfield edges that can invalidate later PASG work: `streaming-billing` trust boundaries, public query correctness in `marketplace-v3` and `registry`, and inconsistent reply/migration safety around factory-style contract creation.

The product architecture documents are consistent with the repository's best existing patterns: Passage services orchestrate billing, fiat conversion, and premium UX off-chain; contracts provide ownership, accounting, routing, and governance primitives. The current gap is not lack of ambition, but lack of enforced protocol boundaries. `streaming-billing` currently models backend-only semantics in comments and README language, but does not encode a backend/operator role strongly enough in contract state and execute paths.

Planning should therefore bias toward three outcomes: first, close or contain the highest-risk trust and correctness defects; second, make the on-chain/off-chain boundary explicit in code and docs; third, normalize pause/migrate/cutover expectations across the in-scope contract families so later PASG utility work lands on stable protocol ground.

**Primary recommendation:** Plan Phase 1 as three focused plans: billing/trust-boundary hardening, brownfield query/reply correctness fixes with targeted tests, and a codified boundary plus migration/safety contract for the active protocol surface.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| ARCH-01 | Protocol contracts must enforce the intended on-chain/off-chain responsibility split rather than relying on comments or backend discipline. | Local code and product docs show `streaming-billing` is the highest-risk violation; research points to explicit operator authorization, registry-backed ownership checks, and removal of caller-supplied authoritative metadata. |
| ARCH-02 | High-risk brownfield correctness gaps must be closed before adding new PASG utility surfaces. | `marketplace-v3` and `registry` expose pagination/filter APIs they do not honor; factory reply parsing is inconsistent and brittle; these are the main correctness hazards to fix early. |
| ARCH-03 | Every active contract family in scope must have a documented pause, migrate, or cutover strategy. | Pause flags and migrate entry points already exist in many contracts, but patterns are inconsistent and undocumented as a protocol-wide rule. |
| REV-03 | Revenue and billing primitives must remain service-invoked protocol components, not autonomous business logic engines. | Product architecture explicitly defines billing and fiat as off-chain or hybrid; research reinforces keeping split/billing contracts as invoked primitives with explicit trust boundaries. |
</phase_requirements>

## Standard Stack

### Core
| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| `cosmwasm-std` | `2.1.3` | Core contract runtime APIs | Pinned workspace baseline across the repo; Phase 1 fixes should preserve this compatibility envelope. |
| `cw-storage-plus` | `2.0` | State maps, indexes, pagination primitives | Existing contracts already use it for maps/indexes; Phase 1 query fixes should extend current storage patterns, not replace them. |
| `cw-multi-test` | `2.1.1` | Contract-level integration tests | Already the repo's test harness for contract behavior; Phase 1 should add missing behavior coverage here instead of inventing a custom harness. |
| `cw2` | `2.0` | Version tracking and migration metadata | Existing contracts already use migrate/version flows; Phase 1 documentation should standardize around this. |
| `cargo unit-test` | alias -> `cargo test --lib` | Fast Rust test loop | Defined in `.cargo/config.toml`; suitable for quick validation once missing test modules are added. |

### Supporting
| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `cw-utils` | `2.0` | Reply and utility helpers | Keep using existing helper patterns where reply handling or validation logic needs small utilities. |
| `sylvia` | `1.2.1` | Contract framework used in parts of the repo | Respect where already present; do not force a cross-repo rewrite during Phase 1. |
| `cargo check --workspace` | current toolchain | Workspace-wide compile safety | Use as a broad regression gate after changing common patterns or shared types. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending existing `cw-multi-test` suites | Ad hoc hand-rolled mocks only | Faster initially, but weaker for state-machine and authorization regression coverage. |
| Fixing declared query APIs | Deprecating or removing parameters | Simpler code change, but breaks the current public contract surface and pushes ambiguity onto clients. |
| Normalizing existing pause/migrate patterns | Introducing a new upgrade framework now | Too much scope for Phase 1; the repo already has enough local primitives to standardize first. |

## Architecture Patterns

### Pattern 1: Service-invoked protocol primitives
**What:** Contracts provide settlement, routing, asset ownership, and verifiable accounting primitives; Passage services orchestrate streaming, fiat entry, discovery, and premium experience logic.
**When to use:** Any feature touching billing, subscription-like behavior, or creator monetization workflows.
**Use in Phase 1:** Harden contracts so they enforce only protocol truths and trust assumptions they can actually verify.

### Pattern 2: Explicit operator/admin authorization for hybrid flows
**What:** If a contract path represents a service-mediated action, model the operator/backend role in config/state and enforce it in execute handlers.
**When to use:** Session lifecycle events, fiat purchase reporting, or any flow where the chain cannot directly observe the real-world event.
**Use in Phase 1:** `streaming-billing` should not accept arbitrary callers or arbitrary authoritative metadata for backend-owned actions.

### Pattern 3: Honest public query surface
**What:** Query parameters exposed in `QueryMsg` must be implemented exactly or removed before release.
**When to use:** Pagination, filtering, counts, or collection-scoped query surfaces.
**Use in Phase 1:** `marketplace-v3` and `registry` should either honor `start_after` / `collection` semantics or stop advertising them.

### Pattern 4: Isolated reply parsing with dedicated tests
**What:** Keep instantiate/reply parsing logic in small helpers or reply modules, and cover it with reply-focused tests before changing surrounding workflows.
**When to use:** Factory contracts, collection deployment flows, minter instantiate callbacks.
**Use in Phase 1:** Normalize the fragile reply paths before broader protocol upgrades make them harder to reason about.

### Pattern 5: Capability-matrix safety over one-off patching
**What:** Document for each active contract family whether it supports pause, migrate, cutover, and critical authorization roles.
**When to use:** Brownfield safety phases preceding major new protocol work.
**Use in Phase 1:** Convert today's scattered behavior into a protocol-wide safety baseline for later PASG phases.

### Anti-Patterns to Avoid
- **Backend semantics only in comments:** If README or `msg.rs` says "backend-only" but execute paths accept public callers, the contract boundary is wrong.
- **Caller-supplied authoritative state:** Do not persist world ownership, world collection, or billing authority from unverified caller input when the registry or config is the real source of truth.
- **Silent query mismatch:** Leaving placeholder pagination/filter parameters in public APIs creates invisible client bugs and weakens future governance/reporting features.
- **Safety by tribal knowledge:** Do not assume teams remember which contracts can be paused or migrated; write the matrix and tie it to code paths.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Platform billing logic | Autonomous on-chain subscription engine | Service-invoked billing primitives plus explicit operator gating | Product docs define billing and fiat as off-chain or hybrid; putting subscription logic on-chain violates the architecture. |
| Authorization semantics | Per-handler ad hoc sender checks | Reusable admin/operator role model in config and consistent guard patterns | Brownfield auth drift is one of the root causes of the current `streaming-billing` risk. |
| Query pagination fixes | One-off in-memory filtering after full scans | Existing `cw-storage-plus` prefix/range pagination patterns or explicit secondary indexes | Honest query semantics and gas safety require storage-aware fixes, not post-processing hacks. |
| Upgrade strategy | New proxy framework in Phase 1 | Standardize existing `migrate`, `cw2`, and pause/cutover rules first | The repo already has enough migration machinery; Phase 1 should normalize before expanding. |

**Key insight:** The main risk here is not missing features. It is blurring what the chain can verify versus what the platform knows off-chain. Phase 1 should tighten that line everywhere it is currently fuzzy.

## Common Pitfalls

### Pitfall 1: Treating hybrid flows as if they were fully on-chain
**What goes wrong:** Session start/stop, fiat reporting, or world-rate updates get modeled as generic public executes.
**Why it happens:** The product idea is real, but the contract cannot directly observe the off-chain event, so developers leave trust assumptions in comments.
**How to avoid:** Add explicit operator roles, verify authoritative references (for example registry ownership), and reject caller-supplied truths when config/state already defines them.
**Warning signs:** `info.sender` unused, TODO comments around ownership checks, README promises not backed by code.

### Pitfall 2: Fixing symptoms without adding tests first
**What goes wrong:** Brownfield defects are patched, but later refactors reintroduce them because the critical path has no coverage.
**Why it happens:** The repo already has partial tests, so it is easy to assume adjacent behavior is covered.
**How to avoid:** For `streaming-billing`, `marketplace-v3` query semantics, and factory replies, create or extend focused `cw-multi-test` coverage before larger rewrites.
**Warning signs:** `lib.rs` has no test module, only helper tests exist, or public APIs lack direct query/execute coverage.

### Pitfall 3: Preserving misleading APIs for convenience
**What goes wrong:** `QueryMsg` keeps filters and pagination parameters that are ignored internally, so clients think they are safe when they are not.
**Why it happens:** The interface was designed ahead of implementation and never reconciled.
**How to avoid:** Make Phase 1 explicitly reconcile `msg.rs` contracts with implementation behavior.
**Warning signs:** `_start_after` parameters, `_collection` parameters, counts that ignore scope filters.

### Pitfall 4: Normalizing safety only in docs
**What goes wrong:** A matrix says a contract is pausable or migratable, but entry points do not enforce that consistently.
**Why it happens:** Brownfield documentation gets written after the fact and drifts from code.
**How to avoid:** Tie the safety inventory to concrete contract files and verification commands in the plans.
**Warning signs:** README claims capabilities missing from `execute`, `migrate`, or config types.

## Code Examples

Verified local patterns to reuse during planning:

### Pause guard pattern already used in core contracts
Source: `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`

```rust
if config.paused && !is_admin_or_operator(&config, &info.sender) {
    return Err(ContractError::Paused {});
}
```

Phase 1 should prefer extending this kind of explicit guard pattern instead of embedding hidden trust assumptions in individual handlers.

### Reply parsing should stay isolated
Source: `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/helpers.rs`

```rust
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let collection_address = extract_contract_address_from_reply(&msg)?;
    // persist + emit downstream messages
}
```

Phase 1 should keep reply extraction isolated and add direct tests around it before changing reply strategy.

## State of the Art

| Old / Risky State | Desired Current State | Impact on Phase 1 |
|-------------------|-----------------------|-------------------|
| README/comments define trust boundary | Config/state + execute guards define trust boundary | Makes billing and revenue primitives trustworthy enough for later PASG utility work. |
| Public queries expose placeholder parameters | Query behavior matches `QueryMsg` exactly | Prevents client-side correctness bugs and future governance/reporting confusion. |
| Safety capabilities are scattered by contract | Phase-wide capability matrix documents pause/migrate/cutover rules | Reduces upgrade risk across the brownfield surface. |
| Fragile reply flows depend on inconsistent parsing assumptions | Reply paths are standardized and covered by dedicated tests | Makes future contract deployment/migration work materially safer. |

**Deprecated/outdated for this repo's architecture:**
- Treating PASG or billing contracts as the place to encode platform subscription logic.
- Relying on backend discipline alone for session authorization.
- Leaving query args in public APIs when implementation ignores them.

## Open Questions

1. **Should `streaming-billing` allow any user-driven session lifecycle, or be strictly operator-mediated?**
   - What we know: Product docs frame streaming as hybrid and the current code assumes backend control in comments.
   - What's unclear: Whether users should retain any direct self-service stop/claim path.
   - Recommendation: Plan around operator-authoritative lifecycle paths first; if user self-service is needed later, add a separate, explicitly bounded path.

2. **How much of the Phase 1 safety matrix should become runtime-enforced versus documented-only?**
   - What we know: Many contracts already expose pause or migrate capabilities.
   - What's unclear: Whether every contract family needs additional code changes now, or whether some only need protocol documentation and verification.
   - Recommendation: Enforce where behavior is currently unsafe or ambiguous; document-and-verify where capability already exists.

3. **Should reply parsing be standardized on reply data everywhere in this phase?**
   - What we know: Current flows mix event scraping and reply-data parsing.
   - What's unclear: Whether full standardization is safe within the scope of Phase 1.
   - Recommendation: Plan to isolate and test reply parsing first; adopt a single mechanism where low-risk, otherwise document the cutover path and keep the change bounded.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `cw-multi-test 2.1.1` |
| Config file | `.cargo/config.toml` |
| Quick run command | `cargo test -p streaming-billing --lib` |
| Full suite command | `cargo unit-test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ARCH-01 | Backend/session/world-ownership actions in `streaming-billing` enforce explicit authority and authoritative metadata sources | integration | `cargo test -p streaming-billing --lib` | ❌ Wave 0 |
| ARCH-02 | `marketplace-v3` and `registry` queries honor declared filtering and pagination semantics; factory reply flows stay correct under test | integration | `cargo test -p marketplace-v3 --lib` / `cargo test -p registry --lib --tests` / `cargo test -p collection-factory --lib` / `cargo test -p ecosystem-factory --lib` | ❌ Wave 0 for marketplace/factories, ✅ partial for registry |
| ARCH-03 | Active contract families have verified pause/migrate/cutover documentation and compile-safe touch points | smoke | `cargo check --workspace` | ✅ |
| REV-03 | Billing/revenue contracts remain service-invoked primitives and reject platform-business overreach in the touched paths | integration | `cargo test -p streaming-billing --lib` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** Run the narrowest affected package test command, starting with `cargo test -p streaming-billing --lib` for billing work and the package-specific `cargo test -p ... --lib` for query/reply fixes.
- **Per wave merge:** Run `cargo unit-test`.
- **Phase gate:** Run `cargo unit-test` and `cargo check --workspace` before `$gsd-verify-work`.

### Wave 0 Gaps
- [ ] `contracts/core/streaming-billing/src/tests.rs` or equivalent module wiring for auth/accounting/session coverage
- [ ] `contracts/nft/marketplace-v3/src/contract/query/tests.rs` for public query semantics
- [ ] `contracts/core/collection-factory/src/contract/reply/tests.rs` for instantiate reply parsing
- [ ] `contracts/core/ecosystem-factory/src/contract/reply/tests.rs` for instantiate reply parsing
- [ ] Focused registry tests for authorized-minter pagination semantics if existing coverage cannot absorb the behavior cleanly

## Sources

### Primary (HIGH confidence)
- `.planning/ROADMAP.md` - Phase 1 goal, requirement IDs, and canonical references
- `.planning/REQUIREMENTS.md` - Formal requirement mapping for ARCH-01, ARCH-02, ARCH-03, REV-03
- `.planning/codebase/CONCERNS.md` - Current brownfield defect inventory and safe-modification guidance
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md` - Product-level protocol/service boundary
- `../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md` - Hybrid billing and fiat-entry assumptions
- `contracts/core/streaming-billing/src/contract.rs` - Actual authorization, accounting, and query behavior
- `contracts/nft/marketplace-v3/src/contract/query.rs` - Actual public query semantics
- `CLAUDE.md` - Repo-specific build, testing, and architecture guidance
- `.cargo/config.toml` - Local cargo aliases used by the repo

### Secondary (MEDIUM confidence)
- `contracts/core/collection-factory/src/contract/execute.rs` and `reply.rs` - Existing pause/operator and reply isolation patterns
- `contracts/core/ecosystem-factory/src/contract/execute.rs` and `reply.rs` - Existing pause/operator and reply handling patterns
- `contracts/core/registry/src/contract/execute.rs` and `query.rs` - Registry-side admin/pagination patterns and current gaps

### Tertiary (LOW confidence)
- None - this research is based on local product docs and current repository state rather than external ecosystem research.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions and tools are pinned locally in workspace manifests and cargo config
- Architecture: HIGH - product architecture docs and repo patterns are aligned on the key boundary decisions
- Pitfalls: HIGH - directly evidenced by `.planning/codebase/CONCERNS.md` and the touched contract sources

**Research date:** 2026-03-17
**Valid until:** 2026-04-16