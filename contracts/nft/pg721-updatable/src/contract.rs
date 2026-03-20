#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw721::ContractInfoResponse;
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg};
use cw_utils::nonpayable;
use url::Url;

use crate::msg::{
    CollectionInfoResponse, ExecuteMsg, Extension, FrozenTokenMetadataResponse, InstantiateMsg,
    NftType, NftTypeExtension, PassageProfileId, QueryMsg, RoyaltyInfoResponse, TokenMetadata,
};
use crate::state::{CollectionInfo, RoyaltyInfo, COLLECTION_INFO, FROZEN_TOKEN_METADATA};
use crate::ContractError;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pg-721-updatable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_DESCRIPTION_LENGTH: u32 = 512;

pub type Pg721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // cw721 instantiation
    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };
    Pg721Contract::default()
        .contract_info
        .save(deps.storage, &info)?;

    let minter = deps.api.addr_validate(&msg.minter)?;
    Pg721Contract::default()
        .minter
        .save(deps.storage, &minter)?;

    // pg721 instantiation
    if msg.collection_info.description.len() > MAX_DESCRIPTION_LENGTH as usize {
        return Err(ContractError::DescriptionTooLong {});
    }

    let image = Url::parse(&msg.collection_info.image)?;

    if let Some(ref external_link) = msg.collection_info.external_link {
        Url::parse(external_link)?;
    }

    let royalty_info: Option<RoyaltyInfo> = match msg.collection_info.royalty_info {
        Some(royalty_info) => Some(RoyaltyInfo {
            payment_address: deps.api.addr_validate(&royalty_info.payment_address)?,
            share: royalty_info.share_validate()?,
        }),
        None => None,
    };

    deps.api.addr_validate(&msg.collection_info.creator)?;

    let collection_info = CollectionInfo {
        nft_type: msg.nft_type,
        creator: msg.collection_info.creator,
        description: msg.collection_info.description,
        image: msg.collection_info.image,
        external_link: msg.collection_info.external_link,
        royalty_info,
    };

    COLLECTION_INFO.save(deps.storage, &collection_info)?;
    FROZEN_TOKEN_METADATA.save(deps.storage, &false)?;

    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("image", image.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::FreezeTokenMetadata {} => execute_freeze_token_metadata(deps, info),
        ExecuteMsg::UpdateTokenMetadata {
            token_id,
            token_uri,
        } => execute_update_token_metadata(deps, info, token_id, token_uri),
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
            extension,
        } => {
            validate_token_metadata(deps.as_ref(), extension.as_ref())?;

            Pg721Contract::default()
                .execute(
                    deps,
                    env,
                    info,
                    Cw721ExecuteMsg::Mint(Cw721MintMsg {
                        token_id,
                        owner,
                        token_uri,
                        extension,
                    }),
                )
                .map_err(ContractError::from)
        }
        other => Pg721Contract::default()
            .execute(deps, env, info, other.into())
            .map_err(ContractError::from),
    }
}

fn execute_freeze_token_metadata(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    assert_collection_creator(deps.as_ref(), &info.sender)?;

    FROZEN_TOKEN_METADATA.save(deps.storage, &true)?;

    Ok(Response::new()
        .add_attribute("action", "freeze_token_metadata")
        .add_attribute("frozen", "true"))
}

fn execute_update_token_metadata(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    token_uri: Option<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    assert_collection_creator(deps.as_ref(), &info.sender)?;

    if FROZEN_TOKEN_METADATA.load(deps.storage)? {
        return Err(ContractError::TokenMetadataFrozen {});
    }

    Pg721Contract::default()
        .tokens
        .update(deps.storage, &token_id, |token| match token {
            Some(mut token_info) => {
                token_info.token_uri = token_uri.clone();
                Ok(token_info)
            }
            None => Err(ContractError::TokenNotFound {}),
        })?;

    let token_uri_attr = token_uri.unwrap_or_default();

    Ok(Response::new()
        .add_attribute("action", "update_token_metadata")
        .add_attribute("token_id", token_id)
        .add_attribute("token_uri", token_uri_attr))
}

