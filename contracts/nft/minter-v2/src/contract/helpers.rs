use super::*;
use crate::state::{Extension, NftType, TokenMetadata};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Pg721ExecuteMsg {
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Extension,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Pg721QueryMsg {
    CollectionInfo {},
}

#[derive(Deserialize)]
struct Pg721CollectionInfoResponse {
    nft_type: NftType,
}

#[derive(Debug)]
pub(super) struct MintContext {
    pub price: Coin,
    pub per_address_limit: u32,
    pub is_whitelist: bool,
}

// ========== Helpers ==========

pub(super) fn validate_mint_conditions(
    deps: &DepsMut,
    env: &Env,
    info: &MessageInfo,
    config: &Config,
) -> Result<(), ContractError> {
    let _ = resolve_mint_context(
        deps.as_ref(),
        env,
        &info.sender,
        config,
        &env.contract.address,
    )?;
    Ok(())
}

pub(super) fn get_current_price(
    deps: &DepsMut,
    env: &Env,
    info: &MessageInfo,
    config: &Config,
) -> Result<(Coin, bool), ContractError> {
    let context = resolve_mint_context(
        deps.as_ref(),
        env,
        &info.sender,
        config,
        &env.contract.address,
    )?;
    Ok((context.price, context.is_whitelist))
}

pub(super) fn get_effective_per_address_limit(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    config: &Config,
    minter_addr: &Addr,
) -> Result<u32, ContractError> {
    Ok(resolve_mint_context(deps, env, sender, config, minter_addr)?.per_address_limit)
}

pub(super) fn validate_registry_requirements(
    deps: Deps,
    config: &Config,
    minter_addr: &Addr,
) -> Result<(), ContractError> {
    validate_collection_requirements(deps, config)?;

    let Some(registry) = &config.registry else {
        return Ok(());
    };

    let minter_auth: RegistryMinterAuthorizedResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::IsMinterAuthorized {
                collection_address: config.cw721_address.to_string(),
                minter_address: minter_addr.to_string(),
            },
        )
        .map_err(|_| ContractError::MinterNotAuthorized {})?;

    if !minter_auth.is_authorized {
        return Err(ContractError::MinterNotAuthorized {});
    }

    Ok(())
}

pub(super) fn validate_collection_requirements(
    deps: Deps,
    config: &Config,
) -> Result<(), ContractError> {
    let Some(registry) = &config.registry else {
        return Ok(());
    };

    let collection: RegistryCollectionResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::Collection {
                address: config.cw721_address.to_string(),
            },
        )
        .map_err(|_| ContractError::CollectionNotRegistered {})?;

    if collection.collection.is_none() {
        return Err(ContractError::CollectionNotRegistered {});
    }

    let mint_allowed: RegistryApprovalStatusResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::CanMintCollection {
                address: config.cw721_address.to_string(),
            },
        )
        .map_err(|_| ContractError::CollectionMintDisabled {})?;

    if !mint_allowed.approved {
        return Err(ContractError::CollectionMintDisabled {});
    }

    Ok(())
}

pub(super) fn get_public_mint_price(
    deps: Deps,
    env: &Env,
    config: &Config,
) -> Result<MintPriceResponse, ContractError> {
    let whitelist_price = if let Some(whitelist) = &config.whitelist {
        let wl_config: WhitelistConfigResponse = deps
            .querier
            .query_wasm_smart(whitelist.to_string(), &WhitelistQueryMsg::Config {})?;
        Some(wl_config.unit_price)
    } else {
        None
    };

    let current_price = match get_active_whitelist_config(deps, config)? {
        Some(wl_config) => wl_config.unit_price,
        None => config.unit_price.clone(),
    };

    let _ = env;

    Ok(MintPriceResponse {
        public_price: config.unit_price.clone(),
        whitelist_price,
        current_price,
    })
}

fn resolve_mint_context(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    config: &Config,
    minter_addr: &Addr,
) -> Result<MintContext, ContractError> {
    if config.paused {
        return Err(ContractError::MintingPaused {});
    }

    validate_registry_requirements(deps, config, minter_addr)?;

    let remaining = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    if remaining == 0 {
        return Err(ContractError::SoldOut {});
    }

    let whitelist_config = get_active_whitelist_config(deps, config)?;
    let context = if let Some(wl_config) = whitelist_config {
        ensure_whitelist_member(deps, config, sender)?;
        MintContext {
            price: wl_config.unit_price,
            per_address_limit: wl_config.per_address_limit,
            is_whitelist: true,
        }
    } else {
        if env.block.time < config.start_time {
            return Err(ContractError::MintingNotStarted {
                start_time: config.start_time.to_string(),
            });
        }

        MintContext {
            price: config.unit_price.clone(),
            per_address_limit: config.per_address_limit,
            is_whitelist: false,
        }
    };

    let count = MINTER_ADDRS.may_load(deps.storage, sender)?.unwrap_or(0);
    if count >= context.per_address_limit {
        return Err(ContractError::MaxMintLimitReached {
            limit: context.per_address_limit,
        });
    }

    Ok(context)
}

pub(super) fn get_active_whitelist_config(
    deps: Deps,
    config: &Config,
) -> Result<Option<WhitelistConfigResponse>, ContractError> {
    let Some(whitelist) = &config.whitelist else {
        return Ok(None);
    };

    let wl_config: WhitelistConfigResponse = deps
        .querier
        .query_wasm_smart(whitelist.to_string(), &WhitelistQueryMsg::Config {})?;

    if wl_config.is_active {
        Ok(Some(wl_config))
    } else {
        Ok(None)
    }
}

fn ensure_whitelist_member(
    deps: Deps,
    config: &Config,
    sender: &Addr,
) -> Result<(), ContractError> {
    let Some(whitelist) = &config.whitelist else {
        return Ok(());
    };

    let member: HasMemberResponse = deps.querier.query_wasm_smart(
        whitelist.to_string(),
        &WhitelistQueryMsg::HasMember {
            member: sender.to_string(),
        },
    )?;

    if !member.has_member {
        return Err(ContractError::NotWhitelisted {});
    }

    Ok(())
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

pub(super) fn query_collection_nft_type(
    deps: Deps,
    config: &Config,
) -> Result<NftType, ContractError> {
    let response: Pg721CollectionInfoResponse = deps.querier.query_wasm_smart(
        config.cw721_address.to_string(),
        &Pg721QueryMsg::CollectionInfo {},
    )?;

    Ok(response.nft_type)
}

pub(super) fn create_mint_msg(
    config: &Config,
    nft_type: &NftType,
    token_id: u32,
    owner: String,
) -> Result<CosmosMsg, ContractError> {
    let token_uri = format!("{}/{}", config.base_token_uri, token_id);

    let exec_msg = Pg721ExecuteMsg::Mint {
        token_id: token_id.to_string(),
        owner,
        token_uri: Some(token_uri),
        extension: Some(TokenMetadata {
            nft_type: nft_type.clone(),
            extension: None,
        }),
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

    if count == 0 {
        let mut stats = MINT_STATS.load(storage)?;
        stats.unique_minters += 1;
        MINT_STATS.save(storage, &stats)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
