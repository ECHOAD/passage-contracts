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
            ecosystem_factory,
            paused,
        } => execute_update_config(deps, info, admin, operators, ecosystem_factory, paused),
        ExecuteMsg::ApproveEcosystemCreator { creator } => {
            execute_approve_ecosystem_creator(deps, info, creator)
        }
        ExecuteMsg::RevokeEcosystemCreator { creator } => {
            execute_revoke_ecosystem_creator(deps, info, creator)
        }

        // Ecosystem operations
        ExecuteMsg::RegisterEcosystem {
            id,
            name,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            detail,
            image_urls,
            animation_url,
            url,
        } => execute_register_ecosystem(
            deps,
            env,
            info,
            id,
            name,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            detail,
            image_urls,
            animation_url,
            url,
        ),
        ExecuteMsg::RegisterEcosystemFromFactory {
            id,
            name,
            creator,
            collection_factory,
            detail,
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
            detail,
            image_urls,
            animation_url,
            url,
        ),
        ExecuteMsg::SubmitEcosystemCreationRequest {
            id,
            name,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            detail,
            image_urls,
            animation_url,
            url,
        } => execute_submit_ecosystem_creation_request(
            deps,
            env,
            info,
            id,
            name,
            ecosystem_type,
            collection_creation_policy,
            collection_factory,
            detail,
            image_urls,
            animation_url,
            url,
        ),
        ExecuteMsg::ResolveEcosystemCreationRequest {
            request_id,
            approved,
            note,
        } => {
            execute_resolve_ecosystem_creation_request(deps, env, info, request_id, approved, note)
        }
        ExecuteMsg::UpdateEcosystem {
            id,
            name,
            detail,
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
            detail,
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
        ExecuteMsg::SubmitCollectionCreationRequest { ecosystem_id, note } => {
            execute_submit_collection_creation_request(deps, env, info, ecosystem_id, note)
        }
        ExecuteMsg::ResolveCollectionCreationRequest {
            ecosystem_id,
            creator,
            approved,
            note,
        } => execute_resolve_collection_creation_request(
            deps,
            env,
            info,
            ecosystem_id,
            creator,
            approved,
            note,
        ),

        // Collection operations
        ExecuteMsg::RegisterCollection {
            address,
            ecosystem_id,
            name,
        } => execute_register_collection(deps, env, info, address, ecosystem_id, name),
        ExecuteMsg::RegisterCollectionFromFactory {
            address,
            ecosystem_id,
            name,
            creator,
        } => execute_register_collection_from_factory(
            deps,
            env,
            info,
            address,
            ecosystem_id,
            name,
            creator,
        ),
        ExecuteMsg::RegisterExistingCollection {
            address,
            ecosystem_id,
            name,
            creator,
        } => execute_register_existing_collection(
            deps,
            env,
            info,
            address,
            ecosystem_id,
            name,
            creator,
        ),
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

        // Dead project recovery governance
        ExecuteMsg::UpdateRecoveryConfig {
            inactivity_period_secs,
            contest_period_secs,
        } => {
            execute_update_recovery_config(deps, info, inactivity_period_secs, contest_period_secs)
        }
        ExecuteMsg::OpenDeadProjectCase {
            target,
            reason,
            evidence_url,
            proposed_replacement,
        } => execute_open_dead_project_case(
            deps,
            env,
            info,
            target,
            reason,
            evidence_url,
            proposed_replacement,
        ),
        ExecuteMsg::ContestDeadProjectCase { case_id, note } => {
            execute_contest_dead_project_case(deps, env, info, case_id, note)
        }
        ExecuteMsg::ResolveDeadProjectCase {
            case_id,
            approved,
            note,
        } => execute_resolve_dead_project_case(deps, env, info, case_id, approved, note),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    operators: Option<Vec<String>>,
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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

fn execute_approve_ecosystem_creator(
    deps: DepsMut,
    info: MessageInfo,
    creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    APPROVED_ECOSYSTEM_CREATORS.save(deps.storage, creator_addr.clone(), &true)?;

    Ok(Response::new()
        .add_attribute("action", "approve_ecosystem_creator")
        .add_attribute("creator", creator_addr))
}

fn execute_revoke_ecosystem_creator(
    deps: DepsMut,
    info: MessageInfo,
    creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    APPROVED_ECOSYSTEM_CREATORS.remove(deps.storage, creator_addr.clone());

    Ok(Response::new()
        .add_attribute("action", "revoke_ecosystem_creator")
        .add_attribute("creator", creator_addr))
}

fn execute_register_ecosystem(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    ecosystem_type: Option<EcosystemType>,
    collection_creation_policy: Option<CollectionCreationPolicy>,
    collection_factory: Option<String>,
    detail: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Direct registration is reserved for cross-ecosystem admins.
    // Non-admin creators must use the ecosystem creation request flow.
    if !is_cross_ecosystem_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    validate_id(&id)?;
    if name.is_empty() {
        return Err(ContractError::EmptyName {});
    }
    if detail.is_empty() {
        return Err(ContractError::EmptyDescription {});
    }
    if image_urls.is_empty() || image_urls.iter().any(|img| img.is_empty()) {
        return Err(ContractError::EmptyImages {});
    }

    // Check if ecosystem already exists
    if ECOSYSTEMS.has(deps.storage, id.clone()) {
        return Err(ContractError::EcosystemAlreadyExists { id });
    }

    let ecosystem_type = ecosystem_type.unwrap_or_default();
    let collection_creation_policy = collection_creation_policy
        .unwrap_or_else(|| CollectionCreationPolicy::default_for_type(&ecosystem_type));
    validate_ecosystem_policy(&ecosystem_type, &collection_creation_policy)?;
    let collection_factory = collection_factory
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let ecosystem = Ecosystem {
        id: id.clone(),
        name,
        admin: info.sender.clone(),
        ecosystem_type,
        collection_creation_policy,
        collection_factory,
        detail,
        image_urls,
        animation_url,
        url,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    ECOSYSTEMS.save(deps.storage, id.clone(), &ecosystem)?;
    ECOSYSTEM_MEMBERS.save(deps.storage, (id.clone(), info.sender.clone()), &true)?;

    let count = ECOSYSTEM_COUNT.load(deps.storage)?;
    ECOSYSTEM_COUNT.save(deps.storage, &(count + 1))?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_ecosystem")
        .add_attribute("ecosystem_id", id)
        .add_attribute("admin", info.sender))
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
    detail: String,
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
    if detail.trim().is_empty() {
        return Err(ContractError::EmptyDescription {});
    }
    if image_urls.is_empty() || image_urls.iter().any(|img| img.trim().is_empty()) {
        return Err(ContractError::EmptyImages {});
    }
    if ECOSYSTEMS.has(deps.storage, id.clone()) {
        return Err(ContractError::EcosystemAlreadyExists { id });
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
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
        detail,
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

#[allow(clippy::too_many_arguments)]
fn execute_submit_ecosystem_creation_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    ecosystem_type: Option<EcosystemType>,
    collection_creation_policy: Option<CollectionCreationPolicy>,
    collection_factory: Option<String>,
    detail: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.ecosystem_factory.is_some() {
        return Err(ContractError::EcosystemFactoryFlowRequired {});
    }

    if is_cross_ecosystem_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    validate_id(&id)?;
    if name.trim().is_empty() {
        return Err(ContractError::EmptyName {});
    }
    if detail.trim().is_empty() {
        return Err(ContractError::EmptyDescription {});
    }
    if image_urls.is_empty() || image_urls.iter().any(|img| img.trim().is_empty()) {
        return Err(ContractError::EmptyImages {});
    }
    if ECOSYSTEMS.has(deps.storage, id.clone()) {
        return Err(ContractError::EcosystemAlreadyExists { id });
    }
    if PENDING_ECOSYSTEM_REQUEST_BY_ID.has(deps.storage, id.clone()) {
        return Err(ContractError::EcosystemCreationRequestAlreadyPending { id });
    }

    let ecosystem_type = ecosystem_type.unwrap_or_default();
    let collection_creation_policy = collection_creation_policy
        .unwrap_or_else(|| CollectionCreationPolicy::default_for_type(&ecosystem_type));
    validate_ecosystem_policy(&ecosystem_type, &collection_creation_policy)?;
    let collection_factory = collection_factory
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let request_id = NEXT_ECOSYSTEM_REQUEST_ID.load(deps.storage)?;
    NEXT_ECOSYSTEM_REQUEST_ID.save(deps.storage, &(request_id + 1))?;

    let request = EcosystemCreationRequest {
        request_id,
        creator: info.sender.clone(),
        id: id.clone(),
        name,
        ecosystem_type,
        collection_creation_policy,
        collection_factory,
        detail,
        image_urls,
        animation_url,
        url,
        status: EcosystemCreationRequestStatus::Pending,
        submitted_at: env.block.time.seconds(),
        reviewed_at: None,
        reviewed_by: None,
        review_note: None,
    };

    ECOSYSTEM_CREATION_REQUESTS.save(deps.storage, request_id, &request)?;
    PENDING_ECOSYSTEM_REQUEST_BY_ID.save(deps.storage, id.clone(), &request_id)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "submit_ecosystem_creation_request")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("ecosystem_id", id)
        .add_attribute("creator", info.sender))
}

