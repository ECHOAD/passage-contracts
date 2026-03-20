use crate::ContractError;
use cosmwasm_std::Decimal;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;
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
    /// Type-specific Passage metadata.
    pub extension: Option<NftTypeExtension>,
}

pub type Extension = Option<TokenMetadata>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
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
