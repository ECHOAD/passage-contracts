# Phase 3: PASG Governance - Research

**Researched:** 2026-03-18  
**Domain:** PASG-scoped governance, native-denom voting power, delegation, quorum, and executable protocol changes  
**Confidence:** MEDIUM-HIGH

## Summary

Local repo evidence shows `contracts/core/multisig` is currently a fixed-member admin multisig, not a PASG-holder governance system. It supports member-gated proposal creation, one-vote-per-member approvals, self-call signer rotation, and unrestricted `CosmosMsg` execution once the threshold is reached. That means all three governance requirements remain open:

- GOV-01 is unmet because PASG holders cannot participate unless they are preconfigured members.
- GOV-02 is unmet because voting is not token-weighted, has no delegation, and uses a simple member threshold rather than quorum plus approval semantics.
- GOV-03 is only partially approximated by process discipline; the current contract can execute arbitrary messages and is not scoped to PASG and protocol parameters.

The safest planning path is to evolve the current `multisig` contract family into a PASG governance module instead of inventing an unrelated governance crate. The roadmap already names `multisig` as the canonical reference, and its proposal storage, pagination, and self-governance patterns are a strong base. The missing pieces are native `upasg` voting-power accounting, per-proposal voting snapshots, delegation state, quorum and approval rules, and an execution allowlist that limits governance to protocol-facing actions.

The most defensible inference from the current repo is that governance voting power should be backed by PASG controlled by the governance contract itself rather than by querying free wallet balances at vote time. Native bank balances are easy to query in the present but difficult to snapshot historically across proposal lifecycles; contract-controlled voting balances avoid that ambiguity and make delegation tractable. This remains compatible with Phase 4, where staking can later become the source or multiplier of governance power through explicit migration or adapter work.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GOV-01 | PASG holder can create protocol proposals, vote on them, and execute passed actions on-chain. | `contracts/core/multisig/src/msg.rs` and `contract.rs` already model proposal lifecycle, execution, and pagination, but only for hard-coded members. Extending this surface is lower risk than replacing it. |
| GOV-02 | Governance enforces token-weighted voting, delegation, quorum, and majority rules on-chain. | `contracts/core/multisig/src/state.rs` stores only member ballots and fixed thresholds. New state is required for voting balances, delegation edges, proposal snapshots, quorum, and approval math. |
| GOV-03 | Governance is limited to PASG and protocol parameters and cannot directly control general off-chain platform operations. | `docs/04-multisig-governance.md`, `CLAUDE.md`, and `../context/product/architecture/ONCHAIN_OFFCHAIN_BOUNDARIES.md` all reinforce that governance belongs to protocol policy, not general platform business operations. Current arbitrary-message execution needs explicit scoping. |
</phase_requirements>

## Architecture Patterns

### Pattern 1: Reuse `multisig` proposal lifecycle, replace member authority with PASG voting power
The current contract already has proposal IDs, ballots, status transitions, execution, pagination, and self-governance mechanics. The phase should preserve this foundation and swap fixed-member semantics for PASG-holder semantics instead of designing a second governance engine.

### Pattern 2: Use contract-controlled PASG balances for deterministic voting power snapshots
Native wallet balances are easy to inspect in the moment but poor as a historical voting-power source across a proposal window. The governance contract should likely accept PASG deposits or bonded voting balances, snapshot proposal voting power at creation, and record per-voter consumed or delegated power. This is an inference from current repo constraints.

### Pattern 3: Governance scope must be enforced by contract, not docs
The current `msgs: Vec<CosmosMsg<Empty>>` execution model is operationally flexible but too broad for PASG-only governance. Phase 3 needs a typed proposal-action surface or a strict allowlist validator for executable messages, so only protocol parameter or contract-governed actions can pass.

### Pattern 4: Keep staking integration deferred but leave a migration seam
Phase 4 is responsible for staking and rewards, but Phase 3 still needs real governance power now. Governance should use its own voting-balance primitive now, but its config and docs should explicitly leave a future adapter or migration path from deposited governance balances to staking-derived power.

### Anti-Patterns to Avoid
- Reusing free wallet balance at vote time without proposal snapshots.
- Preserving unrestricted arbitrary `CosmosMsg` execution under the name of PASG governance.
- Embedding platform subscriptions, streaming operations, or general business workflows into proposal execution or governance scope.
- Depending on Phase 4 staking code that does not exist yet as a prerequisite for Phase 3 voting.
- Leaving delegation as an off-chain convention.

## Likely Contract Insertion Points

### Primary
- `contracts/core/multisig/src/msg.rs`
- `contracts/core/multisig/src/contract.rs`
- `contracts/core/multisig/src/state.rs`
- `contracts/core/multisig/src/error.rs`
- `contracts/core/multisig/src/tests/governance.rs`
- `contracts/core/multisig/README.md`

### Secondary
- `docs/04-multisig-governance.md`
- `README.md`
- `CLAUDE.md`

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `cw-multi-test 2.1.1` |
| Quick run command | `cargo test -p multisig --lib --tests` |
| Secondary quick run | `cargo check -p multisig` |
| Full suite command | `cargo unit-test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|--------------|
| GOV-01 | PASG holders can create, vote, and execute passed governance actions | integration | `cargo test -p multisig --lib --tests` | Partial lifecycle coverage exists, PASG-holder path missing |
| GOV-02 | voting power, delegation, quorum, and approval rules are enforced on-chain | integration + negative cases | `cargo test -p multisig --lib --tests` | Missing |
| GOV-03 | execution is limited to PASG and protocol parameter actions | integration + auth and validation | `cargo test -p multisig --lib --tests`; `cargo unit-test` | Missing |

### Wave 0 Gaps
- [ ] `contracts/core/multisig/src/tests/governance.rs` only covers member-threshold lifecycle and self-call protection; no PASG deposit, delegation, quorum, or scoped-execution cases exist.
- [ ] No query surface currently exposes proposal snapshots, delegated voting power, or execution-scope metadata.
- [ ] No tests prove that non-governable messages are rejected before proposal creation or execution.

## Open Questions

1. Should PASG voting power come from direct governance deposits, escrowed shares, or a balance adapter?
Recommendation: use governance-controlled `upasg` deposits for Phase 3, then leave a migration seam for staking in Phase 4.

2. Should execution scoping be typed proposal actions or validated generic messages?
Recommendation: prefer typed proposal actions if the change set is manageable; otherwise validate a bounded set of `WasmMsg::Execute` targets and payload families with explicit allowlist rules.

3. Should proposer eligibility require a minimum deposited voting power or only any positive PASG balance?
Recommendation: add an explicit proposal threshold in config to deter spam and make it queryable.

## Sources

### Primary
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `CLAUDE.md`
- `.agents/skills/blockchain-smart-contract-engineer/SKILL.md`
- `contracts/core/multisig/src/msg.rs`
- `contracts/core/multisig/src/contract.rs`
- `contracts/core/multisig/src/state.rs`
- `contracts/core/multisig/src/tests/governance.rs`
- `contracts/core/multisig/README.md`
- `docs/04-multisig-governance.md`
- `..\context\product\architecture\ONCHAIN_OFFCHAIN_BOUNDARIES.md`

**Research date:** 2026-03-18  
**Valid until:** 2026-04-17
