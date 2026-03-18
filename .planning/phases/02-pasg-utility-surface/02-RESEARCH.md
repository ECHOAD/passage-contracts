# Phase 2: PASG Utility Surface - Research

**Researched:** 2026-03-18  
**Domain:** PASG utility semantics, native-denom settlement, and shared on-chain fee treatment  
**Confidence:** HIGH

## Summary

Local repo evidence shows PASG is already treated as a native settlement denom (`upasg`) across deployment instructions, contract configs, and tests, not as a dedicated token contract inside this repository. That is an inference from repeated native-coin handling in `README.md`, `CLAUDE.md`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/nft/marketplace-v3/README.md`, `contracts/nft/minter-v2/README.md`, and `contracts/nft/auction-english/src/execute.rs`, combined with the absence of any PASG token-contract crate in the workspace.

The current utility surface is split across two primitives:

- `streaming-billing` handles PASG-aware hybrid billing, points conversion, session settlement, and pending PASG revenue.
- `split-router` handles generic split execution and previewing, but it is denom-agnostic and does not define PASG policy on its own.

Phase 2 should not invent a new economics engine. The right shape is a canonical PASG utility interface that standardizes how protocol-facing contracts detect PASG, apply fee treatment, and route PASG-denominated funds. Based on the repo, the safest interpretation is native-denom-first, with an adapter/wrapper only if future chain or compatibility constraints require it. If a wrapper exists, it should be a compatibility shim, not the source of truth.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PASG-01 | User can pay supported protocol-facing fees with PASG and receive on-chain-verifiable fee treatment where configured. | `contracts/core/streaming-billing/src/contract.rs` already accepts direct PASG deposits, converts them into points, and emits PASG-denominated pending revenue; `contracts/core/split-router/src/contract/execute.rs` and `src/contract/helpers.rs` already route and preview native funds. What is missing is a shared PASG fee-treatment contract surface rather than contract-by-contract handling. |
| PASG-02 | Protocol contracts expose a canonical PASG utility interface instead of duplicating fee or discount logic contract by contract. | `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/split-router/src/msg.rs`, `contracts/nft/marketplace-v3/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/nft/auction-english/src/execute.rs` all carry their own denom or payment assumptions. There is no repo-wide PASG interface yet, so utility semantics will drift unless they are centralized. |
| PASG-03 | PASG utility works with the existing `upasg` settlement model or a documented wrapper/adapter path, with no ambiguity for integrators. | `README.md`, `CLAUDE.md`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/nft/marketplace-v3/README.md`, and `contracts/nft/minter-v2/README.md` all point to `upasg` as the operational denom. No PASG token contract exists in this workspace, so any adapter must preserve native settlement semantics. |
</phase_requirements>

## Standard Stack

### Core
| Library / Tool | Version | Purpose | Why It Matters Here |
|----------------|---------|---------|----------------------|
| `cosmwasm-std` | `2.1.3` | Native contract runtime APIs | All PASG utility logic here is expressed through bank coins, wasm queries, and native denom checks. |
| `cw-storage-plus` | `2.0` | Contract state maps and pagination helpers | Existing utility surfaces already use it for config, revenue, sessions, and split events; Phase 2 should extend these patterns rather than introduce a new persistence model. |
| `cw-multi-test` | `2.1.1` | Contract integration tests | Phase 2 must prove utility behavior in-contract, especially around denomination handling and routing. |
| `cw2` | `2.0` | Version and migration metadata | Relevant because the repo already relies on deploy-and-cutover behavior rather than a universal upgrade framework. |
| Rust | `1.85.x` | Workspace language baseline | No new runtime stack is needed for PASG utility work. |

### Supporting
| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `sylvia` | `1.2.1` | Used by staking crates elsewhere in the repo | Not a Phase 2 dependency, but useful context for how later PASG staking work will differ from the current core-contract surface. |
| `cargo check --workspace` | current toolchain | Compile regression gate | Use after cross-contract message or state changes that affect PASG utility semantics. |
| `cargo unit-test` | alias | Fast repo-wide test gate | Use as the phase-level verification floor once local tests are added. |

## Architecture Patterns

