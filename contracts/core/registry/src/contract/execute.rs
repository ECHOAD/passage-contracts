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
    if config.paused && !is_admin(&config, &info.sender) {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        // Admin operations
        ExecuteMsg::UpdateConfig {
            admin,
            operators,
            recovery_council,
            ecosystem_factory,
            paused,
        } => execute_update_config(
            deps,
            env,
            info,
            admin,
            operators,
            recovery_council,
            ecosystem_factory,
            paused,
        ),
        ExecuteMsg::UpdateCreatorModeration {
            creator,
            ecosystem_creation_enabled,
            collection_creation_enabled,
            mint_enabled,
            trade_enabled,
            reason,
        } => execute_update_creator_moderation(
            deps,
            env,
            info,
            creator,
            ecosystem_creation_enabled,
            collection_creation_enabled,
            mint_enabled,
            trade_enabled,
            reason,
        ),
        ExecuteMsg::UpdateEcosystemModeration {
            ecosystem_id,
            collection_creation_enabled,
            mint_enabled,
            trade_enabled,
            reason,
        } => execute_update_ecosystem_moderation(
            deps,
            env,
            info,
            ecosystem_id,
            collection_creation_enabled,
            mint_enabled,
            trade_enabled,
            reason,
        ),
        ExecuteMsg::UpdateCollectionModeration {
            address,
            mint_enabled,
            trade_enabled,
            reason,
        } => execute_update_collection_moderation(
            deps,
            env,
            info,
            address,
            mint_enabled,
            trade_enabled,
            reason,
        ),
        ExecuteMsg::SetEcosystemRecoveryPolicy {
            ecosystem_id,
            delegate,
            designated_successor,
        } => execute_set_ecosystem_recovery_policy(
            deps,
            env,
            info,
            ecosystem_id,
            delegate,
            designated_successor,
        ),
        ExecuteMsg::SetCollectionRecoveryPolicy {
            address,
            delegate,
            designated_successor,
        } => execute_set_collection_recovery_policy(
            deps,
            env,
            info,
            address,
            delegate,
            designated_successor,
        ),

        // Ecosystem operations
        ExecuteMsg::RegisterEcosystemFromFactory {
            id,
            name,
            creator,
            collection_factory,
            description,
            image_urls,
            animation_url,
            url,
        } => execute_register_ecosystem_from_factory(
            deps,
            env,
            info,
            id,
            name,
            creator,
            collection_factory,
            description,
            image_urls,
            animation_url,
            url,
        ),
        ExecuteMsg::UpdateEcosystem {
            id,
            name,
            description,
            image_urls,
            animation_url,
            url,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            admin,
        } => execute_update_ecosystem(
            deps,
            env,
            info,
            id,
            name,
            description,
            image_urls,
            animation_url,
            url,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            admin,
        ),
        ExecuteMsg::ApproveEcosystemMember {
            ecosystem_id,
            member,
        } => execute_approve_ecosystem_member(deps, info, ecosystem_id, member),
        ExecuteMsg::RevokeEcosystemMember {
            ecosystem_id,
            member,
        } => execute_revoke_ecosystem_member(deps, info, ecosystem_id, member),

        // Collection operations
        ExecuteMsg::RegisterCollection {
            address,
            ecosystem_id,
            name,
            nft_type,
        } => execute_register_collection(deps, env, info, address, ecosystem_id, name, nft_type),
        ExecuteMsg::RegisterCollectionFromFactory {
            address,
            ecosystem_id,
            name,
            creator,
            nft_type,
        } => execute_register_collection_from_factory(
            deps,
            env,
            info,
            address,
            ecosystem_id,
            name,
            creator,
            nft_type,
        ),
        ExecuteMsg::RegisterExistingCollection {
            address,
            ecosystem_id,
            name,
            creator,
            nft_type,
        } => execute_register_existing_collection(
            deps,
            env,
            info,
            address,
            ecosystem_id,
            name,
            creator,
            nft_type,
        ),
        ExecuteMsg::DeregisterCollection { address } => {
            execute_deregister_collection(deps, env, info, address)
        }
        ExecuteMsg::RehomeCollection {
            address,
            ecosystem_id,
        } => execute_rehome_collection(deps, env, info, address, ecosystem_id),
        ExecuteMsg::UpdateCollection {
            address,
            name,
            verified,
            minter,
            marketplace,
        } => execute_update_collection(
            deps,
            env,
            info,
            address,
            name,
            verified,
            minter,
            marketplace,
        ),
        ExecuteMsg::TransferCollectionOwnership {
            address,
            new_creator,
        } => execute_transfer_collection_ownership(deps, env, info, address, new_creator),

        // Minter authorization
        ExecuteMsg::AuthorizeMinter {
            collection_address,
            minter_address,
        } => execute_authorize_minter(deps, env, info, collection_address, minter_address),
        ExecuteMsg::RevokeMinter {
            collection_address,
            minter_address,
        } => execute_revoke_minter(deps, info, collection_address, minter_address),

        // Ownership recovery governance
        ExecuteMsg::UpdateRecoveryConfig {
            abandonment_inactivity_period_secs,
            contest_period_secs,
        } => execute_update_recovery_config(
            deps,
            info,
            abandonment_inactivity_period_secs,
            contest_period_secs,
        ),
        ExecuteMsg::OpenRecoveryCase {
            case_kind,
            target,
            reason,
            evidence_url,
            proposed_replacement,
        } => execute_open_recovery_case(
            deps,
            env,
            info,
            case_kind,
            target,
            reason,
            evidence_url,
            proposed_replacement,
        ),
        ExecuteMsg::ContestRecoveryCase { case_id, note } => {
            execute_contest_recovery_case(deps, env, info, case_id, note)
        }
        ExecuteMsg::ResolveRecoveryCase {
            case_id,
            approved,
            note,
        } => execute_resolve_recovery_case(deps, env, info, case_id, approved, note),
    }
}

