use crate::state::{Ballot, Config, Delegation, Proposal, ProposalAction, ProposalStatus, Vote};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub pasg_denom: String,
    pub proposal_threshold: Uint128,
    pub quorum_bps: u64,
    pub approval_bps: u64,
    pub max_voting_period_secs: u64,
    pub allowed_execute_contracts: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    DepositVotingPower {},
    WithdrawVotingPower {
        amount: Uint128,
    },
    DelegateVotingPower {
        delegate: String,
    },
    UndelegateVotingPower {},
    Propose {
        title: String,
        description: Option<String>,
        actions: Vec<ProposalAction>,
    },
    Vote {
        proposal_id: u64,
        vote: Vote,
    },
    Execute {
        proposal_id: u64,
    },
    Close {
        proposal_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(VotingPowerResponse)]
    VotingPower { address: String },
    #[returns(DelegationResponse)]
    Delegation { address: String },
    #[returns(ProposalResponse)]
    Proposal { proposal_id: u64 },
    #[returns(ProposalsResponse)]
    Proposals {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(VoteResponse)]
    Vote { proposal_id: u64, voter: String },
    #[returns(VotesResponse)]
    Votes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CanExecuteResponse)]
    CanExecute { proposal_id: u64 },
    #[returns(ExecutionTargetsResponse)]
    ExecutionTargets {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct VotingPowerResponse {
    pub address: String,
    pub deposited: Uint128,
    pub delegated_to: Option<String>,
    pub incoming_delegated_power: Uint128,
    pub effective_voting_power: Uint128,
}

#[cw_serde]
pub struct DelegationResponse {
    pub delegation: Option<Delegation>,
}

#[cw_serde]
pub struct ProposalResponse {
    pub proposal: Proposal,
    pub computed_status: ProposalStatus,
}

#[cw_serde]
pub struct ProposalsResponse {
    pub proposals: Vec<ProposalListItem>,
}

#[cw_serde]
pub struct ProposalListItem {
    pub proposal: Proposal,
    pub computed_status: ProposalStatus,
}

#[cw_serde]
pub struct VoteResponse {
    pub ballot: Option<Ballot>,
}

#[cw_serde]
pub struct VotesResponse {
    pub ballots: Vec<Ballot>,
}

#[cw_serde]
pub struct CanExecuteResponse {
    pub proposal_id: u64,
    pub executable: bool,
}

#[cw_serde]
pub struct ExecutionTargetsResponse {
    pub targets: Vec<String>,
}

#[cw_serde]
pub struct ScopeExampleResponse {
    pub proposal_action: Binary,
}
