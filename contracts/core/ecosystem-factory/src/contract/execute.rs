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
            paused,
        } => execute_update_config(deps, info, admin, operators, registry, paused),
        ExecuteMsg::SubmitEcosystemCreationRequest {
            id,
            name,
            ecosystem_type,
            collection_factory,
            detail,
            image_urls,
            animation_url,
            url,
        } => execute_submit_request(
            deps,
            env,
            info,
            id,
            name,
            ecosystem_type,
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
        } => execute_resolve_request(deps, env, info, request_id, approved, note),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
    operators: Option<Vec<String>>,
    registry: Option<String>,
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

    if let Some(new_registry) = registry {
        config.registry = deps.api.addr_validate(&new_registry)?;
    }

    if let Some(new_paused) = paused {
        config.paused = new_paused;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("registry", config.registry)
        .add_attribute("paused", config.paused.to_string()))
}

#[allow(clippy::too_many_arguments)]
fn execute_submit_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    ecosystem_type: Option<EcosystemType>,
    collection_factory: Option<String>,
    detail: String,
    image_urls: Vec<String>,
    animation_url: Option<String>,
    url: Option<String>,
) -> Result<Response, ContractError> {
    validate_request_input(&id, &name, &detail, &image_urls)?;

    if PENDING_REQUEST_BY_ID.has(deps.storage, id.clone()) {
        return Err(ContractError::RequestAlreadyPending { id });
    }

    let request_id = NEXT_REQUEST_ID.load(deps.storage)?;
    NEXT_REQUEST_ID.save(deps.storage, &(request_id + 1))?;

    let collection_factory = collection_factory
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?;

    let request = EcosystemCreationRequest {
        request_id,
        creator: info.sender.clone(),
        id: id.clone(),
        name,
        ecosystem_type: ecosystem_type.unwrap_or_default(),
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

    REQUESTS.save(deps.storage, request_id, &request)?;
    PENDING_REQUEST_BY_ID.save(deps.storage, id.clone(), &request_id)?;

    Ok(Response::new()
        .add_attribute("action", "submit_ecosystem_creation_request")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("ecosystem_id", id)
        .add_attribute("creator", info.sender))
}

fn execute_resolve_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    request_id: u64,
    approved: bool,
    note: Option<String>,
) -> Result<Response, ContractError> {
    if request_id == 0 {
        return Err(ContractError::InvalidRequestId {});
    }

    let config = CONFIG.load(deps.storage)?;
    if !is_admin_or_operator(&config, &info.sender) {
        let cross_admin_status: RegistryApprovalStatusResponse = deps
            .querier
            .query_wasm_smart(
                config.registry.to_string(),
                &RegistryQueryMsg::IsCrossEcosystemAdmin {
                    address: info.sender.to_string(),
                },
            )
            .map_err(|_| ContractError::Unauthorized {})?;

        if !cross_admin_status.approved {
            return Err(ContractError::Unauthorized {});
        }
    }

    let mut request = REQUESTS
        .may_load(deps.storage, request_id)?
        .ok_or(ContractError::RequestNotFound { request_id })?;

    if request.status != EcosystemCreationRequestStatus::Pending {
        return Err(ContractError::RequestAlreadyResolved { request_id });
    }

    request.status = if approved {
        EcosystemCreationRequestStatus::Approved
    } else {
        EcosystemCreationRequestStatus::Rejected
    };
    request.reviewed_at = Some(env.block.time.seconds());
    request.reviewed_by = Some(info.sender.clone());
    request.review_note = note;
    REQUESTS.save(deps.storage, request_id, &request)?;
    PENDING_REQUEST_BY_ID.remove(deps.storage, request.id.clone());

    let mut res = Response::new()
        .add_attribute("action", "resolve_ecosystem_creation_request")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("ecosystem_id", request.id.clone())
        .add_attribute("creator", request.creator.clone())
        .add_attribute("approved", approved.to_string());

    if approved {
        let register_msg = WasmMsg::Execute {
            contract_addr: config.registry.to_string(),
            msg: to_json_binary(&RegistryExecuteMsg::RegisterEcosystemFromFactory {
                id: request.id,
                name: request.name,
                creator: request.creator.to_string(),
                ecosystem_type: Some(request.ecosystem_type),
                collection_factory: request.collection_factory.map(|a| a.to_string()),
                detail: request.detail,
                image_urls: request.image_urls,
                animation_url: request.animation_url,
                url: request.url,
            })?,
            funds: vec![],
        };
        res = res.add_message(register_msg);
    }

    Ok(res)
}
