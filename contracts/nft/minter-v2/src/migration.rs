//! Migration module for minter v1 to minter-v2
//!
//! This module handles the state migration from the legacy minter
//! to minter-v2 with Split Router support.

use std::collections::BTreeSet;

use cosmwasm_std::{Addr, Coin, Order, StdError, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{
    Config, MetadataMode, MintStats, CONFIG, MINTABLE_NUM_TOKENS, MINTABLE_TOKEN_IDS, MINTER_ADDRS,
    MINT_STATS,
};

// ============================================================================
// V1 State Types (for reading old state)
// ============================================================================

/// Config from minter v1
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigV1 {
    pub admin: Addr,
    pub base_token_uri: String,
    pub num_tokens: u32,
    pub cw721_code_id: u64,
    pub unit_price: Coin,
    pub whitelist: Option<Addr>,
    pub start_time: Timestamp,
    pub per_address_limit: u32,
}

/// Config from minter-metadata-onchain
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigMetadataOnchainV1 {
    pub admin: Addr,
    pub max_num_tokens: u32,
    pub cw721_code_id: u64,
    pub unit_price: Coin,
    pub whitelist: Option<Addr>,
    pub start_time: Timestamp,
    pub per_address_limit: u32,
}

// ============================================================================
// V1 Storage Keys (must match exactly what v1 uses)
// ============================================================================

const CONFIG_V1: Item<ConfigV1> = Item::new("config");
const CW721_ADDRESS_V1: Item<Addr> = Item::new("cw721_address");
const MINTABLE_TOKEN_IDS_V1: Map<u32, bool> = Map::new("mt");
const MINTER_ADDRS_V1: Map<Addr, u32> = Map::new("ma");

const CONFIG_METADATA_ONCHAIN_V1: Item<ConfigMetadataOnchainV1> = Item::new("config");
const MINTABLE_TOKEN_IDS_METADATA_ONCHAIN_V1: Item<Vec<u32>> = Item::new("mintable_token_ids");
const MINTER_ADDRS_METADATA_ONCHAIN_V1: Map<Addr, u32> = Map::new("minter_address");

// ============================================================================
// Migration Logic
// ============================================================================

/// Migrate state from minter v1 to minter-v2
pub fn migrate_state(
    storage: &mut dyn Storage,
    source_contract: &str,
    registry: Option<Addr>,
    split_router: Option<Addr>,
    use_split_router: bool,
    base_token_uri: Option<String>,
) -> StdResult<MigrationResult> {
    if source_contract.contains("passage-minter-metadata-onchain") {
        migrate_from_metadata_onchain(
            storage,
            registry,
            split_router,
            use_split_router,
            base_token_uri,
        )
    } else if source_contract.ends_with("passage-minter") {
        migrate_from_minter_v1(storage, registry, split_router, use_split_router)
    } else {
        Err(StdError::generic_err(format!(
            "unsupported migration source contract: {source_contract}"
        )))
    }
}

fn migrate_from_minter_v1(
    storage: &mut dyn Storage,
    registry: Option<Addr>,
    split_router: Option<Addr>,
    use_split_router: bool,
) -> StdResult<MigrationResult> {
    let config_v1 = CONFIG_V1.load(storage)?;
    let cw721_address = CW721_ADDRESS_V1.load(storage)?;

    let config_v2 = Config {
        admin: config_v1.admin,
        cw721_address,
        cw721_code_id: config_v1.cw721_code_id,
        base_token_uri: config_v1.base_token_uri,
        num_tokens: config_v1.num_tokens,
        unit_price: config_v1.unit_price.clone(),
        per_address_limit: config_v1.per_address_limit,
        start_time: config_v1.start_time,
        whitelist: config_v1.whitelist,
        registry,
        split_router: split_router.clone(),
        use_split_router,
        metadata_mode: MetadataMode::OffChain,
        native_asset_template: vec![],
        paused: false,
    };
    CONFIG.save(storage, &config_v2)?;

    let mintable_rows: Vec<(u32, bool)> = MINTABLE_TOKEN_IDS_V1
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let mut mintable_remaining = 0u32;
    for (token_id, is_mintable) in mintable_rows {
        if is_mintable {
            MINTABLE_TOKEN_IDS.save(storage, token_id, &true)?;
            mintable_remaining += 1;
        }
    }
    MINTABLE_NUM_TOKENS.save(storage, &mintable_remaining)?;

    let minter_rows: Vec<(Addr, u32)> = MINTER_ADDRS_V1
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let mut unique_minters = 0u32;
    for (addr, count) in minter_rows {
        if count > 0 {
            MINTER_ADDRS.save(storage, &addr, &count)?;
            unique_minters += 1;
        }
    }

    let total_minted = config_v2.num_tokens.saturating_sub(mintable_remaining);
    let mint_stats = MintStats {
        total_minted,
        total_revenue: config_v2.unit_price.amount * Uint128::from(total_minted),
        total_routed: Uint128::zero(),
        unique_minters,
    };
    MINT_STATS.save(storage, &mint_stats)?;

    Ok(MigrationResult {
        source_contract: "crates.io:passage-minter".to_string(),
        tokens_migrated: config_v2.num_tokens,
        mintable_remaining,
        unique_minters,
        split_router_enabled: use_split_router,
    })
}

fn migrate_from_metadata_onchain(
    storage: &mut dyn Storage,
    registry: Option<Addr>,
    split_router: Option<Addr>,
    use_split_router: bool,
    base_token_uri: Option<String>,
) -> StdResult<MigrationResult> {
    let config_v1 = CONFIG_METADATA_ONCHAIN_V1.load(storage)?;
    let cw721_address = CW721_ADDRESS_V1.load(storage)?;
    let base_token_uri = base_token_uri.ok_or_else(|| {
        StdError::generic_err(
            "base_token_uri is required when migrating from passage-minter-metadata-onchain",
        )
    })?;

    let config_v2 = Config {
        admin: config_v1.admin,
        cw721_address,
        cw721_code_id: config_v1.cw721_code_id,
        base_token_uri,
        num_tokens: config_v1.max_num_tokens,
        unit_price: config_v1.unit_price.clone(),
        per_address_limit: config_v1.per_address_limit,
        start_time: config_v1.start_time,
        whitelist: config_v1.whitelist,
        registry,
        split_router: split_router.clone(),
        use_split_router,
        metadata_mode: MetadataMode::OffChain,
        native_asset_template: vec![],
        paused: false,
    };
    CONFIG.save(storage, &config_v2)?;

    let mintable_ids = MINTABLE_TOKEN_IDS_METADATA_ONCHAIN_V1
        .may_load(storage)?
        .unwrap_or_default();
    let mut seen = BTreeSet::new();
    let mut mintable_remaining = 0u32;
    for token_id in mintable_ids {
        if token_id != 0 && seen.insert(token_id) {
            MINTABLE_TOKEN_IDS.save(storage, token_id, &true)?;
            mintable_remaining += 1;
        }
    }
    MINTABLE_NUM_TOKENS.save(storage, &mintable_remaining)?;

    let minter_rows: Vec<(Addr, u32)> = MINTER_ADDRS_METADATA_ONCHAIN_V1
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let mut unique_minters = 0u32;
    for (addr, count) in minter_rows {
        if count > 0 {
            MINTER_ADDRS.save(storage, &addr, &count)?;
            unique_minters += 1;
        }
    }

    let total_minted = config_v2.num_tokens.saturating_sub(mintable_remaining);
    let mint_stats = MintStats {
        total_minted,
        total_revenue: config_v2.unit_price.amount * Uint128::from(total_minted),
        total_routed: Uint128::zero(),
        unique_minters,
    };
    MINT_STATS.save(storage, &mint_stats)?;

    Ok(MigrationResult {
        source_contract: "crates.io:passage-minter-metadata-onchain".to_string(),
        tokens_migrated: config_v2.num_tokens,
        mintable_remaining,
        unique_minters,
        split_router_enabled: use_split_router,
    })
}

/// Result of migration
pub struct MigrationResult {
    pub source_contract: String,
    pub tokens_migrated: u32,
    pub mintable_remaining: u32,
    pub unique_minters: u32,
    pub split_router_enabled: bool,
}
