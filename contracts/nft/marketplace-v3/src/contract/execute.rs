use super::helpers::*;
use super::*;

// ========== Execute ==========

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check if paused (allow admin actions)
    if config.paused && config.admin != info.sender {
        return Err(ContractError::MarketplacePaused {});
    }

    match msg {
        // Admin
        ExecuteMsg::UpdateConfig {
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            revenue_router,
            use_revenue_router,
            operators,
            paused,
            require_registration,
        } => execute_update_config(
            deps,
            info,
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            revenue_router,
            use_revenue_router,
            operators,
            paused,
            require_registration,
        ),

        // Collection Registration
        ExecuteMsg::RegisterCollection {
            collection,
            trading_fee_bps,
            denom,
        } => execute_register_collection(deps, env, info, collection, trading_fee_bps, denom),
        ExecuteMsg::UpdateCollectionConfig {
            collection,
            active,
            trading_fee_bps,
            denom,
        } => execute_update_collection_config(deps, env, info, collection, active, trading_fee_bps, denom),
        ExecuteMsg::DeactivateCollection { collection, reason } => {
            execute_deactivate_collection(deps, env, info, collection, reason)
        }
        ExecuteMsg::ReactivateCollection { collection } => {
            execute_reactivate_collection(deps, env, info, collection)
        }
        ExecuteMsg::BlacklistCollection { collection, reason } => {
            execute_blacklist_collection(deps, env, info, collection, reason)
        }
        ExecuteMsg::UnblacklistCollection { collection } => {
            execute_unblacklist_collection(deps, env, info, collection)
        }

        // Asks
        ExecuteMsg::SetAsk {
            collection,
            token_id,
            price,
            funds_recipient,
        } => execute_set_ask(
            deps,
            env,
            info,
            collection,
            token_id,
            price,
            funds_recipient,
        ),
        ExecuteMsg::UpdateAsk {
            collection,
            token_id,
            price,
        } => execute_update_ask(deps, env, info, collection, token_id, price),
        ExecuteMsg::RemoveAsk {
            collection,
            token_id,
        } => execute_remove_ask(deps, info, collection, token_id),
        ExecuteMsg::BuyNow {
            collection,
            token_id,
        } => execute_buy_now(deps, env, info, collection, token_id),

        // Bids
        ExecuteMsg::SetBid {
            collection,
            token_id,
            price,
            expires_at,
        } => execute_set_bid(deps, env, info, collection, token_id, price, expires_at),
        ExecuteMsg::RemoveBid {
            collection,
            token_id,
        } => execute_remove_bid(deps, info, collection, token_id),
        ExecuteMsg::AcceptBid {
            collection,
            token_id,
            bidder,
        } => execute_accept_bid(deps, env, info, collection, token_id, bidder),

        // Collection Bids
        ExecuteMsg::SetCollectionBid {
            collection,
            units,
            price,
            expires_at,
        } => execute_set_collection_bid(deps, env, info, collection, units, price, expires_at),
        ExecuteMsg::RemoveCollectionBid { collection } => {
            execute_remove_collection_bid(deps, info, collection)
        }
        ExecuteMsg::AcceptCollectionBid {
            collection,
            token_id,
            bidder,
        } => execute_accept_collection_bid(deps, env, info, collection, token_id, bidder),

        // Operators
        ExecuteMsg::SyncAsk {
            collection,
            token_id,
        } => execute_sync_ask(deps, info, collection, token_id),
        ExecuteMsg::BatchSyncAsks { asks: ask_list } => {
            execute_batch_sync_asks(deps, info, ask_list)
        }
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    denom: Option<String>,
    min_price: Option<Uint128>,
    trading_fee_bps: Option<u64>,
    max_trading_fee_bps: Option<u64>,
    fee_collector: Option<String>,
    registry: Option<String>,
    revenue_router: Option<String>,
    use_revenue_router: Option<bool>,
    operators: Option<Vec<String>>,
    paused: Option<bool>,
    require_registration: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }
    if let Some(new_denom) = denom {
        config.denom = new_denom;
    }
    if let Some(new_min) = min_price {
        config.min_price = new_min;
    }
    if let Some(new_fee) = trading_fee_bps {
        config.trading_fee_bps = new_fee;
    }
    if let Some(new_max_fee) = max_trading_fee_bps {
        config.max_trading_fee_bps = new_max_fee;
    }
    if let Some(new_collector) = fee_collector {
        config.fee_collector = deps.api.addr_validate(&new_collector)?;
    }
    if let Some(new_registry) = registry {
        config.registry = Some(deps.api.addr_validate(&new_registry)?);
    }
    if let Some(new_router) = revenue_router {
        config.revenue_router = Some(deps.api.addr_validate(&new_router)?);
    }
    if let Some(use_router) = use_revenue_router {
        config.use_revenue_router = use_router;
    }
    if let Some(new_operators) = operators {
        config.operators = new_operators
            .iter()
            .map(|o| deps.api.addr_validate(o))
            .collect::<StdResult<Vec<Addr>>>()?;
    }
    if let Some(is_paused) = paused {
        config.paused = is_paused;
    }
    if let Some(require_reg) = require_registration {
        config.require_registration = require_reg;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

// ========== Collection Registration Functions ==========

fn execute_register_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    trading_fee_bps: Option<u64>,
    denom: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin, operators, or registry can register
    let is_authorized = config.admin == info.sender
        || config.operators.contains(&info.sender)
        || config.registry.as_ref() == Some(&info.sender);

    if !is_authorized {
        return Err(ContractError::Unauthorized {});
    }

    // Check if already registered
    if COLLECTION_CONFIGS.has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered {
            collection: collection.clone(),
        });
    }

    // Validate trading fee if provided
    if let Some(fee) = trading_fee_bps {
        if fee > config.max_trading_fee_bps {
            return Err(ContractError::TradingFeeExceedsMax {
                fee_bps: fee,
                max_bps: config.max_trading_fee_bps,
            });
        }
    }

    let collection_config = CollectionConfig {
        collection: collection_addr.clone(),
        active: true,
        blacklisted: false,
        blacklist_reason: None,
        trading_fee_bps,
        denom,
        registered_by: info.sender.clone(),
        registered_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "register_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("registered_by", info.sender))
}

