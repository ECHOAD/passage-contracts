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
        } => to_json_binary(&query_collection_configs(deps, start_after, limit, active_only)?),
        QueryMsg::CanTrade { collection } => {
            to_json_binary(&query_can_trade(deps, collection)?)
        }
        QueryMsg::CollectionDenom { collection } => {
            to_json_binary(&query_collection_denom(deps, collection)?)
        }
        QueryMsg::CollectionFee { collection } => {
            to_json_binary(&query_collection_fee(deps, collection)?)
        }
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
                if active_only && (!config.active || config.blacklisted) {
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

    // If registration is not required, any collection can trade
    if !config.require_registration {
        return Ok(CanTradeResponse {
            can_trade: true,
            reason: None,
        });
    }

    // Check if collection is registered
    let coll_config = COLLECTION_CONFIGS.may_load(deps.storage, collection_addr)?;

    match coll_config {
        None => Ok(CanTradeResponse {
            can_trade: false,
            reason: Some("Collection not registered".to_string()),
        }),
        Some(cfg) => {
            if cfg.blacklisted {
                Ok(CanTradeResponse {
                    can_trade: false,
                    reason: Some(format!(
                        "Collection blacklisted: {}",
                        cfg.blacklist_reason.unwrap_or_else(|| "Unknown".to_string())
                    )),
                })
            } else if !cfg.active {
                Ok(CanTradeResponse {
                    can_trade: false,
                    reason: Some("Collection not active".to_string()),
                })
            } else {
                Ok(CanTradeResponse {
                    can_trade: true,
                    reason: None,
                })
            }
        }
    }
}

fn query_collection_fee(deps: Deps, collection: String) -> StdResult<CollectionFeeResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    let coll_config = COLLECTION_CONFIGS.may_load(deps.storage, collection_addr)?;

    let (trading_fee_bps, is_override) = match coll_config {
        Some(cfg) => match cfg.trading_fee_bps {
            Some(fee) => (fee, true),
            None => (config.trading_fee_bps, false),
        },
        None => (config.trading_fee_bps, false),
    };

    Ok(CollectionFeeResponse {
        collection,
        trading_fee_bps,
        is_override,
    })
}

fn query_collection_denom(deps: Deps, collection: String) -> StdResult<CollectionDenomResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    let override_denom = COLLECTION_DENOMS.may_load(deps.storage, collection_addr)?;
    let (denom, is_override) = match override_denom {
        Some(override_denom) => (override_denom, true),
        None => (config.denom, false),
    };

    Ok(CollectionDenomResponse {
        collection,
        denom,
        is_override,
    })
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
    _start_after: Option<(String, TokenId)>,
    limit: Option<u32>,
) -> StdResult<AsksResponse> {
    let seller_addr = deps.api.addr_validate(&seller)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let ask_list: Vec<Ask> = asks()
        .idx
        .seller
        .prefix(seller_addr)
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks: ask_list })
}

fn query_asks_by_price(
    deps: Deps,
    _collection: Option<String>,
    start_after: Option<u128>,
    limit: Option<u32>,
    descending: Option<bool>,
) -> StdResult<AsksResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let order = if descending.unwrap_or(false) {
        Order::Descending
    } else {
        Order::Ascending
    };
    let start =
        start_after.map(|price| Bound::exclusive((price, (Addr::unchecked(""), String::new()))));

    let ask_list: Vec<Ask> = asks()
        .idx
        .price
        .range(deps.storage, start, None, order)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks: ask_list })
}

fn query_ask_count(deps: Deps, _collection: Option<String>) -> StdResult<CountResponse> {
    let count = asks()
        .range(deps.storage, None, None, Order::Ascending)
        .count() as u64;
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
    _start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<BidsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let bid_list: Vec<Bid> = bids()
        .idx
        .token
        .prefix((collection_addr, token_id))
        .range(deps.storage, None, None, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(BidsResponse { bids: bid_list })
}

fn query_bids_by_bidder(
    deps: Deps,
    bidder: String,
    _start_after: Option<(String, TokenId)>,
    limit: Option<u32>,
) -> StdResult<BidsResponse> {
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let bid_list: Vec<Bid> = bids()
        .idx
        .bidder
        .prefix(bidder_addr)
        .range(deps.storage, None, None, Order::Ascending)
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
    _start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionBidsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let bid_list: Vec<CollectionBid> = collection_bids()
        .idx
        .collection
        .prefix(collection_addr)
        .range(deps.storage, None, None, Order::Descending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CollectionBidsResponse { bids: bid_list })
}

fn query_collection_bids_by_bidder(
    deps: Deps,
    bidder: String,
    _start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionBidsResponse> {
    let bidder_addr = deps.api.addr_validate(&bidder)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let bid_list: Vec<CollectionBid> = collection_bids()
        .idx
        .bidder
        .prefix(bidder_addr)
        .range(deps.storage, None, None, Order::Ascending)
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
    let trading_fee_bps = resolve_collection_trading_fee(deps.storage, &config, &collection_addr)?;

    let platform_fee = price.multiply_ratio(trading_fee_bps as u128, 10_000u128);

    // Query royalty from collection
    let royalty = query_royalty_amount(&deps, &collection_addr, price).unwrap_or(Uint128::zero());

    let seller_proceeds = price - platform_fee - royalty;

    Ok(SalePreviewResponse {
        sale_price: price,
        platform_fee,
        royalty,
        seller_proceeds,
    })
}
