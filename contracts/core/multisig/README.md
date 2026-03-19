# Passage Multisig

`multisig` is the PASG-holder governance contract for Passage protocol parameters.

It no longer models a fixed signer list. Governance power is derived from deposited native `upasg`, can be delegated one hop, and is consumed through proposal snapshots, quorum, and approval rules enforced on-chain.

## Governance Scope

`multisig` is intentionally limited to PASG and protocol-facing actions.

It can:
- update its own governance config
- add or remove allowlisted contract targets for governance execution
- execute zero-fund `WasmMsg::Execute` calls against allowlisted protocol contracts

It cannot directly govern:
- subscriptions or premium tiers
- streaming infrastructure or backend operators
- analytics, search, recommendations, or other off-chain platform systems
- arbitrary bank sends or unrestricted contract execution

## Core Flow

1. Instantiate `multisig` with `pasg_denom`, proposal threshold, quorum, approval threshold, voting period, and the initial allowlist of protocol contracts.
2. PASG holders deposit native `upasg` through `ExecuteMsg::DepositVotingPower {}`.
3. A holder with at least `proposal_threshold` voting power creates a proposal with one or more `ProposalAction` items.
4. Holders vote with weighted `Approve` or `Reject` ballots.
5. Once quorum and approval are satisfied, anyone can call `Execute`.
6. The contract applies any internal governance updates and forwards allowed `WasmExecute` actions.

## Delegation and Snapshot Rules

- Delegation is direct and one hop only.
- A delegator cannot delegate to self or into a delegation cycle.
- Delegation and voting-balance changes are locked while a proposal is open.
- Each proposal stores `total_power_snapshot`, `quorum_bps`, and `approval_bps` so execution readiness is queryable.

## Execute Messages

- `DepositVotingPower {}`
- `WithdrawVotingPower { amount }`
- `DelegateVotingPower { delegate }`
- `UndelegateVotingPower {}`
- `Propose { title, description, actions }`
- `Vote { proposal_id, vote }`
- `Execute { proposal_id }`
- `Close { proposal_id }`

## Proposal Actions

- `UpdateGovernanceConfig { ... }`
- `UpdateExecutionTargets { add, remove }`
- `WasmExecute { contract_addr, msg }`

`WasmExecute` is accepted only when `contract_addr` is currently allowlisted by governance config.

## Query Messages

- `Config`
- `VotingPower { address }`
- `Delegation { address }`
- `Proposal { proposal_id }`
- `Proposals { start_after, limit }`
- `Vote { proposal_id, voter }`
- `Votes { proposal_id, start_after, limit }`
- `CanExecute { proposal_id }`
- `ExecutionTargets { start_after, limit }`

## Operational Guidance

Recommended defaults for Passage:
- keep `pasg_denom` as `upasg`
- set `proposal_threshold` high enough to deter spam
- keep `quorum_bps` and `approval_bps` explicit and queryable
- allowlist only protocol contracts that governance is supposed to control
- keep platform business operations off-chain even when those services react to governance outcomes
