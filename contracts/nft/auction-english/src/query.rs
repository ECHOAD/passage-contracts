use crate::{
    helpers::{auction_is_settleable, validate_collection_registration},
    msg::{AuctionResponse, AuctionsResponse, CanTradeResponse, ConfigResponse, QueryMsg},
    state::{auctions, AuctionStatus, CONFIG},
};
use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::Bound;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CanTrade { collection } => to_json_binary(&query_can_trade(deps, collection)?),
        QueryMsg::Auction {
            collection,
            token_id,
        } => to_json_binary(&query_auction(deps, env, collection, token_id)?),
        QueryMsg::AuctionsByCollection {
            collection,
            start_after,
            limit,
        } => to_json_binary(&query_auctions_by_collection(
            deps,
            collection,
            start_after,
            limit,
        )?),
        QueryMsg::AuctionsBySeller { seller, limit } => {
            to_json_binary(&query_auctions_by_seller(deps, seller, limit)?)
        }
        QueryMsg::AuctionsByEndTime {
            start_after,
            limit,
            descending,
        } => to_json_binary(&query_auctions_by_end_time(
            deps,
            start_after,
            limit,
            descending,
        )?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(CONFIG.load(deps.storage)?.into())
}

fn query_can_trade(deps: Deps, collection: String) -> StdResult<CanTradeResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    match validate_collection_registration(deps, &config, &collection_addr) {
        Ok(()) => Ok(CanTradeResponse {
            can_trade: true,
            reason: None,
        }),
        Err(err) => Ok(CanTradeResponse {
            can_trade: false,
            reason: Some(err.to_string()),
        }),
    }
}

fn query_auction(
    deps: Deps,
    env: Env,
    collection: String,
    token_id: String,
) -> StdResult<AuctionResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let auction = auctions().may_load(deps.storage, (collection_addr, token_id))?;

    let (status, min_bid, can_settle) = match &auction {
        Some(auction) => {
            let status = auction.status(env.block.time);
            let min_bid = match status {
                AuctionStatus::Ended => None,
                _ => {
                    Some(auction.min_bid_coin(CONFIG.load(deps.storage)?.min_bid_increment_percent))
                }
            };
            let can_settle = auction_is_settleable(auction, env.block.time);
            (Some(status), min_bid, can_settle)
        }
        None => (None, None, false),
    };

    Ok(AuctionResponse {
        auction,
        status,
        min_bid,
        can_settle,
    })
}

fn query_auctions_by_collection(
    deps: Deps,
    collection: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AuctionsResponse> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|token_id| Bound::exclusive((collection_addr.clone(), token_id)));

    let auction_list = auctions()
        .idx
        .collection
        .prefix(collection_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, auction)| auction))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AuctionsResponse {
        auctions: auction_list,
    })
}

fn query_auctions_by_seller(
    deps: Deps,
    seller: String,
    limit: Option<u32>,
) -> StdResult<AuctionsResponse> {
    let seller_addr = deps.api.addr_validate(&seller)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let auction_list = auctions()
        .idx
        .seller
        .prefix(seller_addr)
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, auction)| auction))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AuctionsResponse {
        auctions: auction_list,
    })
}

fn query_auctions_by_end_time(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
    descending: Option<bool>,
) -> StdResult<AuctionsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let order = if descending.unwrap_or(false) {
        Order::Descending
    } else {
        Order::Ascending
    };

    let auction_list = auctions()
        .idx
        .end_time
        .range(deps.storage, None, None, order)
        .filter_map(|item| item.ok().map(|(_, auction)| auction))
        .filter(|auction| {
            auction.end_time.is_some()
                && start_after.map_or(true, |start| {
                    let end = auction.end_time.unwrap().seconds();
                    if descending.unwrap_or(false) {
                        end < start
                    } else {
                        end > start
                    }
                })
        })
        .take(limit)
        .collect::<Vec<_>>();

    Ok(AuctionsResponse {
        auctions: auction_list,
    })
}
