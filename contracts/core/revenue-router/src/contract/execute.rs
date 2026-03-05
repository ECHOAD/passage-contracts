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

    // Check if contract is paused (allow admin actions)
    if config.paused && config.admin != info.sender {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        // Admin operations
        ExecuteMsg::UpdateConfig {
            admin,
            platform_fee_collector,
            default_platform_fee,
            registry,
            paused,
        } => execute_update_config(
            deps,
            info,
            admin,
            platform_fee_collector,
            default_platform_fee,
            registry,
            paused,
        ),

        // Distribution rules
        ExecuteMsg::SetDistributionRule {
            collection,
            creator,
            creator_share,
            platform_fee,
            collaborators,
            royalty_pool,
        } => execute_set_distribution_rule(
            deps,
            env,
            info,
            collection,
            creator,
            creator_share,
            platform_fee,
            collaborators,
            royalty_pool,
        ),
        ExecuteMsg::UpdateDistributionRule {
            collection,
            creator,
            creator_share,
            platform_fee,
            collaborators,
            royalty_pool,
            active,
        } => execute_update_distribution_rule(
            deps,
            env,
            info,
            collection,
            creator,
            creator_share,
            platform_fee,
            collaborators,
            royalty_pool,
            active,
        ),
        ExecuteMsg::RemoveDistributionRule { collection } => {
            execute_remove_distribution_rule(deps, info, collection)
        }

        // Ecosystem config
        ExecuteMsg::SetEcosystemConfig {
            ecosystem_id,
            treasury,
        } => execute_set_ecosystem_config(deps, info, ecosystem_id, treasury),

        // Revenue routing
        ExecuteMsg::RoutePrimarySale { collection } => {
            execute_route_primary_sale(deps, env, info, collection)
        }
        ExecuteMsg::RouteSecondarySale {
            collection,
            seller,
            royalty_amount,
        } => execute_route_secondary_sale(deps, env, info, collection, seller, royalty_amount),
        ExecuteMsg::RouteAuctionSale {
            collection,
            seller,
            royalty_amount,
        } => execute_route_auction_sale(deps, env, info, collection, seller, royalty_amount),
        ExecuteMsg::RouteRevenue {
            collection,
            event_type,
            custom_recipients,
        } => execute_route_revenue(deps, env, info, collection, event_type, custom_recipients),

        // Split wallets
        ExecuteMsg::CreateSplitWallet { id, recipients } => {
            execute_create_split_wallet(deps, env, info, id, recipients)
        }
        ExecuteMsg::UpdateSplitWallet { id, recipients } => {
            execute_update_split_wallet(deps, env, info, id, recipients)
        }
        ExecuteMsg::DistributeSplitWallet { id } => execute_distribute_split_wallet(deps, info, id),
        ExecuteMsg::RemoveSplitWallet { id } => execute_remove_split_wallet(deps, info, id),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    platform_fee_collector: Option<String>,
    default_platform_fee: Option<Decimal>,
    registry: Option<String>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }

    if let Some(new_collector) = platform_fee_collector {
        config.platform_fee_collector = deps.api.addr_validate(&new_collector)?;
    }

    if let Some(new_fee) = default_platform_fee {
        let max_fee: Decimal = MAX_PLATFORM_FEE.parse().unwrap();
        if new_fee > max_fee {
            return Err(ContractError::PlatformFeeExceedsMax {
                max: MAX_PLATFORM_FEE.to_string(),
            });
        }
        config.default_platform_fee = new_fee;
    }

    if let Some(new_registry) = registry {
        config.registry = Some(deps.api.addr_validate(&new_registry)?);
    }

    if let Some(new_paused) = paused {
        config.paused = new_paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn execute_set_distribution_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    creator: String,
    creator_share: Decimal,
    platform_fee: Option<Decimal>,
    collaborators: Option<Vec<CollaboratorInput>>,
    royalty_pool: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Check authorization (admin or collection creator via registry)
    if config.admin != info.sender {
        // TODO: Query registry to verify sender is collection creator
        return Err(ContractError::Unauthorized {});
    }

    // Check if rule already exists
    if DISTRIBUTION_RULES.has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::DistributionRuleExists { collection });
    }

    // Validate shares
    if creator_share > Decimal::one() || creator_share.is_zero() {
        return Err(ContractError::InvalidShare {});
    }

    // Validate platform fee
    if let Some(fee) = platform_fee {
        let max_fee: Decimal = MAX_PLATFORM_FEE.parse().unwrap();
        if fee > max_fee {
            return Err(ContractError::PlatformFeeExceedsMax {
                max: MAX_PLATFORM_FEE.to_string(),
            });
        }
    }

    // Process collaborators
    let collab_list = process_collaborators(deps.as_ref(), collaborators)?;

    // Validate total collaborator shares don't exceed 100%
    let total_collab_share: Decimal = collab_list.iter().map(|c| c.share).sum();
    if total_collab_share > Decimal::one() {
        return Err(ContractError::CollaboratorSharesExceedLimit {});
    }

    let royalty_pool_addr = royalty_pool
        .map(|r| deps.api.addr_validate(&r))
        .transpose()?;

    let rule = DistributionRule {
        collection: collection_addr.clone(),
        platform_fee,
        creator: deps.api.addr_validate(&creator)?,
        creator_share,
        collaborators: collab_list,
        royalty_pool: royalty_pool_addr,
        active: true,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    DISTRIBUTION_RULES.save(deps.storage, collection_addr.clone(), &rule)?;

    Ok(Response::new()
        .add_attribute("action", "set_distribution_rule")
        .add_attribute("collection", collection_addr))
}

