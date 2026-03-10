use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg, Empty};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub threshold: u64,
    pub total_members: u64,
    pub max_voting_period_secs: u64,
}

#[cw_serde]
pub struct Member {
    pub addr: Addr,
    pub added_at: u64,
}

#[cw_serde]
pub enum Vote {
    Approve,
    Reject,
}

#[cw_serde]
pub enum ProposalStatus {
    Open,
    Passed,
    Executed,
    Rejected,
}

#[cw_serde]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub proposer: Addr,
    pub msgs: Vec<CosmosMsg<Empty>>,
    pub status: ProposalStatus,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub threshold: u64,
    pub total_members: u64,
    pub expires_at: u64,
    pub created_at: u64,
    pub executed_at: Option<u64>,
}

#[cw_serde]
pub struct Ballot {
    pub voter: Addr,
    pub vote: Vote,
    pub voted_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");
pub const MEMBERS: Map<&Addr, Member> = Map::new("members");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const BALLOTS: Map<(u64, &Addr), Ballot> = Map::new("ballots");
