#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw721::ContractInfoResponse;
use cw_utils::nonpayable;
use url::Url;

use crate::msg::{
    CollectionInfoResponse, ExecuteMsg, Extension, FrozenTokenMetadataResponse, InstantiateMsg,
    QueryMsg, RoyaltyInfoResponse,
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::CollectionInfo;
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
            collection_info: CollectionInfo {
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
            extension: None,
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
        assert_eq!(
            "https://example.com/external.html",
            value.external_link.unwrap()
        );
        assert_eq!(None, value.royalty_info);

        let frozen_res =
            query(deps.as_ref(), mock_env(), QueryMsg::FrozenTokenMetadata {}).unwrap();
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

        // Freeze token metadata.
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            ExecuteMsg::FreezeTokenMetadata {},
        )
        .unwrap();

        let frozen_res =
            query(deps.as_ref(), mock_env(), QueryMsg::FrozenTokenMetadata {}).unwrap();
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
}