fn execute_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    operators: Option<Vec<String>>,
    recovery_council: Option<Vec<String>>,
    ecosystem_factory: Option<String>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(new_admin) = admin {
        config.admin = deps.api.addr_validate(&new_admin)?;
    }

    if let Some(new_operators) = operators {
        config.operators = new_operators
            .iter()
            .map(|o| deps.api.addr_validate(o))
            .collect::<StdResult<Vec<Addr>>>()?;
        for operator in &config.operators {
            touch_creator_activity(deps.storage, operator, env.block.time.seconds())?;
        }
    }

    if let Some(new_recovery_council) = recovery_council {
        config.recovery_council = new_recovery_council
            .iter()
            .map(|o| deps.api.addr_validate(o))
            .collect::<StdResult<Vec<Addr>>>()?;
        for member in &config.recovery_council {
            touch_creator_activity(deps.storage, member, env.block.time.seconds())?;
        }
    }

    if let Some(new_factory) = ecosystem_factory {
        if new_factory.trim().is_empty() {
            return Err(ContractError::EcosystemFactoryRequired {});
        }
        config.ecosystem_factory = Some(deps.api.addr_validate(&new_factory)?);
    }

    if let Some(new_paused) = paused {
        config.paused = new_paused;
    }

    touch_creator_activity(deps.storage, &config.admin, env.block.time.seconds())?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[allow(clippy::too_many_arguments)]
fn execute_update_creator_moderation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    creator: String,
    ecosystem_creation_enabled: Option<bool>,
    collection_creation_enabled: Option<bool>,
    mint_enabled: Option<bool>,
    trade_enabled: Option<bool>,
    reason: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    let mut moderation = creator_moderation(deps.storage, &creator_addr)?;
    if let Some(value) = ecosystem_creation_enabled {
        moderation.ecosystem_creation_enabled = value;
    }
    if let Some(value) = collection_creation_enabled {
        moderation.collection_creation_enabled = value;
    }
    if let Some(value) = mint_enabled {
        moderation.mint_enabled = value;
    }
    if let Some(value) = trade_enabled {
        moderation.trade_enabled = value;
    }
    moderation.reason = normalize_reason(reason);
    moderation.moderated_by = Some(info.sender.clone());
    moderation.moderated_at = Some(env.block.time.seconds());

    CREATOR_MODERATION.save(deps.storage, creator_addr.clone(), &moderation)?;

    Ok(Response::new()
        .add_attribute("action", "update_creator_moderation")
        .add_attribute("creator", creator_addr)
        .add_attribute(
            "ecosystem_creation_enabled",
            moderation.ecosystem_creation_enabled.to_string(),
        )
        .add_attribute(
            "collection_creation_enabled",
            moderation.collection_creation_enabled.to_string(),
        )
        .add_attribute("mint_enabled", moderation.mint_enabled.to_string())
        .add_attribute("trade_enabled", moderation.trade_enabled.to_string()))
}

