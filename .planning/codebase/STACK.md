# Technology Stack

**Analysis Date:** 2026-03-17

## Languages

**Primary:**
- Rust 1.85.x with workspace edition `2021` for the main contract workspace in `Cargo.toml`, including the `contracts/core/*`, modern `contracts/nft/*`, and `contracts/relationship/*` crates.

**Secondary:**
- Rust edition `2018` for the Sylvia-based staking crates in `contracts/staking/nft-vault/Cargo.toml` and `contracts/staking/vault-factory/Cargo.toml`.
- Bash for build optimization helpers in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.
- YAML for CI/bootstrap configuration in `.github/workflows/main.yaml` and `.gitpod.yml`.
- TypeScript/TSX appears only in documentation examples for off-chain integrations in `contracts/core/streaming-billing/README.md`; it is not part of the compiled workspace.

## Runtime

**Environment:**
- Rust toolchain `1.85.1` with `rustfmt`, `clippy`, and the `wasm32-unknown-unknown` target in `rust-toolchain.toml`.
- CosmWasm smart contracts compiled to WebAssembly and intended for deployment to Passage via the `passage tx wasm` CLI flow shown in `README.md`.

**Package Manager:**
- Cargo workspace management in `Cargo.toml`.
- Lockfile: present in `Cargo.lock`.
- JavaScript or Python package manager: not detected at repo root.

## Frameworks

**Core:**
- CosmWasm `2.1.3` for the modern workspace dependency set in `Cargo.toml`; this powers `registry`, `split-router`, `collection-factory`, `ecosystem-factory`, `marketplace-v3`, `minter-v2`, and the staking modules.
- CosmWasm `1.x` remains in legacy crates such as `contracts/nft/pg721/Cargo.toml`, `contracts/nft/minter/Cargo.toml`, `contracts/nft/marketplace-v2/Cargo.toml`, `contracts/nft/marketplace-legacy/Cargo.toml`, `contracts/relationship/follow/Cargo.toml`, and `contracts/relationship/friend/Cargo.toml`.
- `cw721` / `cw721-base` back the NFT collection and minting stack in `contracts/nft/pg721/Cargo.toml`, `contracts/nft/minter-v2/Cargo.toml`, `contracts/nft/minter-v2-metadata-onchain/Cargo.toml`, and `contracts/nft/marketplace-v3/Cargo.toml`.
- Sylvia `1.2.1` drives the staking contracts in `contracts/staking/nft-vault/Cargo.toml`, `contracts/staking/stake-rewards/Cargo.toml`, and `contracts/staking/vault-factory/Cargo.toml`.

**Testing:**
- `cw-multi-test` `=2.1.1` is the workspace default test framework in `Cargo.toml`.
- Legacy crates pin older `cw-multi-test` versions such as `0.13.x` and `0.16.2` in `contracts/nft/marketplace-v2/Cargo.toml`, `contracts/nft/pg721/Cargo.toml`, `contracts/nft/minter/Cargo.toml`, `contracts/relationship/follow/Cargo.toml`, and `contracts/relationship/friend/Cargo.toml`.

**Build/Dev:**
- Cargo aliases are defined in `.cargo/config.toml`: `cargo wasm`, `cargo wasm-debug`, and `cargo unit-test`.
- Dockerized Wasm optimization uses `cosmwasm/optimizer:0.16.1` in `scripts/optimize.sh` and `cosmwasm/optimizer-arm64:0.16.1` in `scripts/optimize-arm.sh`.
- GitHub Actions in `.github/workflows/main.yaml` installs the wasm target, runs `cargo wasm`, and runs `cargo unit-test`.
- Schema generation is a first-class build artifact pattern, with example binaries in files such as `contracts/core/registry/examples/schema.rs`, `contracts/nft/pg721/examples/schema.rs`, `contracts/nft/minter-v2/examples/schema.rs`, and committed outputs in `schema/` plus many `contracts/*/schema/` directories.

## Key Dependencies

