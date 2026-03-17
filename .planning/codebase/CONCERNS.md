# Codebase Concerns

**Analysis Date:** 2026-03-17

## Tech Debt

**Streaming billing relies on off-chain discipline instead of on-chain roles:**
- Issue: `contracts/core/streaming-billing/src/msg.rs` documents `StartSession` as "called by backend" and `StopSession` as backend-controlled, but `contracts/core/streaming-billing/src/contract.rs` does not define or enforce a backend/operator role for those paths. `stripe_webhook_validator` is persisted in `contracts/core/streaming-billing/src/state.rs` and surfaced in config/query types, but no execute path uses it.
- Files: `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/state.rs`, `contracts/core/streaming-billing/README.md`
- Impact: critical payment/session logic is only safe if every off-chain caller behaves correctly; the contract itself does not enforce that trust boundary.
- Fix approach: add an explicit backend/operator role and enforce it on `StartSession`, `StopSession`, and world-management paths; either wire `stripe_webhook_validator` into execution or remove it from config/docs.

**`minter-v2` and `minter-v2-metadata-onchain` have diverged behavior:**
- Issue: `contracts/nft/minter-v2/src/contract/helpers.rs` implements whitelist-aware pricing and whitelist membership checks, with helper coverage in `contracts/nft/minter-v2/src/contract/helpers/tests.rs`. The metadata-onchain variant still contains TODOs and hardcodes public-price behavior in `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`.
- Files: `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/minter-v2/src/contract/helpers/tests.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`
- Impact: the same sale configuration produces different pricing and query semantics depending on metadata mode.
- Fix approach: port the whitelist-aware helper/query logic from `contracts/nft/minter-v2/src/contract/helpers.rs` into the metadata-onchain variant and add matching tests.

**Reply parsing is duplicated and format-sensitive:**
- Issue: reply handlers extract instantiated contract addresses using different techniques: `contracts/core/collection-factory/src/contract/helpers.rs` scrapes `_contract_address` from events, while `contracts/core/ecosystem-factory/src/contract/reply.rs` parses reply data directly. The minter v2 instantiate/reply flow also scrapes instantiate events in `contracts/nft/minter-v2/src/contract/instantiate.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/instantiate.rs`.
- Files: `contracts/core/collection-factory/src/contract/helpers.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/nft/minter-v2/src/contract/instantiate.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/instantiate.rs`
- Impact: contract creation flows are coupled to reply/event layout details and are harder to upgrade safely across CosmWasm/runtime changes.
- Fix approach: standardize reply parsing on one mechanism and cover reply handlers with dedicated tests.

## Known Bugs

**Streaming sessions can be started and stopped by arbitrary callers:**
- Symptoms: `execute_start_session` in `contracts/core/streaming-billing/src/contract.rs` ignores `info.sender` and accepts any `user` string; `execute_stop_session` also ignores `info.sender` and trusts the caller-supplied `duration_seconds`.
- Files: `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/msg.rs`
- Trigger: any address can call `StartSession` for any funded user, then call `StopSession` with an arbitrary duration to charge that user.
- Workaround: no on-chain workaround; safety depends entirely on routing calls through a trusted backend.

**World-rate ownership can be squatted, and session records can drift from stored world config:**
- Symptoms: `execute_set_world_rate` in `contracts/core/streaming-billing/src/contract.rs` contains a TODO for registry ownership verification and saves `info.sender` as the world owner with no authorization check. `execute_start_session` stores the caller-supplied `world_collection` instead of reusing `world_config.world_collection`.
- Files: `contracts/core/streaming-billing/src/contract.rs`
- Trigger: the first caller to set a `world_nft_id` claims ownership, and later session records can persist a collection address that does not match the stored world configuration.
- Workaround: no on-chain workaround.

**`marketplace-v3` exposes query filters/pagination it does not honor:**
- Symptoms: `AsksByPrice { collection }` ignores the `collection` filter, `AskCount { collection }` ignores `collection` and counts every stored ask, and several list queries ignore `start_after` entirely by naming parameters `_start_after` in `contracts/nft/marketplace-v3/src/contract/query.rs`.
- Files: `contracts/nft/marketplace-v3/src/msg.rs`, `contracts/nft/marketplace-v3/src/contract/query.rs`
- Trigger: clients use `AsksByPrice`, `AskCount`, `AsksBySeller`, `BidsByToken`, `BidsByBidder`, `CollectionBidsByCollection`, or `CollectionBidsByBidder` expecting correct filtering/pagination.
- Workaround: client-side filtering/overfetching.

