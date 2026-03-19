# End-to-End Setup

This guide describes the recommended flow from `registry` instantiation to a collection that is ready to sell through `marketplace-v3` or `auction-english`.

Ready-to-use JSON payloads:

- `03-json-examples.md`

Recommended admin pattern:

- `04-multisig-governance.md`

## 0. Instantiate `multisig` first

This is not mandatory, but it is the recommended admin model for Passage.

Recommended usage:

- deploy `multisig` before `registry`
- use the multisig address as `registry` contract admin in `InstantiateMsg.admin`
- use the same multisig address as the CosmWasm instance admin for migration control

Why:

- a compromised signer does not automatically compromise the whole admin surface
- signer rotation can happen in-place without changing the multisig address
- all critical admin actions become proposal-based and auditable

## 1. Instantiate `registry`

`registry` is the first contract that should exist.

Important `InstantiateMsg` fields:

- `admin`: global registry admin.
- `operators`: optional global operators.
- `ecosystem_factory`: optional at deploy time, but must be configured before any ecosystem can be approved and registered.

Usage:

- Deploy `registry` after `multisig` if you are using the recommended setup.
- Deploy `ecosystem-factory` pointing to that `registry`.
- Call `registry.UpdateConfig { ecosystem_factory }` to wire the authorized factory address.
- `registry` does not accept direct ecosystem creation. Approved ecosystems are registered only through `RegisterEcosystemFromFactory`, so ecosystem onboarding remains blocked until the factory address is configured.

Recommended admin wiring:

- `registry InstantiateMsg.admin = <multisig_addr>`
- CosmWasm instance admin for `registry` = `<multisig_addr>`

## 2. Create the ecosystem through `ecosystem-factory`

This is the only supported path.

`ecosystem-factory` is instantiated with:

- `admin`
- `operators`
- `registry`
- `collection_factory_code_id`
- `collection_code_id`

Flow:

1. A creator calls `SubmitEcosystemCreationRequest`.
2. An admin or operator calls `ResolveEcosystemCreationRequest`.
3. If approved, `ecosystem-factory` deploys a dedicated `collection-factory`.
4. In `reply`, the factory calls `registry.RegisterEcosystemFromFactory`.

Expected result:

- The ecosystem is registered in `registry`.
- The ecosystem is linked to a `collection-factory`.

## 3. Validate the dedicated `collection-factory`

When the request is approved, `ecosystem-factory` already instantiates the dedicated `collection-factory` for that ecosystem.

That child factory is instantiated with:

- `admin`
- `operators`
- `registry`
- `ecosystem_id`
- `collection_code_id`
- `enforce_local_allowlist`
- `approved_creators`

Notes:

- `enforce_local_allowlist = true` requires wallets to be approved locally through `ApproveCreator`.
- Even if local allowlisting is disabled, `collection-factory` still queries `registry.CanCreateCollectionInEcosystem`.
- Ecosystem admin and approved ecosystem members can create collections directly. Collection approval flow is not part of this model.

## 4. Create the `pg721` collection

The collection is created through `collection-factory`.

Main message:

- `CreateCollection { name, symbol, minter, collection_info, label }`

`collection_info` includes:

- `description`
- `image`
- `external_link`
- `royalty_info`

Important points:

- `minter` is the address that will be allowed to call `Mint` inside `pg721`.
- If you want to mint manually, use a wallet or contract you control.
- If you want a primary drop with `minter-v2`, do not use this path for that collection; `minter-v2` deploys its own `pg721`.

What happens next:

1. `collection-factory` instantiates `pg721`.
2. In `reply`, it registers the new collection in `registry` via `RegisterCollectionFromFactory`.

Recommended checks:

- `registry.Collection { address }`
- `registry.CollectionsByEcosystem { ecosystem_id }`

Lifecycle note:

- the collection contract address is the canonical collection identity
- later, `registry.DeregisterCollection` can detach it from the ecosystem without destroying the contract
- an unaffiliated collection can later be attached to another ecosystem with `registry.RehomeCollection`

## 5. Instantiate `split-router`

