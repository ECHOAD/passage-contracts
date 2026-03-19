# GSD Debug Knowledge Base

Resolved debug sessions. Used by `gsd-debugger` to surface known-pattern hypotheses at the start of new investigations.

---

## workspace-compile-blockers - workspace cargo gates blocked by stale re-exports and pg721 API drift
- **Date:** 2026-03-18
- **Error patterns:** cargo unit-test, cargo check --workspace, ecosystem-factory, REQUESTS_BY_CREATOR, minter-metadata-onchain, Pg721InstantiateMsg, nft_type
- **Root cause:** `ecosystem-factory` had a stale state re-export list that omitted `REQUESTS_BY_CREATOR`, and `minter-metadata-onchain` had API drift after `pg721` added required `nft_type` and newer message/type shapes; the minter test fixtures had the same drift as the production constructor
- **Fix:** added the missing `REQUESTS_BY_CREATOR` re-export in `contracts/core/ecosystem-factory/src/contract.rs`, forwarded `nft_type` into `Pg721InstantiateMsg` in `contracts/nft/minter-metadata-onchain/src/contract.rs`, and updated `contracts/nft/minter-metadata-onchain/src/contract_tests.rs` to the current `pg721_metadata_onchain::msg` types and metadata fields
- **Files changed:** contracts/core/ecosystem-factory/src/contract.rs, contracts/nft/minter-metadata-onchain/src/contract.rs, contracts/nft/minter-metadata-onchain/src/contract_tests.rs
---

## phase-3-multisig-intent-mismatch - phase 3 replaced the admin multisig with PASG-holder governance
- **Date:** 2026-03-18
- **Error patterns:** phase 3, multisig, intent mismatch, PASG-holder governance, admin-owner control plane, architectural mismatch
- **Root cause:** Phase 3 planning and implementation treated the existing `contracts/core/multisig` admin-owner contract as the place to build PASG-holder governance, even though the repo already depended on that contract as the stable admin/control plane for `registry` and other protocol contracts.
- **Fix:** Reverted the Phase 3 `multisig` reinterpretation, restored the pre-Phase-3 admin multisig code and docs, reopened Phase 3 in roadmap/state/requirements terms, and corrected the Phase 3 planning artifacts so they explicitly preserve `multisig` and require PASG governance to be replanned as a separate layer.
- **Files changed:** .planning/PROJECT.md, .planning/ROADMAP.md, .planning/STATE.md, .planning/phases/03-pasg-governance/03-01-PLAN.md, .planning/phases/03-pasg-governance/03-02-PLAN.md, .planning/phases/03-pasg-governance/03-03-PLAN.md, .planning/phases/03-pasg-governance/03-INTENT-CORRECTION.md, .planning/phases/03-pasg-governance/03-RESEARCH.md, .planning/phases/03-pasg-governance/03-VALIDATION.md, CLAUDE.md, README.md, contracts/core/multisig/README.md, contracts/core/multisig/src/contract.rs, contracts/core/multisig/src/error.rs, contracts/core/multisig/src/msg.rs, contracts/core/multisig/src/state.rs, contracts/core/multisig/src/tests/governance.rs, docs/04-multisig-governance.md
---
