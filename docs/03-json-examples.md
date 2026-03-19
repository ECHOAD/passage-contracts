# JSON Examples

This file contains example JSON payloads for the main contracts in the Passage commerce flow.

## Conventions

- Use placeholders like `<registry_addr>` or `<collection_addr>` and replace them with real addresses.
- `Decimal` values are typically strings, for example `"0.05"`.
- `Timestamp` values in these contracts are safest when treated as nanosecond strings.
- When a message requires funds, the amount does not go inside the JSON message itself; it is attached to the transaction.
- Enums serialized with `cw_serde` generally use `snake_case`.
- `RevenueEventType` is an exception in the current code and uses names such as `PrimarySale` or `SecondaryRoyalty`.
- In `multisig`, a `WasmMsg::Execute` stores its inner contract message as base64-encoded binary. The examples below show both the human-readable inner message and the outer multisig payload.

## 0. `multisig`

### Instantiate

```json
{
  "members": [
    "passage1signer1...",
    "passage1signer2...",
    "passage1signer3..."
  ],
  "threshold": 2,
  "max_voting_period_secs": 86400
}
```

### Propose: pause `registry`

Human-readable inner `registry` execute message:

```json
{
  "update_config": {
    "admin": null,
    "operators": null,
    "ecosystem_factory": null,
    "paused": true
  }
}
```

Outer `multisig` proposal payload:

```json
{
  "propose": {
    "title": "Pause registry",
    "description": "Emergency pause after suspicious admin activity",
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

### Vote on a proposal

```json
{
  "vote": {
    "proposal_id": 1,
    "vote": "approve"
  }
}
```

### Execute a passed proposal

```json
{
  "execute": {
    "proposal_id": 1
  }
}
```

### Rotate multisig members

Human-readable inner `multisig` self-call:

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

Outer `multisig` proposal payload:

```json
{
  "propose": {
    "title": "Rotate compromised signer",
    "description": "Replace signer3 with signer4",
    "msgs": [
      {
        "wasm": {
          "execute": {
            "contract_addr": "passage1multisig...",
            "msg": "<base64 of the inner update_members message>",
            "funds": []
          }
        }
      }
    ]
  }
}
```

### Query: check whether a proposal can execute

```json
{
  "can_execute": {
    "proposal_id": 1
  }
}
```

## 1. `registry`

### Instantiate

```json
{
  "admin": "passage1multisig...",
  "operators": ["passage1ops1...", "passage1ops2..."],
  "recovery_council": ["passage1recovery1...", "passage1recovery2..."],
  "ecosystem_factory": null
}
```

### Rotate the authorized ecosystem factory

```json
{
  "update_config": {
    "admin": null,
    "operators": null,
    "recovery_council": null,
    "ecosystem_factory": "passage1newfactory...",
    "paused": null
  }
}
```

`registry` does not expose direct ecosystem creation. Ecosystems are created by approving requests in `ecosystem-factory`, which then calls `register_ecosystem_from_factory` internally.

### Approve ecosystem member

```json
{
  "approve_ecosystem_member": {
    "ecosystem_id": "music",
    "member": "passage1teammember..."
  }
}
```

### Register already deployed collection

```json
{
  "register_existing_collection": {
    "address": "passage1collection...",
    "ecosystem_id": "music",
    "name": "Genesis Music Collection",
    "creator": "passage1creator..."
  }
}
```

### Store collection runtime pointers

```json
{
  "update_collection": {
    "address": "passage1collection...",
    "name": null,
    "verified": true,
    "minter": "passage1minter...",
    "marketplace": "passage1marketplace..."
  }
}
```

### Moderate a creator

```json
{
  "update_creator_moderation": {
    "creator": "passage1creator...",
    "ecosystem_creation_enabled": true,
    "collection_creation_enabled": true,
    "mint_enabled": true,
    "trade_enabled": false,
    "reason": "Trading temporarily disabled for compliance review"
  }
}
```

### Moderate an ecosystem

```json
{
  "update_ecosystem_moderation": {
    "ecosystem_id": "music",
    "collection_creation_enabled": true,
    "mint_enabled": true,
    "trade_enabled": false,
    "reason": "Marketplace activity paused at ecosystem scope"
  }
}
```

### Moderate a collection

```json
{
  "update_collection_moderation": {
    "address": "passage1collection...",
    "mint_enabled": false,
    "trade_enabled": false,
    "reason": "Collection under review"
  }
}
```

### Set ecosystem recovery policy

```json
{
  "set_ecosystem_recovery_policy": {
    "ecosystem_id": "music",
    "delegate": "passage1trusteddelegate...",
    "designated_successor": "passage1backupowner..."
  }
}
```

### Set collection recovery policy

```json
{
  "set_collection_recovery_policy": {
    "address": "passage1collection...",
    "delegate": "passage1trusteddelegate...",
    "designated_successor": "passage1backupowner..."
  }
}
```

### Update recovery config

```json
{
  "update_recovery_config": {
    "abandonment_inactivity_period_secs": 7776000,
    "contest_period_secs": 2592000
  }
}
```

### Open lost-access recovery case

```json
{
  "open_recovery_case": {
    "case_kind": "lost_access",
    "target": {
      "collection": {
        "address": "passage1collection..."
      }
    },
    "reason": "Creator lost custody of the original wallet",
    "evidence_url": "https://support.example.com/case/123",
    "proposed_replacement": "passage1newowner..."
  }
}
```

### Open abandonment recovery case

```json
{
  "open_recovery_case": {
    "case_kind": "abandonment",
    "target": {
      "ecosystem": {
        "ecosystem_id": "music"
      }
    },
    "reason": "Project appears abandoned and team is unreachable",
    "evidence_url": "https://forum.example.com/thread/456",
    "proposed_replacement": "passage1newadmin..."
  }
}
```

### Contest a recovery case

```json
{
  "contest_recovery_case": {
    "case_id": 1,
    "note": "Control has not been abandoned and original owner remains active"
  }
}
```

### Resolve a recovery case

```json
{
  "resolve_recovery_case": {
    "case_id": 1,
    "approved": true,
    "note": "Approved after contest window and off-chain review"
  }
}
```

### Authorize minter

```json
{
  "authorize_minter": {
    "collection_address": "passage1collection...",
    "minter_address": "passage1minter..."
  }
}
```

### Query: get collection

```json
{
  "collection": {
    "address": "passage1collection..."
  }
}
```

### Query: validate minter

```json
{
  "is_minter_authorized": {
    "collection_address": "passage1collection...",
    "minter_address": "passage1minter..."
  }
}
```

### Query: get collection recovery policy

```json
{
  "collection_recovery_policy": {
    "address": "passage1collection..."
  }
}
```

### Query: get recovery case

```json
{
  "recovery_case": {
    "case_id": 1
  }
}
```

## 2. `ecosystem-factory`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "operators": ["passage1ops1..."],
  "registry": "passage1registry...",
  "collection_factory_code_id": 201,
  "collection_code_id": 301
}
```

