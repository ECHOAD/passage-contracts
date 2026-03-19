# Phase 9: marketplace-v3 registration, ownership validation, and admin approval redesign - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** User direction during `$gsd-do` + `$gsd-plan-phase 9`

<domain>
## Phase Boundary

Redesign `contracts/nft/marketplace-v3` so marketplace membership and collection configuration are governed by an approval workflow instead of permissive direct per-collection overrides.

This phase delivers:
- marketplace-level config simplification
- collection-scoped denom ownership moved out of instantiate defaults and into collection registration/update flows
- request + approval model for collection registration and collection config changes
- ownership validation so collection owners initiate requests, while marketplace admin can still register or update directly
</domain>

<decisions>
## Implementation Decisions

### Marketplace config surface
- Remove `denom` from `InstantiateMsg` and from marketplace-level `UpdateConfig`.
- Keep `trading_fee_bps` as a single marketplace-wide standard fee.
- Remove `max_trading_fee_bps` entirely.
- Remove `require_registration`; registration is mandatory for all trade flows.
- Preserve `min_price`, `fee_collector`, `registry`, `operators`, and pause/admin controls unless implementation review proves one is obsolete.

### Collection config model
- Collection-specific `denom` stays supported, but only at collection registration/update level.
- Collection-specific trading-fee overrides are removed.
- `CollectionFee` should resolve to the marketplace-wide standard fee, not a per-collection override.

### Approval workflow
- A collection owner can submit a request to join the marketplace.
- A collection owner can submit a request to update collection marketplace config.
- Marketplace admin can approve or reject those requests.
- Marketplace admin can register a collection directly without a request.
- Marketplace admin can update collection config directly without a request.

### Ownership validation
- Owner-submitted requests must verify actual collection ownership on-chain.
- Admin direct actions bypass request creation but still must preserve registration invariants.
- Trading must stay blocked for unregistered collections.

### Claude's Discretion
- Exact execute/query message names for request and resolution surfaces.
- Whether operators keep any registration powers or are reduced relative to admin.
- Whether request state lives inline with collection config or in dedicated maps.
- Migration shape for existing `marketplace-v3` config and collection entries.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Current marketplace interface and state
- `contracts/nft/marketplace-v3/src/msg.rs` - current instantiate, execute, and query wire surface
- `contracts/nft/marketplace-v3/src/state.rs` - current config and collection config shape
- `contracts/nft/marketplace-v3/src/contract.rs` - current execution routing and auth assumptions
- `contracts/nft/marketplace-v3/src/execute.rs` - current register/update/trade flow entrypoints
- `contracts/nft/marketplace-v3/src/query.rs` - current config, collection, and trade query behavior

### Existing request/approval pattern
- `contracts/core/registry/src/msg.rs` - `SubmitCollectionCreationRequest` and `ResolveCollectionCreationRequest` message pattern
- `contracts/core/registry/src/contract/execute.rs` - request persistence and approval rules
- `contracts/core/registry/src/contract/query.rs` - request query pattern
- `contracts/core/registry/src/state.rs` - request status and storage model

### Ownership and trading checks
- `contracts/nft/pg721/src/msg.rs` - ownership/query surface used to validate collection ownership
- `contracts/nft/marketplace-v3/src/error.rs` - current authorization and config errors
- `contracts/nft/marketplace-v3/README.md` - current documented behavior that will need correction

### Public docs that will drift if not updated
- `docs/01-end-to-end-setup.md`
- `docs/02-method-reference.md`
- `docs/03-json-examples.md`
</canonical_refs>

<specifics>
## Specific Ideas

- Registration should become a prerequisite, not a toggle.
- The admin path should remain operationally simple for trusted bootstrap or emergency cases.
- Request approval should cover both initial join and later collection config changes.
- The collection owner path should feel similar to `registry` request flows, not a custom ad-hoc one.
</specifics>

<deferred>
## Deferred Ideas

- Whether marketplace requests should integrate directly with `registry` moderation decisions beyond current `CanTradeCollection` checks.
- Whether direct admin registration should emit extra audit metadata or events beyond normal action attributes.
</deferred>

---

*Phase: 09-marketplace-v3-registration-ownership-validation-and-admin-approval-redesign*
*Context gathered: 2026-03-19 via direct user decisions*