**Critical:**
- `cosmwasm-std` `=2.1.3` with `staking`, `stargate`, and `cosmwasm_1_2` features in `Cargo.toml`; this is the base runtime for the current core and commerce contracts.
- `cosmwasm-schema` `=2.1.3` in `Cargo.toml` plus per-contract schema example binaries such as `contracts/core/registry/examples/schema.rs` and `contracts/staking/vault-factory/src/bin/schema.rs`.
- `cw2`, `cw-storage-plus`, `cw-utils`, and `thiserror` are workspace-standard building blocks in `Cargo.toml` and appear in essentially every modern contract manifest under `contracts/core/*`, `contracts/nft/auction-english/Cargo.toml`, and `contracts/nft/marketplace-v3/Cargo.toml`.
- `cw721` and `cw721-base` are central to the NFT contract family in `contracts/nft/pg721/Cargo.toml`, `contracts/nft/minter-v2/Cargo.toml`, and `contracts/nft/minter-v2-metadata-onchain/Cargo.toml`.
- `cw20` is used in staking reward flows in `contracts/staking/nft-vault/Cargo.toml` and `contracts/staking/stake-rewards/Cargo.toml`.

**Infrastructure:**
- `sylvia` is the contract framework for the staking stack in `contracts/staking/nft-vault/Cargo.toml`, `contracts/staking/stake-rewards/Cargo.toml`, and `contracts/staking/vault-factory/Cargo.toml`.
- `uju-cw2-common`, `uju-cw2-nft`, and `uju-index-query` support staking admin, instantiate2, NFT transfer, and query patterns in `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/stake-rewards/src/contract.rs`, and `contracts/staking/vault-factory/src/contract.rs`.
- `schemars` and `serde` provide serialization and schema support across the entire workspace in `Cargo.toml`.
- `sha2` is included in the workspace and used by staking crates in `contracts/staking/nft-vault/Cargo.toml` and `contracts/staking/vault-factory/Cargo.toml`.
- `url` is used in older collection/minter crates such as `contracts/nft/pg721/Cargo.toml`, `contracts/nft/pg721-updatable/Cargo.toml`, `contracts/nft/pg721-metadata-onchain/Cargo.toml`, and `contracts/nft/minter/Cargo.toml`.
- Local path dependencies connect the staking and NFT stacks in `Cargo.toml`: `nft-vault`, `stake-rewards`, `vault-factory`, `pg721`, and `pg721-updatable`.

## Configuration

**Environment:**
- The Rust contract workspace is configured primarily through Cargo manifests and on-chain instantiate/execute messages, not through runtime environment-variable loading in the Rust codebase.
- No root-level `.env*` files were detected during this pass.
- The practical runtime knobs are contract message fields like `registry`, `split_router`, `collection_code_id`, `cw721_code_id`, `denom`, `fee_collector`, and allowlist/admin/operator addresses in files such as `contracts/core/registry/src/msg.rs`, `contracts/core/collection-factory/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/nft/marketplace-v3/src/msg.rs`.
- Docs-only off-chain examples in `contracts/core/streaming-billing/README.md` reference `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, and `RPC_URL`, but those variables are not consumed by the Rust workspace itself.

**Build:**
- Workspace/build definition: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, and `.cargo/config.toml`.
- CI definition: `.github/workflows/main.yaml`.
- Optimization scripts: `scripts/optimize.sh` and `scripts/optimize-arm.sh`.
- Schema outputs: `schema/` at the repo root and committed per-contract schema folders such as `contracts/core/registry/schema/`, `contracts/nft/pg721/schema/`, and `contracts/nft/whitelist/schema/`.

## Platform Requirements

**Development:**
- Rustup with toolchain `1.85.1` and target `wasm32-unknown-unknown` as defined in `rust-toolchain.toml`.
- Docker is required for the optimized production Wasm flow in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.
- A Unix-like shell is assumed by the optimizer scripts in `scripts/*.sh`, even though the workspace itself builds with Cargo on Windows as well.
- Standard developer workflows are `cargo wasm`, `cargo unit-test`, and per-contract schema generation commands documented in `CLAUDE.md` and individual contract `README.md` files.

**Production:**
- Deployment target is the Passage chain, using `passage tx wasm` commands against chain ID `passage-2` and the `upasg` denom as shown in `README.md` and `docs/03-json-examples.md`.
- Output artifacts are Wasm binaries under `artifacts/` and JSON schemas under `schema/` or `contracts/*/schema/`.

---

*Stack analysis: 2026-03-17*