fn execute_resolve_ecosystem_creation_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    request_id: u64,
    approved: bool,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_cross_ecosystem_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut request = ECOSYSTEM_CREATION_REQUESTS
        .may_load(deps.storage, request_id)?
        .ok_or(ContractError::EcosystemCreationRequestNotFound { request_id })?;

    if request.status != EcosystemCreationRequestStatus::Pending {
        return Err(ContractError::EcosystemCreationRequestAlreadyResolved { request_id });
    }

    if approved {
        if ECOSYSTEMS.has(deps.storage, request.id.clone()) {
            return Err(ContractError::EcosystemAlreadyExists { id: request.id });
        }

        let ecosystem = Ecosystem {
            id: request.id.clone(),
            name: request.name.clone(),
            admin: request.creator.clone(),
            ecosystem_type: request.ecosystem_type.clone(),
            collection_creation_policy: request.collection_creation_policy.clone(),
            collection_factory: request.collection_factory.clone(),
            detail: request.detail.clone(),
            image_urls: request.image_urls.clone(),
            animation_url: request.animation_url.clone(),
            url: request.url.clone(),
            created_at: env.block.time.seconds(),
            updated_at: env.block.time.seconds(),
        };

        ECOSYSTEMS.save(deps.storage, request.id.clone(), &ecosystem)?;
        ECOSYSTEM_MEMBERS.save(
            deps.storage,
            (request.id.clone(), request.creator.clone()),
            &true,
        )?;

        let count = ECOSYSTEM_COUNT.load(deps.storage)?;
        ECOSYSTEM_COUNT.save(deps.storage, &(count + 1))?;
        touch_creator_activity(deps.storage, &request.creator, env.block.time.seconds())?;
    }

    request.status = if approved {
        EcosystemCreationRequestStatus::Approved
    } else {
        EcosystemCreationRequestStatus::Rejected
    };
    request.reviewed_at = Some(env.block.time.seconds());
    request.reviewed_by = Some(info.sender.clone());
    request.review_note = note;

    ECOSYSTEM_CREATION_REQUESTS.save(deps.storage, request_id, &request)?;
    PENDING_ECOSYSTEM_REQUEST_BY_ID.remove(deps.storage, request.id.clone());
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "resolve_ecosystem_creation_request")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("ecosystem_id", request.id)
        .add_attribute("creator", request.creator)
        .add_attribute("approved", approved.to_string()))
}

