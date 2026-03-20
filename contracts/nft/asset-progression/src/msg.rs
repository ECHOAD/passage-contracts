use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{AssetKind, ProgressionSnapshot, SnapshotRecord};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SaveSnapshot {
        collection: String,
        token_id: String,
        asset_kind: AssetKind,
        world: String,
        snapshot: ProgressionSnapshot,
    },
    UpdateAdmin {
        admin: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SnapshotResponse)]
    Snapshot {
        collection: String,
        token_id: String,
        world: String,
    },
    #[returns(SnapshotsResponse)]
    SnapshotsByAsset {
        collection: String,
        token_id: String,
        start_after_world: Option<String>,
        limit: Option<u32>,
    },
    #[returns(SnapshotsResponse)]
    SnapshotsByWorld {
        world: String,
        start_after: Option<SnapshotCursor>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct SnapshotCursor {
    pub collection: String,
    pub token_id: String,
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
}

#[cw_serde]
pub struct SnapshotResponse {
    pub snapshot: Option<SnapshotRecord>,
}

#[cw_serde]
pub struct SnapshotsResponse {
    pub snapshots: Vec<SnapshotRecord>,
}
