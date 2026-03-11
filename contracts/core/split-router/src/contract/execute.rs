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
        ExecuteMsg::CreateSplitRule { key, recipients } => {
            execute_create_split_rule(deps, env, info, key, recipients)
        }
        ExecuteMsg::UpdateSplitRule {
            key,
            recipients,
            owner,
            active,
        } => execute_update_split_rule(deps, env, info, key, recipients, owner, active),
        ExecuteMsg::RemoveSplitRule { key } => execute_remove_split_rule(deps, info, key),
        ExecuteMsg::Split { key } => execute_split(deps, env, info, key),
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

fn execute_create_split_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    key: String,
    recipients: Vec<RecipientInput>,
) -> Result<Response, ContractError> {
    validate_key(&key)?;
    if SPLIT_RULES.has(deps.storage, key.as_str()) {
        return Err(ContractError::SplitRuleExists { key });
    }

    let rule = SplitRule {
        key: key.clone(),
        owner: info.sender.clone(),
        recipients: build_recipients(deps.as_ref(), recipients)?,
        active: true,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    SPLIT_RULES.save(deps.storage, key.as_str(), &rule)?;

    Ok(Response::new()
        .add_attribute("action", "create_split_rule")
        .add_attribute("key", key)
        .add_attribute("owner", info.sender))
}

fn execute_update_split_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    key: String,
    recipients: Option<Vec<RecipientInput>>,
    owner: Option<String>,
    active: Option<bool>,
) -> Result<Response, ContractError> {
    validate_key(&key)?;

    let config = CONFIG.load(deps.storage)?;
    let mut rule = SPLIT_RULES
        .load(deps.storage, key.as_str())
        .map_err(|_| ContractError::SplitRuleNotFound { key: key.clone() })?;

    ensure_rule_manager(&config, &info.sender, &rule.owner)?;

    if let Some(recipients) = recipients {
        rule.recipients = build_recipients(deps.as_ref(), recipients)?;
    }

    if let Some(owner) = owner {
        rule.owner = deps.api.addr_validate(&owner)?;
    }

    if let Some(active) = active {
        rule.active = active;
    }

    rule.updated_at = env.block.time.seconds();
    SPLIT_RULES.save(deps.storage, key.as_str(), &rule)?;

    Ok(Response::new()
        .add_attribute("action", "update_split_rule")
        .add_attribute("key", key)
        .add_attribute("owner", rule.owner))
}

fn execute_remove_split_rule(
    deps: DepsMut,
    info: MessageInfo,
    key: String,
) -> Result<Response, ContractError> {
    validate_key(&key)?;

    let config = CONFIG.load(deps.storage)?;
    let rule = SPLIT_RULES
        .load(deps.storage, key.as_str())
        .map_err(|_| ContractError::SplitRuleNotFound { key: key.clone() })?;

    ensure_rule_manager(&config, &info.sender, &rule.owner)?;
    SPLIT_RULES.remove(deps.storage, key.as_str());

    Ok(Response::new()
        .add_attribute("action", "remove_split_rule")
        .add_attribute("key", key))
}

fn execute_split(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    key: String,
) -> Result<Response, ContractError> {
    validate_key(&key)?;

    if info.funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let rule = SPLIT_RULES
        .load(deps.storage, key.as_str())
        .map_err(|_| ContractError::SplitRuleNotFound { key: key.clone() })?;

    if !rule.active {
        return Err(ContractError::SplitRuleInactive { key });
    }

    let total_funds = info.funds.clone();
    let recipient_amounts = calculate_split_amounts(&rule.recipients, &info.funds);
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
        key.as_str(),
        total_funds,
        recipient_amounts,
        &info.sender,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "split")
        .add_attribute("key", key)
        .add_attribute("funds_count", info.funds.len().to_string())
        .add_attribute("event_id", event_id.to_string()))
}

#[cfg(test)]
mod tests;
