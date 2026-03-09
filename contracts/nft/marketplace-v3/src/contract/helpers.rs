use super::*;

// ========== Helpers ==========

/// Resolve the effective denom for a collection
/// Priority: CollectionConfig.denom > Config.denom
pub(super) fn resolve_collection_denom(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
) -> StdResult<String> {
    // First check new CollectionConfig
    if let Some(coll_config) = COLLECTION_CONFIGS.may_load(storage, collection.clone())? {
        return Ok(coll_config.get_denom(&config.denom));
    }
    // Fallback to legacy COLLECTION_DENOMS for migration compatibility
    Ok(COLLECTION_DENOMS
        .may_load(storage, collection.clone())?
        .unwrap_or_else(|| config.denom.clone()))
}

/// Resolve the effective trading fee for a collection
/// Priority: CollectionConfig.trading_fee_bps > Config.trading_fee_bps
pub(super) fn resolve_collection_trading_fee(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
) -> StdResult<u64> {
    if let Some(coll_config) = COLLECTION_CONFIGS.may_load(storage, collection.clone())? {
        return Ok(coll_config.get_trading_fee_bps(config.trading_fee_bps));
    }
    Ok(config.trading_fee_bps)
}

/// Validate that a collection can be traded on the marketplace
/// Checks: registration (if required), active status, blacklist status
pub(super) fn validate_collection(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
) -> Result<(), ContractError> {
    // If registration is required, check CollectionConfig
    if config.require_registration {
        let coll_config = COLLECTION_CONFIGS
            .may_load(storage, collection.clone())?
            .ok_or_else(|| ContractError::CollectionNotRegistered {
                collection: collection.to_string(),
            })?;

        // Check if blacklisted
        if coll_config.blacklisted {
            return Err(ContractError::CollectionBlacklisted {
                collection: collection.to_string(),
                reason: coll_config
                    .blacklist_reason
                    .unwrap_or_else(|| "Unknown".to_string()),
            });
        }

        // Check if active
        if !coll_config.active {
            return Err(ContractError::CollectionNotActive {
                collection: collection.to_string(),
            });
        }
    }

    Ok(())
}

/// Get collection config, returning error if not found (when registration required)
pub(super) fn get_collection_config(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
) -> Result<Option<CollectionConfig>, ContractError> {
    let coll_config = COLLECTION_CONFIGS.may_load(storage, collection.clone())?;

    if config.require_registration && coll_config.is_none() {
        return Err(ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        });
    }

    Ok(coll_config)
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

pub(super) fn verify_nft_ownership(
    deps: &DepsMut,
    collection: &Addr,
    token_id: &str,
    expected_owner: &Addr,
) -> Result<(), ContractError> {
    let owner = query_nft_owner(deps, collection, token_id)?;
    if owner != *expected_owner {
        return Err(ContractError::NotTokenOwner {
            token_id: token_id.to_string(),
        });
    }
    Ok(())
}

pub(super) fn query_nft_owner(
    deps: &DepsMut,
    collection: &Addr,
    token_id: &str,
) -> Result<Addr, ContractError> {
    let query_msg = Cw721QueryMsg::OwnerOf {
        token_id: token_id.to_string(),
        include_expired: Some(false),
    };

    let res: OwnerOfResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&query_msg)?,
        }))
        .map_err(|_| ContractError::NftQueryFailed {})?;

    Ok(deps.api.addr_validate(&res.owner)?)
}

pub(super) fn query_royalty_amount(
    deps: &Deps,
    collection: &Addr,
    price: Uint128,
) -> Result<Uint128, ContractError> {
    let query_msg = Pg721QueryMsg::CollectionInfo {};

    let res: Result<CollectionInfoResponse, _> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&query_msg)?,
        }));

    match res {
        Ok(info) => {
            if let Some(royalty) = info.royalty_info {
                let share: Decimal = royalty.share.parse().unwrap_or(Decimal::zero());
                Ok(price
                    .multiply_ratio(share.atomics().u128(), 10u128.pow(Decimal::DECIMAL_PLACES)))
            } else {
                Ok(Uint128::zero())
            }
        }
        Err(_) => Ok(Uint128::zero()),
    }
}

pub(super) struct SaleInfo {
    pub(super) platform_fee: Uint128,
    pub(super) royalty: Uint128,
    pub(super) trading_fee_bps: u64,
}

pub(super) fn execute_sale(
    deps: &DepsMut,
    _env: &Env,
    config: &Config,
    collection: &Addr,
    token_id: &str,
    _seller: &Addr,
    buyer: &Addr,
    funds_recipient: &Addr,
    sale_denom: &str,
    price: Uint128,
) -> Result<(Vec<CosmosMsg>, SaleInfo), ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    // Get effective trading fee for this collection
    let trading_fee_bps = resolve_collection_trading_fee(deps.storage, config, collection)?;

    // Calculate fees
    let platform_fee = price.multiply_ratio(trading_fee_bps as u128, 10_000u128);
    let royalty =
        query_royalty_amount(&deps.as_ref(), collection, price).unwrap_or(Uint128::zero());
    let seller_amount = price - platform_fee - royalty;

    if config.use_revenue_router {
        if let Some(router) = &config.revenue_router {
            // Route through Revenue Router
            let route_msg = RevenueRouterExecuteMsg::RouteSecondarySale {
                collection: collection.to_string(),
                seller: funds_recipient.to_string(),
                royalty_amount: royalty,
            };

            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: router.to_string(),
                msg: to_json_binary(&route_msg)?,
                funds: vec![Coin {
                    denom: sale_denom.to_string(),
                    amount: price,
                }],
            }));
        } else {
            return Err(ContractError::RevenueRouterNotConfigured {});
        }
    } else {
        // Legacy direct distribution
        // Platform fee
        if !platform_fee.is_zero() {
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: config.fee_collector.to_string(),
                amount: vec![Coin {
                    denom: sale_denom.to_string(),
                    amount: platform_fee,
                }],
            }));
        }

        // Seller payment
        if !seller_amount.is_zero() {
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: funds_recipient.to_string(),
                amount: vec![Coin {
                    denom: sale_denom.to_string(),
                    amount: seller_amount,
                }],
            }));
        }

        // TODO: Query royalty address from collection and send royalty
    }

    // Transfer NFT to buyer
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
        recipient: buyer.to_string(),
        token_id: token_id.to_string(),
    };

    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&transfer_msg)?,
        funds: vec![],
    }));

    Ok((
        messages,
        SaleInfo {
            platform_fee,
            royalty,
            trading_fee_bps,
        },
    ))
}

pub(super) fn update_stats(
    storage: &mut dyn cosmwasm_std::Storage,
    collection: &Addr,
    volume: Uint128,
    fees: Uint128,
    royalty: Uint128,
) -> Result<(), ContractError> {
    // Update market stats
    let mut market_stats = MARKET_STATS.load(storage)?;
    market_stats.total_sales += 1;
    market_stats.total_volume += volume;
    market_stats.total_fees += fees;
    market_stats.total_royalties += royalty;
    MARKET_STATS.save(storage, &market_stats)?;

    // Update collection stats
    let mut coll_stats = COLLECTION_STATS
        .may_load(storage, collection.clone())?
        .unwrap_or_default();
    coll_stats.total_sales += 1;
    coll_stats.total_volume += volume;
    if volume > coll_stats.highest_sale {
        coll_stats.highest_sale = volume;
    }
    COLLECTION_STATS.save(storage, collection.clone(), &coll_stats)?;

    Ok(())
}
