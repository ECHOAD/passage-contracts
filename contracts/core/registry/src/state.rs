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

/// Contract configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address with full control
    pub admin: Addr,
    /// Optional operator addresses that can register on behalf of creators
    pub operators: Vec<Addr>,
    /// Dedicated recovery authority addresses for ownership recovery cases
    pub recovery_council: Vec<Addr>,
    /// Ecosystem factory contract allowed to register approved ecosystems
    pub ecosystem_factory: Option<Addr>,
    /// Whether registration is paused
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Optional Passage-curated validator metadata for chain-native staking UX.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingValidator {
    pub operator_address: String,
    pub moniker: String,
    pub website: Option<String>,
    pub active: bool,
    pub updated_by: Addr,
    pub updated_at: u64,
}

pub const STAKING_VALIDATORS: Map<String, StakingValidator> = Map::new("staking_validators");

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
    ApprovalRequired,
}

impl CollectionCreationPolicy {
    pub fn default_for_type(ecosystem_type: &EcosystemType) -> Self {
        match ecosystem_type {
            EcosystemType::Public => Self::ApprovalRequired,
            EcosystemType::Private => Self::Permissioned,
        }
    }
}

/// Represents an Ecosystem - top level organizational unit
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

/// Primary key for ecosystems: ecosystem_id
pub type EcosystemKey = String;

pub const ECOSYSTEMS: Map<EcosystemKey, Ecosystem> = Map::new("ecosystems");

/// Represents a Collection (pg721) registered within an Ecosystem
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    /// The pg721 contract address
    pub address: Addr,
    pub ecosystem_id: String,
    pub name: String,
    pub nft_type: NftType,
    pub creator: Addr,
    /// Whether this collection is verified/official
    pub verified: bool,
    /// Optional authorized minter address
    pub minter: Option<Addr>,
    /// Optional marketplace address
    pub marketplace: Option<Addr>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Primary key for collections: contract address
pub type CollectionKey = Addr;

/// Indices for Collection
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
            |_pk: &[u8], d: &Collection| d.ecosystem_id.clone(),
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

/// Authorized minter registration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AuthorizedMinter {
    pub minter_address: Addr,
    pub collection_address: Addr,
    pub authorized_by: Addr,
    pub created_at: u64,
}

/// Primary key: (collection_address, minter_address)
pub type MinterKey = (Addr, Addr);

pub const AUTHORIZED_MINTERS: Map<MinterKey, AuthorizedMinter> = Map::new("auth_minters");

/// Approved members allowed to register collections in an ecosystem.
/// Key: (ecosystem_id, member_address)
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

pub const COLLECTION_MODERATION: Map<Addr, CollectionModeration> =
    Map::new("collection_moderation");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectionCreationRequestStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionCreationRequest {
    pub ecosystem_id: String,
    pub creator: Addr,
    pub note: Option<String>,
    pub status: CollectionCreationRequestStatus,
    pub submitted_at: u64,
    pub reviewed_at: Option<u64>,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
}

/// Key: (ecosystem_id, creator)
pub type CollectionCreationRequestKey = (String, Addr);
pub const COLLECTION_CREATION_REQUESTS: Map<
    CollectionCreationRequestKey,
    CollectionCreationRequest,
> = Map::new("collection_creation_requests");

/// Counter for generating unique IDs
pub const ECOSYSTEM_COUNT: Item<u64> = Item::new("ecosystem_count");

/// Recovery configuration for ownership recovery governance flow.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryConfig {
    /// Inactivity signal window used only for abandonment cases.
    pub abandonment_inactivity_period_secs: u64,
    /// Contest window after case creation.
    pub contest_period_secs: u64,
}

pub const RECOVERY_CONFIG: Item<RecoveryConfig> = Item::new("recovery_config");

/// Tracks last known on-chain activity timestamp per creator/admin address.
pub const LAST_CREATOR_ACTIVITY: Map<Addr, u64> = Map::new("last_creator_activity");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryPolicy {
    pub delegate: Option<Addr>,
    pub designated_successor: Option<Addr>,
    pub updated_by: Option<Addr>,
    pub updated_at: Option<u64>,
}

pub const ECOSYSTEM_RECOVERY_POLICIES: Map<String, RecoveryPolicy> =
    Map::new("ecosystem_recovery_policies");
pub const COLLECTION_RECOVERY_POLICIES: Map<Addr, RecoveryPolicy> =
    Map::new("collection_recovery_policies");

/// Recovery target for an ownership recovery case.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryTarget {
    Ecosystem { ecosystem_id: String },
    Collection { address: Addr },
}

/// Ownership recovery case type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCaseKind {
    LostAccess,
    Abandonment,
}

/// Lifecycle of an ownership recovery case.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCaseStatus {
    Open,
    Contested,
    Resolved,
}

/// Governance case data for ownership recovery.
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