### Submit ecosystem creation request

```json
{
  "submit_ecosystem_creation_request": {
    "id": "music",
    "name": "Music Ecosystem",
    "description": "Ecosystem for artists and music collections",
    "image_urls": ["https://cdn.example.com/music.png"],
    "animation_url": null,
    "url": "https://example.com/music"
  }
}
```

### Approve request

```json
{
  "resolve_ecosystem_creation_request": {
    "request_id": 1,
    "approved": true,
    "note": "Approved by governance"
  }
}
```

## 3. `collection-factory`

### Instantiate

```json
{
  "admin": "passage1ecosystemadmin...",
  "operators": ["passage1operator..."],
  "registry": "passage1registry...",
  "ecosystem_id": "music",
  "collection_code_id": 301,
  "enforce_local_allowlist": false,
  "approved_creators": null
}
```

### Approve local creator

```json
{
  "approve_creator": {
    "creator": "passage1creator..."
  }
}
```

### Create collection

```json
{
  "create_collection": {
    "name": "Genesis Music Collection",
    "symbol": "GMC",
    "minter": "passage1creator...",
    "collection_info": {
      "description": "Genesis collection for the music ecosystem",
      "image": "ipfs://bafy.../cover.png",
      "external_link": "https://example.com/gmc",
      "royalty_info": {
        "payment_address": "passage1royaltywallet...",
        "share": "0.05"
      }
    },
    "label": "gmc-mainnet"
  }
}
```

### Query: collections by creator

