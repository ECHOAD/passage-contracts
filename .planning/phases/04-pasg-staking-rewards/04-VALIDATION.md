# Phase 4: PASG Validator Staking & Rewards - Validation Strategy

**Defined:** 2026-03-19
**Updated:** 2026-03-19 after staking-intent correction
**Scope:** validation guardrails for the native validator staking model
**Status:** ready-to-plan

## Validation Guardrails

The Phase 4 plan must satisfy these guardrails before any implementation work starts:

- No task may introduce a default `pasg-staking` CosmWasm vault that keeps its own stake ledger, unbond queue, or reward accumulator for validator delegation.
- The chain-native staking module and Passage validator set remain the source of truth for PASG delegation, undelegation, redelegation, and reward state.
- Existing `contracts/staking/*` crates must stay treated as NFT staking primitives unless a later task proves a narrow adapter need with no duplicate stake ledger.
- Any contract work in this phase must be minimal, justified by a real integration gap, and documented as an adapter or observability surface around the native flow.
- Docs and examples must explicitly separate chain-native PASG staking from the repo's NFT staking contracts.

## Wave 0 Checks For Plan

- The replanned artifact set does not assume a `contracts/staking/pasg-*` implementation by default.
- The task map identifies the native chain message/query surfaces or marks them as explicit dependencies if they live outside this repo.
- Reward accounting is described in terms of validator fee participation and chain/token-program rewards rather than a new in-repo emission schedule.
- If any adapter work is proposed, it is bounded to metadata, query normalization, or operational guidance and does not duplicate chain staking balances.

## Automated Verification To Keep

- `cargo check --workspace`
- `cargo unit-test`

These commands remain the broad regression floor for any Phase 4 adapter or documentation work, but the primary validation for this phase is architectural correctness against the native staking model.