fn execute_update_collection_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    active: Option<bool>,
    trading_fee_bps: Option<u64>,
    denom: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin or operators
    if config.admin != info.sender && !config.operators.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut collection_config = COLLECTION_CONFIGS
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.clone(),
        })?;

    if let Some(is_active) = active {
        collection_config.active = is_active;
    }

    if let Some(fee) = trading_fee_bps {
        if fee > config.max_trading_fee_bps {
            return Err(ContractError::TradingFeeExceedsMax {
                fee_bps: fee,
                max_bps: config.max_trading_fee_bps,
            });
        }
        collection_config.trading_fee_bps = Some(fee);
    }

    if let Some(new_denom) = denom {
        collection_config.denom = Some(new_denom);
    }

    collection_config.updated_at = env.block.time.seconds();

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "update_collection_config")
        .add_attribute("collection", collection_addr))
}

fn execute_deactivate_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    _reason: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin or operators
    if config.admin != info.sender && !config.operators.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut collection_config = COLLECTION_CONFIGS
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.clone(),
        })?;

    collection_config.active = false;
    collection_config.updated_at = env.block.time.seconds();

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "deactivate_collection")
        .add_attribute("collection", collection_addr))
}

fn execute_reactivate_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin or operators
    if config.admin != info.sender && !config.operators.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut collection_config = COLLECTION_CONFIGS
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.clone(),
        })?;

    // Cannot reactivate if blacklisted
    if collection_config.blacklisted {
        return Err(ContractError::CollectionBlacklisted {
            collection: collection.clone(),
            reason: collection_config
                .blacklist_reason
                .unwrap_or_else(|| "Unknown".to_string()),
        });
    }

    collection_config.active = true;
    collection_config.updated_at = env.block.time.seconds();

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "reactivate_collection")
        .add_attribute("collection", collection_addr))
}

fn execute_blacklist_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    reason: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin only (moderation action)
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut collection_config = COLLECTION_CONFIGS
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.clone(),
        })?;

    collection_config.blacklisted = true;
    collection_config.blacklist_reason = Some(reason.clone());
    collection_config.active = false;
    collection_config.updated_at = env.block.time.seconds();

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "blacklist_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("reason", reason))
}

fn execute_unblacklist_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Authorization: admin only (moderation action)
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut collection_config = COLLECTION_CONFIGS
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.clone(),
        })?;

    collection_config.blacklisted = false;
    collection_config.blacklist_reason = None;
    collection_config.updated_at = env.block.time.seconds();
    // Note: does NOT automatically reactivate - admin must call ReactivateCollection

    COLLECTION_CONFIGS.save(deps.storage, collection_addr.clone(), &collection_config)?;

    Ok(Response::new()
        .add_attribute("action", "unblacklist_collection")
        .add_attribute("collection", collection_addr))
}

