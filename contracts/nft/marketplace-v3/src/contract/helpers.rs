use super::*;

// ========== Helpers ==========

pub(super) fn resolve_collection_denom(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Addr,
) -> StdResult<String> {
    Ok(COLLECTION_DENOMS
        .may_load(storage, collection.clone())?
        .unwrap_or_else(|| config.denom.clone()))
}

pub(super) fn validate_collection(config: &Config, collection: &Addr) -> Result<(), ContractError> {
    if !config.allow_any_collection && !config.supported_collections.contains(collection) {
        return Err(ContractError::CollectionNotSupported {
            collection: collection.to_string(),
        });
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

    // Calculate fees
    let platform_fee = price.multiply_ratio(config.trading_fee_bps as u128, 10_000u128);
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
