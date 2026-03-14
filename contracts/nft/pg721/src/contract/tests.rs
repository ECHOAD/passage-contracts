use super::*;

use crate::msg::{CollectionInfoMsg, ComponentExtension, NftType, NftTypeExtension};
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
        nft_type: NftType::Component,
        collection_info: CollectionInfoMsg {
            creator: String::from("creator"),
            description: String::from("Passage Monkeys"),
            image: image.clone(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info: royalty_info,
        },
    };
    let info = mock_info("creator", &coins(0, NATIVE_DENOM));
    let res = instantiate(deps, mock_env(), info.clone(), msg).unwrap();
    assert!(res.attributes[0].eq(&Attribute::new("action", "instantiate")));
    assert!(res.attributes[1].eq(&Attribute::new("contract_name", CONTRACT_NAME)));
    assert!(res.attributes[2].eq(&Attribute::new("contract_version", CONTRACT_VERSION)));
    assert!(res.attributes[3].eq(&Attribute::new("image", image)));
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
    assert_eq!(NftType::Component, value.nft_type);
    assert_eq!(
        "https://example.com/external.html",
        value.external_link.unwrap()
    );
    assert_eq!(None, value.royalty_info);
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
fn mint_rejects_mismatched_token_metadata_type() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut(), None);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint(cw721_base::MintMsg {
            token_id: "1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Avatar,
                extension: None,
            }),
        }),
    )
    .unwrap_err();

    assert_eq!(
        err.to_string(),
        ContractError::NftTypeMismatch {
            expected: "component".to_string(),
            found: "avatar".to_string(),
        }
        .to_string()
    );
}

#[test]
fn mint_accepts_matching_passage_metadata() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut(), None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint(cw721_base::MintMsg {
            token_id: "1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Component,
                extension: Some(NftTypeExtension::Component(ComponentExtension {
                    component_id: "helmet-1".to_string(),
                    compatible_skeletons: vec!["humanoid".to_string()],
                    compatible_slots: vec!["head".to_string()],
                    component_type: "helmet".to_string(),
                    license: "cc-by".to_string(),
                })),
            }),
        }),
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
    let metadata = nft_info.extension.expect("metadata");

    assert_eq!(metadata.nft_type, NftType::Component);
}
