use auction_english::msg;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(msg::InstantiateMsg), &out_dir);
    export_schema(&schema_for!(msg::ExecuteMsg), &out_dir);
    export_schema(&schema_for!(msg::QueryMsg), &out_dir);
    export_schema(&schema_for!(msg::ConfigResponse), &out_dir);
    export_schema(&schema_for!(msg::CanTradeResponse), &out_dir);
    export_schema(&schema_for!(msg::AuctionResponse), &out_dir);
    export_schema(&schema_for!(msg::AuctionsResponse), &out_dir);
}