**Registry authorized-minter pagination is declared but not implemented:**
- Symptoms: `QueryMsg::AuthorizedMinters` accepts `start_after` in `contracts/core/registry/src/msg.rs`, but `query_authorized_minters` in `contracts/core/registry/src/contract/query.rs` names it `_start_after` and ignores it.
- Files: `contracts/core/registry/src/msg.rs`, `contracts/core/registry/src/contract/query.rs`
- Trigger: collections with enough authorized minters to require pagination.
- Workaround: none beyond fetching from the beginning and trimming client-side.

## Security Considerations

**Streaming billing has an incomplete trust boundary:**
- Risk: public callers can mutate session state and charge balances because backend-only semantics are comments, not authorization rules. The same module also allows arbitrary callers to claim world ownership in `execute_set_world_rate`.
- Files: `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/core/streaming-billing/src/state.rs`
- Current mitigation: `ReportFiatPurchase` is limited to `fiat_oracle`, `ForceStopSession` and `UpdateConfig` are admin-gated, and `paused` is enforced in the top-level execute entry point.
- Recommendations: enforce a backend/operator role on session lifecycle calls, validate world ownership against the registry before persisting configs, and stop accepting caller-supplied world metadata where authoritative state already exists.

**Fiat reporting safeguards are weaker than the documented design:**
- Risk: `contracts/core/streaming-billing/README.md` claims timestamp freshness checks and Stripe webhook validation, but `execute_report_fiat_purchase` in `contracts/core/streaming-billing/src/contract.rs` only checks the sender and scans one user's purchase history for a duplicate `transaction_id`.
- Files: `contracts/core/streaming-billing/README.md`, `contracts/core/streaming-billing/src/contract.rs`
- Current mitigation: trusted `fiat_oracle` sender gate and per-user duplicate detection.
- Recommendations: enforce freshness checks against `env.block.time`, use `stripe_webhook_validator` or remove it, and index `transaction_id` globally rather than per user.

**Random mint selection is predictable:**
- Risk: `get_random_token_id` in both v2 minters derives selection from `env.block.height + env.block.time.nanos()`, which is predictable and validator-influenceable.
- Files: `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`
- Current mitigation: none beyond normal sold-out checks and token-availability checks.
- Recommendations: replace this with a deterministic reveal, commit-reveal, or another inventory-selection mechanism that does not expose mint order to block producers and callers.

## Performance Bottlenecks

**Random minting performs an O(1,000,000) scan per mint:**
- Problem: both v2 minters iterate `for i in 1..=1000000u32` and probe `MINTABLE_TOKEN_IDS.has(storage, i)` to locate the selected token.
- Files: `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`
- Cause: remaining supply is stored as a sparse map without a compact index of available token IDs.
- Improvement path: keep a compact list/tree of remaining token IDs and remove entries in O(1) or O(log n).

**Registry recovery-case checks and some queries scan full maps:**
- Problem: `has_open_case_for_target` linearly walks `RECOVERY_CASES`, and `query_collection_creation_requests` / `query_authorized_minters` scan global maps before filtering.
- Files: `contracts/core/registry/src/contract/execute.rs`, `contracts/core/registry/src/contract/query.rs`
- Cause: no secondary index from recovery target to open cases, and no collection- or ecosystem-scoped paginated index for those query families.
- Improvement path: add prefixable indexes for open recovery cases, collection creation requests, and authorized minters.

**Streaming billing execute/query paths scale with per-user history:**
- Problem: duplicate fiat detection scans all purchases for one user, and `query_user_sessions` loads every session for the user with no `limit` or `start_after`.
- Files: `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/msg.rs`
- Cause: there is no transaction ID index and no paginated user-session query.
- Improvement path: add a global transaction-id lookup map and a paginated session query surface over `USER_SESSIONS`.

