#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    helpers::assert_sender_can_save_snapshot,
    msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SnapshotResponse, SnapshotsResponse},
    state::{AssetKind, Config, ProgressionSnapshot, SnapshotRecord, CONFIG, SNAPSHOTS},
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SaveSnapshot {
            collection,
            token_id,
            asset_kind,
            world,
            snapshot,
        } => execute_save_snapshot(deps, env, info, collection, token_id, asset_kind, world, snapshot),
        ExecuteMsg::UpdateAdmin { admin } => execute_update_admin(deps, info, admin),
    }
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
            let snapshot = SNAPSHOTS.may_load(deps.storage, (collection, token_id, world))?;
            to_json_binary(&SnapshotResponse { snapshot })
        }
        QueryMsg::SnapshotsByAsset {
            collection,
            token_id,
            start_after_world,
            limit,
        } => {
            let collection = deps.api.addr_validate(&collection)?;
            let start = start_after_world.map(Bound::exclusive);
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
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let snapshots: StdResult<Vec<SnapshotRecord>> = SNAPSHOTS
                .range(deps.storage, None, None, Order::Ascending)
                .filter_map(|item| match item {
                    Ok((_, snapshot)) if snapshot.world == world => Some(Ok(snapshot)),
                    Ok((_, snapshot)) => {
                        if let Some(cursor) = &start_after {
                            let after = snapshot.collection.as_str() > cursor.collection.as_str()
                                || (snapshot.collection.as_str() == cursor.collection.as_str()
                                    && snapshot.token_id.as_str() > cursor.token_id.as_str());
                            if after {
                                Some(Ok(snapshot))
                            } else {
                                None
                            }
                        } else {
                            Some(Ok(snapshot))
                        }
                    }
                    Err(err) => Some(Err(err)),
                })
                .take(limit)
                .collect();

            to_json_binary(&SnapshotsResponse {
                snapshots: snapshots?,
            })
        }
    }
}

fn execute_save_snapshot(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
    asset_kind: AssetKind,
    world: String,
    snapshot: ProgressionSnapshot,
) -> Result<Response, ContractError> {
    if world.trim().is_empty() {
        return Err(ContractError::EmptyWorld {});
    }

    let collection = deps.api.addr_validate(&collection)?;
    assert_sender_can_save_snapshot(
        deps.as_ref(),
        &env,
        &collection,
        &token_id,
        &asset_kind,
        &info.sender,
    )?;

    let record = SnapshotRecord {
        collection: collection.clone(),
        token_id: token_id.clone(),
        asset_kind,
        world: world.clone(),
        snapshot,
        updated_by: info.sender.clone(),
        updated_at: env.block.time,
    };

    SNAPSHOTS.save(
        deps.storage,
        (collection.clone(), token_id.clone(), world.clone()),
        &record,
    )?;

    Ok(Response::new()
        .add_attribute("action", "save_snapshot")
        .add_attribute("collection", collection)
        .add_attribute("token_id", token_id)
        .add_attribute("world", world)
        .add_attribute("updated_by", info.sender))
}

fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin: String,
) -> Result<Response, ContractError> {
    let next_admin = deps.api.addr_validate(&admin)?;
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        if config.admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }
        config.admin = next_admin.clone();
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_admin")
        .add_attribute("admin", next_admin))
}