fn execute_update_ecosystem(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: Option<String>,
    detail: Option<String>,
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

    if let Some(new_detail) = detail {
        if new_detail.is_empty() {
            return Err(ContractError::EmptyDescription {});
        }
        ecosystem.detail = new_detail;
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

fn execute_submit_collection_creation_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ecosystem_id: String,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if ecosystem.collection_creation_policy != CollectionCreationPolicy::ApprovalRequired {
        return Err(ContractError::CollectionCreationRequestNotAllowed { ecosystem_id });
    }

    if can_create_collection_in_ecosystem(deps.storage, &config, &ecosystem, &info.sender) {
        return Ok(Response::new()
            .add_attribute("action", "submit_collection_creation_request")
            .add_attribute("ecosystem_id", ecosystem_id)
            .add_attribute("creator", info.sender)
            .add_attribute("already_approved", "true"));
    }

    let key = (ecosystem_id.clone(), info.sender.clone());
    if let Some(existing) = COLLECTION_CREATION_REQUESTS.may_load(deps.storage, key.clone())? {
        if existing.status == CollectionCreationRequestStatus::Pending {
            return Err(ContractError::CollectionCreationRequestAlreadyPending { ecosystem_id });
        }
    }

    let request = CollectionCreationRequest {
        ecosystem_id: ecosystem_id.clone(),
        creator: info.sender.clone(),
        note,
        status: CollectionCreationRequestStatus::Pending,
        submitted_at: env.block.time.seconds(),
        reviewed_at: None,
        reviewed_by: None,
        review_note: None,
    };

    COLLECTION_CREATION_REQUESTS.save(deps.storage, key, &request)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "submit_collection_creation_request")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", info.sender))
}

