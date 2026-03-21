use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub created_at: u64,
}

#[cw_serde]
pub struct AssetKey {
    pub collection: String,
    pub token_id: String,
}

#[cw_serde]
pub struct AssignmentRecord {
    pub plugin: AssetKey,
    pub world: AssetKey,
    pub assigned_by: String,
    pub assigned_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ASSIGNMENTS: Map<String, AssignmentRecord> = Map::new("assignment");
