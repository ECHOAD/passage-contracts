# Passage Split Router Contract

`split-router` is the repo's generic native-fund splitter.

It accepts attached funds, preserves the input denoms, and fans those funds out across a configured recipient list. It is not the canonical PASG policy surface.

## PASG Boundary

- `split-router` can forward `upasg`, but it does not define PASG conversion rules, fee treatment policy, adapter behavior, or subscription logic.
- The canonical PASG query surface lives in `streaming-billing` at `QueryMsg::PasgUtility {}`.
- `RouteWorldRevenue` exists as a compatibility execute shape so `streaming-billing` can hand world revenue into the same generic split path.
- Platform billing, premium-tier logic, and service-owned orchestration remain off-chain.

## What The Contract Does

- Stores one active split configuration with weighted recipients.
- Splits any attached native funds across that configuration.
- Records recent split events for auditing and preview validation.
- Exposes queryable routing metadata so integrators can confirm the contract preserves input denoms.

## Instantiate Message

```json
{
  "admin": "passage1admin...",
  "recipients": [
    {
      "address": "passage1creator...",
      "share": "0.85",
      "label": "Creator"
    },
    {
      "address": "passage1collab...",
      "share": "0.15",
      "label": "Collaborator"
    }
  ],
  "active": true
}
```

If `admin` is omitted, the instantiating sender becomes admin.

## Execute Surface

### `UpdateConfig`

Admin-only update for `admin` and `paused`.

```json
{
  "update_config": {
    "admin": null,
    "paused": false
  }
}
```

### `UpdateSplit`

Admin-only update for the recipient set or active flag.

```json
{
  "update_split": {
    "recipients": [
      {
        "address": "passage1creator...",
        "share": "0.80",
        "label": "Creator"
      },
      {
        "address": "passage1producer...",
        "share": "0.20",
        "label": "Producer"
      }
    ],
    "active": true
  }
}
```

### `Split`

Generic split entrypoint. Attach funds in the transaction rather than inside the JSON message.

```json
{
  "split": {}
}
```

### `RouteWorldRevenue`

Compatibility alias for `streaming-billing` world revenue forwarding. It reuses the same native-fund split path and preserves input denoms.

```json
{
  "route_world_revenue": {
    "world_nft_id": "world-1",
    "world_collection": "passage1collection..."
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

### `SplitConfig`

```json
{
  "split_config": {}
}
```

### `SplitEvents`

```json
{
  "split_events": {
    "start_after": null,
    "limit": 20
  }
}
```

### `PreviewSplit`

Preview the exact recipient amounts for attached funds before executing.

```json
{
  "preview_split": {
    "funds": [
      { "denom": "upasg", "amount": "1000000" },
      { "denom": "uatom", "amount": "5000" }
    ]
  }
}
```

### `RoutingMetadata`

Queryable routing metadata for PASG-aware integrators.

```json
{
  "routing_metadata": {}
}
```

`RoutingMetadataResponse` confirms:

- attached funds are forwarded
- input denoms are preserved
- `PreviewSplit` is the preview query
- `Split` and `RouteWorldRevenue` are the execute routes

## Verification Path For PASG-Aware Integrators

1. Query `streaming-billing` with `PasgUtility {}` to learn the canonical PASG model.
2. Query `split-router` with `RoutingMetadata {}` to confirm denom-preserving forwarding.
3. Use `PreviewSplit { funds }` to inspect the exact recipient amounts before sending `upasg`.
4. Execute `Split {}` or `RouteWorldRevenue {}` with attached native funds.

## Build And Test

```bash
cargo build -p split-router
cargo run --example schema
cargo test -p split-router --lib
```
