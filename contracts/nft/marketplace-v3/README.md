# Passage Marketplace v3

`marketplace-v3` is the repo's multi-collection secondary-market contract.

It validates collection trading permissions, enforces collection-specific denom and fee configuration, transfers NFTs, and settles native-coin proceeds to the fee collector, seller, and royalty recipient. It is a PASG consumer, not a PASG policy source.

## PASG Stance

- Native `upasg` is the canonical PASG settlement path in this repository.
- `marketplace-v3` may be configured with another denom for a collection, but that is local marketplace configuration rather than a second canonical PASG model.
- Integrators that need the canonical PASG interface should query `streaming-billing` with `QueryMsg::PasgUtility {}`.
- PASG fee treatment is observable here through collection config queries and execute-response attributes, not through marketplace-owned PASG logic.

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
  "operators": ["passage1operator..."],
  "require_registration": true
}
```

## Execute Surface

### `RegisterCollection`

```json
{
  "register_collection": {
    "collection": "passage1collection...",
    "trading_fee_bps": null,
    "denom": null
  }
}
```

### `UpdateCollectionConfig`

```json
{
  "update_collection_config": {
    "collection": "passage1collection...",
    "active": true,
    "trading_fee_bps": 300,
    "denom": "upasg"
  }
}
```

### `SetAsk`

```json
{
  "set_ask": {
    "collection": "passage1collection...",
    "token_id": "1",
    "price": {
      "denom": "upasg",
      "amount": "1000000"
    },
    "funds_recipient": null
  }
}
```

### `BuyNow`

Attach the exact ask price to the transaction.

```json
{
  "buy_now": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### `SetBid` / `AcceptBid`

`set_bid` requires attached funds equal to `price`.

```json
{
  "set_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "price": {
      "denom": "upasg",
      "amount": "900000"
    },
    "expires_at": 1773597600
  }
}
```

```json
{
  "accept_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "bidder": "passage1bidder..."
  }
}
```

### `SetCollectionBid` / `AcceptCollectionBid`

Attach `units * price.amount` for `set_collection_bid`.

```json
{
  "set_collection_bid": {
    "collection": "passage1collection...",
    "units": 2,
    "price": {
      "denom": "upasg",
      "amount": "800000"
    },
    "expires_at": 1773597600
  }
}
```

```json
{
  "accept_collection_bid": {
    "collection": "passage1collection...",
    "token_id": "42",
    "bidder": "passage1bidder..."
  }
}
```

## Query Surface

### `Config`

Returns the default marketplace denom, fee settings, and registration policy.

```json
{
  "config": {}
}
```

### `CollectionDenom`

Returns the effective payment denom for a collection and whether it is an override.

```json
{
  "collection_denom": {
    "collection": "passage1collection..."
  }
}
```

### `CollectionFee`

Returns the effective trading fee for a collection and whether it is an override.

```json
{
  "collection_fee": {
    "collection": "passage1collection..."
  }
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

### `PreviewSale`

Shows `trading_fee`, `royalty`, and `seller_proceeds` for a proposed sale price.

```json
{
  "preview_sale": {
    "collection": "passage1collection...",
    "price": "1000000"
  }
}
```

## PASG Verification Path

1. Query `streaming-billing` with `PasgUtility {}` for the canonical PASG model.
2. Query `CollectionDenom {}` and confirm whether the collection actually settles in `upasg`.
3. Query `CollectionFee {}` or `PreviewSale {}` to inspect the marketplace fee treatment before settlement.
4. After `BuyNow`, `AcceptBid`, or `AcceptCollectionBid`, inspect the execute response attributes:
   - `pasg_utility_query`
   - `pasg_native_denom`
   - `pasg_settlement_denom`
   - `pasg_uses_native_utility`
   - `pasg_fee_flow`

## Royalty Routing Note

Royalty payouts are sent directly to the royalty recipient unless that recipient is itself a contract. In that case, `marketplace-v3` executes the recipient contract with `Split {}` and the royalty funds attached. That keeps marketplace settlement generic while allowing split-wallet royalty contracts.

## Build, Schema, And Tests

```bash
cargo build -p marketplace-v3
cargo run --example schema
cargo test -p marketplace-v3 --lib
```
