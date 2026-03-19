# Multisig Governance

This guide explains the recommended Passage admin pattern: use `multisig` as the admin address for `registry` and other critical contracts.

## Why `multisig`

Recommended reasons:

- one signer compromise does not automatically compromise the full admin surface
- admin actions become auditable proposals instead of direct wallet actions
- signer rotation can happen without changing the multisig address

## Recommended ownership model

For `registry`:

- `registry InstantiateMsg.admin = <multisig_addr>`
- CosmWasm instance admin for `registry` = `<multisig_addr>`
- `operators` remain separate and should be less trusted than the multisig

## Deployment order

1. Store and instantiate `multisig`.
2. Store and instantiate `registry`.
3. In `registry` instantiate:
   - set `admin = <multisig_addr>`
4. In the CosmWasm instantiate transaction:
   - set instance admin to `<multisig_addr>`
5. Deploy `ecosystem-factory`.
6. Use a multisig proposal to wire `registry.UpdateConfig { ecosystem_factory }`.

## How proposals work

1. A signer sends `multisig.Propose`.
2. The proposal contains one or more `CosmosMsg` messages.
3. Other signers vote with `Approve` or `Reject`.
4. Once the threshold is reached, anyone can call `multisig.Execute`.

Important detail:

- if the proposal wraps a `WasmMsg::Execute`, the inner contract message is stored as base64-encoded binary

## Example: update `registry` config through `multisig`

Inner `registry` message:

```json
{
  "update_config": {
    "admin": null,
    "operators": ["passage1ops1...", "passage1ops2..."],
    "ecosystem_factory": "passage1factory...",
    "paused": null
  }
}
```

Outer `multisig` proposal:

```json
{
  "propose": {
    "title": "Wire ecosystem factory",
    "description": "Authorize the deployed ecosystem factory in registry",
    "msgs": [
      {
        "wasm": {
          "execute": {
            "contract_addr": "passage1registry...",
            "msg": "<base64 of the inner registry message>",
            "funds": []
          }
        }
      }
    ]
  }
}
```

Then:

```json
{
  "vote": {
    "proposal_id": 1,
    "vote": "approve"
  }
}
```

And finally:

```json
{
  "execute": {
    "proposal_id": 1
  }
}
```

## Example: rotate a compromised signer

Because `UpdateMembers` is self-call only, signer rotation is done by proposing a `WasmMsg::Execute` that targets the multisig itself.

Inner `multisig` message:

```json
{
  "update_members": {
    "members": [
      "passage1signer1...",
      "passage1signer2...",
      "passage1signer4..."
    ],
    "threshold": 2,
    "max_voting_period_secs": 86400
  }
}
```

Outer proposal:

```json
{
  "propose": {
    "title": "Replace compromised signer",
    "description": "Remove signer3 and add signer4",
    "msgs": [
      {
        "wasm": {
          "execute": {
            "contract_addr": "passage1multisig...",
            "msg": "<base64 of the inner multisig message>",
            "funds": []
          }
        }
      }
    ]
  }
}
```

## Operational guidance

Recommended defaults for Passage:

- use a contract multisig, not a native multisig account
- use `2 of 3` for a small operating team or `3 of 5` for a more institutional setup
- use hardware wallets for all signers
- avoid giving `operators` the same authority as the multisig

What the multisig should own:

- `registry`
- `ecosystem-factory`
- `marketplace-v3`
- `auction-english`
- any future contract with upgrade or config authority
