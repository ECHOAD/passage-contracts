use super::helpers::*;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::EcosystemCreationRequest { request_id } => {
            to_json_binary(&query_request(deps, request_id)?)
        }
        QueryMsg::EcosystemCreationRequests {
            status,
            start_after,
            limit,
        } => to_json_binary(&query_requests(deps, status, start_after, limit)?),
        QueryMsg::PendingRequestById { id } => to_json_binary(&query_pending_request(deps, id)?),
        QueryMsg::IsAdminOrOperator { address } => {
            to_json_binary(&query_is_admin_or_operator(deps, address)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

fn query_request(deps: Deps, request_id: u64) -> StdResult<EcosystemCreationRequestResponse> {
    let request = REQUESTS.may_load(deps.storage, request_id)?;
    Ok(EcosystemCreationRequestResponse { request })
}

fn query_requests(
    deps: Deps,
    status: Option<EcosystemCreationRequestStatus>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<EcosystemCreationRequestsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let requests = REQUESTS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, req)| match &status {
                Some(expected) if req.status != *expected => None,
                _ => Some(req),
            })
        })
        .take(limit)
        .collect::<Vec<_>>();

    Ok(EcosystemCreationRequestsResponse { requests })
}

fn query_pending_request(deps: Deps, id: String) -> StdResult<PendingRequestResponse> {
    let request_id = PENDING_REQUEST_BY_ID.may_load(deps.storage, id)?;
    Ok(PendingRequestResponse { request_id })
}

fn query_is_admin_or_operator(deps: Deps, address: String) -> StdResult<ApprovalStatusResponse> {
    let config = CONFIG.load(deps.storage)?;
    let address = deps.api.addr_validate(&address)?;
    Ok(ApprovalStatusResponse {
        approved: is_admin_or_operator(&config, &address),
    })
}
