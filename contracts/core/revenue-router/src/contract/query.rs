use super::*;

// ========== Query ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::DistributionRule { collection } => {
            to_json_binary(&query_distribution_rule(deps, collection)?)
        }
        QueryMsg::DistributionRules { start_after, limit } => {
            to_json_binary(&query_distribution_rules(deps, start_after, limit)?)
        }
        QueryMsg::EcosystemConfig { ecosystem_id } => {
            to_json_binary(&query_ecosystem_config(deps, ecosystem_id)?)
        }
        QueryMsg::PreviewDistribution {
            collection,
            amount,
            event_type,
        } => to_json_binary(&query_preview_distribution(
            deps, collection, amount, event_type,
        )?),
        QueryMsg::CollectionStats { collection } => {
            to_json_binary(&query_collection_stats(deps, collection)?)
        }
        QueryMsg::RevenueEvents {
            collection,
            start_after,
            limit,
        } => to_json_binary(&query_revenue_events(deps, collection, start_after, limit)?),
        QueryMsg::SplitWallet { id } => to_json_binary(&query_split_wallet(deps, id)?),
        QueryMsg::SplitWallets { start_after, limit } => {
            to_json_binary(&query_split_wallets(deps, start_after, limit)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

fn query_distribution_rule(deps: Deps, collection: String) -> StdResult<DistributionRuleResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let rule = DISTRIBUTION_RULES.may_load(deps.storage, collection_addr)?;
    Ok(DistributionRuleResponse { rule })
}

fn query_distribution_rules(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<DistributionRulesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?;
    let start_bound = start.map(Bound::exclusive);

    let rules: Vec<DistributionRule> = DISTRIBUTION_RULES
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(DistributionRulesResponse { rules })
}

fn query_ecosystem_config(deps: Deps, ecosystem_id: String) -> StdResult<EcosystemConfigResponse> {
    let config = ECOSYSTEM_CONFIGS.may_load(deps.storage, ecosystem_id)?;
    Ok(EcosystemConfigResponse { config })
}

fn query_preview_distribution(
    deps: Deps,
    collection: String,
    amount: Uint128,
    _event_type: RevenueEventType,
) -> StdResult<DistributionPreviewResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    let rule = DISTRIBUTION_RULES.may_load(deps.storage, collection_addr)?;

    match rule {
        Some(r) => {
            let platform_fee_rate = r.platform_fee.unwrap_or(config.default_platform_fee);
            let platform_fee = amount.multiply_ratio(
                platform_fee_rate.atomics().u128(),
                10u128.pow(Decimal::DECIMAL_PLACES),
            );
            let remaining = amount - platform_fee;

            let mut collaborator_amounts: Vec<(Addr, Uint128)> = vec![];
            let mut creator_remaining = remaining;

            for collab in &r.collaborators {
                let collab_amount = remaining.multiply_ratio(
                    collab.share.atomics().u128(),
                    10u128.pow(Decimal::DECIMAL_PLACES),
                );
                collaborator_amounts.push((collab.address.clone(), collab_amount));
                creator_remaining -= collab_amount;
            }

            Ok(DistributionPreviewResponse {
                total_amount: amount,
                platform_fee,
                creator_amount: creator_remaining,
                collaborator_amounts,
                royalty_amount: None,
            })
        }
        None => Ok(DistributionPreviewResponse {
            total_amount: amount,
            platform_fee: Uint128::zero(),
            creator_amount: amount,
            collaborator_amounts: vec![],
            royalty_amount: None,
        }),
    }
}

fn query_collection_stats(deps: Deps, collection: String) -> StdResult<CollectionStatsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let stats = COLLECTION_STATS
        .may_load(deps.storage, collection_addr)?
        .unwrap_or_default();
    Ok(CollectionStatsResponse { stats })
}

fn query_revenue_events(
    deps: Deps,
    _collection: Option<String>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<RevenueEventsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let events: Vec<RevenueEvent> = REVENUE_EVENTS
        .range(deps.storage, start, None, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(RevenueEventsResponse { events })
}

fn query_split_wallet(deps: Deps, id: String) -> StdResult<SplitWalletResponse> {
    let wallet = SPLIT_WALLETS.may_load(deps.storage, id)?;
    Ok(SplitWalletResponse { wallet })
}

fn query_split_wallets(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<SplitWalletsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let wallets: Vec<SplitWallet> = SPLIT_WALLETS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(SplitWalletsResponse { wallets })
}
