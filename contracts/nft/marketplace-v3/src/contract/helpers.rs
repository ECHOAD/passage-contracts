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
/// Checks: registration/local activation and registry moderation status
pub(super) fn validate_collection(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
    deps: &Deps,
) -> Result<(), ContractError> {
    if let Some(coll_config) = COLLECTION_CONFIGS.may_load(storage, collection.clone())? {
        if !coll_config.active {
            return Err(ContractError::CollectionNotActive {
                collection: collection.to_string(),
            });
        }
    } else if config.require_registration {
        return Err(ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        });
    }

    if let Some(registry) = &config.registry {
        let trade_allowed: RegistryApprovalStatusResponse = deps
            .querier
            .query_wasm_smart(
                registry.to_string(),
                &RegistryQueryMsg::CanTradeCollection {
                    address: collection.to_string(),
                },
            )
            .map_err(|_| ContractError::CollectionTradingDisabled {
                collection: collection.to_string(),
            })?;

        if !trade_allowed.approved {
            return Err(ContractError::CollectionTradingDisabled {
                collection: collection.to_string(),
            });
        }
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
    Ok(query_royalty_payout(deps, collection, price)?
        .map(|payout| payout.amount)
        .unwrap_or(Uint128::zero()))
}

pub(super) struct RoyaltyPayout {
    pub recipient: Addr,
    pub amount: Uint128,
}

pub(super) fn query_royalty_payout(
    deps: &Deps,
    collection: &Addr,
    price: Uint128,
) -> Result<Option<RoyaltyPayout>, ContractError> {
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
                let recipient = deps
                    .api
                    .addr_validate(&royalty.payment_address)
                    .map_err(|_| ContractError::RoyaltyQueryFailed {})?;
                let amount = price
                    .multiply_ratio(share.atomics().u128(), 10u128.pow(Decimal::DECIMAL_PLACES));
                Ok(Some(RoyaltyPayout { recipient, amount }))
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}

pub(super) struct SaleInfo {
    pub(super) trading_fee: Uint128,
    pub(super) royalty: Uint128,
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
    let trading_fee = price.multiply_ratio(trading_fee_bps as u128, 10_000u128);
    let royalty_payout = query_royalty_payout(&deps.as_ref(), collection, price).unwrap_or(None);
    let royalty = royalty_payout
        .as_ref()
        .map(|payout| payout.amount)
        .unwrap_or(Uint128::zero());
    let seller_amount = price - trading_fee - royalty;

    if !trading_fee.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_collector.to_string(),
            amount: vec![Coin {
                denom: sale_denom.to_string(),
                amount: trading_fee,
            }],
        }));
    }

    if !seller_amount.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: funds_recipient.to_string(),
            amount: vec![Coin {
                denom: sale_denom.to_string(),
                amount: seller_amount,
            }],
        }));
    }

    if config.use_split_router {
        if royalty.is_zero() {
            // No creator-side royalty to route.
        } else if let Some(router) = &config.split_router {
            let route_msg = SplitRouterExecuteMsg::Split {
                key: collection.to_string(),
            };

            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: router.to_string(),
                msg: to_json_binary(&route_msg)?,
                funds: vec![Coin {
                    denom: sale_denom.to_string(),
                    amount: royalty,
                }],
            }));
        } else {
            return Err(ContractError::SplitRouterNotConfigured {});
        }
    } else {
        if let Some(payout) = royalty_payout {
            if !payout.amount.is_zero() {
                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: payout.recipient.to_string(),
                    amount: vec![Coin {
                        denom: sale_denom.to_string(),
                        amount: payout.amount,
                    }],
                }));
            }
        }
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
            trading_fee,
            royalty,
        },
    ))
}

pub(super) fn update_stats(
    storage: &mut dyn cosmwasm_std::Storage,
    collection: &Addr,
    volume: Uint128,
    trading_fees: Uint128,
    royalty: Uint128,
) -> Result<(), ContractError> {
    // Update market stats
    let mut market_stats = MARKET_STATS.load(storage)?;
    market_stats.total_sales += 1;
    market_stats.total_volume += volume;
    market_stats.total_trading_fees += trading_fees;
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

#[cfg(test)]
mod tests;
