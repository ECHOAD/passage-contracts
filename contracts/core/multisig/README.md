# Passage Multisig

`multisig` is a proposal-based administrative multisig contract for Passage. `multisig` remains the owner-admin executor.

## Role in the corrected governance model

- `multisig` owns and administers contracts such as `registry`, `ecosystem-factory`, `marketplace-v3`, and `auction-english`.
- PASG governance is a separate layer from multisig and does not replace signer-threshold semantics.
- PASG governance can ratify scoped protocol changes, but final owner/admin execution still happens through standard `multisig.Propose`, `Vote`, and `Execute`.

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

## Execute Messages

- `Propose`
- `Vote`
- `Execute`
- `Close`
- `UpdateMembers`

## Query Messages

- `Config`
- `Member`
- `Members`
- `Proposal`
- `Proposals`
- `Vote`
- `Votes`
- `CanExecute`

multisig remains the owner-admin executor.
