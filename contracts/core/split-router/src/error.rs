use cosmwasm_std::{Decimal, StdError};
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

    #[error("Split rule not found for key: {key}")]
    SplitRuleNotFound { key: String },

    #[error("Split rule already exists for key: {key}")]
    SplitRuleExists { key: String },

    #[error("Split rule is inactive for key: {key}")]
    SplitRuleInactive { key: String },

    #[error("Split rule key cannot be empty")]
    EmptyKey {},

    #[error("Split rule must have at least one recipient")]
    NoRecipients {},

    #[error("Recipient share cannot be zero: {address}")]
    ZeroShare { address: String },

    #[error("Duplicate recipient address: {address}")]
    DuplicateRecipient { address: String },

    #[error("Recipient shares must sum to {expected}, got {actual}")]
    InvalidTotalShare { expected: Decimal, actual: Decimal },

    #[error("Only the rule owner or contract admin can modify this split rule")]
    NotRuleOwner {},
}
