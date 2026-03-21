# Phase 5: Creator Asset Contracts & Monetization - Research

**Researched:** 2026-03-20
**Domain:** Passage typed NFT metadata boundary for metaverse assets and creator monetization
**Confidence:** MEDIUM-HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- `registry` is the canonical ledger for ecosystems and for collections that are officially affiliated with an ecosystem.
- Ecosystem creation resolves into canonical registration in `registry`; workflow contracts are not the final source of truth.
- Collection affiliation to an ecosystem is an administrative relationship recorded by `registry`, not an intrinsic property of the collection contract forever.
- An `ecosystem` is the parent context, for example a world or universe such as `Cyberpunk Universe`.
- Multiple collections can exist inside one ecosystem.
- A collection remains an independent on-chain contract even when registered to an ecosystem.
- If a collection is removed from an ecosystem, it must continue to exist on-chain with its history intact.
- A collection removed from an ecosystem becomes a loose collection with no current ecosystem affiliation.
- A loose collection may later be accepted by another ecosystem.
- When a collection is re-homed into another ecosystem, it keeps its original creator; only the ecosystem affiliation changes.
- Collection creation/registration should not rely on a per-collection request/approval workflow.
- The ecosystem admin can create or register collections directly.
- Approved ecosystem members/operators can also create or register collections directly.
- The correct control mechanism is ecosystem membership and membership revocation, not collection-by-collection approval.
- If access must be removed, the ecosystem admin revokes the member's access.
- The protocol should support administrative removal of a collection from an ecosystem registry entry without destroying the collection contract.
- Removal only detaches the collection from the ecosystem in `registry`.
- Another ecosystem may later adopt that collection into its own registry domain.
- Re-homing must not rewrite original creator ownership semantics.
- Collection contract address should be treated as the canonical collection identifier.
- Artificial collection IDs are unnecessary when the contract address already exists.
- Ecosystem string IDs should be reconsidered and minimized where a stable contract address already exists.
- If ecosystem identity can be represented canonically by an existing ecosystem-bound contract address, that address should be preferred over a synthetic ID; human-readable names/slugs should remain metadata.

### Claude's Discretion
- Whether Phase 5 fully removes `collection creation request` surfaces now or deprecates them first with migration notes.
- Whether ecosystem identity should become `collection_factory` address directly or another ecosystem-bound address already present in the model.
- Exact execute/query names for collection deregistration and ecosystem re-homing flows.
- Whether loose collections remain queryable through a dedicated `unaffiliated collections` surface or through a nullable ecosystem field.

### Deferred Ideas (OUT OF SCOPE)
- Whether ecosystem identities should be fully migrated from string IDs to contract-address keys in this phase, or whether that should be staged for a follow-up migration-focused phase.
- Whether ecosystem adoption of an already-existing loose collection should require an explicit collection-owner acknowledgement in a later phase.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| NFT-01 | Creator can create typed Passage NFT collections through factories with correct collection metadata, royalties, and registration behavior. | Supports keeping collection identity, `nft_type`, creator, and royalties on-chain while pushing heavy manifests off-chain. |
| NFT-02 | Required world, plugin, achievement, avatar, companion, and component extensions are implemented or explicitly retired in docs and code with no silent drift. | Identifies which current extension fields are durable, which drift into runtime concerns, and which should be corrected or retired. |
| NFT-03 | Cross-world usage and monetization rules for Passage assets are enforced through on-chain primitives plus documented off-chain coordination. | Defines the boundary between on-chain rights/monetization and off-chain Unreal or Pixel Streaming consumption. |
| REV-01 | Platform services can execute creator/platform/partner revenue splits for world, marketplace, and asset flows from on-chain rules. | Confirms royalties and world revenue routing belong on-chain; runtime/rendering details do not. |
</phase_requirements>

## Summary

The current Passage model is directionally correct at the collection boundary. `pg721` and `pg721-updatable` keep the collection contract as the canonical collection identity, store collection-level creator and royalty information on-chain, enforce a collection-wide `nft_type`, and explicitly state that runtime and Unreal rendering stay off-chain. That aligns with `ARCH-01`, `NFT-03`, and the repo README guidance.

