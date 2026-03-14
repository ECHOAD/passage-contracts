use crate::state::{CollectionRecord, Config, NftType};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub operators: Option<Vec<String>>,
    pub registry: String,
    pub ecosystem_id: String,
    pub collection_code_id: u64,
    pub enforce_local_allowlist: Option<bool>,
    /// Optional list of wallets initially approved to create collections.
    pub approved_creators: Option<Vec<String>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        operators: Option<Vec<String>>,
        registry: Option<String>,
        ecosystem_id: Option<String>,
        collection_code_id: Option<u64>,
        enforce_local_allowlist: Option<bool>,
        paused: Option<bool>,
    },
    ApproveCreator {
        creator: String,
    },
    RevokeCreator {
        creator: String,
    },
    CreateCollection {
        name: String,
        symbol: String,
        minter: String,
        nft_type: NftType,
        collection_info: CollectionInfoInput,
        label: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ApprovalStatusResponse)]
    IsCreatorApproved { creator: String },
    #[returns(ApprovedCreatorsResponse)]
    ApprovedCreators {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CollectionResponse)]
    Collection { id: u64 },
    #[returns(CollectionsResponse)]
    Collections {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    CollectionsByCreator {
        creator: String,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    CollectionsByNftType {
        nft_type: NftType,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct CollectionInfoInput {
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<RoyaltyInfoInput>,
}

#[cw_serde]
pub struct RoyaltyInfoInput {
    pub payment_address: String,
    pub share: Decimal,
}

// Internal message shape expected by pg721/pg721-updatable instantiate.
#[cw_serde]
pub struct Pg721InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub nft_type: NftType,
    pub collection_info: Pg721CollectionInfo,
}

#[cw_serde]
pub struct Pg721CollectionInfo {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<Pg721RoyaltyInfoResponse>,
}

#[cw_serde]
pub struct Pg721RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: Decimal,
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct ApprovalStatusResponse {
    pub approved: bool,
}

#[cw_serde]
pub enum RegistryQueryMsg {
    Ecosystem {
        id: String,
    },
    IsCrossEcosystemAdmin {
        address: String,
    },
    CanCreateCollectionInEcosystem {
        ecosystem_id: String,
        creator: String,
    },
}

#[cw_serde]
pub struct RegistryApprovalStatusResponse {
    pub approved: bool,
}

#[cw_serde]
pub struct RegistryEcosystem {
    pub admin: String,
}

#[cw_serde]
pub struct RegistryEcosystemResponse {
    pub ecosystem: Option<RegistryEcosystem>,
}

#[cw_serde]
pub enum RegistryExecuteMsg {
    RegisterCollectionFromFactory {
        address: String,
        ecosystem_id: String,
        name: String,
        creator: String,
        nft_type: NftType,
    },
}

#[cw_serde]
pub struct ApprovedCreatorsResponse {
    pub creators: Vec<String>,
}

#[cw_serde]
pub struct CollectionResponse {
    pub collection: Option<CollectionRecord>,
}

#[cw_serde]
pub struct CollectionsResponse {
    pub collections: Vec<CollectionRecord>,
}
