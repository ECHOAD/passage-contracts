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

    #[error("Description cannot be empty")]
    EmptyDescription {},

    #[error("At least one image is required")]
    EmptyImages {},

    #[error("Only ecosystem admin can perform this action")]
    NotEcosystemAdmin {},

    #[error("Creator cannot create ecosystems: {creator}")]
    CreatorCannotCreateEcosystem { creator: String },

    #[error("Address is not approved as member for ecosystem: {ecosystem_id}")]
    EcosystemMemberNotApproved { ecosystem_id: String },

    #[error("Collection creation is disabled for ecosystem: {ecosystem_id}")]
    CollectionCreationDisabled { ecosystem_id: String },

    #[error("Minting is disabled for collection: {address}")]
    CollectionMintDisabled { address: String },

    #[error("Trading is disabled for collection: {address}")]
    CollectionTradeDisabled { address: String },

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

    #[error("Invalid validator operator address: {operator_address}")]
    InvalidValidatorOperatorAddress { operator_address: String },

    #[error("Validator moniker cannot be empty")]
    EmptyValidatorMoniker {},

    #[error("Staking validator not found: {operator_address}")]
    StakingValidatorNotFound { operator_address: String },

    #[error("Recovery configuration is invalid")]
    InvalidRecoveryConfig {},

    #[error("Recovery reason cannot be empty")]
    EmptyRecoveryReason {},

    #[error("Recovery case not found: {case_id}")]
    RecoveryCaseNotFound { case_id: u64 },

    #[error("There is already an open recovery case for target: {target}")]
    RecoveryCaseAlreadyOpen { target: String },

    #[error("Recovery case already resolved: {case_id}")]
    RecoveryCaseAlreadyResolved { case_id: u64 },

    #[error("Recovery case is not contestable: {case_id}")]
    RecoveryCaseNotContestable { case_id: u64 },

    #[error("Recovery case is still in its contest window: {case_id}")]
    RecoveryCaseContestWindowOpen { case_id: u64 },

    #[error("Only the target admin can contest a recovery case")]
    OnlyTargetAdminCanContestRecoveryCase {},

    #[error("Opening a recovery case requires a replacement candidate or designated successor")]
    RecoveryReplacementRequired {},

    #[error("Abandonment threshold not met for target: {target}")]
    RecoveryAbandonmentThresholdNotMet { target: String },

    #[error("Lost-access recovery is not configured for target: {target}")]
    LostAccessRecoveryNotConfigured { target: String },
}