fn execute_resolve_collection_creation_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ecosystem_id: String,
    creator: String,
    approved: bool,
    note: Option<String>,
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

    let creator_addr = deps.api.addr_validate(&creator)?;
    let key = (ecosystem_id.clone(), creator_addr.clone());
    let mut request = COLLECTION_CREATION_REQUESTS
        .may_load(deps.storage, key.clone())?
        .ok_or(ContractError::CollectionCreationRequestNotFound {
            ecosystem_id: ecosystem_id.clone(),
        })?;

    if request.status != CollectionCreationRequestStatus::Pending {
        return Err(ContractError::CollectionCreationRequestAlreadyResolved {
            ecosystem_id: ecosystem_id.clone(),
        });
    }

    request.status = if approved {
        CollectionCreationRequestStatus::Approved
    } else {
        CollectionCreationRequestStatus::Rejected
    };
    request.reviewed_at = Some(env.block.time.seconds());
    request.reviewed_by = Some(info.sender.clone());
    request.review_note = note;

    COLLECTION_CREATION_REQUESTS.save(deps.storage, key, &request)?;

    if approved {
        ECOSYSTEM_MEMBERS.save(
            deps.storage,
            (ecosystem_id.clone(), creator_addr.clone()),
            &true,
        )?;
    }

    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &creator_addr, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "resolve_collection_creation_request")
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", creator_addr)
        .add_attribute("approved", approved.to_string()))
}

fn execute_register_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    // Check ecosystem exists
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
        return Err(ContractError::EcosystemMemberNotApproved { ecosystem_id });
    }

    if name.is_empty() {
        return Err(ContractError::EmptyName {});
    }

    // Check if collection already registered
    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let creator = effective_collection_creator(&config, &ecosystem, &info.sender);
    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: ecosystem_id.clone(),
        name,
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
        .add_attribute("creator", creator))
}

fn execute_register_collection_from_factory(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
    creator: String,
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
        .ok_or(ContractError::EcosystemFactoryNotConfigured {})?;

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
        ecosystem_id: ecosystem_id.clone(),
        name,
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
        .add_attribute("creator", final_creator))
}

