use super::*;

use crate::state::CollectionInfo;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Attribute, Decimal};

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
