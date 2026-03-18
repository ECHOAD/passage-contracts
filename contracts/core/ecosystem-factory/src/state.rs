use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub operators: Vec<Addr>,
    pub registry: Addr,
    /// Code ID for instantiating collection-factory contracts
    pub collection_factory_code_id: u64,
    /// Code ID for instantiating pg721 collection contracts (passed to collection-factory)
    pub collection_code_id: u64,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemCreationRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
    Created,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EcosystemCreationRequest {
    pub request_id: u64,
    pub creator: Addr,
    pub id: String,
    pub name: String,
    pub description: String,
    pub image_urls: Vec<String>,
    pub animation_url: Option<String>,
    pub url: Option<String>,
    pub status: EcosystemCreationRequestStatus,
    pub submitted_at: u64,
    pub reviewed_at: Option<u64>,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
    pub collection_factory: Option<Addr>,
    pub created_at: Option<u64>,
}

/// Pending ecosystem creation waiting for collection-factory reply
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingEcosystemCreation {
    pub request_id: u64,
    pub ecosystem_id: String,
    pub ecosystem_name: String,
    pub creator: Addr,
    pub description: String,
    pub image_urls: Vec<String>,
    pub animation_url: Option<String>,
    pub url: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REQUESTS: Map<u64, EcosystemCreationRequest> = Map::new("requests");
pub const REQUESTS_BY_CREATOR: Map<(&Addr, u64), ()> = Map::new("requests_by_creator");
pub const PENDING_REQUEST_BY_ID: Map<String, u64> = Map::new("pending_request_by_id");
pub const NEXT_REQUEST_ID: Item<u64> = Item::new("next_request_id");
/// Temporary storage for pending ecosystem creations (keyed by reply_id)
pub const PENDING_ECOSYSTEM_CREATIONS: Map<u64, PendingEcosystemCreation> =
    Map::new("pending_ecosystem_creations");