The weakness is at the token-type field boundary. Several fields in the current typed extensions look like protocol semantics but are really interoperability vocabulary or mutable gameplay/runtime state. The most important issue is structural: `pg721` token extensions are validated at mint time, but neither `pg721` nor `pg721-updatable` exposes any execute surface to update extension fields later. `pg721-updatable` only updates `token_uri`. That means fields such as companion `level` and `experience` are not actually modeled as mutable on-chain state in the current contracts; they are frozen mint-time metadata unless a separate corrective contract pattern is introduced.

**Primary recommendation:** Keep typed Passage NFTs, but tighten the boundary: on-chain should hold canonical asset class, ownership, transferability, creator/royalty/revenue rights, and only a minimal interoperability profile; content-addressed manifests, rendering payloads, compatibility details, and mutable runtime state should live off-chain unless Passage adds dedicated protocol logic to mutate and enforce them on-chain.

## Standard Stack

### Core
| Library / Standard | Version | Purpose | Why Standard |
|--------------------|---------|---------|--------------|
| `cosmwasm-std` | `2.1.3` | Contract runtime | Workspace-pinned protocol runtime in this repo. |
| `cw721` / `cw721-base` | `0.18` | NFT ownership, approvals, metadata URI surface | Standard ownership primitive; Passage extends it rather than replacing it. |
| `pg721` | workspace crate | Typed collection contract | Adds Passage `nft_type` and compact typed extension validation. |
| `pg721-updatable` | workspace crate | Same typed model with editable `token_uri` | Allows manifest pointer changes without changing ownership semantics. |
| `registry` | workspace crate | Canonical ecosystem/collection affiliation | Matches Phase 5 context and current repo architecture. |

### Supporting
| Library / Doc | Version | Purpose | When to Use |
|---------------|---------|---------|-------------|
| `cw-multi-test` | `2.1.1` | Contract integration testing | Standard repo test harness for metadata and authorization behavior. |
| ERC-721 metadata extension | current standard | Token metadata URI pattern | Use as the canonical URI points to JSON baseline. |
| IPFS CID addressing | current docs | Content-addressed off-chain manifests and assets | Use for mutable or gateway-resolved manifests where IPFS fits product ops. |
| Unreal Pixel Streaming | UE 5.x docs | Remote rendering over WebRTC | Use as the architectural reminder that rendering and streaming are service/runtime concerns, not NFT concerns. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Compact typed extensions + off-chain manifest | Fully rich on-chain per-type structs | Easier direct queries, but overfits runtime details and becomes expensive and drift-prone. |
| Single `Component`/`Avatar` taxonomy with off-chain profiles | Many new first-class on-chain NFT enums | More explicit per type, but increases migration cost and bakes product vocabulary too early. |
| `pg721-updatable` token URI updates | Direct extension mutation in CW721 metadata | Only makes sense if Passage truly wants mutable protocol state in the NFT contract, which it does not today. |

**Version verification:** Workspace versions verified from `Cargo.toml` on 2026-03-20. External standards are treated as reference patterns, not repo dependencies.

## Architecture Patterns

### Recommended Project Structure
```text
contracts/
+-- nft/
¦   +-- pg721/                  # immutable typed collections
¦   +-- pg721-updatable/        # token_uri-updatable typed collections
¦   +-- pg721-metadata-onchain/ # legacy/full-metadata variant; not the default metaverse path
¦   +-- minter-v2/              # off-chain-manifest mint flow
¦   +-- minter-v2-metadata-onchain/
+-- core/
¦   +-- registry/               # canonical ecosystem + collection affiliation
¦   +-- split-router/           # generic revenue execution
+-- docs/context/
    +-- product/architecture/   # asset semantics and boundary docs
```

### Pattern 1: Collection-level type, token-level compact semantics
**What:** Keep the collection contract typed with `nft_type`; keep token extension fields compact and protocol-facing.
**When to use:** For every Passage asset collection.
**Why:** The repo already enforces this cleanly in `pg721` and `pg721-updatable`.
**Current evidence:** `contracts/nft/pg721/src/msg.rs` and `contracts/nft/pg721/src/contract.rs` validate token metadata against collection `nft_type`.

### Pattern 2: `token_uri` as the manifest pointer
**What:** Use `token_uri` as the canonical pointer to an off-chain JSON manifest; do not duplicate large metadata blobs inside typed extension fields.
**When to use:** Default for worlds, avatars, components, plugins, scenes, emotes, and similar metaverse assets.
**Why:** It matches ERC-721 metadata expectations and Passage already uses `base_token_uri` in `minter-v2`.

