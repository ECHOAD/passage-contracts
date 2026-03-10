use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Auction contract is paused")]
    ContractPaused {},

    #[error("Collection not registered: {collection}")]
    CollectionNotRegistered { collection: String },

    #[error("Trading is disabled for collection: {collection}")]
    CollectionTradingDisabled { collection: String },

    #[error("Trading fee exceeds maximum: {fee_bps} > {max_bps}")]
    TradingFeeExceedsMax { fee_bps: u64, max_bps: u64 },

    #[error("Invalid config: {reason}")]
    InvalidConfig { reason: String },

    #[error("Invalid reserve price. Minimum accepted is {min_price}")]
    InvalidReservePrice { min_price: String },

    #[error("Invalid payment denom. Expected {expected}, got {received}")]
    InvalidPaymentDenom { expected: String, received: String },

    #[error("Auction already exists for {collection}/{token_id}")]
    AuctionAlreadyExists {
        collection: String,
        token_id: String,
    },

    #[error("Auction not found for {collection}/{token_id}")]
    AuctionNotFound {
        collection: String,
        token_id: String,
    },

    #[error("Auction already started")]
    AuctionStarted {},

    #[error("Auction not ended yet")]
    AuctionNotEnded {},

    #[error("Auction already ended")]
    AuctionEnded {},

    #[error("Auction has no bids to settle")]
    NoBidPlaced {},

    #[error("Invalid duration: got {got}, expected between {min} and {max}")]
    InvalidDuration { min: u64, max: u64, got: u64 },

    #[error("Bid too low. Minimum bid is {min_bid}")]
    BidTooLow { min_bid: String },

    #[error("Seller cannot bid on their own auction")]
    SellerCannotBid {},

    #[error("Not the owner of token {token_id}")]
    NotTokenOwner { token_id: String },

    #[error("NFT is not approved for auction transfer")]
    NftNotApproved {},

    #[error("Only the seller can manage this auction")]
    NotSeller {},

    #[error("Split Router not configured")]
    SplitRouterNotConfigured {},

    #[error("Failed to query NFT ownership")]
    NftQueryFailed {},

    #[error("Failed to query royalty info")]
    RoyaltyQueryFailed {},
}
