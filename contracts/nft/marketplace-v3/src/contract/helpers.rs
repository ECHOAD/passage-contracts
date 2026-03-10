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
            let route_msg = SplitRouterExecuteMsg::RouteSecondaryRoyalty {
                collection: collection.to_string(),
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
mod tests {
    use super::*;
    use crate::msg::RoyaltyInfoResponse;
    use cosmwasm_std::{
        from_json,
        testing::{mock_dependencies, mock_env},
        ContractResult, OwnedDeps, SystemError, SystemResult, WasmQuery,
    };

    fn mock_collection_info(
        deps: &mut OwnedDeps<
            cosmwasm_std::testing::MockStorage,
            cosmwasm_std::testing::MockApi,
            cosmwasm_std::testing::MockQuerier,
        >,
        contract_addr: &str,
        royalty_recipient: &str,
        royalty_share: &str,
    ) {
        let contract_addr = contract_addr.to_string();
        let royalty_recipient = royalty_recipient.to_string();
        let royalty_share = royalty_share.to_string();

        deps.querier.update_wasm(move |query| match query {
            WasmQuery::Smart {
                contract_addr: addr,
                msg,
            } if addr == &contract_addr => {
                let parsed: Pg721QueryMsg = from_json(msg).unwrap();
                match parsed {
                    Pg721QueryMsg::CollectionInfo {} => SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&CollectionInfoResponse {
                            creator: "creator".to_string(),
                            description: "desc".to_string(),
                            image: "ipfs://image".to_string(),
                            external_link: None,
                            royalty_info: Some(RoyaltyInfoResponse {
                                payment_address: royalty_recipient.clone(),
                                share: royalty_share.clone(),
                            }),
                        })
                        .unwrap(),
                    )),
                }
            }
            WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
                addr: "unknown".to_string(),
            }),
            _ => SystemResult::Err(SystemError::UnsupportedRequest {
                kind: "unsupported wasm query".to_string(),
            }),
        });
    }

    #[test]
    fn legacy_sale_sends_royalty_payment() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let collection = Addr::unchecked("collection");
        let buyer = Addr::unchecked("buyer");
        let seller = Addr::unchecked("seller");
        let recipient = Addr::unchecked("seller_payout");
        let config = Config {
            admin: Addr::unchecked("admin"),
            denom: "upasg".to_string(),
            min_price: Uint128::new(1),
            trading_fee_bps: 250,
            max_trading_fee_bps: 1000,
            fee_collector: Addr::unchecked("treasury"),
            registry: None,
            split_router: None,
            use_split_router: false,
            operators: vec![],
            paused: false,
            require_registration: false,
        };
        let royalty_recipient = deps.api.addr_make("royalty-wallet");

        mock_collection_info(&mut deps, "collection", royalty_recipient.as_str(), "0.1");

        let (messages, sale_info) = execute_sale(
            &deps.as_mut(),
            &env,
            &config,
            &collection,
            "1",
            &seller,
            &buyer,
            &recipient,
            "upasg",
            Uint128::new(1_000),
        )
        .unwrap();

        assert_eq!(sale_info.trading_fee, Uint128::new(25));
        assert_eq!(sale_info.royalty, Uint128::new(100));
        assert_eq!(messages.len(), 4);

        let royalty_msg = &messages[2];
        match royalty_msg {
            CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, royalty_recipient.as_str());
                assert_eq!(amount[0].amount, Uint128::new(100));
            }
            _ => panic!("expected royalty bank send"),
        }
    }
}