```json
{
  "collections_by_creator": {
    "creator": "passage1creator...",
    "start_after": null,
    "limit": 20
  }
}
```

## 4. `pg721`

`pg721` is usually deployed by `collection-factory` or `minter-v2`, but this is the instantiate shape.

### Direct instantiate

```json
{
  "name": "Genesis Music Collection",
  "symbol": "GMC",
  "minter": "passage1creator...",
  "collection_info": {
    "creator": "passage1creator...",
    "description": "Genesis collection for the music ecosystem",
    "image": "ipfs://bafy.../cover.png",
    "external_link": "https://example.com/gmc",
    "royalty_info": {
      "payment_address": "passage1royaltywallet...",
      "share": "0.05"
    }
  }
}
```

### Approve marketplace or auction for one token

```json
{
  "approve": {
    "spender": "passage1marketplace_or_auction...",
    "token_id": "1",
    "expires": null
  }
}
```

### Approve global operator

```json
{
  "approve_all": {
    "operator": "passage1marketplace_or_auction...",
    "expires": null
  }
}
```

### Query: collection info

```json
{
  "collection_info": {}
}
```

## 5. `streaming-billing`

`streaming-billing` is the canonical PASG utility query surface in this repo.

### Instantiate

```json
{
  "admin": "passage1admin...",
  "split_router": "passage1splitrouter...",
  "registry": "passage1registry...",
  "backend_operator": "passage1backend...",
  "denom": "upasg",
  "points_per_denom": "100",
  "fiat_oracle": "passage1fiatoracle...",
  "stripe_webhook_validator": null
}
```

### Deposit PASG directly for points

Attach the PASG funds in the transaction.

```json
{
  "deposit_crypto": {}
}
```

### Report fiat purchase

```json
{
  "report_fiat_purchase": {
    "user": "passage1buyer...",
    "fiat_amount_usd": "1500",
    "pasg_amount": "1000000",
    "points_awarded": "100",
    "transaction_id": "pi_123456789",
    "timestamp": "1773597600000000000"
  }
}
```

### Configure world rate

```json
{
  "set_world_rate": {
    "world_nft_id": "world-1",
    "world_collection": "passage1collection...",
    "points_per_hour": "250"
  }
}
```

### Distribute world revenue

```json
{
  "distribute_world_revenue": {
    "world_nft_id": "world-1"
  }
}
```

### Query: canonical PASG utility

```json
{
  "pasg_utility": {}
}
```

### Query: conversion rate

```json
{
  "conversion_rate": {}
}
```

### Query: pending revenue

```json
{
  "pending_revenue": {
    "world_nft_id": "world-1"
  }
}
```

## 6. `split-router`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "recipients": [
    {
      "address": "passage1creator...",
      "share": "0.85",
      "label": "Creator"
    },
    {
      "address": "passage1collab...",
      "share": "0.15",
      "label": "Collaborator"
    }
  ],
  "active": true
}
```

### Update split recipients

```json
{
  "update_split": {
    "recipients": [
      {
        "address": "passage1creator...",
        "share": "0.80",
        "label": "Creator"
      },
      {
        "address": "passage1producer...",
        "share": "0.20",
        "label": "Producer"
      }
    ],
    "active": true
  }
}
```

### Split attached funds

Attach the native funds in the transaction.

```json
{
  "split": {}
}
```

### Route world revenue

Attach the native funds in the transaction. This is the compatibility execute shape used by `streaming-billing`.

```json
{
  "route_world_revenue": {
    "world_nft_id": "world-1",
    "world_collection": "passage1collection..."
  }
}
```

### Query: preview split

```json
{
  "preview_split": {
    "funds": [
      {
        "denom": "upasg",
        "amount": "1000000"
      }
    ]
  }
}
```

### Query: routing metadata

```json
{
  "routing_metadata": {}
}
```

## 7. `marketplace-v3`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "min_price": "100000",
  "trading_fee_bps": 250,
  "fee_collector": "passage1treasury...",
  "registry": "passage1registry...",
  "operators": ["passage1operator..."]
}
```

### Submit collection registration request

```json
{
  "submit_collection_registration_request": {
    "collection": "passage1collection...",
    "denom": "upasg",
    "note": "request access to trade in marketplace-v3"
  }
}
```

### Resolve collection registration request

