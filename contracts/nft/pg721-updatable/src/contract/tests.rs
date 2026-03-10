use super::*;

use crate::state::CollectionInfo;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Attribute, Decimal};
use cw721::NftInfoResponse;

const NATIVE_DENOM: &str = "ujunox";

fn setup_contract(deps: DepsMut, royalty_info: Option<RoyaltyInfoResponse>) {
    let collection = String::from("collection0");
    let image: String = "https://example.com/image.png".to_string();
    let msg = InstantiateMsg {
        name: collection,
        symbol: String::from("BOBO"),
        minter: String::from("minter"),
        collection_info: CollectionInfo {
            creator: String::from("creator"),
            description: String::from("Passage Monkeys"),
            image: image.clone(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info,
        },
    };
    let info = mock_info("creator", &coins(0, NATIVE_DENOM));
    let res = instantiate(deps, mock_env(), info, msg).unwrap();
    assert!(res.attributes[0].eq(&Attribute::new("action", "instantiate")));
    assert!(res.attributes[1].eq(&Attribute::new("contract_name", CONTRACT_NAME)));
    assert!(res.attributes[2].eq(&Attribute::new("contract_version", CONTRACT_VERSION)));
    assert!(res.attributes[3].eq(&Attribute::new("image", image)));
}

fn mint_token(deps: DepsMut, token_id: &str, token_uri: Option<String>) {
    let mint_msg = ExecuteMsg::Mint {
        token_id: token_id.to_string(),
        owner: String::from("owner"),
        token_uri,
        extension: None,
    };

    execute(deps, mock_env(), mock_info("minter", &[]), mint_msg).unwrap();
}

#[test]
fn proper_initialization_no_royalties() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut(), None);

    // let's query the collection info
    let res = query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {}).unwrap();
    let value: CollectionInfoResponse = from_json(&res).unwrap();
    assert_eq!("https://example.com/image.png", value.image);
    assert_eq!("Passage Monkeys", value.description);
    assert_eq!(
        "https://example.com/external.html",
        value.external_link.unwrap()
    );
    assert_eq!(None, value.royalty_info);

    let frozen_res = query(deps.as_ref(), mock_env(), QueryMsg::FrozenTokenMetadata {}).unwrap();
    let frozen: FrozenTokenMetadataResponse = from_json(&frozen_res).unwrap();
    assert!(!frozen.frozen);
}

#[test]
fn proper_initialization_with_royalties() {
    let mut deps = mock_dependencies();
    let creator: String = String::from("creator");
    setup_contract(
        deps.as_mut(),
        Some(RoyaltyInfoResponse {
            payment_address: creator.clone(),
            share: Decimal::percent(10),
        }),
    );

    // let's query the collection info
    let res = query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {}).unwrap();
    let value: CollectionInfoResponse = from_json(&res).unwrap();
    assert_eq!(
        Some(RoyaltyInfoResponse {
            payment_address: creator,
            share: Decimal::percent(10),
        }),
        value.royalty_info
    );
}

#[test]
fn update_and_freeze_token_metadata() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut(), None);

    mint_token(
        deps.as_mut(),
        "1",
        Some("ipfs://old-cid/1.json".to_string()),
    );

    let updated_token_uri = Some("ipfs://new-cid/1.json".to_string());

    // Unauthorized sender cannot update metadata.
    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("hacker", &[]),
        ExecuteMsg::UpdateTokenMetadata {
            token_id: "1".to_string(),
            token_uri: updated_token_uri.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err.to_string(), ContractError::Unauthorized {}.to_string());

    // Creator can update metadata.
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[]),
        ExecuteMsg::UpdateTokenMetadata {
            token_id: "1".to_string(),
            token_uri: updated_token_uri.clone(),
        },
    )
    .unwrap();

    let nft_info_bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NftInfo {
            token_id: "1".to_string(),
        },
    )
    .unwrap();
    let nft_info: NftInfoResponse<Extension> = from_json(&nft_info_bin).unwrap();
    assert_eq!(nft_info.token_uri, updated_token_uri);

    // Freeze token metadata.
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[]),
        ExecuteMsg::FreezeTokenMetadata {},
    )
    .unwrap();

    let frozen_res = query(deps.as_ref(), mock_env(), QueryMsg::FrozenTokenMetadata {}).unwrap();
    let frozen: FrozenTokenMetadataResponse = from_json(&frozen_res).unwrap();
    assert!(frozen.frozen);

    // Updates are blocked once frozen.
    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("creator", &[]),
        ExecuteMsg::UpdateTokenMetadata {
            token_id: "1".to_string(),
            token_uri: Some("ipfs://other-cid/1.json".to_string()),
        },
    )
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        ContractError::TokenMetadataFrozen {}.to_string()
    );
}
