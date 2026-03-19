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

    let instantiate_data = res
        .msg_responses
        .first()
        .map(|response| response.value.as_slice())
        .ok_or(ContractError::InvalidInstantiateReplyData {})?;

    parse_instantiate_response_data(instantiate_data)
}

fn parse_instantiate_response_data(data: &[u8]) -> Result<String, ContractError> {
    let mut remaining = data;

    while !remaining.is_empty() {
        let (field_number, wire_type, after_key) = read_key(remaining)?;
        if wire_type != 2 {
            return Err(ContractError::InvalidInstantiateReplyData {});
        }

        let (value, after_value) = read_length_delimited(after_key)?;
        if field_number == 1 {
            return String::from_utf8(value)
                .map_err(|_| ContractError::InvalidInstantiateReplyData {});
        }

        remaining = after_value;
    }

    Err(ContractError::InvalidInstantiateReplyData {})
}

fn read_key(data: &[u8]) -> Result<(u32, u8, &[u8]), ContractError> {
    let (raw_key, rest) = read_varint(data)?;
    Ok(((raw_key >> 3) as u32, (raw_key & 0x07) as u8, rest))
}

fn read_length_delimited(data: &[u8]) -> Result<(Vec<u8>, &[u8]), ContractError> {
    let (len, rest) = read_varint(data)?;
    if rest.len() < len {
        return Err(ContractError::InvalidInstantiateReplyData {});
    }

    Ok((rest[..len].to_vec(), &rest[len..]))
}

fn read_varint(data: &[u8]) -> Result<(usize, &[u8]), ContractError> {
    let mut value: usize = 0;
    let mut shift = 0usize;

    for (index, byte) in data.iter().enumerate().take(9) {
        value |= ((byte & 0x7f) as usize) << shift;
        if byte & 0x80 == 0 {
            return Ok((value, &data[index + 1..]));
        }
        shift += 7;
    }

    Err(ContractError::InvalidInstantiateReplyData {})
}
