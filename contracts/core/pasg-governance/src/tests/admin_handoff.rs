use crate::contract::{execute, instantiate, query};
use crate::error::ContractError;
use crate::msg::{
    AdminAction, ExecuteMsg, InstantiateMsg, ProposalAction, QueryMsg, RatifiedAdminActionResponse,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Uint128};

struct TestCtx {
    deps: cosmwasm_std::OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
    >,
    alice: String,
    multisig: String,
}

fn instantiate_contract() -> TestCtx {
    let mut deps = mock_dependencies();
    let admin_multisig = deps.api.addr_make("multisig").to_string();
    let creator = deps.api.addr_make("creator");
    let alice = deps.api.addr_make("alice").to_string();

    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(creator.as_str(), &[]),
        InstantiateMsg {
            admin_multisig: admin_multisig.clone(),
            native_denom: "upasg".to_string(),
            proposal_deposit: Uint128::new(100),
            voting_period_secs: 3600,
            quorum_bps: 5_000,
            pass_bps: 6_000,
        },
    )
    .unwrap();

    TestCtx {
        deps,
        alice,
        multisig: admin_multisig,
    }
}

#[test]
fn off_scope_admin_action_is_rejected() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(150, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "off scope".to_string(),
            description: None,
            action: ProposalAction::StageAdminAction {
                action: AdminAction::StreamingBillingUpdateConfig {
                    contract_addr: "https://api.passage.example/offchain".to_string(),
                    backend_operator: Some("passage1ops...".to_string()),
                    fiat_oracle: None,
                    stripe_webhook_validator: None,
                    paused: None,
                },
            },
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::OffChainOperationForbidden {
            target: "https://api.passage.example/offchain".to_string(),
        }
    );
}

#[test]
fn registry_staking_validator_handoff_is_metadata_only() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    let action = AdminAction::RegistryUpsertStakingValidator {
        contract_addr: "passage1registry000000000000000000000000".to_string(),
        operator_address: "passagevaloper1alpha".to_string(),
        moniker: "Passage Alpha".to_string(),
        website: Some("https://alpha.passage.io".to_string()),
        active: true,
    };

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "ratify validator metadata".to_string(),
            description: Some("stage registry staking metadata".to_string()),
            action: ProposalAction::StageAdminAction {
                action: action.clone(),
            },
        },
    )
    .unwrap();

    let response = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info("executor", &[]),
        ExecuteMsg::ExecuteProposal { proposal_id: 1 },
    )
    .unwrap();

    assert!(response.messages.is_empty());
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "proposal_action" && attr.value == "stage_admin_action"));

    let ratified: RatifiedAdminActionResponse = from_json(
        query(
            ctx.deps.as_ref(),
            mock_env(),
            QueryMsg::RatifiedAdminAction { proposal_id: 1 },
        )
        .unwrap(),
    )
    .unwrap();

    let action_record = ratified.action.unwrap();
    assert_eq!(action_record.action, action);
    assert!(!action_record.payload_hash.is_empty());
}

#[test]
fn ratified_admin_action_query_matches_multisig_payload() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    let action = AdminAction::MarketplaceV3UpdateConfig {
        contract_addr: "passage1marketplace0000000000000000000000000".to_string(),
        admin: None,
        denom: Some("upasg".to_string()),
        min_price: Some(Uint128::new(250)),
        trading_fee_bps: Some(250),
        max_trading_fee_bps: None,
        fee_collector: Some("passage1treasury000000000000000000000000".to_string()),
        registry: Some("passage1registry000000000000000000000000".to_string()),
        operators: None,
        paused: Some(false),
        require_registration: Some(true),
    };

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "ratify marketplace config".to_string(),
            description: Some("handoff to multisig".to_string()),
            action: ProposalAction::StageAdminAction {
                action: action.clone(),
            },
        },
    )
    .unwrap();

    let response = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info("executor", &[]),
        ExecuteMsg::ExecuteProposal { proposal_id: 1 },
    )
    .unwrap();
    assert!(response
        .attributes
        .iter()
        .any(|attr| attr.key == "ratified_admin_action" && attr.value == "1"));

    let ratified: RatifiedAdminActionResponse = from_json(
        query(
            ctx.deps.as_ref(),
            mock_env(),
            QueryMsg::RatifiedAdminAction { proposal_id: 1 },
        )
        .unwrap(),
    )
    .unwrap();

    let action_record = ratified.action.unwrap();
    assert_eq!(action_record.admin_multisig.to_string(), ctx.multisig);
    assert_eq!(action_record.action, action);
    assert!(!action_record.payload_hash.is_empty());
}
