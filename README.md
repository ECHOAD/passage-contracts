# Passage Contracts

Passage smart contracts written in CosmWasm and deployed to Passage.

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
