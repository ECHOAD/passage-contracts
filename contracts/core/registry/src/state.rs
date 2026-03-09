use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contract configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address with full control
    pub admin: Addr,
    /// Optional operator addresses that can register on behalf of creators
    pub operators: Vec<Addr>,
    /// Optional ecosystem factory contract allowed to register approved ecosystems
    pub ecosystem_factory: Option<Addr>,
    /// Whether registration is paused
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
    pub detail: String,
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
}

impl<'a> IndexList<Collection> for CollectionIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Collection>> + '_> {
        let v: Vec<&dyn Index<Collection>> = vec![&self.ecosystem, &self.creator];
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

/// Approved creators allowed to register ecosystems.
/// Key: creator address
pub const APPROVED_ECOSYSTEM_CREATORS: Map<Addr, bool> = Map::new("approved_ecosystem_creators");

/// Approved members allowed to register collections in an ecosystem.
/// Key: (ecosystem_id, member_address)
pub type EcosystemMemberKey = (String, Addr);
pub const ECOSYSTEM_MEMBERS: Map<EcosystemMemberKey, bool> = Map::new("ecosystem_members");

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemCreationRequestStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EcosystemCreationRequest {
    pub request_id: u64,
    pub creator: Addr,
    pub id: String,
    pub name: String,
    pub ecosystem_type: EcosystemType,
    pub collection_creation_policy: CollectionCreationPolicy,
    pub collection_factory: Option<Addr>,
    pub detail: String,
    pub image_urls: Vec<String>,
    pub animation_url: Option<String>,
    pub url: Option<String>,
    pub status: EcosystemCreationRequestStatus,
    pub submitted_at: u64,
    pub reviewed_at: Option<u64>,
    pub reviewed_by: Option<Addr>,
    pub review_note: Option<String>,
}

pub const ECOSYSTEM_CREATION_REQUESTS: Map<u64, EcosystemCreationRequest> =
    Map::new("ecosystem_creation_requests");
pub const PENDING_ECOSYSTEM_REQUEST_BY_ID: Map<String, u64> =
    Map::new("pending_ecosystem_request_by_id");
pub const NEXT_ECOSYSTEM_REQUEST_ID: Item<u64> = Item::new("next_ecosystem_request_id");

/// Counter for generating unique IDs
pub const ECOSYSTEM_COUNT: Item<u64> = Item::new("ecosystem_count");

/// Recovery configuration for dead project governance flow.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RecoveryConfig {
    /// Inactivity signal window used for governance context.
    /// It no longer blocks opening a case by itself.
    pub inactivity_period_secs: u64,
    /// Contest window after case creation.
    pub contest_period_secs: u64,
}

pub const RECOVERY_CONFIG: Item<RecoveryConfig> = Item::new("recovery_config");

/// Tracks last known on-chain activity timestamp per creator/admin address.
pub const LAST_CREATOR_ACTIVITY: Map<Addr, u64> = Map::new("last_creator_activity");

/// Recovery target for a dead project case.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryTarget {
    Ecosystem { ecosystem_id: String },
    Collection { address: Addr },
}

/// Lifecycle of a dead project case.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeadProjectStatus {
    Open,
    Contested,
    Resolved,
}

/// Governance case data for dead project recovery.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DeadProjectCase {
    pub case_id: u64,
    pub target: RecoveryTarget,
    pub target_admin: Addr,
    pub reporter: Addr,
    pub reason: String,
    pub evidence_url: Option<String>,
    pub proposed_replacement: Option<Addr>,
    pub last_target_activity_at: u64,
    pub status: DeadProjectStatus,
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

pub const DEAD_PROJECT_CASES: Map<u64, DeadProjectCase> = Map::new("dead_project_cases");
pub const DEAD_PROJECT_CASE_COUNT: Item<u64> = Item::new("dead_project_case_count");
