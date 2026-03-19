# Phase 6: Multi-Economy Settlement - Validation Strategy

**Defined:** 2026-03-19
**Scope:** validation guardrails for PASG-aligned local economies, hybrid settlement boundaries, and refund-safe execution
**Status:** ready-for-planning

## Validation Guardrails

- Phase 6 must preserve the clarified product model that creator revenue is commerce-first: initial sale plus resale royalties, not world-usage revenue by default.
- Any local-economy interface must remain PASG-aware and must not bypass the canonical `upasg` utility semantics established in Phase 2.
- `ecosystem` remains administrative context; it must not become the default royalty or economic-policy engine for this phase.
- `registry` remains a canonical affiliation/ownership surface; it should not become a local-economy registry without concrete justification.
- `split-router` must remain generic. Business-specific world-economy logic should not be pushed into routing messages or storage without a clear cross-contract need.
- Refund-safe settlement must reuse or mirror known marketplace and auction patterns before introducing any new generalized escrow primitive.
- If a new primitive becomes necessary, the plan must prove the uncovered gap explicitly rather than assuming "escrow" from roadmap wording alone.

## Wave 0 Checks For Plan

- The plan set covers `ECON-01`, `ECON-02`, and `REV-02`.
- At least one plan explicitly locks the commerce-first creator monetization model and narrows Phase 6 away from default world-usage monetization.
- At least one plan explicitly defines or tightens the PASG-aware local-economy interface.
- At least one plan explicitly covers hybrid payment and refund-safe settlement behavior while reusing existing primitives first.
- At least one plan explicitly aligns docs/examples/schema to the final interpretation so integrators do not infer a broader economic model than intended.

## Automated Verification To Keep

- `cargo check -p streaming-billing -p split-router`
- `cargo test -p streaming-billing --lib`
- `cargo check -p marketplace-v3 -p auction-english`
- `cargo test -p marketplace-v3 --lib`
- `cargo test -p auction-english --lib`
- `cargo check --workspace`
- `cargo unit-test`

These remain the regression floor. Phase 6 should also rely on plan-level grep checks proving that:

- creator monetization remains commerce-first in docs and public interfaces
- local-economy settlement remains PASG-aware
- no default generic escrow primitive is introduced without explicit rationale
- routing remains generic and registry remains non-authoritative for local-economy policy by default
