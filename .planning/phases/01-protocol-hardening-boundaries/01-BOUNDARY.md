# Phase 1 Protocol Boundary Contract

## On-Chain Guarantees

- `registry` is the authoritative on-chain index for Passage ecosystems, collections, moderation state, and ownership-recovery policy.
- `collection-factory` and `ecosystem-factory` create and register protocol objects on-chain, but they do not replace Passage product workflows.
- `marketplace-v3` is an on-chain trading primitive for listings, bids, collection bids, settlement math, and collection-level policy checks.
- `split-router` is an on-chain settlement primitive for sending already-determined revenue shares to recipients.
- `streaming-billing` is an on-chain accounting primitive for points balances, pending world revenue, fiat purchase records, and bounded session settlement.
- Future PASG governance and staking work belong on-chain because token-weighted voting, staking balances, unbonding state, and protocol parameters require trustless enforcement.

## Off-Chain and Hybrid Responsibilities

- Passage services own streaming infrastructure, session telemetry, disconnection detection, and any decision about when a session should start or stop in the product experience.
- Passage services own fiat onboarding, Stripe or other PSP integration, webhook validation, price sourcing, user support, refunds, and any flow that touches PII or compliance obligations.
- Passage services own search, discovery, analytics, moderation tooling, and premium-tier UX. Those concerns must not be forced on-chain for Phase 1.
- `streaming-billing` is hybrid: balances and accounting are on-chain, but the lifecycle inputs are service-invoked and therefore must be explicitly authorized.
- `split-router` is hybrid in invocation only: the split execution is on-chain, but a service, cron, or operator decides when to invoke settlement.

## Service-Invoked Contract Families

- `streaming-billing` is service-invoked for `StartSession`, `StopSession`, and fiat purchase reporting. Only the configured `backend operator`, admin, or fiat oracle may drive those paths.
- `split-router` is service-invoked as a settlement primitive. It should not decide business timing, retry policy, or subscription logic on its own.
- `collection-factory` and `ecosystem-factory` are operator-mediated protocol factories. Registry checks and operator permissions gate creation, but the surrounding creator workflow remains a Passage service concern.
- `marketplace-v3` is mostly user-invoked for trading, with admin, operator, and registry-assisted paths for brownfield operations.
- PASG governance remains protocol-scoped. Governance may change PASG protocol parameters, staking configuration, and fee-treatment rules, but it must not become a general control plane for off-chain Passage operations.

## Explicit Non-Goals

- No Passage subscription engine, premium-tier logic, or event-hosting workflow moves on-chain in Phase 1.
- No contract in this phase becomes the source of truth for fiat payment acceptance, Stripe webhook authenticity, or customer-support outcomes.
- `streaming-billing` does not prove real-world session duration by itself; it enforces who may report lifecycle events and how charges are bounded.
- PASG governance does not govern search ranking, moderation operations, or general platform staffing and infrastructure decisions.
- This phase does not introduce proxy-based upgradeability across the brownfield workspace. It documents the current pause, migrate, and cutover reality instead.

## Sources of Truth

- `registry` registration plus cw721 `OwnerOf` are the source of truth for creator world ownership in billing configuration.
- `streaming-billing` config is the source of truth for the admin, backend operator, fiat oracle, denom, and pause state that service-integrated billing paths must honor.
- `marketplace-v3`, `registry`, `collection-factory`, and `ecosystem-factory` source files are the source of truth for pause and authority behavior summarized in the safety matrix.
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md` and `../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md` remain the product-level architecture references this protocol boundary implements.