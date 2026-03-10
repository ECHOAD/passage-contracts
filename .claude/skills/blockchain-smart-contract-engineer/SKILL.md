---
name: blockchain-smart-contract-engineer
description: >
  Skill profile for the Blockchain / Smart Contract Engineer role within the Passage ecosystem.
  This role is responsible for the PASG token contracts and on-chain mechanisms: token transfers,
  governance voting, staking, fee payment discounts, and in-world currency support. PASG is an
  economic instrument within Passage — NOT the economic operating system itself. Use this skill
  when planning, scoping, hiring, or assigning work related to PASG token contracts, governance
  modules, staking mechanics, on-chain fee discount logic, NFT asset contracts for worlds,
  multi-economy token standards, or any smart contract work that supports Passage's platform-level
  economic capabilities. Also trigger when discussing the separation between Passage (platform)
  and PASG (token) at the protocol level.
---

# Blockchain / Smart Contract Engineer — Passage Ecosystem

## Role Summary

This engineer owns the **on-chain protocol layer** — specifically the PASG token and its associated smart contract mechanics. Their scope is deliberately bounded: PASG is a payment, governance, and staking instrument *within* the Passage economic operating system. The platform-level economic logic (billing, subscriptions, revenue routing) lives off-chain in Passage services; this role builds the on-chain primitives those services interact with.

### Critical Architectural Distinction

> **Passage** = the economic operating system (monetization framework, revenue routing, billing orchestrator, subscription engine).
>
> **PASG** = one of the economic instruments within that system (payment token, governance token, staking token, potential in-world currency).
>
> **PASG is not equity in Passage.** These must remain conceptually and technically separate.

## Core Responsibilities

### PASG Token Contract
- Design and implement the PASG token contract (mint, burn, transfer, delegation).
- Ensure the token supports its four designated roles: payments, governance, staking, and in-world currency.
- Implement fee payment discount logic: users paying with PASG may receive reduced platform fees.
- Maintain clear separation — the token contract must not embed platform business logic (subscriptions, billing, tier access).

### Governance Contracts
- Build the on-chain governance module: proposal creation, voting, execution.
- Define voting weight mechanics (token-weighted, delegation, quorum thresholds).
- Implement parameter change flows: governance can adjust protocol-level parameters (staking rates, discount tiers, fee structures) without redeployment.
- Ensure governance scope is limited to PASG protocol parameters, not Passage platform operations.

### Staking Mechanics
- Design and implement staking contracts: lock periods, reward distribution, unstaking cooldowns.
- Define reward sources and emission schedules.
- Build incentive alignment mechanisms: staking should correlate creator/user engagement with ecosystem health (Risk §9.3).
- Implement slashing conditions if applicable.

### NFT & Asset Contracts
- Build or standardize NFT contracts for in-world assets (items, access passes, collectibles).
- Support creator-defined royalty structures on secondary sales.
- Enable worlds to deploy NFT collections through standardized factory contracts.
- Ensure assets are interoperable across worlds where operators opt in.

### Multi-Economy Token Support
- Build the framework for worlds to deploy custom tokens or point systems that can settle against PASG.
- Define interfaces for hybrid payment systems: PASG + custom tokens + internal currencies.
- Implement exchange/swap primitives or integrate with existing DEX infrastructure.
- Support NFT-based assets, internal currencies, and PASG-denominated systems simultaneously (§8.4).

### Revenue Split Primitives (On-Chain)
- Implement on-chain revenue split contracts that the platform's off-chain revenue routing service can invoke.
- Support multi-party splits (creator, platform, partners), escrow, and refund execution.
- These contracts are *called by* the Passage platform — they do not autonomously manage billing or subscriptions.

### Contract Upgradeability & Safety
- Implement upgradeable patterns (proxy, UUPS, or Diamond) for long-lived contracts.
- Build emergency pause/unpause mechanisms.
- Maintain version management for contract ABIs and migration paths.
- Ensure upgrades never disrupt active economic flows.

## Technical Stack (Expected)

- **Languages**: Solidity (EVM) or Rust/CosmWasm (Cosmos), depending on chain selection.
- **Frameworks**: Hardhat / Foundry (EVM) or cargo + cosmwasm-std (Cosmos).
- **Testing**: Unit tests, integration tests, fuzzing (Echidna/Medusa for EVM), invariant testing.
- **Libraries**: OpenZeppelin (EVM), cw-plus (Cosmos).
- **Standards**: ERC-20/ERC-721/ERC-1155 or CW-20/CW-721 equivalents.

## Key Interfaces

| Collaborates With | On What |
|---|---|
| Backend/Platform API Engineer | ABI definitions, event schemas for off-chain indexing, platform-to-contract call patterns for revenue splits and payments |
| Identity & Auth Engineer | Wallet abstraction integration, custodial signer flows, token-gated access checks |
| Integration/Infra Engineer | Gas relay infrastructure, blockchain node management, event indexing |
| DevOps/Protocol Ops Engineer | Deployment pipelines, upgrade orchestration, testnet management |

## Quality & Risk Criteria

- **Scope Discipline**: Contracts must not absorb platform logic. If billing, subscription, or tier enforcement logic creeps into contracts, the architecture is wrong.
- **Economic Bypass Prevention**: If PASG doesn't offer meaningful advantages (discounts, governance rights, staking rewards), creators and users will transact entirely outside the token (Risk §9.1).
- **Governance Integrity**: Governance must be clearly scoped to PASG parameters. Overreach into platform operations creates legal and operational risk.
- **Non-Equity Clarity**: Contract design must reinforce that PASG is not a security or equity instrument.

## Deliverables

1. PASG token contract (transfer, mint/burn, delegation, fee discount logic).
2. Governance module (proposals, voting, parameter changes, execution).
3. Staking contracts (lock, reward distribution, unstaking).
4. NFT factory and asset contracts (creator collections, royalties, access passes).
5. Multi-economy framework (custom token deployment, PASG settlement interfaces).
6. Revenue split execution contracts (invoked by platform services).
7. Upgradeability infrastructure (proxies, pause mechanisms, version management).
8. Full test suite (>95% branch coverage, fuzz campaigns) and audit-ready documentation.
