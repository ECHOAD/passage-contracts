use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Validate code IDs
    if msg.collection_factory_code_id == 0 {
        return Err(ContractError::InvalidCodeId {
            field: "collection_factory_code_id".to_string(),
        });
    }
    if msg.collection_code_id == 0 {
        return Err(ContractError::InvalidCodeId {
            field: "collection_code_id".to_string(),
        });
    }

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
        collection_factory_code_id: msg.collection_factory_code_id,
        collection_code_id: msg.collection_code_id,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    NEXT_REQUEST_ID.save(deps.storage, &1u64)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "ecosystem-factory")
        .add_attribute("admin", admin)
        .add_attribute(
            "collection_factory_code_id",
            msg.collection_factory_code_id.to_string(),
        )
        .add_attribute("collection_code_id", msg.collection_code_id.to_string()))
}
