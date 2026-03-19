use crate::state::{
    Ballot, Config, PasgUtilityConfig, Proposal, ProposalSnapshot, ProposalStatus,
    RatifiedAdminAction, Vote,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_multisig: String,
    pub native_denom: String,
    pub proposal_deposit: Uint128,
    pub voting_period_secs: u64,
    pub quorum_bps: u64,
    pub pass_bps: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    DepositVotingPower {},
    WithdrawVotingPower { amount: Uint128 },
    Delegate { delegate: String },
    Undelegate {},
    Propose {
        title: String,
        description: Option<String>,
        action: ProposalAction,
    },
    Vote {
        proposal_id: u64,
        vote: Vote,
    },
    ExecuteProposal {
        proposal_id: u64,
    },
    Close {
        proposal_id: u64,
    },
}

#[cw_serde]
pub enum AdminAction {
    StreamingBillingUpdateConfig {
        contract_addr: String,
        backend_operator: Option<String>,
        fiat_oracle: Option<String>,
        stripe_webhook_validator: Option<String>,
        paused: Option<bool>,
    },
    MarketplaceV3UpdateConfig {
        contract_addr: String,
        admin: Option<String>,
        denom: Option<String>,
        min_price: Option<Uint128>,
        trading_fee_bps: Option<u64>,
        max_trading_fee_bps: Option<u64>,
        fee_collector: Option<String>,
        registry: Option<String>,
        operators: Option<Vec<String>>,
        paused: Option<bool>,
        require_registration: Option<bool>,
    },
    AuctionEnglishUpdateConfig {
        contract_addr: String,
        admin: Option<String>,
        denom: Option<String>,
        min_price: Option<Uint128>,
        trading_fee_bps: Option<u64>,
        max_trading_fee_bps: Option<u64>,
        fee_collector: Option<String>,
        registry: Option<String>,
        min_bid_increment_percent: Option<Decimal>,
        min_duration: Option<u64>,
        max_duration: Option<u64>,
        extend_duration: Option<u64>,
        paused: Option<bool>,
        require_registration: Option<bool>,
    },
}

#[cw_serde]
pub enum ProposalAction {
    SetPasgUtilityConfig {
        points_per_pasg: Uint128,
        max_fiat_report_age_secs: u64,
        max_session_duration_secs: u64,
    },
    StageAdminAction {
        action: AdminAction,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(DepositResponse)]
    Deposit { address: String },
    #[returns(VotingPowerResponse)]
    VotingPower { address: String },
    #[returns(PasgUtilityConfigResponse)]
    PasgUtilityConfig {},
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
    #[returns(RatifiedAdminActionResponse)]
    RatifiedAdminAction { proposal_id: u64 },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct DepositResponse {
    pub address: String,
    pub deposited: Uint128,
}

#[cw_serde]
pub struct VotingPowerResponse {
    pub address: String,
    pub deposited: Uint128,
    pub delegated_to: Option<String>,
    pub incoming_delegated_power: Uint128,
    pub effective_voting_power: Uint128,
    pub locked_balance: Uint128,
}

#[cw_serde]
pub struct PasgUtilityConfigResponse {
    pub config: PasgUtilityConfig,
}

#[cw_serde]
pub struct ProposalResponse {
    pub proposal: Proposal,
    pub computed_status: ProposalStatus,
    pub snapshot: ProposalSnapshot,
}

#[cw_serde]
pub struct ProposalsResponse {
    pub proposals: Vec<Proposal>,
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
pub struct RatifiedAdminActionResponse {
    pub action: Option<RatifiedAdminAction>,
}
