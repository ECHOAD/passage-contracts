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
            registry,
            paused,
        } => execute_update_config(deps, info, admin, registry, paused),

        // Distribution rules
        ExecuteMsg::SetDistributionRule {
            collection,
            creator,
            creator_share,
            collaborators,
        } => execute_set_distribution_rule(
            deps,
            env,
            info,
            collection,
            creator,
            creator_share,
            collaborators,
        ),
        ExecuteMsg::UpdateDistributionRule {
            collection,
            creator,
            creator_share,
            collaborators,
            active,
        } => execute_update_distribution_rule(
            deps,
            env,
            info,
            collection,
            creator,
            creator_share,
            collaborators,
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
        ExecuteMsg::RouteSecondaryRoyalty { collection } => {
            execute_route_secondary_sale(deps, env, info, collection)
        }
        ExecuteMsg::RouteAuctionRoyalty { collection } => {
            execute_route_auction_sale(deps, env, info, collection)
        }
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

    if let Some(new_registry) = registry {
        config.registry = Some(deps.api.addr_validate(&new_registry)?);
    }

    if let Some(new_paused) = paused {
        config.paused = new_paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn ensure_can_manage_distribution_rule(
    deps: Deps,
    config: &Config,
    sender: &Addr,
    collection: &Addr,
) -> Result<(), ContractError> {
    if config.admin == *sender {
        return Ok(());
    }

    let registry = config
        .registry
        .clone()
        .ok_or(ContractError::Unauthorized {})?;
    let response: RegistryCollectionResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::Collection {
                address: collection.to_string(),
            },
        )
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        })?;

    let collection_info = response
        .collection
        .ok_or(ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        })?;

    let creator = deps.api.addr_validate(&collection_info.creator)?;
    if creator != *sender {
        return Err(ContractError::NotCollectionCreator {});
    }

    Ok(())
}

fn execute_set_distribution_rule(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    creator: String,
    creator_share: Decimal,
    collaborators: Option<Vec<CollaboratorInput>>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    ensure_can_manage_distribution_rule(deps.as_ref(), &config, &info.sender, &collection_addr)?;

    // Check if rule already exists
    if DISTRIBUTION_RULES.has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::DistributionRuleExists { collection });
    }

    // Validate shares
    if creator_share > Decimal::one() || creator_share.is_zero() {
        return Err(ContractError::InvalidShare {});
    }

    // Process collaborators
    let collab_list = process_collaborators(deps.as_ref(), collaborators)?;

    // Validate total collaborator shares don't exceed 100%
    let total_collab_share: Decimal = collab_list.iter().map(|c| c.share).sum();
    if total_collab_share > Decimal::one() {
        return Err(ContractError::CollaboratorSharesExceedLimit {});
    }

    let rule = DistributionRule {
        collection: collection_addr.clone(),
        creator: deps.api.addr_validate(&creator)?,
        creator_share,
        collaborators: collab_list,
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
    collaborators: Option<Vec<CollaboratorInput>>,
    active: Option<bool>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection)?;

    ensure_can_manage_distribution_rule(deps.as_ref(), &config, &info.sender, &collection_addr)?;

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

    if let Some(new_collabs) = collaborators {
        rule.collaborators = process_collaborators(deps.as_ref(), Some(new_collabs))?;
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
    let collection_addr = deps.api.addr_validate(&collection)?;
    ensure_can_manage_distribution_rule(deps.as_ref(), &config, &info.sender, &collection_addr)?;

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
    route_revenue_internal(deps, env, info, collection, RevenueEventType::PrimarySale)
}

fn execute_route_secondary_sale(
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
        RevenueEventType::SecondaryRoyalty,
    )
}

fn execute_route_auction_sale(
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
        RevenueEventType::AuctionRoyalty,
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
    route_revenue_internal(deps, env, info, collection, event_type)
}

