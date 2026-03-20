use cosmwasm_std::{StdError, Timestamp};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unsupported asset kind for collection/token: expected {expected}, found {found}")]
    UnsupportedAssetKind { expected: String, found: String },

    #[error("Missing token metadata for progression validation")]
    MissingTokenMetadata {},

    #[error("Snapshot world must not be empty")]
    EmptyWorld {},

    #[error("Approval expired at block height {height}")]
    ApprovalExpiredAtHeight { height: u64 },

    #[error("Approval expired at time {time}")]
    ApprovalExpiredAtTime { time: Timestamp },
}