fn execute_set_ask(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
    price: Coin,
    funds_recipient: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Validate collection can be traded
    validate_collection(deps.storage, &config, &collection_addr)?;

    // Validate price
    let expected_denom = resolve_collection_denom(deps.storage, &config, &collection_addr)?;
    if price.denom != expected_denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: expected_denom,
            received: price.denom,
        });
    }
    if price.amount < config.min_price {
        return Err(ContractError::PriceBelowMinimum {
            min_price: config.min_price.to_string(),
        });
    }

    // Verify ownership
    verify_nft_ownership(&deps, &collection_addr, &token_id, &info.sender)?;

    let ask_key = (collection_addr.clone(), token_id.clone());

    // Check if ask already exists
    if asks().has(deps.storage, ask_key.clone()) {
        return Err(ContractError::AskAlreadyExists { token_id });
    }

    let funds_recipient_addr = funds_recipient
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let ask = Ask {
        collection: collection_addr.clone(),
        token_id: token_id.clone(),
        seller: info.sender.clone(),
        price,
        funds_recipient: funds_recipient_addr,
        created_at: env.block.time.seconds(),
        is_active: true,
    };

    asks().save(deps.storage, ask_key, &ask)?;

    Ok(Response::new()
        .add_attribute("action", "set_ask")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("seller", info.sender)
        .add_attribute("price", ask.price.to_string()))
}

fn execute_update_ask(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
    price: Coin,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    let ask_key = (collection_addr.clone(), token_id.clone());
    let mut ask =
        asks()
            .load(deps.storage, ask_key.clone())
            .map_err(|_| ContractError::AskNotFound {
                collection: collection.clone(),
                token_id: token_id.clone(),
            })?;

    if ask.seller != info.sender {
        return Err(ContractError::NotSeller {});
    }

    // Validate price
    let expected_denom = resolve_collection_denom(deps.storage, &config, &collection_addr)?;
    if price.denom != expected_denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: expected_denom,
            received: price.denom,
        });
    }
    if price.amount < config.min_price {
        return Err(ContractError::PriceBelowMinimum {
            min_price: config.min_price.to_string(),
        });
    }

    ask.price = price;
    ask.is_active = true;

    asks().save(deps.storage, ask_key, &ask)?;

    Ok(Response::new()
        .add_attribute("action", "update_ask")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("price", ask.price.to_string()))
}

fn execute_remove_ask(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    let ask_key = (collection_addr.clone(), token_id.clone());
    let ask =
        asks()
            .load(deps.storage, ask_key.clone())
            .map_err(|_| ContractError::AskNotFound {
                collection: collection.clone(),
                token_id: token_id.clone(),
            })?;

    // Only seller or admin can remove
    if ask.seller != info.sender && config.admin != info.sender {
        return Err(ContractError::NotSeller {});
    }

    asks().remove(deps.storage, ask_key)?;

    Ok(Response::new()
        .add_attribute("action", "remove_ask")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id))
}

fn execute_buy_now(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    let ask_key = (collection_addr.clone(), token_id.clone());
    let ask =
        asks()
            .load(deps.storage, ask_key.clone())
            .map_err(|_| ContractError::AskNotFound {
                collection: collection.clone(),
                token_id: token_id.clone(),
            })?;

    if !ask.is_active {
        return Err(ContractError::AskNotActive {});
    }

    if ask.seller == info.sender {
        return Err(ContractError::SellerCannotBuy {});
    }

    // Validate payment
    validate_payment(&info, &ask.price)?;

    // Execute sale
    let (messages, sale_info) = execute_sale(
        &deps,
        &env,
        &config,
        &collection_addr,
        &token_id,
        &ask.seller,
        &info.sender,
        &ask.get_recipient(),
        &ask.price.denom,
        ask.price.amount,
    )?;

    // Remove ask
    asks().remove(deps.storage, ask_key)?;

    // Update stats
    update_stats(
        deps.storage,
        &collection_addr,
        ask.price.amount,
        sale_info.platform_fee,
        sale_info.royalty,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "buy_now")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("buyer", info.sender)
        .add_attribute("seller", ask.seller)
        .add_attribute("price", ask.price.to_string()))
}

