use super::*;

// ========== Instantiate ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg
        .admin
        .map(|a| deps.api.addr_validate(&a))
        .transpose()?
        .unwrap_or(info.sender);

    let operators = msg
        .operators
        .unwrap_or_default()
        .iter()
        .map(|o| deps.api.addr_validate(o))
        .collect::<StdResult<Vec<Addr>>>()?;
    let ecosystem_factory = msg
        .ecosystem_factory
        .ok_or(ContractError::EcosystemFactoryRequired {})?;
    let ecosystem_factory = Some(deps.api.addr_validate(&ecosystem_factory)?);

    let config = Config {
        admin,
        operators,
        ecosystem_factory,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    ECOSYSTEM_COUNT.save(deps.storage, &0u64)?;
    NEXT_ECOSYSTEM_REQUEST_ID.save(deps.storage, &1u64)?;
    DEAD_PROJECT_CASE_COUNT.save(deps.storage, &0u64)?;
    RECOVERY_CONFIG.save(
        deps.storage,
        &RecoveryConfig {
            inactivity_period_secs: 90 * 24 * 60 * 60,
            contest_period_secs: 30 * 24 * 60 * 60,
        },
    )?;

    let now = env.block.time.seconds();
    LAST_CREATOR_ACTIVITY.save(deps.storage, config.admin.clone(), &now)?;
    for operator in config.operators {
        LAST_CREATOR_ACTIVITY.save(deps.storage, operator, &now)?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "registry"))
}
