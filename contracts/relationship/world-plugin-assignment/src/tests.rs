use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdError, StdResult,
};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw_storage_plus::Map;
use cw721::{Approval, Cw721QueryMsg, Expiration, OwnerOfResponse};

use crate::contract::{execute, instantiate, query};
use crate::msg::{
    AssetRef, AssignmentRecordResponse, AssignmentsByPluginResponse, AssignmentsByWorldResponse,
    ExecuteMsg, InstantiateMsg, QueryMsg,
};

const ASSIGNER: &str = "assigner";
const PLUGIN_REBUYER: &str = "plugin-rebuyer";
const WORLD_OPERATOR: &str = "world-operator";

const OWNERS: Map<String, String> = Map::new("owners");
const OPERATORS: Map<(String, String), bool> = Map::new("operators");

#[cw_serde]
enum MockNftExecuteMsg {
    Mint { token_id: String, owner: String },
    TransferNft { token_id: String, recipient: String },
    ApproveAll { operator: String },
}

fn mock_nft_instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

fn mock_nft_execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MockNftExecuteMsg,
) -> StdResult<Response> {
    match msg {
        MockNftExecuteMsg::Mint { token_id, owner } => {
            OWNERS.save(deps.storage, token_id, &owner)?;
            Ok(Response::new())
        }
        MockNftExecuteMsg::TransferNft {
            token_id,
            recipient,
        } => {
            let owner = OWNERS.load(deps.storage, token_id.clone())?;
            if owner != info.sender.as_str() {
                return Err(StdError::generic_err("unauthorized"));
            }
            OWNERS.save(deps.storage, token_id, &recipient)?;
            Ok(Response::new())
        }
        MockNftExecuteMsg::ApproveAll { operator } => {
            OPERATORS.save(
                deps.storage,
                (info.sender.to_string(), operator.clone()),
                &true,
            )?;
            Ok(Response::new())
        }
    }
}

fn mock_nft_query(deps: Deps, _env: Env, msg: Cw721QueryMsg) -> StdResult<Binary> {
    match msg {
        Cw721QueryMsg::OwnerOf { token_id, .. } => {
            let owner = OWNERS.load(deps.storage, token_id)?;
            let approvals = OPERATORS
                .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .filter_map(Result::ok)
                .filter(|((record_owner, _), approved)| record_owner == &owner && *approved)
                .map(|((_, operator), _)| Approval {
                    spender: operator,
                    expires: Expiration::Never {},
                })
                .collect();

            to_json_binary(&OwnerOfResponse { owner, approvals })
        }
        _ => Err(StdError::generic_err("unsupported query")),
    }
}

fn world_plugin_assignment_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn mock_nft_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(
        mock_nft_execute,
        mock_nft_instantiate,
        mock_nft_query,
    ))
}

fn instantiate_mock_nft(app: &mut App, label: &str) -> Addr {
    let code_id = app.store_code(mock_nft_contract());
    app.instantiate_contract(code_id, Addr::unchecked(ASSIGNER), &Empty {}, &[], label, None)
        .unwrap()
}

fn mint_token(app: &mut App, collection: &Addr, token_id: &str, owner: &str) {
    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        collection.clone(),
        &MockNftExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: owner.to_string(),
        },
        &[],
    )
    .unwrap();
}

fn assign_contract_address(app: &mut App) -> Addr {
    let code_id = app.store_code(world_plugin_assignment_contract());
    app.instantiate_contract(
        code_id,
        Addr::unchecked(ASSIGNER),
        &InstantiateMsg {},
        &[],
        "world-plugin-assignment",
        None,
    )
    .unwrap()
}

#[test]
fn assignment_survives_plugin_resale() {
    let mut app = App::default();
    let plugin_collection = instantiate_mock_nft(&mut app, "plugin");
    let world_collection = instantiate_mock_nft(&mut app, "world");
    let assignment = assign_contract_address(&mut app);

    mint_token(&mut app, &plugin_collection, "plugin-1", ASSIGNER);
    mint_token(&mut app, &world_collection, "world-1", ASSIGNER);

    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        assignment.clone(),
        &ExecuteMsg::Assign {
            plugin: AssetRef {
                collection: plugin_collection.to_string(),
                token_id: "plugin-1".to_string(),
            },
            world: AssetRef {
                collection: world_collection.to_string(),
                token_id: "world-1".to_string(),
            },
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        plugin_collection.clone(),
        &MockNftExecuteMsg::TransferNft {
            recipient: PLUGIN_REBUYER.to_string(),
            token_id: "plugin-1".to_string(),
        },
        &[],
    )
    .unwrap();

    let assignment_record: Option<AssignmentRecordResponse> = app
        .wrap()
        .query_wasm_smart(
            assignment,
            &QueryMsg::Assignment {
                plugin: AssetRef {
                    collection: plugin_collection.to_string(),
                    token_id: "plugin-1".to_string(),
                },
                world: AssetRef {
                    collection: world_collection.to_string(),
                    token_id: "world-1".to_string(),
                },
            },
        )
        .unwrap();

    let owner_query = app
        .wrap()
        .query_wasm_smart::<OwnerOfResponse>(
            plugin_collection,
            &Cw721QueryMsg::OwnerOf {
                token_id: "plugin-1".to_string(),
                include_expired: Some(false),
            },
        )
        .unwrap();

    assert_eq!(owner_query.owner, PLUGIN_REBUYER);
    let assignment_record = assignment_record.expect("assignment must persist");
    assert_eq!(assignment_record.assigned_by, ASSIGNER);
    assert_eq!(assignment_record.plugin.token_id, "plugin-1");
    assert_eq!(assignment_record.world.token_id, "world-1");
}

