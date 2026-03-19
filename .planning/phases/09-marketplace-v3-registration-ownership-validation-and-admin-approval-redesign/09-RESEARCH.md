# Phase 9: Marketplace-v3 Registration, Ownership Validation, and Admin Approval Redesign - Research

**Researched:** 2026-03-19
**Domain:** marketplace-v3 registration and collection configuration model
**Confidence:** HIGH

## Summary

The current `marketplace-v3` surface mixes marketplace-global policy with collection-level overrides. `InstantiateMsg` and `UpdateConfig` still carry a default `denom`, `max_trading_fee_bps`, and `require_registration`, while collection registration allows both per-collection `denom` and per-collection `trading_fee_bps` overrides. This means the contract currently supports two fee layers and a registration mode flag, which conflicts with the requested redesign.

The requested direction is structurally clear:
- marketplace-global config should keep a single `trading_fee_bps`
- collection membership should always be mandatory for trading
- collection-specific `denom` stays at registration/update scope only
- collection owners request access or changes, marketplace admin approves/rejects
- marketplace admin can still perform direct registration/update without a request

## Current Drift To Remove

### Marketplace-global config drift
Current `InstantiateMsg` and `ConfigResponse` include:
- `denom`
- `trading_fee_bps`
- `max_trading_fee_bps`
- `require_registration`

Only `trading_fee_bps` still fits the requested long-term model.

### Collection-level drift
Current `RegisterCollection` and `UpdateCollectionConfig` allow:
- collection-specific `trading_fee_bps`
- collection-specific `denom`

The redesign keeps collection-specific `denom` but removes collection-specific fee overrides.

### Access model drift
Current README states registration can be performed by admin, operators, or registry, and query/config surface still treats registration as optional (`require_registration`). The requested model instead makes registration mandatory and adds owner-driven request submission plus admin resolution.

## Reusable Pattern

`registry` already has the repo's closest request/approval pattern:
- `SubmitCollectionCreationRequest`
- `ResolveCollectionCreationRequest`
- typed request status in state
- query surfaces for single request and request lists

That pattern should be adapted rather than inventing a brand-new flow for marketplace membership.

## Architectural Implications

1. `marketplace-v3` config storage will need a breaking shape change, because the stored marketplace config currently owns denom and max-fee policy.
2. Collection config and request state should be disentangled: active registered collection config should be separate from pending requested changes.
3. Ownership validation likely needs an on-chain owner lookup against the collection contract before accepting owner-submitted registration/update requests.
4. Query and docs drift will be significant because current `CollectionFee` and `Config` semantics assume per-collection fee override support.
5. Migration planning matters: existing configs and registered collections need a deterministic mapping into the new state model.

## Planning Direction

A good phase split is:
1. simplify wire/state model and remove marketplace-global denom / max-fee / optional registration
2. add owner-validated request + admin resolution flow for register/update
3. align docs, query semantics, migration expectations, and regression tests

## Primary Sources

- `contracts/nft/marketplace-v3/src/msg.rs`
- `contracts/nft/marketplace-v3/src/state.rs`
- `contracts/nft/marketplace-v3/src/contract.rs`
- `contracts/nft/marketplace-v3/src/execute.rs`
- `contracts/nft/marketplace-v3/src/query.rs`
- `contracts/nft/marketplace-v3/README.md`
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/contract/execute.rs`
- `contracts/core/registry/src/contract/query.rs`
- `contracts/core/registry/src/state.rs`
