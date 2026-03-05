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
            paused,
        } => execute_update_config(deps, info, admin, operators, paused),
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
            detail,
            image_urls,
            animation_url,
            url,
        ),
        ExecuteMsg::UpdateEcosystem {
            id,
            name,
            detail,
            image_urls,
            animation_url,
            url,
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
        } => execute_register_collection(deps, env, info, address, ecosystem_id, name),
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
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    operators: Option<Vec<String>>,
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
    detail: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Gated: only admin/operators or explicitly approved creators can register ecosystems
    let is_approved_creator = APPROVED_ECOSYSTEM_CREATORS.has(deps.storage, info.sender.clone());
    if !is_admin_or_operator(&config, &info.sender) && !is_approved_creator {
        return Err(ContractError::EcosystemCreatorNotApproved {});
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

    let ecosystem = Ecosystem {
        id: id.clone(),
        name,
        admin: info.sender.clone(),
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

    Ok(Response::new()
        .add_attribute("action", "register_ecosystem")
        .add_attribute("ecosystem_id", id)
        .add_attribute("admin", info.sender))
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
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut ecosystem = ECOSYSTEMS
        .load(deps.storage, id.clone())
        .map_err(|_| ContractError::EcosystemNotFound { id: id.clone() })?;

    // Only ecosystem admin or contract admin can update
    if !is_admin(&config, &info.sender) && ecosystem.admin != info.sender {
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

    if let Some(new_admin) = admin {
        let new_admin_addr = deps.api.addr_validate(&new_admin)?;
        ecosystem.admin = new_admin_addr.clone();
        ECOSYSTEM_MEMBERS.save(deps.storage, (id.clone(), new_admin_addr), &true)?;
    }

    ecosystem.updated_at = env.block.time.seconds();

    ECOSYSTEMS.save(deps.storage, id.clone(), &ecosystem)?;

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

    if !is_admin(&config, &info.sender) && ecosystem.admin != info.sender {
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

    if !is_admin(&config, &info.sender) && ecosystem.admin != info.sender {
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
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let collection_addr = deps.api.addr_validate(&address)?;

    // Check ecosystem exists
    let ecosystem = ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    // Gated: admin, ecosystem admin, or explicitly approved ecosystem member
    let member_key = (ecosystem_id.clone(), info.sender.clone());
    let is_member_approved = ECOSYSTEM_MEMBERS.has(deps.storage, member_key);
    if !is_admin(&config, &info.sender) && ecosystem.admin != info.sender && !is_member_approved {
        return Err(ContractError::EcosystemMemberNotApproved { ecosystem_id });
    }

    if name.is_empty() {
        return Err(ContractError::EmptyName {});
    }

    // Check if collection already registered
    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: ecosystem_id.clone(),
        name,
        creator: info.sender.clone(),
        verified: false,
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;

    Ok(Response::new()
        .add_attribute("action", "register_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id))
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
    ECOSYSTEMS
        .load(deps.storage, ecosystem_id.clone())
        .map_err(|_| ContractError::EcosystemNotFound {
            id: ecosystem_id.clone(),
        })?;

    // Check if collection already registered
    if collections().has(deps.storage, collection_addr.clone()) {
        return Err(ContractError::CollectionAlreadyRegistered { address });
    }

    let collection = Collection {
        address: collection_addr.clone(),
        ecosystem_id: ecosystem_id.clone(),
        name,
        creator: creator_addr,
        verified: true, // Existing collections registered by admin are verified
        minter: None,
        marketplace: None,
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    collections().save(deps.storage, collection_addr.clone(), &collection)?;

    Ok(Response::new()
        .add_attribute("action", "register_existing_collection")
        .add_attribute("collection", collection_addr)
        .add_attribute("ecosystem_id", ecosystem_id))
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

    // Only collection creator or contract admin can update
    if !is_admin(&config, &info.sender) && collection.creator != info.sender {
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
        if !is_admin(&config, &info.sender) {
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

    // Only collection creator or contract admin can transfer ownership
    if !is_admin(&config, &info.sender) && collection.creator != info.sender {
        return Err(ContractError::NotCollectionCreator {});
    }

    collection.creator = new_creator_addr.clone();
    collection.updated_at = env.block.time.seconds();

    collections().save(deps.storage, collection_addr.clone(), &collection)?;

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

    // Only collection creator or contract admin can authorize minters
    if !is_admin(&config, &info.sender) && collection.creator != info.sender {
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
        authorized_by: info.sender,
        created_at: env.block.time.seconds(),
    };

    AUTHORIZED_MINTERS.save(deps.storage, key, &authorized_minter)?;

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

    // Only collection creator or contract admin can revoke minters
    if !is_admin(&config, &info.sender) && collection.creator != info.sender {
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
