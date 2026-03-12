use super::*;

// ========== Migrate ==========

/// Migrate from marketplace-v2 to marketplace-v3
///
/// This migrates:
/// - Config (single collection -> multi-collection)
/// - Asks (add collection field, created_at, is_active)
/// - Bids (add collection field, created_at, expires_at)
/// - CollectionBids (add collection field, created_at, expires_at)
/// - Initialize new state: MarketStats, CollectionStats
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let prev_version = get_contract_version(deps.storage)?;
    let from_contract = prev_version.contract.clone();
    let from_version = prev_version.version.clone();

    // Validate we're migrating from marketplace-v2
    if !prev_version.contract.contains("marketplace") {
        return Err(ContractError::Unauthorized {});
    }

    let collection = deps.api.addr_validate(&msg.collection)?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let additional_collections = msg
        .additional_collections
        .unwrap_or_default()
        .iter()
        .map(|c| deps.api.addr_validate(c))
        .collect::<StdResult<Vec<Addr>>>()?;

    let collection_denom_overrides = msg
        .collection_denoms
        .unwrap_or_default()
        .iter()
        .map(|entry| {
            Ok((
                deps.api.addr_validate(&entry.collection)?,
                entry.denom.clone(),
            ))
        })
        .collect::<StdResult<Vec<(Addr, String)>>>()?;

    // Execute migration
    let stats = migrate_state(
        deps.storage,
        collection.clone(),
        registry,
        additional_collections,
        env.block.time.seconds(),
    )?;

    for (collection, denom) in collection_denom_overrides {
        COLLECTION_DENOMS.save(deps.storage, collection, &denom)?;
    }

    // Update contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let event = Event::new("migrate")
        .add_attribute("from_contract", from_contract)
        .add_attribute("from_version", from_version.clone())
        .add_attribute("to_contract", CONTRACT_NAME)
        .add_attribute("to_version", CONTRACT_VERSION)
        .add_attribute("collection", collection.to_string())
        .add_attribute("asks_migrated", stats.asks_migrated.to_string())
        .add_attribute("bids_migrated", stats.bids_migrated.to_string())
        .add_attribute(
            "collection_bids_migrated",
            stats.collection_bids_migrated.to_string(),
        );

    Ok(Response::new()
        .add_event(event)
        .add_attribute("action", "migrate")
        .add_attribute("from_version", from_version)
        .add_attribute("to_version", CONTRACT_VERSION))
}
