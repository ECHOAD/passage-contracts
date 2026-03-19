use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};
use cw_storage_plus::{Item, Map};

pub const BPS_SCALE: u64 = 10_000;

#[cw_serde]
pub struct Config {
    pub pasg_denom: String,
    pub proposal_threshold: Uint128,
    pub quorum_bps: u64,
    pub approval_bps: u64,
    pub max_voting_period_secs: u64,
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
pub enum ProposalAction {
    UpdateGovernanceConfig {
        proposal_threshold: Option<Uint128>,
        quorum_bps: Option<u64>,
        approval_bps: Option<u64>,
        max_voting_period_secs: Option<u64>,
    },
    UpdateExecutionTargets {
        add: Vec<String>,
        remove: Vec<String>,
    },
    WasmExecute {
        contract_addr: String,
        msg: Binary,
    },
}

#[cw_serde]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub proposer: Addr,
    pub actions: Vec<ProposalAction>,
    pub status: ProposalStatus,
    pub yes_power: Uint128,
    pub no_power: Uint128,
    pub total_power_snapshot: Uint128,
    pub quorum_bps: u64,
    pub approval_bps: u64,
    pub expires_at: u64,
    pub created_at: u64,
    pub executed_at: Option<u64>,
}

#[cw_serde]
pub struct Ballot {
    pub voter: Addr,
    pub vote: Vote,
    pub weight: Uint128,
    pub voted_at: u64,
}

#[cw_serde]
pub struct Delegation {
    pub delegator: Addr,
    pub delegate: Addr,
    pub amount: Uint128,
    pub created_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");
pub const OPEN_PROPOSAL_COUNT: Item<u64> = Item::new("open_proposal_count");
pub const VOTING_BALANCES: Map<&Addr, Uint128> = Map::new("voting_balances");
pub const DELEGATIONS: Map<&Addr, Addr> = Map::new("delegations");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const BALLOTS: Map<(u64, &Addr), Ballot> = Map::new("ballots");
pub const ALLOWED_EXECUTION_TARGETS: Map<&Addr, bool> = Map::new("allowed_execution_targets");
