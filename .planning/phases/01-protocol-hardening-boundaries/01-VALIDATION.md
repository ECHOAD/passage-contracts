---
phase: 01
slug: protocol-hardening-boundaries
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-17
---

# Phase 01 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` + `cw-multi-test 2.1.1` |
| **Config file** | `.cargo/config.toml` |
| **Quick run command** | `cargo test -p streaming-billing --lib` |
| **Full suite command** | `cargo unit-test` |
| **Estimated runtime** | ~60-180 seconds |

---

## Sampling Rate

- **After every task commit:** Run the narrowest affected package test command
- **After every plan wave:** Run `cargo unit-test`
- **Before `$gsd-verify-work`:** Full suite must be green plus `cargo check --workspace`
- **Max feedback latency:** 180 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | ARCH-01 | integration | `cargo test -p streaming-billing --lib` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | REV-03 | integration | `cargo test -p streaming-billing --lib` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | ARCH-02 | integration | `cargo test -p marketplace-v3 --lib` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 1 | ARCH-02 | integration | `cargo test -p registry --lib --tests` | ✅ partial | ⬜ pending |
| 01-02-03 | 02 | 1 | ARCH-02 | integration | `cargo test -p collection-factory --lib` and `cargo test -p ecosystem-factory --lib` | ❌ W0 | ⬜ pending |
| 01-03-01 | 03 | 2 | ARCH-03 | smoke | `cargo check --workspace` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `contracts/core/streaming-billing/src/tests.rs` or equivalent test module wiring
- [ ] `contracts/nft/marketplace-v3/src/contract/query/tests.rs`
- [ ] `contracts/core/collection-factory/src/contract/reply/tests.rs`
- [ ] `contracts/core/ecosystem-factory/src/contract/reply/tests.rs`
- [ ] Registry query coverage for authorized-minter pagination if existing tests cannot absorb the fix

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Review protocol boundary and safety matrix against product intent | ARCH-03 / REV-03 | The artifact is documentation plus code alignment, so a human should confirm the published boundary still matches Passage business intent | Compare the final boundary/safety docs against `ONCHAIN_OFFCHAIN_BOUNDARIES.md` and `FIAT_TO_CRYPTO_PAYMENT_FLOW.md`; confirm no platform subscription logic migrated on-chain |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 180s for narrow package loops
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending