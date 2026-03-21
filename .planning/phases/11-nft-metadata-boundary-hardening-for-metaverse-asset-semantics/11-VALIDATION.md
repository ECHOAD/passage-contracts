---
phase: 11
slug: 11-nft-metadata-boundary-hardening-for-metaverse-asset-semantics
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-20
---

# Phase 11 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust cargo test |
| **Config file** | `.cargo/config.toml` |
| **Quick run command** | `cargo check -p pg721` |
| **Full suite command** | `cargo test -p pg721 --lib && cargo test -p pg721-updatable --lib && cargo test -p asset-progression --lib && cargo test -p world-plugin-assignment --lib` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run the plan-local crate command first (`cargo check -p pg721`, `cargo check -p asset-progression`, or `cargo check -p world-plugin-assignment`)
- **After every plan wave:** Run `cargo test -p pg721 --lib && cargo test -p pg721-updatable --lib && cargo test -p asset-progression --lib && cargo test -p world-plugin-assignment --lib`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 11-01-01 | 01 | 1 | NFT-02, NFT-03, REV-01, ARCH-01 | unit | `cargo test -p pg721 --lib && cargo test -p pg721-updatable --lib` | ? | ? pending |
| 11-02-01 | 02 | 2 | NFT-02, NFT-03, ARCH-01, QUAL-01 | unit/integration | `cargo test -p avatar-progression --lib && cargo check -p avatar-progression` | ? | ? pending |
| 11-03-01 | 03 | 3 | NFT-03, REV-01, ARCH-01, QUAL-01 | unit/integration | `cargo test -p world-plugin-assignment --lib && cargo check -p world-plugin-assignment` | ? | ? pending |
| 11-04-01 | 04 | 4 | NFT-02, NFT-03 | schema/doc-smoke | `cargo run --example schema -p pg721 && cargo run --example schema -p pg721-updatable && cargo run --example schema -p asset-progression && cargo run --example schema -p world-plugin-assignment` | 2026-03-20 | passed |

*Status: ? pending � ? green � ? red � ?? flaky*

---

## Wave 0 Requirements

- [x] Existing Rust test infrastructure already covers crate-level verification.
- [x] No framework install step is needed.
- [x] Phase 11 plans include automated verification commands for every task.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Passage runtime UX presents NFT plus state-module data as one composed asset | NFT-03 | UI composition is outside contract test scope | Confirm docs/examples describe the composed presentation model and no contract metadata field claims to carry live UI state |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all missing references
- [x] No watch-mode flags
- [x] Feedback latency < 30s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-03-20

