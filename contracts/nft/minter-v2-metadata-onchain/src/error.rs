use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Minting is paused")]
    MintingPaused {},

    #[error("Minting has not started yet. Start time: {start_time}")]
    MintingNotStarted { start_time: String },

    #[error("Sold out - no tokens remaining")]
    SoldOut {},

    #[error("Invalid payment amount. Expected {expected}, got {received}")]
    InvalidPayment { expected: String, received: String },

    #[error("Invalid payment denom. Expected {expected}, got {received}")]
    InvalidPaymentDenom { expected: String, received: String },

    #[error("Address has reached mint limit: {limit}")]
    MaxMintLimitReached { limit: u32 },

    #[error("Whitelist minting is active but address is not whitelisted")]
    NotWhitelisted {},

    #[error("Whitelist period has ended")]
    WhitelistEnded {},

    #[error("Token ID {token_id} is not available for minting")]
    TokenNotAvailable { token_id: u32 },

    #[error("Invalid token ID: {token_id}")]
    InvalidTokenId { token_id: u32 },

    #[error("Invalid native asset data: {reason}")]
    InvalidNativeAsset { reason: String },

    #[error("Metadata mode is locked to OnChain in this contract")]
    MetadataModeLockedToOnChain {},

    #[error("Batch mint count exceeds available tokens")]
    BatchExceedsAvailable {},

    #[error("Batch mint count exceeds per-address limit")]
    BatchExceedsLimit {},

    #[error("No funds to withdraw")]
    NoFundsToWithdraw {},

    #[error("Cannot withdraw when using Split Router")]
    CannotWithdrawWithSplitRouter {},

    #[error("Split Router not configured")]
    SplitRouterNotConfigured {},

    #[error("Collection not registered in Registry")]
    CollectionNotRegistered {},

    #[error("Minter not authorized for this collection")]
    MinterNotAuthorized {},

    #[error("Invalid instantiate reply data")]
    InvalidInstantiateReplyData {},

    #[error("NFT contract instantiation failed")]
    NftInstantiateFailed {},
}
