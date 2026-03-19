use super::*;
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    ContractResult, Empty, OwnedDeps, SystemError, SystemResult, WasmQuery,
};

fn instantiate_contract(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    admin: &str,
) {
    let admin_addr = deps.api.addr_make(admin);
    let treasury_addr = deps.api.addr_make("treasury");
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin_addr.as_str(), &[]),
        InstantiateMsg {
            admin: Some(admin_addr.to_string()),
            min_price: Uint128::new(1),
            trading_fee_bps: 250,
            fee_collector: treasury_addr.to_string(),
            registry: None,
            operators: None,
        },
    )
    .unwrap();
}

fn mock_collection_creator(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    collection: &str,
    creator: &str,
) {
    let collection = collection.to_string();
    let creator = creator.to_string();
    deps.querier.update_wasm(move |query| match query {
        WasmQuery::Smart { contract_addr, msg } if contract_addr == &collection => {
            let parsed: Pg721QueryMsg = from_json(msg).unwrap();
            match parsed {
                Pg721QueryMsg::CollectionInfo {} => SystemResult::Ok(ContractResult::Ok(
                    to_json_binary(&CollectionInfoResponse {
                        creator: creator.clone(),
                        description: "desc".to_string(),
                        image: "ipfs://image".to_string(),
                        external_link: None,
                        royalty_info: None,
                    })
                    .unwrap(),
                )),
            }
        }
        WasmQuery::Smart { contract_addr, .. } => SystemResult::Err(SystemError::NoSuchContract {
            addr: contract_addr.clone(),
        }),
        _ => SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "unsupported wasm query".to_string(),
        }),
    });
}

#[test]
fn pasg_payment_attributes_reference_canonical_surface() {
    let response = add_pasg_payment_attributes(Response::new(), "upasg", "marketplace_sale");

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_utility_query"
            && attr.value == crate::msg::CANONICAL_PASG_UTILITY_QUERY));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_native_denom"
            && attr.value == crate::msg::CANONICAL_PASG_NATIVE_DENOM));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_uses_native_utility" && attr.value == "true"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_fee_flow" && attr.value == "marketplace_sale"));
}

#[test]
fn pasg_payment_attributes_preserve_non_native_collection_denoms() {
    let response = add_pasg_payment_attributes(Response::new(), "uion", "marketplace_sale");

    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_settlement_denom" && attr.value == "uion"));
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "pasg_uses_native_utility" && attr.value == "false"));
}

#[test]
fn admin_can_register_collection_directly() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps, "admin");
    let admin = deps.api.addr_make("admin");
    let collection = deps.api.addr_make("collection");

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(admin.as_str(), &[]),
        ExecuteMsg::RegisterCollection {
            collection: collection.to_string(),
            denom: "upasg".to_string(),
        },
    )
    .unwrap();

    let config = COLLECTION_CONFIGS
        .load(deps.as_ref().storage, collection)
        .unwrap();
    assert_eq!(config.denom, "upasg");
    assert!(config.active);
}

#[test]
fn registration_request_requires_collection_owner() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps, "admin");
    let collection = deps.api.addr_make("collection");
    let creator = deps.api.addr_make("creator");
    let intruder = deps.api.addr_make("intruder");
    mock_collection_creator(&mut deps, collection.as_str(), creator.as_str());

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(intruder.as_str(), &[]),
        ExecuteMsg::SubmitCollectionRegistrationRequest {
            collection: collection.to_string(),
            denom: "upasg".to_string(),
            note: None,
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::NotCollectionOwner {
            collection: collection.to_string(),
        }
    );
}

#[test]
fn approving_registration_request_registers_collection() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps, "admin");
    let admin = deps.api.addr_make("admin");
    let creator = deps.api.addr_make("creator");
    let collection = deps.api.addr_make("collection");
    mock_collection_creator(&mut deps, collection.as_str(), creator.as_str());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(creator.as_str(), &[]),
        ExecuteMsg::SubmitCollectionRegistrationRequest {
            collection: collection.to_string(),
            denom: "upasg".to_string(),
            note: Some("please add".to_string()),
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(admin.as_str(), &[]),
        ExecuteMsg::ResolveCollectionRegistrationRequest {
            collection: collection.to_string(),
            approved: true,
            denom: Some("uion".to_string()),
            note: Some("approved".to_string()),
        },
    )
    .unwrap();

    let request = COLLECTION_REGISTRATION_REQUESTS
        .load(deps.as_ref().storage, collection.clone())
        .unwrap();
    assert_eq!(request.status, CollectionRequestStatus::Approved);
    assert_eq!(request.reviewed_by, Some(admin.clone()));

    let config = COLLECTION_CONFIGS
        .load(deps.as_ref().storage, collection)
        .unwrap();
    assert_eq!(config.denom, "uion");
    assert_eq!(config.registered_by, admin);
}

#[test]
fn approving_update_request_applies_collection_changes() {
    let mut deps = mock_dependencies();
    instantiate_contract(&mut deps, "admin");
    let admin = deps.api.addr_make("admin");
    let creator = deps.api.addr_make("creator");
    let collection = deps.api.addr_make("collection");
    mock_collection_creator(&mut deps, collection.as_str(), creator.as_str());

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(admin.as_str(), &[]),
        ExecuteMsg::RegisterCollection {
            collection: collection.to_string(),
            denom: "upasg".to_string(),
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(creator.as_str(), &[]),
        ExecuteMsg::SubmitCollectionUpdateRequest {
            collection: collection.to_string(),
            active: Some(false),
            denom: Some("uion".to_string()),
            note: Some("switch denom".to_string()),
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(admin.as_str(), &[]),
        ExecuteMsg::ResolveCollectionUpdateRequest {
            collection: collection.to_string(),
            approved: true,
            active: None,
            denom: None,
            note: Some("done".to_string()),
        },
    )
    .unwrap();

    let request = COLLECTION_UPDATE_REQUESTS
        .load(deps.as_ref().storage, collection.clone())
        .unwrap();
    assert_eq!(request.status, CollectionRequestStatus::Approved);

    let config = COLLECTION_CONFIGS
        .load(deps.as_ref().storage, collection)
        .unwrap();
    assert!(!config.active);
    assert_eq!(config.denom, "uion");
}
