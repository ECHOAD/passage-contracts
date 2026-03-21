# world-plugin-assignment

`world-plugin-assignment` models durable plugin-to-world usage rights as an explicit on-chain relationship.

It keeps plugin NFT ownership separate from world usage rights:

- plugin ownership stays in the plugin NFT collection
- world authority stays in the world NFT collection
- durable assignment rights are persisted in this contract and remain queryable after later NFT resale

Technical plugin deployment, binaries, runtime permissions, and installation mechanics remain off-chain.
