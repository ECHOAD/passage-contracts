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

    #[error("Invalid request id")]
    InvalidRequestId {},

    #[error("Invalid code id for {field}")]
    InvalidCodeId { field: String },

    #[error("Ecosystem id cannot be empty")]
    EmptyEcosystemId {},

    #[error("Ecosystem name cannot be empty")]
    EmptyEcosystemName {},

    #[error("Ecosystem description cannot be empty")]
    EmptyEcosystemDescription {},

    #[error("At least one image is required")]
    EmptyImages {},

    #[error("Creator is not allowed to create ecosystems")]
    EcosystemCreationNotAllowed {},

    #[error("Ecosystem creation request already pending for id: {id}")]
    RequestAlreadyPending { id: String },

    #[error("Ecosystem creation request not found: {request_id}")]
    RequestNotFound { request_id: u64 },

    #[error("Ecosystem creation request already resolved: {request_id}")]
    RequestAlreadyResolved { request_id: u64 },

    #[error("Pending ecosystem creation not found for reply_id: {reply_id}")]
    PendingCreationNotFound { reply_id: u64 },

    #[error("Failed to parse reply data")]
    ReplyParseError {},
}
