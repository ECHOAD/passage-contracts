use crate::state::{Config, SplitConfig, SplitEvent};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub recipients: Vec<RecipientInput>,
    pub active: Option<bool>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        paused: Option<bool>,
    },
    UpdateSplit {
        recipients: Option<Vec<RecipientInput>>,
        active: Option<bool>,
    },
    Split {},
    RouteWorldRevenue {
        world_nft_id: String,
        world_collection: String,
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
    #[returns(SplitConfigResponse)]
    SplitConfig {},
    #[returns(SplitEventsResponse)]
    SplitEvents {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(SplitPreviewResponse)]
    PreviewSplit { funds: Vec<Coin> },
    #[returns(RoutingMetadataResponse)]
    RoutingMetadata {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct SplitConfigResponse {
    pub split: SplitConfig,
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

#[cw_serde]
pub enum RoutingPreviewRoute {
    PreviewSplit,
}

#[cw_serde]
pub enum RoutingExecuteRoute {
    Split,
    RouteWorldRevenue,
}

#[cw_serde]
pub struct RoutingMetadataResponse {
    pub forwards_attached_funds: bool,
    pub preserves_input_denoms: bool,
    pub preview_query: RoutingPreviewRoute,
    pub execute_routes: Vec<RoutingExecuteRoute>,
}



