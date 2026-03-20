use super::*;

use crate::msg::{
    AchievementExtension, AvatarExtension, CollectionInfoMsg, CompanionExtension, NftType,
    NftTypeExtension, PassageProfileId, PluginExtension, WorldTemplateExtension,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Attribute, Decimal};
use cw721::NftInfoResponse;

const NATIVE_DENOM: &str = "ujunox";

fn setup_contract(deps: DepsMut, royalty_info: Option<RoyaltyInfoResponse>) {
    setup_contract_with_type(deps, NftType::Component, royalty_info);
}

fn setup_contract_with_type(
    deps: DepsMut,
    nft_type: NftType,
    royalty_info: Option<RoyaltyInfoResponse>,
) {
    let collection = String::from("collection0");
    let image: String = "https://example.com/image.png".to_string();
    let msg = InstantiateMsg {
        name: collection,
        symbol: String::from("BOBO"),
        minter: String::from("minter"),
        nft_type,
        collection_info: CollectionInfoMsg {
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
        extension: Some(TokenMetadata {
            nft_type: NftType::Component,
            extension: None,
        }),
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
    assert_eq!(NftType::Component, value.nft_type);
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
    assert_eq!(
        nft_info.extension,
        Some(TokenMetadata {
            nft_type: NftType::Component,
            extension: None,
        })
    );

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

#[test]
fn mint_rejects_mismatched_passage_metadata_type() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut(), None);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "2".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/2.json".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Avatar,
                extension: None,
            }),
        },
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
fn mint_rejects_avatar_without_profile_id() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Avatar, None);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "avatar-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/avatar-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Avatar,
                extension: Some(NftTypeExtension::Avatar(AvatarExtension {
                    avatar_id: "avatar-1".to_string(),
                    profile_id: None,
                })),
            }),
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::MissingProfileId {
            nft_type: "avatar".to_string(),
        }
    );
}

#[test]
fn mint_accepts_companion_with_standard_profile_id() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Companion, None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "companion-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/companion-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Companion,
                extension: Some(NftTypeExtension::Companion(CompanionExtension {
                    companion_id: "companion-1".to_string(),
                    profile_id: Some(PassageProfileId::PassageCompanionV1),
                })),
            }),
        },
    )
    .unwrap();
}

#[test]
fn mint_rejects_companion_with_avatar_profile_id() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Companion, None);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "companion-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/companion-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Companion,
                extension: Some(NftTypeExtension::Companion(CompanionExtension {
                    companion_id: "companion-1".to_string(),
                    profile_id: Some(PassageProfileId::PassageAvatarV1),
                })),
            }),
        },
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::InvalidProfileId {
            nft_type: "companion".to_string(),
            expected: PassageProfileId::PassageCompanionV1.to_string(),
            found: PassageProfileId::PassageAvatarV1.to_string(),
        }
    );
}

#[test]
fn mint_accepts_plugin_metadata() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Plugin, None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "plugin-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/plugin-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Plugin,
                extension: Some(NftTypeExtension::Plugin(PluginExtension {
                    plugin_id: "builder-tools".to_string(),
                    plugin_type: "world_editor".to_string(),
                    license: "commercial".to_string(),
                })),
            }),
        },
    )
    .unwrap();

    let nft_info_bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NftInfo {
            token_id: "plugin-1".to_string(),
        },
    )
    .unwrap();
    let nft_info: NftInfoResponse<Extension> = from_json(&nft_info_bin).unwrap();
    let metadata = nft_info.extension.expect("metadata");

    assert_eq!(metadata.nft_type, NftType::Plugin);
}

#[test]
fn mint_accepts_achievement_metadata() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Achievement, None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "achievement-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/achievement-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Achievement,
                extension: Some(NftTypeExtension::Achievement(AchievementExtension {
                    achievement_id: "season-one".to_string(),
                    achievement_type: "quest_completion".to_string(),
                    points: 250,
                    soulbound: true,
                })),
            }),
        },
    )
    .unwrap();

    let nft_info_bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NftInfo {
            token_id: "achievement-1".to_string(),
        },
    )
    .unwrap();
    let nft_info: NftInfoResponse<Extension> = from_json(&nft_info_bin).unwrap();
    let metadata = nft_info.extension.expect("metadata");

    assert_eq!(metadata.nft_type, NftType::Achievement);
}

#[test]
fn mint_accepts_world_template_metadata() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::WorldTemplate, None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint {
            token_id: "world-template-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/world-template-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::WorldTemplate,
                extension: Some(NftTypeExtension::WorldTemplate(WorldTemplateExtension {
                    template_id: "cyberpunk-district".to_string(),
                    category: "cityscape".to_string(),
                })),
            }),
        },
    )
    .unwrap();

    let nft_info_bin = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NftInfo {
            token_id: "world-template-1".to_string(),
        },
    )
    .unwrap();
    let nft_info: NftInfoResponse<Extension> = from_json(&nft_info_bin).unwrap();
    let metadata = nft_info.extension.expect("metadata");

    assert_eq!(metadata.nft_type, NftType::WorldTemplate);
}