fn execute_register_existing_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
    ecosystem_id: String,
    name: String,
    creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only admin can register existing collections
    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let collection_addr = deps.api.addr_validate(&address)?;
    let creator_addr = deps.api.addr_validate(&creator)?;

    // Check ecosystem exists
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    if !can_cross_admin_manage_ecosystem(&config, &ecosystem, &info.sender) {
        if is_owner_controlled_ecosystem(&config, &ecosystem) {
            return Err(ContractError::CrossAdminCannotManageOwnerEcosystem {});
        }
        return Err(ContractError::Unauthorized {});
    }

    // Check if collection already registered
    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let final_creator = effective_collection_creator(&config, &ecosystem, &creator_addr);
    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: ecosystem_id.clone(),
        name,
        creator: final_creator.clone(),
        verified: true, // Existing collections registered by admin are verified
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;
    touch_creator_activity(deps.storage, &info.sender, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &final_creator, env.block.time.seconds())?;
    touch_creator_activity(deps.storage, &creator_addr, env.block.time.seconds())?;

    Ok(Response::new()
        .add_attribute("action", "register_existing_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id)
        .add_attribute("creator", final_creator))
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
    inactivity_period_secs: Option<u64>,
    contest_period_secs: Option<u64>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let mut recovery_config = RECOVERY_CONFIG.load(deps.storage)?;

    if let Some(inactivity) = inactivity_period_secs {
        if inactivity == 0 {
            return Err(ContractError::InvalidRecoveryConfig {});
        }
        recovery_config.inactivity_period_secs = inactivity;
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
            "inactivity_period_secs",
            recovery_config.inactivity_period_secs.to_string(),
        )
        .add_attribute(
            "contest_period_secs",
            recovery_config.contest_period_secs.to_string(),
        ))
}

fn execute_open_dead_project_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
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
    if recovery_config.inactivity_period_secs == 0 || recovery_config.contest_period_secs == 0 {
        return Err(ContractError::InvalidRecoveryConfig {});
    }

    let target = match target {
        RecoveryTargetInput::Ecosystem { ecosystem_id } => {
            RecoveryTarget::Ecosystem { ecosystem_id }
        }
        RecoveryTargetInput::Collection { address } => RecoveryTarget::Collection {
            address: deps.api.addr_validate(&address)?,
        },
    };

    let (target_admin, last_activity_at) =
        resolve_target_admin_and_last_activity(deps.as_ref(), &target)?;
    let inactive_until = last_activity_at + recovery_config.inactivity_period_secs;
    let inactivity_threshold_met = now >= inactive_until;

    if has_open_case_for_target(deps.as_ref(), &target)? {
        return Err(ContractError::DeadProjectCaseAlreadyOpen {
            target: recovery_target_key(&target),
        });
    }

    let proposed_replacement = proposed_replacement
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let case_id = DEAD_PROJECT_CASE_COUNT.load(deps.storage)? + 1;
    DEAD_PROJECT_CASE_COUNT.save(deps.storage, &case_id)?;

    let case = DeadProjectCase {
        case_id,
        target: target.clone(),
        target_admin: target_admin.clone(),
        reporter: info.sender.clone(),
        reason,
        evidence_url,
        proposed_replacement,
        last_target_activity_at: last_activity_at,
        status: DeadProjectStatus::Open,
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

    DEAD_PROJECT_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    Ok(Response::new()
        .add_attribute("action", "open_dead_project_case")
        .add_attribute("case_id", case_id.to_string())
        .add_attribute("target", recovery_target_key(&target))
        .add_attribute("target_admin", target_admin)
        .add_attribute("last_target_activity_at", last_activity_at.to_string())
        .add_attribute("inactive_until", inactive_until.to_string())
        .add_attribute(
            "inactivity_threshold_met",
            inactivity_threshold_met.to_string(),
        )
        .add_attribute("contest_deadline", case.contest_deadline.to_string()))
}

