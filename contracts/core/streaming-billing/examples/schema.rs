use std::fs::create_dir_all;
use std::path::PathBuf;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use streaming_billing::msg::{
    ConfigResponse, ConversionRateResponse, ExecuteMsg, InstantiateMsg, PasgUtilityResponse,
    PendingRevenueResponse, PlatformStatsResponse, PreviewWorldSettlementResponse, QueryMsg,
    UserBalanceResponse, WorldConfigResponse, WorldLocalEconomyResponse, WorldStatsResponse,
};

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(UserBalanceResponse), &out_dir);
    export_schema(&schema_for!(WorldConfigResponse), &out_dir);
    export_schema(&schema_for!(WorldStatsResponse), &out_dir);
    export_schema(&schema_for!(PendingRevenueResponse), &out_dir);
    export_schema(&schema_for!(WorldLocalEconomyResponse), &out_dir);
    export_schema(&schema_for!(PreviewWorldSettlementResponse), &out_dir);
    export_schema(&schema_for!(ConversionRateResponse), &out_dir);
    export_schema(&schema_for!(PasgUtilityResponse), &out_dir);
    export_schema(&schema_for!(PlatformStatsResponse), &out_dir);
}