fn execute_update_ecosystem_moderation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ecosystem_id: String,
    collection_creation_enabled: Option<bool>,
    mint_enabled: Option<bool>,
    trade_enabled: Option<bool>,
    reason: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    if !ECOSYSTEMS.has(deps.storage, ecosystem_id.clone()) {
        return Err(ContractError::EcosystemNotFound { id: ecosystem_id });
    }

    let mut moderation = ecosystem_moderation(deps.storage, &ecosystem_id)?;
    if let Some(value) = collection_creation_enabled {
        moderation.collection_creation_enabled = value;
    }
    if let Some(value) = mint_enabled {
        moderation.mint_enabled = value;
    }
    if let Some(value) = trade_enabled {
        moderation.trade_enabled = value;
    }
    moderation.reason = normalize_reason(reason);
    moderation.moderated_by = Some(info.sender.clone());
    moderation.moderated_at = Some(env.block.time.seconds());

    ECOSYSTEM_MODERATION.save(deps.storage, ecosystem_id.clone(), &moderation)?;

    Ok(Response::new()
        .add_attribute("action", "update_ecosystem_moderation")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute(
            "collection_creation_enabled",
            moderation.collection_creation_enabled.to_string(),
        )
        .add_attribute("mint_enabled", moderation.mint_enabled.to_string())
        .add_attribute("trade_enabled", moderation.trade_enabled.to_string()))
}

fn execute_update_collection_moderation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    mint_enabled: Option<bool>,
    trade_enabled: Option<bool>,
    reason: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let collection_addr = deps.api.addr_validate(&address)?;
    if !collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionNotFound { address });
    }

    let mut moderation = collection_moderation(deps.storage, &collection_addr)?;
    if let Some(value) = mint_enabled {
        moderation.mint_enabled = value;
    }
    if let Some(value) = trade_enabled {
        moderation.trade_enabled = value;
    }
    moderation.reason = normalize_reason(reason);
    moderation.moderated_by = Some(info.sender.clone());
    moderation.moderated_at = Some(env.block.time.seconds());

    COLLECTION_MODERATION.save(deps.storage, collection_addr.clone(), &moderation)?;

    Ok(Response::new()
        .add_attribute("action", "update_collection_moderation")
        .add_attribute("collection", collection_addr)
        .add_attribute("mint_enabled", moderation.mint_enabled.to_string())
        .add_attribute("trade_enabled", moderation.trade_enabled.to_string()))
}

fn execute_set_ecosystem_recovery_policy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ecosystem_id: String,
    delegate: Option<String>,
    designated_successor: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    let policy = RecoveryPolicy {
        delegate: delegate
            .as_ref()
            .map(|addr| deps.api.addr_validate(addr))
            .transpose()?,
        designated_successor: designated_successor
            .as_ref()
            .map(|addr| deps.api.addr_validate(addr))
            .transpose()?,
        updated_by: Some(info.sender.clone()),
        updated_at: Some(env.block.time.seconds()),
    };

    ECOSYSTEM_RECOVERY_POLICIES.save(deps.storage, ecosystem_id.clone(), &policy)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    let delegate_attr = policy
        .delegate
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none".to_string());
    let successor_attr = policy
        .designated_successor
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none".to_string());

    Ok(Response::new()
        .add_attribute("action", "set_ecosystem_recovery_policy")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("delegate", delegate_attr)
        .add_attribute("designated_successor", successor_attr))
}

fn execute_set_collection_recovery_policy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    delegate: Option<String>,
    designated_successor: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;
    let collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: address.clone(),
        })?;

    if !can_manage_collection(deps.storage, &config, &collection, &info.sender)? {
        if is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotCollectionCreator {});
    }

    let policy = RecoveryPolicy {
        delegate: delegate
            .as_ref()
            .map(|addr| deps.api.addr_validate(addr))
            .transpose()?,
        designated_successor: designated_successor
            .as_ref()
            .map(|addr| deps.api.addr_validate(addr))
            .transpose()?,
        updated_by: Some(info.sender.clone()),
        updated_at: Some(env.block.time.seconds()),
    };

    COLLECTION_RECOVERY_POLICIES.save(deps.storage, collection_addr.clone(), &policy)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    let delegate_attr = policy
        .delegate
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none".to_string());
    let successor_attr = policy
        .designated_successor
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none".to_string());

    Ok(Response::new()
        .add_attribute("action", "set_collection_recovery_policy")
        .add_attribute("collection", collection_addr)
        .add_attribute("delegate", delegate_attr)
        .add_attribute("designated_successor", successor_attr))
}

