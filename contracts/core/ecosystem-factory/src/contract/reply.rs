use super::helpers::extract_collection_factory_address_from_reply;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let reply_id = msg.id;

    // Load pending ecosystem creation data
    let pending = PENDING_ECOSYSTEM_CREATIONS
        .may_load(deps.storage, reply_id)?
        .ok_or(ContractError::PendingCreationNotFound { reply_id })?;

    let collection_factory_addr =
        deps.api
            .addr_validate(&extract_collection_factory_address_from_reply(&msg)?)?;

    // Clean up pending data
    PENDING_ECOSYSTEM_CREATIONS.remove(deps.storage, reply_id);

    // Now register the ecosystem in the registry with the collection-factory address
    let config = CONFIG.load(deps.storage)?;

    let register_msg = WasmMsg::Execute {
        contract_addr: config.registry.to_string(),
        msg: to_json_binary(&RegistryExecuteMsg::RegisterEcosystemFromFactory {
            id: pending.ecosystem_id.clone(),
            name: pending.ecosystem_name.clone(),
            creator: pending.creator.to_string(),
            collection_factory: collection_factory_addr.to_string(),
            description: pending.description,
            image_urls: pending.image_urls,
            animation_url: pending.animation_url,
            url: pending.url,
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(register_msg)
        .add_attribute("action", "reply_collection_factory_instantiated")
        .add_attribute("request_id", reply_id.to_string())
        .add_attribute("ecosystem_id", pending.ecosystem_id)
        .add_attribute("collection_factory", collection_factory_addr))
}

#[cfg(test)]
mod tests;
