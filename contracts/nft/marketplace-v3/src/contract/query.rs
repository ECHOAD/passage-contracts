use super::helpers::*;
use super::*;

// ========== Query ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        // Collection queries
        QueryMsg::CollectionConfig { collection } => {
            to_json_binary(&query_collection_config(deps, collection)?)
        }
        QueryMsg::CollectionConfigs {
            start_after,
            limit,
            active_only,
        } => to_json_binary(&query_collection_configs(
            deps,
            start_after,
            limit,
            active_only,
        )?),
        QueryMsg::CanTrade { collection } => to_json_binary(&query_can_trade(deps, collection)?),
        QueryMsg::CollectionDenom { collection } => {
            to_json_binary(&query_collection_denom(deps, collection)?)
        }
        QueryMsg::CollectionFee { collection } => {
            to_json_binary(&query_collection_fee(deps, collection)?)
        }
        QueryMsg::CollectionRegistrationRequest { collection } => {
            to_json_binary(&query_collection_registration_request(deps, collection)?)
        }
        QueryMsg::CollectionRegistrationRequests {
            status,
            start_after,
            limit,
        } => to_json_binary(&query_collection_registration_requests(
            deps,
            status,
            start_after,
            limit,
        )?),
        QueryMsg::CollectionUpdateRequest { collection } => {
            to_json_binary(&query_collection_update_request(deps, collection)?)
        }
        QueryMsg::CollectionUpdateRequests {
            status,
            start_after,
            limit,
        } => to_json_binary(&query_collection_update_requests(
            deps,
            status,
            start_after,
            limit,
        )?),
        QueryMsg::Ask {
            collection,
            token_id,
        } => to_json_binary(&query_ask(deps, collection, token_id)?),
        QueryMsg::AsksByCollection {
            collection,
            start_after,
            limit,
        } => to_json_binary(&query_asks_by_collection(
            deps,
            collection,
            start_after,
            limit,
        )?),
        QueryMsg::AsksBySeller {
            seller,
            start_after,
            limit,
        } => to_json_binary(&query_asks_by_seller(deps, seller, start_after, limit)?),
        QueryMsg::AsksByPrice {
            collection,
            start_after,
            limit,
            descending,
        } => to_json_binary(&query_asks_by_price(
            deps,
            collection,
            start_after,
            limit,
            descending,
        )?),
        QueryMsg::AskCount { collection } => to_json_binary(&query_ask_count(deps, collection)?),
        QueryMsg::Bid {
            collection,
            token_id,
            bidder,
        } => to_json_binary(&query_bid(deps, collection, token_id, bidder)?),
        QueryMsg::BidsByToken {
            collection,
            token_id,
            start_after,
            limit,
        } => to_json_binary(&query_bids_by_token(
            deps,
            collection,
            token_id,
            start_after,
            limit,
        )?),
        QueryMsg::BidsByBidder {
            bidder,
            start_after,
            limit,
        } => to_json_binary(&query_bids_by_bidder(deps, bidder, start_after, limit)?),
        QueryMsg::CollectionBid { collection, bidder } => {
            to_json_binary(&query_collection_bid(deps, collection, bidder)?)
        }
        QueryMsg::CollectionBidsByCollection {
            collection,
            start_after,
            limit,
        } => to_json_binary(&query_collection_bids_by_collection(
            deps,
            collection,
            start_after,
            limit,
        )?),
        QueryMsg::CollectionBidsByBidder {
            bidder,
            start_after,
            limit,
        } => to_json_binary(&query_collection_bids_by_bidder(
            deps,
            bidder,
            start_after,
            limit,
        )?),
        QueryMsg::MarketStats {} => to_json_binary(&query_market_stats(deps)?),
        QueryMsg::CollectionStats { collection } => {
            to_json_binary(&query_collection_stats(deps, collection)?)
        }
        QueryMsg::PreviewSale { collection, price } => {
            to_json_binary(&query_preview_sale(deps, collection, price)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.into())
}

fn query_collection_config(deps: Deps, collection: String) -> StdResult<CollectionConfigResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let config = COLLECTION_CONFIGS.may_load(deps.storage, collection_addr)?;
    Ok(CollectionConfigResponse { config })
}

fn query_collection_configs(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
    active_only: Option<bool>,
) -> StdResult<CollectionConfigsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|s| deps.api.addr_validate(&s))
        .transpose()?
        .map(Bound::exclusive);

    let active_only = active_only.unwrap_or(false);

    let configs: Vec<CollectionConfig> = COLLECTION_CONFIGS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, config)| {
                if active_only && !config.active {
                    None
                } else {
                    Some(config)
                }
            })
        })
        .take(limit)
        .collect();

    Ok(CollectionConfigsResponse { configs })
}

