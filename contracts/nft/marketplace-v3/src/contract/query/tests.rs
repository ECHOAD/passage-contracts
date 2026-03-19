use super::*;
use crate::msg::{AsksResponse, BidsResponse, CountResponse, QueryMsg};
use crate::state::{asks, bids, Ask, Bid};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    Addr, Coin, Storage,
};

fn save_ask(
    storage: &mut dyn Storage,
    collection: &str,
    token_id: &str,
    seller: &str,
    price: u128,
) {
    let ask = Ask {
        collection: Addr::unchecked(collection),
        token_id: token_id.to_string(),
        seller: Addr::unchecked(seller),
        price: Coin::new(price, "upasg"),
        funds_recipient: None,
        created_at: price as u64,
        is_active: true,
    };
    asks()
        .save(
            storage,
            (ask.collection.clone(), ask.token_id.clone()),
            &ask,
        )
        .unwrap();
}

fn save_bid(
    storage: &mut dyn Storage,
    collection: &str,
    token_id: &str,
    bidder: &str,
    price: u128,
) {
    let bid = Bid {
        collection: Addr::unchecked(collection),
        token_id: token_id.to_string(),
        bidder: Addr::unchecked(bidder),
        price: Coin::new(price, "upasg"),
        created_at: price as u64,
        expires_at: None,
    };
    bids()
        .save(
            storage,
            (
                bid.collection.clone(),
                bid.token_id.clone(),
                bid.bidder.clone(),
            ),
            &bid,
        )
        .unwrap();
}

#[test]
fn asks_by_price_respects_collection_filter() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection_a = deps.api.addr_make("collection_a");
    let collection_b = deps.api.addr_make("collection_b");
    let seller = deps.api.addr_make("seller");

    save_ask(
        deps.as_mut().storage,
        collection_b.as_str(),
        "1",
        seller.as_str(),
        50,
    );
    save_ask(
        deps.as_mut().storage,
        collection_b.as_str(),
        "2",
        seller.as_str(),
        75,
    );
    save_ask(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        seller.as_str(),
        100,
    );
    save_ask(
        deps.as_mut().storage,
        collection_a.as_str(),
        "2",
        seller.as_str(),
        125,
    );

    let response: AsksResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::AsksByPrice {
                collection: Some(collection_a.to_string()),
                start_after: None,
                limit: Some(2),
                descending: None,
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.asks.len(), 2);
    assert!(response
        .asks
        .iter()
        .all(|ask| ask.collection == collection_a));
    assert_eq!(response.asks[0].token_id, "1");
    assert_eq!(response.asks[1].token_id, "2");
}

#[test]
fn asks_by_seller_respects_start_after() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection_a = deps.api.addr_make("collection_a");
    let collection_b = deps.api.addr_make("collection_b");
    let collection_c = deps.api.addr_make("collection_c");
    let seller = deps.api.addr_make("seller");
    let mut ordered_collections = vec![
        collection_a.clone(),
        collection_b.clone(),
        collection_c.clone(),
    ];
    ordered_collections.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    save_ask(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        seller.as_str(),
        100,
    );
    save_ask(
        deps.as_mut().storage,
        collection_b.as_str(),
        "1",
        seller.as_str(),
        110,
    );
    save_ask(
        deps.as_mut().storage,
        collection_c.as_str(),
        "1",
        seller.as_str(),
        120,
    );

    let response: AsksResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::AsksBySeller {
                seller: seller.to_string(),
                start_after: Some((ordered_collections[0].to_string(), "1".to_string())),
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.asks.len(), 2);
    assert_eq!(response.asks[0].collection, ordered_collections[1]);
    assert_eq!(response.asks[1].collection, ordered_collections[2]);
}

#[test]
fn ask_count_respects_collection_filter() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection_a = deps.api.addr_make("collection_a");
    let collection_b = deps.api.addr_make("collection_b");
    let seller = deps.api.addr_make("seller");

    save_ask(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        seller.as_str(),
        100,
    );
    save_ask(
        deps.as_mut().storage,
        collection_a.as_str(),
        "2",
        seller.as_str(),
        110,
    );
    save_ask(
        deps.as_mut().storage,
        collection_b.as_str(),
        "1",
        seller.as_str(),
        120,
    );

    let response: CountResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::AskCount {
                collection: Some(collection_a.to_string()),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.count, 2);
}

#[test]
fn bids_by_token_respects_start_after() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let collection_a = deps.api.addr_make("collection_a");
    let bidder_c = deps.api.addr_make("bidder_c");
    let bidder_b = deps.api.addr_make("bidder_b");
    let bidder_a = deps.api.addr_make("bidder_a");
    let mut ordered_bidders = vec![bidder_a.clone(), bidder_b.clone(), bidder_c.clone()];
    ordered_bidders.sort_by(|left, right| right.as_str().cmp(left.as_str()));

    save_bid(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        bidder_c.as_str(),
        100,
    );
    save_bid(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        bidder_b.as_str(),
        100,
    );
    save_bid(
        deps.as_mut().storage,
        collection_a.as_str(),
        "1",
        bidder_a.as_str(),
        100,
    );

    let response: BidsResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::BidsByToken {
                collection: collection_a.to_string(),
                token_id: "1".to_string(),
                start_after: Some(ordered_bidders[1].to_string()),
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.bids.len(), 1);
    assert_eq!(response.bids[0].bidder, ordered_bidders[2]);
}