fn execute_update_distribution_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    creator: Option<String>,
    creator_share: Option<Decimal>,
    platform_fee: Option<Decimal>,
    collaborators: Option<Vec<CollaboratorInput>>,
    royalty_pool: Option<String>,
    active: Option<bool>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Check authorization
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let mut rule = DISTRIBUTION_RULES
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::DistributionRuleNotFound {
            collection: collection.clone(),
        })?;

    if let Some(new_creator) = creator {
        rule.creator = deps.api.addr_validate(&new_creator)?;
    }

    if let Some(new_share) = creator_share {
        if new_share > Decimal::one() || new_share.is_zero() {
            return Err(ContractError::InvalidShare {});
        }
        rule.creator_share = new_share;
    }

    if let Some(new_fee) = platform_fee {
        let max_fee: Decimal = MAX_PLATFORM_FEE.parse().unwrap();
        if new_fee > max_fee {
            return Err(ContractError::PlatformFeeExceedsMax {
                max: MAX_PLATFORM_FEE.to_string(),
            });
        }
        rule.platform_fee = Some(new_fee);
    }

    if let Some(new_collabs) = collaborators {
        rule.collaborators = process_collaborators(deps.as_ref(), Some(new_collabs))?;
    }

    if let Some(new_pool) = royalty_pool {
        rule.royalty_pool = Some(deps.api.addr_validate(&new_pool)?);
    }

    if let Some(new_active) = active {
        rule.active = new_active;
    }

    rule.updated_at = env.block.time.seconds();

    DISTRIBUTION_RULES.save(deps.storage, collection_addr.clone(), &rule)?;

    Ok(Response::new()
        .add_attribute("action", "update_distribution_rule")
        .add_attribute("collection", collection_addr))
}

fn execute_remove_distribution_rule(
    deps: DepsMut,
    info: MessageInfo,
    collection: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let collection_addr = deps.api.addr_validate(&collection)?;

    if !DISTRIBUTION_RULES.has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::DistributionRuleNotFound { collection });
    }

    DISTRIBUTION_RULES.remove(deps.storage, collection_addr.clone());

    Ok(Response::new()
        .add_attribute("action", "remove_distribution_rule")
        .add_attribute("collection", collection_addr))
}

fn execute_set_ecosystem_config(
    deps: DepsMut,
    info: MessageInfo,
    ecosystem_id: String,
    treasury: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let treasury_addr = treasury.map(|t| deps.api.addr_validate(&t)).transpose()?;
    let ecosystem_config = EcosystemConfig {
        ecosystem_id: ecosystem_id.clone(),
        treasury: treasury_addr,
    };

    ECOSYSTEM_CONFIGS.save(deps.storage, ecosystem_id.clone(), &ecosystem_config)?;

    Ok(Response::new()
        .add_attribute("action", "set_ecosystem_config")
        .add_attribute("ecosystem_id", ecosystem_id))
}

fn execute_route_primary_sale(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
) -> Result<Response, ContractError> {
    route_revenue_internal(
        deps,
        env,
        info,
        collection,
        RevenueEventType::PrimarySale,
        None,
        None,
    )
}

