pub(super) use cosmwasm_std::{
    entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut,
    Env, MessageInfo, Order, Response, StdResult, Uint128,
};
pub(super) use cw2::set_contract_version;
pub(super) use cw_storage_plus::Bound;

pub(super) use crate::error::ContractError;
pub(super) use crate::msg::{
    CollaboratorInput, CollectionStatsResponse, ConfigResponse, DistributionPreviewResponse,
    DistributionRuleResponse, DistributionRulesResponse, EcosystemConfigResponse, ExecuteMsg,
    InstantiateMsg, QueryMsg, RecipientInput, RevenueEventsResponse, SplitRecipientInput,
    SplitWalletResponse, SplitWalletsResponse,
};
pub(super) use crate::state::{
    Collaborator, Config, DistributionRule, EcosystemConfig, RevenueEvent, RevenueEventType,
    SplitRecipient, SplitWallet, COLLECTION_STATS, CONFIG, DISTRIBUTION_RULES, ECOSYSTEM_CONFIGS,
    REVENUE_EVENTS, REVENUE_EVENT_COUNT, SPLIT_WALLETS,
};

pub(super) const CONTRACT_NAME: &str = "crates.io:passage-revenue-router";
pub(super) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(super) const DEFAULT_LIMIT: u32 = 10;
pub(super) const MAX_LIMIT: u32 = 100;
pub(super) const MAX_PLATFORM_FEE: &str = "0.10"; // 10% max platform fee
pub(super) const MAX_EVENTS_STORED: u64 = 1000;

mod execute;
mod helpers;
mod instantiate;
mod query;

pub use execute::execute;
pub use instantiate::instantiate;
pub use query::query;
