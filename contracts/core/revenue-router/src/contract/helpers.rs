use super::*;

// ========== Helpers ==========

pub(super) fn process_collaborators(
    deps: Deps,
    collaborators: Option<Vec<CollaboratorInput>>,
) -> Result<Vec<Collaborator>, ContractError> {
    match collaborators {
        Some(collabs) => {
            let list: Vec<Collaborator> = collabs
                .into_iter()
                .map(|c| {
                    if c.share > Decimal::one() {
                        return Err(ContractError::InvalidShare {});
                    }
                    Ok(Collaborator {
                        address: deps.api.addr_validate(&c.address)?,
                        share: c.share,
                        name: c.name,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(list)
        }
        None => Ok(vec![]),
    }
}

pub(super) fn record_revenue_event(
    storage: &mut dyn cosmwasm_std::Storage,
    env: &Env,
    collection: &Addr,
    event_type: &RevenueEventType,
    total_amount: Uint128,
    denom: &str,
    creator_amount: Uint128,
    collaborator_amounts: Vec<(Addr, Uint128)>,
    sender: &Addr,
) -> Result<u64, ContractError> {
    let mut event_id = REVENUE_EVENT_COUNT.load(storage)?;
    event_id += 1;

    // Clean up old events if we exceed the limit
    if event_id > MAX_EVENTS_STORED {
        let old_id = event_id - MAX_EVENTS_STORED;
        REVENUE_EVENTS.remove(storage, old_id);
    }

    let event = RevenueEvent {
        id: event_id,
        collection: collection.clone(),
        event_type: event_type.clone(),
        total_amount,
        denom: denom.to_string(),
        creator_amount,
        collaborator_amounts,
        timestamp: env.block.time.seconds(),
        tx_sender: sender.clone(),
    };

    REVENUE_EVENTS.save(storage, event_id, &event)?;
    REVENUE_EVENT_COUNT.save(storage, &event_id)?;

    Ok(event_id)
}

pub(super) fn update_collection_stats(
    storage: &mut dyn cosmwasm_std::Storage,
    collection: &Addr,
    event_type: &RevenueEventType,
    total_amount: Uint128,
    creator_amount: Uint128,
) -> Result<(), ContractError> {
    let mut stats = COLLECTION_STATS
        .may_load(storage, collection.clone())?
        .unwrap_or_default();

    match event_type {
        RevenueEventType::PrimarySale => {
            stats.total_primary_volume += total_amount;
        }
        RevenueEventType::SecondaryRoyalty | RevenueEventType::AuctionRoyalty => {
            stats.total_secondary_royalties += total_amount;
        }
        _ => {}
    }

    stats.total_creator_earnings += creator_amount;
    stats.event_count += 1;

    COLLECTION_STATS.save(storage, collection.clone(), &stats)?;

    Ok(())
}
