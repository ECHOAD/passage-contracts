use super::helpers::calculate_split_amounts;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::SplitConfig {} => to_json_binary(&query_split_config(deps)?),
        QueryMsg::SplitEvents { start_after, limit } => {
            to_json_binary(&query_split_events(deps, start_after, limit)?)
        }
        QueryMsg::PreviewSplit { funds } => to_json_binary(&query_preview_split(deps, funds)?),
        QueryMsg::RoutingMetadata {} => to_json_binary(&query_routing_metadata()),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

fn query_split_config(deps: Deps) -> StdResult<SplitConfigResponse> {
    Ok(SplitConfigResponse {
        split: SPLIT_CONFIG.load(deps.storage)?,
    })
}

fn query_split_events(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<SplitEventsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let events: Vec<SplitEvent> = SPLIT_EVENTS
        .range(deps.storage, start, None, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, event)| event))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(SplitEventsResponse { events })
}

fn query_preview_split(deps: Deps, funds: Vec<Coin>) -> StdResult<SplitPreviewResponse> {
    let split = SPLIT_CONFIG.load(deps.storage)?;
    let recipient_amounts = calculate_split_amounts(&split.recipients, &funds)
        .into_iter()
        .map(|(address, funds)| (address.to_string(), funds))
        .collect();

    Ok(SplitPreviewResponse {
        total_funds: funds,
        recipient_amounts,
    })
}

fn query_routing_metadata() -> RoutingMetadataResponse {
    RoutingMetadataResponse {
        forwards_attached_funds: true,
        preserves_input_denoms: true,
        preview_query: RoutingPreviewRoute::PreviewSplit,
        execute_routes: vec![
            RoutingExecuteRoute::Split,
            RoutingExecuteRoute::RouteWorldRevenue,
        ],
    }
}

#[cfg(test)]
mod tests;
