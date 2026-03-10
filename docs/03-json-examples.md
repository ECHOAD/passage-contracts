# JSON Examples

Este archivo contiene ejemplos de payloads JSON para los contratos principales del flujo comercial de Passage.

## Convenciones

- Usa placeholders como `<registry_addr>` o `<collection_addr>` y reemplazalos por direcciones reales.
- Los `Decimal` normalmente van como string, por ejemplo `"0.05"`.
- Los `Timestamp` en estos contratos conviene tratarlos como string en nanosegundos.
- Cuando un mensaje requiere fondos, el monto no va dentro del JSON del mensaje; se adjunta en la transaccion.
- Los enums serializados con `cw_serde` usan `snake_case`.
- `RevenueEventType` es una excepcion en el estado actual y usa nombres tipo `PrimarySale` o `SecondaryRoyalty`.

## 1. `registry`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "operators": ["passage1ops1...", "passage1ops2..."],
  "ecosystem_factory": "passage1ecosystemfactory..."
}
```

### Aprobar creador de ecosystem

```json
{
  "approve_ecosystem_creator": {
    "creator": "passage1creator..."
  }
}
```

### Registrar ecosystem directo

```json
{
  "register_ecosystem": {
    "id": "music",
    "name": "Music Ecosystem",
    "ecosystem_type": "public",
    "collection_creation_policy": "approval_required",
    "collection_factory": "passage1collectionfactory...",
    "detail": "Ecosistema para colecciones musicales",
    "image_urls": ["https://cdn.example.com/music-cover.png"],
    "animation_url": null,
    "url": "https://example.com/music"
  }
}
```

### Aprobar miembro del ecosystem

```json
{
  "approve_ecosystem_member": {
    "ecosystem_id": "music",
    "member": "passage1teammember..."
  }
}
```

### Registrar coleccion ya desplegada

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

### Guardar runtime pointers de la coleccion

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

### Autorizar minter

```json
{
  "authorize_minter": {
    "collection_address": "passage1collection...",
    "minter_address": "passage1minter..."
  }
}
```

### Query: ver coleccion

```json
{
  "collection": {
    "address": "passage1collection..."
  }
}
```

### Query: validar minter

```json
{
  "is_minter_authorized": {
    "collection_address": "passage1collection...",
    "minter_address": "passage1minter..."
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
    "detail": "Ecosistema para artistas y colecciones musicales",
    "image_urls": ["https://cdn.example.com/music.png"],
    "animation_url": null,
    "url": "https://example.com/music"
  }
}
```

### Aprobar request

```json
{
  "resolve_ecosystem_creation_request": {
    "request_id": 1,
    "approved": true,
    "note": "Aprobado por governance"
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

### Aprobar creador local

```json
{
  "approve_creator": {
    "creator": "passage1creator..."
  }
}
```

### Crear coleccion

```json
{
  "create_collection": {
    "name": "Genesis Music Collection",
    "symbol": "GMC",
    "minter": "passage1creator...",
    "collection_info": {
      "description": "Coleccion genesis del ecosystem musical",
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

### Query: colecciones del creator

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

Normalmente `pg721` lo despliega `collection-factory` o `minter-v2`, pero este es el shape del instantiate.

### Instantiate directo

```json
{
  "name": "Genesis Music Collection",
  "symbol": "GMC",
  "minter": "passage1creator...",
  "collection_info": {
    "creator": "passage1creator...",
    "description": "Coleccion genesis del ecosystem musical",
    "image": "ipfs://bafy.../cover.png",
    "external_link": "https://example.com/gmc",
    "royalty_info": {
      "payment_address": "passage1royaltywallet...",
      "share": "0.05"
    }
  }
}
```

### Aprobar marketplace o auction para un token

```json
{
  "approve": {
    "spender": "passage1marketplace_or_auction...",
    "token_id": "1",
    "expires": null
  }
}
```

### Aprobar operator global

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

## 5. `split-router`

### Instantiate

```json
{
  "admin": "passage1admin...",
  "registry": "passage1registry..."
}
```

### Crear regla de distribucion

```json
{
  "set_distribution_rule": {
    "collection": "passage1collection...",
    "creator": "passage1creator...",
    "creator_share": "0.85",
    "collaborators": [
      {
        "address": "passage1artistmanager...",
        "share": "0.10",
        "name": "Manager"
      },
      {
        "address": "passage1producer...",
        "share": "0.05",
        "name": "Producer"
      }
    ]
  }
}
```

### Actualizar regla

```json
{
  "update_distribution_rule": {
    "collection": "passage1collection...",
    "creator": null,
    "creator_share": "0.80",
    "collaborators": [
      {
        "address": "passage1artistmanager...",
        "share": "0.10",
        "name": "Manager"
      },
      {
        "address": "passage1producer...",
        "share": "0.05",
        "name": "Producer"
      },
      {
        "address": "passage1designer...",
        "share": "0.05",
        "name": "Designer"
      }
    ],
    "active": true
  }
}
```

### Route primary sale

Normalmente este mensaje lo llama `minter-v2` con fondos adjuntos.

```json
{
  "route_primary_sale": {
    "collection": "passage1collection..."
  }
}
```

### Route secondary royalty

Normalmente este mensaje lo llama `marketplace-v3` con el royalty adjunto.

```json
{
  "route_secondary_royalty": {
    "collection": "passage1collection..."
  }
}
```

### Query: preview distribution

```json
{
  "preview_distribution": {
    "collection": "passage1collection...",
    "amount": "1000000",
    "event_type": "SecondaryRoyalty"
  }
}
```

## 6. `marketplace-v3`

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
  "split_router": "passage1splitrouter...",
  "use_split_router": true,
  "operators": ["passage1operator..."],
  "require_registration": true
}
```

### Registrar coleccion

```json
{
  "register_collection": {
    "collection": "passage1collection...",
    "trading_fee_bps": null,
    "denom": null
  }
}
```

### Publicar venta fija

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

### Comprar venta fija

Adjunta exactamente `1000000upasg` en la transaccion.

```json
{
  "buy_now": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Poner bid por token

Adjunta el bid en la transaccion.

```json
{
  "set_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "price": {
      "denom": "upasg",
      "amount": "900000"
    },
    "expires_at": 1735689600
  }
}
```

### Aceptar bid

```json
{
  "accept_bid": {
    "collection": "passage1collection...",
    "token_id": "1",
    "bidder": "passage1bidder..."
  }
}
```

### Poner collection bid

Adjunta `units * price.amount` en la transaccion.

```json
{
  "set_collection_bid": {
    "collection": "passage1collection...",
    "units": 3,
    "price": {
      "denom": "upasg",
      "amount": "800000"
    },
    "expires_at": 1735689600
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

## 7. `auction-english`

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
  "split_router": "passage1splitrouter...",
  "use_split_router": true,
  "min_bid_increment_percent": "0.05",
  "min_duration": 3600,
  "max_duration": 604800,
  "extend_duration": 300,
  "require_registration": true
}
```

### Crear subasta

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

### Actualizar reserve price

```json
{
  "update_reserve_price": {
    "collection": "passage1collection...",
    "token_id": "1",
    "reserve_price": {
      "denom": "upasg",
      "amount": "1200000"
    }
  }
}
```

### Poner bid

Adjunta el monto del bid en la transaccion.

```json
{
  "place_bid": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Liquidar subasta

```json
{
  "settle_auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

### Query: ver subasta

```json
{
  "auction": {
    "collection": "passage1collection...",
    "token_id": "1"
  }
}
```

## 8. `minter-v2`

### Instantiate

```json
{
  "base_token_uri": "ipfs://bafy.../metadata",
  "num_tokens": 1000,
  "cw721_code_id": 301,
  "cw721_instantiate_msg": {
    "name": "Genesis Drop",
    "symbol": "GDROP",
    "minter": "passage1mintercontract...",
    "collection_info": {
      "creator": "passage1creator...",
      "description": "Drop primario del ecosystem",
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
  "registry": "passage1registry...",
  "split_router": "passage1splitrouter...",
  "use_split_router": true,
  "metadata_mode": "off_chain",
  "native_asset_template": []
}
```

### Mint

Adjunta exactamente el `unit_price`.

```json
{
  "mint": {}
}
```

### Batch mint

Adjunta `count * unit_price.amount`.

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
    "split_router": "passage1splitrouter...",
    "use_split_router": true,
    "metadata_mode": "off_chain",
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

### Query: can mint

```json
{
  "can_mint": {
    "address": "passage1buyer..."
  }
}
```

## 9. CLI shape sugerido

Ejemplo conceptual de ejecucion con `wasmd`:

```bash
wasmd tx wasm execute <contract_addr> '<json_msg>' --from <wallet> --amount 1000000upasg
```

Ejemplo conceptual de query:

```bash
wasmd query wasm contract-state smart <contract_addr> '<json_query>'
```

Si vas a operar desde backend o frontend, reutiliza estos mismos payloads como body del mensaje CosmWasm.
