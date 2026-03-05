use super::*;

// ========== Helpers ==========

pub(super) fn is_admin(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr
}

pub(super) fn is_admin_or_operator(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.operators.contains(addr)
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