**`nft-vault` rewrites an entire claim vector per user:**
- Problem: `Claims` stores `Vec<Claim>` under one address key, and `claim_tokens` partitions/clones the whole vector on each claim.
- Files: `contracts/staking/nft-vault/src/claim.rs`
- Cause: claim storage is aggregated by user rather than by claim/release bucket.
- Improvement path: split each claim into its own key, matching the TODO already present in `contracts/staking/nft-vault/src/claim.rs`.

## Fragile Areas

**Registry execute flow is too concentrated for its current test surface:**
- Files: `contracts/core/registry/src/contract/execute.rs`, `contracts/core/registry/src/tests/collections.rs`, `contracts/core/registry/src/tests/recovery.rs`
- Why fragile: `contracts/core/registry/src/contract/execute.rs` is a 1,498-line file spanning moderation, ecosystem membership, collection registration, minter authorization, and recovery governance, but the local tests only cover one collection query and three recovery scenarios.
- Safe modification: split handlers by domain before larger changes, and add tests for ecosystem membership, moderation toggles, collection registration, minter authorization, and ownership transfer first.
- Test coverage: coverage is thin outside the recovery path and one collection filter query.

**`marketplace-v3` has large state transitions with only helper-level tests:**
- Files: `contracts/nft/marketplace-v3/src/contract/execute.rs`, `contracts/nft/marketplace-v3/src/contract/query.rs`, `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`
- Why fragile: ask, bid, collection-bid, sync, and config flows span the main execute/query modules, but detected tests only exercise royalty-routing helper behavior.
- Safe modification: add execute/query tests around ask lifecycle, bid lifecycle, collection bids, sync paths, and pagination before touching indexes or payment logic.
- Test coverage: no direct tests detected for the public query contract or execute state machine.

**Factory reply paths are operationally important and untested:**
- Files: `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/helpers.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`
- Why fragile: reply handlers parse instantiate responses, persist new records, and immediately emit downstream registry-registration messages.
- Safe modification: keep reply parsing isolated, add mocked reply tests first, and avoid changing reply IDs or emitted payload shapes without coverage.
- Test coverage: no tests detected in `contracts/core/collection-factory/src/` or `contracts/core/ecosystem-factory/src/`.

**Streaming billing combines accounting, trust boundaries, and revenue routing with no local tests:**
- Files: `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/lib.rs`
- Why fragile: deposit handling, fiat credits, session charging, world ownership, and revenue distribution all live in one 874-line module.
- Safe modification: build unit tests around session lifecycle, fiat reporting, world config changes, and revenue distribution before further feature work.
- Test coverage: no `#[cfg(test)]` module or colocated tests detected.

## Scaling Limits

**Random-mint throughput is bounded by ID-space size, not remaining supply:**
- Current capacity: every random mint may inspect up to one million candidate token IDs in `contracts/nft/minter-v2/src/contract/helpers.rs` and `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`.
- Limit: sparse inventories become increasingly expensive even when only a few tokens remain.
- Scaling path: switch to a compact remaining-token structure.

**Streaming billing histories are effectively unbounded for heavy users:**
- Current capacity: `PurchaseHistory` is paginated, but duplicate detection and `UserSessions` are not bounded by indexed lookups.
- Limit: users with long purchase/session histories make both execute and query paths more expensive.
- Scaling path: add dedicated transaction and session indexes plus paginated query APIs.

**Registry recovery/admin surfaces degrade as global state grows:**
- Current capacity: open-case checks and some authorization queries scan global maps.
- Limit: unrelated ecosystems and collections contribute to gas/latency for each new recovery case or authorized-minter query.
- Scaling path: add scoped indexes keyed by target, status, collection, and ecosystem.

**`nft-vault` claim state grows as a per-user blob:**
- Current capacity: every unstake appends to one `Vec<Claim>` value in `contracts/staking/nft-vault/src/claim.rs`.
- Limit: users with many partial unbondings pay full read/clone/write costs on each claim.
- Scaling path: move to per-claim keys and maturity-based pagination.

## Dependencies at Risk

**Exact-pinned core dependencies make upgrades high-friction:**
- Risk: `Cargo.toml` pins `cosmwasm-std = "=2.1.3"`, `cosmwasm-schema = "=2.1.3"`, `cw-multi-test = "=2.1.1"`, and `sylvia = "=1.2.1"` exactly across the workspace.
- Impact: runtime or security upgrades will require coordinated changes across every contract at once, which is riskier because reply handling and test coverage are inconsistent today.
- Migration plan: broaden test coverage on `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/registry/src/contract/execute.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`, and the factory reply modules before relaxing these pins.

