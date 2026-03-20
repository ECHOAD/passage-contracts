use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, Env};

use crate::{error::ContractError, state::AssetKind};

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

impl NftType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::Avatar => "avatar",
            Self::Companion => "companion",
            Self::World => "world",
            Self::Plugin => "plugin",
            Self::Achievement => "achievement",
            Self::WorldTemplate => "world_template",
        }
    }
}

pub fn assert_sender_can_save_snapshot(
    deps: Deps,
    env: &Env,
    collection: &Addr,
    token_id: &str,
    asset_kind: &AssetKind,
    sender: &Addr,
) -> Result<(), ContractError> {
    let owner: OwnerOfResponse = deps.querier.query_wasm_smart(
        collection,
        &Cw721QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: Some(false),
        },
    )?;
    let token_info: NftInfoResponse<TokenMetadata> = deps.querier.query_wasm_smart(
        collection,
        &Cw721QueryMsg::NftInfo {
            token_id: token_id.to_string(),
        },
    )?;

    let expected = asset_kind.as_str();
    let found = token_info.extension.nft_type.as_str();
    if expected != found {
        return Err(ContractError::UnsupportedAssetKind {
            expected: expected.to_string(),
            found: found.to_string(),
        });
    }

    if owner.owner == sender.as_str() {
        return Ok(());
    }

    let approved = owner
        .approvals
        .iter()
        .any(|approval| approval.spender == sender.as_str() && !approval.is_expired(env));

    if approved {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

impl Approval {
    fn is_expired(&self, env: &Env) -> bool {
        match &self.expires {
            Expiration::AtHeight(height) => env.block.height >= *height,
            Expiration::AtTime(time) => env.block.time >= *time,
            Expiration::Never {} => false,
        }
    }
}