#[allow(clippy::too_many_arguments)]
fn execute_register_ecosystem_from_factory(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    creator: String,
    collection_factory: String,
    description: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let authorized_factory = config
        .ecosystem_factory
        .clone()
        .ok_or(ContractError::EcosystemFactoryNotConfigured {})?;

    if info.sender != authorized_factory {
        return Err(ContractError::NotEcosystemFactory {});
    }

    validate_id(&id)?;
    if name.trim().is_empty() {
        return Err(ContractError::EmptyName {});
    }
    if description.trim().is_empty() {
        return Err(ContractError::EmptyDescription {});
    }
    if image_urls.is_empty() || image_urls.iter().any(|img| img.trim().is_empty()) {
        return Err(ContractError::EmptyImages {});
    }
    if ECOSYSTEMS.has(deps.storage, id.clone()) {
        return Err(ContractError::EcosystemAlreadyExists { id });
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    if !can_create_ecosystem(deps.storage, &creator_addr)? {
        return Err(ContractError::CreatorCannotCreateEcosystem { creator });
    }
    let collection_factory_addr = deps.api.addr_validate(&collection_factory)?;

    // All ecosystems from factory are Private with Permissioned policy
    let ecosystem_type = EcosystemType::Private;
    let collection_creation_policy = CollectionCreationPolicy::Permissioned;

    let ecosystem = Ecosystem {
        id: id.clone(),
        name,
        admin: creator_addr.clone(),
        ecosystem_type,
        collection_creation_policy,
        collection_factory: Some(collection_factory_addr.clone()),
        description,
        image_urls,
        animation_url,
        url,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    ECOSYSTEMS.save(deps.storage, id.clone(), &ecosystem)?;
    ECOSYSTEM_MEMBERS.save(deps.storage, (id.clone(), creator_addr.clone()), &true)?;

    let count = ECOSYSTEM_COUNT.load(deps.storage)?;
    ECOSYSTEM_COUNT.save(deps.storage, &(count + 1))?;
    touch_creator_activity(deps.storage, &creator_addr, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_ecosystem_from_factory")
        .add_attribute("ecosystem_id", id)
        .add_attribute("creator", creator_addr)
        .add_attribute("collection_factory", collection_factory_addr)
        .add_attribute("factory", info.sender))
}

fn execute_update_ecosystem(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: Option<String>,
    description: Option<String>,
    image_urls: Option<Vec<String>>,
    animation_url: Option<String>,
    url: Option<String>,
    ecosystem_type: Option<EcosystemType>,
    collection_creation_policy: Option<CollectionCreationPolicy>,
    collection_factory: Option<String>,
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut ecosystem = ECOSYSTEMS
        .load(deps.storage, id.clone())
        .map_err(|_| ContractError::EcosystemNotFound { id: id.clone() })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    if let Some(new_name) = name {
        if new_name.is_empty() {
            return Err(ContractError::EmptyName {});
        }
        ecosystem.name = new_name;
    }

    if let Some(new_description) = description {
        if new_description.is_empty() {
            return Err(ContractError::EmptyDescription {});
        }
        ecosystem.description = new_description;
    }

    if let Some(new_image_urls) = image_urls {
        if new_image_urls.is_empty() || new_image_urls.iter().any(|img| img.is_empty()) {
            return Err(ContractError::EmptyImages {});
        }
        ecosystem.image_urls = new_image_urls;
    }

    if let Some(new_animation_url) = animation_url {
        ecosystem.animation_url = Some(new_animation_url);
    }

    if let Some(new_url) = url {
        ecosystem.url = Some(new_url);
    }

    if let Some(new_type) = ecosystem_type {
        ecosystem.ecosystem_type = new_type;
    }

    if let Some(new_policy) = collection_creation_policy {
        ecosystem.collection_creation_policy = new_policy;
    }

    validate_ecosystem_policy(
        &ecosystem.ecosystem_type,
        &ecosystem.collection_creation_policy,
    )?;

    if let Some(new_factory) = collection_factory {
        if new_factory.trim().is_empty() {
            ecosystem.collection_factory = None;
        } else {
            ecosystem.collection_factory = Some(deps.api.addr_validate(&new_factory)?);
        }
    }

    if let Some(new_admin) = admin {
        let new_admin_addr = deps.api.addr_validate(&new_admin)?;
        ecosystem.admin = new_admin_addr.clone();
        ECOSYSTEM_MEMBERS.save(deps.storage, (id.clone(), new_admin_addr), &true)?;
    }

    ecosystem.updated_at = env.block.time.seconds();

    ECOSYSTEMS.save(deps.storage, id.clone(), &ecosystem)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "update_ecosystem")
        .add_attribute("ecosystem_id", id))
}

