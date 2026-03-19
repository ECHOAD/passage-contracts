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

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let operators = msg
        .operators
        .unwrap_or_default()
        .iter()
        .map(|o| deps.api.addr_validate(o))
        .collect::<StdResult<Vec<Addr>>>()?;

    let config = Config {
        admin,
        min_price: msg.min_price,
        trading_fee_bps: msg.trading_fee_bps,
        fee_collector,
        registry,
        operators,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    MARKET_STATS.save(deps.storage, &MarketStats::default())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "marketplace-v3")
        .add_attribute("trading_fee_bps", config.trading_fee_bps.to_string()))
}
