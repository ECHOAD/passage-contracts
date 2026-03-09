use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::IsCreatorApproved { creator } => {
            to_json_binary(&query_is_creator_approved(deps, creator)?)
        }
        QueryMsg::ApprovedCreators { start_after, limit } => {
            to_json_binary(&query_approved_creators(deps, start_after, limit)?)
        }
        QueryMsg::Collection { id } => to_json_binary(&query_collection(deps, id)?),
        QueryMsg::Collections { start_after, limit } => {
            to_json_binary(&query_collections(deps, start_after, limit)?)
        }
        QueryMsg::CollectionsByCreator {
            creator,
            start_after,
            limit,
        } => to_json_binary(&query_collections_by_creator(
            deps,
            creator,
            start_after,
            limit,
        )?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

fn query_is_creator_approved(deps: Deps, creator: String) -> StdResult<ApprovalStatusResponse> {
    let creator = deps.api.addr_validate(&creator)?;
    let approved = APPROVED_CREATORS.has(deps.storage, creator);
    Ok(ApprovalStatusResponse { approved })
}

fn query_approved_creators(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ApprovedCreatorsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?
        .map(Bound::exclusive);

    let creators = APPROVED_CREATORS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .filter_map(|item| item.ok().map(|(addr, _)| addr.to_string()))
        .collect::<Vec<_>>();

    Ok(ApprovedCreatorsResponse { creators })
}

fn query_collection(deps: Deps, id: u64) -> StdResult<CollectionResponse> {
    let collection = COLLECTIONS.may_load(deps.storage, id)?;
    Ok(CollectionResponse { collection })
}

fn query_collections(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let collections = COLLECTIONS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, collection)| collection))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionsResponse { collections })
}

fn query_collections_by_creator(
    deps: Deps,
    creator: String,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let creator = deps.api.addr_validate(&creator)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let collections = COLLECTIONS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, collection)| {
                if collection.creator == creator {
                    Some(collection)
                } else {
                    None
                }
            })
        })
        .take(limit)
        .collect::<Vec<_>>();

    Ok(CollectionsResponse { collections })
}
