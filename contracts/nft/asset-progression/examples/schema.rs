use std::fs::create_dir_all;
use std::path::PathBuf;

use asset_progression::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SnapshotResponse, SnapshotsResponse,
};
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(SnapshotResponse), &out_dir);
    export_schema(&schema_for!(SnapshotsResponse), &out_dir);
}
