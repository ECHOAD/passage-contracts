use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{AdminAction, ExecuteMsg, InstantiateMsg, ProposalAction};
use crate::state::Vote;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Timestamp, Uint128};

struct TestCtx {
    deps: cosmwasm_std::OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
    >,
    alice: String,
    bob: String,
    carol: String,
}

fn instantiate_contract() -> TestCtx {
    let mut deps = mock_dependencies();
    let admin_multisig = deps.api.addr_make("multisig");
    let creator = deps.api.addr_make("creator");
    let alice = deps.api.addr_make("alice").to_string();
    let bob = deps.api.addr_make("bob").to_string();
    let carol = deps.api.addr_make("carol").to_string();

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
        alice,
        bob,
        carol,
    }
}

#[test]
fn delegation_counts_once_for_passed_proposal() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(100, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &coins(120, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Delegate {
            delegate: ctx.bob.clone(),
        },
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &[]),
        ExecuteMsg::Propose {
            title: "Delegated pass".to_string(),
            description: None,
            action: ProposalAction::SetPasgUtilityConfig {
                points_per_pasg: Uint128::new(500),
                max_fiat_report_age_secs: 600,
                max_session_duration_secs: 3600,
            },
        },
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Vote {
            proposal_id: 1,
            vote: Vote::Approve,
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InsufficientVotingPower {
            required: Uint128::new(1),
            actual: Uint128::zero(),
        }
    );

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info("executor", &[]),
        ExecuteMsg::ExecuteProposal { proposal_id: 1 },
    )
    .unwrap();
}

#[test]
fn delegation_cycle_is_rejected() {
    let mut ctx = instantiate_contract();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Delegate {
            delegate: ctx.bob.clone(),
        },
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &[]),
        ExecuteMsg::Delegate {
            delegate: ctx.alice.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::CycleInDelegation {});
}

#[test]
fn below_quorum_cannot_execute() {
    let mut ctx = instantiate_contract();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(100, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.carol, &coins(300, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "Low quorum".to_string(),
            description: None,
            action: ProposalAction::SetPasgUtilityConfig {
                points_per_pasg: Uint128::new(10),
                max_fiat_report_age_secs: 10,
                max_session_duration_secs: 10,
            },
        },
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info("executor", &[]),
        ExecuteMsg::ExecuteProposal { proposal_id: 1 },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::QuorumNotMet {});
}

#[test]
fn below_pass_threshold_cannot_execute() {
    let mut ctx = instantiate_contract();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(100, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "Low pass threshold".to_string(),
            description: None,
            action: ProposalAction::SetPasgUtilityConfig {
                points_per_pasg: Uint128::new(10),
                max_fiat_report_age_secs: 10,
                max_session_duration_secs: 10,
            },
        },
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &[]),
        ExecuteMsg::Vote {
            proposal_id: 1,
            vote: Vote::Reject,
        },
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info("executor", &[]),
        ExecuteMsg::ExecuteProposal { proposal_id: 1 },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::PassThresholdNotMet {});
}

#[test]
fn withdraw_rejects_locked_balance_until_close() {
    let mut ctx = instantiate_contract();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &coins(100, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();
    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.bob, &coins(200, "upasg")),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::Propose {
            title: "Lock withdraw".to_string(),
            description: None,
            action: ProposalAction::StageAdminAction {
                action: AdminAction::StreamingBillingUpdateConfig {
                    contract_addr: ctx.carol.clone(),
                    backend_operator: None,
                    fiat_oracle: None,
                    stripe_webhook_validator: None,
                    paused: Some(false),
                },
            },
        },
    )
    .unwrap();

    let err = execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::WithdrawVotingPower {
            amount: Uint128::new(50),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InsufficientUnlockedBalance {
            requested: Uint128::new(50),
            unlocked: Uint128::zero(),
        }
    );

    let mut expired_env = mock_env();
    expired_env.block.time = Timestamp::from_seconds(expired_env.block.time.seconds() + 7200);
    execute(
        ctx.deps.as_mut(),
        expired_env,
        mock_info("closer", &[]),
        ExecuteMsg::Close { proposal_id: 1 },
    )
    .unwrap();

    execute(
        ctx.deps.as_mut(),
        mock_env(),
        mock_info(&ctx.alice, &[]),
        ExecuteMsg::WithdrawVotingPower {
            amount: Uint128::new(50),
        },
    )
    .unwrap();
}




