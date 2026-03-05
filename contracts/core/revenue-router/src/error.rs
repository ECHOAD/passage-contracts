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

    #[error("No funds sent")]
    NoFundsSent {},

    #[error("Invalid funds: expected {expected}, got {received}")]
    InvalidFunds { expected: String, received: String },

    #[error("Distribution rule not found for collection: {collection}")]
    DistributionRuleNotFound { collection: String },

    #[error("Distribution rule already exists for collection: {collection}")]
    DistributionRuleExists { collection: String },

    #[error("Invalid share: must be between 0 and 1")]
    InvalidShare {},

    #[error("Total collaborator shares exceed 100%")]
    CollaboratorSharesExceedLimit {},

    #[error("Platform fee exceeds maximum allowed: {max}%")]
    PlatformFeeExceedsMax { max: String },

    #[error("Split wallet not found: {id}")]
    SplitWalletNotFound { id: String },

    #[error("Split wallet already exists: {id}")]
    SplitWalletExists { id: String },

    #[error("Split wallet has no recipients")]
    NoSplitRecipients {},

    #[error("Total weight cannot be zero")]
    ZeroTotalWeight {},

    #[error("Ecosystem config not found: {ecosystem_id}")]
    EcosystemConfigNotFound { ecosystem_id: String },

    #[error("Collection not registered in registry: {collection}")]
    CollectionNotRegistered { collection: String },

    #[error("Invalid decimal value")]
    InvalidDecimal {},

    #[error("Insufficient funds for distribution")]
    InsufficientFunds {},

    #[error("Only collection creator can set distribution rules")]
    NotCollectionCreator {},

    #[error("Only split wallet admin can modify it")]
    NotSplitWalletAdmin {},
}
