#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Response, StdResult,
};
use cw2::{get_contract_version, set_contract_version};

use crate::ContractError;
use cw721::ContractInfoResponse;
use url::Url;

use crate::msg::{
    CollectionInfoResponse, ExecuteMsg, Extension, InstantiateMsg, Metadata, MigrateMsg, QueryMsg,
    RoyaltyInfoResponse,
};
use crate::state::{CollectionInfo, RoyaltyInfo, COLLECTION_INFO};

pub type Pg721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pg-721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_DESCRIPTION_LENGTH: u32 = 512;

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
    Pg721MetadataContract::default()
        .contract_info
        .save(deps.storage, &info)?;

    let minter = deps.api.addr_validate(&msg.minter)?;
    Pg721MetadataContract::default()
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
    if let ExecuteMsg::Mint(mint_msg) = &msg {
        validate_token_metadata(deps.as_ref(), mint_msg.extension.as_ref())?;
    }

    Pg721MetadataContract::default()
        .execute(deps, env, info, msg)
        .map_err(ContractError::from)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CollectionInfo {} => to_json_binary(&query_config(deps)?),
        _ => Pg721MetadataContract::default().query(deps, env, msg.into()),
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let storage_version: &str = &get_contract_version(deps.storage)?.version.to_string();

    let mut response = Response::new();
    if storage_version < CONTRACT_VERSION {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        let minter = deps.api.addr_validate(&msg.minter)?;
        Pg721MetadataContract::default()
            .minter
            .save(deps.storage, &minter)?;

        let event = Event::new("migrate-storage").add_attribute("new-minter", minter.to_string());
        response.events.push(event);
    }

    let event = Event::new("contract-migrated")
        .add_attribute("prev-version", storage_version)
        .add_attribute("next-version", CONTRACT_VERSION);
    response.events.push(event);
    Ok(response)
}

fn validate_token_metadata(deps: Deps, metadata: Option<&Metadata>) -> Result<(), ContractError> {
    let Some(metadata) = metadata else {
        return Ok(());
    };

    let collection_info = COLLECTION_INFO.load(deps.storage)?;

    if let Some(nft_type) = &metadata.nft_type {
        if *nft_type != collection_info.nft_type {
            return Err(ContractError::NftTypeMismatch {
                expected: collection_info.nft_type.to_string(),
                found: nft_type.to_string(),
            });
        }
    }

    if let Some(extension) = &metadata.extension {
        let extension_type = extension.nft_type();
        if extension_type != collection_info.nft_type {
            return Err(ContractError::NftTypeExtensionMismatch {
                expected: collection_info.nft_type.to_string(),
                found: extension_type.to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
