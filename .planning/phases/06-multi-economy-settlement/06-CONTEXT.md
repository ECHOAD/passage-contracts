# Phase 6: Multi-Economy Settlement - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning

## Phase Boundary

Define how worlds may expose local economies or internal units without turning world usage into the primary creator monetization path. The phase is about safe PASG-aligned settlement boundaries, hybrid-path constraints, and reuse of existing marketplace, auction, routing, and billing primitives where appropriate.

## Implementation Decisions

### World revenue model
- World usage is not the primary on-chain revenue source for creators.
- Creator revenue on-chain is anchored to initial sale plus resale royalties.
- Passage revenue on-chain is anchored to marketplace fees.
- Local world economies are allowed conceptually, but they are not the core creator monetization path for this phase.
- `streaming-billing` must not be reinterpreted as the main economic model for world creator revenue.

### Role of ecosystem vs collection
- Royalties and asset monetization live at the `collection` level.
- `ecosystem` is not the primary economic policy engine.
- `ecosystem` remains an administrative and organizational context for membership, affiliation, and governance of collections.
- Phase 6 should not move royalty logic upward into ecosystem-level controls by default.

### Meaning of multi-economy settlement
- Worlds may have local units or internal economies, but the protocol's strong on-chain guarantees still center on asset commerce and PASG-aware settlement surfaces.
- Phase 6 should not assume usage billing is the default monetization path for worlds.
- The phase should focus on settlement boundaries and interfaces, not on inventing a mandatory contract-local world-economy engine.
- Any local-world economy support should remain compatible with PASG utility rather than bypass it.

### Refund-safe settlement stance
- Refund-safe behavior matters where existing commerce flows already need it.
- Current marketplace and auction primitives are the first place to look for safe refund and settlement patterns.
- Phase 6 should not assume a brand new generic escrow primitive by default.
- New escrow or refund machinery is only justified if research finds a concrete uncovered gap.

### Claude's Discretion
- Whether Phase 6 should primarily extend `streaming-billing`, stay interface-and-doc focused, or add a minimal new primitive if a real gap is proven.
- Whether any world-local economy metadata belongs in `registry`, a world-specific config surface, or documentation only.
- Exact shape of any hybrid-payment interface, as long as it does not contradict the locked revenue and royalty decisions above.

## Specific Ideas

- CEO/product clarification: worlds do not generate creator revenue from usage by default.
- A creator selling a world should earn from the initial sale and future resales.
- Marketplace convenience matters; excessive royalties would push users to off-market resales.
- This phase should respect the real product monetization model rather than overfitting to the older roadmap wording around usage, escrow, or billing.

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase definition
- `.planning/ROADMAP.md` - Phase 6 goal, success criteria, and official scope anchor
- `.planning/REQUIREMENTS.md` - `ECON-01`, `ECON-02`, and `REV-02` requirement targets
- `.planning/STATE.md` - current milestone position and prior phase decisions
- `.planning/PROJECT.md` - program-level boundaries and core value

### PASG utility and billing boundaries
- `contracts/core/streaming-billing/src/msg.rs` - current PASG utility execute/query surface and points model
- `contracts/core/streaming-billing/src/contract.rs` - current session, points, and pending revenue behavior
- `contracts/core/streaming-billing/README.md` - explicit business-boundary statements for PASG utility vs off-chain billing
- `contracts/core/split-router/src/msg.rs` - generic routing surface that Phase 6 must not pollute with type-specific policy
- `contracts/core/split-router/README.md` - routing expectations and generic split behavior

### Commerce and creator monetization baseline
- `contracts/nft/marketplace-v3/src/msg.rs` - current sale and registration surface for resale commerce
- `contracts/nft/marketplace-v3/src/contract/execute.rs` - current refund and settlement paths for bids and sales
- `contracts/nft/auction-english/src/msg.rs` - custodial auction settlement surface
- `contracts/nft/auction-english/src/execute.rs` - current auction closeout and safe settlement logic
- `contracts/nft/pg721/src/msg.rs` - collection-level royalty and typed asset surface
- `contracts/nft/pg721-updatable/src/msg.rs` - matching typed asset surface with updatable token metadata

### Registry and asset-context model
- `contracts/core/registry/src/msg.rs` - canonical ecosystem and collection affiliation surfaces after Phase 5
- `.planning/phases/05-creator-asset-contracts-monetization/05-CONTEXT.md` - prior creator asset decisions that Phase 6 must build on
- `.planning/phases/05-creator-asset-contracts-monetization/05-VERIFICATION.md` - proof of the settled collection/ecosystem/royalty baseline

### Product architecture inputs
- `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md` - trust boundary between protocol and platform services
- `../context/product/architecture/FIAT_TO_CRYPTO_PAYMENT_FLOW.md` - existing hybrid payment and points framing behind `streaming-billing`

## Existing Code Insights

### Reusable Assets
- `streaming-billing` already provides PASG-native accounting, points balances, session charging, and world revenue forwarding semantics.
- `split-router` already provides generic routing and preview semantics without locking in business-specific token policy.
- `marketplace-v3` already contains refund-safe bid flows and explicit PASG settlement attributes.
- `auction-english` already contains custodial settlement logic for commerce flows.

### Established Patterns
- PASG utility is query-first and native-denom-first.
- Collection-level royalties are already the source of creator-side monetization semantics.
- Existing phases deliberately keep platform billing, subscriptions, rendering, and similar workflows off-chain.
- New protocol surfaces should prefer tightening existing bounded primitives before adding new generalized machinery.

### Integration Points
- If Phase 6 needs on-chain local economy support, it will likely connect through `streaming-billing`, `split-router`, and collection/world ownership checks via `registry` and cw721.
- If Phase 6 needs refund-safe extensions, it should first reuse marketplace or auction settlement patterns.
- Any new interface must remain coherent with Phase 2 PASG utility semantics and Phase 5 creator-asset monetization semantics.

## Deferred Ideas

- Reinterpreting world usage as the main creator revenue source - explicitly out of scope for this phase.
- Moving royalties from collection-level semantics into ecosystem-level policy - deferred and currently not desired.
- Designing a generic escrow primitive without first proving a concrete gap - deferred pending research evidence.

---

*Phase: 06-multi-economy-settlement*
*Context gathered: 2026-03-19*
