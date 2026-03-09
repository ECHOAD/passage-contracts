use crate::state::{Config, EcosystemCreationRequest, EcosystemCreationRequestStatus};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub operators: Option<Vec<String>>,
    pub registry: String,
    /// Code ID for collection-factory contracts
    pub collection_factory_code_id: u64,
    /// Code ID for pg721 collection contracts (passed to collection-factory)
    pub collection_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        operators: Option<Vec<String>>,
        registry: Option<String>,
        collection_factory_code_id: Option<u64>,
        collection_code_id: Option<u64>,
        paused: Option<bool>,
    },
    SubmitEcosystemCreationRequest {
        id: String,
        name: String,
        detail: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    ResolveEcosystemCreationRequest {
        request_id: u64,
        approved: bool,
        note: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(EcosystemCreationRequestResponse)]
    EcosystemCreationRequest { request_id: u64 },
    #[returns(EcosystemCreationRequestsResponse)]
    EcosystemCreationRequests {
        status: Option<EcosystemCreationRequestStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(PendingRequestResponse)]
    PendingRequestById { id: String },
    #[returns(ApprovalStatusResponse)]
    IsAdminOrOperator { address: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
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
pub struct PendingRequestResponse {
    pub request_id: Option<u64>,
}

#[cw_serde]
pub struct ApprovalStatusResponse {
    pub approved: bool,
}

#[cw_serde]
pub enum RegistryExecuteMsg {
    RegisterEcosystemFromFactory {
        id: String,
        name: String,
        creator: String,
        collection_factory: String,
        detail: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
}

#[cw_serde]
pub enum RegistryQueryMsg {
    IsCrossEcosystemAdmin { address: String },
}

#[cw_serde]
pub struct RegistryApprovalStatusResponse {
    pub approved: bool,
}

/// InstantiateMsg for the collection-factory contract
#[cw_serde]
pub struct CollectionFactoryInstantiateMsg {
    pub admin: Option<String>,
    pub operators: Option<Vec<String>>,
    pub registry: String,
    pub ecosystem_id: String,
    pub collection_code_id: u64,
    pub enforce_local_allowlist: Option<bool>,
    pub approved_creators: Option<Vec<String>>,
}
