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
