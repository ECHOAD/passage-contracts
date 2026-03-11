use crate::state::{Config, SplitEvent, SplitRule};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        paused: Option<bool>,
    },
    CreateSplitRule {
        key: String,
        recipients: Vec<RecipientInput>,
    },
    UpdateSplitRule {
        key: String,
        recipients: Option<Vec<RecipientInput>>,
        owner: Option<String>,
        active: Option<bool>,
    },
    RemoveSplitRule {
        key: String,
    },
    Split {
        key: String,
    },
}

#[cw_serde]
pub struct RecipientInput {
    pub address: String,
    pub share: Decimal,
    pub label: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SplitRuleResponse)]
    SplitRule { key: String },
    #[returns(SplitRulesResponse)]
    SplitRules {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(SplitRulesResponse)]
    SplitRulesByOwner {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(SplitEventsResponse)]
    SplitEvents {
        key: Option<String>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(SplitPreviewResponse)]
    PreviewSplit { key: String, funds: Vec<Coin> },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct SplitRuleResponse {
    pub rule: Option<SplitRule>,
}

#[cw_serde]
pub struct SplitRulesResponse {
    pub rules: Vec<SplitRule>,
}

#[cw_serde]
pub struct SplitEventsResponse {
    pub events: Vec<SplitEvent>,
}

#[cw_serde]
pub struct SplitPreviewResponse {
    pub total_funds: Vec<Coin>,
    pub recipient_amounts: Vec<(String, Vec<Coin>)>,
}
