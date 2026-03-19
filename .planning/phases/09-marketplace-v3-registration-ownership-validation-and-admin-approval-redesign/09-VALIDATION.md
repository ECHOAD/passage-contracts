---
phase: 09
slug: marketplace-v3-registration-ownership-validation-and-admin-approval-redesign
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-19
---

# Phase 09 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test |
| **Config file** | none - existing Rust workspace |
| **Quick run command** | `cargo test -p marketplace-v3 --lib` |
| **Full suite command** | `cargo test -p marketplace-v3 --lib && cargo check -p marketplace-v3` |
| **Estimated runtime** | ~40 seconds |

## Sampling Rate

- **After every task commit:** Run `cargo test -p marketplace-v3 --lib`
- **After every plan wave:** Run `cargo test -p marketplace-v3 --lib && cargo check -p marketplace-v3`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 40 seconds

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 09-01-01 | 01 | 1 | NFT-01 | unit | `cargo test -p marketplace-v3 --lib config` | existing | pending |
| 09-02-01 | 02 | 2 | NFT-01 | unit | `cargo test -p marketplace-v3 --lib registration` | existing | pending |
| 09-03-01 | 03 | 3 | QUAL-01 | regression | `cargo test -p marketplace-v3 --lib` | existing | pending |

## Wave 0 Requirements

Existing infrastructure covers all phase requirements.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Request and approval JSON examples match final schema naming | QUAL-01 | Public docs and examples can drift even when unit tests pass | Compare `docs/03-json-examples.md` against generated schema and execute/query enums before closeout |

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 40s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-03-19
