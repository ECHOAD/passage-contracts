# Passage Marketplace v3

`marketplace-v3` is the repo's multi-collection secondary-market contract.

It validates registry trade permissions, requires explicit collection registration before trading, settles native-coin sales, and treats PASG as an external utility surface rather than a marketplace-owned policy system.

## Model

- Registration is mandatory. Unregistered collections cannot list, bid, or settle through the marketplace.
- `trading_fee_bps` is marketplace-global.
- `denom` is collection-scoped and is defined when a collection is registered or updated.
- Collection creators submit registration and update requests.
- Marketplace admin may approve or reject those requests, or may register and update collections directly without using the request queue.

## PASG Stance

- Native `upasg` is the canonical PASG settlement path in this repository.
- A collection may settle in another denom, but that is local collection configuration, not a second PASG model.
- Integrators that need the canonical PASG interface should query `streaming-billing` with `QueryMsg::PasgUtility {}`.
- Settlement responses expose `pasg_utility_query`, `pasg_native_denom`, `pasg_settlement_denom`, `pasg_uses_native_utility`, and `pasg_fee_flow`.

## Instantiate Message

```json
{
  "admin": "passage1admin...",
  "min_price": "100000",
  "trading_fee_bps": 250,
  "fee_collector": "passage1treasury...",
  "registry": "passage1registry...",
  "operators": ["passage1operator..."]
}
```

## Registration Flows

Owner request path:

```json
{
  "submit_collection_registration_request": {
    "collection": "passage1collection...",
    "denom": "upasg",
    "note": "request access to trade in marketplace-v3"
  }
}
```

```json
{
  "resolve_collection_registration_request": {
    "collection": "passage1collection...",
    "approved": true,
    "denom": null,
    "note": "approved"
  }
}
```

Admin direct path:

```json
{
  "register_collection": {
    "collection": "passage1collection...",
    "denom": "upasg"
  }
}
```

## Update Flows

Owner request path:

```json
{
  "submit_collection_update_request": {
    "collection": "passage1collection...",
    "active": true,
    "denom": "uion",
    "note": "switch settlement denom"
  }
}
```

```json
{
  "resolve_collection_update_request": {
    "collection": "passage1collection...",
    "approved": true,
    "active": null,
    "denom": null,
    "note": "approved"
  }
}
```

Admin direct path:

```json
{
  "update_collection_config": {
    "collection": "passage1collection...",
    "active": true,
    "denom": "upasg"
  }
}
```

## Trading Surface

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

### `SetCollectionBid` / `AcceptCollectionBid`

`set_collection_bid` requires attached funds equal to `units * price.amount`.

## Query Surface

### `Config`

Returns admin, `min_price`, marketplace-global `trading_fee_bps`, fee collector, registry, operators, and pause state.

### `CollectionConfig`

Returns the registered collection config, including its required settlement denom.

### `CollectionRegistrationRequest`

Returns the latest registration request for a collection.

### `CollectionRegistrationRequests`

Lists registration requests and can filter by `pending`, `approved`, or `rejected`.

### `CollectionUpdateRequest`

Returns the latest pending or resolved update request for a collection.

### `CollectionUpdateRequests`

Lists collection update requests and can filter by status.

### `CollectionDenom`

Returns the collection-scoped settlement denom.

### `CollectionFee`

Returns the marketplace-global fee. `is_override` is always `false`.

### `CanTrade`

Returns whether the collection is both registered locally and allowed by registry moderation.

### `PreviewSale`

Shows `trading_fee`, `royalty`, and `seller_proceeds` for a proposed sale price.

## Royalty Routing Note

Royalty payouts are sent directly to the royalty recipient unless that recipient is itself a contract. In that case, `marketplace-v3` executes the recipient contract with `Split {}` and the royalty funds attached.

## Build, Schema, And Tests

```bash
cargo build -p marketplace-v3
cargo run --example schema -p marketplace-v3
cargo test -p marketplace-v3 --lib
```
