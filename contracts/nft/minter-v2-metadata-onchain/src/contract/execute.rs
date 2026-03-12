use super::helpers::*;
use super::*;

// ========== Execute ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {} => execute_mint(deps, env, info),
        ExecuteMsg::MintTo { recipient } => execute_mint_to(deps, env, info, recipient),
        ExecuteMsg::MintFor {
            token_id,
            recipient,
        } => execute_mint_for(deps, env, info, token_id, recipient),
        ExecuteMsg::BatchMint { count } => execute_batch_mint(deps, env, info, count),
        ExecuteMsg::UpdateConfig {
            admin,
            per_address_limit,
            unit_price,
            whitelist,
            registry,
            collector_address,
            metadata_mode,
            paused,
        } => execute_update_config(
            deps,
            info,
            admin,
            per_address_limit,
            unit_price,
            whitelist,
            registry,
            collector_address,
            metadata_mode,
            paused,
        ),
        ExecuteMsg::UpdateStartTime { start_time } => {
            execute_update_start_time(deps, info, start_time)
        }
        ExecuteMsg::SetWhitelist { whitelist } => execute_set_whitelist(deps, info, whitelist),
        ExecuteMsg::RemoveWhitelist {} => execute_remove_whitelist(deps, info),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info),
        ExecuteMsg::WithdrawTo { recipient } => execute_withdraw_to(deps, env, info, recipient),
        ExecuteMsg::SetNativeAssetTemplate { native_assets } => {
            execute_set_native_asset_template(deps, info, native_assets)
        }
        ExecuteMsg::SetTokenNativeAssetOverride {
            token_id,
            native_assets,
        } => execute_set_token_native_asset_override(deps, info, token_id, native_assets),
        ExecuteMsg::ClearTokenNativeAssetOverride { token_id } => {
            execute_clear_token_native_asset_override(deps, info, token_id)
        }
    }
}

fn build_collector_payout_msg(
    deps: Deps,
    collector_address: &Addr,
    funds: Coin,
) -> Result<CosmosMsg, ContractError> {
    if deps
        .querier
        .query_wasm_contract_info(collector_address)
        .is_ok()
    {
        return Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collector_address.to_string(),
            msg: to_json_binary(&SplitExecuteMsg::Split {})?,
            funds: vec![funds],
        }));
    }

    Ok(CosmosMsg::Bank(BankMsg::Send {
        to_address: collector_address.to_string(),
        amount: vec![funds],
    }))
}

fn execute_mint(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Validate minting conditions
    validate_mint_conditions(&deps, &env, &info, &config)?;

    // Get and validate payment
    let (mint_price, is_whitelist) = get_current_price(&deps, &env, &info, &config)?;
    validate_payment(&info, &mint_price)?;

    // Get random available token
    let token_id = get_random_token_id(deps.storage, &env)?;

    // Perform mint
    let mint_msg = create_mint_msg(deps.storage, &config, token_id, info.sender.to_string())?;

    // Update state
    increment_mint_count(deps.storage, &info.sender)?;
    MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &(remaining - 1))?;

    // Update stats
    let mut stats = MINT_STATS.load(deps.storage)?;
    stats.total_minted += 1;
    stats.total_revenue += mint_price.amount;

    // Handle payment routing
    let mut messages: Vec<CosmosMsg> = vec![mint_msg];

    if let Some(collector_address) = &config.collector_address {
        messages.push(build_collector_payout_msg(
            deps.as_ref(),
            collector_address,
            mint_price.clone(),
        )?);
        stats.total_routed += mint_price.amount;
    }

    MINT_STATS.save(deps.storage, &stats)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "mint")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("minter", info.sender)
        .add_attribute("price", mint_price.to_string())
        .add_attribute("is_whitelist", is_whitelist.to_string()))
}

