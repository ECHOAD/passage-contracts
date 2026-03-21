use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct AssetRef {
    pub collection: String,
    pub token_id: String,
}

#[cw_serde]
pub struct AssignmentRecordResponse {
    pub plugin: AssetRef,
    pub world: AssetRef,
    pub assigned_by: String,
    pub assigned_at: u64,
}

#[cw_serde]
pub struct AssignmentsByWorldResponse {
    pub world: AssetRef,
    pub assignments: Vec<AssignmentRecordResponse>,
}

#[cw_serde]
pub struct AssignmentsByPluginResponse {
    pub plugin: AssetRef,
    pub assignments: Vec<AssignmentRecordResponse>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Assign {
        plugin: AssetRef,
        world: AssetRef,
    },
    Remove {
        plugin: AssetRef,
        world: AssetRef,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Assignment {
        plugin: AssetRef,
        world: AssetRef,
    },
    AssignmentsByWorld {
        world: AssetRef,
    },
    AssignmentsByPlugin {
        plugin: AssetRef,
    },
}
