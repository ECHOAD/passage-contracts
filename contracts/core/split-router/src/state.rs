use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub paused: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Recipient {
    pub address: Addr,
    pub share: Decimal,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitConfig {
    pub recipients: Vec<Recipient>,
    pub active: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

pub const SPLIT_CONFIG: Item<SplitConfig> = Item::new("split_config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SplitEvent {
    pub id: u64,
    pub total_funds: Vec<Coin>,
    pub recipient_amounts: Vec<(Addr, Vec<Coin>)>,
    pub timestamp: u64,
    pub sender: Addr,
}

pub const SPLIT_EVENT_COUNT: Item<u64> = Item::new("split_event_count");
pub const SPLIT_EVENTS: Map<u64, SplitEvent> = Map::new("split_events");
