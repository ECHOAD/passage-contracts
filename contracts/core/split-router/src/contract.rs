pub(super) use cosmwasm_std::{
    entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut,
    Env, MessageInfo, Order, Response, StdError, StdResult,
};
pub(super) use cw2::set_contract_version;
pub(super) use cw_storage_plus::Bound;

pub(super) use crate::error::ContractError;
pub(super) use crate::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, RecipientInput, SplitEventsResponse,
    SplitPreviewResponse, SplitRuleResponse, SplitRulesResponse,
};
pub(super) use crate::state::{
    Config, Recipient, SplitEvent, SplitRule, CONFIG, SPLIT_EVENTS, SPLIT_EVENT_COUNT, SPLIT_RULES,
};

pub(super) const CONTRACT_NAME: &str = "crates.io:passage-split-router";
pub(super) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(super) const DEFAULT_LIMIT: u32 = 10;
pub(super) const MAX_LIMIT: u32 = 100;
pub(super) const MAX_EVENTS_STORED: u64 = 1000;

mod execute;
mod helpers;
mod instantiate;
mod query;

pub use execute::execute;
pub use instantiate::instantiate;
pub use query::query;
