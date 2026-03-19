use super::reply;
use crate::msg::RegistryExecuteMsg;
use crate::state::{Config, NftType, PendingCreation, COLLECTION_COUNT, CONFIG, PENDING_CREATIONS};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    Addr, Binary, CosmosMsg, MsgResponse, Reply, SubMsgResponse, SubMsgResult, WasmMsg,
};

fn encode_varint(mut value: usize) -> Vec<u8> {
    let mut encoded = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        encoded.push(byte);
        if value == 0 {
            break;
        }
    }
    encoded
}

fn encode_instantiate_reply_data(contract_address: &str) -> Binary {
    let mut bytes = vec![0x0a];
    bytes.extend(encode_varint(contract_address.len()));
    bytes.extend(contract_address.as_bytes());
    Binary::new(bytes)
}

fn make_reply(contract_address: &str) -> Reply {
    Reply {
        id: 7,
        payload: Binary::default(),
        gas_used: 0,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
            msg_responses: vec![MsgResponse {
                type_url: "/cosmwasm.wasm.v1.MsgInstantiateContractResponse".to_string(),
                value: encode_instantiate_reply_data(contract_address),
            }],
        }),
    }
}

#[test]
fn reply_uses_instantiate_response_data() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection_address = deps.api.addr_make("collection_address");
    let registry = Addr::unchecked("registry");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                registry: registry.clone(),
                ecosystem_id: "eco-1".to_string(),
                collection_code_id: 99,
                enforce_local_allowlist: false,
                paused: false,
            },
        )
        .unwrap();
    COLLECTION_COUNT.save(deps.as_mut().storage, &0).unwrap();

    PENDING_CREATIONS
        .save(
            deps.as_mut().storage,
            7,
            &PendingCreation {
                request_id: 7,
                creator: Addr::unchecked("creator"),
                ecosystem_id: "eco-1".to_string(),
                name: "Collection".to_string(),
                symbol: "CLT".to_string(),
                nft_type: NftType::Component,
                requested_at: 1,
            },
        )
        .unwrap();

    let res = reply(deps.as_mut(), env, make_reply(collection_address.as_str())).unwrap();

    assert_eq!(COLLECTION_COUNT.load(deps.as_ref().storage).unwrap(), 1);
    assert!(PENDING_CREATIONS
        .may_load(deps.as_ref().storage, 7)
        .unwrap()
        .is_none());

    let message = match &res.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        }) => {
            assert_eq!(contract_addr, registry.as_str());
            assert!(funds.is_empty());
            let parsed: RegistryExecuteMsg = from_json(msg).unwrap();
            parsed
        }
        other => panic!("unexpected message: {other:?}"),
    };

    assert_eq!(
        message,
        RegistryExecuteMsg::RegisterCollectionFromFactory {
            address: collection_address.to_string(),
            ecosystem_id: "eco-1".to_string(),
            name: "Collection".to_string(),
            creator: "creator".to_string(),
            nft_type: NftType::Component,
        }
    );
}

#[test]
fn reply_rejects_missing_pending_creation() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                registry: Addr::unchecked("registry"),
                ecosystem_id: "eco-1".to_string(),
                collection_code_id: 99,
                enforce_local_allowlist: false,
                paused: false,
            },
        )
        .unwrap();

    let err = reply(deps.as_mut(), env, make_reply("collection-addr")).unwrap_err();
    assert_eq!(
        err,
        crate::error::ContractError::PendingCreationNotFound { id: 7 }
    );
}
