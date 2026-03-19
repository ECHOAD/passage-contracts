use crate::state::{Ballot, Config, Member, Proposal, ProposalStatus, Vote};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty};

#[cw_serde]
pub struct InstantiateMsg {
    pub members: Vec<String>,
    pub threshold: u64,
    pub max_voting_period_secs: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Propose {
        title: String,
        description: Option<String>,
        msgs: Vec<CosmosMsg<Empty>>,
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
    UpdateMembers {
        members: Vec<String>,
        threshold: u64,
        max_voting_period_secs: Option<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(MemberResponse)]
    Member { address: String },
    #[returns(MembersResponse)]
    Members {
        start_after: Option<String>,
        limit: Option<u32>,
    },
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
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct MemberResponse {
    pub member: Option<Member>,
}

#[cw_serde]
pub struct MembersResponse {
    pub members: Vec<Member>,
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