`split-router` routes creator proceeds and royalties.

It is instantiated with:

- `admin`
- `registry`

Then you should create a rule per collection:

- `SetDistributionRule { collection, creator, creator_share, collaborators }`

Recommended usage:

- Configure it before enabling sales.
- If you will use `marketplace-v3` or `auction-english` with `use_split_router = true`, the collection must already have a rule.

Useful queries:

- `DistributionRule { collection }`
- `PreviewDistribution { collection, amount, event_type }`

## 5.5 Optional local economy wiring through `streaming-billing`

Use this only if a world needs bounded points-based usage accounting.

Important boundaries:

- local world points are optional and auxiliary
- they still settle through canonical PASG-aware accounting
- creator monetization does not move here by default; it remains anchored to initial sales and resales at collection level
- `ecosystem` remains administrative context, not the local-economy policy engine

Useful queries:

- `PasgUtility`
- `WorldLocalEconomy { world_nft_id }`
- `PreviewWorldSettlement { world_nft_id, duration_seconds | points, user }`
- `PendingRevenue { world_nft_id }`
## 6. Fixed-price sale path with `marketplace-v3`

`marketplace-v3` is for secondary sales. It does not create or custody collections.

Creator asset boundary:

- typed creator asset semantics such as `plugin`, `achievement`, and `world_template` live in the shared `pg721` metadata surface
- royalty and monetization semantics stay contract-facing
- runtime, rendering, and Unreal-specific payloads stay off-chain

### 6.1 Instantiate the marketplace

Important fields:

- `admin`
- `min_price`
- `trading_fee_bps`
- `fee_collector`
- `registry`
- `operators`

Recommended usage:

- `registry = <registry>`

Notes:

- registration is mandatory in `marketplace-v3`
- `trading_fee_bps` is marketplace-global
- settlement `denom` is configured per collection, not at instantiate time

### 6.2 Register the collection in the marketplace

Owner-request path:

- `SubmitCollectionRegistrationRequest { collection, denom, note }`
- admin resolves it with `ResolveCollectionRegistrationRequest { collection, approved, denom, note }`

Admin-direct path:

- `RegisterCollection { collection, denom }`

Notes:

- This enables the collection inside the marketplace.
- It does not replace registration in `registry`; these are two separate registrations.
- owner submits a request, admin approves or rejects it
- admin may register directly without the request mechanism

Optional but recommended:

- Save the runtime pointer in `registry` with `UpdateCollection { marketplace: Some(...) }`

### 6.3 Approve the marketplace in `pg721`

Before listing, the owner must approve the marketplace in the `pg721` collection.

Relevant `pg721` methods:

- `Approve { spender, token_id, expires }`
- `ApproveAll { operator, expires }`

Without this approval, the sale will not be able to complete when a buyer arrives.

### 6.4 Create the sale

Message:

- `SetAsk { collection, token_id, price, funds_recipient }`

From there you have three paths:

- direct purchase with `BuyNow`
- token-specific offer with `SetBid` and acceptance with `AcceptBid`
- collection-wide offer with `SetCollectionBid` and acceptance with `AcceptCollectionBid`

### 6.5 How sale settlement works

In the current model:

- `marketplace-v3` charges `trading_fee_bps`
- the seller receives `sale_price - trading_fee - royalty`
- if the royalty recipient is a contract, the marketplace calls it with `Split {}`
- otherwise the royalty is paid directly to the `payment_address` from `pg721`

Useful queries:

- `CanTrade { collection }`
- `PreviewSale { collection, price }`
- `Ask { collection, token_id }`
- `CollectionStats { collection }`
- `CollectionRegistrationRequest { collection }`
- `CollectionUpdateRequest { collection }`

## 7. Auction path with `auction-english`

`auction-english` is also for secondary sales, but it is custodial.

### 7.1 Instantiate the auction contract

Important fields:

- `admin`
- `denom`
- `min_price`
- `trading_fee_bps`
- `max_trading_fee_bps`
- `fee_collector`
- `registry`
- `split_router`
- `use_split_router`
- `min_bid_increment_percent`
- `min_duration`
- `max_duration`
- `extend_duration`
- `require_registration`