fn execute_set_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
    price: Coin,
    expires_at: Option<u64>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    validate_collection(deps.storage, &config, &collection_addr)?;

    // Validate price
    let expected_denom = resolve_collection_denom(deps.storage, &config, &collection_addr)?;
    if price.denom != expected_denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: expected_denom,
            received: price.denom,
        });
    }
    if price.amount < config.min_price {
        return Err(ContractError::PriceBelowMinimum {
            min_price: config.min_price.to_string(),
        });
    }

    // Validate payment
    validate_payment(&info, &price)?;

    let bid_key = (
        collection_addr.clone(),
        token_id.clone(),
        info.sender.clone(),
    );

    // Check if bid already exists
    if bids().has(deps.storage, bid_key.clone()) {
        return Err(ContractError::BidAlreadyExists {});
    }

    let bid = Bid {
        collection: collection_addr.clone(),
        token_id: token_id.clone(),
        bidder: info.sender.clone(),
        price,
        created_at: env.block.time.seconds(),
        expires_at,
    };

    bids().save(deps.storage, bid_key, &bid)?;

    Ok(Response::new()
        .add_attribute("action", "set_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("bidder", info.sender)
        .add_attribute("price", bid.price.to_string()))
}

fn execute_remove_bid(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bid_key = (
        collection_addr.clone(),
        token_id.clone(),
        info.sender.clone(),
    );

    let bid = bids()
        .load(deps.storage, bid_key.clone())
        .map_err(|_| ContractError::BidNotFound {})?;

    if bid.bidder != info.sender {
        return Err(ContractError::NotBidder {});
    }

    bids().remove(deps.storage, bid_key)?;

    // Refund bid amount
    let refund_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![bid.price],
    };

    Ok(Response::new()
        .add_message(refund_msg)
        .add_attribute("action", "remove_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("bidder", info.sender))
}

fn execute_accept_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
    bidder: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bidder_addr = deps.api.addr_validate(&bidder)?;

    // Verify ownership
    verify_nft_ownership(&deps, &collection_addr, &token_id, &info.sender)?;

    let bid_key = (
        collection_addr.clone(),
        token_id.clone(),
        bidder_addr.clone(),
    );
    let bid = bids()
        .load(deps.storage, bid_key.clone())
        .map_err(|_| ContractError::BidNotFound {})?;

    // Check expiration
    if let Some(expires) = bid.expires_at {
        if env.block.time.seconds() > expires {
            return Err(ContractError::BidExpired {});
        }
    }

    // Execute sale
    let (messages, sale_info) = execute_sale(
        &deps,
        &env,
        &config,
        &collection_addr,
        &token_id,
        &info.sender,
        &bidder_addr,
        &info.sender,
        &bid.price.denom,
        bid.price.amount,
    )?;

    // Remove bid
    bids().remove(deps.storage, bid_key)?;

    // Also remove any ask for this token
    let ask_key = (collection_addr.clone(), token_id.clone());
    if asks().has(deps.storage, ask_key.clone()) {
        asks().remove(deps.storage, ask_key)?;
    }

    // Update stats
    update_stats(
        deps.storage,
        &collection_addr,
        bid.price.amount,
        sale_info.platform_fee,
        sale_info.royalty,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "accept_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("seller", info.sender)
        .add_attribute("bidder", bidder_addr)
        .add_attribute("price", bid.price.to_string()))
}

fn execute_set_collection_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    units: u32,
    price: Coin,
    expires_at: Option<u64>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    validate_collection(deps.storage, &config, &collection_addr)?;

    // Validate price
    let expected_denom = resolve_collection_denom(deps.storage, &config, &collection_addr)?;
    if price.denom != expected_denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: expected_denom,
            received: price.denom,
        });
    }

    let total_cost = price.amount * Uint128::from(units);
    let expected_payment = Coin {
        denom: price.denom.clone(),
        amount: total_cost,
    };
    validate_payment(&info, &expected_payment)?;

    let bid_key = (collection_addr.clone(), info.sender.clone());

    if collection_bids().has(deps.storage, bid_key.clone()) {
        return Err(ContractError::CollectionBidAlreadyExists {});
    }

    let col_bid = CollectionBid {
        collection: collection_addr.clone(),
        bidder: info.sender.clone(),
        units,
        price,
        created_at: env.block.time.seconds(),
        expires_at,
    };

    collection_bids().save(deps.storage, bid_key, &col_bid)?;

    Ok(Response::new()
        .add_attribute("action", "set_collection_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("bidder", info.sender)
        .add_attribute("units", units.to_string())
        .add_attribute("price_per_unit", col_bid.price.to_string()))
}

