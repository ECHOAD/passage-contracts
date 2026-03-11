use super::helpers::calculate_split_amounts;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::SplitRule { key } => to_json_binary(&query_split_rule(deps, key)?),
        QueryMsg::SplitRules { start_after, limit } => {
            to_json_binary(&query_split_rules(deps, start_after, limit)?)
        }
        QueryMsg::SplitRulesByOwner {
            owner,
            start_after,
            limit,
        } => to_json_binary(&query_split_rules_by_owner(
            deps,
            owner,
            start_after,
            limit,
        )?),
        QueryMsg::SplitEvents {
            key,
            start_after,
            limit,
        } => to_json_binary(&query_split_events(deps, key, start_after, limit)?),
        QueryMsg::PreviewSplit { key, funds } => {
            to_json_binary(&query_preview_split(deps, key, funds)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

fn query_split_rule(deps: Deps, key: String) -> StdResult<SplitRuleResponse> {
    Ok(SplitRuleResponse {
        rule: SPLIT_RULES.may_load(deps.storage, key.as_str())?,
    })
}

fn query_split_rules(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<SplitRulesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_deref().map(Bound::exclusive);

    let rules: Vec<SplitRule> = SPLIT_RULES
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, rule)| rule))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(SplitRulesResponse { rules })
}

fn query_split_rules_by_owner(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<SplitRulesResponse> {
    let owner = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_deref().map(Bound::exclusive);

    let rules: Vec<SplitRule> = SPLIT_RULES
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| match item {
            Ok((_key, rule)) if rule.owner == owner => Some(Ok(rule)),
            Ok(_) => None,
            Err(err) => Some(Err(err)),
        })
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(SplitRulesResponse { rules })
}

fn query_split_events(
    deps: Deps,
    key: Option<String>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<SplitEventsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let events: Vec<SplitEvent> = SPLIT_EVENTS
        .range(deps.storage, start, None, Order::Descending)
        .filter_map(|item| match item {
            Ok((_id, event)) => {
                if key
                    .as_ref()
                    .map(|expected| &event.key == expected)
                    .unwrap_or(true)
                {
                    Some(Ok(event))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        })
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(SplitEventsResponse { events })
}

fn query_preview_split(
    deps: Deps,
    key: String,
    funds: Vec<Coin>,
) -> StdResult<SplitPreviewResponse> {
    let rule = SPLIT_RULES
        .may_load(deps.storage, key.as_str())?
        .ok_or_else(|| StdError::generic_err(format!("split rule not found for key: {key}")))?;

    let recipient_amounts = calculate_split_amounts(&rule.recipients, &funds)
        .into_iter()
        .map(|(address, funds)| (address.to_string(), funds))
        .collect();

    Ok(SplitPreviewResponse {
        total_funds: funds,
        recipient_amounts,
    })
}

#[cfg(test)]
mod tests;
