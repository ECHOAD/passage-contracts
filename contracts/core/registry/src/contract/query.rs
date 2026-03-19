use super::helpers::{
    can_create_collection_in_ecosystem, can_create_ecosystem, can_mint_collection,
    can_trade_collection, collection_moderation, collection_recovery_policy, creator_moderation,
    ecosystem_moderation, ecosystem_recovery_policy, is_cross_ecosystem_admin,
};
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
        QueryMsg::CreatorModeration { creator } => {
            to_json_binary(&query_creator_moderation(deps, creator)?)
        }
        QueryMsg::EcosystemModeration { ecosystem_id } => {
            to_json_binary(&query_ecosystem_moderation(deps, ecosystem_id)?)
        }
        QueryMsg::CollectionModeration { address } => {
            to_json_binary(&query_collection_moderation(deps, address)?)
        }
        QueryMsg::EcosystemRecoveryPolicy { ecosystem_id } => {
            to_json_binary(&query_ecosystem_recovery_policy(deps, ecosystem_id)?)
        }
        QueryMsg::CollectionRecoveryPolicy { address } => {
            to_json_binary(&query_collection_recovery_policy(deps, address)?)
        }
        QueryMsg::CanCreateEcosystem { creator } => {
            to_json_binary(&query_can_create_ecosystem(deps, creator)?)
        }
        QueryMsg::IsCrossEcosystemAdmin { address } => {
            to_json_binary(&query_is_cross_ecosystem_admin(deps, address)?)
        }
        QueryMsg::IsEcosystemMember {
            ecosystem_id,
            member,
        } => to_json_binary(&query_is_ecosystem_member(deps, ecosystem_id, member)?),
        QueryMsg::CanCreateCollectionInEcosystem {
            ecosystem_id,
            creator,
        } => to_json_binary(&query_can_create_collection_in_ecosystem(
            deps,
            ecosystem_id,
            creator,
        )?),
        QueryMsg::CollectionCreationRequest {
            ecosystem_id,
            creator,
        } => to_json_binary(&query_collection_creation_request(
            deps,
            ecosystem_id,
            creator,
        )?),
        QueryMsg::CollectionCreationRequests {
            ecosystem_id,
            status,
            start_after_creator,
            limit,
        } => to_json_binary(&query_collection_creation_requests(
            deps,
            ecosystem_id,
            status,
            start_after_creator,
            limit,
        )?),

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
        QueryMsg::CollectionsByNftType {
            nft_type,
            start_after,
            limit,
        } => to_json_binary(&query_collections_by_nft_type(
            deps,
            nft_type,
            start_after,
            limit,
        )?),
        QueryMsg::IsCollectionVerified { address } => {
            to_json_binary(&query_is_collection_verified(deps, address)?)
        }
        QueryMsg::CanMintCollection { address } => {
            to_json_binary(&query_can_mint_collection(deps, address)?)
        }
        QueryMsg::CanTradeCollection { address } => {
            to_json_binary(&query_can_trade_collection(deps, address)?)
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

        QueryMsg::RecoveryConfig {} => to_json_binary(&query_recovery_config(deps)?),
        QueryMsg::RecoveryCase { case_id } => to_json_binary(&query_recovery_case(deps, case_id)?),
        QueryMsg::RecoveryCases {
            status,
            start_after,
            limit,
        } => to_json_binary(&query_recovery_cases(deps, status, start_after, limit)?),
        QueryMsg::LastCreatorActivity { creator } => {
            to_json_binary(&query_last_creator_activity(deps, creator)?)
        }
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

fn query_creator_moderation(deps: Deps, creator: String) -> StdResult<CreatorModerationResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let moderation = creator_moderation(deps.storage, &creator_addr)?;
    Ok(CreatorModerationResponse {
        creator,
        moderation,
    })
}

fn query_ecosystem_moderation(
    deps: Deps,
    ecosystem_id: String,
) -> StdResult<EcosystemModerationResponse> {
    let moderation = ecosystem_moderation(deps.storage, &ecosystem_id)?;
    Ok(EcosystemModerationResponse {
        ecosystem_id,
        moderation,
    })
}

fn query_collection_moderation(
    deps: Deps,
    address: String,
) -> StdResult<CollectionModerationResponse> {
    let collection_addr = deps.api.addr_validate(&address)?;
    let moderation = collection_moderation(deps.storage, &collection_addr)?;
    Ok(CollectionModerationResponse {
        address,
        moderation,
    })
}

fn query_ecosystem_recovery_policy(
    deps: Deps,
    ecosystem_id: String,
) -> StdResult<RecoveryPolicyResponse> {
    let policy = ecosystem_recovery_policy(deps.storage, &ecosystem_id)?;
    Ok(RecoveryPolicyResponse { policy })
}

fn query_collection_recovery_policy(
    deps: Deps,
    address: String,
) -> StdResult<RecoveryPolicyResponse> {
    let collection_addr = deps.api.addr_validate(&address)?;
    let policy = collection_recovery_policy(deps.storage, &collection_addr)?;
    Ok(RecoveryPolicyResponse { policy })
}

