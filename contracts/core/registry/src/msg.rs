use crate::state::{
    Collection, CollectionCreationPolicy, CollectionCreationRequest,
    CollectionCreationRequestStatus, CollectionModeration, Config, CreatorModeration, Ecosystem,
    EcosystemModeration, EcosystemType, RecoveryCase, RecoveryCaseKind, RecoveryCaseStatus,
    RecoveryConfig, RecoveryPolicy,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    /// Admin address with full control
    pub admin: Option<String>,
    /// Optional operator addresses
    pub operators: Option<Vec<String>>,
    /// Optional recovery authority addresses
    pub recovery_council: Option<Vec<String>>,
    /// Optional ecosystem factory contract. It can be wired after deployment with `UpdateConfig`.
    pub ecosystem_factory: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ========== Admin Operations ==========
    /// Update contract configuration
    UpdateConfig {
        admin: Option<String>,
        operators: Option<Vec<String>>,
        recovery_council: Option<Vec<String>>,
        ecosystem_factory: Option<String>,
        paused: Option<bool>,
    },
    /// Update moderation settings for a creator
    UpdateCreatorModeration {
        creator: String,
        ecosystem_creation_enabled: Option<bool>,
        collection_creation_enabled: Option<bool>,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    /// Update moderation settings for an ecosystem
    UpdateEcosystemModeration {
        ecosystem_id: String,
        collection_creation_enabled: Option<bool>,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    /// Update moderation settings for a collection
    UpdateCollectionModeration {
        address: String,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    /// Set the ownership recovery policy for an ecosystem
    SetEcosystemRecoveryPolicy {
        ecosystem_id: String,
        delegate: Option<String>,
        designated_successor: Option<String>,
    },
    /// Set the ownership recovery policy for a collection
    SetCollectionRecoveryPolicy {
        address: String,
        delegate: Option<String>,
        designated_successor: Option<String>,
    },

    // ========== Ecosystem Operations ==========
    /// Register a new ecosystem from an authorized ecosystem factory
    RegisterEcosystemFromFactory {
        id: String,
        name: String,
        creator: String,
        /// Collection factory address (required - created by ecosystem-factory)
        collection_factory: String,
        description: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    /// Update an existing ecosystem
    UpdateEcosystem {
        id: String,
        name: Option<String>,
        description: Option<String>,
        image_urls: Option<Vec<String>>,
        animation_url: Option<String>,
        url: Option<String>,
        ecosystem_type: Option<EcosystemType>,
        collection_creation_policy: Option<CollectionCreationPolicy>,
        collection_factory: Option<String>,
        admin: Option<String>,
    },
    /// Approve a team member to register collections in an ecosystem
    ApproveEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    /// Revoke team member approval for an ecosystem
    RevokeEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    /// Submit request to be approved for collection creation in an ecosystem
    SubmitCollectionCreationRequest {
        ecosystem_id: String,
        note: Option<String>,
    },
    /// Approve/reject a pending collection creation request
    ResolveCollectionCreationRequest {
        ecosystem_id: String,
        creator: String,
        approved: bool,
        note: Option<String>,
    },

    // ========== Collection Operations ==========
    /// Register a new collection (pg721 contract)
    RegisterCollection {
        address: String,
        ecosystem_id: String,
        name: String,
    },
    /// Register collection from authorized ecosystem factory
    RegisterCollectionFromFactory {
        address: String,
        ecosystem_id: String,
        name: String,
        creator: String,
    },
    /// Register an existing/already-deployed collection
    RegisterExistingCollection {
        address: String,
        ecosystem_id: String,
        name: String,
        creator: String,
    },
    /// Update collection metadata
    UpdateCollection {
        address: String,
        name: Option<String>,
        verified: Option<bool>,
        minter: Option<String>,
        marketplace: Option<String>,
    },
    /// Transfer collection ownership
    TransferCollectionOwnership {
        address: String,
        new_creator: String,
    },

    // ========== Minter Authorization ==========
    /// Authorize a minter for a collection
    AuthorizeMinter {
        collection_address: String,
        minter_address: String,
    },
    /// Revoke minter authorization
    RevokeMinter {
        collection_address: String,
        minter_address: String,
    },

    // ========== Ownership Recovery Governance ==========
    /// Update recovery configuration (admin only)
    UpdateRecoveryConfig {
        abandonment_inactivity_period_secs: Option<u64>,
        contest_period_secs: Option<u64>,
    },
    /// Open an ownership recovery case
    OpenRecoveryCase {
        case_kind: RecoveryCaseKind,
        target: RecoveryTargetInput,
        reason: String,
        evidence_url: Option<String>,
        proposed_replacement: Option<String>,
    },
    /// Contest a recovery case (target admin only)
    ContestRecoveryCase { case_id: u64, note: Option<String> },
    /// Resolve a recovery case (recovery authority/admin only)
    ResolveRecoveryCase {
        case_id: u64,
        approved: bool,
        note: Option<String>,
    },
}

#[cw_serde]
pub enum RecoveryTargetInput {
    Ecosystem { ecosystem_id: String },
    Collection { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},

    // ========== Ecosystem Queries ==========
    /// Get ecosystem by ID
    #[returns(EcosystemResponse)]
    Ecosystem { id: String },
    /// List all ecosystems
    #[returns(EcosystemsResponse)]
    Ecosystems {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// List ecosystems by admin
    #[returns(EcosystemsResponse)]
    EcosystemsByAdmin {
        admin: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Get creator moderation settings
    #[returns(CreatorModerationResponse)]
    CreatorModeration { creator: String },
    /// Get ecosystem moderation settings
    #[returns(EcosystemModerationResponse)]
    EcosystemModeration { ecosystem_id: String },
    /// Get collection moderation settings
    #[returns(CollectionModerationResponse)]
    CollectionModeration { address: String },
    /// Get ecosystem recovery policy
    #[returns(RecoveryPolicyResponse)]
    EcosystemRecoveryPolicy { ecosystem_id: String },
    /// Get collection recovery policy
    #[returns(RecoveryPolicyResponse)]
    CollectionRecoveryPolicy { address: String },
    /// Check whether a creator can create ecosystems
    #[returns(ApprovalStatusResponse)]
    CanCreateEcosystem { creator: String },
    /// Check if address is a cross-ecosystem admin/operator
    #[returns(ApprovalStatusResponse)]
    IsCrossEcosystemAdmin { address: String },
    /// Check if an address is approved member of an ecosystem
    #[returns(ApprovalStatusResponse)]
    IsEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    /// Check whether creator can create collections in an ecosystem (policy-aware)
    #[returns(ApprovalStatusResponse)]
    CanCreateCollectionInEcosystem {
        ecosystem_id: String,
        creator: String,
    },
    /// Get collection creation request by ecosystem+creator
    #[returns(CollectionCreationRequestResponse)]
    CollectionCreationRequest {
        ecosystem_id: String,
        creator: String,
    },
    /// List collection creation requests in an ecosystem
    #[returns(CollectionCreationRequestsResponse)]
    CollectionCreationRequests {
        ecosystem_id: String,
        status: Option<CollectionCreationRequestStatus>,
        start_after_creator: Option<String>,
        limit: Option<u32>,
    },

    // ========== Collection Queries ==========
    /// Get collection by address
    #[returns(CollectionResponse)]
    Collection { address: String },
    /// List all collections
    #[returns(CollectionsResponse)]
    Collections {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// List collections by ecosystem
    #[returns(CollectionsResponse)]
    CollectionsByEcosystem {
        ecosystem_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// List collections by creator
    #[returns(CollectionsResponse)]
    CollectionsByCreator {
        creator: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Check if collection is verified/official
    #[returns(IsVerifiedResponse)]
    IsCollectionVerified { address: String },
    /// Check whether a collection can mint
    #[returns(ApprovalStatusResponse)]
    CanMintCollection { address: String },
    /// Check whether a collection can trade
    #[returns(ApprovalStatusResponse)]
    CanTradeCollection { address: String },

    // ========== Minter Queries ==========
    /// Check if minter is authorized for collection
    #[returns(IsMinterAuthorizedResponse)]
    IsMinterAuthorized {
        collection_address: String,
        minter_address: String,
    },
    /// List authorized minters for a collection
    #[returns(AuthorizedMintersResponse)]
    AuthorizedMinters {
        collection_address: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    // ========== Ownership Recovery Queries ==========
    /// Get ownership recovery config
    #[returns(RecoveryConfigResponse)]
    RecoveryConfig {},
    /// Get recovery case by ID
    #[returns(RecoveryCaseResponse)]
    RecoveryCase { case_id: u64 },
    /// List recovery cases
    #[returns(RecoveryCasesResponse)]
    RecoveryCases {
        status: Option<RecoveryCaseStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    /// Get latest known creator activity timestamp
    #[returns(LastCreatorActivityResponse)]
    LastCreatorActivity { creator: String },
}

// ========== Response Types ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct EcosystemResponse {
    pub ecosystem: Option<Ecosystem>,
}

#[cw_serde]
pub struct EcosystemsResponse {
    pub ecosystems: Vec<Ecosystem>,
}

#[cw_serde]
pub struct CollectionResponse {
    pub collection: Option<Collection>,
}

#[cw_serde]
pub struct CollectionsResponse {
    pub collections: Vec<Collection>,
}

#[cw_serde]
pub struct IsVerifiedResponse {
    pub is_verified: bool,
}

#[cw_serde]
pub struct IsMinterAuthorizedResponse {
    pub is_authorized: bool,
}

#[cw_serde]
pub struct AuthorizedMintersResponse {
    pub minters: Vec<Addr>,
}

#[cw_serde]
pub struct ApprovalStatusResponse {
    pub approved: bool,
}

#[cw_serde]
pub struct CreatorModerationResponse {
    pub creator: String,
    pub moderation: CreatorModeration,
}

#[cw_serde]
pub struct EcosystemModerationResponse {
    pub ecosystem_id: String,
    pub moderation: EcosystemModeration,
}

#[cw_serde]
pub struct CollectionModerationResponse {
    pub address: String,
    pub moderation: CollectionModeration,
}

#[cw_serde]
pub struct RecoveryPolicyResponse {
    pub policy: RecoveryPolicy,
}

#[cw_serde]
pub struct CollectionCreationRequestResponse {
    pub request: Option<CollectionCreationRequest>,
}

#[cw_serde]
pub struct CollectionCreationRequestsResponse {
    pub requests: Vec<CollectionCreationRequest>,
}

#[cw_serde]
pub struct RecoveryConfigResponse {
    pub config: RecoveryConfig,
}

#[cw_serde]
pub struct RecoveryCaseResponse {
    pub case: Option<RecoveryCase>,
}

#[cw_serde]
pub struct RecoveryCasesResponse {
    pub cases: Vec<RecoveryCase>,
}

#[cw_serde]
pub struct LastCreatorActivityResponse {
    pub creator: String,
    pub last_activity_at: Option<u64>,
}
