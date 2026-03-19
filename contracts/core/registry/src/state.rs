use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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
pub struct Config {
    pub admin: Addr,
    pub operators: Vec<Addr>,
    pub recovery_council: Vec<Addr>,
    pub ecosystem_factory: Option<Addr>,
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemType {
    Public,
    Private,
}

impl Default for EcosystemType {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectionCreationPolicy {
    Open,
    Permissioned,
}

impl CollectionCreationPolicy {
    pub fn default_for_type(ecosystem_type: &EcosystemType) -> Self {
        match ecosystem_type {
            EcosystemType::Public => Self::Open,
            EcosystemType::Private => Self::Permissioned,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ecosystem {
    pub id: String,
    pub name: String,
    pub admin: Addr,
    pub ecosystem_type: EcosystemType,
    pub collection_creation_policy: CollectionCreationPolicy,
    pub collection_factory: Option<Addr>,
    pub description: String,
    pub image_urls: Vec<String>,
    pub animation_url: Option<String>,
    pub url: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub type EcosystemKey = String;
pub const ECOSYSTEMS: Map<EcosystemKey, Ecosystem> = Map::new("ecosystems");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    pub address: Addr,
    pub ecosystem_id: Option<String>,
    pub name: String,
    pub nft_type: NftType,
    pub creator: Addr,
    pub verified: bool,
    pub minter: Option<Addr>,
    pub marketplace: Option<Addr>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub type CollectionKey = Addr;

pub struct CollectionIndices<'a> {
    pub ecosystem: MultiIndex<'a, String, Collection, CollectionKey>,
    pub creator: MultiIndex<'a, Addr, Collection, CollectionKey>,
    pub nft_type: MultiIndex<'a, String, Collection, CollectionKey>,
}

impl<'a> IndexList<Collection> for CollectionIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Collection>> + '_> {
        let v: Vec<&dyn Index<Collection>> = vec![&self.ecosystem, &self.creator, &self.nft_type];
        Box::new(v.into_iter())
    }
}

pub fn collections<'a>() -> IndexedMap<CollectionKey, Collection, CollectionIndices<'a>> {
    let indexes = CollectionIndices {
        ecosystem: MultiIndex::new(
            |_pk: &[u8], d: &Collection| d.ecosystem_id.clone().unwrap_or_default(),
            "collections",
            "collections__ecosystem",
        ),
        creator: MultiIndex::new(
            |_pk: &[u8], d: &Collection| d.creator.clone(),
            "collections",
            "collections__creator",
        ),
        nft_type: MultiIndex::new(
            |_pk: &[u8], d: &Collection| d.nft_type.as_str().to_string(),
            "collections",
            "collections__nft_type",
        ),
    };
    IndexedMap::new("collections", indexes)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AuthorizedMinter {
    pub minter_address: Addr,
    pub collection_address: Addr,
    pub authorized_by: Addr,
    pub created_at: u64,
}

pub type MinterKey = (Addr, Addr);
pub const AUTHORIZED_MINTERS: Map<MinterKey, AuthorizedMinter> = Map::new("auth_minters");

pub type EcosystemMemberKey = (String, Addr);
pub const ECOSYSTEM_MEMBERS: Map<EcosystemMemberKey, bool> = Map::new("ecosystem_members");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreatorModeration {
    pub ecosystem_creation_enabled: bool,
    pub collection_creation_enabled: bool,
    pub mint_enabled: bool,
    pub trade_enabled: bool,
    pub reason: Option<String>,
    pub moderated_by: Option<Addr>,
    pub moderated_at: Option<u64>,
}

pub const CREATOR_MODERATION: Map<Addr, CreatorModeration> = Map::new("creator_moderation");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EcosystemModeration {
    pub collection_creation_enabled: bool,
    pub mint_enabled: bool,
    pub trade_enabled: bool,
    pub reason: Option<String>,
    pub moderated_by: Option<Addr>,
    pub moderated_at: Option<u64>,
}

pub const ECOSYSTEM_MODERATION: Map<String, EcosystemModeration> = Map::new("ecosystem_moderation");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionModeration {
    pub mint_enabled: bool,
    pub trade_enabled: bool,
    pub reason: Option<String>,
    pub moderated_by: Option<Addr>,
    pub moderated_at: Option<u64>,
}

pub const COLLECTION_MODERATION: Map<Addr, CollectionModeration> = Map::new("collection_moderation");

pub const ECOSYSTEM_COUNT: Item<u64> = Item::new("ecosystem_count");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryConfig {
    pub abandonment_inactivity_period_secs: u64,
    pub contest_period_secs: u64,
}

pub const RECOVERY_CONFIG: Item<RecoveryConfig> = Item::new("recovery_config");
pub const LAST_CREATOR_ACTIVITY: Map<Addr, u64> = Map::new("last_creator_activity");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryPolicy {
    pub delegate: Option<Addr>,
    pub designated_successor: Option<Addr>,
    pub updated_by: Option<Addr>,
    pub updated_at: Option<u64>,
}

pub const ECOSYSTEM_RECOVERY_POLICIES: Map<String, RecoveryPolicy> = Map::new("ecosystem_recovery_policies");
pub const COLLECTION_RECOVERY_POLICIES: Map<Addr, RecoveryPolicy> = Map::new("collection_recovery_policies");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryTarget {
    Ecosystem { ecosystem_id: String },
    Collection { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCaseKind {
    LostAccess,
    Abandonment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCaseStatus {
    Open,
    Contested,
    Resolved,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryCase {
    pub case_id: u64,
    pub kind: RecoveryCaseKind,
    pub target: RecoveryTarget,
    pub target_admin: Addr,
    pub opened_by: Addr,
    pub reason: String,
    pub evidence_url: Option<String>,
    pub replacement_candidate: Addr,
    pub last_target_activity_at: u64,
    pub status: RecoveryCaseStatus,
    pub opened_at: u64,
    pub contest_deadline: u64,
    pub contested_at: Option<u64>,
    pub contested_by: Option<Addr>,
    pub contest_note: Option<String>,
    pub resolved_at: Option<u64>,
    pub resolved_by: Option<Addr>,
    pub resolution_approved: Option<bool>,
    pub resolution_note: Option<String>,
}

pub const RECOVERY_CASES: Map<u64, RecoveryCase> = Map::new("recovery_cases");
pub const RECOVERY_CASE_COUNT: Item<u64> = Item::new("recovery_case_count");
