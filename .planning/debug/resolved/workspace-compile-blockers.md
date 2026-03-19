---
status: resolved
trigger: "Investigate issue: workspace-compile-blockers\n\n**Summary:** `cargo unit-test` and `cargo check --workspace` are blocked by compile failures in `contracts/core/ecosystem-factory` and `contracts/nft/minter-metadata-onchain`. The user wants both fixed."
created: 2026-03-18T21:00:37.6058556-04:00
updated: 2026-03-18T21:12:30.0000000-04:00
---

## Current Focus

hypothesis: the compile blockers are fixed, human verification confirmed that outcome, and the session can be archived cleanly
test: archive the debug session, preserve the resolution record, and commit only the blocker fix files plus debug bookkeeping
expecting: the session file moves to resolved, knowledge base is updated, and git records only the intended files
next_action: archive the session and create the targeted commits

## Symptoms

expected: `cargo unit-test` and `cargo check --workspace` should complete successfully for the workspace.
actual: workspace verification fails before completion.
errors: `ecosystem-factory` cannot find `REQUESTS_BY_CREATOR` in `src/contract/execute.rs:188` and `src/contract/query.rs:84`; `minter-metadata-onchain` is missing `nft_type` in a `Pg721InstantiateMsg` initializer at `src/contract.rs:102`.
reproduction: run `cargo unit-test` and `cargo check --workspace` from repo root.
started: surfaced during Phase 2 verification on 2026-03-18/2026-03-19; treated as pre-existing out-of-scope blockers at the time.

## Eliminated

## Evidence

- timestamp: 2026-03-18T21:01:55.6562202-04:00
  checked: `.planning/debug/knowledge-base.md`
  found: no knowledge-base file exists yet
  implication: there is no prior resolved-pattern shortcut for this investigation

- timestamp: 2026-03-18T21:01:55.6562202-04:00
  checked: `contracts/core/ecosystem-factory/src/state.rs`, `contracts/core/ecosystem-factory/src/contract.rs`, `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/ecosystem-factory/src/contract/query.rs`
  found: `REQUESTS_BY_CREATOR` is defined in state, used from `execute.rs` and `query.rs` through `super::*`, but omitted from the state re-export list in `contract.rs`
  implication: the ecosystem-factory blocker is a missing module-prelude import, not a missing state definition

- timestamp: 2026-03-18T21:01:55.6562202-04:00
  checked: `contracts/nft/minter-metadata-onchain/src/contract.rs`, `contracts/nft/minter-metadata-onchain/src/msg.rs`, `contracts/nft/pg721-metadata-onchain/src/msg.rs`, `contracts/nft/pg721/src/msg.rs`
  found: `Pg721InstantiateMsg` requires `nft_type`, and `minter-metadata-onchain` receives that same type in `cw721_instantiate_msg` but fails to forward the field when constructing the instantiate message
  implication: the minter blocker is constructor drift caused by a new required field that is already available at the call site

- timestamp: 2026-03-18T21:01:55.6562202-04:00
  checked: `cargo check -p ecosystem-factory`, `cargo check -p minter-metadata-onchain`
  found: both commands failed before compilation because sandboxed `rustc` execution returned `Access is denied. (os error 5)`
  implication: code-level verification requires escalated cargo execution

- timestamp: 2026-03-18T21:02:57.5738839-04:00
  checked: `cargo check -p ecosystem-factory`
  found: the crate now compiles successfully; only an existing unused-import warning remains in `contracts/core/ecosystem-factory/src/contract.rs`
  implication: the missing `REQUESTS_BY_CREATOR` re-export fix addressed the reported ecosystem-factory blocker

- timestamp: 2026-03-18T21:02:57.5738839-04:00
  checked: `cargo check -p minter-metadata-onchain`
  found: the crate now compiles successfully
  implication: forwarding `nft_type` addressed the reported minter-metadata-onchain blocker

- timestamp: 2026-03-18T21:02:57.5738839-04:00
  checked: `cargo check --workspace`
  found: the full workspace check now succeeds; only existing warnings remain in `streaming-billing` and `ecosystem-factory`
  implication: the two original compile blockers no longer block workspace compilation

- timestamp: 2026-03-18T21:02:57.5738839-04:00
  checked: `cargo unit-test`
  found: unit-test initially still failed in `contracts/nft/minter-metadata-onchain/src/contract_tests.rs` because test fixtures omitted the new `nft_type` field, built `CollectionInfo` from the wrong module type, and constructed `Metadata` without newly required fields
  implication: the remaining workspace gate failure was localized to stale minter-metadata-onchain tests, not the production contract code

- timestamp: 2026-03-18T21:07:49.3533859-04:00
  checked: `contracts/nft/minter-metadata-onchain/src/contract_tests.rs`
  found: the stale test fixtures were updated to use `CollectionInfoMsg`, pass `nft_type`, and fill the newer `Metadata` fields
  implication: the minter test suite is aligned with the current `pg721` APIs

- timestamp: 2026-03-18T21:09:04.5411300-04:00
  checked: `cargo unit-test`
  found: the full workspace unit-test gate now succeeds
  implication: both requested verification commands now pass end-to-end

## Resolution

root_cause: `ecosystem-factory` had a stale state re-export list that omitted `REQUESTS_BY_CREATOR`, and `minter-metadata-onchain` had API drift after `pg721` added required `nft_type` and newer message/type shapes; the minter test fixtures had the same drift as the production constructor
fix: added the missing `REQUESTS_BY_CREATOR` re-export in `contracts/core/ecosystem-factory/src/contract.rs`, forwarded `nft_type` into `Pg721InstantiateMsg` in `contracts/nft/minter-metadata-onchain/src/contract.rs`, and updated `contracts/nft/minter-metadata-onchain/src/contract_tests.rs` to the current `pg721_metadata_onchain::msg` types and metadata fields
verification: `cargo check -p ecosystem-factory`, `cargo check -p minter-metadata-onchain`, `cargo check --workspace`, and `cargo unit-test` all pass; user confirmed fixed in the real workflow on 2026-03-18
files_changed: ["contracts/core/ecosystem-factory/src/contract.rs", "contracts/nft/minter-metadata-onchain/src/contract.rs", "contracts/nft/minter-metadata-onchain/src/contract_tests.rs"]
