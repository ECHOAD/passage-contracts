use super::*;

// ========== Instantiate ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cw721_address = deps.api.addr_validate(&msg.cw721_address)?;

    // Validate whitelist if provided
    let whitelist = msg
        .whitelist
        .map(|w| deps.api.addr_validate(&w))
        .transpose()?;

    let registry = msg
        .registry
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    // Ensure the target collection already exists and is pg721-compatible.
    let provisional_config = Config {
        admin: info.sender.clone(),
        cw721_address: cw721_address.clone(),
        base_token_uri: msg.base_token_uri.clone(),
        num_tokens: msg.num_tokens,
        unit_price: msg.unit_price.clone(),
        per_address_limit: msg.per_address_limit,
        start_time: msg.start_time,
        whitelist: whitelist.clone(),
        registry: registry.clone(),
        paused: false,
    };
    let _ = super::helpers::query_collection_nft_type(deps.as_ref(), &provisional_config)?;
    super::helpers::validate_collection_requirements(deps.as_ref(), &provisional_config)?;

    let config = Config {
        admin: info.sender.clone(),
        cw721_address,
        base_token_uri: msg.base_token_uri,
        num_tokens: msg.num_tokens,
        unit_price: msg.unit_price,
        per_address_limit: msg.per_address_limit,
        start_time: msg.start_time,
        whitelist,
        registry,
        paused: false,
    };

    CONFIG.save(deps.storage, &config)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &msg.num_tokens)?;
    MINT_STATS.save(deps.storage, &MintStats::default())?;

    // Initialize mintable token IDs
    for i in 1..=msg.num_tokens {
        MINTABLE_TOKEN_IDS.save(deps.storage, i, &true)?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", "minter-v2")
        .add_attribute("admin", info.sender)
        .add_attribute("cw721_address", config.cw721_address))
}
