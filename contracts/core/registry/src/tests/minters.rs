use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    Addr, Storage,
};

use crate::contract::query;
use crate::msg::{AuthorizedMintersResponse, QueryMsg};
use crate::state::{AuthorizedMinter, AUTHORIZED_MINTERS};

fn save_authorized_minter(storage: &mut dyn Storage, collection: &str, minter: &str) {
    let collection_addr = Addr::unchecked(collection);
    let minter_addr = Addr::unchecked(minter);
    AUTHORIZED_MINTERS
        .save(
            storage,
            (collection_addr.clone(), minter_addr.clone()),
            &AuthorizedMinter {
                minter_address: minter_addr,
                collection_address: collection_addr,
                authorized_by: Addr::unchecked("admin"),
                created_at: 1,
            },
        )
        .unwrap();
}

#[test]
fn authorized_minters_respects_start_after() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection = deps.api.addr_make("collection_a");
    let minter_a = deps.api.addr_make("minter_a");
    let minter_b = deps.api.addr_make("minter_b");
    let minter_c = deps.api.addr_make("minter_c");
    let mut ordered_minters = vec![minter_a.clone(), minter_b.clone(), minter_c.clone()];
    ordered_minters.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_a.as_str());
    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_b.as_str());
    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_c.as_str());

    let response: AuthorizedMintersResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::AuthorizedMinters {
                collection_address: collection.to_string(),
                start_after: Some(ordered_minters[0].to_string()),
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.minters, vec![ordered_minters[1].clone(), ordered_minters[2].clone()]);
}

#[test]
fn authorized_minters_respects_limit() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection = deps.api.addr_make("collection_a");
    let minter_a = deps.api.addr_make("minter_a");
    let minter_b = deps.api.addr_make("minter_b");
    let minter_c = deps.api.addr_make("minter_c");
    let minter_d = deps.api.addr_make("minter_d");
    let mut ordered_minters = vec![
        minter_a.clone(),
        minter_b.clone(),
        minter_c.clone(),
        minter_d.clone(),
    ];
    ordered_minters.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_a.as_str());
    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_b.as_str());
    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_c.as_str());
    save_authorized_minter(deps.as_mut().storage, collection.as_str(), minter_d.as_str());

    let first_page: AuthorizedMintersResponse = from_json(
        query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::AuthorizedMinters {
                collection_address: collection.to_string(),
                start_after: None,
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        first_page.minters,
        vec![ordered_minters[0].clone(), ordered_minters[1].clone()]
    );

    let second_page: AuthorizedMintersResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::AuthorizedMinters {
                collection_address: collection.to_string(),
                start_after: Some(ordered_minters[1].to_string()),
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        second_page.minters,
        vec![ordered_minters[2].clone(), ordered_minters[3].clone()]
    );
}