fn execute_route_secondary_sale(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    seller: String,
    royalty_amount: Uint128,
) -> Result<Response, ContractError> {
    let seller_addr = deps.api.addr_validate(&seller)?;
    route_revenue_internal(
        deps,
        env,
        info,
        collection,
        RevenueEventType::SecondarySale,
        Some(seller_addr),
        Some(royalty_amount),
    )
}

fn execute_route_auction_sale(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    seller: String,
    royalty_amount: Uint128,
) -> Result<Response, ContractError> {
    let seller_addr = deps.api.addr_validate(&seller)?;
    route_revenue_internal(
        deps,
        env,
        info,
        collection,
        RevenueEventType::Auction,
        Some(seller_addr),
        Some(royalty_amount),
    )
}

fn execute_route_revenue(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    event_type: RevenueEventType,
    _custom_recipients: Option<Vec<RecipientInput>>,
) -> Result<Response, ContractError> {
    route_revenue_internal(deps, env, info, collection, event_type, None, None)
}

fn route_revenue_internal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    event_type: RevenueEventType,
    seller: Option<Addr>,
    royalty_amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    // Get funds sent with message
    if info.funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let fund = &info.funds[0];
    let total_amount = fund.amount;
    let denom = fund.denom.clone();

    // Get distribution rule
    let rule = DISTRIBUTION_RULES
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::DistributionRuleNotFound {
            collection: collection.clone(),
        })?;

    if !rule.active {
        return Err(ContractError::DistributionRuleNotFound { collection });
    }

    // Calculate distributions
    let platform_fee_rate = rule.platform_fee.unwrap_or(config.default_platform_fee);
    let platform_fee = total_amount.multiply_ratio(
        platform_fee_rate.atomics().u128(),
        10u128.pow(Decimal::DECIMAL_PLACES),
    );

    let mut messages: Vec<CosmosMsg> = vec![];
    let remaining = total_amount - platform_fee;

    // Platform fee
    if !platform_fee.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: config.platform_fee_collector.to_string(),
            amount: vec![Coin {
                denom: denom.clone(),
                amount: platform_fee,
            }],
        }));
    }

    // For secondary sales, handle seller payment and royalties
    let creator_amount;
    let mut collaborator_amounts: Vec<(Addr, Uint128)> = vec![];

    match event_type {
        RevenueEventType::SecondarySale | RevenueEventType::Auction => {
            // Royalty goes to creator/collaborators
            let royalty = royalty_amount.unwrap_or(Uint128::zero());
            let seller_amount = remaining - royalty;

            // Pay seller
            if let Some(seller_addr) = seller {
                if !seller_amount.is_zero() {
                    messages.push(CosmosMsg::Bank(BankMsg::Send {
                        to_address: seller_addr.to_string(),
                        amount: vec![Coin {
                            denom: denom.clone(),
                            amount: seller_amount,
                        }],
                    }));
                }
            }

            // Distribute royalty
            creator_amount = distribute_to_creator_and_collaborators(
                &rule,
                royalty,
                &denom,
                &mut messages,
                &mut collaborator_amounts,
            );
        }
        _ => {
            // Primary sale - all goes to creator/collaborators
            creator_amount = distribute_to_creator_and_collaborators(
                &rule,
                remaining,
                &denom,
                &mut messages,
                &mut collaborator_amounts,
            );
        }
    }

    // Record event
    let event_id = record_revenue_event(
        deps.storage,
        &env,
        &collection_addr,
        &event_type,
        total_amount,
        &denom,
        platform_fee,
        creator_amount,
        collaborator_amounts.clone(),
        &info.sender,
    )?;

    // Update collection stats
    update_collection_stats(
        deps.storage,
        &collection_addr,
        &event_type,
        total_amount,
        platform_fee,
        creator_amount,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "route_revenue")
        .add_attribute("collection", collection_addr)
        .add_attribute("event_type", format!("{:?}", event_type))
        .add_attribute("total_amount", total_amount)
        .add_attribute("platform_fee", platform_fee)
        .add_attribute("creator_amount", creator_amount)
        .add_attribute("event_id", event_id.to_string()))
}

