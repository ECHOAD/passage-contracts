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

- source of truth for ecosystems, mutable collection affiliation, typed collection classes, and authorized minters

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
- `RegisterCollection`
- `RegisterCollectionFromFactory`
- `RegisterExistingCollection`
- `DeregisterCollection`
- `RehomeCollection`
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
- `RegisterExistingCollection`: manually onboard an already deployed collection into an ecosystem
- `DeregisterCollection`: detach a collection from its current ecosystem without destroying the contract
- `RehomeCollection`: attach an unaffiliated collection to a new ecosystem while preserving creator provenance
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
- `UnaffiliatedCollections`
- `CollectionsByNftType`
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

## Creator Asset Contracts

This repo uses an ecosystem-centric creator asset model:

- `registry` is the canonical ledger for ecosystems and collection affiliation
- collection address is the canonical collection identity
- ecosystem affiliation can be detached and later re-homed
- typed asset semantics live in the shared `pg721` family
- runtime, rendering, and Unreal-specific behavior stay off-chain

## `pg721`

Role:

- base typed NFT collection contract for durable Passage asset semantics

Instantiate:

- `InstantiateMsg { name, symbol, minter, nft_type, collection_info }`

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

Supported asset classes:

- `component`
- `avatar`
- `companion`
- `world`
- `plugin`
- `achievement`
- `world_template`

Typed extension variants:

- `Component(ComponentExtension)`
- `Avatar(AvatarExtension)`
- `Companion(CompanionExtension)`
- `World(WorldExtension)`
- `Plugin(PluginExtension)`
- `Achievement(AchievementExtension)`
- `WorldTemplate(WorldTemplateExtension)`

Notes:

- `token_uri` is the manifest pointer for rendering, compatibility details, and other runtime-facing metadata
- `profile_id` is a minimal Passage compatibility marker, not a freeform runtime payload
- `WorldExtension` carries `revenue_shares` for world revenue routing
- avatar and companion progression does not live in generic NFT metadata
- plugin, achievement, and world_template metadata stay compact and contract-facing
- runtime and rendering metadata stay off-chain

## `pg721-updatable`

Role:

- typed NFT collection contract with creator-controlled token URI updates until frozen

Instantiate:

- `InstantiateMsg { name, symbol, minter, nft_type, collection_info }`

Additional execute and query messages:

- `UpdateTokenMetadata`
- `FreezeTokenMetadata`
- `FrozenTokenMetadata`

Boundary notes:

- `UpdateTokenMetadata` only changes `token_uri`; it does not mutate typed extension data
- manifests behind `token_uri` hold rendering/runtime detail and expanded compatibility metadata
- mutable state with protocol significance uses dedicated surfaces rather than generic NFT metadata

## `asset-progression`

Role:

- dedicated state surface for world-scoped avatar and companion progression snapshots

Instantiate:

- `InstantiateMsg { admin }`

Execute:

- `SaveSnapshot`
- `UpdateAdmin`

Queries:

- `Config`
- `Snapshot`
- `SnapshotsByAsset`
- `SnapshotsByWorld`

Notes:

- progression is persisted at save points instead of being written into generic NFT metadata
- the NFT remains the durable identity and ownership surface
- gameplay formulas and rich runtime state remain world-defined and off-chain

## `world-plugin-assignment`

Role:

- dedicated relationship surface for durable plugin-to-world assignment rights

Instantiate:

- `InstantiateMsg {}`

Execute:

- `Assign`
- `Remove`

Queries:

- `Assignment`
- `AssignmentsByWorld`
- `AssignmentsByPlugin`

Notes:

- assignment rights remain queryable even if the plugin NFT is later sold
- plugin deployment, binaries, runtime permissions, and installation mechanics stay off-chain
- generic NFT metadata does not carry durable world-assignment state

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
- marketplace-scoped fee config with collection-scoped denoms and activation, while moderation comes from `registry`

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