### Pattern 3: Keep monetization semantics on-chain, keep rendering off-chain
**What:** Rights, royalties, transferability, and revenue split semantics stay on-chain; Unreal asset bundles, composition state, session state, and stream transport stay off-chain.
**When to use:** Always.
**Why:** This is explicitly required by `.planning/REQUIREMENTS.md`, Phase 5 context, and the `pg721` READMEs.

### Pattern 4: Mutable runtime state requires its own contract surface
**What:** If Passage wants mutable gameplay state on-chain, model it as dedicated state plus explicit execute/query flows, not as frozen NFT extension fields.
**When to use:** Companion progression, plugin license counters, access-pass redemption state, install counts, expiration, and similar mutable facts.
**Why:** Current `pg721` contracts do not support extension mutation. `pg721-updatable` only updates `token_uri`.

### Anti-Patterns to Avoid
- **Mint-time runtime snapshots:** Do not mint fields like `current_instances`, `equipped_components`, or live avatar state into the NFT extension unless the contract can mutate and validate them afterward.
- **Engine-specific contract vocabulary:** Do not put Unreal bones, Pixel Streaming session data, renderer config, or transport URLs on-chain.
- **Duplicate manifest pointers:** Do not store both rich per-type `metadata_uri` fields and a separate `token_uri` unless they have different enforceable semantics.
- **Rights hidden only in off-chain JSON:** Royalties, soulbound flags, transfer restrictions, and revenue splits should not be off-chain conventions only.

## Recommended Metadata Boundary

### Keep on-chain
- Canonical collection identity: collection contract address.
- Canonical asset class: collection `nft_type` and matching token extension discriminant.
- Creator provenance and collection-level royalty info.
- Transfer restrictions that affect protocol behavior, such as soulbound or non-transferable semantics.
- Revenue execution semantics used by protocol contracts or services, such as world `revenue_shares`.
- Minimal compatibility profile only when other protocol actors must query it deterministically.
  Example: a stable `component_type`, or a manifest/profile hash if Passage later standardizes it.

### Keep off-chain in content-addressed manifests
- 3D models, textures, audio, animation assets, world bundles, plugin binaries, thumbnails, and preview media.
- Runtime composition and rendering state: avatar equipment, scene assembly, particle systems, Unreal blueprints, and Pixel Streaming session details.
- Detailed compatibility matrices: skeleton definitions, slot schemas, attachment bones, supported engine versions, and render pipeline hints.
- Descriptive metadata and UX attributes: names, long descriptions, rich attributes, rarity display, screenshots, and marketplace presentation fields.
- Large mutable data: install counters, progression logs, live ability unlocks, and inventory snapshots.

### Optional on-chain references
- Manifest URI via `token_uri`.
- Manifest or compatibility profile hash if Passage needs tamper detection independent of the URI.
- Schema version string only if on-chain consumers need to branch on it; otherwise keep versioning in the manifest.

### Do not put on-chain
- Unreal-specific class names, bones, attachment sockets, LOD tables, Pixel Streaming transport/session config, WebRTC endpoints, or backend database identifiers as canonical protocol fields.
- Mutable gameplay counters in `pg721` metadata unless a dedicated execute/query surface exists.
- Marketplace- or renderer-only tags that no contract enforces.

## NFT Type Modeling Guidance

### Overall assessment of current Passage model
| Area | Assessment | Reason |
|------|------------|--------|
| Collection `nft_type` | Good | Durable protocol classification; enforced in contract. |
| Collection creator + royalties | Good | Durable monetization and provenance semantics. |
| World `revenue_shares` | Good | Clear protocol effect; supports revenue routing. |
| Token `token_uri` usage | Good | Natural boundary for IPFS/Arweave manifests. |
| Avatar `equipment_state_uri` / `equipment_hash` | Mostly good | References off-chain state instead of storing composition directly; still should remain optional and not imply on-chain runtime authority. |
| Companion `level` / `experience` in current generic NFT metadata | Overfit / structurally wrong in current contracts | Modeled as mutable state, but current contracts cannot mutate extension fields. |
| Component compatibility fields on-chain | Borderline | Useful for quick filtering, but vocabulary drift risk is high without schema/profile versioning. |
| Plugin runtime permissions and install counts as NFT metadata | Wrong place | These are license or deployment state, not stable identity metadata. |
| World template branding/customization as URIs | Better than full on-chain rules | Keeps brand/runtime details off-chain. |
| `pg721-metadata-onchain` as default metaverse path | Not recommended | Rich on-chain metadata duplicates standard JSON fields and is a poor fit for large or evolving metaverse assets. |

