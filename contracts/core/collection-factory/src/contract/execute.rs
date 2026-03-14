use super::helpers::*;
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.paused && !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::ContractPaused {});
    }

    match msg {
        ExecuteMsg::UpdateConfig {
            admin,
            operators,
            registry,
            ecosystem_id,
            collection_code_id,
            enforce_local_allowlist,
            paused,
        } => execute_update_config(
            deps,
            info,
            admin,
            operators,
            registry,
            ecosystem_id,
            collection_code_id,
            enforce_local_allowlist,
            paused,
        ),
        ExecuteMsg::ApproveCreator { creator } => execute_approve_creator(deps, info, creator),
        ExecuteMsg::RevokeCreator { creator } => execute_revoke_creator(deps, info, creator),
        ExecuteMsg::CreateCollection {
            name,
            symbol,
            minter,
            nft_type,
            collection_info,
            label,
        } => execute_create_collection(
            deps,
            env,
            info,
            name,
            symbol,
            minter,
            nft_type,
            collection_info,
            label,
        ),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    operators: Option<Vec<String>>,
    registry: Option<String>,
    ecosystem_id: Option<String>,
    collection_code_id: Option<u64>,
    enforce_local_allowlist: Option<bool>,
    paused: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if !is_admin(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(admin) = admin {
        config.admin = deps.api.addr_validate(&admin)?;
        APPROVED_CREATORS.save(deps.storage, config.admin.clone(), &true)?;
    }

    if let Some(operators) = operators {
        config.operators = operators
            .iter()
            .map(|o| deps.api.addr_validate(o))
            .collect::<StdResult<Vec<Addr>>>()?;
    }

    if let Some(registry) = registry {
        config.registry = deps.api.addr_validate(&registry)?;
    }

    if let Some(ecosystem_id) = ecosystem_id {
        if ecosystem_id.trim().is_empty() {
            return Err(ContractError::EmptyEcosystemId {});
        }
        config.ecosystem_id = ecosystem_id;
    }

    if let Some(code_id) = collection_code_id {
        if code_id == 0 {
            return Err(ContractError::InvalidCodeId {});
        }
        config.collection_code_id = code_id;
    }

    if let Some(paused) = paused {
        config.paused = paused;
    }

    if let Some(enforce_local_allowlist) = enforce_local_allowlist {
        config.enforce_local_allowlist = enforce_local_allowlist;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("registry", config.registry)
        .add_attribute("ecosystem_id", config.ecosystem_id)
        .add_attribute("collection_code_id", config.collection_code_id.to_string())
        .add_attribute(
            "enforce_local_allowlist",
            config.enforce_local_allowlist.to_string(),
        )
        .add_attribute("paused", config.paused.to_string()))
}

fn execute_approve_creator(
    deps: DepsMut,
    info: MessageInfo,
    creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    APPROVED_CREATORS.save(deps.storage, creator_addr.clone(), &true)?;

    Ok(Response::new()
        .add_attribute("action", "approve_creator")
        .add_attribute("creator", creator_addr))
}

fn execute_revoke_creator(
    deps: DepsMut,
    info: MessageInfo,
    creator: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let creator_addr = deps.api.addr_validate(&creator)?;
    APPROVED_CREATORS.remove(deps.storage, creator_addr.clone());

    Ok(Response::new()
        .add_attribute("action", "revoke_creator")
        .add_attribute("creator", creator_addr))
}

#[allow(clippy::too_many_arguments)]
fn execute_create_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    symbol: String,
    minter: String,
    nft_type: NftType,
    collection_info: CollectionInfoInput,
    label: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let approved_local = APPROVED_CREATORS.has(deps.storage, info.sender.clone());
    if config.enforce_local_allowlist && !approved_local {
        return Err(ContractError::CreatorNotApproved {});
    }

    let registry_approval: RegistryApprovalStatusResponse = deps
        .querier
        .query_wasm_smart(
            config.registry.to_string(),
            &RegistryQueryMsg::CanCreateCollectionInEcosystem {
                ecosystem_id: config.ecosystem_id.clone(),
                creator: info.sender.to_string(),
            },
        )
        .map_err(|_| ContractError::CreatorNotApprovedInEcosystem {})?;

    if !registry_approval.approved {
        return Err(ContractError::CreatorNotApprovedInEcosystem {});
    }

    let ecosystem_response: RegistryEcosystemResponse = deps
        .querier
        .query_wasm_smart(
            config.registry.to_string(),
            &RegistryQueryMsg::Ecosystem {
                id: config.ecosystem_id.clone(),
            },
        )
        .map_err(|_| ContractError::EcosystemNotFound {})?;

    let ecosystem = ecosystem_response
        .ecosystem
        .ok_or(ContractError::EcosystemNotFound {})?;
    let ecosystem_admin = deps.api.addr_validate(&ecosystem.admin)?;

    let cross_admin_status: RegistryApprovalStatusResponse = deps
        .querier
        .query_wasm_smart(
            config.registry.to_string(),
            &RegistryQueryMsg::IsCrossEcosystemAdmin {
                address: info.sender.to_string(),
            },
        )
        .map_err(|_| ContractError::Unauthorized {})?;

    let effective_creator = if cross_admin_status.approved {
        ecosystem_admin
    } else {
        info.sender.clone()
    };

    validate_create_collection_input(&name, &symbol, &minter, &collection_info)?;

    let minter_addr = deps.api.addr_validate(&minter)?;
    let CollectionInfoInput {
        description,
        image,
        external_link,
        royalty_info,
    } = collection_info;

    let royalty_info = royalty_info.map(|r| {
        let payment_address = deps.api.addr_validate(&r.payment_address)?;
        Ok::<Pg721RoyaltyInfoResponse, ContractError>(Pg721RoyaltyInfoResponse {
            payment_address: payment_address.to_string(),
            share: r.share,
        })
    });
    let royalty_info = royalty_info.transpose()?;

    let instantiate_msg = Pg721InstantiateMsg {
        name: name.clone(),
        symbol: symbol.clone(),
        minter: minter_addr.to_string(),
        nft_type: nft_type.clone(),
        collection_info: Pg721CollectionInfo {
            creator: effective_creator.to_string(),
            description,
            image,
            external_link,
            royalty_info,
        },
    };

    let request_id = NEXT_REQUEST_ID.load(deps.storage)?;
    NEXT_REQUEST_ID.save(deps.storage, &(request_id + 1))?;

    let pending = PendingCreation {
        request_id,
        creator: effective_creator.clone(),
        ecosystem_id: config.ecosystem_id.clone(),
        name: name.clone(),
        symbol: symbol.clone(),
        nft_type: nft_type.clone(),
        requested_at: env.block.time.seconds(),
    };
    PENDING_CREATIONS.save(deps.storage, request_id, &pending)?;

    let label = label.unwrap_or_else(|| format!("collection-{}-{}", symbol, env.block.height));
    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(effective_creator.to_string()),
        code_id: config.collection_code_id,
        msg: to_json_binary(&instantiate_msg)?,
        funds: vec![],
        label,
    };
    let submsg = SubMsg::reply_on_success(instantiate_msg, request_id);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("action", "create_collection")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("requested_by", info.sender)
        .add_attribute("creator", effective_creator)
        .add_attribute("ecosystem_id", config.ecosystem_id)
        .add_attribute("name", name)
        .add_attribute("symbol", symbol)
        .add_attribute("nft_type", nft_type.to_string()))
}
