use super::*;
use cosmwasm_std::{
    testing::{message_info, mock_dependencies, mock_env},
    Addr, BankMsg, Coin, CosmosMsg, Decimal, StdResult, Uint128,
};

fn save_base_state(deps: DepsMut) {
    CONFIG
        .save(
            deps.storage,
            &Config {
                admin: Addr::unchecked("admin"),
                paused: false,
            },
        )
        .unwrap();
    SPLIT_CONFIG
        .save(
            deps.storage,
            &SplitConfig {
                recipients: vec![
                    Recipient {
                        address: Addr::unchecked("platform"),
                        share: Decimal::percent(25),
                        label: Some("platform".to_string()),
                    },
                    Recipient {
                        address: Addr::unchecked("creator"),
                        share: Decimal::percent(75),
                        label: Some("creator".to_string()),
                    },
                ],
                active: true,
                created_at: 0,
                updated_at: 0,
            },
        )
        .unwrap();
    SPLIT_EVENT_COUNT.save(deps.storage, &0).unwrap();
}

#[test]
fn update_split_requires_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    save_base_state(deps.as_mut());

    let err = execute_update_split(
        deps.as_mut(),
        env,
        message_info(&Addr::unchecked("stranger"), &[]),
        Some(vec![RecipientInput {
            address: "creator".to_string(),
            share: Decimal::one(),
            label: None,
        }]),
        None,
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn split_sends_remainder_to_last_recipient_and_records_event() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = deps.api.addr_make("buyer");

    save_base_state(deps.as_mut());

    let res = execute_split(
        deps.as_mut(),
        env,
        message_info(&sender, &[Coin::new(101u128, "upasg")]),
    )
    .unwrap();

    assert_eq!(res.messages.len(), 2);

    match &res.messages[0].msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, "platform");
            assert_eq!(amount[0].amount, Uint128::new(25));
        }
        _ => panic!("expected first bank send"),
    }

    match &res.messages[1].msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, "creator");
            assert_eq!(amount[0].amount, Uint128::new(76));
        }
        _ => panic!("expected second bank send"),
    }

    let event = SPLIT_EVENTS.load(deps.as_ref().storage, 1).unwrap();
    assert_eq!(event.total_funds, vec![Coin::new(101u128, "upasg")]);
    assert_eq!(
        event.recipient_amounts[0].1,
        vec![Coin::new(25u128, "upasg")]
    );
    assert_eq!(
        event.recipient_amounts[1].1,
        vec![Coin::new(76u128, "upasg")]
    );
}

#[test]
fn split_handles_multiple_denoms_with_shared_percentages() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = deps.api.addr_make("buyer");

    save_base_state(deps.as_mut());

    let res = execute_split(
        deps.as_mut(),
        env,
        message_info(
            &sender,
            &[Coin::new(101u128, "upasg"), Coin::new(7u128, "uatom")],
        ),
    )
    .unwrap();

    assert_eq!(res.messages.len(), 2);

    match &res.messages[0].msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, "platform");
            assert_eq!(
                amount,
                &vec![Coin::new(25u128, "upasg"), Coin::new(1u128, "uatom")]
            );
        }
        _ => panic!("expected platform bank send"),
    }

    match &res.messages[1].msg {
        CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
            assert_eq!(to_address, "creator");
            assert_eq!(
                amount,
                &vec![Coin::new(76u128, "upasg"), Coin::new(6u128, "uatom")]
            );
        }
        _ => panic!("expected creator bank send"),
    }
}

#[test]
fn split_fails_when_inactive() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender = deps.api.addr_make("buyer");

    save_base_state(deps.as_mut());
    SPLIT_CONFIG
        .update(deps.as_mut().storage, |mut split| -> StdResult<_> {
            split.active = false;
            Ok(split)
        })
        .unwrap();

    let err = execute_split(
        deps.as_mut(),
        env,
        message_info(&sender, &[Coin::new(10u128, "upasg")]),
    )
    .unwrap_err();

    assert_eq!(err, ContractError::SplitInactive {});
}