### Pattern 1: Native-denom settlement primitive
**What:** PASG already behaves like a native coin (`upasg`) in the repo, not like an ERC-20/CW20-style token contract.
**Evidence:** `README.md`, `CLAUDE.md`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/split-router/src/contract/execute/tests.rs`, `contracts/nft/marketplace-v3/README.md`, `contracts/nft/minter-v2/README.md`, `contracts/nft/auction-english/src/execute.rs`.
**Implication:** The canonical interface should speak native coin semantics first. Any wrapper should map to native settlement, not redefine it.

### Pattern 2: Shared utility surface over per-contract denom logic
**What:** The repo already has several denom-aware contracts, but they each make local decisions.
**Evidence:** `contracts/core/streaming-billing/src/msg.rs` stores `denom` and `points_per_denom`; `contracts/nft/marketplace-v3/src/msg.rs` and `contracts/nft/auction-english/src/msg.rs` each define their own payment denom; `contracts/core/split-router/src/msg.rs` only models a generic splitter.
**Implication:** Phase 2 should centralize utility policy in one canonical interface, then let consumers call it.

### Pattern 3: Service-invoked hybrid billing
**What:** `streaming-billing` is already documented and implemented as a service-invoked primitive.
**Evidence:** `contracts/core/streaming-billing/README.md` and `contracts/core/streaming-billing/src/contract.rs` show backend operator, fiat oracle, and admin-gated lifecycle paths.
**Implication:** PASG utility must preserve the boundary between verifiable on-chain accounting and off-chain service orchestration.

### Pattern 4: Registry-verified world configuration
**What:** Billing configuration for worlds is validated against registry and cw721 ownership, not caller assertions.
**Evidence:** `contracts/core/streaming-billing/src/contract.rs` uses `RegistryQueryMsg::Collection` and `OwnerOf` before storing world config.
**Implication:** If PASG utility is extended into world-facing billing, keep registry-backed verification as the trust anchor.

### Anti-Patterns to Avoid
- Duplicating PASG fee logic in each consumer contract. That already exists as local denom handling in `streaming-billing`, `marketplace-v3`, `minter-v2`, and `auction-english`.
- Treating `split-router` as a PASG policy engine. It currently splits whatever bank funds it receives and does not enforce PASG-specific semantics.
- Assuming `stripe_webhook_validator` implies on-chain webhook verification. The config field exists, but the contract and docs show Stripe signature validation stays off-chain today.
- Introducing a wrapper that changes settlement units without a clear migration story. The repo currently operates on native `upasg` semantics.
- Encoding fee treatment only in README prose or comments. If it matters, it needs a message/query surface and tests.

## Likely Contract Insertion Points

### Primary
- `contracts/core/streaming-billing/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- `contracts/core/streaming-billing/src/state.rs`
- `contracts/core/streaming-billing/README.md`

Why here: this is the only contract in the repo that already combines PASG deposits, points conversion, fiat reporting, session settlement, and PASG-denominated pending revenue. If a canonical PASG utility interface is going to exist anywhere first, this is the highest-leverage place.

### Secondary
- `contracts/core/split-router/src/msg.rs`
- `contracts/core/split-router/src/contract.rs`
- `contracts/core/split-router/src/contract/helpers.rs`
- `contracts/core/split-router/README.md`

Why here: this is the reusable money-routing primitive. It should remain generic, but it is the natural place to expose shared preview/routing behavior if PASG-specific treatment must be visible to integrators.

### Consumer Touchpoints
- `contracts/nft/marketplace-v3/src/msg.rs`
- `contracts/nft/marketplace-v3/src/contract/execute.rs`
- `contracts/nft/marketplace-v3/src/contract/query.rs`
- `contracts/nft/minter-v2/src/msg.rs`
- `contracts/nft/minter-v2/src/contract/execute.rs`
- `contracts/nft/auction-english/src/execute.rs`

Why here: these contracts currently hardcode or resolve their own payment denoms. They are the likely follow-on integration points once the canonical PASG utility semantics are defined.

### Documentation and Operator Surface
- `README.md`
- `CLAUDE.md`
- `.planning/codebase/STACK.md`
- `.planning/codebase/INTEGRATIONS.md`

