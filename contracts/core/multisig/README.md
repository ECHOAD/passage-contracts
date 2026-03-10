# Passage Multisig

`multisig` is a proposal-based administrative multisig contract for Passage.

It is designed to hold administrative control over contracts such as `registry`, `ecosystem-factory`, `marketplace-v3`, and `auction-english` without forcing those contracts to know anything about multisig internals.

## Core Flow

1. Instantiate the multisig with a member set and approval threshold.
2. Point contract-level admin fields to the multisig address.
3. Members create proposals with one or more `CosmosMsg` messages.
4. Members vote `Approve` or `Reject`.
5. Once the threshold is reached, anyone can execute the proposal.

## Self-Governance

Signer rotation is handled by the multisig itself:

- propose a `WasmMsg::Execute` that targets the multisig contract
- use `ExecuteMsg::UpdateMembers`
- once executed, the signer set and threshold are updated in place

This keeps the multisig address stable while allowing compromised signers to be removed.

## Execute Messages

- `Propose`
- `Vote`
- `Execute`
- `Close`
- `UpdateMembers` (self-call only)

## Query Messages

- `Config`
- `Member`
- `Members`
- `Proposal`
- `Proposals`
- `Vote`
- `Votes`
- `CanExecute`
