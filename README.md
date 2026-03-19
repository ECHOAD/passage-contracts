# Passage Contracts

Passage smart contracts written in CosmWasm and deployed to Passage.

## PASG Utility Model

This workspace treats PASG as a native-denom utility surface, not as a separate in-repo token contract deliverable.

- `upasg` is the canonical settlement denom across this repository.
- `contracts/core/streaming-billing/src/msg.rs` is the source-of-truth PASG utility interface.
- Integrators should query `streaming-billing` with `QueryMsg::PasgUtility {}` for canonical denom, conversion semantics, compatibility metadata, scope boundaries, and supplemental local-economy queries.
- The canonical PASG utility execute surface is `DepositCrypto`, `ReportFiatPurchase`, `WithdrawPoints`, `DistributeWorldRevenue`, and `BatchDistributeRevenue`.
- Optional world-local economies remain auxiliary; creator monetization stays anchored to collection sales, resales, and marketplace fees.
- `split-router` remains denom-agnostic. It routes funds generically and does not define PASG policy.
- Platform billing, subscription, Stripe orchestration, fiat conversion, and similar business logic stay off-chain.
- If a wrapper or adapter is introduced later, it is compatibility-only and must forward to native `upasg` settlement.

## PASG Staking Model

PASG staking in this repo means chain-native staking through native validator delegation on Passage, not a duplicate CosmWasm staking vault.

- Native action surface: `delegate(upasg, validator)`, `undelegate(upasg, validator, amount)`, `redelegate(upasg, src_validator, dst_validator, amount)`, plus queries for delegations, undelegations, validator assignments, validators, and rewards.
- Source of truth: the Passage chain staking module and active validator set, not contract-local balances in this workspace.
- Unbonding: any 21-day wait is a chain-level staking rule or validator-program dependency, not a repo-local claim queue.
- Validator selection: wallets and services should choose one or more Passage validators using chain-native validator metadata, commission, uptime, and operator policy guidance rather than assuming a hardcoded validator inside a contract.
- Non-target crates: `contracts/staking/nft-vault` and `contracts/staking/stake-rewards` are NFT staking primitives, not PASG validator staking contracts.

## Creator Asset Model

This workspace uses an ecosystem-centric creator asset model.

- `registry` is the canonical ledger for ecosystems and collection affiliation.
- Collections are independent on-chain contracts, usually deployed through per-ecosystem `collection-factory` instances.
- A collection can be deregistered from one ecosystem and later re-homed into another without losing creator provenance.
- Shared typed asset semantics live in `pg721` and `pg721-updatable` for `component`, `avatar`, `companion`, `world`, `plugin`, `achievement`, and `world_template`.
- Runtime, rendering, and Unreal-specific payloads remain off-chain.

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

1. Read `contracts/core/streaming-billing/README.md` for the contract-level boundary.
2. Query `streaming-billing` via `PasgUtility` before duplicating denom or conversion assumptions elsewhere.
3. Treat marketplace, minter, auction, and routing contracts as PASG consumers, not PASG policy sources.
4. Keep service-owned billing and subscription rules off-chain even when settlement is verifiable on-chain.
5. Treat PASG staking as chain-native staking and native validator delegation, not a repo-local staking vault.
6. Treat `contracts/staking/nft-vault` and `contracts/staking/stake-rewards` as NFT staking contracts unless a later scoped adapter is explicitly documented.