fn execute_approve_ecosystem_member(
    deps: DepsMut,
    info: MessageInfo,
    ecosystem_id: String,
    member: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    let member_addr = deps.api.addr_validate(&member)?;
    ECOSYSTEM_MEMBERS.save(
        deps.storage,
        (ecosystem_id.clone(), member_addr.clone()),
        &true,
    )?;

    Ok(Response::new()
        .add_attribute("action", "approve_ecosystem_member")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("member", member_addr))
}

fn execute_revoke_ecosystem_member(
    deps: DepsMut,
    info: MessageInfo,
    ecosystem_id: String,
    member: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    let member_addr = deps.api.addr_validate(&member)?;
    ECOSYSTEM_MEMBERS.remove(deps.storage, (ecosystem_id.clone(), member_addr.clone()));

    Ok(Response::new()
        .add_attribute("action", "revoke_ecosystem_member")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("member", member_addr))
}

fn execute_register_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
    nft_type: NftType,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if ecosystem.collection_factory.is_some() {
        return Err(ContractError::CollectionFactoryRequired { ecosystem_id });
    }

    if !can_create_collection_in_ecosystem(deps.storage, &config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::EcosystemMemberNotApproved {
            ecosystem_id: ecosystem.id,
        });
    }

    if name.is_empty() {
        return Err(ContractError::EmptyName {});
    }

    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let creator = effective_collection_creator(&config, &ecosystem, &info.sender);
    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: Some(ecosystem_id.clone()),
        name,
        nft_type: nft_type.clone(),
        creator: creator.clone(),
        verified: false,
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &creator, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", creator)
        .add_attribute("nft_type", nft_type.to_string()))
}

fn execute_register_collection_from_factory(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
    creator: String,
    nft_type: NftType,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;
    let creator_addr = deps.api.addr_validate(&creator)?;

    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    let authorized_factory = ecosystem
        .collection_factory
        .clone()
        .ok_or(ContractError::CollectionFactoryRequired {
            ecosystem_id: ecosystem_id.clone(),
        })?;

    if info.sender != authorized_factory {
        if !can_cross_admin_manage_ecosystem(&config, &ecosystem, &info.sender) {
            if is_cross_ecosystem_admin(&config, &info.sender)
                && is_owner_controlled_ecosystem(&config, &ecosystem)
            {
                return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
            }
            return Err(ContractError::NotEcosystemFactory {});
        }
    }

    if !can_create_collection_in_ecosystem(deps.storage, &config, &ecosystem, &creator_addr) {
        if is_cross_ecosystem_admin(&config, &creator_addr)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::EcosystemMemberNotApproved { ecosystem_id });
    }

    if name.trim().is_empty() {
        return Err(ContractError::EmptyName {});
    }

    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let final_creator = effective_collection_creator(&config, &ecosystem, &creator_addr);
    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: Some(ecosystem_id.clone()),
        name,
        nft_type: nft_type.clone(),
        creator: final_creator.clone(),
        verified: false,
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &final_creator, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &creator_addr, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_collection_from_factory")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", final_creator)
        .add_attribute("nft_type", nft_type.to_string()))
}

fn execute_register_existing_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
    creator: String,
    nft_type: NftType,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;
    let creator_addr = deps.api.addr_validate(&creator)?;

    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: Some(ecosystem_id.clone()),
        name,
        nft_type: nft_type.clone(),
        creator: creator_addr.clone(),
        verified: true,
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &creator_addr, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_existing_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", creator_addr)
        .add_attribute("nft_type", nft_type.to_string()))
}

fn execute_deregister_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    let mut collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: address.clone(),
        })?;

    let ecosystem_id = collection
        .ecosystem_id
        .clone()
        .ok_or(ContractError::CollectionNotAffiliated { address })?;

    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    collection.ecosystem_id = None;
    collection.updated_at = env.block.time.seconds();
    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "deregister_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id))
}

fn execute_rehome_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    let mut collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: address.clone(),
        })?;

    if let Some(current_ecosystem_id) = &collection.ecosystem_id {
        return Err(ContractError::CollectionAlreadyAffiliated {
            ecosystem_id: current_ecosystem_id.clone(),
        });
    }

    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_cross_ecosystem_admin(&config, &info.sender)
            && is_owner_controlled_ecosystem(&config, &ecosystem)
        {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotEcosystemAdmin {});
    }

    collection.ecosystem_id = Some(ecosystem_id.clone());
    collection.updated_at = env.block.time.seconds();
    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "rehome_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", collection.creator))
}