fn query_can_trade(deps: Deps, collection: String) -> StdResult<CanTradeResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    match validate_collection(deps.storage, &config, &collection_addr, &deps) {
        Ok(()) => Ok(CanTradeResponse {
            can_trade: true,
            reason: None,
        }),
        Err(ContractError::CollectionNotRegistered { .. }) => Ok(CanTradeResponse {
            can_trade: false,
            reason: Some("Collection not registered".to_string()),
        }),
        Err(ContractError::CollectionNotActive { .. }) => Ok(CanTradeResponse {
            can_trade: false,
            reason: Some("Collection not active on marketplace".to_string()),
        }),
        Err(ContractError::CollectionTradingDisabled { .. }) => Ok(CanTradeResponse {
            can_trade: false,
            reason: Some("Collection blocked by registry moderation".to_string()),
        }),
        Err(err) => Err(cosmwasm_std::StdError::generic_err(err.to_string())),
    }
}

fn query_collection_fee(deps: Deps, collection: String) -> StdResult<CollectionFeeResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(CollectionFeeResponse {
        collection,
        trading_fee_bps: config.trading_fee_bps,
        is_override: false,
    })
}

fn query_collection_denom(deps: Deps, collection: String) -> StdResult<CollectionDenomResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let coll_config = COLLECTION_CONFIGS
        .may_load(deps.storage, collection_addr)?
        .ok_or_else(|| cosmwasm_std::StdError::generic_err("Collection not registered"))?;

    Ok(CollectionDenomResponse {
        collection,
        denom: coll_config.denom,
        is_override: true,
    })
}

fn query_collection_registration_request(
    deps: Deps,
    collection: String,
) -> StdResult<CollectionRegistrationRequestResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let request = COLLECTION_REGISTRATION_REQUESTS.may_load(deps.storage, collection_addr)?;
    Ok(CollectionRegistrationRequestResponse { request })
}

fn query_collection_registration_requests(
    deps: Deps,
    status: Option<CollectionRequestStatus>,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionRegistrationRequestsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|value| deps.api.addr_validate(&value))
        .transpose()?
        .map(Bound::exclusive);

    let requests = COLLECTION_REGISTRATION_REQUESTS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, request)| match &status {
                Some(status) if request.status != *status => None,
                _ => Some(request),
            })
        })
        .take(limit)
        .collect();

    Ok(CollectionRegistrationRequestsResponse { requests })
}

fn query_collection_update_request(
    deps: Deps,
    collection: String,
) -> StdResult<CollectionUpdateRequestResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let request = COLLECTION_UPDATE_REQUESTS.may_load(deps.storage, collection_addr)?;
    Ok(CollectionUpdateRequestResponse { request })
}

fn query_collection_update_requests(
    deps: Deps,
    status: Option<CollectionRequestStatus>,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionUpdateRequestsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|value| deps.api.addr_validate(&value))
        .transpose()?
        .map(Bound::exclusive);

    let requests = COLLECTION_UPDATE_REQUESTS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, request)| match &status {
                Some(status) if request.status != *status => None,
                _ => Some(request),
            })
        })
        .take(limit)
        .collect();

    Ok(CollectionUpdateRequestsResponse { requests })
}

fn query_ask(deps: Deps, collection: String, token_id: TokenId) -> StdResult<AskResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let ask_key = (collection_addr, token_id);
    let ask = asks().may_load(deps.storage, ask_key)?;
    Ok(AskResponse { ask })
}

fn query_asks_by_collection(
    deps: Deps,
    collection: String,
    start_after: Option<TokenId>,
    limit: Option<u32>,
) -> StdResult<AsksResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|token_id| Bound::exclusive((collection_addr.clone(), token_id)));

    let ask_list: Vec<Ask> = asks()
        .idx
        .collection
        .prefix(collection_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks: ask_list })
}

fn query_asks_by_seller(
    deps: Deps,
    seller: String,
    start_after: Option<(String, TokenId)>,
    limit: Option<u32>,
) -> StdResult<AsksResponse> {
    let seller_addr = deps.api.addr_validate(&seller)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|(collection, token_id)| {
            deps.api
                .addr_validate(&collection)
                .map(|collection_addr| Bound::exclusive((collection_addr, token_id)))
        })
        .transpose()?;

    let ask_list: Vec<Ask> = asks()
        .idx
        .seller
        .prefix(seller_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks: ask_list })
}

fn query_asks_by_price(
    deps: Deps,
    collection: Option<String>,
    start_after: Option<u128>,
    limit: Option<u32>,
    descending: Option<bool>,
) -> StdResult<AsksResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let descending = descending.unwrap_or(false);
    let order = if descending {
        Order::Descending
    } else {
        Order::Ascending
    };
    let start =
        start_after.map(|price| Bound::exclusive((price, (Addr::unchecked(""), String::new()))));
    let collection_addr = collection
        .map(|collection| deps.api.addr_validate(&collection))
        .transpose()?;
    let (min, max) = if descending {
        (None, start)
    } else {
        (start, None)
    };

    let ask_list: Vec<Ask> = asks()
        .idx
        .price
        .range(deps.storage, min, max, order)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?
        .into_iter()
        .filter(|ask| {
            collection_addr
                .as_ref()
                .map_or(true, |collection_addr| &ask.collection == collection_addr)
        })
        .take(limit)
        .collect();

    Ok(AsksResponse { asks: ask_list })
}

