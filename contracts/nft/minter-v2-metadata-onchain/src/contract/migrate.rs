use super::*;

// ========== Migrate ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let prev_version = get_contract_version(deps.storage)?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let collector_address = msg
        .collector_address
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let result = migrate_state(
        deps.storage,
        &prev_version.contract,
        registry,
        collector_address,
        msg.base_token_uri,
    )?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let event = Event::new("migrate")
        .add_attribute("from_contract", prev_version.contract)
        .add_attribute("from_version", prev_version.version)
        .add_attribute("to_contract", CONTRACT_NAME)
        .add_attribute("to_version", CONTRACT_VERSION)
        .add_attribute("source_state", result.source_contract)
        .add_attribute("tokens_migrated", result.tokens_migrated.to_string())
        .add_attribute("mintable_remaining", result.mintable_remaining.to_string())
        .add_attribute("unique_minters", result.unique_minters.to_string())
        .add_attribute(
            "collector_configured",
            result.collector_configured.to_string(),
        );

    Ok(Response::new()
        .add_event(event)
        .add_attribute("action", "migrate")
        .add_attribute("to_version", CONTRACT_VERSION))
}
