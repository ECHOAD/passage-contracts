use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin_multisig: Addr,
    pub native_denom: String,
    pub proposal_deposit: Uint128,
    pub voting_period_secs: u64,
    pub quorum_bps: u64,
    pub pass_bps: u64,
}

#[cw_serde]
pub struct PasgUtilityConfig {
    pub points_per_pasg: Uint128,
    pub max_fiat_report_age_secs: u64,
    pub max_session_duration_secs: u64,
    pub updated_by_proposal: Option<u64>,
}

#[cw_serde]
pub enum AdminAction {
    UpdateStreamingBillingConfig {
        contract_addr: String,
        backend_operator: Option<String>,
        paused: Option<bool>,
    },
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
    pub action: crate::msg::ProposalAction,
    pub status: ProposalStatus,
    pub yes_power: Uint128,
    pub no_power: Uint128,
    pub total_power_snapshot: Uint128,
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
pub struct RatifiedAdminAction {
    pub proposal_id: u64,
    pub admin_multisig: Addr,
    pub action: AdminAction,
    pub payload_hash: String,
    pub ratified_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");
pub const PASG_UTILITY_CONFIG: Item<PasgUtilityConfig> = Item::new("pasg_utility_config");
pub const TOTAL_DEPOSITED: Item<Uint128> = Item::new("total_deposited");
pub const DEPOSITS: Map<&Addr, Uint128> = Map::new("deposits");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const BALLOTS: Map<(u64, &Addr), Ballot> = Map::new("ballots");
pub const RATIFIED_ADMIN_ACTIONS: Map<u64, RatifiedAdminAction> = Map::new("ratified_admin_actions");