fn route_revenue_internal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    event_type: RevenueEventType,
) -> Result<Response, ContractError> {
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

    let mut messages: Vec<CosmosMsg> = vec![];
    let creator_amount;
    let mut collaborator_amounts: Vec<(Addr, Uint128)> = vec![];

    creator_amount = distribute_to_creator_and_collaborators(
        &rule,
        total_amount,
        &denom,
        &mut messages,
        &mut collaborator_amounts,
    );

    // Record event
    let event_id = record_revenue_event(
        deps.storage,
        &env,
        &collection_addr,
        &event_type,
        total_amount,
        &denom,
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
        creator_amount,
    )?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "route_revenue")
        .add_attribute("collection", collection_addr)
        .add_attribute("event_type", format!("{:?}", event_type))
        .add_attribute("total_amount", total_amount)
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
    let creator_base = amount.multiply_ratio(
        rule.creator_share.atomics().u128(),
        10u128.pow(Decimal::DECIMAL_PLACES),
    );

    // Pay collaborators from the creator-governed portion, then the creator receives the remainder.
    for collab in &rule.collaborators {
        let collab_amount = creator_base.multiply_ratio(
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        from_json,
        testing::{message_info, mock_dependencies, mock_env},
        ContractResult, Decimal, OwnedDeps, SystemError, SystemResult, WasmQuery,
    };

    fn mock_registry_collection(
        deps: &mut OwnedDeps<
            cosmwasm_std::testing::MockStorage,
            cosmwasm_std::testing::MockApi,
            cosmwasm_std::testing::MockQuerier,
        >,
        registry: &str,
        creator: &str,
    ) {
        let registry = registry.to_string();
        let creator = creator.to_string();

        deps.querier.update_wasm(move |query| match query {
            WasmQuery::Smart { contract_addr, msg } if contract_addr == &registry => {
                let parsed: RegistryQueryMsg = from_json(msg).unwrap();
                match parsed {
                    RegistryQueryMsg::Collection { .. } => SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&RegistryCollectionResponse {
                            collection: Some(crate::msg::RegistryCollection {
                                creator: creator.to_string(),
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
    fn collection_creator_can_set_rule_via_registry() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let admin = deps.api.addr_make("admin");
        let registry = deps.api.addr_make("registry");
        let creator = deps.api.addr_make("creator");
        let collection = deps.api.addr_make("collection");
        let creator_wallet = deps.api.addr_make("creator-wallet");

        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    admin: admin.clone(),
                    registry: Some(registry.clone()),
                    paused: false,
                },
            )
            .unwrap();
        REVENUE_EVENT_COUNT.save(deps.as_mut().storage, &0).unwrap();
        mock_registry_collection(&mut deps, registry.as_str(), creator.as_str());

        let res = execute_set_distribution_rule(
            deps.as_mut(),
            env,
            message_info(&creator, &[]),
            collection.to_string(),
            creator_wallet.to_string(),
            Decimal::percent(90),
            None,
        )
        .unwrap();

        assert_eq!(
            res.attributes[0],
            cosmwasm_std::attr("action", "set_distribution_rule")
        );
        assert!(DISTRIBUTION_RULES.has(deps.as_ref().storage, collection));
    }

    #[test]
    fn collaborators_are_calculated_from_creator_share_base() {
        let rule = DistributionRule {
            collection: Addr::unchecked("collection"),
            creator: Addr::unchecked("creator"),
            creator_share: Decimal::percent(50),
            collaborators: vec![Collaborator {
                address: Addr::unchecked("collab"),
                share: Decimal::percent(50),
                name: None,
            }],
            active: true,
            created_at: 0,
            updated_at: 0,
        };

        let mut messages = Vec::new();
        let mut collaborator_amounts = Vec::new();
        let creator_amount = distribute_to_creator_and_collaborators(
            &rule,
            Uint128::new(1_000),
            "upasg",
            &mut messages,
            &mut collaborator_amounts,
        );

        assert_eq!(creator_amount, Uint128::new(750));
        assert_eq!(collaborator_amounts[0].1, Uint128::new(250));
        assert_eq!(messages.len(), 2);
    }
}