fn execute_update_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    name: Option<String>,
    verified: Option<bool>,
    minter: Option<String>,
    marketplace: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    let mut collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: address.clone(),
        })?;

    if !can_manage_collection(deps.storage, &config, &collection, &info.sender)? {
        if is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotCollectionCreator {});
    }

    if let Some(new_name) = name {
        if new_name.is_empty() {
            return Err(ContractError::EmptyName {});
        }
        collection.name = new_name;
    }

    // Only admin can set verified status
    if let Some(new_verified) = verified {
        if !is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::Unauthorized {});
        }
        collection.verified = new_verified;
    }

    if let Some(new_minter) = minter {
        collection.minter = Some(deps.api.addr_validate(&new_minter)?);
    }

    if let Some(new_marketplace) = marketplace {
        collection.marketplace = Some(deps.api.addr_validate(&new_marketplace)?);
    }

    collection.updated_at = env.block.time.seconds();

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "update_collection")
        .add_attribute("collection", collection_addr))
}

fn execute_transfer_collection_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    new_creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;
    let new_creator_addr = deps.api.addr_validate(&new_creator)?;

    let mut collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: address.clone(),
        })?;

    if !can_manage_collection(deps.storage, &config, &collection, &info.sender)? {
        if is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotCollectionCreator {});
    }

    collection.creator = new_creator_addr.clone();
    collection.updated_at = env.block.time.seconds();

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &new_creator_addr, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "transfer_collection_ownership")
        .add_attribute("collection", collection_addr)
        .add_attribute("new_creator", new_creator_addr))
}

fn execute_authorize_minter(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: String,
    minter_address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection_address)?;
    let minter_addr = deps.api.addr_validate(&minter_address)?;

    let collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: collection_address.clone(),
        })?;

    if !can_manage_collection(deps.storage, &config, &collection, &info.sender)? {
        if is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotCollectionCreator {});
    }

    let key = (collection_addr.clone(), minter_addr.clone());

    if AUTHORIZED_MINTERS.has(deps.storage, key.clone()) {
        return Err(ContractError::MinterAlreadyAuthorized {
            address: minter_address,
        });
    }

    let authorized_minter = AuthorizedMinter {
        minter_address: minter_addr.clone(),
        collection_address: collection_addr.clone(),
        authorized_by: info.sender.clone(),
        created_at: env.block.time.seconds(),
    };

    AUTHORIZED_MINTERS.save(deps.storage, key, &authorized_minter)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "authorize_minter")
        .add_attribute("collection", collection_addr)
        .add_attribute("minter", minter_addr))
}

fn execute_revoke_minter(
    deps: DepsMut,
    info: MessageInfo,
    collection_address: String,
    minter_address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&collection_address)?;
    let minter_addr = deps.api.addr_validate(&minter_address)?;

    let collection = collections()
        .load(deps.storage, collection_addr.clone())
        .map_err(|_| ContractError::CollectionNotFound {
            address: collection_address.clone(),
        })?;

    if !can_manage_collection(deps.storage, &config, &collection, &info.sender)? {
        if is_cross_ecosystem_admin(&config, &info.sender) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::NotCollectionCreator {});
    }

    let key = (collection_addr.clone(), minter_addr.clone());

    if !AUTHORIZED_MINTERS.has(deps.storage, key.clone()) {
        return Err(ContractError::MinterNotAuthorized {
            address: minter_address,
        });
    }

    AUTHORIZED_MINTERS.remove(deps.storage, key);

    Ok(Response::new()
        .add_attribute("action", "revoke_minter")
        .add_attribute("collection", collection_addr)
        .add_attribute("minter", minter_addr))
}

fn execute_update_recovery_config(
    deps: DepsMut,
    info: MessageInfo,
    abandonment_inactivity_period_secs: Option<u64>,
    contest_period_secs: Option<u64>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut recovery_config = RECOVERY_CONFIG.load(deps.storage)?;

    if let Some(inactivity) = abandonment_inactivity_period_secs {
        if inactivity == 0 {
            return Err(ContractError::InvalidRecoveryConfig {});
        }
        recovery_config.abandonment_inactivity_period_secs = inactivity;
    }

    if let Some(contest) = contest_period_secs {
        if contest == 0 {
            return Err(ContractError::InvalidRecoveryConfig {});
        }
        recovery_config.contest_period_secs = contest;
    }

    RECOVERY_CONFIG.save(deps.storage, &recovery_config)?;

    Ok(Response::new()
        .add_attribute("action", "update_recovery_config")
        .add_attribute(
            "abandonment_inactivity_period_secs",
            recovery_config
                .abandonment_inactivity_period_secs
                .to_string(),
        )
        .add_attribute(
            "contest_period_secs",
            recovery_config.contest_period_secs.to_string(),
        ))
}

