# Multisig Governance

This guide documents the Phase 3 PASG governance model for Passage.

`multisig` is no longer just a fixed-signer administrative shell. It is now the PASG-holder governance primitive for protocol-scoped decisions: deposited native `upasg` creates voting power, holders can delegate directly, and proposals pass only when on-chain quorum and approval rules are met.

## Scope Boundary

Governance is limited to PASG and protocol-facing configuration.

It can govern:
- its own governance parameters
- the allowlist of contracts governance may execute against
- zero-fund execute messages to allowlisted protocol contracts

It does not govern:
- subscriptions, premium tiers, Stripe orchestration, or fiat ops
- streaming infrastructure or backend operator workflows
- analytics, search, recommendation systems, or other off-chain platform operations
- arbitrary bank sends or unrestricted contract calls

This boundary is enforced in contract logic, not left as an operational convention.

## Instantiate Model

Example instantiate payload:

```json
{
  "pasg_denom": "upasg",
  "proposal_threshold": "1000000",
  "quorum_bps": 5000,
  "approval_bps": 6000,
  "max_voting_period_secs": 86400,
  "allowed_execute_contracts": [
    "passage1registry...",
    "passage1streaming..."
  ]
}
```

Meaning:
- `pasg_denom`: native PASG denomination used for governance deposits
- `proposal_threshold`: minimum voting power required to open a proposal
- `quorum_bps`: minimum participation required from the proposal snapshot
- `approval_bps`: minimum approval ratio among counted votes
- `allowed_execute_contracts`: protocol contract targets governance may call

## Governance Power Lifecycle

1. A holder deposits `upasg` via `DepositVotingPower`.
2. The contract tracks deposited balance as governance power.
3. A holder may delegate directly to another holder with `DelegateVotingPower`.
4. While a proposal is open, deposits, withdrawals, and delegation changes are locked so the vote window stays deterministic.
5. After proposals resolve, holders can rebalance or withdraw with `WithdrawVotingPower`.

## Proposal Lifecycle

1. A holder with at least `proposal_threshold` creates `Propose { title, description, actions }`.
2. The contract snapshots `total_power_snapshot`, `quorum_bps`, and `approval_bps` into the proposal.
3. Holders vote `Approve` or `Reject` with weighted power.
4. `CanExecute` becomes true only when quorum and approval are both satisfied.
5. Anyone can call `Execute { proposal_id }` for a passed proposal.
6. Expired proposals that did not pass can be closed with `Close { proposal_id }`.

## Supported Proposal Actions

### Update governance config

```json
{
  "update_governance_config": {
    "proposal_threshold": "1500000",
    "quorum_bps": 5500,
    "approval_bps": 6500,
    "max_voting_period_secs": 172800
  }
}
```

### Update execution targets

```json
{
  "update_execution_targets": {
    "add": ["passage1registry..."],
    "remove": ["passage1oldtarget..."]
  }
}
```

### Execute against an allowlisted protocol contract

```json
{
  "wasm_execute": {
    "contract_addr": "passage1registry...",
    "msg": "<base64 encoded execute message>"
  }
}
```

## Queries Operators Should Use

- `Config {}`: current governance parameters
- `VotingPower { address }`: deposited power, incoming delegated power, effective power, delegate target
- `Delegation { address }`: outgoing delegation for an address
- `Proposal { proposal_id }`: stored proposal plus computed status
- `CanExecute { proposal_id }`: whether a proposal is executable right now
- `ExecutionTargets {}`: allowlisted protocol targets

## Recommended Ownership Model

For contracts such as `registry` and similar protocol primitives:
- set contract-level admin to the `multisig` address
- set CosmWasm instance admin to the `multisig` address when governance should control migration or config
- keep lower-trust operator roles separate from governance

## Operational Defaults

Recommended defaults for Passage:
- use native `upasg` deposits for governance power
- treat delegation as a convenience for participation, not a replacement for quorum
- allowlist only contracts whose config belongs to PASG or protocol governance
- never use governance as a transport for off-chain business operations
