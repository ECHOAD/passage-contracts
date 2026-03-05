use super::*;

// ========== Instantiate ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg
        .admin
        .map(|a| deps.api.addr_validate(&a))
        .transpose()?
        .unwrap_or(info.sender);

    let fee_collector = deps.api.addr_validate(&msg.fee_collector)?;

    let supported_collections = msg
        .supported_collections
        .unwrap_or_default()
        .iter()
        .map(|c| deps.api.addr_validate(c))
        .collect::<StdResult<Vec<Addr>>>()?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let revenue_router = msg
        .revenue_router
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let operators = msg
        .operators
        .unwrap_or_default()
        .iter()
        .map(|o| deps.api.addr_validate(o))
        .collect::<StdResult<Vec<Addr>>>()?;

    let collection_denom_overrides = msg
        .collection_denoms
        .unwrap_or_default()
        .iter()
        .map(|entry| {
            Ok((
                deps.api.addr_validate(&entry.collection)?,
                entry.denom.clone(),
            ))
        })
        .collect::<StdResult<Vec<(Addr, String)>>>()?;

    let config = Config {
        admin,
        supported_collections,
        allow_any_collection: msg.allow_any_collection.unwrap_or(true),
        denom: msg.denom,
        min_price: msg.min_price,
        trading_fee_bps: msg.trading_fee_bps,
        fee_collector,
        registry,
        revenue_router: revenue_router.clone(),
        use_revenue_router: msg.use_revenue_router.unwrap_or(revenue_router.is_some()),
        operators,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    for (collection, denom) in collection_denom_overrides {
        COLLECTION_DENOMS.save(deps.storage, collection, &denom)?;
    }
    MARKET_STATS.save(deps.storage, &MarketStats::default())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "marketplace-v3"))
}
