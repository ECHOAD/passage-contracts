use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use pasg_governance::msg::{
    ConfigResponse, DepositResponse, ExecuteMsg, InstantiateMsg, PasgUtilityConfigResponse,
    ProposalResponse, ProposalsResponse, QueryMsg, RatifiedAdminActionResponse, VoteResponse,
    VotesResponse,
};
use std::fs::create_dir_all;
use std::path::PathBuf;

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(DepositResponse), &out_dir);
    export_schema(&schema_for!(PasgUtilityConfigResponse), &out_dir);
    export_schema(&schema_for!(ProposalResponse), &out_dir);
    export_schema(&schema_for!(ProposalsResponse), &out_dir);
    export_schema(&schema_for!(VoteResponse), &out_dir);
    export_schema(&schema_for!(VotesResponse), &out_dir);
    export_schema(&schema_for!(RatifiedAdminActionResponse), &out_dir);
}
