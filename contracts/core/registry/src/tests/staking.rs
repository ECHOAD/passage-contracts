use cosmwasm_std::from_json;
use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi};
use cosmwasm_std::to_json_binary;

use crate::contract::{execute, query};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, StakingValidatorResponse, StakingValidatorsResponse};
use crate::state::{Config, CONFIG};

#[test]
fn staking_validator_queries_return_metadata_only() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let api = MockApi::default();
    let registry_admin = api.addr_make("registry_admin");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: registry_admin.clone(),
                operators: vec![],
                recovery_council: vec![],
                ecosystem_factory: None,
                paused: false,
            },
        )
        .unwrap();

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&registry_admin, &[]),
        ExecuteMsg::UpsertStakingValidator {
            operator_address: "passagevaloper1alpha".to_string(),
            moniker: "Passage Alpha".to_string(),
            website: Some("https://alpha.passage.io".to_string()),
            active: true,
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&registry_admin, &[]),
        ExecuteMsg::UpsertStakingValidator {
            operator_address: "passagevaloper1beta".to_string(),
            moniker: "Passage Beta".to_string(),
            website: None,
            active: false,
        },
    )
    .unwrap();

    let response: StakingValidatorsResponse = from_json(
        query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::StakingValidators {
                active_only: Some(true),
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.validators.len(), 1);
    assert_eq!(
        response.validators[0].operator_address,
        "passagevaloper1alpha"
    );
    assert_eq!(response.validators[0].moniker, "Passage Alpha");
    assert!(response.validators[0].active);

    let json = String::from_utf8(to_json_binary(&response).unwrap().to_vec()).unwrap();
    assert!(!json.contains("delegated_balance"));
    assert!(!json.contains("unbond_queue"));
    assert!(!json.contains("reward_per_token"));

    let single: StakingValidatorResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::StakingValidator {
                operator_address: "passagevaloper1beta".to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        single.validator.unwrap().operator_address,
        "passagevaloper1beta"
    );
}

#[test]
fn staking_validator_updates_require_registry_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let api = MockApi::default();
    let registry_admin = api.addr_make("registry_admin");
    let operator = api.addr_make("operator");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: registry_admin,
                operators: vec![operator.clone()],
                recovery_council: vec![],
                ecosystem_factory: None,
                paused: false,
            },
        )
        .unwrap();

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&operator, &[]),
        ExecuteMsg::UpsertStakingValidator {
            operator_address: "passagevaloper1alpha".to_string(),
            moniker: "Passage Alpha".to_string(),
            website: None,
            active: true,
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}
