# PASG Governance

`pasg-governance` is the PASG-holder voting layer for Passage. PASG governance is a separate layer from multisig.

## Responsibility split

- PASG governance creates proposals, tracks deposited `upasg`, enforces delegation, quorum, pass thresholds, and deposit locks.
- PASG governance can directly execute governance-owned PASG utility parameter changes.
- PASG governance can ratify only typed protocol admin actions through `ratified_admin_action` records.
- `multisig` remains the owner-admin executor for `registry`, `marketplace-v3`, `auction-english`, and similar contracts.

## Typed admin handoff

The scoped handoff catalog is limited to:

- `StreamingBillingUpdateConfig`
- `MarketplaceV3UpdateConfig`
- `AuctionEnglishUpdateConfig`

A passed admin proposal stores:

- `proposal_id`
- `admin_multisig`
- typed `action`
- deterministic `payload_hash`

That record is what multisig signers mirror into a standard `multisig.Propose` flow.

## Non-goals

- no raw arbitrary `CosmosMsg` payloads
- no replacement of the existing multisig admin plane
- no governance over off-chain product systems like analytics, search, rendering, or streaming infrastructure

multisig remains the owner-admin executor.