fn execute_contest_dead_project_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    case_id: u64,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut case = DEAD_PROJECT_CASES
        .may_load(deps.storage, case_id)?
        .ok_or(ContractError::DeadProjectCaseNotFound { case_id })?;

    if case.status != DeadProjectStatus::Open || now > case.contest_deadline {
        return Err(ContractError::DeadProjectCaseNotContestable { case_id });
    }

    if info.sender != case.target_admin {
        return Err(ContractError::OnlyTargetAdminCanContest {});
    }

    case.status = DeadProjectStatus::Contested;
    case.contested_at = Some(now);
    case.contested_by = Some(info.sender.clone());
    case.contest_note = note;

    DEAD_PROJECT_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    Ok(Response::new()
        .add_attribute("action", "contest_dead_project_case")
        .add_attribute("case_id", case_id.to_string()))
}

fn execute_resolve_dead_project_case(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    case_id: u64,
    approved: bool,
    note: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let now = env.block.time.seconds();
    let mut case = DEAD_PROJECT_CASES
        .may_load(deps.storage, case_id)?
        .ok_or(ContractError::DeadProjectCaseNotFound { case_id })?;

    if case.status == DeadProjectStatus::Resolved {
        return Err(ContractError::DeadProjectCaseAlreadyResolved { case_id });
    }

    case.status = DeadProjectStatus::Resolved;
    case.resolved_at = Some(now);
    case.resolved_by = Some(info.sender.clone());
    case.resolution_approved = Some(approved);
    case.resolution_note = note;

    let mut replacement_attr = None;
    if approved {
        let replacement = case
            .proposed_replacement
            .clone()
            .ok_or(ContractError::RecoveryReplacementRequired {})?;

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

    DEAD_PROJECT_CASES.save(deps.storage, case_id, &case)?;
    touch_creator_activity(deps.storage, &info.sender, now)?;

    let mut response = Response::new()
        .add_attribute("action", "resolve_dead_project_case")
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
    let cases = DEAD_PROJECT_CASES.range(deps.storage, None, None, Order::Ascending);
    for item in cases {
        let (_, case) = item?;
        if case.target == *target && case.status != DeadProjectStatus::Resolved {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn approved_dead_project_case_transfers_collection_creator() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    admin: Addr::unchecked("admin"),
                    operators: vec![],
                    ecosystem_factory: Some(Addr::unchecked("factory")),
                    paused: false,
                },
            )
            .unwrap();

        collections()
            .save(
                deps.as_mut().storage,
                Addr::unchecked("collection"),
                &Collection {
                    address: Addr::unchecked("collection"),
                    ecosystem_id: "eco".to_string(),
                    name: "Collection".to_string(),
                    creator: Addr::unchecked("old_creator"),
                    verified: false,
                    minter: None,
                    marketplace: None,
                    created_at: 1,
                    updated_at: 1,
                },
            )
            .unwrap();

        DEAD_PROJECT_CASES
            .save(
                deps.as_mut().storage,
                1,
                &DeadProjectCase {
                    case_id: 1,
                    target: RecoveryTarget::Collection {
                        address: Addr::unchecked("collection"),
                    },
                    target_admin: Addr::unchecked("old_creator"),
                    reporter: Addr::unchecked("reporter"),
                    reason: "inactive".to_string(),
                    evidence_url: None,
                    proposed_replacement: Some(Addr::unchecked("new_creator")),
                    last_target_activity_at: 1,
                    status: DeadProjectStatus::Open,
                    opened_at: 1,
                    contest_deadline: 100,
                    contested_at: None,
                    contested_by: None,
                    contest_note: None,
                    resolved_at: None,
                    resolved_by: None,
                    resolution_approved: None,
                    resolution_note: None,
                },
            )
            .unwrap();

        let res = execute_resolve_dead_project_case(
            deps.as_mut(),
            env,
            mock_info("admin", &[]),
            1,
            true,
            None,
        )
        .unwrap();

        let collection = collections()
            .load(deps.as_ref().storage, Addr::unchecked("collection"))
            .unwrap();
        assert_eq!(collection.creator, Addr::unchecked("new_creator"));
        assert!(res
            .attributes
            .iter()
            .any(|attr| attr.key == "replacement" && attr.value == "new_creator"));
    }
}
