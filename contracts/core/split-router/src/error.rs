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

    #[error("Split is inactive")]
    SplitInactive {},

    #[error("No funds sent")]
    NoFundsSent {},

    #[error("Split must have at least one recipient")]
    NoRecipients {},

    #[error("Recipient share cannot be zero: {address}")]
    ZeroShare { address: String },

    #[error("Duplicate recipient address: {address}")]
    DuplicateRecipient { address: String },

    #[error("Recipient shares must sum to {expected}, got {actual}")]
    InvalidTotalShare { expected: Decimal, actual: Decimal },
}
