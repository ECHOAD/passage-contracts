# Streaming Billing Contract

`streaming-billing` is the Passage hybrid billing primitive for streaming points, session settlement, and world revenue accumulation. It is not a subscription engine and it is not a fully autonomous billing system.

## What The Contract Guarantees

- Tracks user points balances funded by direct PASG deposits or fiat purchase reports.
- Records purchase history for both crypto and fiat conversion flows.
- Stores per-world billing configuration and accumulates pending revenue for distribution.
- Charges points when an authorized service settles a session.
- Converts earned points into PASG-denominated pending revenue for downstream routing.

## Hybrid Boundary

This contract is **service-invoked**.

- Passage services decide when a real streaming session starts or stops.
- Passage services own Stripe integration, webhook verification, fiat conversion, support flows, and any refund logic.
- The chain only enforces who may report those lifecycle events and how accounting is bounded.

## Authority Model

The contract currently recognizes four distinct authority surfaces:

- **admin**: updates config, pauses the contract, and can override world-rate setup or emergency-stop sessions.
- **backend operator**: may call `StartSession` and `StopSession` for service-owned lifecycle events.
- **fiat oracle**: may call `ReportFiatPurchase` after off-chain payment confirmation.
- **world owner**: may configure a world rate after verified ownership is confirmed on-chain.

## World Ownership Flow

World billing configuration is no longer based on trusted caller metadata.

1. `SetWorldRate` validates the supplied collection against `registry`.
2. The contract queries the collection with cw721 `OwnerOf` for the `world_nft_id`.
3. The verified owner becomes the stored `WorldConfig.owner`.
4. The validated collection address becomes the canonical `WorldConfig.world_collection`.
5. `StartSession` treats the incoming collection value as a compatibility hint only and always stores the configured canonical collection.

That means `world ownership` for billing config comes from registry registration plus cw721 ownership, not from backend assertions.

## Fiat Purchase Guards

Fiat purchase reporting remains hybrid and intentionally narrow.

- Only the configured fiat oracle may report a purchase.
- The contract keeps a **global fiat transaction** replay index in `FIAT_PURCHASE_TX_IDS`.
- A report is rejected if its timestamp is in the future.
- A report is rejected if it is older than `900` seconds from the current block time.

The contract does **not** verify Stripe signatures on-chain, even though a `stripe_webhook_validator` config field exists. Signature validation and webhook authentication stay off-chain today.

## Session Settlement Guards

Session lifecycle is restricted to the admin or configured **backend operator**.

- Public callers cannot start sessions.
- Public callers cannot stop sessions.
- The **maximum allowed session duration** is `86,400` seconds.
- If a user does not have enough points for the reported duration, the contract charges the remaining balance only.

This design keeps the lifecycle service-owned while still bounding overcharge risk on-chain.

## Revenue Distribution

`DistributeWorldRevenue` and `BatchDistributeRevenue` convert accumulated pending revenue into a downstream execute call for `split-router`.

What this contract does:

- Maintains per-world pending PASG and points balances.
- Emits the settlement execute message with funds attached.
- Resets the pending balance after distribution.

What this contract does not do:

- Decide business timing for payouts.
- Discover recipients off-chain.
- Replace Passage revenue-routing or reporting services.

## Current Non-Goals And Gaps

- No migrate entrypoint is implemented today, so upgrades require redeploy or manual state migration.
- No on-chain Stripe webhook verification exists today.
- No premium-tier, hosting, or subscription logic belongs in this contract.
- No guarantee exists that the contract alone can prove real-world session duration; the service still reports that input.

## Key Execute Messages

### `DepositCrypto`

User deposits PASG directly and receives streaming points according to `points_per_pasg`.

### `ReportFiatPurchase`

Authorized fiat oracle credits points after off-chain payment confirmation, subject to the global replay guard and freshness checks.

### `StartSession`

Authorized backend operator starts a session for a user in a configured world.

### `StopSession`

Authorized backend operator settles a session using the reported duration, bounded by the maximum allowed session duration.

### `SetWorldRate`

Verified world owner or admin configures the hourly points rate for a world after registry and cw721 ownership verification.

## Recommended Verification After Changes

- `cargo test -p streaming-billing --lib`
- `cargo check -p streaming-billing`

Those commands are the minimum test gate for changes to this contract family.