fn execute_mint_to(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only admin can mint to
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let recipient_addr = deps.api.addr_validate(&recipient)?;

    // Get random available token
    let token_id = get_random_token_id(deps.storage, &env)?;

    // Perform mint
    let mint_msg = create_mint_msg(deps.storage, &config, token_id, recipient.clone())?;

    // Update state
    increment_mint_count(deps.storage, &recipient_addr)?;
    MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &(remaining - 1))?;

    // Update stats
    let mut stats = MINT_STATS.load(deps.storage)?;
    stats.total_minted += 1;
    MINT_STATS.save(deps.storage, &stats)?;

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_to")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("recipient", recipient))
}

fn execute_mint_for(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: u32,
    recipient: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only admin can mint for
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let recipient_addr = deps.api.addr_validate(&recipient)?;

    // Check if token is available
    if !MINTABLE_TOKEN_IDS.has(deps.storage, token_id) {
        return Err(ContractError::TokenNotAvailable { token_id });
    }

    // Perform mint
    let mint_msg = create_mint_msg(deps.storage, &config, token_id, recipient.clone())?;

    // Update state
    increment_mint_count(deps.storage, &recipient_addr)?;
    MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &(remaining - 1))?;

    // Update stats
    let mut stats = MINT_STATS.load(deps.storage)?;
    stats.total_minted += 1;
    MINT_STATS.save(deps.storage, &stats)?;

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_for")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("recipient", recipient))
}

fn execute_batch_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    count: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Validate minting conditions
    validate_mint_conditions(&deps, &env, &info, &config)?;

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    if count > remaining {
        return Err(ContractError::BatchExceedsAvailable {});
    }

    // Check per-address limit
    let current_count = MINTER_ADDRS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(0);
    if current_count + count > config.per_address_limit {
        return Err(ContractError::BatchExceedsLimit {});
    }

    // Get and validate payment
    let (mint_price, _) = get_current_price(&deps, &env, &info, &config)?;
    let total_price = Coin {
        denom: mint_price.denom.clone(),
        amount: mint_price.amount * Uint128::from(count),
    };
    validate_payment(&info, &total_price)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    let mut minted_ids: Vec<u32> = vec![];

    for _ in 0..count {
        let token_id = get_random_token_id(deps.storage, &env)?;
        let mint_msg = create_mint_msg(deps.storage, &config, token_id, info.sender.to_string())?;
        messages.push(mint_msg);
        minted_ids.push(token_id);
        MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);
    }

    // Update state
    MINTER_ADDRS.save(deps.storage, &info.sender, &(current_count + count))?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &(remaining - count))?;

    // Update stats
    let mut stats = MINT_STATS.load(deps.storage)?;
    stats.total_minted += count;
    stats.total_revenue += total_price.amount;

    if let Some(collector_address) = &config.collector_address {
        messages.push(build_collector_payout_msg(
            deps.as_ref(),
            collector_address,
            total_price.clone(),
        )?);
        stats.total_routed += total_price.amount;
    }

    MINT_STATS.save(deps.storage, &stats)?;

    let ids_str: Vec<String> = minted_ids.iter().map(|id| id.to_string()).collect();

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "batch_mint")
        .add_attribute("count", count.to_string())
        .add_attribute("token_ids", ids_str.join(","))
        .add_attribute("minter", info.sender)
        .add_attribute("total_price", total_price.to_string()))
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    per_address_limit: Option<u32>,
    unit_price: Option<Coin>,
    whitelist: Option<String>,
    registry: Option<String>,
    collector_address: Option<String>,
    metadata_mode: Option<MetadataMode>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }

    if let Some(limit) = per_address_limit {
        config.per_address_limit = limit;
    }

    if let Some(price) = unit_price {
        config.unit_price = price;
    }

    if let Some(wl) = whitelist {
        config.whitelist = Some(deps.api.addr_validate(&wl)?);
    }

    if let Some(reg) = registry {
        config.registry = Some(deps.api.addr_validate(&reg)?);
    }

    if let Some(address) = collector_address {
        config.collector_address = Some(deps.api.addr_validate(&address)?);
    }

    if let Some(mode) = metadata_mode {
        if !mode.uses_onchain_metadata() {
            return Err(ContractError::MetadataModeLockedToOnChain {});
        }
        config.metadata_mode = MetadataMode::OnChain;
    }

    if let Some(is_paused) = paused {
        config.paused = is_paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("metadata_mode", format!("{:?}", config.metadata_mode)))
}

