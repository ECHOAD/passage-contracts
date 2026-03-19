# 05-01 Summary

## Outcome

Wave 1 rebuilt the registry-side creator asset lifecycle around ecosystem membership and mutable collection affiliation.

## What Changed

- Removed collection creation request flow from `registry`.
- Kept ecosystem onboarding in `ecosystem-factory`, but collection onboarding now relies on ecosystem admin or approved member authorization.
- Added `DeregisterCollection` so collections can leave an ecosystem without losing their on-chain contract.
- Added `RehomeCollection` so unaffiliated collections can join a different ecosystem later.
- Preserved collection creator provenance across deregistration and re-home flows.
- Added `UnaffiliatedCollections` query support.

## Verification

- `cargo check -p registry -p collection-factory -p ecosystem-factory`
- `cargo test -p registry --lib`

## Notes

- Collection address remains the canonical collection identity.
- Ecosystem affiliation is now an administrative relationship in `registry`, not a permanent property of the collection contract.
