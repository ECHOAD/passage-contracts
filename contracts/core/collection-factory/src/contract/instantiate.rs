use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.collection_code_id == 0 {
        return Err(ContractError::InvalidCodeId {});
    }

    let admin = msg
        .admin
        .map(|a| deps.api.addr_validate(&a))
        .transpose()?
        .unwrap_or(info.sender.clone());

    let operators = msg
        .operators
        .unwrap_or_default()
        .iter()
        .map(|o| deps.api.addr_validate(o))
        .collect::<StdResult<Vec<Addr>>>()?;
    let registry = deps.api.addr_validate(&msg.registry)?;

    if msg.ecosystem_id.trim().is_empty() {
        return Err(ContractError::EmptyEcosystemId {});
    }

    let config = Config {
        admin: admin.clone(),
        operators,
        registry,
        ecosystem_id: msg.ecosystem_id.clone(),
        collection_code_id: msg.collection_code_id,
        enforce_local_allowlist: msg.enforce_local_allowlist.unwrap_or(false),
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    NEXT_REQUEST_ID.save(deps.storage, &1u64)?;
    COLLECTION_COUNT.save(deps.storage, &0u64)?;

    if let Some(approved) = msg.approved_creators {
        for creator in approved {
            let creator_addr = deps.api.addr_validate(&creator)?;
            APPROVED_CREATORS.save(deps.storage, creator_addr, &true)?;
        }
    }

    // Ensure admin can always create collections.
    APPROVED_CREATORS.save(deps.storage, admin.clone(), &true)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "collection-factory")
        .add_attribute("admin", admin)
        .add_attribute("ecosystem_id", msg.ecosystem_id))
}
