# Method Reference

This reference summarizes what each smart contract does and which methods matter most.

If you need exact payload examples, also check:

- `03-json-examples.md`
- `04-multisig-governance.md`

## `multisig`

Role:

- proposal-based admin contract for `registry` and other critical Passage contracts

Instantiate:

- `InstantiateMsg { members, threshold, max_voting_period_secs }`

Most important execute messages:

- `Propose`
- `Vote`
- `Execute`
- `Close`
- `UpdateMembers`

When to use them:

- `Propose`: create a proposal containing one or more `CosmosMsg`
- `Vote`: approve or reject a proposal
- `Execute`: dispatch proposal messages once the threshold is reached
- `Close`: explicitly close an expired or failed proposal
- `UpdateMembers`: rotate signers or threshold; this is self-call only and must be executed through a multisig proposal

Most useful queries:

- `Config`
- `Member`
- `Members`
- `Proposal`
- `Proposals`
- `Vote`
- `Votes`
- `CanExecute`

## `registry`

Role:

- source of truth for ecosystems, collections, and authorized minters

Instantiate:

- `InstantiateMsg { admin, operators, recovery_council, ecosystem_factory }`

Note:

- `ecosystem_factory` can be wired during instantiate or later with `UpdateConfig`, but ecosystem creation still only happens through `RegisterEcosystemFromFactory`

Most important execute messages:

- `UpdateConfig`
- `UpdateCreatorModeration`
- `UpdateEcosystemModeration`
- `UpdateCollectionModeration`
- `SetEcosystemRecoveryPolicy`
- `SetCollectionRecoveryPolicy`
- `RegisterEcosystemFromFactory`
- `UpdateEcosystem`
- `ApproveEcosystemMember`
- `RevokeEcosystemMember`
- `SubmitCollectionCreationRequest`
- `ResolveCollectionCreationRequest`
- `RegisterCollection`
- `RegisterCollectionFromFactory`
- `RegisterExistingCollection`
- `UpdateCollection`
- `TransferCollectionOwnership`
- `AuthorizeMinter`
- `RevokeMinter`
- `UpdateRecoveryConfig`
- `OpenRecoveryCase`
- `ContestRecoveryCase`
- `ResolveRecoveryCase`

When to use them:

- `UpdateCreatorModeration`: block or allow ecosystem creation, collection creation, minting, and trading for a creator
- `UpdateEcosystemModeration`: block or allow collection creation, minting, and trading at ecosystem scope
- `UpdateCollectionModeration`: block or allow minting and trading for one collection
- `SetEcosystemRecoveryPolicy`: define who can open a lost-access recovery case for an ecosystem and who should receive control if approved
- `SetCollectionRecoveryPolicy`: define who can open a lost-access recovery case for a collection and who should receive control if approved
- `RegisterEcosystemFromFactory`: callback from `ecosystem-factory`; this is the only ecosystem creation path
- `RegisterCollectionFromFactory`: callback from `collection-factory`
- `RegisterExistingCollection`: manually onboard an already deployed collection
- `UpdateCollection`: store runtime pointers such as `minter` and `marketplace`
- `AuthorizeMinter`: required if a `minter-v2` instance will operate with `registry` enabled
- `OpenRecoveryCase`: open either `lost_access` or `abandonment` recovery
- `ResolveRecoveryCase`: recovery authority decision after the contest window

Most useful queries:

- `Config`
- `Ecosystem`
- `Ecosystems`
- `CanCreateEcosystem`
- `CanCreateCollectionInEcosystem`
- `CanMintCollection`
- `CanTradeCollection`
- `CreatorModeration`
- `EcosystemModeration`
- `CollectionModeration`
- `EcosystemRecoveryPolicy`
- `CollectionRecoveryPolicy`
- `Collection`
- `CollectionsByEcosystem`
- `IsCollectionVerified`
- `IsMinterAuthorized`
- `AuthorizedMinters`
- `RecoveryConfig`
- `RecoveryCase`
- `RecoveryCases`

## `ecosystem-factory`

Role:

- governed flow for creating ecosystems and deploying their `collection-factory`

Instantiate:

- `InstantiateMsg { admin, operators, registry, collection_factory_code_id, collection_code_id }`

Execute:

- `UpdateConfig`
- `SubmitEcosystemCreationRequest`
- `ResolveEcosystemCreationRequest`

Queries:

- `Config`
- `EcosystemCreationRequest`
- `EcosystemCreationRequests`
- `PendingRequestById`
- `IsAdminOrOperator`

Note:

- when a request is approved, this contract deploys the `collection-factory` and registers the ecosystem in `registry`

## `collection-factory`

Role:

- deploy `pg721` inside an ecosystem and register it in `registry`

Instantiate:

- `InstantiateMsg { admin, operators, registry, ecosystem_id, collection_code_id, enforce_local_allowlist, approved_creators }`

Execute:

- `UpdateConfig`
- `ApproveCreator`
- `RevokeCreator`
- `CreateCollection`

`CreateCollection` receives:

- `name`
- `symbol`
- `minter`
- `collection_info`
- `label`

Queries:

- `Config`
- `IsCreatorApproved`
- `ApprovedCreators`
- `Collection`
- `Collections`
- `CollectionsByCreator`

Notes:

- if `enforce_local_allowlist` is enabled, the wallet must be approved locally
- it always also queries `registry.CanCreateCollectionInEcosystem`
- in `reply`, the factory calls `registry.RegisterCollectionFromFactory`

## `pg721`

Role:

- base NFT collection contract

Instantiate:

- `InstantiateMsg { name, symbol, minter, collection_info }`

Relevant execute messages:

- `Mint`
- `TransferNft`
- `SendNft`
- `Approve`
- `ApproveAll`
- `Revoke`
- `RevokeAll`
- `Burn`

Relevant queries:

- `OwnerOf`
- `Approval`
- `Approvals`
- `AllOperators`
- `NftInfo`
- `AllNftInfo`
- `Tokens`
- `AllTokens`
- `Minter`
- `CollectionInfo`

Commerce usage:

- `Approve` or `ApproveAll` for `marketplace-v3` and `auction-english`
- `CollectionInfo` to resolve royalties

## `split-router`

Role:

- route creator-side mint and royalty proceeds

Instantiate:

- `InstantiateMsg { admin, registry }`

Execute:

- `UpdateConfig`
- `SetDistributionRule`
- `UpdateDistributionRule`
- `RemoveDistributionRule`
- `SetEcosystemConfig`
- `RoutePrimarySale`
- `RouteSecondaryRoyalty`
- `RouteAuctionRoyalty`
- `RouteRevenue`
- `CreateSplitWallet`
- `UpdateSplitWallet`
- `DistributeSplitWallet`
- `RemoveSplitWallet`

When to use them:

- `SetDistributionRule`: create the collection rule
- `RoutePrimarySale`: called by `minter-v2`
- `RouteSecondaryRoyalty`: called by `marketplace-v3`
- `RouteAuctionRoyalty`: called by `auction-english`

Queries:

- `Config`
- `DistributionRule`
- `DistributionRules`
- `EcosystemConfig`
- `PreviewDistribution`
- `CollectionStats`
- `RevenueEvents`
- `SplitWallet`
- `SplitWallets`

## `marketplace-v3`

Role:

- fixed-price sales, token bids, and collection bids
- marketplace-level fee config with collection-scoped denoms and activation, while moderation comes from `registry`

Instantiate:

- `InstantiateMsg { admin, min_price, trading_fee_bps, fee_collector, registry, operators }`

Admin execute:

- `UpdateConfig`
- `RegisterCollection`
- `UpdateCollectionConfig`
- `ResolveCollectionRegistrationRequest`
- `ResolveCollectionUpdateRequest`
- `DeactivateCollection`
- `ReactivateCollection`

Owner request execute:

- `SubmitCollectionRegistrationRequest`
- `SubmitCollectionUpdateRequest`

Trading execute:

- `SetAsk`
- `UpdateAsk`
- `RemoveAsk`
- `BuyNow`
- `SetBid`
- `RemoveBid`
- `AcceptBid`
- `SetCollectionBid`
- `RemoveCollectionBid`
- `AcceptCollectionBid`
- `SyncAsk`
- `BatchSyncAsks`

When to use them:

- `SetAsk`: publish a fixed-price listing
- `BuyNow`: buy a fixed-price listing
- `SetBid` / `AcceptBid`: token-specific offer flow
- `SetCollectionBid` / `AcceptCollectionBid`: collection-wide offer flow
- `SubmitCollectionRegistrationRequest`: owner submits a request to join the marketplace
- `ResolveCollectionRegistrationRequest`: admin approves or rejects the request
- `RegisterCollection`: admin may register directly without a request
- `SubmitCollectionUpdateRequest`: owner proposes denom or activation changes
- `ResolveCollectionUpdateRequest`: admin approves or rejects the requested update
- `DeactivateCollection` / `ReactivateCollection`: local marketplace operational switch; this is not the moderation source of truth

