# multisig

## Status

Current

## Purpose

Serves as the stable admin-owner execution plane for protocol contracts such as registry and marketplace surfaces.

## Instantiation

Instantiate with the signer/admin set, proposal thresholds, timing rules, and any initial admin-owned contract relationships that this multisig must control.

Real payload source of truth: `contracts/core/multisig/src/msg.rs` and, when available, the crate `schema/` output.

## Actors and permissions

Multisig members, protocol contracts owned by the multisig, and PASG governance when it hands over ratified admin actions.

## Key messages

- Creates, votes on, and executes admin proposals.
- Owns the final execution step for protocol admin changes.
- Acts as the bounded bridge target for PASG governance admin handoff.

## Relationships

- Owns contracts like `registry` and can execute updates on them.
- Receives typed ratified actions from `pasg-governance`.
- Should not be confused with the PASG voting layer itself.

## Hypothetical example

Hypothetical flow: PASG governance ratifies a marketplace config update, then multisig mirrors that action into a normal admin proposal and executes the change on-chain.

## References

- Code: `contracts/core/multisig/src/msg.rs`
- Local context: `contracts/core/multisig/README.md`
- Shared guide: `docs/en/relationships.md`
- Instantiate guide: `docs/en/instantiate-guides.md`
