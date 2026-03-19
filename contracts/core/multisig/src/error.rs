use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Proposal threshold must be greater than zero")]
    InvalidProposalThreshold {},

    #[error("Voting period must be greater than zero")]
    InvalidVotingPeriod {},

    #[error("Quorum basis points must be between 1 and 10000")]
    InvalidQuorumBps {},

    #[error("Approval basis points must be between 1 and 10000")]
    InvalidApprovalBps {},

    #[error("Title cannot be empty")]
    EmptyTitle {},

    #[error("Proposal must contain at least one action")]
    EmptyProposalActions {},

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

    #[error("Vote already recorded for this voter")]
    AlreadyVoted {},

    #[error("No voting power available")]
    NoVotingPower {},

    #[error("Proposer does not meet proposal threshold")]
    InsufficientProposalPower {},

    #[error("Insufficient voting balance")]
    InsufficientVotingBalance {},

    #[error("Governance power is locked while proposals are open")]
    GovernancePowerLocked {},

    #[error("Expected native PASG payment in denom {denom}")]
    InvalidPasgPayment { denom: String },

    #[error("No PASG funds were provided")]
    NoPasgFunds {},

    #[error("Delegation already exists")]
    AlreadyDelegated {},

    #[error("No active delegation")]
    NoDelegation {},

    #[error("Cannot delegate to self")]
    CannotDelegateToSelf {},

    #[error("Delegation would create a cycle")]
    DelegationCycle {},

    #[error("Cannot delegate while receiving delegated power")]
    CannotDelegateWithIncomingPower {},

    #[error("Target contract is not allowed for governance execution: {contract_addr}")]
    TargetNotAllowed { contract_addr: String },

    #[error("Duplicate execution target: {address}")]
    DuplicateExecutionTarget { address: String },

    #[error(
        "Cannot withdraw more than available balance: requested {requested}, available {available}"
    )]
    WithdrawAmountExceedsBalance {
        requested: Uint128,
        available: Uint128,
    },
}
