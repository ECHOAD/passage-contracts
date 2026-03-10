use super::helpers::{
    get_active_whitelist_config, get_public_mint_price, validate_registry_requirements,
};
use super::*;
use cosmwasm_std::StdError;

// ========== Query ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::MintableNumTokens {} => to_json_binary(&query_mintable_num_tokens(deps)?),
        QueryMsg::StartTime {} => to_json_binary(&query_start_time(deps)?),
        QueryMsg::MintPrice {} => to_json_binary(&query_mint_price(deps, env)?),
        QueryMsg::MintCount { address } => to_json_binary(&query_mint_count(deps, address)?),
        QueryMsg::CanMint { address } => to_json_binary(&query_can_mint(deps, env, address)?),
        QueryMsg::MintStats {} => to_json_binary(&query_mint_stats(deps)?),
        QueryMsg::IsMintingActive {} => to_json_binary(&query_is_minting_active(deps, env)?),
        QueryMsg::NativeAssetTemplate {} => to_json_binary(&query_native_asset_template(deps)?),
        QueryMsg::TokenNativeAssets { token_id } => {
            to_json_binary(&query_token_native_assets(deps, token_id)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.into())
}

fn query_mintable_num_tokens(deps: Deps) -> StdResult<MintableNumTokensResponse> {
    let count = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    Ok(MintableNumTokensResponse { count })
}

fn query_start_time(deps: Deps) -> StdResult<StartTimeResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(StartTimeResponse {
        start_time: config.start_time,
    })
}

fn query_mint_price(deps: Deps, _env: Env) -> StdResult<MintPriceResponse> {
    let config = CONFIG.load(deps.storage)?;
    get_public_mint_price(deps, &_env, &config)
        .map_err(|err| cosmwasm_std::StdError::generic_err(err.to_string()))
}

fn query_mint_count(deps: Deps, address: String) -> StdResult<MintCountResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let count = MINTER_ADDRS.may_load(deps.storage, &addr)?.unwrap_or(0);
    Ok(MintCountResponse { address, count })
}

fn query_can_mint(deps: Deps, env: Env, address: String) -> StdResult<CanMintResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = deps.api.addr_validate(&address)?;

    if config.paused {
        return Ok(CanMintResponse {
            can_mint: false,
            reason: Some("Minting is paused".to_string()),
        });
    }

    if let Err(err) = validate_registry_requirements(deps, &config, &env.contract.address) {
        return Ok(CanMintResponse {
            can_mint: false,
            reason: Some(match err {
                ContractError::CollectionNotRegistered {} => {
                    "Collection is not registered in registry".to_string()
                }
                ContractError::MinterNotAuthorized {} => {
                    "Minter is not authorized in registry".to_string()
                }
                _ => "Registry validation failed".to_string(),
            }),
        });
    }

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    if remaining == 0 {
        return Ok(CanMintResponse {
            can_mint: false,
            reason: Some("Sold out".to_string()),
        });
    }

    let whitelist_config = get_active_whitelist_config(deps, &config)
        .map_err(|err| StdError::generic_err(err.to_string()))?;
    if let Some(wl_config) = whitelist_config {
        let member: HasMemberResponse = deps.querier.query_wasm_smart(
            config
                .whitelist
                .clone()
                .expect("active whitelist must exist")
                .to_string(),
            &WhitelistQueryMsg::HasMember {
                member: addr.to_string(),
            },
        )?;

        if !member.has_member {
            return Ok(CanMintResponse {
                can_mint: false,
                reason: Some(
                    "Whitelist minting is active but address is not whitelisted".to_string(),
                ),
            });
        }

        let count = MINTER_ADDRS.may_load(deps.storage, &addr)?.unwrap_or(0);
        if count >= wl_config.per_address_limit {
            return Ok(CanMintResponse {
                can_mint: false,
                reason: Some("Address has reached mint limit".to_string()),
            });
        }
    } else {
        if env.block.time < config.start_time {
            return Ok(CanMintResponse {
                can_mint: false,
                reason: Some("Minting has not started".to_string()),
            });
        }

        let count = MINTER_ADDRS.may_load(deps.storage, &addr)?.unwrap_or(0);
        if count >= config.per_address_limit {
            return Ok(CanMintResponse {
                can_mint: false,
                reason: Some("Address has reached mint limit".to_string()),
            });
        }
    }

    Ok(CanMintResponse {
        can_mint: true,
        reason: None,
    })
}

fn query_mint_stats(deps: Deps) -> StdResult<MintStatsResponse> {
    let stats = MINT_STATS.load(deps.storage)?;
    Ok(MintStatsResponse { stats })
}

fn query_is_minting_active(deps: Deps, env: Env) -> StdResult<IsMintingActiveResponse> {
    let config = CONFIG.load(deps.storage)?;

    if config.paused {
        return Ok(IsMintingActiveResponse {
            is_active: false,
            reason: Some("Minting is paused".to_string()),
        });
    }

    if let Err(err) = validate_registry_requirements(deps, &config, &env.contract.address) {
        return Ok(IsMintingActiveResponse {
            is_active: false,
            reason: Some(match err {
                ContractError::CollectionNotRegistered {} => {
                    "Collection is not registered in registry".to_string()
                }
                ContractError::MinterNotAuthorized {} => {
                    "Minter is not authorized in registry".to_string()
                }
                _ => "Registry validation failed".to_string(),
            }),
        });
    }

    let whitelist_active = get_active_whitelist_config(deps, &config)
        .map_err(|err| StdError::generic_err(err.to_string()))?
        .is_some();
    if !whitelist_active && env.block.time < config.start_time {
        return Ok(IsMintingActiveResponse {
            is_active: false,
            reason: Some("Minting has not started".to_string()),
        });
    }

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    if remaining == 0 {
        return Ok(IsMintingActiveResponse {
            is_active: false,
            reason: Some("Sold out".to_string()),
        });
    }

    Ok(IsMintingActiveResponse {
        is_active: true,
        reason: None,
    })
}

fn query_native_asset_template(deps: Deps) -> StdResult<NativeAssetTemplateResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(NativeAssetTemplateResponse {
        native_assets: config.native_asset_template,
    })
}

fn query_token_native_assets(deps: Deps, token_id: u32) -> StdResult<TokenNativeAssetsResponse> {
    let config = CONFIG.load(deps.storage)?;
    let override_assets = TOKEN_NATIVE_ASSET_OVERRIDES.may_load(deps.storage, token_id)?;

    match override_assets {
        Some(native_assets) => Ok(TokenNativeAssetsResponse {
            token_id,
            source: "override".to_string(),
            native_assets,
        }),
        None => Ok(TokenNativeAssetsResponse {
            token_id,
            source: "template".to_string(),
            native_assets: config.native_asset_template,
        }),
    }
}
