use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi};
use cosmwasm_std::{coins, BankMsg, CosmosMsg};

use crate::contract::{execute, instantiate};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::Vote;

struct TestAddrs {
    deployer: cosmwasm_std::Addr,
    alice: cosmwasm_std::Addr,
    bob: cosmwasm_std::Addr,
    carol: cosmwasm_std::Addr,
    mallory: cosmwasm_std::Addr,
    anyone: cosmwasm_std::Addr,
}

fn instantiate_default(deps: cosmwasm_std::DepsMut) -> TestAddrs {
    let api = MockApi::default();
    let addrs = TestAddrs {
        deployer: api.addr_make("deployer"),
        alice: api.addr_make("alice"),
        bob: api.addr_make("bob"),
        carol: api.addr_make("carol"),
        mallory: api.addr_make("mallory"),
        anyone: api.addr_make("anyone"),
    };
    instantiate(
        deps,
        mock_env(),
        message_info(&addrs.deployer, &[]),
        InstantiateMsg {
            members: vec![
                addrs.alice.to_string(),
                addrs.bob.to_string(),
                addrs.carol.to_string(),
            ],
            threshold: 2,
            max_voting_period_secs: 3600,
        },
    )
    .unwrap();
    addrs
}

#[test]
fn propose_vote_and_execute() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());

    let propose = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::Propose {
            title: "Pay ops".into(),
            description: None,
            msgs: vec![CosmosMsg::Bank(BankMsg::Send {
                to_address: "ops".into(),
                amount: coins(100, "upasg"),
            })],
        },
    )
    .unwrap();
    assert_eq!(propose.attributes[1].value, "1");

    let mut vote_env = mock_env();
    vote_env.block.time = vote_env.block.time.plus_seconds(10);
    let vote = execute(
        deps.as_mut(),
        vote_env.clone(),
        message_info(&addrs.bob, &[]),
        ExecuteMsg::Vote {
            proposal_id: 1,
            vote: Vote::Approve,
        },
    )
    .unwrap();
    assert_eq!(vote.attributes[4].value, "Passed");

    let execute_response = execute(
        deps.as_mut(),
        vote_env,
        message_info(&addrs.anyone, &[]),
        ExecuteMsg::Execute { proposal_id: 1 },
    )
    .unwrap();
    assert_eq!(execute_response.messages.len(), 1);
    assert_eq!(execute_response.attributes[0].value, "execute");
}

#[test]
fn non_member_cannot_propose() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.mallory, &[]),
        ExecuteMsg::Propose {
            title: "Bad".into(),
            description: None,
            msgs: vec![CosmosMsg::Bank(BankMsg::Send {
                to_address: "mallory".into(),
                amount: coins(1, "upasg"),
            })],
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::NotMember {});
}

#[test]
fn update_members_is_self_only() {
    let mut deps = mock_dependencies();
    let addrs = instantiate_default(deps.as_mut());

    let err = execute(
        deps.as_mut(),
        mock_env(),
        message_info(&addrs.alice, &[]),
        ExecuteMsg::UpdateMembers {
            members: vec![addrs.alice.to_string(), addrs.bob.to_string()],
            threshold: 2,
            max_voting_period_secs: Some(7200),
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}
