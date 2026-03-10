use super::*;

use crate::ExecuteMsg;
use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, WasmMsg};
use cw721::NftInfoResponse;
use cw721_base::helpers::Cw721Contract;
use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, Executor};

const CREATOR: &str = "creator";

pub fn contract_cw721_passage() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query);
    Box::new(contract)
}

const TOKEN_ID: &str = "Enterprise";

fn init() -> (BasicApp, Cw721Contract, MintMsg<Option<Metadata>>) {
    let mut app = App::default();
    let code_id = app.store_code(contract_cw721_passage());

    let init_msg = InstantiateMsg {
        name: "SpaceShips".to_string(),
        symbol: "SPACE".to_string(),
        minter: CREATOR.to_string(),
    };
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(CREATOR),
            &init_msg,
            &[],
            "passage",
            None,
        )
        .unwrap();
    let contract = Cw721Contract(contract_addr);

    let mint_msg = MintMsg {
        token_id: TOKEN_ID.to_string(),
        owner: "john".to_string(),
        token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
        extension: Some(Metadata {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
    };
    let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
    let cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract.addr().to_string(),
        msg: to_json_binary(&exec_msg).unwrap(),
        funds: vec![],
    });
    app.execute(Addr::unchecked(CREATOR), cosmos_msg).unwrap();
    (app, contract, mint_msg)
}

#[test]
fn use_metadata_extension() {
    let (app, contract, mint_msg) = init();

    let res: NftInfoResponse<Extension> = contract
        .nft_info::<String, Extension>(&app.wrap(), TOKEN_ID.into())
        .unwrap();
    assert_eq!(res.token_uri, mint_msg.token_uri);
    assert_eq!(res.extension, mint_msg.extension);
}

#[test]
fn burn_disallowed() {
    let (mut app, contract, _) = init();

    let exec_msg = ExecuteMsg::Burn {
        token_id: TOKEN_ID.to_string(),
    };
    let cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract.addr().to_string(),
        msg: to_json_binary(&exec_msg).unwrap(),
        funds: vec![],
    });
    let res = app
        .execute(Addr::unchecked("john"), cosmos_msg)
        .unwrap_err();
    assert!(res
        .chain()
        .any(|cause| cause.to_string().contains("Operation not allowed")));
}
