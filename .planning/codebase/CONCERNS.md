# Codebase Concerns

**Analysis Date:** 2026-03-18

## Tech Debt

**Streaming billing still carries an unused webhook-validator field:**
- Issue: `contracts/core/streaming-billing/src/msg.rs` and `contracts/core/streaming-billing/src/state.rs` still define `stripe_webhook_validator`, but `contracts/core/streaming-billing/src/contract.rs` does not use it in any execute path.
- Why: the field appears to be preserved from an earlier integration design.
- Impact: webhook validation stays entirely off-chain, and the config surface is larger than the actual behavior.
- Fix approach: either remove the field from config/docs or wire it into a real on-chain verification flow.

**`nft-vault` stores claims as a per-user vector:**
- Issue: `contracts/staking/nft-vault/src/claim.rs` keeps `Vec<Claim>` under a single address key and already flags the design with a TODO.
- Why: the current shape is simple for small volumes and matches the existing tests in `contracts/staking/nft-vault/src/claim/test.rs`.
- Impact: each claim update rewrites the full vector for that user.
- Fix approach: move to one record per claim or per release bucket.

## Known Bugs

**`minter-v2-metadata-onchain` does not report whitelist pricing in `MintPrice`:**
- Symptoms: `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs` returns `whitelist_price: None` even though execute logic in `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs` can honor whitelist pricing.
- Trigger: clients query mint price while whitelist minting is active.
- Workaround: inspect the active whitelist contract directly or rely on execute-side pricing checks.
- Root cause: the query path still hardcodes a public-price response.

## Security Considerations

**Random mint selection is predictable in both v2 minters:**
- Risk: `get_random_token_id` in `contracts/nft/minter-v2/src/contract/helpers.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs` derives selection from `env.block.height + env.block.time.nanos()` and then linearly scans token IDs.
- Current mitigation: sold-out checks and token-availability checks prevent invalid mints, but not predictability.
- Recommendations: replace the selection logic with commit-reveal, deterministic allocation, or another scheme that does not expose mint order to block producers.

**Streaming billing now gates session lifecycle, but webhook attestation still stays off-chain:**
- Risk: `contracts/core/streaming-billing/src/contract.rs` now enforces `backend_operator` and role checks for `StartSession` and `StopSession`, but it still does not validate Stripe signatures on-chain even though `stripe_webhook_validator` is stored.
- Current mitigation: `ReportFiatPurchase` is limited to `fiat_oracle`, timestamps are bounded, and duplicate transaction IDs are tracked globally in `FIAT_PURCHASE_TX_IDS`.
- Recommendations: keep webhook verification fully off-chain by design, or implement a real on-chain attestation path and remove the dead config surface.

## Performance Bottlenecks

**Random minting still does a one-million-ID scan:**
- Problem: both v2 minters iterate `1..=1000000` in `contracts/nft/minter-v2/src/contract/helpers.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs` to find the selected token.
- Measurement: the scan upper bound is fixed at 1,000,000 candidate token IDs.
- Cause: remaining inventory is tracked as a sparse map instead of a compact index.
- Improvement path: maintain a dense remaining-token structure and remove entries in O(1) or O(log n).

**Streaming billing user-session queries grow without pagination:**
- Problem: `query_user_sessions` in `contracts/core/streaming-billing/src/contract.rs` loads every session for a user and `contracts/core/streaming-billing/src/msg.rs` exposes no `limit` or `start_after`.
- Measurement: the query cost scales with the full per-user session history.
- Cause: there is no paginated session query surface over `USER_SESSIONS`.
- Improvement path: add pagination and, if needed, a dedicated session index.

**`nft-vault` claim release rewrites whole claim lists:**
- Problem: `claim_tokens` in `contracts/staking/nft-vault/src/claim.rs` partitions and clones the stored `Vec<Claim>` on every claim.
- Measurement: the work done per claim scales with the full pending-claim list for that address.
- Cause: claims are aggregated under one address key.
- Improvement path: store claims as individually addressable records and paginate by maturity.

## Fragile Areas

