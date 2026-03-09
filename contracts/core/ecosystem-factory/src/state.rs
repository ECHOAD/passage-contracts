use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub operators: Vec<Addr>,
    pub registry: Addr,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemType {
    Public,
    Private,
}

impl Default for EcosystemType {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemCreationRequestStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EcosystemCreationRequest {
    pub request_id: u64,
    pub creator: Addr,
    pub id: String,
    pub name: String,
    pub ecosystem_type: EcosystemType,
    pub collection_factory: Option<Addr>,
    pub detail: String,
    pub image_urls: Vec<String>,
    pub animation_url: Option<String>,
    pub url: Option<String>,
    pub status: EcosystemCreationRequestStatus,
    pub submitted_at: u64,
    pub reviewed_at: Option<u64>,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REQUESTS: Map<u64, EcosystemCreationRequest> = Map::new("requests");
pub const PENDING_REQUEST_BY_ID: Map<String, u64> = Map::new("pending_request_by_id");
pub const NEXT_REQUEST_ID: Item<u64> = Item::new("next_request_id");
