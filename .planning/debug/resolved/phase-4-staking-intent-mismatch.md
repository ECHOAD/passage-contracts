---
status: awaiting_human_verify
trigger: "Investigate issue: phase-4-staking-intent-mismatch"
created: 2026-03-19T19:25:43.6573997Z
updated: 2026-03-19T19:34:32.1481373Z
---

## Current Focus

hypothesis: Phase 4 planning drifted because existing `contracts/staking/*` primitives were interpreted as the intended PASG staking product, even though the clarified whitepaper intent is native validator delegation at the chain level.
test: Human-verify that the corrected Phase 4 planning docs now match the intended PASG staking model before any new Phase 4 plan is written.
expecting: The user should see validator delegation, validator fee participation, PASG rewards, and chain-native/adaptor surfaces as the Phase 4 default, with contract-vault language only appearing as explicit non-goals or contrast.
next_action: Wait for user confirmation that the updated planning docs reflect the intended whitepaper staking model.

## Symptoms

expected: Phase 4 should reflect PASG holders staking and delegating to one or more Passage validators at the chain level, with validator fee participation and token rewards, not a separate CosmWasm staking vault by default.
actual: `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, and derived research read Phase 4 as a protocol staking contract with 21-day unbonding, emission logic, fee-backed rewards, and contract-local query/accounting.
errors: No runtime error. This is an architecture/intent mismatch between project planning docs and clarified product intent from the PASG whitepaper.
reproduction: Read Phase 4 in `.planning/ROADMAP.md` and `STAK-01/02/03` in `.planning/REQUIREMENTS.md`; compare with user-provided whitepaper statement: "PASG holders can stake and delegate their tokens to one or more Passage validators, who will receive a percentage of fees from staked tokens delegated to them. PASG holders in turn will receive additional PASG token rewards for contributing to the network."
started: The mismatch surfaced during Phase 4 pre-research on 2026-03-19, before planning/execution. Previous research inferred a protocol staking contract from current roadmap wording and existing `contracts/staking/*` references.

## Eliminated

## Evidence

- timestamp: 2026-03-19T19:25:43.6573997Z
  checked: .planning/debug/knowledge-base.md
  found: Knowledge base match on [phase, intent mismatch, architectural mismatch] -> `phase-3-multisig-intent-mismatch`, which corrected a prior roadmap/research misread caused by treating an existing contract family as the intended product design.
  implication: Phase 4 likely has the same class of planning error and should be corrected in planning artifacts before implementation starts.
- timestamp: 2026-03-19T19:28:38.2937186Z
  checked: .planning/ROADMAP.md, .planning/REQUIREMENTS.md, .planning/PROJECT.md, .planning/phases/04-pasg-staking-rewards/04-RESEARCH.md
  found: Phase 4 is currently specified as PASG staking with 21-day unbonding, emission logic, fee-backed reward transition, contract-local reward accounting, and a new `pasg-staking` contract shape under `contracts/staking/*`.
  implication: The active planning artifacts encode a CosmWasm staking-vault design rather than chain-native validator delegation.
- timestamp: 2026-03-19T19:28:38.2937186Z
  checked: contracts/staking/nft-vault/src/contract.rs, contracts/staking/stake-rewards/src/contract.rs, contracts/staking/vault-factory/src/contract.rs
  found: The existing staking crates are NFT-vault and reward-account contracts with local stake balances, claim queues, instantiate2 deployment, and contract-driven reward accounting.
  implication: Reusing these crates as the Phase 4 target naturally biases planning toward a vault-based PASG staking contract.
- timestamp: 2026-03-19T19:28:38.2937186Z
  checked: ..\context\whitepaper-tokenomics.html
  found: The local whitepaper states that PASG holders "can stake and delegate their tokens to one or more Passage validators," validators receive a percentage of fees from delegated stake, and holders receive additional PASG token rewards.
  implication: The intended product is native validator delegation and validator-linked economics, not a default standalone CosmWasm staking vault.
- timestamp: 2026-03-19T19:33:17.1029811Z
  checked: corrected planning docs after first rewrite
  found: The main intent correction landed, but `PROJECT.md` missed the new decision row, `STATE.md` still had stale project/session reference metadata, and `ROADMAP.md` lost a blank line after the rewritten Phase 4 section.
  implication: A second cleanup pass is needed before verification can be considered complete.
- timestamp: 2026-03-19T19:34:32.1481373Z
  checked: .planning/ROADMAP.md, .planning/REQUIREMENTS.md, .planning/PROJECT.md, .planning/STATE.md, .planning/phases/04-pasg-staking-rewards/04-RESEARCH.md, .planning/phases/04-pasg-staking-rewards/04-INTENT-CORRECTION.md
  found: Phase 4 now reads as native validator delegation across roadmap, requirements, project, state, and research docs; remaining staking-vault terms appear only in explicit non-goal or correction language. A git status check was blocked by repository safe-directory ownership, but the file readback confirmed the doc changes.
  implication: The planning-doc fix is self-verified and ready for human confirmation.

## Resolution

root_cause: Phase 4 planning and research anchored on the existing `contracts/staking/*` NFT staking primitives and translated PASG staking into a new contract-local staking vault with unbond queues, emissions, and reward-account accounting, even though the local PASG whitepaper defines staking as chain-native delegation to Passage validators.
fix: Corrected Phase 4 planning artifacts to describe PASG validator delegation, validator fee participation, PASG rewards, and chain-native or documented adapter surfaces; added an explicit Phase 4 intent-correction note and rewrote the Phase 4 research away from a default `pasg-staking` vault design.
verification: Re-read every corrected Phase 4 planning artifact and searched for leftover vault-specific phrasing; the remaining matches are intentional non-goal/correction statements, not active phase requirements. No code or runtime tests were needed because the fix is documentation-only.
files_changed: [.planning/ROADMAP.md, .planning/REQUIREMENTS.md, .planning/PROJECT.md, .planning/STATE.md, .planning/phases/04-pasg-staking-rewards/04-RESEARCH.md, .planning/phases/04-pasg-staking-rewards/04-INTENT-CORRECTION.md]