fn query_can_create_ecosystem(deps: Deps, creator: String) -> StdResult<ApprovalStatusResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let approved = can_create_ecosystem(deps.storage, &creator_addr)?;
    Ok(ApprovalStatusResponse { approved })
}

fn query_is_cross_ecosystem_admin(
    deps: Deps,
    address: String,
) -> StdResult<ApprovalStatusResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let config = CONFIG.load(deps.storage)?;
    let approved = is_cross_ecosystem_admin(&config, &addr);
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

fn query_can_create_collection_in_ecosystem(
    deps: Deps,
    ecosystem_id: String,
    creator: String,
) -> StdResult<ApprovalStatusResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let config = CONFIG.load(deps.storage)?;
    let ecosystem = ECOSYSTEMS
        .may_load(deps.storage, ecosystem_id)?
        .ok_or_else(|| cosmwasm_std::StdError::generic_err("ecosystem not found"))?;

    let approved =
        can_create_collection_in_ecosystem(deps.storage, &config, &ecosystem, &creator_addr);
    Ok(ApprovalStatusResponse { approved })
}

fn query_collection_creation_request(
    deps: Deps,
    ecosystem_id: String,
    creator: String,
) -> StdResult<CollectionCreationRequestResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let request =
        COLLECTION_CREATION_REQUESTS.may_load(deps.storage, (ecosystem_id, creator_addr))?;
    Ok(CollectionCreationRequestResponse { request })
}

fn query_collection_creation_requests(
    deps: Deps,
    ecosystem_id: String,
    status: Option<CollectionCreationRequestStatus>,
    start_after_creator: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionCreationRequestsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_creator = start_after_creator
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?;

    let requests = COLLECTION_CREATION_REQUESTS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            item.ok()
                .and_then(|((request_ecosystem_id, creator_addr), request)| {
                    if request_ecosystem_id != ecosystem_id {
                        return None;
                    }

                    if let Some(start) = &start_creator {
                        if creator_addr <= *start {
                            return None;
                        }
                    }

                    if let Some(expected_status) = &status {
                        if &request.status != expected_status {
                            return None;
                        }
                    }

                    Some(request)
                })
        })
        .take(limit)
        .collect::<Vec<_>>();

    Ok(CollectionCreationRequestsResponse { requests })
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

fn query_collections_by_nft_type(
    deps: Deps,
    nft_type: NftType,
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
        .nft_type
        .prefix(nft_type.as_str().to_string())
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

fn query_can_mint_collection(deps: Deps, address: String) -> StdResult<ApprovalStatusResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let collection = collections()
        .may_load(deps.storage, addr)?
        .ok_or_else(|| cosmwasm_std::StdError::generic_err("collection not found"))?;
    let approved = can_mint_collection(deps.storage, &collection)?;
    Ok(ApprovalStatusResponse { approved })
}

fn query_can_trade_collection(deps: Deps, address: String) -> StdResult<ApprovalStatusResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let collection = collections()
        .may_load(deps.storage, addr)?
        .ok_or_else(|| cosmwasm_std::StdError::generic_err("collection not found"))?;
    let approved = can_trade_collection(deps.storage, &collection)?;
    Ok(ApprovalStatusResponse { approved })
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
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AuthorizedMintersResponse> {
    let collection_addr = deps.api.addr_validate(&collection_address)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|minter| deps.api.addr_validate(&minter))
        .transpose()?
        .map(Bound::exclusive);

    let minters: Vec<Addr> = AUTHORIZED_MINTERS
        .prefix(collection_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .map(|item| item.map(|(minter, _)| minter))
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AuthorizedMintersResponse { minters })
}

fn query_recovery_config(deps: Deps) -> StdResult<RecoveryConfigResponse> {
    let config = RECOVERY_CONFIG.load(deps.storage)?;
    Ok(RecoveryConfigResponse { config })
}

fn query_recovery_case(deps: Deps, case_id: u64) -> StdResult<RecoveryCaseResponse> {
    let case = RECOVERY_CASES.may_load(deps.storage, case_id)?;
    Ok(RecoveryCaseResponse { case })
}

fn query_recovery_cases(
    deps: Deps,
    status: Option<RecoveryCaseStatus>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<RecoveryCasesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let cases: Vec<RecoveryCase> = RECOVERY_CASES
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, c)| match &status {
                Some(s) if c.status != *s => None,
                _ => Some(c),
            })
        })
        .take(limit)
        .collect();

    Ok(RecoveryCasesResponse { cases })
}

fn query_last_creator_activity(
    deps: Deps,
    creator: String,
) -> StdResult<LastCreatorActivityResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let last_activity_at = LAST_CREATOR_ACTIVITY.may_load(deps.storage, creator_addr)?;
    Ok(LastCreatorActivityResponse {
        creator,
        last_activity_at,
    })
}
