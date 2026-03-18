use super::helpers::extract_contract_address_from_reply;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let request_id = msg.id;
    let config = CONFIG.load(deps.storage)?;

    let pending = PENDING_CREATIONS
        .may_load(deps.storage, request_id)?
        .ok_or(ContractError::PendingCreationNotFound { id: request_id })?;

    let collection_address = extract_contract_address_from_reply(&msg)?;
    let collection_address = deps.api.addr_validate(&collection_address)?;

    let next_collection_id = COLLECTION_COUNT.load(deps.storage)? + 1;
    COLLECTION_COUNT.save(deps.storage, &next_collection_id)?;

    let record = CollectionRecord {
        id: next_collection_id,
        creator: pending.creator.clone(),
        collection_address: collection_address.clone(),
        name: pending.name,
        symbol: pending.symbol,
        nft_type: pending.nft_type.clone(),
        created_at: env.block.time.seconds(),
    };

    COLLECTIONS.save(deps.storage, next_collection_id, &record)?;
    PENDING_CREATIONS.remove(deps.storage, request_id);

    let register_msg = WasmMsg::Execute {
        contract_addr: config.registry.to_string(),
        msg: to_json_binary(&RegistryExecuteMsg::RegisterCollectionFromFactory {
            address: collection_address.to_string(),
            ecosystem_id: pending.ecosystem_id.clone(),
            name: record.name.clone(),
            creator: record.creator.to_string(),
            nft_type: record.nft_type.clone(),
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(register_msg)
        .add_attribute("action", "create_collection_reply")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("collection_id", next_collection_id.to_string())
        .add_attribute("creator", pending.creator)
        .add_attribute("collection_address", collection_address)
        .add_attribute("ecosystem_id", pending.ecosystem_id)
        .add_attribute("nft_type", record.nft_type.to_string()))
}

#[cfg(test)]
mod tests;
