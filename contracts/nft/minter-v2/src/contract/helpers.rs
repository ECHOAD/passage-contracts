use super::*;

// ========== Helpers ==========

pub(super) fn validate_mint_conditions(
    deps: &DepsMut,
    env: &Env,
    info: &MessageInfo,
    config: &Config,
) -> Result<(), ContractError> {
    if config.paused {
        return Err(ContractError::MintingPaused {});
    }

    if env.block.time < config.start_time {
        return Err(ContractError::MintingNotStarted {
            start_time: config.start_time.to_string(),
        });
    }

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    if remaining == 0 {
        return Err(ContractError::SoldOut {});
    }

    let count = MINTER_ADDRS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or(0);
    if count >= config.per_address_limit {
        return Err(ContractError::MaxMintLimitReached {
            limit: config.per_address_limit,
        });
    }

    Ok(())
}

pub(super) fn get_current_price(
    _deps: &DepsMut,
    _env: &Env,
    _info: &MessageInfo,
    config: &Config,
) -> Result<(Coin, bool), ContractError> {
    // TODO: Check whitelist for special pricing
    // For now, return public price
    Ok((config.unit_price.clone(), false))
}

pub(super) fn validate_payment(info: &MessageInfo, expected: &Coin) -> Result<(), ContractError> {
    let payment = info
        .funds
        .iter()
        .find(|c| c.denom == expected.denom)
        .cloned()
        .unwrap_or(Coin {
            denom: expected.denom.clone(),
            amount: Uint128::zero(),
        });

    if payment.amount != expected.amount {
        return Err(ContractError::InvalidPayment {
            expected: expected.to_string(),
            received: payment.to_string(),
        });
    }

    Ok(())
}

pub(super) fn get_random_token_id(
    storage: &mut dyn cosmwasm_std::Storage,
    env: &Env,
) -> Result<u32, ContractError> {
    let remaining = MINTABLE_NUM_TOKENS.load(storage)?;
    if remaining == 0 {
        return Err(ContractError::SoldOut {});
    }

    // Simple pseudo-random selection based on block info
    let seed = env.block.height + env.block.time.nanos();
    let index = (seed % remaining as u64) as u32;

    // Find the nth available token
    let mut count = 0u32;
    let mut token_id = 0u32;

    for i in 1..=1000000u32 {
        // Reasonable upper bound
        if MINTABLE_TOKEN_IDS.has(storage, i) {
            if count == index {
                token_id = i;
                break;
            }
            count += 1;
        }
    }

    if token_id == 0 {
        // Fallback: get first available
        for i in 1..=1000000u32 {
            if MINTABLE_TOKEN_IDS.has(storage, i) {
                token_id = i;
                break;
            }
        }
    }

    Ok(token_id)
}

pub(super) fn create_mint_msg(
    config: &Config,
    token_id: u32,
    owner: String,
) -> Result<CosmosMsg, ContractError> {
    let token_uri = format!("{}/{}", config.base_token_uri, token_id);

    let exec_msg: cw721_base::ExecuteMsg<cosmwasm_std::Empty, cosmwasm_std::Empty> =
        cw721_base::ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner,
            token_uri: Some(token_uri),
            extension: cosmwasm_std::Empty {},
        };

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.cw721_address.to_string(),
        msg: to_json_binary(&exec_msg)?,
        funds: vec![],
    }))
}

pub(super) fn increment_mint_count(
    storage: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
) -> Result<(), ContractError> {
    let count = MINTER_ADDRS.may_load(storage, addr)?.unwrap_or(0);
    MINTER_ADDRS.save(storage, addr, &(count + 1))?;

    // Update unique minters count if this is first mint
    if count == 0 {
        let mut stats = MINT_STATS.load(storage)?;
        stats.unique_minters += 1;
        MINT_STATS.save(storage, &stats)?;
    }

    Ok(())
}
