# External Integrations

**Analysis Date:** 2026-03-18

## APIs & External Services

**Passage chain / CosmWasm runtime:**
- Primary deployment and execution target for all contracts in the workspace.
  - SDK/Client: `passage tx wasm` in `README.md`, CosmWasm messages in contract code, and `cw-multi-test` in tests.
  - Auth: wallet signatures plus admin/operator/registry addresses passed through instantiate and execute messages in files like `contracts/core/registry/src/msg.rs`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/nft/marketplace-v3/src/msg.rs`.

**On-chain registry and governance contracts:**
- `registry` is the source-of-truth dependency for collection registration, mint authorization, moderation, and several factory flows.
  - SDK/Client: `WasmQuery::Smart` and `WasmMsg::Execute` usage in `contracts/core/collection-factory/src/contract.rs`, `contracts/core/ecosystem-factory/src/contract.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/nft/marketplace-v3/src/contract/helpers.rs`, and `contracts/core/streaming-billing/src/contract.rs`.
  - Auth: registry-backed approvals and admin/operator checks are enforced on-chain.

**NFT collection and minting contracts:**
- `pg721`, `pg721-updatable`, `minter-v2`, `marketplace-v3`, `auction-english`, `royalty-group`, and `whitelist` interact for collections, minting, trading, and royalties.
  - SDK/Client: `WasmMsg::Instantiate`, owner/metadata queries, and royalty/trade helper calls in the files above.
  - Auth: minter and registry addresses are passed via instantiate and migrate messages; some flows require collection ownership or allowlist approval.

**Revenue routing:**
- `split-router` receives revenue payouts from `marketplace-v3`, `auction-english`, and `streaming-billing`.
  - SDK/Client: execute messages in `contracts/core/split-router/src/contract.rs` and downstream execute calls in the paying contracts.
  - Auth: contract-admin controls split configuration; `contracts/core/split-router/MIGRATION.md` says it has no migrate entrypoint.

**Staking graph:**
- `vault-factory`, `nft-vault`, and `stake-rewards` form the staking / reward-instantiation chain.
  - SDK/Client: `WasmMsg::Instantiate2`, CW20 balance queries, and reward-account contract calls in `contracts/staking/vault-factory/src/contract.rs`, `contracts/staking/nft-vault/src/contract.rs`, and `contracts/staking/stake-rewards/src/contract.rs`.
  - Auth: admin-only and contract-admin checks are implemented via `uju-cw2-common`.

**CW20 reward assets:**
- Used by staking rewards and NFT vaults for reward distribution.
  - SDK/Client: `Cw20QueryMsg::Balance` and `Cw20ExecuteMsg::Transfer` in `contracts/staking/nft-vault/src/contract.rs` and `contracts/staking/stake-rewards/src/contract.rs`.
  - Auth: reward mutation is restricted to the staking contract call path.

**Stripe / fiat bridge (docs and partial on-chain surface):**
- `streaming-billing` models the on-chain side of a fiat-to-PASG bridge, but Stripe handling stays off-chain.
  - SDK/Client: fields and execute/query messages in `contracts/core/streaming-billing/src/msg.rs` and `contracts/core/streaming-billing/src/contract.rs`.
  - Auth: `fiat_oracle` and optional `stripe_webhook_validator` config fields are stored on-chain, but the README in `contracts/core/streaming-billing/README.md` says Stripe signature verification is not implemented on-chain.

**Price feeds (docs-only):**
- The streaming billing design references external market-data sources for fiat-to-PASG conversion, but no live price-feed integration is implemented in the repo.
  - SDK/Client: referenced in `contracts/core/streaming-billing/README.md`.
  - Auth: none in repo.

## Data Storage

**Databases:**
- None.
  - Connection: not applicable.
  - Client: persistent application state lives in on-chain CosmWasm storage through `cw-storage-plus` in files such as `contracts/core/registry/src/state.rs`, `contracts/core/streaming-billing/src/state.rs`, `contracts/nft/marketplace-v3/src/state.rs`, `contracts/staking/nft-vault/src/state.rs`, and `contracts/staking/stake-rewards/src/state.rs`.

**File Storage:**
- Local filesystem only.
  - Source and manifests: repository root plus `contracts/`.
  - Built artifacts: `artifacts/`.
  - Generated schemas: `schema/` and per-contract folders like `contracts/core/registry/schema/` and `contracts/nft/pg721/schema/`.

**Caching:**
- No runtime cache service detected.
- Build-time Docker volume caches are used in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.

## Authentication & Identity

**Auth Provider:**
- Custom CosmWasm address-based authorization.
  - Implementation: admin/operator fields, contract ownership, whitelist membership, and multisig governance in `contracts/core/registry/src/msg.rs`, `contracts/core/multisig/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, `contracts/nft/marketplace-v3/src/msg.rs`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/staking/vault-factory/src/contract.rs`, `contracts/staking/nft-vault/src/contract.rs`, and `contracts/staking/stake-rewards/src/contract.rs`.

## Monitoring & Observability

**Error Tracking:**
- None detected.

**Logs:**
- The main observability mechanism is contract events and attributes emitted from execute and reply handlers, for example in `contracts/core/registry/src/contract.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/nft/marketplace-v3/src/contract/execute.rs`, and `contracts/staking/nft-vault/src/contract.rs`.
- GitHub Actions logs in `.github/workflows/main.yaml` cover build and test output.

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
- The documented streaming-billing companion flows expect `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, and `RPC_URL` in `contracts/core/streaming-billing/README.md`.

**Secrets location:**
- No checked-in secret files were read, and no root `.env*` files were detected.
- Off-chain secrets are expected to live in the environment of the external backend or frontend that implements the examples from `contracts/core/streaming-billing/README.md`.

## Webhooks & Callbacks

**Incoming:**
- GitHub Actions workflow triggers in `.github/workflows/main.yaml`.
- A Stripe webhook endpoint is documented in `contracts/core/streaming-billing/README.md`; this is an off-chain companion surface, not an in-repo webhook implementation.
- On-chain reply callbacks capture instantiated contract addresses in `contracts/core/ecosystem-factory/src/contract/reply.rs` and `contracts/core/collection-factory/src/contract/reply.rs`.

**Outgoing:**
- Contract executions to other on-chain modules occur in `contracts/core/ecosystem-factory/src/contract/reply.rs`, `contracts/core/collection-factory/src/contract/reply.rs`, `contracts/core/streaming-billing/src/contract.rs`, `contracts/nft/marketplace-v3/src/contract/helpers.rs`, and `contracts/nft/auction-english/src/execute.rs`.
- Smart-contract queries to `registry`, `pg721`, `whitelist`, and CW20 contracts occur in `contracts/core/ecosystem-factory/src/contract/execute.rs`, `contracts/core/collection-factory/src/contract/execute.rs`, `contracts/nft/minter-v2/src/contract/helpers.rs`, `contracts/staking/nft-vault/src/contract.rs`, and `contracts/staking/stake-rewards/src/contract.rs`.

---

*Integration audit: 2026-03-18*