fn distribute_to_creator_and_collaborators(
    rule: &DistributionRule,
    amount: Uint128,
    denom: &str,
    messages: &mut Vec<CosmosMsg>,
    collaborator_amounts: &mut Vec<(Addr, Uint128)>,
) -> Uint128 {
    let mut remaining = amount;

    // Pay collaborators first (from creator's share)
    for collab in &rule.collaborators {
        let collab_amount = amount.multiply_ratio(
            collab.share.atomics().u128(),
            10u128.pow(Decimal::DECIMAL_PLACES),
        );
        if !collab_amount.is_zero() {
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: collab.address.to_string(),
                amount: vec![Coin {
                    denom: denom.to_string(),
                    amount: collab_amount,
                }],
            }));
            collaborator_amounts.push((collab.address.clone(), collab_amount));
            remaining -= collab_amount;
        }
    }

    // Pay creator the rest
    if !remaining.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: rule.creator.to_string(),
            amount: vec![Coin {
                denom: denom.to_string(),
                amount: remaining,
            }],
        }));
    }

    remaining
}

fn execute_create_split_wallet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    recipients: Vec<SplitRecipientInput>,
) -> Result<Response, ContractError> {
    if SPLIT_WALLETS.has(deps.storage, id.clone()) {
        return Err(ContractError::SplitWalletExists { id });
    }

    if recipients.is_empty() {
        return Err(ContractError::NoSplitRecipients {});
    }

    let recipient_list: Vec<SplitRecipient> = recipients
        .into_iter()
        .map(|r| {
            Ok(SplitRecipient {
                address: deps.api.addr_validate(&r.address)?,
                weight: r.weight,
                name: r.name,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    let total_weight: u64 = recipient_list.iter().map(|r| r.weight).sum();
    if total_weight == 0 {
        return Err(ContractError::ZeroTotalWeight {});
    }

    let wallet = SplitWallet {
        id: id.clone(),
        admin: info.sender.clone(),
        recipients: recipient_list,
        total_weight,
        created_at: env.block.time.seconds(),
    };

    SPLIT_WALLETS.save(deps.storage, id.clone(), &wallet)?;

    Ok(Response::new()
        .add_attribute("action", "create_split_wallet")
        .add_attribute("id", id)
        .add_attribute("admin", info.sender))
}

fn execute_update_split_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    recipients: Vec<SplitRecipientInput>,
) -> Result<Response, ContractError> {
    let mut wallet = SPLIT_WALLETS
        .load(deps.storage, id.clone())
        .map_err(|_| ContractError::SplitWalletNotFound { id: id.clone() })?;

    if wallet.admin != info.sender {
        return Err(ContractError::NotSplitWalletAdmin {});
    }

    if recipients.is_empty() {
        return Err(ContractError::NoSplitRecipients {});
    }

    let recipient_list: Vec<SplitRecipient> = recipients
        .into_iter()
        .map(|r| {
            Ok(SplitRecipient {
                address: deps.api.addr_validate(&r.address)?,
                weight: r.weight,
                name: r.name,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    let total_weight: u64 = recipient_list.iter().map(|r| r.weight).sum();
    if total_weight == 0 {
        return Err(ContractError::ZeroTotalWeight {});
    }

    wallet.recipients = recipient_list;
    wallet.total_weight = total_weight;

    SPLIT_WALLETS.save(deps.storage, id.clone(), &wallet)?;

    Ok(Response::new()
        .add_attribute("action", "update_split_wallet")
        .add_attribute("id", id))
}

fn execute_distribute_split_wallet(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let wallet = SPLIT_WALLETS
        .load(deps.storage, id.clone())
        .map_err(|_| ContractError::SplitWalletNotFound { id: id.clone() })?;

    if info.funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let fund = &info.funds[0];
    let total_amount = fund.amount;
    let denom = fund.denom.clone();

    let mut messages: Vec<CosmosMsg> = vec![];

    for recipient in &wallet.recipients {
        let share = Decimal::from_ratio(recipient.weight, wallet.total_weight);
        let amount = total_amount
            .multiply_ratio(share.atomics().u128(), 10u128.pow(Decimal::DECIMAL_PLACES));

        if !amount.is_zero() {
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.address.to_string(),
                amount: vec![Coin {
                    denom: denom.clone(),
                    amount,
                }],
            }));
        }
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "distribute_split_wallet")
        .add_attribute("id", id)
        .add_attribute("total_amount", total_amount))
}

fn execute_remove_split_wallet(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let wallet = SPLIT_WALLETS
        .load(deps.storage, id.clone())
        .map_err(|_| ContractError::SplitWalletNotFound { id: id.clone() })?;

    if wallet.admin != info.sender && config.admin != info.sender {
        return Err(ContractError::NotSplitWalletAdmin {});
    }

    SPLIT_WALLETS.remove(deps.storage, id.clone());

    Ok(Response::new()
        .add_attribute("action", "remove_split_wallet")
        .add_attribute("id", id))
}
