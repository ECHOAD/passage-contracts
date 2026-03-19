# Phase 04 Research: PASG Validator Staking & Rewards

## Scope

Phase 4 should align PASG staking to the chain-native validator delegation model described in `../context/whitepaper-tokenomics.html`. PASG holders delegate native `upasg` to one or more Passage validators, validators receive a percentage of fees from delegated stake, and holders receive additional PASG rewards for participating in network security.

Any 21-day unbonding expectation should be treated as a chain staking rule or validator-program rule, not as a default CosmWasm claim queue.

## Intent Source

The local whitepaper is explicit about staking intent:
- PASG holders delegate to one or more Passage validators.
- Validators receive a percentage of fees from delegated stake.
- Delegators receive additional PASG token rewards.

Implication: Phase 4 is about native staking/delegation surfaces and validator economics, not inventing a parallel staking vault.

## Repo Reality

This repo already contains staking-domain contracts, but they are not the PASG target architecture:
- `contracts/staking/nft-vault` tracks NFT stakes and contract-local unbond claims.
- `contracts/staking/stake-rewards` tracks contract-local cumulative rewards.
- `contracts/staking/vault-factory` deploys vault instances.

These crates are useful brownfield context only. They must not define the default PASG staking shape.

## Research Priorities

### 1. Chain staking surface

Identify the native delegation, undelegation, redelegation, validator-query, and reward-withdrawal surfaces available on the Passage chain for `upasg`.

### 2. Validator economics

Identify how validator fee participation works for delegated PASG and how additional PASG rewards are funded, accrued, and claimed.

### 3. Required contract surface

Determine whether Phase 4 needs any CosmWasm contract at all. Prefer no new contract unless a thin adapter, registry, or policy surface is required around the native staking flow.

### 4. Query and observability model

Define how wallets, services, and tests read delegation positions, validator assignments, undelegation entries, and reward state without duplicating a staking ledger in CosmWasm storage.

### 5. Governance touchpoints

Clarify which staking parameters are chain-governed, validator-program governed, or PASG-governance governed.

## Default Architecture Direction

- Source of truth: chain-native staking module and validator set
- User action: delegate, undelegate, or redelegate PASG to one or more Passage validators
- Rewards: validator fee share plus PASG rewards as defined by the chain/token program
- Contract work: only minimal adapters, metadata, or policy surfaces if the native flow leaves a real gap
- Non-goal: a standalone `pasg-staking` vault that keeps balances, unbond claims, and reward accumulators in CosmWasm storage

## Do Not Assume

- a new `pasg-staking` contract under `contracts/staking/*`
- contract-local unbond queues for the default flow
- reward-per-token emission math as the primary PASG staking engine
- instantiate2 vault factories or reward-account modules for validator delegation
- a fee-backed reward transition unless the validator/token program actually requires that mechanism

## Brownfield Reuse Rules

- Reuse `contracts/staking/*` only if a later scoped feature explicitly needs NFT staking or a separate incentive module.
- Do not let existing crate shapes dictate the PASG staking product.
- If a helper contract is eventually needed, it should wrap or observe native delegation rather than replace it.

## Recommended Phase Shape

- 04-01: map native validator delegation, unbonding, and reward surfaces for PASG
- 04-02: define validator fee participation, PASG reward handling, and any required adapter/query interfaces
- 04-03: document operational flow, governance dependencies, and any minimal implementation required by the native model

## Open Questions

- Which Passage chain module and message/query surfaces expose validator delegation today?
- Is the unbonding period fixed at 21 days on-chain or configured elsewhere?
- How are validator fee shares and PASG reward claims separated or combined for delegators?
- What, if anything, must be added in CosmWasm to make this flow usable by Passage services and wallets?

## Recommendation

Replan Phase 4 around native validator delegation first. Treat existing `contracts/staking/*` as non-target brownfield context unless later evidence proves a thin contract adapter is required.