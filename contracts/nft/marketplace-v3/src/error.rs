use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Marketplace is paused")]
    MarketplacePaused {},

    // ========== Collection Registration Errors ==========
    #[error("Collection not registered: {collection}")]
    CollectionNotRegistered { collection: String },

    #[error("Collection already registered: {collection}")]
    CollectionAlreadyRegistered { collection: String },

    #[error("Collection not active: {collection}")]
    CollectionNotActive { collection: String },

    #[error("Trading is disabled for collection: {collection}")]
    CollectionTradingDisabled { collection: String },

    #[error("Trading fee exceeds maximum: {fee_bps} > {max_bps}")]
    TradingFeeExceedsMax { fee_bps: u64, max_bps: u64 },

    // Legacy error for backward compatibility
    #[error("Collection not supported: {collection}")]
    CollectionNotSupported { collection: String },

    #[error("Ask not found for token {token_id} in collection {collection}")]
    AskNotFound {
        collection: String,
        token_id: String,
    },

    #[error("Ask already exists for token {token_id}")]
    AskAlreadyExists { token_id: String },

    #[error("Ask is not active")]
    AskNotActive {},

    #[error("Bid not found")]
    BidNotFound {},

    #[error("Bid already exists")]
    BidAlreadyExists {},

    #[error("Bid has expired")]
    BidExpired {},

    #[error("Collection bid not found")]
    CollectionBidNotFound {},

    #[error("Collection bid already exists")]
    CollectionBidAlreadyExists {},

    #[error("Price below minimum: {min_price}")]
    PriceBelowMinimum { min_price: String },

    #[error("Invalid payment amount. Expected {expected}, got {received}")]
    InvalidPayment { expected: String, received: String },

    #[error("Invalid payment denom. Expected {expected}, got {received}")]
    InvalidPaymentDenom { expected: String, received: String },

    #[error("Seller cannot buy their own listing")]
    SellerCannotBuy {},

    #[error("Seller cannot bid on their own listing")]
    SellerCannotBid {},

    #[error("Not the owner of token {token_id}")]
    NotTokenOwner { token_id: String },

    #[error("Not the seller of this listing")]
    NotSeller {},

    #[error("Not the bidder")]
    NotBidder {},

    #[error("NFT not approved for marketplace")]
    NftNotApproved {},

    #[error("Insufficient bid funds. Need {needed}, have {available}")]
    InsufficientBidFunds { needed: String, available: String },

    #[error("No collection bid units remaining")]
    NoCollectionBidUnits {},

    #[error("Failed to query NFT ownership")]
    NftQueryFailed {},

    #[error("Failed to query royalty info")]
    RoyaltyQueryFailed {},
}
