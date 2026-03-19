# Passage Contracts

Passage smart contracts written in CosmWasm and deployed to Passage.

## PASG Utility Model

This workspace treats PASG as a native-denom utility surface, not as a separate in-repo token contract deliverable.

- `upasg` is the canonical settlement denom across this repository.
- `contracts/core/streaming-billing/src/msg.rs` is the source-of-truth PASG utility interface.
- Integrators should query `streaming-billing` with `QueryMsg::PasgUtility {}` for canonical denom, conversion semantics, compatibility metadata, and scope boundaries.
- The canonical PASG utility execute surface is `DepositCrypto`, `ReportFiatPurchase`, `WithdrawPoints`, `DistributeWorldRevenue`, and `BatchDistributeRevenue`.
- `split-router` remains denom-agnostic. It routes funds generically and does not define PASG policy.
- Platform billing, subscription, Stripe orchestration, fiat conversion, and similar business logic stay off-chain.
- If a wrapper or adapter is introduced later, it is compatibility-only and must forward to native `upasg` settlement.

## PASG Governance Model

`contracts/core/multisig` is the PASG-holder governance contract for protocol-scoped changes.

- Governance power comes from deposited native `upasg`.
- Holders can delegate voting power directly.
- Proposals store quorum and approval thresholds from their creation-time snapshot.
- Governance may execute only against allowlisted protocol contracts or its own config surface.
- Governance does not control subscriptions, streaming infrastructure, analytics, search, or other off-chain platform systems.

## Diagram

![Diagram Protocol](Passage%20Protocol%20-%20Contract%20Interactions%20-%20DIAGRAM.png)

## Commands

**Deploy to mainnet**

```bash
passage tx wasm store artifacts/marketplace_legacy.wasm  --from <from_address> --chain-id=passage-2 --node <node> --gas-prices 0.1upasg--gas auto --gas-adjustment 1.3 -b block
```

**Deploy to testnet**

```bash
passage tx wasm store artifacts/minter_metadata_onchain.wasm  --from <from_address> --chain-id=passage-2 \
  --gas-prices 0.1upasg --gas auto --gas-adjustment 1.3 -b block -y
```

## Migrate

```bash
passage tx wasm migrate <contract_address> 2805 '{"num_mintable_tokens":5000}' --from <from_address> --chain-id=passage-2 --gas-prices 0.1upasg --gas auto --gas-adjustment 1.3 -b block -y
```

```bash
passage tx wasm set-contract-admin <contract_address> <new_admin> --from <from_address> --chain-id=passage-2 --gas-prices 0.1upasg --gas auto --gas-adjustment 1.3 -b block -y
```

## Integrator Guidance

When you need PASG semantics in this repo:

1. Read `contracts/core/streaming-billing/README.md` for the canonical PASG utility boundary.
2. Query `streaming-billing` via `PasgUtility` before duplicating denom or conversion assumptions elsewhere.
3. Treat marketplace, minter, auction, and routing contracts as PASG consumers, not PASG policy sources.
4. Treat `contracts/core/multisig` as the PASG governance surface for protocol actions, not as a general platform admin bus.
5. Keep service-owned billing, subscriptions, streaming ops, analytics, and similar business rules off-chain even when settlement or governance is verifiable on-chain.
