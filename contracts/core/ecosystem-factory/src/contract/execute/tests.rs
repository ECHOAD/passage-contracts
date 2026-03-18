use super::super::execute;
use crate::{
    contract::{instantiate, query},
    error::ContractError,
    msg::{
        EcosystemCreationRequestResponse, EcosystemCreationRequestsResponse, ExecuteMsg,
        InstantiateMsg, QueryMsg, RegistryApprovalStatusResponse, RegistryQueryMsg,
    },
    state::{EcosystemCreationRequestStatus, PENDING_REQUEST_BY_ID, REQUESTS},
};
use cosmwasm_std::{
    coin, from_json,
    testing::{message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
    to_json_binary, Addr, ContractResult, OwnedDeps, SystemError, SystemResult, WasmQuery,
};
use cw_utils::PaymentError;

type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

const ADMIN: &str = "admin";
const CREATOR: &str = "creator";
const OTHER_CREATOR: &str = "other-creator";
const REGISTRY: &str = "registry";

fn addr(label: &str) -> Addr {
    MockApi::default().addr_make(label)
}

fn instantiate_contract(deps: &mut TestDeps) {
    instantiate(
        deps.as_mut(),
        mock_env(),
        message_info(&addr("deployer"), &[]),
        InstantiateMsg {
            admin: Some(addr(ADMIN).to_string()),
            operators: None,
            registry: addr(REGISTRY).to_string(),
            collection_factory_code_id: 17,
            collection_code_id: 23,
        },
    )
    .unwrap();
}

fn allow_registry_queries(deps: &mut TestDeps, can_create: bool) {
    let registry = addr(REGISTRY).to_string();

    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart { contract_addr, msg } if contract_addr == &registry => {
            let parsed: RegistryQueryMsg = from_json(msg).unwrap();
            match parsed {
                RegistryQueryMsg::CanCreateEcosystem { .. }
                | RegistryQueryMsg::IsCrossEcosystemAdmin { .. } => {
                    SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&RegistryApprovalStatusResponse {
                            approved: can_create,
                        })
                        .unwrap(),
                    ))
                }
            }
        }
        WasmQuery::Smart { .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: "unknown".to_string(),
        }),
        _ => SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "unsupported request".to_string(),
        }),
    });
}

fn submit_request(deps: &mut TestDeps, creator: &str, ecosystem_id: &str) {
    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addr(creator), &[]),
        ExecuteMsg::SubmitEcosystemCreationRequest {
            id: ecosystem_id.to_string(),
            name: format!("{} world", ecosystem_id),
            description: "desc".to_string(),
            image_urls: vec!["https://example.com/image.png".to_string()],
            animation_url: None,
            url: None,
        },
    )
    .unwrap();
}

#[test]
fn submit_request_rejects_attached_funds() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps);
    allow_registry_queries(&mut deps, true);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addr(CREATOR), &[coin(10, "upasg")]),
        ExecuteMsg::SubmitEcosystemCreationRequest {
            id: "eco-1".to_string(),
            name: "Eco".to_string(),
            description: "desc".to_string(),
            image_urls: vec!["https://example.com/image.png".to_string()],
            animation_url: None,
            url: None,
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Payment(PaymentError::NonPayable {}));
}

#[test]
fn creator_can_cancel_pending_request() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps);
    allow_registry_queries(&mut deps, true);

    submit_request(&mut deps, CREATOR, "eco-1");

    let res = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addr(CREATOR), &[]),
        ExecuteMsg::CancelEcosystemCreationRequest { request_id: 1 },
    )
    .unwrap();

    assert_eq!(res.attributes[0].value, "cancel_ecosystem_creation_request");
    assert_eq!(
        REQUESTS.load(deps.as_ref().storage, 1).unwrap().status,
        EcosystemCreationRequestStatus::Cancelled
    );
    assert!(PENDING_REQUEST_BY_ID
        .may_load(deps.as_ref().storage, "eco-1".to_string())
        .unwrap()
        .is_none());
}

#[test]
fn only_creator_can_cancel_request() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps);
    allow_registry_queries(&mut deps, true);

    submit_request(&mut deps, CREATOR, "eco-1");

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addr(OTHER_CREATOR), &[]),
        ExecuteMsg::CancelEcosystemCreationRequest { request_id: 1 },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::OnlyCreatorCanCancel { request_id: 1 });
}

#[test]
fn creator_requests_query_filters_by_creator() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps);
    allow_registry_queries(&mut deps, true);

    submit_request(&mut deps, CREATOR, "eco-1");
    submit_request(&mut deps, OTHER_CREATOR, "eco-2");

    let bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::CreatorEcosystemCreationRequests {
            creator: addr(CREATOR).to_string(),
            status: Some(EcosystemCreationRequestStatus::Pending),
            start_after: None,
            limit: None,
        },
    )
    .unwrap();

    let res: EcosystemCreationRequestsResponse = from_json(&bin).unwrap();
    assert_eq!(res.requests.len(), 1);
    assert_eq!(res.requests[0].id, "eco-1");

    let single = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::EcosystemCreationRequest { request_id: 1 },
    )
    .unwrap();
    let single: EcosystemCreationRequestResponse = from_json(&single).unwrap();
    assert_eq!(single.request.unwrap().creator, addr(CREATOR));
}