#[test]
fn assignment_requires_real_plugin_and_world_authority() {
    let mut app = App::default();
    let plugin_collection = instantiate_mock_nft(&mut app, "plugin");
    let world_collection = instantiate_mock_nft(&mut app, "world");
    let assignment = assign_contract_address(&mut app);

    mint_token(&mut app, &plugin_collection, "plugin-1", ASSIGNER);
    mint_token(&mut app, &world_collection, "world-1", ASSIGNER);

    let err = app
        .execute_contract(
            Addr::unchecked("intruder"),
            assignment.clone(),
            &ExecuteMsg::Assign {
                plugin: AssetRef {
                    collection: plugin_collection.to_string(),
                    token_id: "plugin-1".to_string(),
                },
                world: AssetRef {
                    collection: world_collection.to_string(),
                    token_id: "world-1".to_string(),
                },
            },
            &[],
        )
        .unwrap_err();

    let stored: Option<AssignmentRecordResponse> = app
        .wrap()
        .query_wasm_smart(
            assignment,
            &QueryMsg::Assignment {
                plugin: AssetRef {
                    collection: plugin_collection.to_string(),
                    token_id: "plugin-1".to_string(),
                },
                world: AssetRef {
                    collection: world_collection.to_string(),
                    token_id: "world-1".to_string(),
                },
            },
        )
        .unwrap();

    assert!(err.to_string().contains("sender: intruder"));
    assert!(stored.is_none());
}

#[test]
fn operator_can_assign_when_approved_by_plugin_and_world_owners() {
    let mut app = App::default();
    let plugin_collection = instantiate_mock_nft(&mut app, "plugin");
    let world_collection = instantiate_mock_nft(&mut app, "world");
    let assignment = assign_contract_address(&mut app);

    mint_token(&mut app, &plugin_collection, "plugin-1", ASSIGNER);
    mint_token(&mut app, &world_collection, "world-1", ASSIGNER);

    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        plugin_collection.clone(),
        &MockNftExecuteMsg::ApproveAll {
            operator: WORLD_OPERATOR.to_string(),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        world_collection.clone(),
        &MockNftExecuteMsg::ApproveAll {
            operator: WORLD_OPERATOR.to_string(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(WORLD_OPERATOR),
        assignment.clone(),
        &ExecuteMsg::Assign {
            plugin: AssetRef {
                collection: plugin_collection.to_string(),
                token_id: "plugin-1".to_string(),
            },
            world: AssetRef {
                collection: world_collection.to_string(),
                token_id: "world-1".to_string(),
            },
        },
        &[],
    )
    .unwrap();

    let by_plugin: AssignmentsByPluginResponse = app
        .wrap()
        .query_wasm_smart(
            assignment,
            &QueryMsg::AssignmentsByPlugin {
                plugin: AssetRef {
                    collection: plugin_collection.to_string(),
                    token_id: "plugin-1".to_string(),
                },
            },
        )
        .unwrap();

    assert_eq!(by_plugin.assignments.len(), 1);
    assert_eq!(by_plugin.assignments[0].assigned_by, WORLD_OPERATOR);
}

#[test]
fn query_indexes_assignments_by_world_and_plugin() {
    let mut app = App::default();
    let plugin_collection = instantiate_mock_nft(&mut app, "plugin");
    let world_collection = instantiate_mock_nft(&mut app, "world");
    let assignment = assign_contract_address(&mut app);

    mint_token(&mut app, &plugin_collection, "plugin-1", ASSIGNER);
    mint_token(&mut app, &world_collection, "world-1", ASSIGNER);

    app.execute_contract(
        Addr::unchecked(ASSIGNER),
        assignment.clone(),
        &ExecuteMsg::Assign {
            plugin: AssetRef {
                collection: plugin_collection.to_string(),
                token_id: "plugin-1".to_string(),
            },
            world: AssetRef {
                collection: world_collection.to_string(),
                token_id: "world-1".to_string(),
            },
        },
        &[],
    )
    .unwrap();

    let by_world: AssignmentsByWorldResponse = app
        .wrap()
        .query_wasm_smart(
            assignment.clone(),
            &QueryMsg::AssignmentsByWorld {
                world: AssetRef {
                    collection: world_collection.to_string(),
                    token_id: "world-1".to_string(),
                },
            },
        )
        .unwrap();
    let by_plugin: AssignmentsByPluginResponse = app
        .wrap()
        .query_wasm_smart(
            assignment,
            &QueryMsg::AssignmentsByPlugin {
                plugin: AssetRef {
                    collection: plugin_collection.to_string(),
                    token_id: "plugin-1".to_string(),
                },
            },
        )
        .unwrap();

    assert_eq!(by_world.assignments.len(), 1);
    assert_eq!(by_plugin.assignments.len(), 1);

    let world_binary = to_json_binary(&by_world).unwrap();
    let decoded: AssignmentsByWorldResponse = from_json(&world_binary).unwrap();
    assert_eq!(decoded.assignments[0].plugin.token_id, "plugin-1");
}
