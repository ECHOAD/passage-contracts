# ecosystem-factory

Passage Ecosystem Factory - approval workflow for ecosystem creation

## Overview

This contract is part of the Passage CosmWasm workspace.

## Build

```bash
cargo build --package ecosystem-factory --release
```

## Generate Schema

```bash
cargo run --package ecosystem-factory --example schema
```

## Test

```bash
cargo test --package ecosystem-factory
```

## Notes

- Users submit ecosystem creation requests without attaching funds.
- Admins/operators approve requests and pay the gas for the instantiation transaction.
- Successful approvals instantiate the per-ecosystem `collection-factory` and register the ecosystem in `registry`.
- Request history is append-only: requests can be pending, rejected, cancelled, approved, or created.
