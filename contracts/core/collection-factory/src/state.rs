use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftType {
    Component,
    Avatar,
    Companion,
    World,
    Plugin,
    Achievement,
    WorldTemplate,
}

impl NftType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::Avatar => "avatar",
            Self::Companion => "companion",
            Self::World => "world",
            Self::Plugin => "plugin",
            Self::Achievement => "achievement",
            Self::WorldTemplate => "world_template",
        }
    }
}

impl fmt::Display for NftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub operators: Vec<Addr>,
    pub registry: Addr,
    pub ecosystem_id: String,
    pub collection_code_id: u64,
    pub enforce_local_allowlist: bool,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingCreation {
    pub request_id: u64,
    pub creator: Addr,
    pub ecosystem_id: String,
    pub name: String,
    pub symbol: String,
    pub nft_type: NftType,
    pub requested_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionRecord {
    pub id: u64,
    pub creator: Addr,
    pub collection_address: Addr,
    pub name: String,
    pub symbol: String,
    pub nft_type: NftType,
    pub created_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const APPROVED_CREATORS: Map<Addr, bool> = Map::new("approved_creators");
pub const PENDING_CREATIONS: Map<u64, PendingCreation> = Map::new("pending_creations");
pub const COLLECTIONS: Map<u64, CollectionRecord> = Map::new("collections");
pub const NEXT_REQUEST_ID: Item<u64> = Item::new("next_request_id");
pub const COLLECTION_COUNT: Item<u64> = Item::new("collection_count");
