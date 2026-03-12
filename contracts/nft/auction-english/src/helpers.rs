use crate::{
    error::ContractError,
    msg::{
        CollectionInfoResponse, Cw721ExecuteMsg, Cw721QueryMsg, OwnerOfResponse, Pg721QueryMsg,
        RegistryApprovalStatusResponse, RegistryCollectionResponse, RegistryQueryMsg,
        RoyaltyInfoResponse,
    },
    state::{Auction, Config},
};
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, Deps, Env, MessageInfo, QueryRequest,
    StdResult, Uint128, WasmMsg, WasmQuery,
};

pub struct RoyaltyPayout {
    pub recipient: Addr,
    pub amount: Uint128,
}

pub fn validate_config(config: &Config) -> Result<(), ContractError> {
    if config.trading_fee_bps > config.max_trading_fee_bps {
        return Err(ContractError::TradingFeeExceedsMax {
            fee_bps: config.trading_fee_bps,
            max_bps: config.max_trading_fee_bps,
        });
    }

    if config.max_trading_fee_bps > 10_000 {
        return Err(ContractError::InvalidConfig {
            reason: "max_trading_fee_bps cannot exceed 10000".to_string(),
        });
    }

    if config.min_price.is_zero() {
        return Err(ContractError::InvalidConfig {
            reason: "min_price must be greater than zero".to_string(),
        });
    }

    if config.min_bid_increment_percent.is_zero()
        || config.min_bid_increment_percent >= Decimal::one()
    {
        return Err(ContractError::InvalidConfig {
            reason: "min_bid_increment_percent must be greater than zero and less than one"
                .to_string(),
        });
    }

    if config.min_duration == 0 || config.max_duration == 0 || config.extend_duration == 0 {
        return Err(ContractError::InvalidConfig {
            reason: "durations must be greater than zero".to_string(),
        });
    }

    if config.min_duration > config.max_duration {
        return Err(ContractError::InvalidConfig {
            reason: "min_duration cannot exceed max_duration".to_string(),
        });
    }

    if config.require_registration && config.registry.is_none() {
        return Err(ContractError::InvalidConfig {
            reason: "registry must be set when require_registration is true".to_string(),
        });
    }

    Ok(())
}

pub fn validate_collection_registration(
    deps: Deps,
    config: &Config,
    collection: &Addr,
) -> Result<(), ContractError> {
    let Some(registry) = config.registry.as_ref() else {
        return Ok(());
    };

    let response: RegistryCollectionResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::Collection {
                address: collection.to_string(),
            },
        )
        .map_err(|_| ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        })?;

    if response.collection.is_none() {
        return Err(ContractError::CollectionNotRegistered {
            collection: collection.to_string(),
        });
    }

    let trade_allowed: RegistryApprovalStatusResponse = deps
        .querier
        .query_wasm_smart(
            registry.to_string(),
            &RegistryQueryMsg::CanTradeCollection {
                address: collection.to_string(),
            },
        )
        .map_err(|_| ContractError::CollectionTradingDisabled {
            collection: collection.to_string(),
        })?;

    if !trade_allowed.approved {
        return Err(ContractError::CollectionTradingDisabled {
            collection: collection.to_string(),
        });
    }

    Ok(())
}

pub fn validate_reserve_price(config: &Config, reserve_price: &Coin) -> Result<(), ContractError> {
    if reserve_price.denom != config.denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: config.denom.clone(),
            received: reserve_price.denom.clone(),
        });
    }

    if reserve_price.amount < config.min_price {
        return Err(ContractError::InvalidReservePrice {
            min_price: Coin {
                denom: config.denom.clone(),
                amount: config.min_price,
            }
            .to_string(),
        });
    }

    Ok(())
}

pub fn validate_duration(config: &Config, duration: u64) -> Result<(), ContractError> {
    if duration < config.min_duration || duration > config.max_duration {
        return Err(ContractError::InvalidDuration {
            min: config.min_duration,
            max: config.max_duration,
            got: duration,
        });
    }

    Ok(())
}

pub fn query_owner(
    deps: Deps,
    collection: &Addr,
    token_id: &str,
) -> Result<OwnerOfResponse, ContractError> {
    let response: OwnerOfResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&Cw721QueryMsg::OwnerOf {
                token_id: token_id.to_string(),
                include_expired: Some(false),
            })?,
        }))
        .map_err(|_| ContractError::NftQueryFailed {})?;

    Ok(response)
}

pub fn ensure_owner_and_approval(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    collection: &Addr,
    token_id: &str,
) -> Result<(), ContractError> {
    let owner_response = query_owner(deps, collection, token_id)?;
    let owner = deps.api.addr_validate(&owner_response.owner)?;

    if owner != *sender {
        return Err(ContractError::NotTokenOwner {
            token_id: token_id.to_string(),
        });
    }

    let approved = owner_response
        .approvals
        .iter()
        .any(|approval| approval.spender == env.contract.address.as_str());

    if !approved {
        return Err(ContractError::NftNotApproved {});
    }

    Ok(())
}

pub fn validate_bid_funds(info: &MessageInfo, denom: &str) -> Result<Uint128, ContractError> {
    let Some(payment) = info.funds.first() else {
        return Err(ContractError::InvalidPaymentDenom {
            expected: denom.to_string(),
            received: "none".to_string(),
        });
    };

    if info.funds.len() != 1 {
        return Err(ContractError::InvalidConfig {
            reason: "bid payments must send exactly one coin".to_string(),
        });
    }

    if payment.denom != denom {
        return Err(ContractError::InvalidPaymentDenom {
            expected: denom.to_string(),
            received: payment.denom.clone(),
        });
    }

    Ok(payment.amount)
}

pub fn build_transfer_nft_msg(
    collection: &Addr,
    token_id: &str,
    recipient: &Addr,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection.to_string(),
        msg: to_json_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: recipient.to_string(),
            token_id: token_id.to_string(),
        })?,
        funds: vec![],
    }))
}

pub fn query_royalty_payout(
    deps: Deps,
    collection: &Addr,
    amount: Uint128,
) -> Result<Option<RoyaltyPayout>, ContractError> {
    let result: Result<CollectionInfoResponse, _> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&Pg721QueryMsg::CollectionInfo {})?,
        }));

    let Ok(collection_info) = result else {
        return Ok(None);
    };

    let Some(RoyaltyInfoResponse {
        payment_address,
        share,
    }) = collection_info.royalty_info
    else {
        return Ok(None);
    };

    let share: Decimal = share
        .parse()
        .map_err(|_| ContractError::RoyaltyQueryFailed {})?;
    let recipient = deps
        .api
        .addr_validate(&payment_address)
        .map_err(|_| ContractError::RoyaltyQueryFailed {})?;
    let royalty_amount =
        amount.multiply_ratio(share.atomics().u128(), 10u128.pow(Decimal::DECIMAL_PLACES));

    Ok(Some(RoyaltyPayout {
        recipient,
        amount: royalty_amount,
    }))
}

pub fn auction_is_settleable(auction: &Auction, now: cosmwasm_std::Timestamp) -> bool {
    matches!(auction.status(now), crate::state::AuctionStatus::Ended) && auction.high_bid.is_some()
}

pub fn build_bank_send_msg(to: &Addr, denom: &str, amount: Uint128) -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: to.to_string(),
        amount: vec![Coin {
            denom: denom.to_string(),
            amount,
        }],
    })
}
