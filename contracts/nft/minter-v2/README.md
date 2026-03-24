# Passage Minter v2

`minter-v2` is the repo's primary-sale minting contract.

It follows a collection-first model: the target `pg721` collection must already exist before the
minter is instantiated.

It mints from an attached `pg721` collection, enforces timing and per-address limits, and collects native-coin payments according to `unit_price`. It is a PASG consumer, not a PASG policy engine.

## PASG Stance

- Native `upasg` is the canonical PASG settlement path in this repository.
- `unit_price.denom` is the mint contract's local settlement configuration. When it is `upasg`, the mint flow is using the native PASG path directly.
- Integrators that need the canonical PASG interface should query `streaming-billing` with `QueryMsg::PasgUtility {}`.
- PASG fee treatment here is verified through `Config {}`, `MintPrice {}`, and execute-response attributes on payment-bearing flows.

## Instantiate Message

```json
{
  "base_token_uri": "ipfs://bafy.../metadata/",
  "num_tokens": 1000,
  "cw721_address": "passage1collection...",
  "start_time": "1773597600000000000",
  "per_address_limit": 3,
  "unit_price": {
    "denom": "upasg",
    "amount": "1000000"
  },
  "whitelist": null,
  "registry": "passage1registry..."
}
```

## Execute Surface

### `Mint`

Attach exactly the current mint price.

```json
{
  "mint": {}
}
```

### `BatchMint`

Attach `count * current_price.amount`.

```json
{
  "batch_mint": {
    "count": 2
  }
}
```

### `UpdateConfig`

```json
{
  "update_config": {
    "admin": null,
    "per_address_limit": 5,
    "unit_price": {
      "denom": "upasg",
      "amount": "1200000"
    },
    "whitelist": null,
    "registry": "passage1registry...",
    "paused": false
  }
}
```

### `Withdraw` / `WithdrawTo`

These are admin-only legacy fund-release paths for the minter's own held balance.

```json
{
  "withdraw": {}
}
```

```json
{
  "withdraw_to": {
    "recipient": "passage1treasury..."
  }
}
```

## Query Surface

### `Config`

Returns the configured `unit_price`, whitelist pointer, registry pointer, and pause state.

```json
{
  "config": {}
}
```

### `MintPrice`

Returns `public_price`, optional `whitelist_price`, and the `current_price` that callers must pay right now.

```json
{
  "mint_price": {}
}
```

### `CanMint`

```json
{
  "can_mint": {
    "address": "passage1buyer..."
  }
}
```

### `MintStats`

```json
{
  "mint_stats": {}
}
```

### `IsMintingActive`

```json
{
  "is_minting_active": {}
}
```

## PASG Verification Path

1. Instantiate the collection first and register it in `registry`.
2. Authorize the `minter-v2` address for that collection in `registry`.
3. Query `streaming-billing` with `PasgUtility {}` for the canonical PASG model.
4. Query `Config {}` or `MintPrice {}` and confirm `current_price.denom` / `unit_price.denom` is `upasg` when the drop is PASG-native.
5. Use `CanMint { address }` before sending funds.
6. After `Mint`, `BatchMint`, `Withdraw`, or `WithdrawTo`, inspect the execute response attributes:
   - `pasg_utility_query`
   - `pasg_native_denom`
   - `pasg_settlement_denom`
   - `pasg_uses_native_utility`
   - `pasg_fee_flow`

## Non-Goals

- `minter-v2` does not own PASG conversion semantics.
- `minter-v2` does not implement subscription or premium-tier logic.
- `minter-v2` does not replace the canonical PASG query surface in `streaming-billing`.
- `minter-v2` does not deploy collections anymore.

## Build, Schema, And Tests

```bash
cargo build -p minter-v2
cargo run --example schema
cargo test -p minter-v2 --lib --tests
```
