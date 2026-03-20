#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SnapshotResponse, SnapshotsResponse},
    state::{Config, SnapshotRecord, CONFIG, SNAPSHOTS, WORLD_SNAPSHOTS},
};

const CONTRACT_NAME: &str = "crates.io:asset-progression";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_LIMIT: u32 = 30;
const MAX_LIMIT: u32 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = deps.api.addr_validate(&msg.admin)?;
    CONFIG.save(deps.storage, &Config { admin: admin.clone() })?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::Unauthorized {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&ConfigResponse {
            admin: CONFIG.load(deps.storage)?.admin.to_string(),
        }),
        QueryMsg::Snapshot {
            collection,
            token_id,
            world,
        } => {
            let collection = deps.api.addr_validate(&collection)?;
            let snapshot = SNAPSHOTS
                .may_load(deps.storage, (collection, token_id, world))?;
            to_json_binary(&SnapshotResponse { snapshot })
        }
        QueryMsg::SnapshotsByAsset {
            collection,
            token_id,
            start_after_world,
            limit,
        } => {
            let collection = deps.api.addr_validate(&collection)?;
            let start = start_after_world.map(cosmwasm_std::Bound::exclusive);
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let snapshots: StdResult<Vec<SnapshotRecord>> = SNAPSHOTS
                .prefix((collection, token_id))
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| item.map(|(_, snapshot)| snapshot))
                .collect();

            to_json_binary(&SnapshotsResponse {
                snapshots: snapshots?,
            })
        }
        QueryMsg::SnapshotsByWorld {
            world,
            start_after,
            limit,
        } => {
            let start = start_after.map(|cursor| {
                let collection = deps.api.addr_validate(&cursor.collection).unwrap();
                cosmwasm_std::Bound::exclusive((collection, cursor.token_id))
            });
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let snapshots: StdResult<Vec<SnapshotRecord>> = WORLD_SNAPSHOTS
                .prefix(world)
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| {
                    let ((collection, token_id), _) = item?;
                    SNAPSHOTS.load(deps.storage, (collection, token_id, String::new()))
                })
                .collect();

            to_json_binary(&SnapshotsResponse {
                snapshots: snapshots?,
            })
        }
    }
}

#[cfg(test)]
mod tests;
