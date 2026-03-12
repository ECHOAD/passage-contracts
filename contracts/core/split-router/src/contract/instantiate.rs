use super::helpers::build_recipients;
use super::*;

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

    let config = Config {
        admin,
        paused: false,
    };
    let split = SplitConfig {
        recipients: build_recipients(deps.as_ref(), msg.recipients)?,
        active: msg.active.unwrap_or(true),
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    CONFIG.save(deps.storage, &config)?;
    SPLIT_CONFIG.save(deps.storage, &split)?;
    SPLIT_EVENT_COUNT.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "split"))
}
