---
slug: phase-4-validator-metadata-overreach
status: diagnosed
created: 2026-03-19
updated: 2026-03-19
---

# Phase 4 Validator Metadata Overreach

## Summary

Phase 4 overreached its corrected scope by adding validator metadata storage and admin surfaces to `registry` and `pasg-governance` even though the intended architecture was to use the chain/node validator endpoint directly.

## Expected

- PASG staking remains documented as chain-native delegation to Passage validators.
- No validator metadata or allowlist surface is added to `registry`.
- No staking-metadata admin action is added to `pasg-governance`.
- Phase 4 docs, summaries, verification, and requirement tracking reflect the no-adapter stance for validator discovery.

## Actual

- `registry` gained `UpsertStakingValidator`, `RemoveStakingValidator`, `StakingValidator`, and `StakingValidators`.
- `pasg-governance` gained `RegistryUpsertStakingValidator` and `RegistryRemoveStakingValidator` admin actions plus tests.
- `docs/03-json-examples.md`, `docs/04-multisig-governance.md`, and Phase 4 closeout artifacts describe the validator metadata adapter as part of the completed scope.

## Evidence

Relevant commits:
- `821230c` `feat(04-02): align staking reward docs with native flow`
- `4918ddc` `test(04-02): add failing staking metadata guard tests`
- `5b724be` `feat(04-02): add validator metadata adapter`
- `94d4fa2` `docs(04-02): finalize staking plan metadata`
- `fb3d1d7` `docs(04): close validator staking phase`

Relevant files:
- `contracts/core/registry/src/msg.rs`
- `contracts/core/registry/src/contract/execute.rs`
- `contracts/core/registry/src/contract/query.rs`
- `contracts/core/registry/src/state.rs`
- `contracts/core/registry/src/tests/staking.rs`
- `contracts/core/pasg-governance/src/msg.rs`
- `contracts/core/pasg-governance/src/contract.rs`
- `contracts/core/pasg-governance/src/tests/admin_handoff.rs`
- `docs/03-json-examples.md`
- `docs/04-multisig-governance.md`
- `.planning/phases/04-pasg-staking-rewards/04-02-SUMMARY.md`
- `.planning/phases/04-pasg-staking-rewards/04-03-SUMMARY.md`
- `.planning/phases/04-pasg-staking-rewards/04-VERIFICATION.md`

## Root Cause

Phase 4 Wave 2 asked whether any minimal adapter was needed if native staking integration left a gap. Execution interpreted optional validator discovery UX as justification for a registry-backed metadata adapter, even though the intended answer was stricter: validator discovery should come from the blockchain/node endpoint and Phase 4 should not add any repo-local validator surface.

## Correct Fix Direction

1. Revert validator metadata code from `registry`.
2. Revert validator metadata admin actions from `pasg-governance`.
3. Remove validator metadata examples from docs.
4. Update Phase 4 summaries and verification to describe a docs-only / chain-native result with no validator adapter.
5. Keep the chain-native staking boundary and the completed STAK requirements, but satisfy them through documentation and existing chain endpoints rather than new contract surfaces.

## Recommended Next Step

Apply the fix now and then archive this debug session as resolved.
