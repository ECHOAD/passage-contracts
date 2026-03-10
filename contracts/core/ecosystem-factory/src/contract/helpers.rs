use super::*;

pub(super) fn is_admin(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr
}

pub(super) fn is_admin_or_operator(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.operators.contains(addr)
}

pub(super) fn validate_request_input(
    id: &str,
    name: &str,
    description: &str,
    image_urls: &[String],
) -> Result<(), ContractError> {
    if id.trim().is_empty() {
        return Err(ContractError::EmptyEcosystemId {});
    }
    if name.trim().is_empty() {
        return Err(ContractError::EmptyEcosystemName {});
    }
    if description.trim().is_empty() {
        return Err(ContractError::EmptyEcosystemDescription {});
    }
    if image_urls.is_empty() || image_urls.iter().any(|img| img.trim().is_empty()) {
        return Err(ContractError::EmptyImages {});
    }
    Ok(())
}
