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
    /// Whether registration is paused
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Represents an Ecosystem - top level organizational unit
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ecosystem {
    pub id: String,
    pub name: String,
    pub admin: Addr,
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

/// Counter for generating unique IDs
pub const ECOSYSTEM_COUNT: Item<u64> = Item::new("ecosystem_count");
