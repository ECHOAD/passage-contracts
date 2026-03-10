use super::*;

// ========== Helpers ==========

pub(super) fn is_admin(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr
}

pub(super) fn is_admin_or_operator(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.operators.contains(addr)
}

pub(super) fn is_recovery_council(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.recovery_council.contains(addr)
}

pub(super) fn is_cross_ecosystem_admin(config: &Config, addr: &Addr) -> bool {
    is_admin_or_operator(config, addr)
}

pub(super) fn validate_id(id: &str) -> Result<(), ContractError> {
    // ID must be alphanumeric with hyphens, 3-64 characters
    if id.len() < 3 || id.len() > 64 {
        return Err(ContractError::InvalidIdFormat { id: id.to_string() });
    }

    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ContractError::InvalidIdFormat { id: id.to_string() });
    }

    Ok(())
}

pub(super) fn validate_ecosystem_policy(
    ecosystem_type: &EcosystemType,
    policy: &CollectionCreationPolicy,
) -> Result<(), ContractError> {
    match ecosystem_type {
        EcosystemType::Public => {
            if !matches!(policy, CollectionCreationPolicy::ApprovalRequired) {
                return Err(ContractError::InvalidEcosystemPolicyCombination {});
            }
        }
        EcosystemType::Private => {
            if !matches!(policy, CollectionCreationPolicy::Permissioned) {
                return Err(ContractError::InvalidEcosystemPolicyCombination {});
            }
        }
    }

    Ok(())
}

pub(super) fn is_owner_controlled_ecosystem(config: &Config, ecosystem: &Ecosystem) -> bool {
    !is_cross_ecosystem_admin(config, &ecosystem.admin)
}

pub(super) fn can_cross_admin_manage_ecosystem(
    config: &Config,
    ecosystem: &Ecosystem,
    sender: &Addr,
) -> bool {
    is_cross_ecosystem_admin(config, sender) && !is_owner_controlled_ecosystem(config, ecosystem)
}

pub(super) fn can_manage_ecosystem(config: &Config, ecosystem: &Ecosystem, sender: &Addr) -> bool {
    ecosystem.admin == *sender || can_cross_admin_manage_ecosystem(config, ecosystem, sender)
}

