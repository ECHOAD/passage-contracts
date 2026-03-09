use super::*;

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

    let operators = msg
        .operators
        .unwrap_or_default()
        .iter()
        .map(|o| deps.api.addr_validate(o))
        .collect::<StdResult<Vec<Addr>>>()?;
    let registry = deps.api.addr_validate(&msg.registry)?;

    let config = Config {
        admin: admin.clone(),
        operators,
        registry,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    NEXT_REQUEST_ID.save(deps.storage, &1u64)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "ecosystem-factory")
        .add_attribute("admin", admin))
}