### Worlds
**Keep:** `world_id` if it is a stable Passage protocol identifier, `revenue_shares`, optional non-transferable flag if needed.
**Off-chain:** world package manifests, scene graph, max players, plugins installed, spawn points, environment settings, lighting, streaming configs.
**Current Passage:** `WorldExtension { world_id, revenue_shares }` is appropriately compact.

### Wearables / Components
**Keep:** a stable asset ID, a coarse asset subtype if contracts or indexers need it, license/rights if enforced.
**Off-chain:** detailed compatibility, attachment rules, skeleton mappings, sockets, animation retargeting info, render bundle references.
**Current Passage:** `compatible_skeletons` and `compatible_slots` are only acceptable if treated as a small interoperability profile. They should not become engine-specific taxonomies. A profile hash or manifest-based compatibility section is safer.

### Emotes
**Recommendation:** Do not overload current `Component` semantics unless emotes are just another attachable asset in Passage UX. If emotes are animation licenses with different marketplace or entitlement rules, add a first-class `Emote` type in a later corrective phase. Otherwise model as `Component` with off-chain manifest category.

### Avatars / Characters
**Keep:** stable avatar identity, optional base compatibility profile, optional equipment-state reference hash only if verification matters.
**Off-chain:** actual equipped items, cosmetic customization, animation controllers, rig details, Unreal assembly state.
**Current Passage:** `skeleton_type`, `slot_schema_uri`, `equipment_state_uri`, and `equipment_hash` are closer to the correct boundary than the original design docs. The remaining risk is that `skeleton_type` becomes an engine taxonomy frozen on-chain.

### Companions
**Keep now:** stable companion identity only.
**Keep on-chain only if Passage adds dedicated state transitions:** `level` and `experience` in a separate progression-capable contract surface.
**Off-chain:** abilities, visual evolution, stats formulas, behavior trees, current AI/runtime state.
**Current Passage:** the docs approve on-chain progression, but the actual `pg721` surface does not support it. This is the clearest mismatch between design intent and contract capability.

### Scenes
**Recommendation:** Treat scenes as off-chain manifests referenced by a world or template NFT unless a scene is independently ownable, tradeable, and monetized. If so, add a first-class `Scene` type rather than forcing it into `WorldTemplate`.

### Access passes
**Recommendation:** Access passes should not be just metadata. If Passage needs redemption, expiration, usage limits, or gating, use dedicated contract state or a dedicated pass contract. Metadata can describe the pass, but entitlement counters and expiry must be enforceable state.

### Plugins
**Keep:** stable plugin/license identity, optional license class, optional rights URI.
**Off-chain:** plugin binaries, dependency graph, required runtime permissions, installation docs, engine compatibility, API surface.
**Do not keep as plain NFT metadata:** `current_instances`, live install counts, or world deployment state.

### Monetization-bearing assets
**Keep:** collection royalties, explicit world or asset revenue shares when protocol routers/services need them, transferability restrictions, and provenance.
**Off-chain:** payout presentation, storefront copy, promotion metadata, marketplace ranking signals.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| NFT identity and ownership | Custom ownership ledger | `cw721` / `cw721-base` with Passage typed wrappers | Already standard and integrated. |
| Rich asset metadata transport | Custom on-chain blobs | `token_uri` to IPFS/Arweave JSON manifest | Standard, cheaper, and compatible with off-chain consumers. |
| Runtime rendering authority | NFT extension fields for Unreal state | Off-chain Unreal/Pixel Streaming services resolving manifests | Rendering is explicitly off-chain in repo requirements. |
| Mutable gameplay state in static metadata | Ad hoc extension fields like `level` without mutation APIs | Dedicated stateful contract or separate progression module | Prevents silent drift between metadata and actual state. |
| Engine compatibility taxonomy in contract | Hardcoded Unreal slot/bone/socket vocabulary | Manifest/profile schema with versioning | Avoids protocol lock-in to one runtime model. |

