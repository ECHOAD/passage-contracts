use crate::state::{
    Collection, CollectionCreationPolicy, CollectionModeration, Config, CreatorModeration,
    Ecosystem, EcosystemModeration, EcosystemType, NftType, RecoveryCase, RecoveryCaseKind,
    RecoveryCaseStatus, RecoveryConfig, RecoveryPolicy,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub operators: Option<Vec<String>>,
    pub recovery_council: Option<Vec<String>>,
    pub ecosystem_factory: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        operators: Option<Vec<String>>,
        recovery_council: Option<Vec<String>>,
        ecosystem_factory: Option<String>,
        paused: Option<bool>,
    },
    UpdateCreatorModeration {
        creator: String,
        ecosystem_creation_enabled: Option<bool>,
        collection_creation_enabled: Option<bool>,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    UpdateEcosystemModeration {
        ecosystem_id: String,
        collection_creation_enabled: Option<bool>,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    UpdateCollectionModeration {
        address: String,
        mint_enabled: Option<bool>,
        trade_enabled: Option<bool>,
        reason: Option<String>,
    },
    SetEcosystemRecoveryPolicy {
        ecosystem_id: String,
        delegate: Option<String>,
        designated_successor: Option<String>,
    },
    SetCollectionRecoveryPolicy {
        address: String,
        delegate: Option<String>,
        designated_successor: Option<String>,
    },
    RegisterEcosystemFromFactory {
        id: String,
        name: String,
        creator: String,
        collection_factory: String,
        description: String,
        image_urls: Vec<String>,
        animation_url: Option<String>,
        url: Option<String>,
    },
    UpdateEcosystem {
        id: String,
        name: Option<String>,
        description: Option<String>,
        image_urls: Option<Vec<String>>,
        animation_url: Option<String>,
        url: Option<String>,
        ecosystem_type: Option<EcosystemType>,
        collection_creation_policy: Option<CollectionCreationPolicy>,
        collection_factory: Option<String>,
        admin: Option<String>,
    },
    ApproveEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    RevokeEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    RegisterCollection {
        address: String,
        ecosystem_id: String,
        name: String,
        nft_type: NftType,
    },
    RegisterCollectionFromFactory {
        address: String,
        ecosystem_id: String,
        name: String,
        creator: String,
        nft_type: NftType,
    },
    RegisterExistingCollection {
        address: String,
        ecosystem_id: String,
        name: String,
        creator: String,
        nft_type: NftType,
    },
    DeregisterCollection {
        address: String,
    },
    RehomeCollection {
        address: String,
        ecosystem_id: String,
    },
    UpdateCollection {
        address: String,
        name: Option<String>,
        verified: Option<bool>,
        minter: Option<String>,
        marketplace: Option<String>,
    },
    TransferCollectionOwnership {
        address: String,
        new_creator: String,
    },
    AuthorizeMinter {
        collection_address: String,
        minter_address: String,
    },
    RevokeMinter {
        collection_address: String,
        minter_address: String,
    },
    UpdateRecoveryConfig {
        abandonment_inactivity_period_secs: Option<u64>,
        contest_period_secs: Option<u64>,
    },
    OpenRecoveryCase {
        case_kind: RecoveryCaseKind,
        target: RecoveryTargetInput,
        reason: String,
        evidence_url: Option<String>,
        proposed_replacement: Option<String>,
    },
    ContestRecoveryCase { case_id: u64, note: Option<String> },
    ResolveRecoveryCase {
        case_id: u64,
        approved: bool,
        note: Option<String>,
    },
}

#[cw_serde]
pub enum RecoveryTargetInput {
    Ecosystem { ecosystem_id: String },
    Collection { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(EcosystemResponse)]
    Ecosystem { id: String },
    #[returns(EcosystemsResponse)]
    Ecosystems {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(EcosystemsResponse)]
    EcosystemsByAdmin {
        admin: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CreatorModerationResponse)]
    CreatorModeration { creator: String },
    #[returns(EcosystemModerationResponse)]
    EcosystemModeration { ecosystem_id: String },
    #[returns(RecoveryPolicyResponse)]
    EcosystemRecoveryPolicy { ecosystem_id: String },
    #[returns(CollectionModerationResponse)]
    CollectionModeration { address: String },
    #[returns(RecoveryPolicyResponse)]
    CollectionRecoveryPolicy { address: String },
    #[returns(ApprovalStatusResponse)]
    CanCreateEcosystem { creator: String },
    #[returns(ApprovalStatusResponse)]
    IsCrossEcosystemAdmin { address: String },
    #[returns(ApprovalStatusResponse)]
    IsEcosystemMember {
        ecosystem_id: String,
        member: String,
    },
    #[returns(ApprovalStatusResponse)]
    CanCreateCollectionInEcosystem {
        ecosystem_id: String,
        creator: String,
    },
    #[returns(CollectionResponse)]
    Collection { address: String },
    #[returns(CollectionsResponse)]
    Collections {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    CollectionsByEcosystem {
        ecosystem_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    UnaffiliatedCollections {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    CollectionsByCreator {
        creator: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CollectionsResponse)]
    CollectionsByNftType {
        nft_type: NftType,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(IsVerifiedResponse)]
    IsCollectionVerified { address: String },
    #[returns(ApprovalStatusResponse)]
    CanMintCollection { address: String },
    #[returns(ApprovalStatusResponse)]
    CanTradeCollection { address: String },
    #[returns(IsMinterAuthorizedResponse)]
    IsMinterAuthorized {
        collection_address: String,
        minter_address: String,
    },
    #[returns(AuthorizedMintersResponse)]
    AuthorizedMinters {
        collection_address: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(RecoveryConfigResponse)]
    RecoveryConfig {},
    #[returns(RecoveryCaseResponse)]
    RecoveryCase { case_id: u64 },
    #[returns(RecoveryCasesResponse)]
    RecoveryCases {
        status: Option<RecoveryCaseStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(LastCreatorActivityResponse)]
    LastCreatorActivity { creator: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct EcosystemResponse {
    pub ecosystem: Option<Ecosystem>,
}

#[cw_serde]
pub struct EcosystemsResponse {
    pub ecosystems: Vec<Ecosystem>,
}

#[cw_serde]
pub struct CollectionResponse {
    pub collection: Option<Collection>,
}

#[cw_serde]
pub struct CollectionsResponse {
    pub collections: Vec<Collection>,
}

#[cw_serde]
pub struct IsVerifiedResponse {
    pub is_verified: bool,
}

#[cw_serde]
pub struct IsMinterAuthorizedResponse {
    pub is_authorized: bool,
}

#[cw_serde]
pub struct AuthorizedMintersResponse {
    pub minters: Vec<Addr>,
}

#[cw_serde]
pub struct ApprovalStatusResponse {
    pub approved: bool,
}

#[cw_serde]
pub struct CreatorModerationResponse {
    pub creator: String,
    pub moderation: CreatorModeration,
}

#[cw_serde]
pub struct EcosystemModerationResponse {
    pub ecosystem_id: String,
    pub moderation: EcosystemModeration,
}

#[cw_serde]
pub struct CollectionModerationResponse {
    pub address: String,
    pub moderation: CollectionModeration,
}

#[cw_serde]
pub struct RecoveryPolicyResponse {
    pub policy: RecoveryPolicy,
}

#[cw_serde]
pub struct RecoveryConfigResponse {
    pub config: RecoveryConfig,
}

#[cw_serde]
pub struct RecoveryCaseResponse {
    pub case: Option<RecoveryCase>,
}

#[cw_serde]
pub struct RecoveryCasesResponse {
    pub cases: Vec<RecoveryCase>,
}

#[cw_serde]
pub struct LastCreatorActivityResponse {
    pub creator: String,
    pub last_activity_at: Option<u64>,
}
