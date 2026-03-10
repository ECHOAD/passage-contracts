use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract is paused")]
    ContractPaused {},

    #[error("Ecosystem already exists: {id}")]
    EcosystemAlreadyExists { id: String },

    #[error("Ecosystem not found: {id}")]
    EcosystemNotFound { id: String },

    #[error("Collection already registered: {address}")]
    CollectionAlreadyRegistered { address: String },

    #[error("Collection not found: {address}")]
    CollectionNotFound { address: String },

    #[error("Invalid collection contract: {address}")]
    InvalidCollectionContract { address: String },

    #[error("Minter already authorized: {address}")]
    MinterAlreadyAuthorized { address: String },

    #[error("Minter not authorized: {address}")]
    MinterNotAuthorized { address: String },

    #[error("Invalid ID format: {id}")]
    InvalidIdFormat { id: String },

    #[error("Name cannot be empty")]
    EmptyName {},

    #[error("Detail cannot be empty")]
    EmptyDescription {},

    #[error("At least one image is required")]
    EmptyImages {},

    #[error("Only ecosystem admin can perform this action")]
    NotEcosystemAdmin {},

    #[error("Creator is not approved to register ecosystems")]
    EcosystemCreatorNotApproved {},

    #[error("Ecosystem creation request already pending for ecosystem id: {id}")]
    EcosystemCreationRequestAlreadyPending { id: String },

    #[error("Ecosystem creation request not found: {request_id}")]
    EcosystemCreationRequestNotFound { request_id: u64 },

    #[error("Ecosystem creation request already resolved: {request_id}")]
    EcosystemCreationRequestAlreadyResolved { request_id: u64 },

    #[error("Address is not approved as member for ecosystem: {ecosystem_id}")]
    EcosystemMemberNotApproved { ecosystem_id: String },

    #[error("Cross-ecosystem admin cannot manage owner-controlled ecosystem")]
    CrossAdminCannotManageOwnerEcosystem {},

    #[error("Invalid ecosystem policy combination")]
    InvalidEcosystemPolicyCombination {},

    #[error("Ecosystem factory is not configured")]
    EcosystemFactoryNotConfigured {},

    #[error("Ecosystem factory is required")]
    EcosystemFactoryRequired {},

    #[error("Only configured ecosystem factory can perform this action")]
    NotEcosystemFactory {},

    #[error("Ecosystem creation must go through ecosystem-factory when configured")]
    EcosystemFactoryFlowRequired {},

    #[error("Collection creation request already pending for ecosystem: {ecosystem_id}")]
    CollectionCreationRequestAlreadyPending { ecosystem_id: String },

    #[error("Collection creation request not found for ecosystem: {ecosystem_id}")]
    CollectionCreationRequestNotFound { ecosystem_id: String },

    #[error("Collection creation request already resolved for ecosystem: {ecosystem_id}")]
    CollectionCreationRequestAlreadyResolved { ecosystem_id: String },

    #[error("Collection creation requests are not allowed for ecosystem: {ecosystem_id}")]
    CollectionCreationRequestNotAllowed { ecosystem_id: String },

    #[error(
        "Collection registration must go through configured collection-factory for ecosystem: {ecosystem_id}"
    )]
    CollectionFactoryRequired { ecosystem_id: String },

    #[error("Only collection creator can perform this action")]
    NotCollectionCreator {},

    #[error("Recovery configuration is invalid")]
    InvalidRecoveryConfig {},

    #[error("Dead project reason cannot be empty")]
    EmptyRecoveryReason {},

    #[error("Dead project case not found: {case_id}")]
    DeadProjectCaseNotFound { case_id: u64 },

    #[error("There is already an open dead project case for target: {target}")]
    DeadProjectCaseAlreadyOpen { target: String },

    #[error("Dead project case already resolved: {case_id}")]
    DeadProjectCaseAlreadyResolved { case_id: u64 },

    #[error("Dead project case is not contestable: {case_id}")]
    DeadProjectCaseNotContestable { case_id: u64 },

    #[error("Only target admin can contest dead project case")]
    OnlyTargetAdminCanContest {},

    #[error("Approved dead project recovery requires a proposed replacement")]
    RecoveryReplacementRequired {},
}