Queries:

- `Config`
- `CollectionConfig`
- `CollectionConfigs`
- `CanTrade`
- `CollectionDenom`
- `CollectionFee`
- `CollectionRegistrationRequest`
- `CollectionRegistrationRequests`
- `CollectionUpdateRequest`
- `CollectionUpdateRequests`
- `Ask`
- `AsksByCollection`
- `AsksBySeller`
- `AsksByPrice`
- `AskCount`
- `Bid`
- `BidsByToken`
- `BidsByBidder`
- `CollectionBid`
- `CollectionBidsByCollection`
- `CollectionBidsByBidder`
- `MarketStats`
- `CollectionStats`
- `PreviewSale`

## `auction-english`

Role:

- custodial reserve auction per NFT

Instantiate:

- `InstantiateMsg { admin, denom, min_price, trading_fee_bps, max_trading_fee_bps, fee_collector, registry, split_router, use_split_router, min_bid_increment_percent, min_duration, max_duration, extend_duration, require_registration }`

Execute:

- `UpdateConfig`
- `CreateAuction`
- `UpdateReservePrice`
- `CancelAuction`
- `PlaceBid`
- `SettleAuction`

Core rules:

- `CreateAuction` moves the NFT into the contract
- `UpdateReservePrice` is only allowed before the first bid
- `CancelAuction` is only allowed before the first bid
- `PlaceBid` starts or increases the auction
- `SettleAuction` pays out and transfers the NFT to the winner

Queries:

- `Config`
- `CanTrade`
- `Auction`
- `AuctionsByCollection`
- `AuctionsBySeller`
- `AuctionsByEndTime`

## `minter-v2`

Role:

- primary sale flow and deployment of its own `pg721`

Instantiate:

- `InstantiateMsg { base_token_uri, num_tokens, cw721_code_id, cw721_instantiate_msg, start_time, per_address_limit, unit_price, whitelist, registry, split_router, use_split_router, metadata_mode, native_asset_template }`

Execute:

- `Mint`
- `MintTo`
- `MintFor`
- `BatchMint`
- `UpdateConfig`
- `UpdateStartTime`
- `SetWhitelist`
- `RemoveWhitelist`
- `Withdraw`
- `WithdrawTo`
- `SetNativeAssetTemplate`
- `SetTokenNativeAssetOverride`
- `ClearTokenNativeAssetOverride`

Operational notes:

- `Mint` and `BatchMint` use `split-router` if enabled
- if `registry` is configured, the minter requires:
  - the collection to exist in `registry`
  - the minter to be authorized through `AuthorizeMinter`
- `Withdraw` only applies when `use_split_router = false`

Queries:

- `Config`
- `MintableNumTokens`
- `StartTime`
- `MintPrice`
- `MintCount`
- `CanMint`
- `MintStats`
- `IsMintingActive`
- `NativeAssetTemplate`
- `TokenNativeAssets`

## Contract relationships

Most important practical relationships:

- `collection-factory` depends on `registry`
- `marketplace-v3` queries `pg721` and optionally `registry` and `split-router`
- `auction-english` queries `pg721` and optionally `registry` and `split-router`
- `minter-v2` deploys `pg721` and optionally uses `registry` and `split-router`
- `split-router` can use `registry` for creator-side validation

If the goal is a secondary sale, the minimum path is usually:

1. `registry`
2. `collection-factory` + `pg721`
3. `split-router`
4. `marketplace-v3` or `auction-english`

## `pasg-governance`

Role:

- PASG-holder governance contract that is separate from multisig
- directly executes governance-owned PASG parameter updates
- ratifies scoped admin actions for multisig follow-through

Instantiate:

- `InstantiateMsg { admin_multisig, native_denom, proposal_deposit, voting_period_secs, quorum_bps, pass_bps }`

Most important execute messages:

- `DepositVotingPower`
- `WithdrawVotingPower`
- `Delegate`
- `Undelegate`
- `Propose`
- `Vote`
- `ExecuteProposal`
- `Close`

Typed admin actions:

- `StreamingBillingUpdateConfig`
- `MarketplaceV3UpdateConfig`
- `AuctionEnglishUpdateConfig`

Most useful queries:

- `VotingPower`
- `Proposal`
- `Proposals`
- `Vote`
- `Votes`
- `PasgUtilityConfig`
- `ratified_admin_action`

Handoff rule:

- PASG governance ratifies the scoped action.
- multisig remains the owner-admin executor and mirrors the typed action into its own `Propose` / `Vote` / `Execute` flow.
