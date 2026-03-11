use super::*;
use cosmwasm_std::{
    testing::{message_info, mock_dependencies, mock_env},
    BankMsg, Coin, Uint128,
};

#[test]
fn create_split_rule_sets_sender_as_owner() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin = deps.api.addr_make("admin");
    let owner = deps.api.addr_make("owner");
    let platform = deps.api.addr_make("platform");
    let creator = deps.api.addr_make("creator");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin,
                paused: false,
            },
        )
        .unwrap();
    SPLIT_EVENT_COUNT.save(deps.as_mut().storage, &0).unwrap();

    let res = execute_create_split_rule(
        deps.as_mut(),
        env,
        message_info(&owner, &[]),
        "collection-key".to_string(),
        vec![
            RecipientInput {
                address: platform.to_string(),
                share: Decimal::percent(25),
                label: Some("platform".to_string()),
            },
            RecipientInput {
                address: creator.to_string(),
                share: Decimal::percent(75),
                label: Some("creator".to_string()),
            },
        ],
    )
    .unwrap();

    assert_eq!(
        res.attributes[0],
        cosmwasm_std::attr("action", "create_split_rule")
    );

    let rule = SPLIT_RULES
        .load(deps.as_ref().storage, "collection-key")
        .unwrap();
    assert_eq!(rule.owner, owner);
    assert_eq!(rule.recipients.len(), 2);
}

#[test]
fn split_sends_remainder_to_last_recipient_and_records_event() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin = deps.api.addr_make("admin");
    let owner = deps.api.addr_make("owner");
    let sender = deps.api.addr_make("buyer");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin,
                paused: false,
            },
        )
        .unwrap();
    SPLIT_EVENT_COUNT.save(deps.as_mut().storage, &0).unwrap();
    SPLIT_RULES
        .save(
            deps.as_mut().storage,
            "collection-key",
            &SplitRule {
                key: "collection-key".to_string(),
                owner,
                recipients: vec![
                    Recipient {
                        address: Addr::unchecked("platform"),
                        share: Decimal::percent(25),
                        label: None,
                    },
                    Recipient {
                        address: Addr::unchecked("creator"),
                        share: Decimal::percent(75),
                        label: None,
                    },
                ],
                active: true,
                created_at: 0,
                updated_at: 0,
            },
        )
        .unwrap();

    let res = execute_split(
        deps.as_mut(),
        env,
        message_info(&sender, &[Coin::new(101u128, "upasg")]),
        "collection-key".to_string(),
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
    assert_eq!(event.key, "collection-key");
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
    let admin = deps.api.addr_make("admin");
    let owner = deps.api.addr_make("owner");
    let sender = deps.api.addr_make("buyer");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin,
                paused: false,
            },
        )
        .unwrap();
    SPLIT_EVENT_COUNT.save(deps.as_mut().storage, &0).unwrap();
    SPLIT_RULES
        .save(
            deps.as_mut().storage,
            "multi-denom-key",
            &SplitRule {
                key: "multi-denom-key".to_string(),
                owner,
                recipients: vec![
                    Recipient {
                        address: Addr::unchecked("platform"),
                        share: Decimal::percent(25),
                        label: None,
                    },
                    Recipient {
                        address: Addr::unchecked("creator"),
                        share: Decimal::percent(75),
                        label: None,
                    },
                ],
                active: true,
                created_at: 0,
                updated_at: 0,
            },
        )
        .unwrap();

    let res = execute_split(
        deps.as_mut(),
        env,
        message_info(
            &sender,
            &[Coin::new(101u128, "upasg"), Coin::new(7u128, "uatom")],
        ),
        "multi-denom-key".to_string(),
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

    let event = SPLIT_EVENTS.load(deps.as_ref().storage, 1).unwrap();
    assert_eq!(
        event.total_funds,
        vec![Coin::new(101u128, "upasg"), Coin::new(7u128, "uatom")]
    );
    assert_eq!(
        event.recipient_amounts[0].1,
        vec![Coin::new(25u128, "upasg"), Coin::new(1u128, "uatom")]
    );
    assert_eq!(
        event.recipient_amounts[1].1,
        vec![Coin::new(76u128, "upasg"), Coin::new(6u128, "uatom")]
    );
}

#[test]
fn update_split_rule_requires_owner_or_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin = deps.api.addr_make("admin");
    let owner = deps.api.addr_make("owner");
    let stranger = deps.api.addr_make("stranger");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: admin.clone(),
                paused: false,
            },
        )
        .unwrap();
    SPLIT_RULES
        .save(
            deps.as_mut().storage,
            "rule-1",
            &SplitRule {
                key: "rule-1".to_string(),
                owner,
                recipients: vec![Recipient {
                    address: Addr::unchecked("creator"),
                    share: Decimal::one(),
                    label: None,
                }],
                active: true,
                created_at: 0,
                updated_at: 0,
            },
        )
        .unwrap();

    let err = execute_update_split_rule(
        deps.as_mut(),
        env,
        message_info(&stranger, &[]),
        "rule-1".to_string(),
        None,
        Some("new-owner".to_string()),
        None,
    )
    .unwrap_err();

    assert_eq!(err, ContractError::NotRuleOwner {});
}
