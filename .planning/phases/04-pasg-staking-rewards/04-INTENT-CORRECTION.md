# Phase 4 Intent Correction

**Date:** 2026-03-19
**Source:** `.planning/debug/phase-4-staking-intent-mismatch.md`, `../context/whitepaper-tokenomics.html`

## Correction

Phase 4 must model PASG staking as chain-native delegation to one or more Passage validators.

It must not default to a standalone CosmWasm staking vault with local unbond queues, emission schedules, or reward-account accounting.

## Required Invariants

- Use native validator delegation as the source of truth for PASG stake, undelegation, and rewards.
- Treat existing `contracts/staking/*` crates as NFT staking primitives unless a later scoped adapter need is proven.
- Describe staking economics in terms of validator fee participation and PASG token rewards, not a contract-local emission program invented by planning docs.
- Any Phase 4 contract work must be minimal and justified by a real gap in the native staking flow.

## Planning Effect

The prior Phase 4 research and requirement wording were based on the wrong architectural premise and are superseded.

Next action: plan Phase 4 against this corrected validator-delegation intent before implementation starts.