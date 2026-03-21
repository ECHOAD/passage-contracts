#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::{assignment_key, is_token_authority, validate_asset_ref};
use crate::msg::{
    AssetRef, AssignmentRecordResponse, AssignmentsByPluginResponse, AssignmentsByWorldResponse,
    ExecuteMsg, InstantiateMsg, QueryMsg,
};
use crate::state::{AssetKey, AssignmentRecord, Config, ASSIGNMENTS, CONFIG};

const CONTRACT_NAME: &str = "crates.io:world-plugin-assignment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            created_at: env.block.time.seconds(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Assign { plugin, world } => execute_assign(deps, env, info, plugin, world),
        ExecuteMsg::Remove { plugin, world } => execute_remove(deps, info, plugin, world),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Assignment { plugin, world } => {
            to_json_binary(&query_assignment(deps, plugin, world)?)
        }
        QueryMsg::AssignmentsByWorld { world } => {
            to_json_binary(&query_assignments_by_world(deps, world)?)
        }
        QueryMsg::AssignmentsByPlugin { plugin } => {
            to_json_binary(&query_assignments_by_plugin(deps, plugin)?)
        }
    }
}

fn execute_assign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    plugin: AssetRef,
    world: AssetRef,
) -> Result<Response, ContractError> {
    let (plugin_collection, plugin_token_id) = validate_asset_ref(deps.as_ref(), &plugin)?;
    let (world_collection, world_token_id) = validate_asset_ref(deps.as_ref(), &world)?;

    let plugin_allowed =
        is_token_authority(deps.as_ref(), &info.sender, &plugin_collection, &plugin_token_id)?;
    let world_allowed =
        is_token_authority(deps.as_ref(), &info.sender, &world_collection, &world_token_id)?;

    if !(plugin_allowed && world_allowed) {
        return Err(ContractError::Unauthorized {});
    }

    let record = AssignmentRecord {
        plugin: AssetKey {
            collection: plugin_collection.to_string(),
            token_id: plugin_token_id.clone(),
        },
        world: AssetKey {
            collection: world_collection.to_string(),
            token_id: world_token_id.clone(),
        },
        assigned_by: info.sender.to_string(),
        assigned_at: env.block.time.seconds(),
    };
    ASSIGNMENTS.save(
        deps.storage,
        assignment_key(
            plugin_collection.as_str(),
            &plugin_token_id,
            world_collection.as_str(),
            &world_token_id,
        ),
        &record,
    )?;

    Ok(Response::new()
        .add_attribute("action", "assign_plugin_to_world")
        .add_attribute("plugin_collection", plugin_collection.to_string())
        .add_attribute("plugin_token_id", plugin_token_id)
        .add_attribute("world_collection", world_collection.to_string())
        .add_attribute("world_token_id", world_token_id)
        .add_attribute("assigned_by", info.sender))
}

fn execute_remove(
    deps: DepsMut,
    info: MessageInfo,
    plugin: AssetRef,
    world: AssetRef,
) -> Result<Response, ContractError> {
    let (plugin_collection, plugin_token_id) = validate_asset_ref(deps.as_ref(), &plugin)?;
    let (world_collection, world_token_id) = validate_asset_ref(deps.as_ref(), &world)?;

    let key = assignment_key(
        plugin_collection.as_str(),
        &plugin_token_id,
        world_collection.as_str(),
        &world_token_id,
    );

    let record = ASSIGNMENTS
        .may_load(deps.storage, key.clone())?
        .ok_or(ContractError::AssignmentNotFound {})?;

    if record.assigned_by != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    ASSIGNMENTS.remove(deps.storage, key);

    Ok(Response::new()
        .add_attribute("action", "remove_plugin_assignment")
        .add_attribute("plugin_collection", plugin_collection.to_string())
        .add_attribute("plugin_token_id", plugin_token_id)
        .add_attribute("world_collection", world_collection.to_string())
        .add_attribute("world_token_id", world_token_id))
}

fn query_assignment(
    deps: Deps,
    plugin: AssetRef,
    world: AssetRef,
) -> StdResult<Option<AssignmentRecordResponse>> {
    let (plugin_collection, plugin_token_id) = validate_asset_ref(deps, &plugin)?;
    let (world_collection, world_token_id) = validate_asset_ref(deps, &world)?;

    ASSIGNMENTS
        .may_load(
            deps.storage,
            assignment_key(
                plugin_collection.as_str(),
                &plugin_token_id,
                world_collection.as_str(),
                &world_token_id,
            ),
        )
        .map(|maybe| maybe.map(assignment_to_response))
}

fn query_assignments_by_world(
    deps: Deps,
    world: AssetRef,
) -> StdResult<AssignmentsByWorldResponse> {
    let (world_collection, world_token_id) = validate_asset_ref(deps, &world)?;
    let assignments = ASSIGNMENTS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(Result::ok)
        .map(|(_, record)| record)
        .filter(|record| {
            record.world.collection == world_collection.as_str()
                && record.world.token_id == world_token_id
        })
        .map(assignment_to_response)
        .collect();

    Ok(AssignmentsByWorldResponse { world, assignments })
}

fn query_assignments_by_plugin(
    deps: Deps,
    plugin: AssetRef,
) -> StdResult<AssignmentsByPluginResponse> {
    let (plugin_collection, plugin_token_id) = validate_asset_ref(deps, &plugin)?;
    let assignments = ASSIGNMENTS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(Result::ok)
        .map(|(_, record)| record)
        .filter(|record| {
            record.plugin.collection == plugin_collection.as_str()
                && record.plugin.token_id == plugin_token_id
        })
        .map(assignment_to_response)
        .collect();

    Ok(AssignmentsByPluginResponse {
        plugin,
        assignments,
    })
}

fn assignment_to_response(record: AssignmentRecord) -> AssignmentRecordResponse {
    AssignmentRecordResponse {
        plugin: AssetRef {
            collection: record.plugin.collection,
            token_id: record.plugin.token_id,
        },
        world: AssetRef {
            collection: record.world.collection,
            token_id: record.world.token_id,
        },
        assigned_by: record.assigned_by,
        assigned_at: record.assigned_at,
    }
}
