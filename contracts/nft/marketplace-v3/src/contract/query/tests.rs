use super::*;
use crate::msg::{
    AsksResponse, BidsResponse, CollectionFeeResponse, CollectionRegistrationRequestsResponse,
    CountResponse, QueryMsg,
};
use crate::state::{
    asks, bids, Ask, Bid, CollectionConfig, CollectionRegistrationRequest, CollectionRequestStatus,
    Config, COLLECTION_CONFIGS, COLLECTION_REGISTRATION_REQUESTS, CONFIG,
};
use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env},
    Addr, Coin, Storage, Uint128,
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

#[test]
fn collection_fee_is_marketplace_global() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: Addr::unchecked("admin"),
                min_price: Uint128::new(1),
                trading_fee_bps: 333,
                fee_collector: Addr::unchecked("treasury"),
                registry: None,
                operators: vec![],
                paused: false,
            },
        )
        .unwrap();

    COLLECTION_CONFIGS
        .save(
            deps.as_mut().storage,
            Addr::unchecked("collection"),
            &CollectionConfig {
                collection: Addr::unchecked("collection"),
                active: true,
                denom: "uion".to_string(),
                registered_by: Addr::unchecked("admin"),
                registered_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();

    let response: CollectionFeeResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::CollectionFee {
                collection: "collection".to_string(),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.trading_fee_bps, 333);
    assert!(!response.is_override);
}

#[test]
fn collection_registration_requests_filter_by_status() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    COLLECTION_REGISTRATION_REQUESTS
        .save(
            deps.as_mut().storage,
            Addr::unchecked("collection-a"),
            &CollectionRegistrationRequest {
                collection: Addr::unchecked("collection-a"),
                requester: Addr::unchecked("creator-a"),
                denom: "upasg".to_string(),
                note: None,
                status: CollectionRequestStatus::Pending,
                reviewed_by: None,
                review_note: None,
                created_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();

    COLLECTION_REGISTRATION_REQUESTS
        .save(
            deps.as_mut().storage,
            Addr::unchecked("collection-b"),
            &CollectionRegistrationRequest {
                collection: Addr::unchecked("collection-b"),
                requester: Addr::unchecked("creator-b"),
                denom: "uion".to_string(),
                note: None,
                status: CollectionRequestStatus::Approved,
                reviewed_by: Some(Addr::unchecked("admin")),
                review_note: None,
                created_at: 2,
                updated_at: 3,
            },
        )
        .unwrap();

    let response: CollectionRegistrationRequestsResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::CollectionRegistrationRequests {
                status: Some(CollectionRequestStatus::Pending),
                start_after: None,
                limit: Some(10),
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.requests.len(), 1);
    assert_eq!(
        response.requests[0].collection,
        Addr::unchecked("collection-a")
    );
}
