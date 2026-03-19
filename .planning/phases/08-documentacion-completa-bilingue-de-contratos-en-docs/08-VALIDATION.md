---
phase: 08
slug: documentacion-completa-bilingue-de-contratos-en-docs
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-03-19
---

# Phase 08 - Validation Strategy

> Per-phase validation contract for bilingual contract documentation coverage.

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `rg` / file coverage checks |
| **Config file** | none |
| **Quick run command** | `rg -n "^# |^## " docs/es docs/en` |
| **Full suite command** | `rg -n "^# |^## " docs/es docs/en && rg -n "historical|reference|hist[oó]rica|referencia" docs/es docs/en` |
| **Estimated runtime** | ~10 seconds |

## Sampling Rate

- **After every task commit:** run the targeted `rg` check for the pages touched in that task
- **After every plan wave:** run the full bilingual structure check across `docs/es` and `docs/en`
- **Before `$gsd-verify-work`:** confirm every contract crate has a mirrored page in both languages
- **Max feedback latency:** 10 seconds

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 08-01-01 | 01 | 1 | DOC-02 | docs | `rg -n "overview|relationships|instantiate|flows" docs/es docs/en` | pending | pending |
| 08-02-01 | 02 | 2 | DOC-01 | coverage | `rg -n "collection-factory|registry|multisig|streaming-billing" docs/es/contracts/core docs/en/contracts/core` | pending | pending |
| 08-03-01 | 03 | 3 | DOC-01 | coverage | `rg -n "marketplace-v3|pg721|minter-v2|auction-english" docs/es/contracts/nft docs/en/contracts/nft` | pending | pending |
| 08-04-01 | 04 | 4 | DOC-03 | docs | `rg -n "historical|reference|hist[oó]rica|referencia" docs/es docs/en` | pending | pending |

## Wave 0 Requirements

Existing repo docs and contract-local READMEs provide enough source material to proceed without creating prerequisite code artifacts.

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| English and Spanish trees tell the same structural story | DOC-01, DOC-02 | A grep can prove presence, not translation parity | Compare family indexes and one sample contract page per family in both languages before closeout |
| Hypothetical examples remain explanatory and do not invent unsupported protocol behavior | DOC-02 | Requires judgment against roadmap/context decisions | Read `flows.md` and `instantiate-guides.md` in both languages and compare against `08-CONTEXT.md` |

## Validation Sign-Off

- [x] All tasks have deterministic documentation checks
- [x] Sampling continuity is maintained across all waves
- [x] Wave 0 dependencies are satisfied by existing repo docs and schemas
- [x] No watch-mode or interactive commands required
- [x] Feedback latency < 10s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-03-19
