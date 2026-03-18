use super::*;
use cw_utils::parse_instantiate_response_data;

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
pub(super) fn extract_collection_factory_address_from_reply(
    msg: &Reply,
) -> Result<String, ContractError> {
    let res = msg
        .result
        .clone()
        .into_result()
        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::generic_err(format!(
            "SubMsg failed: {}",
            e
        ))))?;

    let instantiate_data = res
        .msg_responses
        .first()
        .map(|response| response.value.as_slice())
        .ok_or(ContractError::ReplyParseError {})?;

    let parsed = parse_instantiate_response_data(instantiate_data)
        .map_err(|_| ContractError::ReplyParseError {})?;

    Ok(parsed.contract_address)
}
