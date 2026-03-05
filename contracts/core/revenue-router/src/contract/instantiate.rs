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

    let platform_fee_collector = deps.api.addr_validate(&msg.platform_fee_collector)?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    // Validate platform fee
    let max_fee: Decimal = MAX_PLATFORM_FEE.parse().unwrap();
    if msg.default_platform_fee > max_fee {
        return Err(ContractError::PlatformFeeExceedsMax {
            max: MAX_PLATFORM_FEE.to_string(),
        });
    }

    let config = Config {
        admin,
        platform_fee_collector,
        default_platform_fee: msg.default_platform_fee,
        registry,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    REVENUE_EVENT_COUNT.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "revenue-router"))
}
