use super::*;

// ========== Instantiate ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if matches!(msg.metadata_mode, Some(MetadataMode::OffChain)) {
        return Err(ContractError::MetadataModeLockedToOnChain {});
    }

    // Validate whitelist if provided
    let whitelist = msg
        .whitelist
        .map(|w| deps.api.addr_validate(&w))
        .transpose()?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let collector_address = msg
        .collector_address
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    // Create initial config (cw721_address will be set in reply)
    let config = Config {
        admin: info.sender.clone(),
        cw721_address: Addr::unchecked(""), // Will be set in reply
        cw721_code_id: msg.cw721_code_id,
        base_token_uri: msg.base_token_uri,
        num_tokens: msg.num_tokens,
        unit_price: msg.unit_price,
        per_address_limit: msg.per_address_limit,
        start_time: msg.start_time,
        whitelist,
        registry,
        collector_address,
        metadata_mode: MetadataMode::OnChain,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &msg.num_tokens)?;
    MINT_STATS.save(deps.storage, &MintStats::default())?;

    // Initialize mintable token IDs
    for i in 1..=msg.num_tokens {
        MINTABLE_TOKEN_IDS.save(deps.storage, i, &true)?;
    }

    // Instantiate pg721 NFT contract
    let cw721_instantiate_msg = WasmMsg::Instantiate {
        admin: Some(info.sender.to_string()),
        code_id: msg.cw721_code_id,
        msg: to_json_binary(&msg.cw721_instantiate_msg)?,
        funds: vec![],
        label: format!("pg721-{}", env.block.height),
    };

    let submsg = SubMsg::reply_on_success(cw721_instantiate_msg, INSTANTIATE_CW721_REPLY_ID);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "minter-v2-metadata-onchain")
        .add_attribute("metadata_mode", format!("{:?}", config.metadata_mode))
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_CW721_REPLY_ID => handle_cw721_instantiate_reply(deps, msg),
        _ => Err(ContractError::InvalidInstantiateReplyData {}),
    }
}

fn handle_cw721_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    let res = msg
        .result
        .into_result()
        .map_err(|_| ContractError::NftInstantiateFailed {})?;

    // Find the contract address from instantiate events
    let cw721_address = res
        .events
        .iter()
        .find(|e| e.ty == "instantiate")
        .and_then(|e| {
            e.attributes
                .iter()
                .find(|a| a.key == "_contract_address")
                .map(|a| a.value.clone())
        })
        .ok_or(ContractError::InvalidInstantiateReplyData {})?;

    let mut config = CONFIG.load(deps.storage)?;
    config.cw721_address = deps.api.addr_validate(&cw721_address)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate_cw721_reply")
        .add_attribute("cw721_address", cw721_address))
}
