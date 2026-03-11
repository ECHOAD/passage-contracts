use crate::state::{Auction, AuctionStatus, Config};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub denom: String,
    pub min_price: Uint128,
    pub trading_fee_bps: u64,
    pub max_trading_fee_bps: Option<u64>,
    pub fee_collector: String,
    pub registry: Option<String>,
    pub split_router: Option<String>,
    pub use_split_router: Option<bool>,
    pub min_bid_increment_percent: Decimal,
    pub min_duration: u64,
    pub max_duration: u64,
    pub extend_duration: u64,
    pub require_registration: Option<bool>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<String>,
        denom: Option<String>,
        min_price: Option<Uint128>,
        trading_fee_bps: Option<u64>,
        max_trading_fee_bps: Option<u64>,
        fee_collector: Option<String>,
        registry: Option<String>,
        split_router: Option<String>,
        use_split_router: Option<bool>,
        min_bid_increment_percent: Option<Decimal>,
        min_duration: Option<u64>,
        max_duration: Option<u64>,
        extend_duration: Option<u64>,
        paused: Option<bool>,
        require_registration: Option<bool>,
    },
    CreateAuction {
        collection: String,
        token_id: String,
        reserve_price: Coin,
        duration: u64,
        seller_funds_recipient: Option<String>,
    },
    UpdateReservePrice {
        collection: String,
        token_id: String,
        reserve_price: Coin,
    },
    CancelAuction {
        collection: String,
        token_id: String,
    },
    PlaceBid {
        collection: String,
        token_id: String,
    },
    SettleAuction {
        collection: String,
        token_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CanTradeResponse)]
    CanTrade { collection: String },
    #[returns(AuctionResponse)]
    Auction {
        collection: String,
        token_id: String,
    },
    #[returns(AuctionsResponse)]
    AuctionsByCollection {
        collection: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(AuctionsResponse)]
    AuctionsBySeller { seller: String, limit: Option<u32> },
    #[returns(AuctionsResponse)]
    AuctionsByEndTime {
        start_after: Option<u64>,
        limit: Option<u32>,
        descending: Option<bool>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub denom: String,
    pub min_price: Uint128,
    pub trading_fee_bps: u64,
    pub max_trading_fee_bps: u64,
    pub fee_collector: String,
    pub registry: Option<String>,
    pub split_router: Option<String>,
    pub use_split_router: bool,
    pub min_bid_increment_percent: Decimal,
    pub min_duration: u64,
    pub max_duration: u64,
    pub extend_duration: u64,
    pub paused: bool,
    pub require_registration: bool,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> Self {
        Self {
            admin: config.admin.to_string(),
            denom: config.denom,
            min_price: config.min_price,
            trading_fee_bps: config.trading_fee_bps,
            max_trading_fee_bps: config.max_trading_fee_bps,
            fee_collector: config.fee_collector.to_string(),
            registry: config.registry.map(|addr| addr.to_string()),
            split_router: config.split_router.map(|addr| addr.to_string()),
            use_split_router: config.use_split_router,
            min_bid_increment_percent: config.min_bid_increment_percent,
            min_duration: config.min_duration,
            max_duration: config.max_duration,
            extend_duration: config.extend_duration,
            paused: config.paused,
            require_registration: config.require_registration,
        }
    }
}

#[cw_serde]
pub struct CanTradeResponse {
    pub can_trade: bool,
    pub reason: Option<String>,
}

#[cw_serde]
pub struct AuctionResponse {
    pub auction: Option<Auction>,
    pub status: Option<AuctionStatus>,
    pub min_bid: Option<Coin>,
    pub can_settle: bool,
}

#[cw_serde]
pub struct AuctionsResponse {
    pub auctions: Vec<Auction>,
}

#[cw_serde]
pub enum SplitRouterExecuteMsg {
    Split { key: String },
}

#[cw_serde]
pub enum Cw721ExecuteMsg {
    TransferNft { recipient: String, token_id: String },
}

#[cw_serde]
pub enum Cw721QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
}

#[cw_serde]
pub struct OwnerOfResponse {
    pub owner: String,
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct Approval {
    pub spender: String,
    pub expires: Expiration,
}

#[cw_serde]
pub enum Expiration {
    AtHeight(u64),
    AtTime(Timestamp),
    Never {},
}

#[cw_serde]
pub enum Pg721QueryMsg {
    CollectionInfo {},
}

#[cw_serde]
pub struct CollectionInfoResponse {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub royalty_info: Option<RoyaltyInfoResponse>,
}

#[cw_serde]
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: String,
}

#[cw_serde]
pub enum RegistryQueryMsg {
    Collection { address: String },
    CanTradeCollection { address: String },
}

#[cw_serde]
pub struct RegistryCollectionResponse {
    pub collection: Option<RegistryCollection>,
}

#[cw_serde]
pub struct RegistryCollection {
    pub creator: String,
}

#[cw_serde]
pub struct RegistryApprovalStatusResponse {
    pub approved: bool,
}
