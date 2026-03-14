use cosmwasm_std::from_json;
use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env, MockApi};

use crate::contract::{execute, query};
use crate::msg::{CollectionsResponse, ExecuteMsg, QueryMsg};
use crate::state::{
    CollectionCreationPolicy, Config, Ecosystem, EcosystemType, NftType, CONFIG, ECOSYSTEMS,
};

#[test]
fn collections_by_nft_type_returns_only_matching_collections() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let api = MockApi::default();
    let registry_admin = api.addr_make("registry_admin");
    let creator = api.addr_make("creator");
    let factory = api.addr_make("factory");
    let component_collection = api.addr_make("component_collection");
    let avatar_collection = api.addr_make("avatar_collection");

    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                admin: registry_admin,
                operators: vec![],
                recovery_council: vec![],
                ecosystem_factory: None,
                paused: false,
            },
        )
        .unwrap();

    ECOSYSTEMS
        .save(
            deps.as_mut().storage,
            "eco".to_string(),
            &Ecosystem {
                id: "eco".to_string(),
                name: "Eco".to_string(),
                admin: creator.clone(),
                ecosystem_type: EcosystemType::Private,
                collection_creation_policy: CollectionCreationPolicy::Permissioned,
                collection_factory: Some(factory.clone()),
                description: "desc".to_string(),
                image_urls: vec!["https://example.com/image.png".to_string()],
                animation_url: None,
                url: None,
                created_at: 1,
                updated_at: 1,
            },
        )
        .unwrap();

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&factory, &[]),
        ExecuteMsg::RegisterCollectionFromFactory {
            address: component_collection.to_string(),
            ecosystem_id: "eco".to_string(),
            name: "Components".to_string(),
            creator: creator.to_string(),
            nft_type: NftType::Component,
        },
    )
    .unwrap();

    execute(
        deps.as_mut(),
        env.clone(),
        message_info(&factory, &[]),
        ExecuteMsg::RegisterCollectionFromFactory {
            address: avatar_collection.to_string(),
            ecosystem_id: "eco".to_string(),
            name: "Avatars".to_string(),
            creator: creator.to_string(),
            nft_type: NftType::Avatar,
        },
    )
    .unwrap();

    let response: CollectionsResponse = from_json(
        query(
            deps.as_ref(),
            env,
            QueryMsg::CollectionsByNftType {
                nft_type: NftType::Component,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(response.collections.len(), 1);
    assert_eq!(response.collections[0].address, component_collection);
    assert_eq!(response.collections[0].nft_type, NftType::Component);
}
