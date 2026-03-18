# Phase 2: PASG Utility Surface - Validation Strategy

**Defined:** 2026-03-18
**Scope:** task-level verification for the three Phase 2 plans
**Status:** draft
**nyquist_compliant:** false

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` + `cw-multi-test 2.1.1` |
| **Config file** | `.cargo/config.toml` |
| **Quick run command** | `cargo test -p streaming-billing --lib` |
| **Full suite command** | `cargo unit-test` |
| **Estimated runtime** | ~60-180 seconds |

## Wave Graph

- Plan 01 tasks -> wave 1
- Plan 02 tasks -> wave 2
- Plan 03 tasks -> wave 3

## Sampling Rate

- After every task commit: run the narrowest affected package test command.
- After every plan wave: run `cargo unit-test`.
- Before phase sign-off: run `cargo check --workspace`.
- Max feedback latency: 180 seconds.

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | PASG-02, PASG-03 | docs | `cargo test -p streaming-billing --lib` | partial | pending |
| 02-01-02 | 01 | 1 | PASG-02, PASG-03 | docs | `cargo test -p split-router --lib` | partial | pending |
| 02-02-01 | 02 | 2 | PASG-01, PASG-02 | integration | `cargo test -p streaming-billing --lib` | partial | pending |
| 02-02-02 | 02 | 2 | PASG-01, PASG-02 | integration | `cargo test -p split-router --lib` | W0 | pending |
| 02-02-03 | 02 | 2 | PASG-01, PASG-02 | integration | `cargo test -p marketplace-v3 --lib`; `cargo test -p minter-v2 --lib --tests`; `cargo test -p auction-english --lib` | W0 | pending |
| 02-03-01 | 03 | 3 | PASG-03, PASG-02 | docs | `rg -n "canonical PASG|native \`upasg\`|adapter|compatibility-only|service-invoked|fee treatment|query" README.md CLAUDE.md contracts/core/streaming-billing/README.md contracts/core/split-router/README.md contracts/nft/marketplace-v3/README.md contracts/nft/minter-v2/README.md contracts/nft/auction-english/README.md` | partial | pending |
| 02-03-02 | 03 | 3 | PASG-03, PASG-01 | integration | `cargo test -p split-router --lib`; `cargo test -p marketplace-v3 --lib`; `cargo test -p minter-v2 --lib --tests`; `cargo test -p auction-english --lib`; `cargo check -p streaming-billing` | partial | pending |
| 02-03-03 | 03 | 3 | PASG-03, PASG-01, PASG-02 | smoke | `rg -n "native \`upasg\`|canonical PASG|adapter|compatibility-only|service-invoked|fee treatment|query" README.md CLAUDE.md docs/03-json-examples.md contracts/core/streaming-billing/README.md contracts/core/split-router/README.md contracts/nft/marketplace-v3/README.md contracts/nft/minter-v2/README.md contracts/nft/auction-english/README.md`; `cargo unit-test`; `cargo check --workspace` | partial | pending |

Status legend: `partial`, `W0`, `pending`, `done`.

## Wave 0 Requirements

- `contracts/core/streaming-billing/src/tests.rs` must cover PASG-denom deposit and withdraw paths, conversion-rate behavior, and pending revenue settlement.
- `contracts/core/streaming-billing/src/contract.rs` must cover `DepositCrypto`, `WithdrawPoints`, and `query_conversion_rate`.
- `contracts/core/split-router/src/contract/execute/tests.rs` must cover PASG-aware routing assumptions, zero-funds rejection, and malformed recipient handling.
- `contracts/core/split-router/src/contract/query/tests.rs` must cover preview math and event pagination.
- Shared fixture wiring in `cw-multi-test` helpers must provide a native `upasg` bank balance and any registry or mock setup needed by `streaming-billing`.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Confirm the canonical PASG utility story still reads as native-denom-first in docs and messages | PASG-03 | The code can prove native coin behavior, but the product-facing interpretation of `upasg` versus any wrapper or adapter needs a human sign-off | Review `README.md`, `CLAUDE.md`, `contracts/core/streaming-billing/README.md`, and `contracts/core/split-router/README.md`; confirm the final surface does not imply a CW20-style PASG contract |

## Validation Sign-Off

- All tasks have an automated verify or Wave 0 dependency.
- Sampling continuity: no 3 consecutive tasks without automated verify.
- Wave 0 covers all missing references.
- No watch-mode flags.
- Feedback latency stays under 180 seconds.
- `nyquist_compliant: true` is set before closure.

**Approval:** pending