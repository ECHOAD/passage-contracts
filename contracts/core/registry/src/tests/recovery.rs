use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi};
use cosmwasm_std::Addr;

use crate::contract::execute;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, RecoveryTargetInput};
use crate::state::{
    collections, Collection, Config, NftType, RecoveryCase, RecoveryCaseKind, RecoveryCaseStatus,
    RecoveryConfig, RecoveryPolicy, RecoveryTarget, COLLECTION_RECOVERY_POLICIES, CONFIG,
    LAST_CREATOR_ACTIVITY, RECOVERY_CASES, RECOVERY_CASE_COUNT, RECOVERY_CONFIG,
};

#[test]
fn approved_recovery_case_transfers_collection_creator() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    env.block.time = env.block.time.plus_seconds(200);

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                recovery_council: vec![Addr::unchecked("council")],
                ecosystem_factory: Some(Addr::unchecked("factory")),
                paused: false,
            },
        )
        .unwrap();

    collections()
        .save(
            deps.as_mut().storage,
            Addr::unchecked("collection"),
            &Collection {
                address: Addr::unchecked("collection"),
                ecosystem_id: Some("eco".to_string()),
                name: "Collection".to_string(),
                nft_type: NftType::Component,
                creator: Addr::unchecked("old_creator"),
                verified: false,
                minter: None,
                marketplace: None,
                created_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();

    RECOVERY_CASES
        .save(
            deps.as_mut().storage,
            1,
            &RecoveryCase {
                case_id: 1,
                kind: RecoveryCaseKind::LostAccess,
                target: RecoveryTarget::Collection {
                    address: Addr::unchecked("collection"),
                },
                target_admin: Addr::unchecked("old_creator"),
                opened_by: Addr::unchecked("delegate"),
                reason: "inactive".to_string(),
                evidence_url: None,
                replacement_candidate: Addr::unchecked("new_creator"),
                last_target_activity_at: 1,
                status: RecoveryCaseStatus::Open,
                opened_at: 1,
                contest_deadline: 100,
                contested_at: None,
                contested_by: None,
                contest_note: None,
                resolved_at: None,
                resolved_by: None,
                resolution_approved: None,
                resolution_note: None,
            },
        )
        .unwrap();

    let res = execute(
        deps.as_mut(),
        env,
        message_info(&Addr::unchecked("council"), &[]),
        ExecuteMsg::ResolveRecoveryCase {
            case_id: 1,
            approved: true,
            note: None,
        },
    )
    .unwrap();

    let collection = collections()
        .load(deps.as_ref().storage, Addr::unchecked("collection"))
        .unwrap();
    assert_eq!(collection.creator, Addr::unchecked("new_creator"));
    assert!(res
        .attributes
        .iter()
        .any(|attr| attr.key == "replacement" && attr.value == "new_creator"));
}

#[test]
fn lost_access_case_requires_delegate_or_recovery_authority() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let api = MockApi::default();
    let collection_addr = api.addr_make("collection");
    let creator_addr = api.addr_make("creator");
    let delegate_addr = api.addr_make("delegate");
    let successor_addr = api.addr_make("new_creator");
    let random_addr = api.addr_make("random");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                recovery_council: vec![Addr::unchecked("council")],
                ecosystem_factory: Some(Addr::unchecked("factory")),
                paused: false,
            },
        )
        .unwrap();
    RECOVERY_CONFIG
        .save(
            deps.as_mut().storage,
            &RecoveryConfig {
                abandonment_inactivity_period_secs: 90,
                contest_period_secs: 30,
            },
        )
        .unwrap();
    collections()
        .save(
            deps.as_mut().storage,
            collection_addr.clone(),
            &Collection {
                address: collection_addr.clone(),
                ecosystem_id: Some("eco".to_string()),
                name: "Collection".to_string(),
                nft_type: NftType::Avatar,
                creator: creator_addr.clone(),
                verified: false,
                minter: None,
                marketplace: None,
                created_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();
    COLLECTION_RECOVERY_POLICIES
        .save(
            deps.as_mut().storage,
            collection_addr.clone(),
            &RecoveryPolicy {
                delegate: Some(delegate_addr.clone()),
                designated_successor: Some(successor_addr.clone()),
                updated_by: Some(creator_addr.clone()),
                updated_at: Some(1),
            },
        )
        .unwrap();
    LAST_CREATOR_ACTIVITY
        .save(
            deps.as_mut().storage,
            creator_addr,
            &(env.block.time.seconds() - 50),
        )
        .unwrap();
    RECOVERY_CASE_COUNT.save(deps.as_mut().storage, &0).unwrap();

    let unauthorized = execute(
        deps.as_mut(),
        env.clone(),
        message_info(&random_addr, &[]),
        ExecuteMsg::OpenRecoveryCase {
            case_kind: RecoveryCaseKind::LostAccess,
            target: RecoveryTargetInput::Collection {
                address: collection_addr.to_string(),
            },
            reason: "wallet lost".to_string(),
            evidence_url: None,
            proposed_replacement: Some(successor_addr.to_string()),
        },
    )
    .unwrap_err();
    assert_eq!(unauthorized, ContractError::Unauthorized {});

    let opened = execute(
        deps.as_mut(),
        env,
        message_info(&delegate_addr, &[]),
        ExecuteMsg::OpenRecoveryCase {
            case_kind: RecoveryCaseKind::LostAccess,
            target: RecoveryTargetInput::Collection {
                address: collection_addr.to_string(),
            },
            reason: "wallet lost".to_string(),
            evidence_url: None,
            proposed_replacement: None,
        },
    )
    .unwrap();

    assert!(opened.attributes.iter().any(|attr| {
        attr.key == "replacement_candidate" && attr.value == successor_addr.to_string()
    }));
}

#[test]
fn abandonment_case_requires_inactivity_threshold() {
    let mut deps = mock_dependencies();
    let mut env = mock_env();
    env.block.time = env.block.time.plus_seconds(50);
    let api = MockApi::default();
    let collection_addr = api.addr_make("collection");
    let creator_addr = api.addr_make("creator");
    let successor_addr = api.addr_make("new_creator");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                recovery_council: vec![Addr::unchecked("council")],
                ecosystem_factory: Some(Addr::unchecked("factory")),
                paused: false,
            },
        )
        .unwrap();
    RECOVERY_CONFIG
        .save(
            deps.as_mut().storage,
            &RecoveryConfig {
                abandonment_inactivity_period_secs: 100,
                contest_period_secs: 30,
            },
        )
        .unwrap();
    collections()
        .save(
            deps.as_mut().storage,
            collection_addr.clone(),
            &Collection {
                address: collection_addr.clone(),
                ecosystem_id: Some("eco".to_string()),
                name: "Collection".to_string(),
                nft_type: NftType::Companion,
                creator: creator_addr.clone(),
                verified: false,
                minter: None,
                marketplace: None,
                created_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();
    LAST_CREATOR_ACTIVITY
        .save(
            deps.as_mut().storage,
            creator_addr,
            &(env.block.time.seconds() - 50),
        )
        .unwrap();
    RECOVERY_CASE_COUNT.save(deps.as_mut().storage, &0).unwrap();

    let err = execute(
        deps.as_mut(),
        env,
        message_info(&Addr::unchecked("reporter"), &[]),
        ExecuteMsg::OpenRecoveryCase {
            case_kind: RecoveryCaseKind::Abandonment,
            target: RecoveryTargetInput::Collection {
                address: collection_addr.to_string(),
            },
            reason: "project abandoned".to_string(),
            evidence_url: None,
            proposed_replacement: Some(successor_addr.to_string()),
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::RecoveryAbandonmentThresholdNotMet {
            target: format!("collection:{collection_addr}")
        }
    );
}

