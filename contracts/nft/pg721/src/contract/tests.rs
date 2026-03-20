use super::*;

use crate::msg::{
    AchievementExtension, AvatarExtension, CollectionInfoMsg, CompanionExtension,
    ComponentExtension, NftType, NftTypeExtension, PassageProfileId, PluginExtension,
    WorldTemplateExtension,
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

#[test]
fn mint_rejects_avatar_without_profile_id() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Avatar, None);

    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint(cw721_base::MintMsg {
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
        }),
    )
    .unwrap_err();

    assert_eq!(
        err.to_string(),
        ContractError::MissingProfileId {
            nft_type: "avatar".to_string(),
        }
        .to_string()
    );
}

#[test]
fn mint_accepts_avatar_with_standard_profile_id() {
    let mut deps = mock_dependencies();
    setup_contract_with_type(deps.as_mut(), NftType::Avatar, None);

    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("minter", &[]),
        ExecuteMsg::Mint(cw721_base::MintMsg {
            token_id: "avatar-1".to_string(),
            owner: "owner".to_string(),
            token_uri: Some("ipfs://cid/avatar-1".to_string()),
            extension: Some(TokenMetadata {
                nft_type: NftType::Avatar,
                extension: Some(NftTypeExtension::Avatar(AvatarExtension {
                    avatar_id: "avatar-1".to_string(),
                    profile_id: Some(PassageProfileId::PassageAvatarV1),
                })),
            }),
        }),
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
        ExecuteMsg::Mint(cw721_base::MintMsg {
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
        }),
    )
    .unwrap_err();

    assert_eq!(
        err.to_string(),
        ContractError::InvalidProfileId {
            nft_type: "companion".to_string(),
            expected: PassageProfileId::PassageCompanionV1.to_string(),
            found: PassageProfileId::PassageAvatarV1.to_string(),
        }
        .to_string()
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
        ExecuteMsg::Mint(cw721_base::MintMsg {
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
        }),
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
        ExecuteMsg::Mint(cw721_base::MintMsg {
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
        }),
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
        ExecuteMsg::Mint(cw721_base::MintMsg {
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
        }),
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