fn query_ask_count(deps: Deps, collection: Option<String>) -> StdResult<CountResponse> {
    let count = if let Some(collection) = collection {
        let collection_addr = deps.api.addr_validate(&collection)?;
        asks()
            .idx
            .collection
            .prefix(collection_addr)
            .range(deps.storage, None, None, Order::Ascending)
            .count() as u64
    } else {
        asks()
            .range(deps.storage, None, None, Order::Ascending)
            .count() as u64
    };
    Ok(CountResponse { count })
}

fn query_bid(
    deps: Deps,
    collection: String,
    token_id: TokenId,
    bidder: String,
) -> StdResult<BidResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let bid_key = (collection_addr, token_id, bidder_addr);
    let bid = bids().may_load(deps.storage, bid_key)?;
    Ok(BidResponse { bid })
}

fn query_bids_by_token(
    deps: Deps,
    collection: String,
    token_id: TokenId,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<BidsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let end = start_after
        .map(|bidder| {
            deps.api.addr_validate(&bidder).map(|bidder_addr| {
                Bound::exclusive((collection_addr.clone(), token_id.clone(), bidder_addr))
            })
        })
        .transpose()?;

    let bid_list: Vec<Bid> = bids()
        .idx
        .token
        .prefix((collection_addr, token_id))
        .range(deps.storage, None, end, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(BidsResponse { bids: bid_list })
}

fn query_bids_by_bidder(
    deps: Deps,
    bidder: String,
    start_after: Option<(String, TokenId)>,
    limit: Option<u32>,
) -> StdResult<BidsResponse> {
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|(collection, token_id)| {
            deps.api.addr_validate(&collection).map(|collection_addr| {
                Bound::exclusive((collection_addr, token_id, bidder_addr.clone()))
            })
        })
        .transpose()?;

    let bid_list: Vec<Bid> = bids()
        .idx
        .bidder
        .prefix(bidder_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(BidsResponse { bids: bid_list })
}

fn query_collection_bid(
    deps: Deps,
    collection: String,
    bidder: String,
) -> StdResult<CollectionBidResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let bid_key = (collection_addr, bidder_addr);
    let bid = collection_bids().may_load(deps.storage, bid_key)?;
    Ok(CollectionBidResponse { bid })
}

fn query_collection_bids_by_collection(
    deps: Deps,
    collection: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionBidsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let end = start_after
        .map(|bidder| {
            deps.api
                .addr_validate(&bidder)
                .map(|bidder_addr| Bound::exclusive((collection_addr.clone(), bidder_addr)))
        })
        .transpose()?;

    let bid_list: Vec<CollectionBid> = collection_bids()
        .idx
        .collection
        .prefix(collection_addr)
        .range(deps.storage, None, end, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionBidsResponse { bids: bid_list })
}

fn query_collection_bids_by_bidder(
    deps: Deps,
    bidder: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionBidsResponse> {
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after
        .map(|collection| {
            deps.api
                .addr_validate(&collection)
                .map(|collection_addr| Bound::exclusive((collection_addr, bidder_addr.clone())))
        })
        .transpose()?;

    let bid_list: Vec<CollectionBid> = collection_bids()
        .idx
        .bidder
        .prefix(bidder_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionBidsResponse { bids: bid_list })
}

fn query_market_stats(deps: Deps) -> StdResult<MarketStatsResponse> {
    let stats = MARKET_STATS.load(deps.storage)?;
    Ok(MarketStatsResponse { stats })
}

fn query_collection_stats(deps: Deps, collection: String) -> StdResult<CollectionStatsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let stats = COLLECTION_STATS
        .may_load(deps.storage, collection_addr)?
        .unwrap_or_default();
    Ok(CollectionStatsResponse { stats })
}

fn query_preview_sale(
    deps: Deps,
    collection: String,
    price: Uint128,
) -> StdResult<SalePreviewResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Get effective trading fee for this collection
    let trading_fee_bps = resolve_collection_trading_fee(&config)?;

    let trading_fee = price.multiply_ratio(trading_fee_bps as u128, 10_000u128);

    // Query royalty from collection
    let royalty = query_royalty_amount(&deps, &collection_addr, price).unwrap_or(Uint128::zero());

    let seller_proceeds = price - trading_fee - royalty;

    Ok(SalePreviewResponse {
        sale_price: price,
        trading_fee,
        royalty,
        seller_proceeds,
    })
}

#[cfg(test)]
mod tests;
