# Phase 06 Research

**Phase:** 06 - Multi-Economy Settlement
**Date:** 2026-03-19
**Status:** Updated from clarified context

## Goal

Define how Passage worlds may expose local units or points-based economies while keeping PASG utility canonical, preserving collection-level creator monetization, and reusing existing refund-safe commerce primitives instead of inventing a generalized escrow layer by default.

## Inputs Reviewed

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/PROJECT.md`
- `.planning/phases/06-multi-economy-settlement/06-CONTEXT.md`
- `.planning/phases/05-creator-asset-contracts-monetization/05-CONTEXT.md`
- `.planning/phases/05-creator-asset-contracts-monetization/05-VERIFICATION.md`
- `contracts/core/streaming-billing/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- `contracts/core/streaming-billing/README.md`
- `contracts/core/split-router/src/msg.rs`
- `contracts/core/split-router/src/contract.rs`
- `contracts/core/split-router/README.md`
- `contracts/core/registry/src/msg.rs`
- `contracts/nft/marketplace-v3/src/msg.rs`
- `contracts/nft/marketplace-v3/src/contract/execute.rs`
- `contracts/nft/auction-english/src/msg.rs`
- `contracts/nft/auction-english/src/execute.rs`
- `contracts/nft/pg721/src/msg.rs`
- `contracts/nft/pg721-updatable/src/msg.rs`
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md`
- `../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md`

## Confirmed Current State

1. `streaming-billing` already owns the strongest PASG-native accounting surface in the repo: direct PASG deposits, fiat-assisted PASG accounting, points balances, world rate config, session charging, and world-revenue distribution.
2. `streaming-billing` still carries wording and semantics that can be read as "world usage revenue" even though the clarified product stance says creator revenue is not primarily usage-based.
3. `split-router` is intentionally generic and denom-preserving. It should stay that way rather than becoming a world-economy policy engine.
4. `marketplace-v3` and `auction-english` already contain the repo's most concrete refund-safe settlement patterns: displaced bid refunds, custody/closeout logic, and PASG settlement attributes.
5. Phase 5 locked creator monetization at the `collection` level. Royalties and creator revenue semantics should not move upward into `ecosystem`.
6. `registry` is now the canonical affiliation ledger for ecosystems and collections, but there is no reason yet to turn it into a local-economy registry by default.

## Clarified Product Direction

The user provided direct product guidance from Passage leadership:

- worlds do not generate creator revenue from usage by default
- creators earn on the initial sale plus future resales
- Passage earns marketplace fees
- excessive royalties damage marketplace liquidity and can push trading off-platform

That means Phase 6 cannot be planned as "monetize world usage on-chain." Instead, it must be planned as "make optional local economies compatible with PASG and existing commerce settlement rules."

## Main Gaps

### Gap 1: The roadmap wording still over-suggests usage billing as the center of the phase

The original success criteria talk about local tokens, hybrid paths, session-based settlement, escrow, and refunds in one block. In code, the only major existing local-economy primitive is `streaming-billing`, which already mixes:

- points accounting
- session charging
- PASG conversion
- world revenue forwarding

Without a tighter plan, implementation could drift into re-centering creator economics around usage, which now contradicts the clarified product model.

### Gap 2: There is no explicit standard interface yet for "local economy settles through PASG"

`streaming-billing` exposes points and conversion, but there is no clean, named "world local economy" interface that says:

- what a world-local unit is
- whether it is points-only or metadata-backed
- how it previews or settles against PASG
- how integrators query that path without bypassing Phase 2 PASG utility semantics

### Gap 3: Hybrid payment is underspecified

The repo has PASG-native payment and fiat-assisted PASG accounting, but no single bounded interface yet that makes hybrid settlement explicit for local-world flows. Planning has to decide whether hybrid means:

- PASG plus points conversion in `streaming-billing`
- PASG-aware previews with existing billing primitives
- or a larger generalized escrow layer

Given current product direction, the first two are safer than the third.

### Gap 4: Refund-safe settlement should extend known commerce patterns, not create a general escrow system

The product still needs refund-safe behavior where architecture requires it, but the current repo already has examples in marketplace and auction flows. The strongest default is:

- reuse or mirror existing refund-safe patterns
- only add new escrow machinery when a concrete uncovered settlement gap is proven

### Gap 5: Registry and ecosystem should not absorb local-economy policy by default

Phase 5 clarified that `ecosystem` is an administrative context and `collection` owns royalties. That implies Phase 6 should keep local-economy configuration close to the world or billing surface unless a real cross-contract need proves otherwise.

## Recommended Phase 6 Direction

### Direction 1: Reframe Phase 6 around PASG-aligned local-economy interfaces, not creator usage monetization

The phase should explicitly protect the creator-revenue model:

- initial sale
- resale royalties
- marketplace fees

Local economies remain allowed, but they are auxiliary and must remain compatible with PASG utility rather than replacing commerce-based monetization.

### Direction 2: Treat `streaming-billing` as the likely anchor, but narrow its semantics

`streaming-billing` already contains:

- PASG conversion
- local points balances
- session accounting
- world configuration

That makes it the most likely place to define a standard local-economy interface. But the phase should tighten that contract's public semantics so it no longer implies "creator revenue from usage" by default.

### Direction 3: Keep `split-router` generic and use it only where routing is genuinely needed

Phase 2 and Phase 5 both established that business-specific policy should not pollute routing. Phase 6 should keep that line:

- `split-router` can remain generic for actual routing or preview
- local-economy/business meaning should live in billing/config/query surfaces and docs

### Direction 4: Reuse marketplace/auction refund-safe patterns before inventing a new escrow primitive

The plan should inspect and extend existing:

- bid refund behavior in `marketplace-v3`
- auction custody/closeout behavior in `auction-english`

Only if those patterns do not cover the needed hybrid settlement path should Phase 6 introduce a new minimal primitive.

### Direction 5: Keep local-economy configuration out of `registry` unless a concrete cross-contract discovery need appears

Nothing in the clarified product direction says registry must catalog world-local economies. The default should therefore be:

- world-local config stays near `streaming-billing` or another bounded world-facing contract
- `registry` remains an affiliation and ownership lookup layer

## Likely Work Breakdown

### 06-01
Define and lock the multi-economy interface boundary:

- clarify that creator revenue is commerce-first, not usage-first
- define a standard local-unit or points model that still settles through PASG
- tighten `streaming-billing` query/execute semantics
- keep registry and ecosystem out of the economic-policy center

Likely files:
- `contracts/core/streaming-billing/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- `contracts/core/streaming-billing/README.md`
- `contracts/core/split-router/src/msg.rs`
- `README.md`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`

### 06-02
Add or refine hybrid settlement and refund-safe behavior using existing primitives first:

- PASG-aware previews and settlement rules for local points or internal units
- refund-safe execution and tests for any new hybrid path
- avoid a generic escrow layer unless a gap is proven

Likely files:
- `contracts/core/streaming-billing/src/msg.rs`
- `contracts/core/streaming-billing/src/contract.rs`
- `contracts/core/streaming-billing/src/state.rs`
- `contracts/core/streaming-billing/src/error.rs`
- `contracts/core/streaming-billing/src/tests/*`
- `contracts/nft/marketplace-v3/src/contract/execute.rs`
- `contracts/nft/auction-english/src/execute.rs`
- `contracts/core/split-router/src/msg.rs`

### 06-03
Align world settlement docs, schemas, and integration surfaces:

- document the final local-economy model
- show how it coexists with collection-level royalties
- make clear where chain-native, protocol, and off-chain boundaries sit

Likely files:
- `README.md`
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`
- `contracts/core/streaming-billing/README.md`
- `contracts/core/split-router/README.md`
- `contracts/core/streaming-billing/examples/schema.rs`

## Planning Risks

1. `streaming-billing` already mixes points, sessions, and world revenue. Small API changes can ripple through docs and tests quickly.
2. If the plan is too vague, implementation can accidentally rebuild a "usage-revenue engine" that contradicts product direction.
3. If the plan is too aggressive, it can drag `registry` or `ecosystem` into economic-policy concerns that Phase 5 deliberately avoided.
4. Hybrid payment is easy to overdesign. Phase 6 must prefer bounded interfaces and proof of need over generic settlement machinery.
5. Refund-safe settlement is real, but only in specific flows. Treating it as a universal escrow requirement would likely overbuild the phase.

## Recommended Validation Strategy

- Verify that at least one plan explicitly protects the commerce-first creator-revenue model.
- Verify that at least one plan explicitly defines a standard local-economy settlement interface through PASG-aware semantics.
- Verify that no plan makes `ecosystem` or `registry` the default economic-policy authority.
- Verify that refund-safe settlement is addressed through existing commerce patterns first, with any new primitive justified explicitly.
- Keep `cargo check -p streaming-billing -p split-router` and `cargo check --workspace` in the verification floor.
- Keep targeted tests around any new hybrid or refund-safe paths in `streaming-billing`, plus regression coverage for touched marketplace or auction behavior.

## Research Conclusion

Phase 6 should now be planned as a three-wave boundary-tightening phase:
1. lock the local-economy interface around PASG-aware settlement without redefining creator monetization
2. implement any needed hybrid/refund-safe extensions in existing bounded primitives first
3. align docs, examples, schemas, and verification to one consistent multi-economy story
