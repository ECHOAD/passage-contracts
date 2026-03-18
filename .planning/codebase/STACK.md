# Technology Stack

**Analysis Date:** 2026-03-18

## Languages

**Primary:**
- Rust 1.85.x with workspace edition `2021` for the main contract workspace in `Cargo.toml`, covering crates such as `contracts/core/registry/Cargo.toml`, `contracts/core/collection-factory/Cargo.toml`, `contracts/core/ecosystem-factory/Cargo.toml`, `contracts/core/multisig/Cargo.toml`, `contracts/core/split-router/Cargo.toml`, `contracts/nft/marketplace-v3/Cargo.toml`, `contracts/nft/minter-v2/Cargo.toml`, and `contracts/relationship/follow/Cargo.toml`.

**Secondary:**
- Rust edition `2018` for the Sylvia-based staking crates in `contracts/staking/nft-vault/Cargo.toml`, `contracts/staking/stake-rewards/Cargo.toml`, and `contracts/staking/vault-factory/Cargo.toml`.
- Shell script for `scripts/optimize.sh` and `scripts/optimize-arm.sh`.
- YAML for `.github/workflows/main.yaml` and `.gitpod.yml`.

## Runtime

**Environment:**
- Rust toolchain `1.85.1` with `rustfmt`, `clippy`, and the `wasm32-unknown-unknown` target from `rust-toolchain.toml`.
- CosmWasm smart contracts compiled to WebAssembly and deployed with the Passage CLI flow shown in `README.md`.

**Package Manager:**
- Cargo workspace management in `Cargo.toml`.
- Lockfile: `Cargo.lock` present.
- No Node, npm, Bun, or Python runtime is part of the contract workspace root.

## Frameworks

**Core:**
- CosmWasm `2.1.3` in the workspace dependency set from `Cargo.toml` for the modern contract family.
- Older CosmWasm 1.x-era crates remain in legacy manifests such as `contracts/nft/pg721/Cargo.toml`, `contracts/nft/pg721-updatable/Cargo.toml`, `contracts/nft/whitelist/Cargo.toml`, `contracts/relationship/follow/Cargo.toml`, and `contracts/relationship/friend/Cargo.toml`.
- `cw721` / `cw721-base` `0.18` back the NFT collection and minting stack in `contracts/nft/pg721/Cargo.toml`, `contracts/nft/pg721-updatable/Cargo.toml`, `contracts/nft/minter-v2/Cargo.toml`, and `contracts/nft/marketplace-v3/Cargo.toml`.
- Sylvia `1.2.1` drives the staking contracts in `contracts/staking/nft-vault/Cargo.toml`, `contracts/staking/stake-rewards/Cargo.toml`, and `contracts/staking/vault-factory/Cargo.toml`.

**Testing:**
- `cw-multi-test` `=2.1.1` is the workspace default test framework in `Cargo.toml`.

**Build/Dev:**
- Cargo aliases are defined in `.cargo/config.toml`: `cargo wasm`, `cargo wasm-debug`, and `cargo unit-test`.
- Schema generation is a first-class build artifact pattern via per-contract `examples/schema.rs` files and `contracts/staking/nft-vault/src/bin/schema.rs`, `contracts/staking/stake-rewards/src/bin/schema.rs`, and `contracts/staking/vault-factory/src/bin/schema.rs`.
- Release Wasm packaging is handled by `scripts/optimize.sh` and `scripts/optimize-arm.sh`.

## Key Dependencies

**Critical:**
- `cosmwasm-std` `=2.1.3` and `cosmwasm-schema` `=2.1.3` - contract runtime and schema generation for the modern workspace.
- `cw2`, `cw-storage-plus`, and `cw-utils` - versioning, storage, and helper primitives used across the core contracts.
- `cw721` and `cw721-base` - NFT collection, minting, and query support.
- `cw20` - reward-asset support in staking contracts.
- `sylvia` - contract framework for the staking stack.
- `uju-cw2-common`, `uju-cw2-nft`, and `uju-index-query` - staking/admin and query helpers in `contracts/staking/nft-vault/src/contract.rs`, `contracts/staking/stake-rewards/src/contract.rs`, and `contracts/staking/vault-factory/src/contract.rs`.
- `schemars`, `serde`, `thiserror`, and `sha2` - serialization, schema, errors, and hashing across the workspace.

## Configuration

**Environment:**
- Contract behavior is configured through Cargo manifests and instantiate/execute messages, not through runtime environment loading in the Rust workspace.
- No root-level `.env*` files were detected in this pass.
- Practical runtime knobs are message fields such as `registry`, `split_router`, `denom`, `backend_operator`, `fiat_oracle`, `stripe_webhook_validator`, `admin`, and `operators` in files like `contracts/core/registry/src/msg.rs`, `contracts/core/streaming-billing/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, and `contracts/nft/marketplace-v3/src/msg.rs`.

**Build:**
- Workspace and toolchain definition live in `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, and `.cargo/config.toml`.
- CI definition lives in `.github/workflows/main.yaml`.
- Optimized Wasm build scripts live in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.

## Platform Requirements

**Development:**
- Rustup with toolchain `1.85.1` and target `wasm32-unknown-unknown`.
- Docker is required for the optimized production Wasm flow in `scripts/optimize.sh` and `scripts/optimize-arm.sh`.
- Any platform that can run Rust/Cargo locally is acceptable; the repo itself does not require a browser or Node runtime.

**Production:**
- Deployment target is the Passage chain, using `passage tx wasm` commands against chain ID `passage-2` and the `upasg` denom as shown in `README.md`.
- Build outputs are Wasm binaries under `artifacts/` and JSON schemas under `schema/` or contract-local folders such as `contracts/core/registry/schema/` and `contracts/nft/pg721/schema/`.

---

*Stack analysis: 2026-03-18*