fn execute_update_start_time(
    deps: DepsMut,
    info: MessageInfo,
    start_time: cosmwasm_std::Timestamp,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.start_time = start_time;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_start_time")
        .add_attribute("start_time", start_time.to_string()))
}

fn execute_set_whitelist(
    deps: DepsMut,
    info: MessageInfo,
    whitelist: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.whitelist = Some(deps.api.addr_validate(&whitelist)?);
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "set_whitelist")
        .add_attribute("whitelist", whitelist))
}

fn execute_remove_whitelist(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.whitelist = None;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "remove_whitelist"))
}

fn validate_native_assets(native_assets: &[NativeAsset]) -> Result<(), ContractError> {
    for asset in native_assets {
        if asset.asset_id.trim().is_empty() {
            return Err(ContractError::InvalidNativeAsset {
                reason: "asset_id cannot be empty".to_string(),
            });
        }
        if asset.name.trim().is_empty() {
            return Err(ContractError::InvalidNativeAsset {
                reason: format!("name cannot be empty for asset_id {}", asset.asset_id),
            });
        }
        if asset.image_url.trim().is_empty() {
            return Err(ContractError::InvalidNativeAsset {
                reason: format!("image_url cannot be empty for asset_id {}", asset.asset_id),
            });
        }
    }
    Ok(())
}

fn execute_set_native_asset_template(
    deps: DepsMut,
    info: MessageInfo,
    native_assets: Vec<NativeAsset>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    validate_native_assets(&native_assets)?;
    config.native_asset_template = native_assets;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "set_native_asset_template")
        .add_attribute("count", config.native_asset_template.len().to_string()))
}

fn execute_set_token_native_asset_override(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u32,
    native_assets: Vec<NativeAsset>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if token_id == 0 || token_id > config.num_tokens {
        return Err(ContractError::InvalidTokenId { token_id });
    }

    validate_native_assets(&native_assets)?;
    TOKEN_NATIVE_ASSET_OVERRIDES.save(deps.storage, token_id, &native_assets)?;

    Ok(Response::new()
        .add_attribute("action", "set_token_native_asset_override")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("count", native_assets.len().to_string()))
}

fn execute_clear_token_native_asset_override(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if token_id == 0 || token_id > config.num_tokens {
        return Err(ContractError::InvalidTokenId { token_id });
    }

    TOKEN_NATIVE_ASSET_OVERRIDES.remove(deps.storage, token_id);

    Ok(Response::new()
        .add_attribute("action", "clear_token_native_asset_override")
        .add_attribute("token_id", token_id.to_string()))
}

fn execute_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if config.collector_address.is_some() {
        return Err(ContractError::CannotWithdrawWithCollectorAddress {});
    }

    let balance = deps
        .querier
        .query_balance(&env.contract.address, &config.unit_price.denom)?;

    if balance.amount.is_zero() {
        return Err(ContractError::NoFundsToWithdraw {});
    }

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![balance.clone()],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "withdraw")
        .add_attribute("amount", balance.to_string())
        .add_attribute("recipient", info.sender))
}

fn execute_withdraw_to(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if config.collector_address.is_some() {
        return Err(ContractError::CannotWithdrawWithCollectorAddress {});
    }

    let recipient_addr = deps.api.addr_validate(&recipient)?;
    let balance = deps
        .querier
        .query_balance(&env.contract.address, &config.unit_price.denom)?;

    if balance.amount.is_zero() {
        return Err(ContractError::NoFundsToWithdraw {});
    }

    let msg = BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: vec![balance.clone()],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "withdraw_to")
        .add_attribute("amount", balance.to_string())
        .add_attribute("recipient", recipient_addr))
}