fn execute_open_recovery_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    case_kind: RecoveryCaseKind,
    target: RecoveryTargetInput,
    reason: String,
    evidence_url: Option<String>,
    proposed_replacement: Option<String>,
) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let reason = reason.trim().to_string();
    if reason.is_empty() {
        return Err(ContractError::EmptyRecoveryReason {});
    }

    let recovery_config = RECOVERY_CONFIG.load(deps.storage)?;
    if recovery_config.abandonment_inactivity_period_secs == 0
        || recovery_config.contest_period_secs == 0
    {
        return Err(ContractError::InvalidRecoveryConfig {});
    }
    let config = CONFIG.load(deps.storage)?;

    let target = match target {
        RecoveryTargetInput::Ecosystem { ecosystem_id } => {
            RecoveryTarget::Ecosystem { ecosystem_id }
        }
        RecoveryTargetInput::Collection { address } => RecoveryTarget::Collection {
            address: deps.api.addr_validate(&address)?,
        },
    };

    let target_key = recovery_target_key(&target);
    let (target_admin, last_activity_at) =
        resolve_target_admin_and_last_activity(deps.as_ref(), &target)?;
    if has_open_case_for_target(deps.as_ref(), &target)? {
        return Err(ContractError::RecoveryCaseAlreadyOpen { target: target_key });
    }

    let policy = recovery_policy_for_target(deps.storage, &target)?;
    let replacement_candidate = proposed_replacement
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .or(policy.designated_successor.clone())
        .ok_or(ContractError::RecoveryReplacementRequired {})?;

    match case_kind {
        RecoveryCaseKind::Abandonment => {
            let inactive_until =
                last_activity_at + recovery_config.abandonment_inactivity_period_secs;
            if now < inactive_until {
                return Err(ContractError::RecoveryAbandonmentThresholdNotMet {
                    target: recovery_target_key(&target),
                });
            }
        }
        RecoveryCaseKind::LostAccess => {
            let is_policy_actor = policy.delegate.as_ref() == Some(&info.sender)
                || policy.designated_successor.as_ref() == Some(&info.sender);
            if policy.delegate.is_none()
                && policy.designated_successor.is_none()
                && !is_recovery_council(&config, &info.sender)
            {
                return Err(ContractError::LostAccessRecoveryNotConfigured {
                    target: recovery_target_key(&target),
                });
            }
            if !is_policy_actor && !is_recovery_council(&config, &info.sender) {
                return Err(ContractError::Unauthorized {});
            }
        }
    }

    let case_id = RECOVERY_CASE_COUNT.load(deps.storage)? + 1;
    RECOVERY_CASE_COUNT.save(deps.storage, &case_id)?;

    let case = RecoveryCase {
        case_id,
        kind: case_kind.clone(),
        target: target.clone(),
        target_admin: target_admin.clone(),
        opened_by: info.sender.clone(),
        reason,
        evidence_url,
        replacement_candidate: replacement_candidate.clone(),
        last_target_activity_at: last_activity_at,
        status: RecoveryCaseStatus::Open,
        opened_at: now,
        contest_deadline: now + recovery_config.contest_period_secs,
        contested_at: None,
        contested_by: None,
        contest_note: None,
        resolved_at: None,
        resolved_by: None,
        resolution_approved: None,
        resolution_note: None,
    };

    RECOVERY_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    let mut response = Response::new()
        .add_attribute("action", "open_recovery_case")
        .add_attribute("case_id", case_id.to_string())
        .add_attribute("case_kind", format!("{case_kind:?}"))
        .add_attribute("target", recovery_target_key(&target))
        .add_attribute("target_admin", target_admin)
        .add_attribute("last_target_activity_at", last_activity_at.to_string())
        .add_attribute("replacement_candidate", replacement_candidate)
        .add_attribute("contest_deadline", case.contest_deadline.to_string());

    if case_kind == RecoveryCaseKind::Abandonment {
        let inactive_until = last_activity_at + recovery_config.abandonment_inactivity_period_secs;
        response = response.add_attribute("inactive_until", inactive_until.to_string());
    }

    Ok(response)
}

