# External Integrations

**Analysis Date:** 2026-03-17

## APIs & External Services

**Blockchain / Passage Runtime:**
- Passage chain is the primary production integration target for every contract in the workspace; deployment and migration commands use the `passage` CLI, chain ID `passage-2`, and denom `upasg` in `README.md` and `docs/03-json-examples.md`.
  - SDK/Client: `passage tx wasm` CLI commands in `README.md`; `SigningCosmWasmClient` example in `contracts/core/streaming-billing/README.md`.
  - Auth: wallet signing and contract-admin/operator addresses passed through instantiate and update messages in files like `contracts/core/registry/src/msg.rs`, `contracts/nft/marketplace-v3/src/msg.rs`, and `contracts/nft/minter-v2/src/msg.rs`.

**On-Chain Registry Interface:**
- `registry` is the source-of-truth integration for ecosystem approval, collection registration, mint authorization, and trading moderation.
  - SDK/Client: `query_wasm_smart` and `WasmMsg::Execute` usage in `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, and `contracts/nft/marketplace-v3/src/contract/helpers.rs`.
  - Auth: authorized factory wiring and multisig-admin patterns described in `docs/01-end-to-end-setup.md` and `docs/04-multisig-governance.md`.

**NFT Collection / Royalty Contracts:**
- `pg721` collections are instantiated or queried by factory, minter, marketplace, and auction contracts, and royalty payout can target either a wallet or a split contract.
  - SDK/Client: `WasmMsg::Instantiate` in `contracts/core/collection-factory/src/contract/execute.rs` and `contracts/nft/minter-v2/src/contract/instantiate.rs`; NFT owner and royalty queries in `contracts/nft/marketplace-v3/src/contract/helpers.rs`; royalty settlement execution in `contracts/nft/auction-english/src/execute.rs`.
  - Auth: minter address is embedded at collection instantiation time; optional registry-side `AuthorizeMinter` flow is described in `docs/01-end-to-end-setup.md` and enforced in `contracts/nft/minter-v2/src/contract/helpers.rs`.

**Split / Revenue Routing Contracts:**
- Royalty and revenue routing integrates with split contracts rather than a centralized payout service.
  - SDK/Client: `Split` execution handling in `contracts/core/split-router/src/contract/execute.rs`; payout execution from `contracts/core/streaming-billing/src/contract.rs`, `contracts/nft/marketplace-v3/src/contract/helpers.rs`, and `contracts/nft/auction-english/src/execute.rs`.
  - Auth: contract-configured admin in `contracts/core/split-router/src/contract/instantiate.rs` and admin-only updates in `contracts/core/split-router/src/contract/execute.rs`.

**Staking Contract Graph:**
- `vault-factory` creates `nft-vault` contracts with deterministic `instantiate2`, and `nft-vault` creates reward-account contracts from `stake-rewards`.
  - SDK/Client: `WasmMsg::Instantiate2` flows in `contracts/staking/vault-factory/src/contract.rs` and `contracts/staking/nft-vault/src/contract.rs`; reward claim/change calls between vault and rewards contracts in `contracts/staking/nft-vault/src/contract.rs`.
  - Auth: `only_contract_admin` checks from `uju_cw2_common` in `contracts/staking/vault-factory/src/contract.rs` and `contracts/staking/nft-vault/src/contract.rs`.

**CW20 Reward Token Integration:**
- Staking reward accounts can hold and distribute CW20 tokens in addition to native tokens.
  - SDK/Client: `Cw20QueryMsg::Balance` and `Cw20ExecuteMsg::Transfer` in `contracts/staking/nft-vault/src/contract.rs` and `contracts/staking/stake-rewards/src/contract.rs`.
  - Auth: the stake contract is the only caller allowed to mutate reward state in `contracts/staking/stake-rewards/src/contract.rs`.

**Stripe / Fiat Billing Bridge (docs and partial contract surface):**
- `streaming-billing` defines the on-chain side of a fiat-to-PASG purchase bridge, including fiat purchase reporting and webhook-validation placeholders.
  - SDK/Client: contract fields and execute/query messages in `contracts/core/streaming-billing/src/msg.rs` and `contracts/core/streaming-billing/src/contract.rs`; off-chain examples in `contracts/core/streaming-billing/README.md`.
  - Auth: `fiat_oracle` and `stripe_webhook_validator` addresses are contract-configured in `contracts/core/streaming-billing/src/msg.rs` and validated in `contracts/core/streaming-billing/src/contract.rs`.

**Price Oracle / Market Data (docs-only):**
- The streaming billing design references external PASG pricing inputs to convert USD to PASG before writing a purchase on-chain.
  - SDK/Client: Osmosis and CoinGecko/CoinMarketCap are described as example data sources in `contracts/core/streaming-billing/README.md`.
  - Auth: none detected in the repo because this price-fetching logic is documented, not implemented here.

## Data Storage

**Databases:**
- None detected.
  - Connection: Not applicable.
  - Client: Persistent application state lives in on-chain CosmWasm storage through `cw-storage-plus` in files such as `contracts/core/registry/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`, `contracts/core/streaming-billing/src/state.rs`, `contracts/staking/nft-vault/src/contract.rs`, and `contracts/staking/stake-rewards/src/contract.rs`.

**File Storage:**
- Local filesystem only.
  - Source and manifests: repository root plus `contracts/`.
  - Built artifacts: `artifacts/`.
  - Generated schemas: `schema/` and per-contract folders like `contracts/core/registry/schema/` and `contracts/nft/pg721/schema/`.

**Caching:**
- No runtime cache service detected.
- Build-time Docker volume caches are used in `scripts/optimize.sh` and `scripts/optimize-arm.sh` via the `*_cache` and `registry_cache` mounts.

## Authentication & Identity

**Auth Provider:**
- Custom CosmWasm address-based authorization.
  - Implementation: admin/operator fields, contract ownership, whitelist membership, and multisig governance in `contracts/core/registry/src/msg.rs`, `contracts/core/multisig/README.md`, `contracts/nft/whitelist/Cargo.toml`, `contracts/nft/minter-v2/src/contract/helpers.rs`, and `docs/04-multisig-governance.md`.

## Monitoring & Observability

**Error Tracking:**
- None detected.

**Logs:**
- The main observability mechanism is contract events and attributes emitted from responses, for example in `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/nft/minter-v2/src/contract/instantiate.rs`, `contracts/nft/auction-english/src/execute.rs`, and `contracts/staking/nft-vault/src/contract.rs`.
- The streaming billing design recommends webhook logging, suspicious-activity monitoring, and monitoring dashboards in `contracts/core/streaming-billing/README.md`, but those operational services are not implemented in this repo.

## CI/CD & Deployment

**Hosting:**
- Smart contracts are deployed to the Passage chain; no web application hosting platform is defined in this repository.

**CI Pipeline:**
- GitHub Actions in `.github/workflows/main.yaml`.
- The workflow runs on `ubuntu-latest`, installs the wasm target, builds with `cargo wasm`, and runs `cargo unit-test`.
- Production Wasm optimization is a manual Docker-based step in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.

## Environment Configuration

**Required env vars:**
- None required by the Rust contract workspace itself.
- The documented streaming-billing companion services expect `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, and `RPC_URL` in `contracts/core/streaming-billing/README.md`.

**Secrets location:**
- No checked-in secret files were read, and no root `.env*` files were detected.
- Off-chain secrets are expected to live in the environment of the external backend/frontend that implements the examples from `contracts/core/streaming-billing/README.md`.

## Webhooks & Callbacks

**Incoming:**
- GitHub Actions receives repository event triggers on `push`, `pull_request`, and `workflow_dispatch` in `.github/workflows/main.yaml`.
- A Stripe webhook endpoint at `/webhooks/stripe` is documented in `contracts/core/streaming-billing/README.md`; this is an example integration surface, not an implementation in this repo.
- On-chain reply callbacks capture instantiated contract addresses in `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, and `contracts/nft/minter-v2/src/contract/instantiate.rs`.

**Outgoing:**
- Contract executions to other on-chain modules occur in `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/nft/marketplace-v3/src/contract/helpers.rs`, and `contracts/nft/auction-english/src/execute.rs`.
- Smart-contract queries to `registry`, `pg721`, `whitelist`, and CW20 contracts occur in `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/staking/nft-vault/src/contract.rs`, and `contracts/staking/stake-rewards/src/contract.rs`.

---

*Integration audit: 2026-03-17*
