use super::helpers::*;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.paused && config.admin != info.sender {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        ExecuteMsg::UpdateConfig { admin, paused } => {
            execute_update_config(deps, info, admin, paused)
        }
        ExecuteMsg::UpdateSplit { recipients, active } => {
            execute_update_split(deps, env, info, recipients, active)
        }
        ExecuteMsg::Split {} => execute_split(deps, env, info),
        ExecuteMsg::RouteWorldRevenue {
            world_nft_id,
            world_collection,
        } => execute_route_world_revenue(deps, env, info, world_nft_id, world_collection),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(admin) = admin {
        config.admin = deps.api.addr_validate(&admin)?;
    }

    if let Some(paused) = paused {
        config.paused = paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn execute_update_split(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipients: Option<Vec<RecipientInput>>,
    active: Option<bool>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut split = SPLIT_CONFIG.load(deps.storage)?;
    if let Some(recipients) = recipients {
        split.recipients = build_recipients(deps.as_ref(), recipients)?;
    }

    if let Some(active) = active {
        split.active = active;
    }

    split.updated_at = env.block.time.seconds();
    SPLIT_CONFIG.save(deps.storage, &split)?;

    Ok(Response::new().add_attribute("action", "update_split"))
}

fn execute_split(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let split = SPLIT_CONFIG.load(deps.storage)?;
    if !split.active {
        return Err(ContractError::SplitInactive {});
    }

    let total_funds = info.funds.clone();
    let recipient_amounts = calculate_split_amounts(&split.recipients, &info.funds);
    let mut messages: Vec<CosmosMsg> = vec![];
    for (recipient, amount) in &recipient_amounts {
        if amount.is_empty() {
            continue;
        }

        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient.to_string(),
            amount: amount.clone(),
        }));
    }

    let event_id = record_split_event(
        deps.storage,
        &env,
        total_funds,
        recipient_amounts,
        &info.sender,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "split")
        .add_attribute("funds_count", info.funds.len().to_string())
        .add_attribute("preserves_input_denoms", "true")
        .add_attribute("event_id", event_id.to_string()))
}

fn execute_route_world_revenue(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    world_nft_id: String,
    world_collection: String,
) -> Result<Response, ContractError> {
    let mut response = execute_split(deps, env, info)?;
    response.attributes.retain(|attr| attr.key != "action");

    Ok(response
        .add_attribute("action", "route_world_revenue")
        .add_attribute("world_nft_id", world_nft_id)
        .add_attribute("world_collection", world_collection))
}

#[cfg(test)]
mod tests;


