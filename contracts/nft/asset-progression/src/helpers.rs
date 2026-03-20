use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, Env};

use crate::{
    error::ContractError,
    state::AssetKind,
};

#[cw_serde]
pub enum Cw721QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    NftInfo {
        token_id: String,
    },
}

#[cw_serde]
pub struct OwnerOfResponse {
    pub owner: String,
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct Approval {
    pub spender: String,
    pub expires: Expiration,
}

#[cw_serde]
pub enum Expiration {
    AtHeight(u64),
    AtTime(cosmwasm_std::Timestamp),
    Never {},
}

#[cw_serde]
pub struct NftInfoResponse<T> {
    pub token_uri: Option<String>,
    pub extension: T,
}

#[cw_serde]
pub struct TokenMetadata {
    pub nft_type: NftType,
}

#[cw_serde]
pub enum NftType {
    Component,
    Avatar,
    Companion,
    World,
    Plugin,
    Achievement,
    WorldTemplate,
}

pub fn assert_sender_can_save_snapshot(
    _deps: Deps,
    _env: &Env,
    _collection: &Addr,
    _token_id: &str,
    _asset_kind: &AssetKind,
    _sender: &Addr,
) -> Result<(), ContractError> {
    Err(ContractError::Unauthorized {})
}