**`contracts/core/streaming-billing/src/contract.rs` mixes accounting, world config, and revenue routing:**
- Why fragile: one module handles deposits, fiat credits, session lifecycle, world-rate setup, and revenue distribution.
- Common failures: a change in one path can affect multiple state maps or invariants.
- Safe modification: add focused tests around `StartSession`, `StopSession`, `SetWorldRate`, `DistributeWorldRevenue`, and `query_user_sessions` before changing state shape.
- Test coverage: `contracts/core/streaming-billing/src/tests.rs` covers auth and accounting basics, but not the distribution path or session-query scale behavior.

**Registry query flows still rely on broad state scans in a few places:**
- Why fragile: `contracts/core/registry/src/contract/query.rs` supports several listing endpoints, including collection-creation requests and ecosystem listings, through global map iteration and filtering.
- Common failures: new filters or ordering rules can silently change query semantics.
- Safe modification: extend the focused registry tests under `contracts/core/registry/src/tests/` before changing indexes or query order.
- Test coverage: `contracts/core/registry/src/tests/minters.rs` covers authorized-minter pagination, but I did not find comparable coverage for the collection-creation-request listing path.

## Scaling Limits

**Random-mint throughput is bounded by the ID space, not remaining supply:**
- Current capacity: every random mint may inspect up to one million candidate token IDs in both v2 minter variants.
- Limit: sparse inventories become more expensive even when only a few tokens remain.
- Scaling path: switch to a compact remaining-token structure.

**Streaming billing histories grow with user activity:**
- Current capacity: purchase history is paginated in `contracts/core/streaming-billing/src/contract.rs`, but session history is not.
- Limit: heavy users make the `UserSessions` query progressively more expensive.
- Scaling path: add paginated session queries and consider secondary indexes for high-volume users.

**`nft-vault` claim state grows as a per-user blob:**
- Current capacity: each user keeps a single vector of claims in `contracts/staking/nft-vault/src/claim.rs`.
- Limit: users with many partial unbondings pay full read/clone/write costs.
- Scaling path: move to per-claim keys and maturity-based pagination.

## Dependencies at Risk

**Workspace dependencies are pinned exactly:**
- Risk: `Cargo.toml` pins `cosmwasm-std = "=2.1.3"`, `cosmwasm-schema = "=2.1.3"`, `cw-multi-test = "=2.1.1"`, and `sylvia = "=1.2.1"` exactly across the workspace.
- Impact: runtime or security upgrades require coordinated changes across the entire contract set.
- Migration plan: broaden test coverage on `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`, and the factory reply modules before relaxing pins.

## Missing Critical Features

**On-chain Stripe attestation is still missing:**
- Problem: `contracts/core/streaming-billing/src/msg.rs` and `contracts/core/streaming-billing/src/state.rs` keep a `stripe_webhook_validator` field, but `contracts/core/streaming-billing/src/contract.rs` never consumes it.
- Blocks: any design that wants the contract itself to prove Stripe webhook authenticity.
- Current workaround: keep signature verification off-chain.

**`MintPrice` in the metadata-onchain minter does not surface whitelist price state:**
- Problem: `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs` hardcodes `whitelist_price` to `None`.
- Blocks: clients cannot preview whitelist pricing from the query API even when the execute path may use it.
- Current workaround: inspect execute-side rules or the whitelist contract directly.

## Test Coverage Gaps

**`streaming-billing` still lacks coverage for the revenue-distribution and session-query edges:**
- What's not tested: `DistributeWorldRevenue`, `BatchDistributeRevenue`, and `UserSessions` pagination behavior in `contracts/core/streaming-billing/src/contract.rs`.
- Risk: distribution regressions or query blowups can ship without local feedback.
- Priority: High

**`minter-v2-metadata-onchain` has no dedicated local tests in this crate:**
- What's not tested: whitelist pricing query parity, registry-gated mint behavior, and metadata-onchain mint helper behavior in `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`.
- Risk: the metadata-onchain variant can drift from `contracts/nft/minter-v2/src/contract/helpers.rs`.
- Priority: High

**Registry listing paths still need more query-specific coverage:**
- What's not tested: collection-creation-request listings and admin-scoped ecosystem listings in `contracts/core/registry/src/contract/query.rs`.
- Risk: filter and ordering regressions could appear in higher-volume registry states.
- Priority: Medium

---

*Concerns audit: 2026-03-18*

