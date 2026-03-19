use crate::contract::{execute, instantiate, query};
use crate::msg::{
    AdminAction, DepositResponse, ExecuteMsg, InstantiateMsg, PasgUtilityConfigResponse, ProposalAction,
    ProposalResponse, QueryMsg, RatifiedAdminActionResponse,
};
use crate::state::ProposalStatus;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin, coins, from_json, Addr, Uint128};

struct TestCtx {
    deps: cosmwasm_std::OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
    >,
    admin_multisig: Addr,
    alice: Addr,
}

fn instantiate_contract() -> TestCtx {
    let mut deps = mock_dependencies();
    let admin_multisig = deps.api.addr_make("multisig");
    let alice = deps.api.addr_make("alice");
    let creator = deps.api.addr_make("creator");

    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(creator.as_str(), &[]),
        InstantiateMsg {
            admin_multisig: admin_multisig.to_string(),
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
        admin_multisig,
        alice,
    }
}

#[test]
fn proposal_lifecycle_executes_param_change() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &coins(150, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &[]),
        ExecuteMsg::Propose {
            title: "Tune utility".to_string(),
            description: Some("adjust config".to_string()),
            action: ProposalAction::SetPasgUtilityConfig {
                points_per_pasg: Uint128::new(250),
                max_fiat_report_age_secs: 900,
                max_session_duration_secs: 7200,
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
        .any(|attr| attr.key == "proposal_action" && attr.value == "set_pasg_utility_config"));

    let config: PasgUtilityConfigResponse = from_json(
        query(ctx.deps.as_ref(), mock_env(), QueryMsg::PasgUtilityConfig {}).unwrap(),
    )
    .unwrap();
    assert_eq!(config.config.points_per_pasg, Uint128::new(250));
    assert_eq!(config.config.updated_by_proposal, Some(1));

    let proposal: ProposalResponse = from_json(
        query(ctx.deps.as_ref(), mock_env(), QueryMsg::Proposal { proposal_id: 1 }).unwrap(),
    )
    .unwrap();
    assert_eq!(proposal.computed_status, ProposalStatus::Executed);
}

#[test]
fn stage_admin_action_records_handoff_without_dispatch() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &[]),
        ExecuteMsg::Propose {
            title: "Stage admin change".to_string(),
            description: None,
            action: ProposalAction::StageAdminAction {
                action: AdminAction::StreamingBillingUpdateConfig {
                    contract_addr: "passage1billing000000000000000000000000000".to_string(),
                    backend_operator: Some("passage1ops0000000000000000000000000000000".to_string()),
                    fiat_oracle: None,
                    stripe_webhook_validator: None,
                    paused: Some(false),
                },
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

    let staged: RatifiedAdminActionResponse = from_json(
        query(
            ctx.deps.as_ref(),
            mock_env(),
            QueryMsg::RatifiedAdminAction { proposal_id: 1 },
        )
        .unwrap(),
    )
    .unwrap();
    assert!(staged.action.is_some());
    assert_eq!(staged.action.unwrap().admin_multisig, ctx.admin_multisig);
}

#[test]
fn non_depositor_cannot_propose() {
    let mut ctx = instantiate_contract();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &[]),
        ExecuteMsg::Propose {
            title: "No deposit".to_string(),
            description: None,
            action: ProposalAction::SetPasgUtilityConfig {
                points_per_pasg: Uint128::new(1),
                max_fiat_report_age_secs: 1,
                max_session_duration_secs: 1,
            },
        },
    )
    .unwrap_err();

    assert!(format!("{err}").contains("insufficient deposited voting power"));
}

#[test]
fn wrong_denom_deposit_is_rejected() {
    let mut ctx = instantiate_contract();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &[coin(100, "uatom")]),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap_err();

    assert!(format!("{err}").contains("wrong deposit denom"));
}

#[test]
fn deposit_query_reports_balance() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(ctx.alice.as_str(), &coins(123, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    let deposit: DepositResponse = from_json(
        query(
            ctx.deps.as_ref(),
            mock_env(),
            QueryMsg::Deposit {
                address: ctx.alice.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(deposit.deposited, Uint128::new(123));
}