Recommended usage:

- `require_registration = true`
- `registry = <registry>`
- `split_router = <split-router>`
- `use_split_router = true`

### 7.2 Approve the auction contract in `pg721`

The owner must approve the auction contract in the `pg721` collection.

Relevant methods:

- `Approve`
- `ApproveAll`

### 7.3 Create the auction

Message:

- `CreateAuction { collection, token_id, reserve_price, duration, seller_funds_recipient }`

Important:

- the NFT is transferred into the auction contract when the auction is created
- before the first bid, the seller can still:
  - `UpdateReservePrice`
  - `CancelAuction`

### 7.4 Receive bids

Message:

- `PlaceBid { collection, token_id }`

Behavior:

- the first bid starts the closing timer
- later bids must respect `min_bid_increment_percent`
- if little time remains, the auction extends using `extend_duration`

### 7.5 Close and settle

Message:

- `SettleAuction { collection, token_id }`

Behavior:

- anyone can settle after the auction has ended
- `trading_fee` is paid to the `fee_collector`
- seller proceeds are paid to the seller or to `seller_funds_recipient`
- the royalty goes to `split-router` if enabled
- the NFT is transferred to the winner

Useful queries:

- `CanTrade { collection }`
- `Auction { collection, token_id }`
- `AuctionsByCollection`
- `AuctionsBySeller`
- `AuctionsByEndTime`

## 8. Optional primary sale path with `minter-v2`

This path is different. `minter-v2` deploys its own `pg721` collection.

Recommended flow:

1. Instantiate `minter-v2`.
2. Wait for `reply` or query `Config {}` to obtain `cw721_address`.
3. Register that collection in `registry` with `RegisterExistingCollection`.
4. Authorize the minter with `AuthorizeMinter { collection_address, minter_address }`.
5. Optionally save the runtime pointer with `UpdateCollection { minter: Some(...) }`.
6. Create the `split-router` rule for that collection.
7. Open minting with `start_time` and then use `Mint` or `BatchMint`.

Without steps 3 and 4, `minter-v2` can be blocked if `registry` is configured, because it validates:

- that the collection exists in `registry`
- that the minter is authorized in `registry`

## 9. Minimum production checklist

### Fixed-price sale

1. `registry` deployed and operational
2. collection registered in `registry`
3. `split-router` rule created for the collection
4. `marketplace-v3` instantiated
5. collection registered in `marketplace-v3`
6. owner approved the marketplace in `pg721`
7. `SetAsk`

### Auction

1. `registry` deployed and operational
2. collection registered in `registry`
3. `split-router` rule created for the collection
4. `auction-english` instantiated
5. owner approved the auction contract in `pg721`
6. `CreateAuction`
7. bids
8. `SettleAuction`

For exact JSON messages, also use:

- `03-json-examples.md`

## PASG governance note

PASG governance is a separate layer from multisig.

Use `pasg-governance` for PASG-holder voting and ratification, but keep `multisig` as the owner-admin executor for `registry`, `marketplace-v3`, `auction-english`, and similar protocol contracts. A passed PASG admin proposal should be read through `ratified_admin_action` and then mirrored into a normal `multisig.Propose` flow.

## PASG staking note

PASG staking follows chain-native staking through native validator delegation on Passage, not a duplicate CosmWasm staking vault.

- Expected actions: delegate `upasg`, undelegate from a validator, redelegate between validators, and query delegations, undelegations, validator assignments, and rewards through chain-native surfaces.
- Validator selection belongs to wallet or service UX. Use chain-native validator metadata, commission, uptime, and policy guidance to choose one or more validators rather than assuming a single contract-owned validator.
- Any 21-day unbonding period is a chain-level staking rule or validator-program dependency, not a repo-local claim queue.
- `contracts/staking/nft-vault`, `contracts/staking/stake-rewards`, and `contracts/staking/vault-factory` remain NFT staking primitives and factory tooling, not the default PASG staking path.

