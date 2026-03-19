# PASG Governance

`pasg-governance` is the dedicated PASG holder governance contract for Passage. It is separate from `multisig`.

This crate defines:
- PASG-holder proposal creation and voting
- governance-owned PASG utility parameter changes
- staged admin handoff records that reference the separate admin multisig

It does not replace `contracts/core/multisig`, and it does not inherit signer membership semantics from that contract.