**Key insight:** The NFT should prove identity, rights, and protocol-relevant semantics. It should not become the scene graph, runtime database, or renderer contract.

## Common Pitfalls

### Pitfall 1: Treating off-chain runtime data as NFT truth
**What goes wrong:** Equipment loadouts, plugin installs, or render bundles get treated as if they were immutable protocol facts.
**Why it happens:** The metadata structs look like they can hold anything.
**How to avoid:** Ask whether a contract or service must enforce the field. If not, keep it in the off-chain manifest.
**Warning signs:** Fields mention bones, sockets, WebRTC, session state, install counts, or dynamic inventories.

### Pitfall 2: Modeling mutable gameplay state in immutable extension fields
**What goes wrong:** `level`, `experience`, or other counters are minted once and never updated, or drift into off-chain truth.
**Why it happens:** Current `pg721` validation is strong, but mutation support is absent.
**How to avoid:** Only keep mutable facts on-chain if there is a dedicated execute/query path and tests for state transitions.
**Warning signs:** The type contains counters but the contract only exposes `Mint` and `UpdateTokenMetadata { token_uri }`.

### Pitfall 3: Freezing engine vocabulary too early
**What goes wrong:** On-chain slot/skeleton fields become Unreal-specific and block future runtimes or cross-world conventions.
**Why it happens:** Unreal is the current product runtime.
**How to avoid:** Keep compatibility in versioned manifests; only keep a minimal profile ID or hash on-chain if needed.
**Warning signs:** Field values look like renderer internals rather than protocol classes.

### Pitfall 4: Putting rights only in manifests
**What goes wrong:** Royalties, soulbound behavior, or monetization splits become advisory instead of enforceable.
**Why it happens:** The off-chain manifest is easier to evolve.
**How to avoid:** Keep any field with economic or transfer effect on-chain.
**Warning signs:** A field changes sale behavior but is only described in JSON.

### Pitfall 5: Assuming Pixel Streaming changes NFT modeling
**What goes wrong:** The NFT starts to carry transport or rendering assumptions because the current product uses Pixel Streaming.
**Why it happens:** The streamed experience is user-visible, so it feels like part of the token.
**How to avoid:** Treat Pixel Streaming as a consumer of manifests and ownership proofs, not as part of token semantics.
**Warning signs:** Metadata mentions streams, peers, sessions, codecs, or browser delivery.

## Corrective Actions / Phase Recommendation

### Recommendation
**Yes: a focused corrective phase is justified.**

This should not be a Phase 5 rewrite. It should be a narrow corrective phase for NFT metadata boundary hardening, because the collection-level model is good but the token-level semantics still mix three different categories:
1. durable protocol semantics,
2. interoperability hints, and
3. mutable runtime/gameplay state.

### Minimum corrective scope
- Reclassify each typed extension field as `keep on-chain`, `move to off-chain manifest`, or `requires dedicated stateful contract`.
- Remove or deprecate mutable-state fields that the current `pg721` family cannot mutate safely.
  Highest-priority candidate: companion `level` and `experience` in generic token metadata.
- Define a canonical Passage manifest contract for `token_uri`.
  Include schema version, content-addressed asset references, compatibility profiles, and optional hash fields.
- Tighten avatar/component compatibility modeling.
  Prefer manifest-based compatibility definitions with optional profile IDs or hashes on-chain instead of rich engine vocabulary on-chain.
- Explicitly position `pg721-metadata-onchain` as a non-default path for metaverse assets, or constrain its use to small immutable assets.
- Decide which future asset classes deserve first-class on-chain types versus manifest categories.
  Likely candidates for explicit evaluation: `Emote`, `Scene`, `AccessPass`.

### What does not need a corrective rewrite
- Collection contract per collection.
- Collection-level `nft_type`.
- Collection creator and royalties.
- World `revenue_shares`.
- Registry as the canonical affiliation layer.
- Off-chain rendering/runtime boundary.