Why here: every integrator needs the same answer on whether PASG is native-denom-first or adapter-first. The current repo documentation still implies `upasg` without a single canonical utility contract description.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `cw-multi-test 2.1.1` |
| Quick run command | `cargo test -p streaming-billing --lib` |
| Secondary quick run | `cargo test -p split-router --lib` |
| Full suite command | `cargo unit-test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|--------------|
| PASG-01 | PASG deposits, withdrawals, and fee treatment are enforced on-chain where configured | integration | `cargo test -p streaming-billing --lib` and `cargo test -p split-router --lib` | Partial for billing, partial for split-router |
| PASG-02 | PASG utility is exposed through one canonical interface instead of duplicated local logic | integration | `cargo test -p streaming-billing --lib`, `cargo test -p split-router --lib`, then consumer-package tests after integration | No canonical interface yet |
| PASG-03 | Native `upasg` settlement is unambiguous for integrators, or adapter behavior is documented and tested | smoke + integration | `cargo check --workspace` and package-level tests for affected contracts | Partial through docs and existing denom use |

### Sampling Rate
- **Per task commit:** run the narrowest affected package test command first, usually `cargo test -p streaming-billing --lib` or `cargo test -p split-router --lib`.
- **Per integration wave:** run `cargo unit-test`.
- **Phase gate:** run `cargo unit-test` and `cargo check --workspace` before verification.

### Wave 0 Gaps
- [ ] `contracts/core/streaming-billing/src/tests.rs` does not cover `DepositCrypto`, `WithdrawPoints`, `query_conversion_rate`, or the `DistributeWorldRevenue` / `BatchDistributeRevenue` message construction path.
- [ ] `contracts/core/streaming-billing/src/tests.rs` does not prove the PASG denom path end to end when `denom` is changed from `upasg`.
- [ ] `contracts/core/split-router/src/contract/execute/tests.rs` does not cover zero-funds rejection, event-cap pruning at `MAX_EVENTS_STORED`, or a failure path for malformed recipient state.
- [ ] `contracts/core/split-router/src/contract/query/tests.rs` only covers preview math, not event pagination or config changes.
- [ ] No tests exist for a shared PASG utility interface because that interface does not exist yet.

## Open Questions

1. **Should the canonical PASG utility interface live in `streaming-billing`, `split-router`, or a small shared helper/message crate?**
   - What we know: `streaming-billing` is the only contract already combining PASG accounting with service-invoked billing, while `split-router` is the reusable routing primitive.
   - What is unclear: whether the canonical interface should be a contract, a message module, or just a standardized helper layer.
   - Recommendation: design it so `streaming-billing` owns utility semantics and `split-router` stays generic unless Phase 2 proves a shared contract boundary is necessary.

2. **Do we want an explicit adapter/wrapper at all, or should Phase 2 standardize on native `upasg` and document that as the only supported settlement path?**
   - What we know: the repo already behaves like native-denom-first.
   - What is unclear: whether future chain portability or backwards compatibility needs justify a wrapper.
   - Recommendation: treat adapter/wrapper support as optional until a real compatibility gap is proven.

3. **Which consumer contracts must integrate in Phase 2 versus later phases?**
   - What we know: `marketplace-v3`, `minter-v2`, and `auction-english` all have local denom semantics today.
   - What is unclear: whether Phase 2 should only define the interface, or also wire the first consumers.
   - Recommendation: define the interface now, then bind the first high-value consumers only where the test surface is strong enough to verify the behavior.

## Sources

### Primary (HIGH confidence)
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/codebase/STACK.md`
- `.planning/codebase/INTEGRATIONS.md`
- `.planning/codebase/CONCERNS.md`
- `README.md`
- `CLAUDE.md`
- `contracts/core/streaming-billing/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- `contracts/core/streaming-billing/src/tests.rs`
- `contracts/core/streaming-billing/README.md`
- `contracts/core/split-router/src/msg.rs`
- `contracts/core/split-router/src/contract.rs`
- `contracts/core/split-router/src/contract/execute.rs`
- `contracts/core/split-router/src/contract/execute/tests.rs`
- `contracts/core/split-router/src/contract/query.rs`
- `contracts/core/split-router/src/contract/query/tests.rs`
- `contracts/core/split-router/src/contract/helpers.rs`
- `contracts/core/split-router/README.md`
- `contracts/nft/marketplace-v3/src/msg.rs`
- `contracts/nft/marketplace-v3/src/contract/execute.rs`
- `contracts/nft/marketplace-v3/README.md`
- `contracts/nft/minter-v2/src/msg.rs`
- `contracts/nft/minter-v2/README.md`
- `contracts/nft/auction-english/src/execute.rs`
- `D:\Projects\Nodefleet\Passage\context\product\architecture\FIAT_TO_CRYPTO_PAYMENT_FLOW.md`

### Secondary (MEDIUM confidence)
- `contracts/nft/marketplace-v3/src/contract/query.rs`
- `contracts/nft/marketplace-v3/src/contract/helpers.rs`
- `contracts/nft/minter-v2/src/contract/execute.rs`

### Tertiary (LOW confidence)
- None. This research is intentionally local-only and based on current repo/product artifacts.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions and tools are pinned locally in workspace manifests and planning docs.
- Architecture: HIGH - the repo already exhibits a native-denom-first PASG model with a clear service-invoked billing boundary.
- Pitfalls: HIGH - local docs and tests show concrete drift risks around denom duplication, generic split routing, and missing canonical PASG semantics.

**Research date:** 2026-03-18  
**Valid until:** 2026-04-17
