use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Member list cannot be empty")]
    EmptyMembers {},

    #[error("Duplicate member: {address}")]
    DuplicateMember { address: String },

    #[error("Threshold must be greater than zero")]
    InvalidThreshold {},

    #[error("Threshold cannot exceed member count")]
    ThresholdTooHigh {},

    #[error("Voting period must be greater than zero")]
    InvalidVotingPeriod {},

    #[error("Title cannot be empty")]
    EmptyTitle {},

    #[error("Proposal must contain at least one message")]
    EmptyProposalMsgs {},

    #[error("Proposal not found: {proposal_id}")]
    ProposalNotFound { proposal_id: u64 },

    #[error("Proposal is not open")]
    ProposalNotOpen {},

    #[error("Proposal is not passed")]
    ProposalNotPassed {},

    #[error("Proposal is expired")]
    ProposalExpired {},

    #[error("Proposal is already executed")]
    ProposalAlreadyExecuted {},

    #[error("Vote already recorded for this member")]
    AlreadyVoted {},

    #[error("Address is not a multisig member")]
    NotMember {},
}
