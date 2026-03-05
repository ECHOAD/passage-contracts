use super::*;

// ========== Query ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),

        // Ecosystem queries
        QueryMsg::Ecosystem { id } => to_json_binary(&query_ecosystem(deps, id)?),
        QueryMsg::Ecosystems { start_after, limit } => {
            to_json_binary(&query_ecosystems(deps, start_after, limit)?)
        }
        QueryMsg::EcosystemsByAdmin {
            admin,
            start_after,
            limit,
        } => to_json_binary(&query_ecosystems_by_admin(deps, admin, start_after, limit)?),
        QueryMsg::IsEcosystemCreatorApproved { creator } => {
            to_json_binary(&query_is_ecosystem_creator_approved(deps, creator)?)
        }
        QueryMsg::IsEcosystemMember {
            ecosystem_id,
            member,
        } => to_json_binary(&query_is_ecosystem_member(deps, ecosystem_id, member)?),

        // Collection queries
        QueryMsg::Collection { address } => to_json_binary(&query_collection(deps, address)?),
        QueryMsg::Collections { start_after, limit } => {
            to_json_binary(&query_collections(deps, start_after, limit)?)
        }
        QueryMsg::CollectionsByEcosystem {
            ecosystem_id,
            start_after,
            limit,
        } => to_json_binary(&query_collections_by_ecosystem(
            deps,
            ecosystem_id,
            start_after,
            limit,
        )?),
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
        QueryMsg::IsCollectionVerified { address } => {
            to_json_binary(&query_is_collection_verified(deps, address)?)
        }

        // Minter queries
        QueryMsg::IsMinterAuthorized {
            collection_address,
            minter_address,
        } => to_json_binary(&query_is_minter_authorized(
            deps,
            collection_address,
            minter_address,
        )?),
        QueryMsg::AuthorizedMinters {
            collection_address,
            start_after,
            limit,
        } => to_json_binary(&query_authorized_minters(
            deps,
            collection_address,
            start_after,
            limit,
        )?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

fn query_ecosystem(deps: Deps, id: String) -> StdResult<EcosystemResponse> {
    let ecosystem = ECOSYSTEMS.may_load(deps.storage, id)?;
    Ok(EcosystemResponse { ecosystem })
}

fn query_ecosystems(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<EcosystemsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let ecosystems: Vec<Ecosystem> = ECOSYSTEMS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(EcosystemsResponse { ecosystems })
}

fn query_ecosystems_by_admin(
    deps: Deps,
    admin: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<EcosystemsResponse> {
    let admin_addr = deps.api.addr_validate(&admin)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let ecosystems: Vec<Ecosystem> = ECOSYSTEMS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok()
                .and_then(|(_, e)| if e.admin == admin_addr { Some(e) } else { None })
        })
        .take(limit)
        .collect();

    Ok(EcosystemsResponse { ecosystems })
}

fn query_is_ecosystem_creator_approved(
    deps: Deps,
    creator: String,
) -> StdResult<ApprovalStatusResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let approved = APPROVED_ECOSYSTEM_CREATORS.has(deps.storage, creator_addr);
    Ok(ApprovalStatusResponse { approved })
}

fn query_is_ecosystem_member(
    deps: Deps,
    ecosystem_id: String,
    member: String,
) -> StdResult<ApprovalStatusResponse> {
    let member_addr = deps.api.addr_validate(&member)?;
    let approved = ECOSYSTEM_MEMBERS.has(deps.storage, (ecosystem_id, member_addr));
    Ok(ApprovalStatusResponse { approved })
}

fn query_collection(deps: Deps, address: String) -> StdResult<CollectionResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let collection = collections().may_load(deps.storage, addr)?;
    Ok(CollectionResponse { collection })
}

fn query_collections(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?;
    let start_bound = start.map(Bound::exclusive);

    let collection_list: Vec<Collection> = collections()
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionsResponse {
        collections: collection_list,
    })
}

fn query_collections_by_ecosystem(
    deps: Deps,
    ecosystem_id: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?;
    let start_bound = start.map(Bound::exclusive);

    let collection_list: Vec<Collection> = collections()
        .idx
        .ecosystem
        .prefix(ecosystem_id)
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionsResponse {
        collections: collection_list,
    })
}

fn query_collections_by_creator(
    deps: Deps,
    creator: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?;
    let start_bound = start.map(Bound::exclusive);

    let collection_list: Vec<Collection> = collections()
        .idx
        .creator
        .prefix(creator_addr)
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionsResponse {
        collections: collection_list,
    })
}

fn query_is_collection_verified(deps: Deps, address: String) -> StdResult<IsVerifiedResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let collection = collections().may_load(deps.storage, addr)?;
    let is_verified = collection.map(|c| c.verified).unwrap_or(false);
    Ok(IsVerifiedResponse { is_verified })
}

fn query_is_minter_authorized(
    deps: Deps,
    collection_address: String,
    minter_address: String,
) -> StdResult<IsMinterAuthorizedResponse> {
    let collection_addr = deps.api.addr_validate(&collection_address)?;
    let minter_addr = deps.api.addr_validate(&minter_address)?;

    let key = (collection_addr, minter_addr);
    let is_authorized = AUTHORIZED_MINTERS.has(deps.storage, key);

    Ok(IsMinterAuthorizedResponse { is_authorized })
}

fn query_authorized_minters(
    deps: Deps,
    collection_address: String,
    _start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AuthorizedMintersResponse> {
    let collection_addr = deps.api.addr_validate(&collection_address)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let minters: Vec<Addr> = AUTHORIZED_MINTERS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|((coll, minter), _)| {
                if coll == collection_addr {
                    Some(minter)
                } else {
                    None
                }
            })
        })
        .take(limit)
        .collect();

    Ok(AuthorizedMintersResponse { minters })
}
