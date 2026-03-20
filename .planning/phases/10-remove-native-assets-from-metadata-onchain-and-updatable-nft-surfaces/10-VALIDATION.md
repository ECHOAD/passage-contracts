---
phase: 10
slug: remove-native-assets-from-metadata-onchain-and-updatable-nft-surfaces
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-19
---

# Phase 10 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test |
| **Config file** | none - existing Rust workspace |
| **Quick run command** | `cargo test -p minter-v2-metadata-onchain --lib` |
| **Full suite command** | `cargo test -p pg721-updatable --lib && cargo test -p minter-v2-metadata-onchain --lib && cargo run --example schema -p pg721-updatable && cargo run --example schema -p pg721-metadata-onchain && cargo run --example schema -p minter-v2-metadata-onchain` |
| **Estimated runtime** | ~90 seconds |

## Sampling Rate

- **After every task commit:** Run `cargo test -p minter-v2-metadata-onchain --lib`
- **After every plan wave:** Run the package-specific tests for files touched in that wave plus the relevant schema command
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 90 seconds

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 10-01-01 | 01 | 1 | NFT-02 | regression | `cargo test -p pg721-updatable --lib` | existing | pending |
| 10-02-01 | 02 | 2 | NFT-02, QUAL-01 | unit | `cargo test -p minter-v2-metadata-onchain --lib` | existing | pending |
| 10-03-01 | 03 | 3 | QUAL-01 | schema/regression | `cargo run --example schema -p pg721-updatable && cargo run --example schema -p pg721-metadata-onchain && cargo run --example schema -p minter-v2-metadata-onchain` | existing | pending |

## Wave 0 Requirements

Existing Rust test and schema infrastructure covers this phase.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Bilingual and legacy/current docs no longer claim `native_assets` support on active NFT surfaces | QUAL-01 | Public docs can drift after schema changes | `rg -n "native_assets|native_asset_template|TokenNativeAssets|SetNativeAssetTemplate" docs contracts/nft -g "*.md"` should only match historical or migration notes intentionally retained |

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 90s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-03-19
