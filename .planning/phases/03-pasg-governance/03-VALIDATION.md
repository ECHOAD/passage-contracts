# Phase 3: PASG Governance - Validation Strategy

**Defined:** 2026-03-18
**Scope:** task-level verification for the three Phase 3 plans
**Status:** draft
**nyquist_compliant:** false

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` + `cw-multi-test 2.1.1` |
| **Config file** | `.cargo/config.toml` |
| **Quick run command** | `cargo test -p multisig --lib --tests` |
| **Full suite command** | `cargo unit-test` |
| **Estimated runtime** | ~60-180 seconds |

## Wave Graph

- Plan 01 tasks -> wave 1
- Plan 02 tasks -> wave 2
- Plan 03 tasks -> wave 3

## Sampling Rate

- After every task commit: run the narrowest affected governance command first.
- After every plan wave: run `cargo test -p multisig --lib --tests`.
- Before phase sign-off: run `cargo unit-test` and `cargo check --workspace`.
- Max feedback latency: 180 seconds.

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | GOV-01, GOV-03 | integration | `cargo test -p multisig --lib --tests` | partial | pending |
| 03-01-02 | 01 | 1 | GOV-01, GOV-03 | docs/query | `cargo check -p multisig` | W0 | pending |
| 03-02-01 | 02 | 2 | GOV-02 | integration | `cargo test -p multisig --lib --tests` | W0 | pending |
| 03-02-02 | 02 | 2 | GOV-02 | integration + negative | `cargo test -p multisig --lib --tests` | W0 | pending |
| 03-03-01 | 03 | 3 | GOV-03 | integration + auth | `cargo test -p multisig --lib --tests` | W0 | pending |
| 03-03-02 | 03 | 3 | GOV-01, GOV-03 | docs | `rg -n "PASG|governance|delegat|quorum|proposal|scope|upasg" docs/04-multisig-governance.md contracts/core/multisig/README.md README.md CLAUDE.md` | partial | pending |
| 03-03-03 | 03 | 3 | GOV-01, GOV-02, GOV-03 | smoke | `cargo unit-test`; `cargo check --workspace` | partial | pending |

## Wave 0 Requirements

- `contracts/core/multisig/src/tests/governance.rs` must add PASG deposit, proposal snapshot, delegation, quorum, approval, and rejected-action coverage.
- `contracts/core/multisig/src/msg.rs` and `contracts/core/multisig/src/state.rs` must expose queryable governance config, voting-power state, delegation state, and proposal snapshot surfaces.
- `contracts/core/multisig/src/contract.rs` must reject out-of-scope executable actions before they can be used as passed PASG governance.
- Governance docs must distinguish legacy admin multisig usage from PASG-holder governance semantics.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Confirm the final governance scope stays limited to PASG and protocol actions and does not drift into off-chain platform operations | GOV-03 | The code can reject many message families, but a reviewer still needs to judge the product and architecture boundary of the documented scope | Review `docs/04-multisig-governance.md`, `contracts/core/multisig/README.md`, `README.md`, and `CLAUDE.md`; confirm no text claims governance over subscriptions, streaming operations, analytics, search, or other off-chain systems |

## Validation Sign-Off

- All tasks have an automated verify or Wave 0 dependency.
- Sampling continuity: no 3 consecutive tasks without automated verify.
- Wave 0 covers all missing references.
- No watch-mode flags.
- Feedback latency stays under 180 seconds.
- `nyquist_compliant: true` is set before closure.

**Approval:** pending