### Severity by current field
| Field / Pattern | Recommendation |
|-----------------|----------------|
| `collection_info.creator`, `royalty_info`, collection `nft_type` | Keep as-is |
| `WorldExtension.revenue_shares` | Keep as-is |
| `AvatarExtension.equipment_state_uri`, `equipment_hash` | Keep, but treat as verification/reference only |
| `AvatarExtension.skeleton_type` | Keep only if Passage standardizes a runtime-agnostic compatibility profile; otherwise move to manifest |
| `ComponentExtension.compatible_skeletons`, `compatible_slots` | Move toward manifest/profile-based compatibility; avoid expanding on-chain taxonomy |
| `CompanionExtension.level`, `experience` | Remove from generic NFT metadata unless a dedicated progression-capable contract is added |
| `PluginExtension.permissions_uri` | Keep as an external policy/doc pointer if useful; do not expand on-chain runtime permissions |
| `pg721-metadata-onchain::Metadata` rich OpenSea-style fields | Do not use as default metaverse asset model |

## Code Examples

Verified and recommended patterns:

### Current good pattern: collection-level type plus compact token extension
```rust
pub struct TokenMetadata {
    pub nft_type: NftType,
    pub extension: Option<NftTypeExtension>,
}
```
Source: `contracts/nft/pg721/src/msg.rs`

### Current critical limitation: only `token_uri` is mutable in `pg721-updatable`
```rust
pub enum ExecuteMsg {
    FreezeTokenMetadata {},
    UpdateTokenMetadata {
        token_id: String,
        token_uri: Option<String>,
    },
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Extension,
    },
}
```
Source: `contracts/nft/pg721-updatable/src/msg.rs`

### Recommended Passage boundary pattern
```rust
pub struct CompanionExtension {
    pub companion_id: String,
    // No mutable progression counters here in generic pg721.
    // token_uri points to the content-addressed manifest.
}

pub struct WorldExtension {
    pub world_id: String,
    pub revenue_shares: Vec<RevenueShare>,
}
```

### Recommended off-chain manifest shape
```json
{
  "schema_version": "passage.asset-manifest/v1",
  "asset_class": "avatar",
  "content": {
    "base_model": "ar://...",
    "preview_image": "ipfs://..."
  },
  "compatibility": {
    "profile_id": "humanoid-v2",
    "slot_schema": "ipfs://..."
  },
  "runtime": {
    "unreal": {
      "supported": true
    }
  }
}
```
The manifest can include runtime-specific subsections because it is off-chain and versioned; the NFT contract should not enforce them directly.

## State of the Art

| Old / Risky Approach | Current Recommended Approach | Impact |
|----------------------|------------------------------|--------|
| Rich on-chain per-type metadata | Compact on-chain semantics + content-addressed manifests | Better boundary discipline and lower drift risk. |
| Putting mutable runtime state into NFT extension | Separate stateful contracts or off-chain runtime state | Prevents frozen metadata from pretending to be live state. |
| Runtime-specific contract fields | Runtime-agnostic on-chain identity plus versioned manifests | Keeps Passage portable beyond Unreal/Pixel Streaming. |
| Full on-chain metadata blobs for metaverse assets | URI-pointer model with optional hash verification | Better fit for IPFS/Arweave asset bundles. |

**Deprecated or not recommended as default:**
- `pg721-metadata-onchain` as the default collection model for complex metaverse assets.
- Mutable gameplay semantics embedded in generic CW721 metadata without explicit state transitions.

## Open Questions

1. **Should Passage standardize a first-class compatibility profile registry?**
- What we know: current `compatible_skeletons`, `compatible_slots`, and `skeleton_type` are useful but drift-prone.
- What's unclear: whether the protocol truly needs on-chain filtering by compatibility, or whether indexers/manifests are enough.
- Recommendation: start with manifest schema + optional profile ID/hash; only add registry-level compatibility primitives if a real enforcement need appears.

2. **Are `Emote`, `Scene`, and `AccessPass` protocol-distinct enough for first-class `NftType` variants?**
- What we know: they are important product classes, but current contracts only expose seven types.
- What's unclear: whether their monetization or transfer semantics differ enough from existing types.
- Recommendation: decide this in the corrective phase before adding new enums.

3. **Does Passage want on-chain progression at all, or only verifiable off-chain progression?**
- What we know: docs previously approved companion progression on-chain, but current contracts do not implement that capability.
- What's unclear: whether the product wants the gas and contract complexity of real progression logic.
- Recommendation: do not leave progression half-modeled in metadata; either implement a dedicated progression surface or move it off-chain.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `cw-multi-test 2.1.1` |
| Config file | `.cargo/config.toml` |
| Quick run command | `cargo test -p pg721 -p pg721-updatable -p pg721-metadata-onchain --lib` |
| Full suite command | `cargo unit-test` |

