use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use multisig::msg;
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(msg::InstantiateMsg), &out_dir);
    export_schema(&schema_for!(msg::ExecuteMsg), &out_dir);
    export_schema(&schema_for!(msg::QueryMsg), &out_dir);
}
