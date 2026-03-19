use crate::ContractError;
use cosmwasm_std::{Binary, Decimal};
use cw721::Expiration;
use cw721_base::{
    msg::QueryMsg as Cw721QueryMsg, ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub nft_type: NftType,
    pub collection_info: CollectionInfoMsg<RoyaltyInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfoMsg<T> {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: Decimal,
}

impl RoyaltyInfoResponse {
    pub fn share_validate(&self) -> Result<Decimal, ContractError> {
        if self.share > Decimal::one() {
            return Err(ContractError::InvalidRoyalities {});
        }

        Ok(self.share)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

impl fmt::Display for NftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NativeAsset {
    pub asset_id: String,
    pub name: String,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RevenueShare {
    pub address: String,
    pub share: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct ComponentExtension {
    pub component_id: String,
    pub compatible_skeletons: Vec<String>,
    pub compatible_slots: Vec<String>,
    pub component_type: String,
    pub license: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct AvatarExtension {
    pub avatar_id: String,
    pub skeleton_type: String,
    pub slot_schema_uri: String,
    pub equipment_state_uri: String,
    pub equipment_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct CompanionExtension {
    pub companion_id: String,
    pub level: u32,
    pub experience: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct WorldExtension {
    pub world_id: String,
    pub revenue_shares: Vec<RevenueShare>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct PluginExtension {
    pub plugin_id: String,
    pub plugin_type: String,
    pub license: String,
    pub permissions_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct AchievementExtension {
    pub achievement_id: String,
    pub achievement_type: String,
    pub points: u32,
    pub soulbound: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct WorldTemplateExtension {
    pub template_id: String,
    pub category: String,
    pub branding_uri: Option<String>,
    pub customization_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftTypeExtension {
    Component(ComponentExtension),
    Avatar(AvatarExtension),
    Companion(CompanionExtension),
    World(WorldExtension),
    Plugin(PluginExtension),
    Achievement(AchievementExtension),
    WorldTemplate(WorldTemplateExtension),
}

impl NftTypeExtension {
    pub fn nft_type(&self) -> NftType {
        match self {
            Self::Component(_) => NftType::Component,
            Self::Avatar(_) => NftType::Avatar,
            Self::Companion(_) => NftType::Companion,
            Self::World(_) => NftType::World,
            Self::Plugin(_) => NftType::Plugin,
            Self::Achievement(_) => NftType::Achievement,
            Self::WorldTemplate(_) => NftType::WorldTemplate,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMetadata {
    pub nft_type: NftType,
    /// Native dependents included with this NFT.
    pub native_assets: Option<Vec<NativeAsset>>,
    /// Type-specific Passage metadata.
    pub extension: Option<NftTypeExtension>,
}

pub type Extension = Option<TokenMetadata>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Freeze token metadata so creator can no longer update token URIs.
    FreezeTokenMetadata {},
    /// Creator can update token_uri while not frozen.
    UpdateTokenMetadata {
        token_id: String,
        token_uri: Option<String>,
    },
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Extension,
    },
    Burn {
        token_id: String,
    },
}

impl From<ExecuteMsg> for Cw721ExecuteMsg<Extension> {
    fn from(msg: ExecuteMsg) -> Cw721ExecuteMsg<Extension> {
        match msg {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                Cw721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721ExecuteMsg::RevokeAll { operator },
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => Cw721ExecuteMsg::Mint(Cw721MintMsg {
                token_id,
                owner,
                token_uri,
                extension,
            }),
            ExecuteMsg::Burn { token_id } => Cw721ExecuteMsg::Burn { token_id },
            _ => unreachable!("invalid ExecuteMsg conversion to Cw721ExecuteMsg"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    FrozenTokenMetadata {},
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
    CollectionInfo {},
}

impl From<QueryMsg> for Cw721QueryMsg {
    fn from(msg: QueryMsg) -> Cw721QueryMsg {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            _ => unreachable!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionInfoResponse {
    pub nft_type: NftType,
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<RoyaltyInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FrozenTokenMetadataResponse {
    pub frozen: bool,
}
