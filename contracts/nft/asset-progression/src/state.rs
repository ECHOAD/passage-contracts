use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub enum AssetKind {
    Avatar,
    Companion,
}

impl AssetKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Avatar => "avatar",
            Self::Companion => "companion",
        }
    }
}

#[cw_serde]
pub struct ProgressionSnapshot {
    pub level: u32,
    pub xp: u64,
    pub checkpoint: Option<String>,
    pub saved_at: Timestamp,
}

#[cw_serde]
pub struct SnapshotRecord {
    pub collection: Addr,
    pub token_id: String,
    pub asset_kind: AssetKind,
    pub world: String,
    pub snapshot: ProgressionSnapshot,
    pub updated_by: Addr,
    pub updated_at: Timestamp,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const SNAPSHOTS: Map<(Addr, String, String), SnapshotRecord> = Map::new("snapshots");
pub const WORLD_SNAPSHOTS: Map<(String, Addr, String), bool> = Map::new("world_snapshots");
