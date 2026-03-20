use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use cw_utils::PaymentError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw721(#[from] Cw721ContractError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidCreationFee")]
    InvalidCreationFee {},

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Invalid Royalities")]
    InvalidRoyalities {},

    #[error("Description too long")]
    DescriptionTooLong {},

    #[error("Token metadata nft_type `{found}` does not match collection nft_type `{expected}`")]
    NftTypeMismatch { expected: String, found: String },

    #[error("Token metadata extension `{found}` does not match collection nft_type `{expected}`")]
    NftTypeExtensionMismatch { expected: String, found: String },

    #[error("Token metadata for `{nft_type}` requires a standardized Passage profile_id")]
    MissingProfileId { nft_type: String },

    #[error("Token metadata for `{nft_type}` must use standardized profile_id `{expected}`, found `{found}`")]
    InvalidProfileId {
        nft_type: String,
        expected: String,
        found: String,
    },

    #[error("Token metadata is frozen")]
    TokenMetadataFrozen {},

    #[error("Token not found")]
    TokenNotFound {},

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Parse(#[from] ParseError),
}
