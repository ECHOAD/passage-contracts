use crate::state::{
    Collection, CollectionCreationPolicy, CollectionCreationRequest,
    CollectionCreationRequestStatus, Config, DeadProjectCase, DeadProjectStatus, Ecosystem,
    EcosystemCreationRequest, EcosystemCreationRequestStatus, EcosystemType, RecoveryConfig,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    /// Admin address with full control
    pub admin: Option<String>,
    /// Optional operator addresses
    pub operators: Option<Vec<String>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ========== Admin Operations ==========
    /// Update contract configuration
    UpdateConfig {
        admin: Option<String>,
        operators: Option<Vec<String>>,
        paused: Option<bool>,
    },
    /// Approve a creator to register new ecosystems
    ApproveEcosystemCreator { creator: String },
    /// Revoke ecosystem creation approval
    RevokeEcosystemCreator { creator: String },

    // ========== Ecosystem Operations ==========
    /// Register a new ecosystem
    RegisterEcosystem {
        id: String,
        name: String,
        ecosystem_type: Option<EcosystemType>,
        collection_creation_policy: Option<CollectionCreationPolicy>,
        collection_factory: Option<String>,
        detail: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    /// Submit a new ecosystem creation request for admin approval
    SubmitEcosystemCreationRequest {
        id: String,
        name: String,
        ecosystem_type: Option<EcosystemType>,
        collection_creation_policy: Option<CollectionCreationPolicy>,
        collection_factory: Option<String>,
        detail: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    /// Approve/reject an ecosystem creation request
    ResolveEcosystemCreationRequest {
        request_id: u64,
        approved: bool,
        note: Option<String>,
    },
    /// Update an existing ecosystem
    UpdateEcosystem {
        id: String,
        name: Option<String>,
        detail: Option<String>,
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

    // ========== Dead Project Recovery Governance ==========
    /// Update recovery configuration (admin only)
    UpdateRecoveryConfig {
        inactivity_period_secs: Option<u64>,
        contest_period_secs: Option<u64>,
    },
    /// Open a dead project recovery case
    OpenDeadProjectCase {
        target: RecoveryTargetInput,
        reason: String,
        evidence_url: Option<String>,
        proposed_replacement: Option<String>,
    },
    /// Contest a dead project case (target admin only)
    ContestDeadProjectCase { case_id: u64, note: Option<String> },
    /// Resolve a dead project case (registry governance/admin only)
    ResolveDeadProjectCase {
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
    /// Check if an address is approved to register ecosystems
    #[returns(ApprovalStatusResponse)]
    IsEcosystemCreatorApproved { creator: String },
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
    /// Get ecosystem creation request by id
    #[returns(EcosystemCreationRequestResponse)]
    EcosystemCreationRequest { request_id: u64 },
    /// List ecosystem creation requests
    #[returns(EcosystemCreationRequestsResponse)]
    EcosystemCreationRequests {
        status: Option<EcosystemCreationRequestStatus>,
        start_after: Option<u64>,
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

    // ========== Dead Project Recovery Queries ==========
    /// Get dead project recovery config
    #[returns(RecoveryConfigResponse)]
    RecoveryConfig {},
    /// Get dead project case by ID
    #[returns(DeadProjectCaseResponse)]
    DeadProjectCase { case_id: u64 },
    /// List dead project cases
    #[returns(DeadProjectCasesResponse)]
    DeadProjectCases {
        status: Option<DeadProjectStatus>,
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
pub struct CollectionCreationRequestResponse {
    pub request: Option<CollectionCreationRequest>,
}

#[cw_serde]
pub struct CollectionCreationRequestsResponse {
    pub requests: Vec<CollectionCreationRequest>,
}

#[cw_serde]
pub struct EcosystemCreationRequestResponse {
    pub request: Option<EcosystemCreationRequest>,
}

#[cw_serde]
pub struct EcosystemCreationRequestsResponse {
    pub requests: Vec<EcosystemCreationRequest>,
}

#[cw_serde]
pub struct RecoveryConfigResponse {
    pub config: RecoveryConfig,
}

#[cw_serde]
pub struct DeadProjectCaseResponse {
    pub case: Option<DeadProjectCase>,
}

#[cw_serde]
pub struct DeadProjectCasesResponse {
    pub cases: Vec<DeadProjectCase>,
}

#[cw_serde]
pub struct LastCreatorActivityResponse {
    pub creator: String,
    pub last_activity_at: Option<u64>,
}