fn execute_remove_collection_bid(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
) -> Result<Response, ContractError> {
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bid_key = (collection_addr.clone(), info.sender.clone());

    let col_bid = collection_bids()
        .load(deps.storage, bid_key.clone())
        .map_err(|_| ContractError::CollectionBidNotFound {})?;

    if col_bid.bidder != info.sender {
        return Err(ContractError::NotBidder {});
    }

    collection_bids().remove(deps.storage, bid_key)?;

    // Refund remaining funds
    let refund_amount = col_bid.total_cost();
    let refund_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: col_bid.price.denom,
            amount: refund_amount,
        }],
    };

    Ok(Response::new()
        .add_message(refund_msg)
        .add_attribute("action", "remove_collection_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("bidder", info.sender))
}

fn execute_accept_collection_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
    bidder: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;
    let bidder_addr = deps.api.addr_validate(&bidder)?;

    // Verify ownership
    verify_nft_ownership(&deps, &collection_addr, &token_id, &info.sender)?;

    let bid_key = (collection_addr.clone(), bidder_addr.clone());
    let mut col_bid = collection_bids()
        .load(deps.storage, bid_key.clone())
        .map_err(|_| ContractError::CollectionBidNotFound {})?;

    if col_bid.units == 0 {
        return Err(ContractError::NoCollectionBidUnits {});
    }

    // Check expiration
    if let Some(expires) = col_bid.expires_at {
        if env.block.time.seconds() > expires {
            return Err(ContractError::BidExpired {});
        }
    }

    // Execute sale
    let (messages, sale_info) = execute_sale(
        &deps,
        &env,
        &config,
        &collection_addr,
        &token_id,
        &info.sender,
        &bidder_addr,
        &info.sender,
        &col_bid.price.denom,
        col_bid.price.amount,
    )?;

    // Update or remove collection bid
    col_bid.units -= 1;
    if col_bid.units == 0 {
        collection_bids().remove(deps.storage, bid_key)?;
    } else {
        collection_bids().save(deps.storage, bid_key, &col_bid)?;
    }

    // Remove any ask for this token
    let ask_key = (collection_addr.clone(), token_id.clone());
    if asks().has(deps.storage, ask_key.clone()) {
        asks().remove(deps.storage, ask_key)?;
    }

    // Update stats
    update_stats(
        deps.storage,
        &collection_addr,
        col_bid.price.amount,
        sale_info.platform_fee,
        sale_info.royalty,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "accept_collection_bid")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("seller", info.sender)
        .add_attribute("bidder", bidder_addr)
        .add_attribute("price", col_bid.price.to_string()))
}

fn execute_sync_ask(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
    token_id: TokenId,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only operators or admin can sync
    if config.admin != info.sender && !config.operators.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let collection_addr = deps.api.addr_validate(&collection)?;
    let ask_key = (collection_addr.clone(), token_id.clone());

    if let Ok(mut ask) = asks().load(deps.storage, ask_key.clone()) {
        // Check if seller still owns the NFT
        let owner = query_nft_owner(&deps, &collection_addr, &token_id)?;
        if owner != ask.seller {
            ask.is_active = false;
            asks().save(deps.storage, ask_key, &ask)?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "sync_ask")
        .add_attribute("collection", collection_addr)
        .add_attribute("token_id", token_id))
}

fn execute_batch_sync_asks(
    deps: DepsMut,
    info: MessageInfo,
    ask_list: Vec<(String, TokenId)>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender && !config.operators.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    for (collection, token_id) in ask_list.iter() {
        let collection_addr = deps.api.addr_validate(collection)?;
        let ask_key = (collection_addr.clone(), token_id.clone());

        if let Ok(mut ask) = asks().load(deps.storage, ask_key.clone()) {
            if let Ok(owner) = query_nft_owner(&deps, &collection_addr, token_id) {
                if owner != ask.seller {
                    ask.is_active = false;
                    asks().save(deps.storage, ask_key, &ask)?;
                }
            }
        }
    }

    Ok(Response::new()
        .add_attribute("action", "batch_sync_asks")
        .add_attribute("count", ask_list.len().to_string()))
}
