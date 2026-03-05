use crate::state::{Collection, Config, Ecosystem};
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
        detail: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    /// Update an existing ecosystem
    UpdateEcosystem {
        id: String,
        name: Option<String>,
        detail: Option<String>,
        image_urls: Option<Vec<String>>,
        animation_url: Option<String>,
        url: Option<String>,
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

    // ========== Collection Operations ==========
    /// Register a new collection (pg721 contract)
    RegisterCollection {
        address: String,
        ecosystem_id: String,
        name: String,
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
    /// Check if an address is approved member of an ecosystem
    #[returns(ApprovalStatusResponse)]
    IsEcosystemMember {
        ecosystem_id: String,
        member: String,
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
