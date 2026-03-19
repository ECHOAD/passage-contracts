use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("native_denom cannot be empty")]
    EmptyNativeDenom {},

    #[error("invalid voting period")]
    InvalidVotingPeriod {},

    #[error("invalid bps value")]
    InvalidBps {},

    #[error("proposal deposit must be non-zero")]
    InvalidProposalDeposit {},

    #[error("missing native deposit")]
    MissingDeposit {},

    #[error("wrong deposit denom: expected {expected}, got {actual}")]
    WrongDepositDenom { expected: String, actual: String },

    #[error("insufficient deposited voting power: required {required}, actual {actual}")]
    InsufficientVotingPower { required: Uint128, actual: Uint128 },

    #[error("proposal not found: {proposal_id}")]
    ProposalNotFound { proposal_id: u64 },

    #[error("proposal is not open")]
    ProposalNotOpen {},

    #[error("proposal is not passed")]
    ProposalNotPassed {},

    #[error("proposal already executed")]
    ProposalAlreadyExecuted {},

    #[error("proposal expired")]
    ProposalExpired {},

    #[error("already voted")]
    AlreadyVoted {},

    #[error("title cannot be empty")]
    EmptyTitle {},
}
