use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128, Uint256};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub denom: String,
    pub min_price: Uint128,
    pub trading_fee_bps: u64,
    pub max_trading_fee_bps: u64,
    pub fee_collector: Addr,
    pub registry: Option<Addr>,
    pub min_bid_increment_percent: Decimal,
    pub min_duration: u64,
    pub max_duration: u64,
    pub extend_duration: u64,
    pub paused: bool,
    pub require_registration: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct HighBid {
    pub bidder: Addr,
    pub coin: Coin,
    pub placed_at: Timestamp,
}

#[cw_serde]
pub struct Auction {
    pub collection: Addr,
    pub token_id: String,
    pub seller: Addr,
    pub reserve_price: Coin,
    pub duration: u64,
    pub seller_funds_recipient: Option<Addr>,
    pub high_bid: Option<HighBid>,
    pub first_bid_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub created_at: u64,
}

#[cw_serde]
pub enum AuctionStatus {
    Created,
    Active,
    Ended,
}

impl Auction {
    pub fn status(&self, now: Timestamp) -> AuctionStatus {
        match self.end_time {
            None => AuctionStatus::Created,
            Some(end_time) if now < end_time => AuctionStatus::Active,
            Some(_) => AuctionStatus::Ended,
        }
    }

    pub fn funds_recipient(&self) -> Addr {
        self.seller_funds_recipient
            .clone()
            .unwrap_or_else(|| self.seller.clone())
    }

    pub fn min_bid_coin(&self, min_bid_increment_percent: Decimal) -> Coin {
        let amount = match &self.high_bid {
            Some(high_bid) => mul_decimal_ceil(
                high_bid.coin.amount,
                Decimal::one() + min_bid_increment_percent,
            ),
            None => self.reserve_price.amount,
        };

        Coin {
            denom: self.reserve_price.denom.clone(),
            amount,
        }
    }
}

fn mul_decimal_ceil(amount: Uint128, multiplier: Decimal) -> Uint128 {
    let numerator = Uint256::from(amount) * Uint256::from(multiplier.atomics());
    let denominator = Uint256::from(10u128.pow(Decimal::DECIMAL_PLACES));
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    let rounded = if remainder.is_zero() {
        quotient
    } else {
        quotient + Uint256::one()
    };

    Uint128::try_from(rounded).expect("decimal multiplication overflow")
}

pub type AuctionKey = (Addr, String);

pub struct AuctionIndexes<'a> {
    pub collection: MultiIndex<'a, Addr, Auction, AuctionKey>,
    pub seller: MultiIndex<'a, Addr, Auction, AuctionKey>,
    pub end_time: MultiIndex<'a, u64, Auction, AuctionKey>,
}

impl<'a> IndexList<Auction> for AuctionIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Auction>> + '_> {
        let v: Vec<&dyn Index<Auction>> = vec![&self.collection, &self.seller, &self.end_time];
        Box::new(v.into_iter())
    }
}

pub fn auctions<'a>() -> IndexedMap<AuctionKey, Auction, AuctionIndexes<'a>> {
    let indexes = AuctionIndexes {
        collection: MultiIndex::new(
            |_pk: &[u8], auction: &Auction| auction.collection.clone(),
            "auctions",
            "auctions__collection",
        ),
        seller: MultiIndex::new(
            |_pk: &[u8], auction: &Auction| auction.seller.clone(),
            "auctions",
            "auctions__seller",
        ),
        end_time: MultiIndex::new(
            |_pk: &[u8], auction: &Auction| auction.end_time.map_or(u64::MAX, |end| end.seconds()),
            "auctions",
            "auctions__end_time",
        ),
    };

    IndexedMap::new("auctions", indexes)
}
