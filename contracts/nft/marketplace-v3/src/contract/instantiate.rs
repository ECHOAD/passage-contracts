use super::*;

// ========== Instantiate ==========

/// Default maximum trading fee: 10%
const DEFAULT_MAX_TRADING_FEE_BPS: u64 = 1000;

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

    let max_trading_fee_bps = msg
        .max_trading_fee_bps
        .unwrap_or(DEFAULT_MAX_TRADING_FEE_BPS);

    // Validate trading fee doesn't exceed max
    if msg.trading_fee_bps > max_trading_fee_bps {
        return Err(ContractError::TradingFeeExceedsMax {
            fee_bps: msg.trading_fee_bps,
            max_bps: max_trading_fee_bps,
        });
    }

    let config = Config {
        admin,
        denom: msg.denom,
        min_price: msg.min_price,
        trading_fee_bps: msg.trading_fee_bps,
        max_trading_fee_bps,
        fee_collector,
        registry,
        operators,
        paused: false,
        require_registration: msg.require_registration.unwrap_or(true),
    };

    CONFIG.save(deps.storage, &config)?;
    MARKET_STATS.save(deps.storage, &MarketStats::default())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "marketplace-v3")
        .add_attribute(
            "require_registration",
            config.require_registration.to_string(),
        ))
}
