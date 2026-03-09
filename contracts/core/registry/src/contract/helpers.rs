use super::*;

// ========== Helpers ==========

pub(super) fn is_admin(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr
}

pub(super) fn is_admin_or_operator(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.operators.contains(addr)
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
    if matches!(ecosystem_type, EcosystemType::Private)
        && matches!(policy, CollectionCreationPolicy::Open)
    {
        return Err(ContractError::InvalidEcosystemPolicyCombination {});
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

pub(super) fn can_create_collection_in_ecosystem(
    storage: &dyn cosmwasm_std::Storage,
    config: &Config,
    ecosystem: &Ecosystem,
    creator: &Addr,
) -> bool {
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

pub(super) fn recovery_target_key(target: &RecoveryTarget) -> String {
    match target {
        RecoveryTarget::Ecosystem { ecosystem_id } => format!("ecosystem:{ecosystem_id}"),
        RecoveryTarget::Collection { address } => format!("collection:{address}"),
    }
}
