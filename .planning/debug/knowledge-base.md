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

