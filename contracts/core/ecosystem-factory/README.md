# ecosystem-factory

`ecosystem-factory` is the governed onboarding flow for new ecosystems.

## Role

- Accept ecosystem creation requests.
- Let protocol admin or operators approve or reject them.
- Instantiate the dedicated `collection-factory` for the approved ecosystem.
- Register the final ecosystem in `registry` through `RegisterEcosystemFromFactory`.

## Model

- `registry` is the canonical ledger for ecosystems.
- `ecosystem-factory` is the onboarding workflow, not the long-term source of truth.
- Each approved ecosystem gets its own `collection-factory`, which then manages direct collection creation for that ecosystem team.

## Main Execute Messages

- `UpdateConfig`
- `SubmitEcosystemCreationRequest`
- `ResolveEcosystemCreationRequest`

## Notes

- Ecosystem approval happens here.
- Collection approval does not. Once the ecosystem exists, collection creation is handled by ecosystem admin and approved members through the dedicated `collection-factory`.
