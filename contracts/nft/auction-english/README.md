# Passage Auction English

`auction-english` is the repo's reserve-style NFT auction contract.

It escrows the NFT, accepts native-coin bids, enforces duration and bid-increment rules, and settles proceeds to the fee collector, seller, and royalty recipient. It is a PASG consumer, not a PASG policy surface.

## PASG Stance

- Native `upasg` is the canonical PASG settlement path in this repository.
- The auction contract's `denom` field controls which native coin a given deployment accepts for bids and settlement.
- Integrators that need the canonical PASG interface should query `streaming-billing` with `QueryMsg::PasgUtility {}`.
- PASG fee treatment here is visible through `Config {}` and through execute-response attributes on `PlaceBid` and `SettleAuction`.

## Instantiate Message

```json
{
  "admin": "passage1admin...",
  "denom": "upasg",
  "min_price": "100000",
  "trading_fee_bps": 250,
  "max_trading_fee_bps": 1000,
  "fee_collector": "passage1treasury...",
  "registry": "passage1registry...",
  "min_bid_increment_percent": "0.05",
  "min_duration": 3600,
  "max_duration": 604800,
  "extend_duration": 300,
  "require_registration": true
}
```

## Execute Surface

### `CreateAuction`

```json
{
  "create_auction": {
    "collection": "passage1collection...",
    "token_id": "1",
    "reserve_price": {
      "denom": "upasg",
      "amount": "1000000"
    },
    "duration": 86400,
    "seller_funds_recipient": null
  }
}
```

### `PlaceBid`

Attach the bid funds in the configured native denom.

```json
{
  "place_bid": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### `SettleAuction`

```json
{
  "settle_auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

## Query Surface

### `Config`

```json
{
  "config": {}
}
```

### `CanTrade`

```json
{
  "can_trade": {
    "collection": "passage1collection..."
  }
}
```

### `Auction`

Returns the active auction, computed status, computed minimum bid, and `can_settle` flag.

```json
{
  "auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### `AuctionsByCollection`

```json
{
  "auctions_by_collection": {
    "collection": "passage1collection...",
    "start_after": null,
    "limit": 20
  }
}
```

### `AuctionsByEndTime`

```json
{
  "auctions_by_end_time": {
    "start_after": null,
    "limit": 20,
    "descending": false
  }
}
```

## PASG Verification Path

1. Query `streaming-billing` with `PasgUtility {}` for the canonical PASG model.
2. Query `Config {}` and confirm the auction `denom` is `upasg` when the auction is using the native PASG path.
3. Query `Auction {}` to inspect current bid state and the computed `min_bid` before sending funds.
4. After `PlaceBid` or `SettleAuction`, inspect the execute response attributes:
   - `pasg_utility_query`
   - `pasg_native_denom`
   - `pasg_settlement_denom`
   - `pasg_uses_native_utility`
   - `pasg_fee_flow`

## Royalty Routing Note

If the royalty recipient is itself a contract, `auction-english` forwards the royalty payment to that contract with `Split {}` and the royalty funds attached. Otherwise it sends the royalty directly with a bank transfer. That keeps royalty routing generic while still allowing split-wallet recipients.

## Build, Schema, And Tests

```bash
cargo build -p auction-english
cargo run --example schema
cargo test -p auction-english --lib
```
