use cosmwasm_std::{to_json_binary, Addr, Deps, QueryRequest, StdResult, WasmQuery};
use cw721::{Cw721QueryMsg, OwnerOfResponse};

use crate::msg::AssetRef;

pub fn validate_asset_ref(deps: Deps, asset: &AssetRef) -> StdResult<(Addr, String)> {
    let collection = deps.api.addr_validate(&asset.collection)?;
    Ok((collection, asset.token_id.clone()))
}

pub fn assignment_key(plugin_collection: &str, plugin_token_id: &str, world_collection: &str, world_token_id: &str) -> String {
    format!("{plugin_collection}::{plugin_token_id}::{world_collection}::{world_token_id}")
}

pub fn query_token_authority(
    deps: Deps,
    collection: &Addr,
    token_id: &str,
) -> StdResult<OwnerOfResponse> {
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&Cw721QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: Some(false),
        })?,
    }))
}

pub fn is_token_authority(
    deps: Deps,
    sender: &Addr,
    collection: &Addr,
    token_id: &str,
) -> StdResult<bool> {
    let owner = query_token_authority(deps, collection, token_id)?;
    if owner.owner == sender.as_str() {
        return Ok(true);
    }

    Ok(owner
        .approvals
        .iter()
        .any(|approval| approval.spender == sender.as_str()))
}