fn assert_collection_creator(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let collection_info = COLLECTION_INFO.load(deps.storage)?;
    let creator = deps.api.addr_validate(&collection_info.creator)?;

    if creator != *sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CollectionInfo {} => to_json_binary(&query_config(deps)?),
        QueryMsg::FrozenTokenMetadata {} => to_json_binary(&query_frozen_token_metadata(deps)?),
        _ => Pg721Contract::default().query(deps, env, msg.into()),
    }
}

fn query_config(deps: Deps) -> StdResult<CollectionInfoResponse> {
    let info = COLLECTION_INFO.load(deps.storage)?;

    let royalty_info_res: Option<RoyaltyInfoResponse> = match info.royalty_info {
        Some(royalty_info) => Some(RoyaltyInfoResponse {
            payment_address: royalty_info.payment_address.to_string(),
            share: royalty_info.share,
        }),
        None => None,
    };

    Ok(CollectionInfoResponse {
        nft_type: info.nft_type,
        creator: info.creator,
        description: info.description,
        image: info.image,
        external_link: info.external_link,
        royalty_info: royalty_info_res,
    })
}

fn query_frozen_token_metadata(deps: Deps) -> StdResult<FrozenTokenMetadataResponse> {
    let frozen = FROZEN_TOKEN_METADATA.load(deps.storage)?;
    Ok(FrozenTokenMetadataResponse { frozen })
}

fn validate_token_metadata(
    deps: Deps,
    metadata: Option<&TokenMetadata>,
) -> Result<(), ContractError> {
    let Some(metadata) = metadata else {
        return Ok(());
    };

    let collection_info = COLLECTION_INFO.load(deps.storage)?;
    if metadata.nft_type != collection_info.nft_type {
        return Err(ContractError::NftTypeMismatch {
            expected: collection_info.nft_type.to_string(),
            found: metadata.nft_type.to_string(),
        });
    }

    if let Some(extension) = &metadata.extension {
        let extension_type = extension.nft_type();
        if extension_type != collection_info.nft_type {
            return Err(ContractError::NftTypeExtensionMismatch {
                expected: collection_info.nft_type.to_string(),
                found: extension_type.to_string(),
            });
        }

        validate_standard_profile_id(&collection_info.nft_type, extension)?;
    } else if matches!(collection_info.nft_type, NftType::Avatar | NftType::Companion) {
        return Err(ContractError::MissingProfileId {
            nft_type: collection_info.nft_type.to_string(),
        });
    }

    Ok(())
}

fn validate_standard_profile_id(
    nft_type: &NftType,
    extension: &NftTypeExtension,
) -> Result<(), ContractError> {
    match (nft_type, extension) {
        (NftType::Avatar, NftTypeExtension::Avatar(extension)) => validate_profile_id(
            nft_type,
            extension.profile_id.as_ref(),
            &PassageProfileId::PassageAvatarV1,
        ),
        (NftType::Companion, NftTypeExtension::Companion(extension)) => validate_profile_id(
            nft_type,
            extension.profile_id.as_ref(),
            &PassageProfileId::PassageCompanionV1,
        ),
        _ => Ok(()),
    }
}

fn validate_profile_id(
    nft_type: &NftType,
    profile_id: Option<&PassageProfileId>,
    expected: &PassageProfileId,
) -> Result<(), ContractError> {
    let Some(profile_id) = profile_id else {
        return Err(ContractError::MissingProfileId {
            nft_type: nft_type.to_string(),
        });
    };

    if profile_id != expected {
        return Err(ContractError::InvalidProfileId {
            nft_type: nft_type.to_string(),
            expected: expected.to_string(),
            found: profile_id.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests;
