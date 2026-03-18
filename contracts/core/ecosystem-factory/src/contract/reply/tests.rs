use super::reply;
use crate::msg::RegistryExecuteMsg;
use crate::state::{
    Config, EcosystemCreationRequest, EcosystemCreationRequestStatus, PendingEcosystemCreation,
    CONFIG, PENDING_ECOSYSTEM_CREATIONS, REQUESTS,
};
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
        id: 12,
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
    let factory_addr = deps.api.addr_make("collection_factory");
    let registry = Addr::unchecked("registry");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                operators: vec![],
                registry: registry.clone(),
                collection_factory_code_id: 11,
                collection_code_id: 99,
                paused: false,
            },
        )
        .unwrap();

    PENDING_ECOSYSTEM_CREATIONS
        .save(
            deps.as_mut().storage,
            12,
            &PendingEcosystemCreation {
                request_id: 12,
                ecosystem_id: "eco-1".to_string(),
                ecosystem_name: "Eco".to_string(),
                creator: Addr::unchecked("creator"),
                description: "desc".to_string(),
                image_urls: vec!["https://example.com/image.png".to_string()],
                animation_url: Some("https://example.com/anim.mp4".to_string()),
                url: None,
            },
        )
        .unwrap();
    REQUESTS
        .save(
            deps.as_mut().storage,
            12,
            &EcosystemCreationRequest {
                request_id: 12,
                creator: Addr::unchecked("creator"),
                id: "eco-1".to_string(),
                name: "Eco".to_string(),
                description: "desc".to_string(),
                image_urls: vec!["https://example.com/image.png".to_string()],
                animation_url: Some("https://example.com/anim.mp4".to_string()),
                url: None,
                status: EcosystemCreationRequestStatus::Approved,
                submitted_at: 1,
                reviewed_at: Some(2),
                reviewed_by: Some(Addr::unchecked("admin")),
                review_note: None,
                collection_factory: None,
                created_at: None,
            },
        )
        .unwrap();

    let res = reply(deps.as_mut(), env, make_reply(factory_addr.as_str())).unwrap();

    assert!(PENDING_ECOSYSTEM_CREATIONS
        .may_load(deps.as_ref().storage, 12)
        .unwrap()
        .is_none());
    let stored = REQUESTS.load(deps.as_ref().storage, 12).unwrap();
    assert_eq!(stored.status, EcosystemCreationRequestStatus::Created);
    assert_eq!(stored.collection_factory, Some(factory_addr.clone()));
    assert_eq!(stored.created_at, Some(1571797419));

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
        RegistryExecuteMsg::RegisterEcosystemFromFactory {
            id: "eco-1".to_string(),
            name: "Eco".to_string(),
            creator: "creator".to_string(),
            collection_factory: factory_addr.to_string(),
            description: "desc".to_string(),
            image_urls: vec!["https://example.com/image.png".to_string()],
            animation_url: Some("https://example.com/anim.mp4".to_string()),
            url: None,
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
                collection_factory_code_id: 11,
                collection_code_id: 99,
                paused: false,
            },
        )
        .unwrap();

    let err = reply(deps.as_mut(), env, make_reply("collection-factory")).unwrap_err();
    assert_eq!(
        err,
        crate::error::ContractError::PendingCreationNotFound { reply_id: 12 }
    );
}
