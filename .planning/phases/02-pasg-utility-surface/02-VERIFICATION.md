---
phase: 02
slug: pasg-utility-surface
status: passed
verified: 2026-03-18
requirements:
  - PASG-01
  - PASG-02
  - PASG-03
---

# Phase 02 Verification

## Goal

Turn PASG from an assumed denom into an explicit protocol utility layer that contracts and services can integrate consistently.

## Requirement Coverage

| Requirement | Status | Evidence |
|---|---|---|
| PASG-01 | Passed | `streaming-billing` enforces native `upasg` PASG treatment, `split-router` exposes the compatibility routing hook, and the fee-bearing consumer flows in `marketplace-v3`, `minter-v2`, and `auction-english` emit canonical PASG reference attributes. |
| PASG-02 | Passed | `contracts/core/streaming-billing/src/msg.rs` remains the canonical PASG interface, `contracts/core/split-router/schema/*.json` now expose the generic routing metadata/query surface, and the rewritten contract READMEs plus `docs/03-json-examples.md` map integrators to concrete query and execute names instead of local assumptions. |
| PASG-03 | Passed | The repo-level PASG narrative is now consistently native-`upasg`-first across `README.md`, `CLAUDE.md`, `contracts/core/streaming-billing/README.md`, `contracts/core/split-router/README.md`, `contracts/nft/marketplace-v3/README.md`, `contracts/nft/minter-v2/README.md`, `contracts/nft/auction-english/README.md`, and `docs/03-json-examples.md`. |

## Automated Verification

- `rg -n "native \`upasg\`|canonical PASG|adapter|compatibility-only|service-invoked|fee treatment|query" README.md CLAUDE.md docs/03-json-examples.md contracts/core/streaming-billing/README.md contracts/core/split-router/README.md contracts/nft/marketplace-v3/README.md contracts/nft/minter-v2/README.md contracts/nft/auction-english/README.md` - passed
- `cargo test -p split-router --lib` - passed
- `cargo test -p marketplace-v3 --lib` - passed
- `cargo test -p minter-v2 --lib --tests` - passed
- `cargo test -p auction-english --lib` - passed
- `cargo check -p streaming-billing` - passed
- `cargo unit-test` - failed outside Phase 2 ownership in `ecosystem-factory` because `REQUESTS_BY_CREATOR` is missing in `src/contract/execute.rs:188` and `src/contract/query.rs:84`
- `cargo check --workspace` - failed outside Phase 2 ownership in `ecosystem-factory` (same missing import) and `minter-metadata-onchain` (`Pg721InstantiateMsg` initializer missing `nft_type` in `src/contract.rs:102`)

## Manual Review Notes

- The docs now consistently present PASG as a protocol utility instrument inside Passage rather than a hidden in-repo token contract.
- Consumer docs and examples point readers to queryable surfaces first, then to execute-response attributes when the PASG check happens after settlement.
- No platform subscription, premium-tier, or other off-chain business logic was moved on-chain during Phase 2.

## Verdict

Phase 02 passed. The PASG utility story is now explicit and consistent across code, docs, examples, and schema outputs. Full repo-wide green verification remains blocked by the previously logged `ecosystem-factory` and `minter-metadata-onchain` defects outside Phase 2 scope.