```json
{
  "resolve_collection_registration_request": {
    "collection": "passage1collection...",
    "approved": true,
    "denom": null,
    "note": "approved"
  }
}
```

### Admin direct register collection

```json
{
  "register_collection": {
    "collection": "passage1collection...",
    "denom": "upasg"
  }
}
```

### Submit collection update request

```json
{
  "submit_collection_update_request": {
    "collection": "passage1collection...",
    "active": true,
    "denom": "uion",
    "note": "switch denom"
  }
}
```

### Resolve collection update request

```json
{
  "resolve_collection_update_request": {
    "collection": "passage1collection...",
    "approved": true,
    "active": null,
    "denom": null,
    "note": "approved"
  }
}
```

### Publish fixed-price listing

```json
{
  "set_ask": {
    "collection": "passage1collection...",
    "token_id": "1",
    "price": {
      "denom": "upasg",
      "amount": "1000000"
    },
    "funds_recipient": null
  }
}
```

### Buy fixed-price listing

Attach exactly `1000000upasg` to the transaction.

```json
{
  "buy_now": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Place token bid

Attach the bid amount to the transaction.

```json
{
  "set_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "price": {
      "denom": "upasg",
      "amount": "900000"
    },
    "expires_at": 1773597600
  }
}
```

### Accept bid

```json
{
  "accept_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "bidder": "passage1bidder..."
  }
}
```

### Place collection bid

Attach `units * price.amount` to the transaction.

```json
{
  "set_collection_bid": {
    "collection": "passage1collection...",
    "units": 2,
    "price": {
      "denom": "upasg",
      "amount": "800000"
    },
    "expires_at": 1773597600
  }
}
```

### Query: collection denom

```json
{
  "collection_denom": {
    "collection": "passage1collection..."
  }
}
```

### Query: collection fee

```json
{
  "collection_fee": {
    "collection": "passage1collection..."
  }
}
```

### Query: preview sale

```json
{
  "preview_sale": {
    "collection": "passage1collection...",
    "price": "1000000"
  }
}
```

After `buy_now`, `accept_bid`, or `accept_collection_bid`, inspect the response attributes `pasg_utility_query`, `pasg_native_denom`, `pasg_settlement_denom`, `pasg_uses_native_utility`, and `pasg_fee_flow`.

## 8. `auction-english`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "denom": "upasg",
  "min_price": "100000",
  "trading_fee_bps": 250,
  "max_trading_fee_bps": 1000,
  "fee_collector": "passage1treasury...",
  "registry": "passage1registry...",
  "min_bid_increment_percent": "0.05",
  "min_duration": 3600,
  "max_duration": 604800,
  "extend_duration": 300,
  "require_registration": true
}
```

### Create auction

```json
{
  "create_auction": {
    "collection": "passage1collection...",
    "token_id": "1",
    "reserve_price": {
      "denom": "upasg",
      "amount": "1000000"
    },
    "duration": 86400,
    "seller_funds_recipient": null
  }
}
```

### Place bid

Attach the bid amount in the configured native denom.

