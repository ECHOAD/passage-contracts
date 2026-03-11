use super::*;
use std::collections::HashSet;

pub(super) fn validate_key(key: &str) -> Result<(), ContractError> {
    if key.trim().is_empty() {
        return Err(ContractError::EmptyKey {});
    }

    Ok(())
}

pub(super) fn build_recipients(
    deps: Deps,
    recipients: Vec<RecipientInput>,
) -> Result<Vec<Recipient>, ContractError> {
    let recipients: Vec<Recipient> = recipients
        .into_iter()
        .map(|recipient| {
            Ok(Recipient {
                address: deps.api.addr_validate(&recipient.address)?,
                share: recipient.share,
                label: recipient.label,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    validate_recipients(&recipients)?;

    Ok(recipients)
}

pub(super) fn validate_recipients(recipients: &[Recipient]) -> Result<(), ContractError> {
    if recipients.is_empty() {
        return Err(ContractError::NoRecipients {});
    }

    let mut seen = HashSet::new();
    let mut total_share = Decimal::zero();

    for recipient in recipients {
        if recipient.share.is_zero() {
            return Err(ContractError::ZeroShare {
                address: recipient.address.to_string(),
            });
        }

        let address = recipient.address.to_string();
        if !seen.insert(address.clone()) {
            return Err(ContractError::DuplicateRecipient { address });
        }

        total_share += recipient.share;
    }

    if total_share != Decimal::one() {
        return Err(ContractError::InvalidTotalShare {
            expected: Decimal::one(),
            actual: total_share,
        });
    }

    Ok(())
}

pub(super) fn ensure_rule_manager(
    config: &Config,
    sender: &Addr,
    owner: &Addr,
) -> Result<(), ContractError> {
    if config.admin == *sender || owner == sender {
        return Ok(());
    }

    Err(ContractError::NotRuleOwner {})
}

pub(super) fn calculate_split_amounts(
    recipients: &[Recipient],
    funds: &[Coin],
) -> Vec<(Addr, Vec<Coin>)> {
    let mut recipient_amounts: Vec<(Addr, Vec<Coin>)> = recipients
        .iter()
        .map(|recipient| (recipient.address.clone(), Vec::new()))
        .collect();

    let last_index = recipients.len().saturating_sub(1);

    for coin in funds {
        let mut remaining = coin.amount;

        for (index, recipient) in recipients.iter().enumerate() {
            let amount = if index == last_index {
                remaining
            } else {
                let amount = coin.amount.multiply_ratio(
                    recipient.share.atomics().u128(),
                    10u128.pow(Decimal::DECIMAL_PLACES),
                );
                remaining = remaining.saturating_sub(amount);
                amount
            };

            if !amount.is_zero() {
                recipient_amounts[index].1.push(Coin {
                    denom: coin.denom.clone(),
                    amount,
                });
            }
        }
    }

    recipient_amounts
}

pub(super) fn record_split_event(
    storage: &mut dyn cosmwasm_std::Storage,
    env: &Env,
    key: &str,
    total_funds: Vec<Coin>,
    recipient_amounts: Vec<(Addr, Vec<Coin>)>,
    sender: &Addr,
) -> Result<u64, ContractError> {
    let mut event_id = SPLIT_EVENT_COUNT.load(storage)?;
    event_id += 1;

    if event_id > MAX_EVENTS_STORED {
        let old_id = event_id - MAX_EVENTS_STORED;
        SPLIT_EVENTS.remove(storage, old_id);
    }

    let event = SplitEvent {
        id: event_id,
        key: key.to_string(),
        total_funds,
        recipient_amounts,
        timestamp: env.block.time.seconds(),
        sender: sender.clone(),
    };

    SPLIT_EVENTS.save(storage, event_id, &event)?;
    SPLIT_EVENT_COUNT.save(storage, &event_id)?;

    Ok(event_id)
}
