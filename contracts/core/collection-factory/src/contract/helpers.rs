use super::*;

pub(super) fn is_admin(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr
}

pub(super) fn is_admin_or_operator(config: &Config, addr: &Addr) -> bool {
    config.admin == *addr || config.operators.contains(addr)
}

pub(super) fn ensure_non_empty(value: &str, err: ContractError) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        return Err(err);
    }
    Ok(())
}

pub(super) fn validate_create_collection_input(
    name: &str,
    symbol: &str,
    minter: &str,
    collection_info: &CollectionInfoInput,
) -> Result<(), ContractError> {
    ensure_non_empty(name, ContractError::EmptyName {})?;
    ensure_non_empty(symbol, ContractError::EmptySymbol {})?;
    ensure_non_empty(minter, ContractError::EmptyMinter {})?;
    ensure_non_empty(
        &collection_info.description,
        ContractError::EmptyDescription {},
    )?;
    ensure_non_empty(&collection_info.image, ContractError::EmptyImage {})?;

    if let Some(royalty) = &collection_info.royalty_info {
        if royalty.share > cosmwasm_std::Decimal::one() {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "royalty share cannot be greater than 1",
            )));
        }
    }

    Ok(())
}

pub(super) fn extract_contract_address_from_reply(msg: &Reply) -> Result<String, ContractError> {
    let res = msg
        .result
        .clone()
        .into_result()
        .map_err(|_| ContractError::InvalidInstantiateReplyData {})?;

    res.events
        .iter()
        .find(|e| e.ty == "instantiate")
        .and_then(|e| {
            e.attributes
                .iter()
                .find(|a| a.key == "_contract_address")
                .map(|a| a.value.clone())
        })
        .ok_or(ContractError::InvalidInstantiateReplyData {})
}