### Phase Requirements ? Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| NFT-01 | Typed collection metadata and royalty behavior remain correct | unit/integration | `cargo test -p pg721 -p pg721-updatable --lib` | Yes |
| NFT-02 | Typed extensions match intended boundary and no silent drift remains | unit/doc-smoke | `cargo test -p pg721 -p pg721-updatable -p pg721-metadata-onchain --lib` | Partial |
| NFT-03 | Cross-world use stays query-based and off-chain coordinated | doc/manual + targeted unit | `rg -n "runtime|off-chain|Unreal|render" contracts/nft/pg721/README.md contracts/nft/pg721-updatable/README.md .planning/REQUIREMENTS.md` | Partial |
| REV-01 | World or collection monetization semantics remain queryable | unit/integration | `cargo test -p pg721 -p minter-v2 --lib` | Yes |

### Sampling Rate
- **Per task commit:** `cargo test -p pg721 -p pg721-updatable --lib`
- **Per wave merge:** `cargo unit-test`
- **Phase gate:** `cargo unit-test` and `cargo check --workspace` before verification

### Wave 0 Gaps
- [ ] Add explicit regression tests that prove `pg721-updatable` mutates `token_uri` only, not typed extensions.
- [ ] Add tests or documentation assertions for any field retained as protocol-level semantics after corrective pruning.
- [ ] If a corrective phase adds mutable progression or entitlement state, create dedicated contract tests for those state transitions before implementation.

## Sources

### Primary (HIGH confidence)
- Repo code: `contracts/nft/pg721/src/msg.rs`, `contracts/nft/pg721/src/contract.rs`, `contracts/nft/pg721-updatable/src/msg.rs`, `contracts/nft/pg721-updatable/src/contract.rs`, `contracts/nft/pg721-metadata-onchain/src/msg.rs`, `contracts/nft/minter-v2/src/msg.rs`, `contracts/nft/minter-v2-metadata-onchain/src/msg.rs`
- Repo docs: `.planning/REQUIREMENTS.md`, `.planning/phases/05-creator-asset-contracts-monetization/05-CONTEXT.md`, `contracts/nft/pg721/README.md`, `contracts/nft/pg721-updatable/README.md`
- Product architecture docs: `../context/product/architecture/NFT_COLLECTION_STRATEGY.md`, `../context/product/architecture/NFT_METADATA_DESIGN.md`, `../context/product/architecture/NFT_DESIGN_CORRECTIONS.md`, `../context/product/architecture/IMPLEMENTATION_REVIEW.md`

### Secondary (MEDIUM confidence)
- ERC-721 metadata URI pattern: https://eips.ethereum.org/EIPS/eip-721
- IPFS canonical `ipfs://{CID}` addressing and gateway resolution: https://docs.ipfs.tech/concepts/ipfs-gateway/
- Epic Unreal Engine Pixel Streaming docs: https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/PixelStreaming

### Tertiary (LOW confidence)
- Arweave permanence was used as a product-architecture assumption from repo docs; I did not find a single concise official Arweave doc page during this pass that stated the same boundary as cleanly as the repo architecture docs.

## Confidence

### Confidence breakdown
- **Standard stack:** HIGH. Grounded in repo `Cargo.toml`, contract code, and existing READMEs.
- **Architecture patterns:** MEDIUM-HIGH. Grounded in repo requirements, architecture docs, and current contract behavior.
- **Metadata boundary:** MEDIUM-HIGH. Strongly supported by repo boundary docs and current code; some future asset-class recommendations are prescriptive design guidance.
- **Pitfalls:** MEDIUM-HIGH. The key pitfalls are directly evidenced by current code, especially immutable extension fields.

**What might I have missed?**
- A separate in-repo contract that already implements mutable progression or entitlement state for typed NFTs. I did not find one in the required Phase 5 inputs.
- A missing shared NFT module: the requested input `contracts/nft/shared/src/lib.rs` does not exist in the current workspace, so this research did not rely on it.

**Research date:** 2026-03-20
**Valid until:** 2026-04-19