pub(super) fn normalize_reason(reason: Option<String>) -> Option<String> {
    reason.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub(super) fn default_creator_moderation() -> CreatorModeration {
    CreatorModeration {
        ecosystem_creation_enabled: true,
        collection_creation_enabled: true,
        mint_enabled: true,
        trade_enabled: true,
        reason: None,
        moderated_by: None,
        moderated_at: None,
    }
}

pub(super) fn default_ecosystem_moderation() -> EcosystemModeration {
    EcosystemModeration {
        collection_creation_enabled: true,
        mint_enabled: true,
        trade_enabled: true,
        reason: None,
        moderated_by: None,
        moderated_at: None,
    }
}

pub(super) fn default_collection_moderation() -> CollectionModeration {
    CollectionModeration {
        mint_enabled: true,
        trade_enabled: true,
        reason: None,
        moderated_by: None,
        moderated_at: None,
    }
}

pub(super) fn default_recovery_policy() -> RecoveryPolicy {
    RecoveryPolicy {
        delegate: None,
        designated_successor: None,
        updated_by: None,
        updated_at: None,
    }
}

pub(super) fn creator_moderation(
    storage: &dyn cosmwasm_std::Storage,
    creator: &Addr,
) -> StdResult<CreatorModeration> {
    Ok(CREATOR_MODERATION
        .may_load(storage, creator.clone())?
        .unwrap_or_else(default_creator_moderation))
}

pub(super) fn ecosystem_moderation(
    storage: &dyn cosmwasm_std::Storage,
    ecosystem_id: &str,
) -> StdResult<EcosystemModeration> {
    Ok(ECOSYSTEM_MODERATION
        .may_load(storage, ecosystem_id.to_string())?
        .unwrap_or_else(default_ecosystem_moderation))
}

pub(super) fn collection_moderation(
    storage: &dyn cosmwasm_std::Storage,
    collection: &Addr,
) -> StdResult<CollectionModeration> {
    Ok(COLLECTION_MODERATION
        .may_load(storage, collection.clone())?
        .unwrap_or_else(default_collection_moderation))
}

pub(super) fn ecosystem_recovery_policy(
    storage: &dyn cosmwasm_std::Storage,
    ecosystem_id: &str,
) -> StdResult<RecoveryPolicy> {
    Ok(ECOSYSTEM_RECOVERY_POLICIES
        .may_load(storage, ecosystem_id.to_string())?
        .unwrap_or_else(default_recovery_policy))
}

pub(super) fn collection_recovery_policy(
    storage: &dyn cosmwasm_std::Storage,
    collection: &Addr,
) -> StdResult<RecoveryPolicy> {
    Ok(COLLECTION_RECOVERY_POLICIES
        .may_load(storage, collection.clone())?
        .unwrap_or_else(default_recovery_policy))
}

pub(super) fn can_create_ecosystem(
    storage: &dyn cosmwasm_std::Storage,
    creator: &Addr,
) -> StdResult<bool> {
    Ok(creator_moderation(storage, creator)?.ecosystem_creation_enabled)
}

pub(super) fn can_create_collection_in_ecosystem(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    ecosystem: &Ecosystem,
    creator: &Addr,
) -> bool {
    let moderation_subject = if is_cross_ecosystem_admin(config, creator) {
        ecosystem.admin.clone()
    } else {
        creator.clone()
    };

    let creator_moderation = match creator_moderation(storage, &moderation_subject) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if !creator_moderation.collection_creation_enabled {
        return false;
    }

    let ecosystem_moderation = match ecosystem_moderation(storage, &ecosystem.id) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if !ecosystem_moderation.collection_creation_enabled {
        return false;
    }

    if ecosystem.admin == *creator {
        return true;
    }

    if is_cross_ecosystem_admin(config, creator) {
        return !is_owner_controlled_ecosystem(config, ecosystem);
    }

    let is_member = ECOSYSTEM_MEMBERS.has(storage, (ecosystem.id.clone(), creator.clone()));

    match ecosystem.collection_creation_policy {
        CollectionCreationPolicy::Open => true,
        CollectionCreationPolicy::Permissioned | CollectionCreationPolicy::ApprovalRequired => {
            is_member
        }
    }
}

pub(super) fn can_mint_collection(
    storage: &dyn cosmwasm_std::Storage,
    collection: &Collection,
) -> StdResult<bool> {
    let creator_rules = creator_moderation(storage, &collection.creator)?;
    if !creator_rules.mint_enabled {
        return Ok(false);
    }

    let ecosystem_rules = ecosystem_moderation(storage, &collection.ecosystem_id)?;
    if !ecosystem_rules.mint_enabled {
        return Ok(false);
    }

    let collection_rules = collection_moderation(storage, &collection.address)?;
    Ok(collection_rules.mint_enabled)
}

pub(super) fn can_trade_collection(
    storage: &dyn cosmwasm_std::Storage,
    collection: &Collection,
) -> StdResult<bool> {
    let creator_rules = creator_moderation(storage, &collection.creator)?;
    if !creator_rules.trade_enabled {
        return Ok(false);
    }

    let ecosystem_rules = ecosystem_moderation(storage, &collection.ecosystem_id)?;
    if !ecosystem_rules.trade_enabled {
        return Ok(false);
    }

    let collection_rules = collection_moderation(storage, &collection.address)?;
    Ok(collection_rules.trade_enabled)
}

pub(super) fn can_manage_collection(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    collection: &Collection,
    sender: &Addr,
) -> StdResult<bool> {
    let ecosystem = ECOSYSTEMS.may_load(storage, collection.ecosystem_id.clone())?;

    if is_cross_ecosystem_admin(config, sender)
        && ecosystem
            .as_ref()
            .map(|eco| is_owner_controlled_ecosystem(config, eco))
            .unwrap_or(false)
    {
        return Ok(false);
    }

    if collection.creator == *sender {
        return Ok(true);
    }

    if !is_cross_ecosystem_admin(config, sender) {
        return Ok(false);
    }

    Ok(ecosystem
        .as_ref()
        .map(|eco| can_cross_admin_manage_ecosystem(config, eco, sender))
        .unwrap_or(false))
}

pub(super) fn effective_collection_creator(
    config: &Config,
    ecosystem: &Ecosystem,
    requested_creator: &Addr,
) -> Addr {
    if is_cross_ecosystem_admin(config, requested_creator) {
        ecosystem.admin.clone()
    } else {
        requested_creator.clone()
    }
}

pub(super) fn touch_creator_activity(
    storage: &mut dyn cosmwasm_std::Storage,
    addr: &Addr,
    at: u64,
) -> StdResult<()> {
    LAST_CREATOR_ACTIVITY.save(storage, addr.clone(), &at)
}

pub(super) fn recovery_policy_for_target(
    storage: &dyn cosmwasm_std::Storage,
    target: &RecoveryTarget,
) -> StdResult<RecoveryPolicy> {
    match target {
        RecoveryTarget::Ecosystem { ecosystem_id } => {
            ecosystem_recovery_policy(storage, ecosystem_id)
        }
        RecoveryTarget::Collection { address } => collection_recovery_policy(storage, address),
    }
}

pub(super) fn recovery_target_key(target: &RecoveryTarget) -> String {
    match target {
        RecoveryTarget::Ecosystem { ecosystem_id } => format!("ecosystem:{ecosystem_id}"),
        RecoveryTarget::Collection { address } => format!("collection:{address}"),
    }
}