```json
{
  "place_bid": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Settle auction

```json
{
  "settle_auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Query: config

```json
{
  "config": {}
}
```

### Query: auction

```json
{
  "auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

After `place_bid` or `settle_auction`, inspect the response attributes `pasg_utility_query`, `pasg_native_denom`, `pasg_settlement_denom`, `pasg_uses_native_utility`, and `pasg_fee_flow`.

## 9. `minter-v2`

### Instantiate

```json
{
  "base_token_uri": "ipfs://bafy.../metadata/",
  "num_tokens": 1000,
  "cw721_code_id": 301,
  "cw721_instantiate_msg": {
    "name": "Genesis Drop",
    "symbol": "GDROP",
    "minter": "passage1mintercontract...",
    "collection_info": {
      "creator": "passage1creator...",
      "description": "Primary drop for the ecosystem",
      "image": "ipfs://bafy.../cover.png",
      "external_link": "https://example.com/drop",
      "royalty_info": {
        "payment_address": "passage1royaltywallet...",
        "share": "0.05"
      }
    }
  },
  "start_time": "1773597600000000000",
  "per_address_limit": 3,
  "unit_price": {
    "denom": "upasg",
    "amount": "1000000"
  },
  "whitelist": null,
  "registry": "passage1registry..."
}
```

### Mint

Attach exactly the current price.

```json
{
  "mint": {}
}
```

### Batch mint

Attach `count * current_price.amount`.

```json
{
  "batch_mint": {
    "count": 2
  }
}
```

### Update config

```json
{
  "update_config": {
    "admin": null,
    "per_address_limit": 5,
    "unit_price": {
      "denom": "upasg",
      "amount": "1200000"
    },
    "whitelist": null,
    "registry": "passage1registry...",
    "paused": false
  }
}
```

### Query: config

```json
{
  "config": {}
}
```

### Query: mint price

```json
{
  "mint_price": {}
}
```

### Query: can mint

```json
{
  "can_mint": {
    "address": "passage1buyer..."
  }
}
```

After `mint`, `batch_mint`, `withdraw`, or `withdraw_to`, inspect the response attributes `pasg_utility_query`, `pasg_native_denom`, `pasg_settlement_denom`, `pasg_uses_native_utility`, and `pasg_fee_flow`.

## 10. Suggested CLI shape

Conceptual `wasmd` execute example:

```bash
wasmd tx wasm execute <contract_addr> '<json_msg>' --from <wallet> --amount 1000000upasg
```

Conceptual query example:

```bash
wasmd query wasm contract-state smart <contract_addr> '<json_query>'
```

If you are operating from backend or frontend code, you can reuse these exact payloads as the CosmWasm message body.

## 11. Chain-native PASG staking

Validator fee participation and PASG rewards are chain-native outcomes. Use the staking and distribution modules for delegation, undelegation, validator discovery, and reward-state reads.

### Conceptual delegate transaction

```bash
passage tx staking delegate <validator_operator_addr> 1000000upasg --from <delegator>
```

### Conceptual undelegation transaction

```bash
passage tx staking undelegate <validator_operator_addr> 250000upasg --from <delegator>
```

### Conceptual reward-state query

```bash
passage query distribution rewards <delegator> <validator_operator_addr>
```

### Conceptual undelegation-state query

```bash
passage query staking unbonding-delegation <delegator> <validator_operator_addr>
```

## 12. `pasg-governance`

### Propose a direct PASG utility change

```json
{
  "propose": {
    "title": "Tune PASG utility",
    "description": "Adjust utility conversion and session caps",
    "action": {
      "set_pasg_utility_config": {
        "points_per_pasg": "250",
        "max_fiat_report_age_secs": 900,
        "max_session_duration_secs": 7200
      }
    }
  }
}
```

### Propose a scoped admin handoff for `streaming-billing`

```json
{
  "propose": {
    "title": "Pause streaming billing",
    "description": "Ratify a scoped admin handoff for multisig follow-through",
    "action": {
      "stage_admin_action": {
        "action": {
          "streaming_billing_update_config": {
            "contract_addr": "passage1streaming...",
            "backend_operator": null,
            "fiat_oracle": null,
            "stripe_webhook_validator": null,
            "paused": true
          }
        }
      }
    }
  }
}
```

### Query the `ratified_admin_action`

```json
{
  "ratified_admin_action": {
    "proposal_id": 7
  }
}
```

Expected shape:

```json
{
  "action": {
    "proposal_id": 7,
    "admin_multisig": "passage1multisig...",
    "action": {
      "streaming_billing_update_config": {
        "contract_addr": "passage1streaming...",
        "backend_operator": null,
        "fiat_oracle": null,
        "stripe_webhook_validator": null,
        "paused": true
      }
    },
    "payload_hash": "<deterministic_hash>",
    "ratified_at": 1773597600
  }
}
```

### Mirror the ratified action through `multisig`

Human-readable multisig payload for the same `StreamingBillingUpdateConfig` intent:

```json
{
  "propose": {
    "title": "Mirror PASG ratified streaming change",
    "description": "payload_hash must match the governance ratification record",
    "msgs": [
      {
        "wasm": {
          "execute": {
            "contract_addr": "passage1streaming...",
            "msg": "<base64 of streaming-billing update_config>",
            "funds": []
          }
        }
      }
    ]
  }
}
```

Other scoped admin catalog entries follow the same model:

- `StreamingBillingUpdateConfig`
- `MarketplaceV3UpdateConfig`
- `AuctionEnglishUpdateConfig`
