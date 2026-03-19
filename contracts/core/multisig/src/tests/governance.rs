use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi};
use cosmwasm_std::{coin, to_json_binary, Binary, CosmosMsg, Uint128, WasmMsg};

use crate::contract::{execute, instantiate, query};
use crate::error::ContractError;
use crate::msg::{
    CanExecuteResponse, DelegationResponse, ExecuteMsg, InstantiateMsg, QueryMsg, VoteResponse,
    VotingPowerResponse,
};
use crate::state::{ProposalAction, Vote};

struct TestAddrs {
    deployer: cosmwasm_std::Addr,
    alice: cosmwasm_std::Addr,
    bob: cosmwasm_std::Addr,
    carol: cosmwasm_std::Addr,
    mallory: cosmwasm_std::Addr,
    registry: cosmwasm_std::Addr,
}

fn instantiate_default(deps: cosmwasm_std::DepsMut) -> TestAddrs {
    let api = MockApi::default();
    let addrs = TestAddrs {
        deployer: api.addr_make("deployer"),
        alice: api.addr_make("alice"),
        bob: api.addr_make("bob"),
        carol: api.addr_make("carol"),
        mallory: api.addr_make("mallory"),
        registry: api.addr_make("registry"),
    };
    instantiate(
        deps,
        mock_env(),
        message_info(&addrs.deployer, &[]),
        InstantiateMsg {
            pasg_denom: "upasg".into(),
            proposal_threshold: 100u128.into(),
            quorum_bps: 5_000,
            approval_bps: 6_000,
            max_voting_period_secs: 3600,
            allowed_execute_contracts: vec![addrs.registry.to_string()],
        },
    )
    .unwrap();
    addrs
}

fn deposit(
    deps: cosmwasm_std::DepsMut,
    address: &cosmwasm_std::Addr,
    amount: u128,
) -> cosmwasm_std::Response {
    execute(
        deps,
        mock_env(),
        message_info(address, &[coin(amount, "upasg")]),
        ExecuteMsg::DepositVotingPower {},
    )
    .unwrap()
}

#[test]
fn pasg_holder_can_propose_vote_and_execute_allowed_action() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 150);
    deposit(deps.as_mut(), &addrs.bob, 150);

    let propose = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Update registry".into(),
            description: Some("Protocol-scoped action".into()),
            actions: vec![ProposalAction::WasmExecute {
                contract_addr: addrs.registry.to_string(),
                msg: to_json_binary(&"update_config").unwrap(),
            }],
        },
    )
    .unwrap();
    assert_eq!(propose.attributes[1].value, "1");

    let vote = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.bob, &[]),
        ExecuteMsg::Vote {
            proposal_id: 1,
            vote: Vote::Approve,
        },
    )
    .unwrap();
    assert_eq!(vote.attributes.last().unwrap().value, "Passed");

    let execute_response = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.carol, &[]),
        ExecuteMsg::Execute { proposal_id: 1 },
    )
    .unwrap();
    assert_eq!(execute_response.messages.len(), 1);
    match &execute_response.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, &addrs.registry.to_string());
        }
        other => panic!("unexpected message: {:?}", other),
    }

    let can_execute: CanExecuteResponse = cosmwasm_std::from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::CanExecute { proposal_id: 1 },
        )
        .unwrap(),
    )
    .unwrap();
    assert!(!can_execute.executable);
}

#[test]
fn proposer_must_meet_pasg_threshold() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 50);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Too small".into(),
            description: None,
            actions: vec![ProposalAction::UpdateGovernanceConfig {
                proposal_threshold: Some(150u128.into()),
                quorum_bps: None,
                approval_bps: None,
                max_voting_period_secs: None,
            }],
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::InsufficientProposalPower {});
}

#[test]
fn rejects_out_of_scope_execution_targets() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 150);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Hack analytics".into(),
            description: None,
            actions: vec![ProposalAction::WasmExecute {
                contract_addr: addrs.mallory.to_string(),
                msg: Binary::from(vec![1, 2, 3]),
            }],
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::TargetNotAllowed {
            contract_addr: addrs.mallory.to_string(),
        }
    );
}

#[test]
fn delegation_is_weighted_and_queryable() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 150);
    deposit(deps.as_mut(), &addrs.bob, 100);

    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::DelegateVotingPower {
            delegate: addrs.bob.to_string(),
        },
    )
    .unwrap();

    let bob_power: VotingPowerResponse = cosmwasm_std::from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::VotingPower {
                address: addrs.bob.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(bob_power.effective_voting_power, Uint128::new(250));

    let delegation: DelegationResponse = cosmwasm_std::from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Delegation {
                address: addrs.alice.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(delegation.delegation.unwrap().delegate, addrs.bob);

    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.bob, &[]),
        ExecuteMsg::Propose {
            title: "Tighten quorum".into(),
            description: None,
            actions: vec![ProposalAction::UpdateGovernanceConfig {
                proposal_threshold: None,
                quorum_bps: Some(6_000),
                approval_bps: None,
                max_voting_period_secs: None,
            }],
        },
    )
    .unwrap();

    let vote = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.bob, &[]),
        ExecuteMsg::Vote {
            proposal_id: 1,
            vote: Vote::Approve,
        },
    )
    .unwrap();
    assert_eq!(vote.attributes[4].value, "250");

    let ballot: VoteResponse = cosmwasm_std::from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Vote {
                proposal_id: 1,
                voter: addrs.bob.to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(ballot.ballot.unwrap().weight, Uint128::new(250));
}

#[test]
fn insufficient_quorum_rejects_on_close() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 150);
    deposit(deps.as_mut(), &addrs.bob, 100);

    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Raise threshold".into(),
            description: None,
            actions: vec![ProposalAction::UpdateGovernanceConfig {
                proposal_threshold: Some(200u128.into()),
                quorum_bps: None,
                approval_bps: None,
                max_voting_period_secs: None,
            }],
        },
    )
    .unwrap();

    let mut expired_env = mock_env();
    expired_env.block.time = expired_env.block.time.plus_seconds(7200);

    let err = execute(
        deps.as_mut(),
        expired_env.clone(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Execute { proposal_id: 1 },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::ProposalNotPassed {});

    execute(
        deps.as_mut(),
        expired_env,
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Close { proposal_id: 1 },
    )
    .unwrap();
}

#[test]
fn balances_are_locked_while_proposal_is_open() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());
    deposit(deps.as_mut(), &addrs.alice, 150);
    deposit(deps.as_mut(), &addrs.bob, 100);

    execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Open vote".into(),
            description: None,
            actions: vec![ProposalAction::UpdateExecutionTargets {
                add: vec![addrs.mallory.to_string()],
                remove: vec![],
            }],
        },
    )
    .unwrap();

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.bob, &[]),
        ExecuteMsg::WithdrawVotingPower {
            amount: 10u128.into(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::GovernancePowerLocked {});
}