## Missing Critical Features

**Streaming billing has no enforceable backend/session-attestation role:**
- Problem: the API surface in `contracts/core/streaming-billing/src/msg.rs` assumes backend-controlled session management, but config/state do not model that role and `contracts/core/streaming-billing/src/contract.rs` does not enforce it.
- Blocks: safe production use of session lifecycle, duration accounting, and user charging.

**Streaming billing does not implement the documented freshness and webhook checks:**
- Problem: `contracts/core/streaming-billing/README.md` documents timestamp freshness and webhook validation, but those checks are absent from `contracts/core/streaming-billing/src/contract.rs`.
- Blocks: trustworthy fiat credits and strong replay protection.

**`marketplace-v3` pagination/filter support is incomplete:**
- Problem: several public query parameters in `contracts/nft/marketplace-v3/src/msg.rs` are placeholders only, because `contracts/nft/marketplace-v3/src/contract/query.rs` ignores them.
- Blocks: reliable frontend pagination and filtered marketplace views.

**Whitelist pricing is incomplete in `minter-v2-metadata-onchain`:**
- Problem: whitelist configuration exists in `contracts/nft/minter-v2-metadata-onchain/src/msg.rs` and state, but price resolution/query logic still uses public pricing.
- Blocks: consistent sale behavior across v2 minter variants.

**Streaming billing does not enforce the README's maximum-session-duration guarantee:**
- Problem: `contracts/core/streaming-billing/README.md` promises backend-controlled duration and a 24-hour auto-stop, but `contracts/core/streaming-billing/src/contract.rs` accepts arbitrary `duration_seconds` with no upper bound.
- Blocks: bounded session charging and safe unattended session cleanup.

## Test Coverage Gaps

**`streaming-billing` is untested:**
- What's not tested: deposit accounting, fiat purchase reporting, session start/stop authorization, world-rate ownership, and revenue distribution in `contracts/core/streaming-billing/src/contract.rs`
- Files: `contracts/core/streaming-billing/src/contract.rs`, `contracts/core/streaming-billing/src/lib.rs`
- Risk: critical auth/accounting defects can ship unnoticed.
- Priority: High

**Factory registration and reply flows are untested:**
- What's not tested: create/reply/register flows in `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/ecosystem-factory/src/contract/execute.rs`, and `contracts/core/ecosystem-factory/src/contract/reply.rs`
- Files: `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`
- Risk: deployment-time failures or reply parsing regressions can break factory workflows without local feedback.
- Priority: High

**`minter-v2-metadata-onchain` lacks parity tests:**
- What's not tested: whitelist-aware pricing, registry gating, and random token selection in the metadata-onchain minter variant
- Files: `contracts/nft/minter-v2-metadata-onchain/src/contract/execute.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/helpers.rs`, `contracts/nft/minter-v2-metadata-onchain/src/contract/query.rs`
- Risk: behavioral drift from `contracts/nft/minter-v2/src/contract/helpers.rs` can persist unnoticed.
- Priority: High

**`marketplace-v3` execute/query behavior is mostly untested:**
- What's not tested: ask lifecycle, bid lifecycle, collection-bid lifecycle, sync flows, and public query pagination/filter semantics
- Files: `contracts/nft/marketplace-v3/src/contract/execute.rs`, `contracts/nft/marketplace-v3/src/contract/query.rs`, `contracts/nft/marketplace-v3/src/contract/helpers/tests.rs`
- Risk: public marketplace behavior can regress even though helper-level royalty tests still pass.
- Priority: High

**`stake-rewards` and `vault-factory` have no local contract tests:**
- What's not tested: reward accrual math, claim flows, instantiate2 addressing, and vault creation flows
- Files: `contracts/staking/stake-rewards/src/contract.rs`, `contracts/staking/vault-factory/src/contract.rs`
- Risk: accounting or deployment bugs remain latent in staking infrastructure.
- Priority: Medium

---

*Concerns audit: 2026-03-17*