fn execute_contest_recovery_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    case_id: u64,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut case = RECOVERY_CASES
        .may_load(deps.storage, case_id)?
        .ok_or(ContractError::RecoveryCaseNotFound { case_id })?;

    if case.status != RecoveryCaseStatus::Open || now > case.contest_deadline {
        return Err(ContractError::RecoveryCaseNotContestable { case_id });
    }

    if info.sender != case.target_admin {
        return Err(ContractError::OnlyTargetAdminCanContestRecoveryCase {});
    }

    case.status = RecoveryCaseStatus::Contested;
    case.contested_at = Some(now);
    case.contested_by = Some(info.sender.clone());
    case.contest_note = note;

    RECOVERY_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    Ok(Response::new()
        .add_attribute("action", "contest_recovery_case")
        .add_attribute("case_id", case_id.to_string()))
}

fn execute_resolve_recovery_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    case_id: u64,
    approved: bool,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_recovery_council(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let now = env.block.time.seconds();
    let mut case = RECOVERY_CASES
        .may_load(deps.storage, case_id)?
        .ok_or(ContractError::RecoveryCaseNotFound { case_id })?;

    if case.status == RecoveryCaseStatus::Resolved {
        return Err(ContractError::RecoveryCaseAlreadyResolved { case_id });
    }
    if now <= case.contest_deadline {
        return Err(ContractError::RecoveryCaseContestWindowOpen { case_id });
    }

    case.status = RecoveryCaseStatus::Resolved;
    case.resolved_at = Some(now);
    case.resolved_by = Some(info.sender.clone());
    case.resolution_approved = Some(approved);
    case.resolution_note = note;

    let mut replacement_attr = None;
    if approved {
        let replacement = case.replacement_candidate.clone();

        match &case.target {
            RecoveryTarget::Ecosystem { ecosystem_id } => {
                let mut ecosystem = ECOSYSTEMS
                    .load(deps.storage, ecosystem_id.clone())
                    .map_err(|_| ContractError::EcosystemNotFound {
                        id: ecosystem_id.clone(),
                    })?;
                ecosystem.admin = replacement.clone();
                ecosystem.updated_at = now;
                ECOSYSTEMS.save(deps.storage, ecosystem_id.clone(), &ecosystem)?;
                ECOSYSTEM_MEMBERS.save(
                    deps.storage,
                    (ecosystem_id.clone(), replacement.clone()),
                    &true,
                )?;
            }
            RecoveryTarget::Collection { address } => {
                let mut collection =
                    collections()
                        .load(deps.storage, address.clone())
                        .map_err(|_| ContractError::CollectionNotFound {
                            address: address.to_string(),
                        })?;
                collection.creator = replacement.clone();
                collection.updated_at = now;
                collections().save(deps.storage, address.clone(), &collection)?;
            }
        }

        touch_creator_activity(deps.storage, &replacement, now)?;
        replacement_attr = Some(replacement);
    }

    RECOVERY_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    let mut response = Response::new()
        .add_attribute("action", "resolve_recovery_case")
        .add_attribute("case_id", case_id.to_string())
        .add_attribute("approved", approved.to_string());

    if let Some(replacement) = replacement_attr {
        response = response.add_attribute("replacement", replacement);
    }

    Ok(response)
}

fn resolve_target_admin_and_last_activity(
    deps: Deps,
    target: &RecoveryTarget,
) -> Result<(Addr, u64), ContractError> {
    match target {
        RecoveryTarget::Ecosystem { ecosystem_id } => {
            let ecosystem = ECOSYSTEMS
                .load(deps.storage, ecosystem_id.clone())
                .map_err(|_| ContractError::EcosystemNotFound {
                    id: ecosystem_id.clone(),
                })?;
            let last_activity_at = LAST_CREATOR_ACTIVITY
                .may_load(deps.storage, ecosystem.admin.clone())?
                .unwrap_or(ecosystem.updated_at);
            Ok((ecosystem.admin, last_activity_at))
        }
        RecoveryTarget::Collection { address } => {
            let collection = collections()
                .load(deps.storage, address.clone())
                .map_err(|_| ContractError::CollectionNotFound {
                    address: address.to_string(),
                })?;
            let last_activity_at = LAST_CREATOR_ACTIVITY
                .may_load(deps.storage, collection.creator.clone())?
                .unwrap_or(collection.updated_at);
            Ok((collection.creator, last_activity_at))
        }
    }
}

fn has_open_case_for_target(deps: Deps, target: &RecoveryTarget) -> StdResult<bool> {
    let cases = RECOVERY_CASES.range(deps.storage, None, None, Order::Ascending);
    for item in cases {
        let (_, case) = item?;
        if case.target == *target && case.status != RecoveryCaseStatus::Resolved {
            return Ok(true);
        }
    }
    Ok(false)
}


