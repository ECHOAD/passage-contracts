use cosmwasm_std::{
    from_json,
    testing::{message_info, mock_dependencies, mock_env, MockApi},
    to_json_binary, ContractResult, Empty, QuerierResult, SystemError, SystemResult, Timestamp,
    WasmQuery,
};

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
    helpers::{
        Approval, Cw721QueryMsg, Expiration, NftInfoResponse, NftType, OwnerOfResponse,
        TokenMetadata,
    },
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SnapshotResponse, SnapshotsResponse},
    state::{AssetKind, ProgressionSnapshot},
};

fn addr(name: &str) -> String {
    MockApi::default().addr_make(name).to_string()
}

fn install_collection_queries(
    deps: &mut cosmwasm_std::OwnedDeps<
        cosmwasm_std::testing::MockStorage,
        cosmwasm_std::testing::MockApi,
        cosmwasm_std::testing::MockQuerier,
        Empty,
    >,
    collection: String,
    owner: String,
    approvals: Vec<Approval>,
    nft_type: NftType,
) {
    deps.querier.update_wasm(move |query| -> QuerierResult {
        match query {
            WasmQuery::Smart { contract_addr, msg } if contract_addr == &collection => {
                let parsed: Cw721QueryMsg = from_json(msg).unwrap();
                match parsed {
                    Cw721QueryMsg::OwnerOf { .. } => SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&OwnerOfResponse {
                            owner: owner.clone(),
                            approvals: approvals.clone(),
                        })
                        .unwrap(),
                    )),
                    Cw721QueryMsg::NftInfo { .. } => SystemResult::Ok(ContractResult::Ok(
                        to_json_binary(&NftInfoResponse {
                            token_uri: Some("ipfs://token".to_string()),
                            extension: TokenMetadata {
                                nft_type: nft_type.clone(),
                            },
                        })
                        .unwrap(),
                    )),
                }
            }
            WasmQuery::Smart { contract_addr, .. } => {
                SystemResult::Err(SystemError::NoSuchContract {
                    addr: contract_addr.clone(),
                })
            }
            _ => panic!("unexpected query"),
        }
    });
}

fn info(sender: &str) -> cosmwasm_std::MessageInfo {
    message_info(&MockApi::default().addr_make(sender), &[])
}

#[test]
fn owner_can_save_and_overwrite_world_snapshot() {
    let mut deps = mock_dependencies();
    let collection = addr("collection");
    install_collection_queries(
        &mut deps,
        collection.clone(),
        addr("holder"),
        vec![],
        NftType::Avatar,
    );

    instantiate(
        deps.as_mut(),
        mock_env(),
        info("creator"),
        InstantiateMsg {
            admin: addr("admin"),
        },
    )
    .unwrap();

    let first = ExecuteMsg::SaveSnapshot {
        collection: collection.clone(),
        token_id: "avatar-1".to_string(),
        asset_kind: AssetKind::Avatar,
        world: "world-1".to_string(),
        snapshot: ProgressionSnapshot {
            level: 4,
            xp: 90,
            checkpoint: Some("cp-1".to_string()),
            saved_at: Timestamp::from_seconds(100),
        },
    };

    execute(deps.as_mut(), mock_env(), info("holder"), first).unwrap();

    let second = ExecuteMsg::SaveSnapshot {
        collection: collection.clone(),
        token_id: "avatar-1".to_string(),
        asset_kind: AssetKind::Avatar,
        world: "world-1".to_string(),
        snapshot: ProgressionSnapshot {
            level: 5,
            xp: 140,
            checkpoint: Some("cp-2".to_string()),
            saved_at: Timestamp::from_seconds(150),
        },
    };

    execute(deps.as_mut(), mock_env(), info("holder"), second).unwrap();

    let response: SnapshotResponse = from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Snapshot {
                collection,
                token_id: "avatar-1".to_string(),
                world: "world-1".to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();

    let snapshot = response.snapshot.expect("snapshot must exist");
    assert_eq!(snapshot.snapshot.level, 5);
    assert_eq!(snapshot.snapshot.xp, 140);
    assert_eq!(snapshot.updated_by, MockApi::default().addr_make("holder"));
}

#[test]
fn active_cw721_approval_can_save_snapshot() {
    let mut deps = mock_dependencies();
    let collection = addr("collection");
    install_collection_queries(
        &mut deps,
        collection.clone(),
        addr("holder"),
        vec![Approval {
            spender: addr("world-operator"),
            expires: Expiration::Never {},
        }],
        NftType::Companion,
    );

    instantiate(
        deps.as_mut(),
        mock_env(),
        info("creator"),
        InstantiateMsg {
            admin: addr("admin"),
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        mock_env(),
        info("world-operator"),
        ExecuteMsg::SaveSnapshot {
            collection: collection.clone(),
            token_id: "companion-7".to_string(),
            asset_kind: AssetKind::Companion,
            world: "world-2".to_string(),
            snapshot: ProgressionSnapshot {
                level: 12,
                xp: 888,
                checkpoint: None,
                saved_at: Timestamp::from_seconds(200),
            },
        },
    )
    .unwrap();

    let response: SnapshotsResponse = from_json(
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::SnapshotsByAsset {
                collection,
                token_id: "companion-7".to_string(),
                start_after_world: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.snapshots.len(), 1);
    assert_eq!(response.snapshots[0].snapshot.level, 12);
}

#[test]
fn rejects_sender_without_live_owner_or_cw721_approval() {
    let mut deps = mock_dependencies();
    let collection = addr("collection");
    install_collection_queries(
        &mut deps,
        collection.clone(),
        addr("holder"),
        vec![],
        NftType::Avatar,
    );

    instantiate(
        deps.as_mut(),
        mock_env(),
        info("creator"),
        InstantiateMsg {
            admin: addr("admin"),
        },
    )
    .unwrap();

    let err = execute(
        deps.as_mut(),
        mock_env(),
        info("intruder"),
        ExecuteMsg::SaveSnapshot {
            collection,
            token_id: "avatar-1".to_string(),
            asset_kind: AssetKind::Avatar,
            world: "world-1".to_string(),
            snapshot: ProgressionSnapshot {
                level: 1,
                xp: 1,
                checkpoint: None,
                saved_at: Timestamp::from_seconds(100),
            },
        },
    )
    .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});
}

#[test]
fn rejects_progression_writes_for_wrong_asset_kind() {
    let mut deps = mock_dependencies();
    let collection = addr("collection");
    install_collection_queries(
        &mut deps,
        collection.clone(),
        addr("holder"),
        vec![],
        NftType::World,
    );

    instantiate(
        deps.as_mut(),
        mock_env(),
        info("creator"),
        InstantiateMsg {
            admin: addr("admin"),
        },
    )
    .unwrap();

    let err = execute(
        deps.as_mut(),
        mock_env(),
        info("holder"),
        ExecuteMsg::SaveSnapshot {
            collection,
            token_id: "world-1".to_string(),
            asset_kind: AssetKind::Avatar,
            world: "world-1".to_string(),
            snapshot: ProgressionSnapshot {
                level: 1,
                xp: 10,
                checkpoint: None,
                saved_at: Timestamp::from_seconds(100),
            },
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::UnsupportedAssetKind {
            expected: "avatar".to_string(),
            found: "world".to_string(),
        }
    );
}