- primary sale flow for an already deployed `pg721`

Instantiate:

- `InstantiateMsg { cw721_address, base_token_uri, num_tokens, start_time, per_address_limit, unit_price, whitelist, registry }`

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

Operational notes:

- `minter-v2` no longer deploys collections; it points at an existing `pg721`
- if `registry` is configured, the minter requires:
  - the collection to exist in `registry`
  - minting to be enabled for that collection in `registry`
  - the minter to be authorized through `AuthorizeMinter`
- the target collection must already expose a compatible `CollectionInfo {}` query surface

Queries:

- `Config`
- `MintableNumTokens`
- `StartTime`
- `MintPrice`
- `MintCount`
- `CanMint`
- `MintStats`
- `IsMintingActive`

## Contract relationships

Most important practical relationships:

- `collection-factory` depends on `registry`
- `marketplace-v3` queries `pg721` and optionally `registry` and `split-router`
- `auction-english` queries `pg721` and optionally `registry` and `split-router`
- `minter-v2` operates an existing `pg721` and optionally uses `registry`
- `split-router` can use `registry` for creator-side validation

If the goal is a secondary sale, the minimum path is usually:

1. `registry`
2. `collection-factory` + `pg721`
3. `split-router`
4. `marketplace-v3` or `auction-english`

## PASG native staking model

Architecture boundary:

- PASG staking means chain-native staking through native validator delegation on Passage, not a duplicate CosmWasm staking vault.
- The source of truth for delegation balances, undelegation entries, validator assignments, and reward state is the chain staking module plus the active validator set.
- `contracts/staking/nft-vault` and `contracts/staking/stake-rewards` remain NFT staking contracts. They are not the default PASG validator staking target.

Expected action surface:

- `delegate(upasg, validator)`
- `undelegate(upasg, validator, amount)`
- `redelegate(upasg, src_validator, dst_validator, amount)`
- queries for delegations, undelegations, validator assignments, validators, and rewards

Operational notes:

- Validator fee participation and PASG rewards are outputs of the chain staking and token program. Wallets and services should read reward-state and undelegation status from chain-native staking and distribution queries rather than from a contract-local reward engine.
- Any 21-day unbonding period is a chain-wide staking rule or validator-program dependency, not a contract-local claim queue.
- Validator selection belongs to wallet and service UX. Choose one or more Passage validators using validator identity, commission, uptime, and any Passage governance or operator policy guidance.
- Validator discovery, commission, uptime, delegations, undelegations, and rewards should be read from chain-native staking and distribution endpoints rather than from repo-local contract storage.

## `streaming-billing`

Role:

- canonical PASG utility contract for this repo
- bounded local-economy and points accounting surface
- service-invoked session settlement and downstream PASG-aware world settlement forwarding

Instantiate:

- `InstantiateMsg { admin, split_router, registry, backend_operator, denom, points_per_denom, fiat_oracle, stripe_webhook_validator }`

Most important execute messages:

- `DepositCrypto`
- `ReportFiatPurchase`
- `WithdrawPoints`
- `SetWorldRate`
- `UpdateWorldRate`
- `DistributeWorldRevenue`
- `BatchDistributeRevenue`

Most useful queries:

- `PasgUtility`
- `ConversionRate`
- `WorldLocalEconomy`
- `PreviewWorldSettlement`
- `WorldConfig`
- `PendingRevenue`
- `PlatformStats`

Notes:

- local world economies are optional and currently use a points model
- those local units still settle through PASG-aware accounting
- creator monetization does not move here by default; it remains anchored to collection sales and resales
- refund-safe settlement should reuse existing marketplace and auction patterns before introducing new escrow machinery
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
- PASG governance does not model validator discovery, custody stake, calculate PASG rewards, or alter undelegation state. Those remain chain-native concerns.
- multisig remains the owner-admin executor and mirrors the typed action into its own `Propose` / `Vote` / `Execute` flow.



