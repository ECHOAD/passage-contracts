# Multisig Governance

This guide explains the corrected Passage governance model.

## Two-layer model

PASG governance is a separate layer from multisig.

- `pasg-governance` handles PASG-holder proposals, deposited `upasg` voting power, delegation, quorum, pass thresholds, and governance-owned PASG utility parameter updates.
- `multisig` remains the owner-admin executor for `registry` and other protocol contracts.
- PASG governance can ratify only scoped protocol admin actions and exposes them through `ratified_admin_action` records.

`multisig` remains the owner-admin executor.

## Why `multisig` stays separate

- registry and protocol ownership remain under explicit signer control
- PASG governance does not get arbitrary raw message dispatch
- multisig signers can review the deterministic `payload_hash` before mirroring the action into `Propose`

## Handoff flow

1. PASG holders deposit `upasg` in `pasg-governance`.
2. A holder creates a proposal with either `SetPasgUtilityConfig` or a scoped admin action such as `StreamingBillingUpdateConfig`, `MarketplaceV3UpdateConfig`, or `AuctionEnglishUpdateConfig`.
3. If the PASG proposal passes, `pasg-governance` either executes the PASG-owned parameter change directly or stores a `ratified_admin_action` record with `proposal_id`, `admin_multisig`, typed `action`, and `payload_hash`.
4. Multisig signers create a normal `multisig.Propose` carrying the equivalent contract `update_config` call.
5. Multisig members `Vote` and then `Execute`.

## Example relationship

- `registry InstantiateMsg.admin = <multisig_addr>`
- CosmWasm instance admin for `registry` = `<multisig_addr>`
- PASG governance does not replace that owner/admin relationship

## Operational rule

Never treat PASG governance as a drop-in replacement for multisig membership. PASG governance ratifies scoped protocol intent; multisig performs final owner/admin execution.

multisig remains the owner-admin executor.
