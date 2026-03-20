# Hypothetical Flows

## Flow 1: ecosystem -> collection -> mint -> resale

1. An admin creates an ecosystem through `ecosystem-factory`.
2. `registry` records the ecosystem and membership.
3. The admin or an approved member uses `collection-factory` to deploy a `pg721` collection.
4. The collection is registered in `registry`.
5. An optional `minter-v2` handles the primary sale.
6. Later, `marketplace-v3` supports resale and applies global marketplace fee plus royalties.

## Flow 2: auction

1. The NFT owner approves `auction-english`.
2. They create a reserve auction.
3. Buyers place bids.
4. The winner settles the auction and the contract distributes seller proceeds, fee, and royalties.

## Flow 3: PASG governance

1. Holders deposit `upasg` into `pasg-governance`.
2. A proposal is created.
3. The proposal passes quorum and threshold.
4. If the action is administrative, `pasg-governance` publishes a ratified action.
5. `multisig` executes that action on the target contract.

## Flow 4: auxiliary local economy

1. A world operator uses `streaming-billing` to expose a points-based local economy.
2. That economy remains auxiliary to the main monetization path.
3. Initial sale and resale economics still live in NFT collections and commerce contracts.

## Short technical flow

- Query `streaming-billing` through `PasgUtility` for PASG semantics.
- Query `registry` for affiliation, mint permission, and trade permission.
- Execute marketplace or auction flows only against registered, tradeable collections.
