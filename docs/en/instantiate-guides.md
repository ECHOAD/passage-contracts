# Instantiate Guides

## Typical deployment order

1. `registry`
2. `multisig`
3. `ecosystem-factory`
4. per-ecosystem `collection-factory`
5. `pg721*` or `minter*` contracts
6. `marketplace-v3` and/or `auction-english`
7. `split-router`, `royalty-group`, `streaming-billing`, depending on the flow
8. `pasg-governance` if PASG governance is enabled

## General rule

Each contract defines its real `InstantiateMsg` in `src/msg.rs` or generated schema. This guide explains dependency ordering and reasoning; it does not replace the real field list.

## Repeated field types

- `admin`
- linked contract addresses such as `registry`, `multisig`, `split-router`, `collection`, `stake`
- `code_id` when a factory deploys child contracts
- economic parameters such as `trading_fee_bps`, `min_price`, durations, or denoms

## Dependency examples

- `collection-factory` must know `registry` and the collection code ID it will deploy.
- `marketplace-v3` needs `registry`, `fee_collector`, operators, and its global marketplace fee.
- `pasg-governance` needs the `admin_multisig` that will execute ratified actions.
- `stake-rewards` needs the authorized stake contract address.

## Recommendation

Read the contract-specific page first, then confirm the real payload in `msg.rs` or `schema/` before instantiating.
