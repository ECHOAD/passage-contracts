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

    #[error("Insufficient points: have {have}, need {need}")]
    InsufficientPoints { have: u128, need: u128 },

    #[error("Invalid payment: expected {expected} {denom}")]
    InvalidPayment { expected: String, denom: String },

    #[error("No payment sent")]
    NoPayment {},

    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: u64 },

    #[error("Session already stopped: {session_id}")]
    SessionAlreadyStopped { session_id: u64 },

    #[error("Session still active: {session_id}")]
    SessionStillActive { session_id: u64 },

    #[error("World config not found: {world_nft_id}")]
    WorldConfigNotFound { world_nft_id: String },

    #[error("World not active: {world_nft_id}")]
    WorldNotActive { world_nft_id: String },

    #[error("Invalid duration: {duration_seconds} seconds")]
    InvalidDuration { duration_seconds: u64 },

    #[error("User balance not found: {user}")]
    UserBalanceNotFound { user: String },

    #[error("No pending revenue for world: {world_nft_id}")]
    NoPendingRevenue { world_nft_id: String },

    #[error("Invalid conversion rate")]
    InvalidConversionRate {},

    #[error("Fiat oracle not configured")]
    FiatOracleNotConfigured {},

    #[error("Invalid fiat purchase: {reason}")]
    InvalidFiatPurchase { reason: String },

    #[error("Duplicate transaction: {transaction_id}")]
    DuplicateTransaction { transaction_id: String },

    #[error("Withdrawal amount too small")]
    WithdrawalTooSmall {},

    #[error("Invalid world owner")]
    InvalidWorldOwner {},

    #[error("Invalid world collection: {reason}")]
    InvalidWorldCollection { reason: String },
}
