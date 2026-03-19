use crate::error::ContractError;
use crate::msg::{
    AdminAction, ConfigResponse, DepositResponse, ExecuteMsg, InstantiateMsg,
    PasgUtilityConfigResponse, ProposalAction, ProposalResponse, ProposalsResponse, QueryMsg,
    RatifiedAdminActionResponse, VoteResponse, VotesResponse, VotingPowerResponse,
};
use crate::state::{
    Ballot, Config, Delegation, LockedBalance, PasgUtilityConfig, Proposal, ProposalSnapshot,
    ProposalStatus, RatifiedAdminAction, Vote, BALLOTS, CONFIG, DELEGATIONS, DEPOSITS,
    LOCKED_BALANCES, PASG_UTILITY_CONFIG, PROPOSALS, PROPOSAL_COUNT, PROPOSAL_SNAPSHOTS,
    RATIFIED_ADMIN_ACTIONS, TOTAL_DEPOSITED,
};
use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use sha2::{Digest, Sha256};

const CONTRACT_NAME: &str = "crates.io:pasg-governance";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PAGE_LIMIT: usize = 30;
const MAX_PAGE_LIMIT: usize = 100;
const BPS_SCALE: u128 = 10_000;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.native_denom.trim().is_empty() {
        return Err(ContractError::EmptyNativeDenom {});
    }
    if msg.voting_period_secs == 0 {
        return Err(ContractError::InvalidVotingPeriod {});
    }
    if msg.proposal_deposit.is_zero() {
        return Err(ContractError::InvalidProposalDeposit {});
    }
    if msg.quorum_bps == 0
        || msg.quorum_bps > BPS_SCALE as u64
        || msg.pass_bps == 0
        || msg.pass_bps > BPS_SCALE as u64
    {
        return Err(ContractError::InvalidBps {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin_multisig = deps.api.addr_validate(&msg.admin_multisig)?;
    let config = Config {
        admin_multisig: admin_multisig.clone(),
        native_denom: msg.native_denom.trim().to_string(),
        proposal_deposit: msg.proposal_deposit,
        voting_period_secs: msg.voting_period_secs,
        quorum_bps: msg.quorum_bps,
        pass_bps: msg.pass_bps,
    };

    CONFIG.save(deps.storage, &config)?;
    PROPOSAL_COUNT.save(deps.storage, &0)?;
    TOTAL_DEPOSITED.save(deps.storage, &Uint128::zero())?;
    PASG_UTILITY_CONFIG.save(
        deps.storage,
        &PasgUtilityConfig {
            points_per_pasg: Uint128::zero(),
            max_fiat_report_age_secs: 0,
            max_session_duration_secs: 0,
            updated_by_proposal: None,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin_multisig", admin_multisig)
        .add_attribute("native_denom", config.native_denom))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DepositVotingPower {} => execute_deposit(deps, info),
        ExecuteMsg::WithdrawVotingPower { amount } => execute_withdraw(deps, info, amount),
        ExecuteMsg::Delegate { delegate } => execute_delegate(deps, env, info, delegate),
        ExecuteMsg::Undelegate {} => execute_undelegate(deps, info),
        ExecuteMsg::Propose {
            title,
            description,
            action,
        } => execute_propose(deps, env, info, title, description, action),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::ExecuteProposal { proposal_id } => {
            execute_execute_proposal(deps, env, proposal_id)
        }
        ExecuteMsg::Close { proposal_id } => execute_close(deps, env, proposal_id),
    }
}

fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let amount = must_pay_native(&info, &config.native_denom)?;

    DEPOSITS.update(deps.storage, &info.sender, |current| -> StdResult<_> {
        Ok(current.unwrap_or_default() + amount)
    })?;
    TOTAL_DEPOSITED.update(deps.storage, |current| -> StdResult<_> {
        Ok(current + amount)
    })?;

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", amount))
}

fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let deposited = DEPOSITS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    if deposited < amount || amount.is_zero() {
        return Err(ContractError::InsufficientVotingPower {
            required: amount,
            actual: deposited,
        });
    }

    let locked_balance = locked_balance(deps.as_ref(), &info.sender)?;
    let unlocked = deposited.saturating_sub(locked_balance);
    if amount > unlocked {
        return Err(ContractError::InsufficientUnlockedBalance {
            requested: amount,
            unlocked,
        });
    }

    let remaining = deposited.checked_sub(amount).unwrap();
    DEPOSITS.save(deps.storage, &info.sender, &remaining)?;
    TOTAL_DEPOSITED.update(deps.storage, |current| -> StdResult<_> {
        Ok(current.checked_sub(amount).unwrap())
    })?;

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin::new(amount.u128(), config.native_denom)],
        })
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", amount))
}

fn execute_delegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    delegate: String,
) -> Result<Response, ContractError> {
    let delegate = deps.api.addr_validate(&delegate)?;
    if delegate == info.sender {
        return Err(ContractError::SelfDelegation {});
    }
    ensure_no_delegation_cycle(deps.as_ref(), &info.sender, &delegate)?;

    DELEGATIONS.save(
        deps.storage,
        &info.sender,
        &Delegation {
            delegator: info.sender.clone(),
            delegate: delegate.clone(),
            created_at: env.block.time.seconds(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "delegate")
        .add_attribute("delegator", info.sender)
        .add_attribute("delegate", delegate))
}

fn execute_undelegate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    DELEGATIONS.remove(deps.storage, &info.sender);

    Ok(Response::new()
        .add_attribute("action", "undelegate")
        .add_attribute("delegator", info.sender))
}

fn execute_propose(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: Option<String>,
    action: ProposalAction,
) -> Result<Response, ContractError> {
    if title.trim().is_empty() {
        return Err(ContractError::EmptyTitle {});
    }
    validate_proposal_action(&action)?;

    let config = CONFIG.load(deps.storage)?;
    let deposited = DEPOSITS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    if deposited < config.proposal_deposit {
        return Err(ContractError::InsufficientVotingPower {
            required: config.proposal_deposit,
            actual: deposited,
        });
    }

    let proposal_id =
        PROPOSAL_COUNT.update(deps.storage, |current| -> StdResult<_> { Ok(current + 1) })?;
    let now = env.block.time.seconds();
    let snapshot = ProposalSnapshot {
        proposal_id,
        total_power: snapshot_total_power(deps.as_ref())?,
        quorum_bps: config.quorum_bps,
        pass_bps: config.pass_bps,
        created_at: now,
    };
    let voting_addresses = resolve_voting_addresses(deps.as_ref(), &info.sender)?;
    let voting_power = resolve_effective_power(deps.as_ref(), &info.sender)?;
    if voting_power.is_zero() {
        return Err(ContractError::InsufficientVotingPower {
            required: Uint128::new(1),
            actual: voting_power,
        });
    }

    let mut proposal = Proposal {
        id: proposal_id,
        title: title.trim().to_string(),
        description: normalize_optional_text(description),
        proposer: info.sender.clone(),
        action,
        status: ProposalStatus::Open,
        yes_power: voting_power,
        no_power: Uint128::zero(),
        total_power_snapshot: snapshot.total_power,
        expires_at: now + config.voting_period_secs,
        created_at: now,
        executed_at: None,
    };

    BALLOTS.save(
        deps.storage,
        (proposal_id, &info.sender),
        &Ballot {
            voter: info.sender.clone(),
            vote: Vote::Approve,
            weight: voting_power,
            voted_at: now,
        },
    )?;
    PROPOSAL_SNAPSHOTS.save(deps.storage, proposal_id, &snapshot)?;
    lock_addresses_for_proposal(deps.storage, proposal_id, &voting_addresses)?;

    refresh_status(&mut proposal, now, &snapshot);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("proposer", info.sender)
        .add_attribute("yes_power", voting_power)
        .add_attribute("status", format!("{:?}", proposal.status)))
}

fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
    vote: Vote,
) -> Result<Response, ContractError> {
    if BALLOTS.has(deps.storage, (proposal_id, &info.sender)) {
        return Err(ContractError::AlreadyVoted {});
    }

    let voting_power = resolve_effective_power(deps.as_ref(), &info.sender)?;
    if voting_power.is_zero() {
        return Err(ContractError::InsufficientVotingPower {
            required: Uint128::new(1),
            actual: voting_power,
        });
    }

    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let snapshot = PROPOSAL_SNAPSHOTS.load(deps.storage, proposal_id)?;
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, &snapshot);
    if proposal.status != ProposalStatus::Open {
        return Err(if now > proposal.expires_at {
            ContractError::ProposalExpired {}
        } else {
            ContractError::ProposalNotOpen {}
        });
    }

    match vote {
        Vote::Approve => proposal.yes_power += voting_power,
        Vote::Reject => proposal.no_power += voting_power,
    }

    BALLOTS.save(
        deps.storage,
        (proposal_id, &info.sender),
        &Ballot {
            voter: info.sender.clone(),
            vote: vote.clone(),
            weight: voting_power,
            voted_at: now,
        },
    )?;
    let voting_addresses = resolve_voting_addresses(deps.as_ref(), &info.sender)?;
    lock_addresses_for_proposal(deps.storage, proposal_id, &voting_addresses)?;

    refresh_status(&mut proposal, now, &snapshot);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", info.sender)
        .add_attribute("vote", format!("{:?}", vote))
        .add_attribute("weight", voting_power)
        .add_attribute("status", format!("{:?}", proposal.status)))
}

fn execute_execute_proposal(
    deps: DepsMut,
    env: Env,
    proposal_id: u64,
) -> Result<Response, ContractError> {
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let config = CONFIG.load(deps.storage)?;
    let snapshot = PROPOSAL_SNAPSHOTS.load(deps.storage, proposal_id)?;
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, &snapshot);
    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status != ProposalStatus::Passed {
        if !proposal_meets_quorum(&proposal, &snapshot) {
            return Err(ContractError::QuorumNotMet {});
        }
        if !proposal_meets_pass_threshold(&proposal, &snapshot) {
            return Err(ContractError::PassThresholdNotMet {});
        }
        return Err(ContractError::ProposalNotPassed {});
    }

    let mut response = Response::new()
        .add_attribute("action", "execute_proposal")
        .add_attribute("proposal_id", proposal_id.to_string());

    match proposal.action.clone() {
        ProposalAction::SetPasgUtilityConfig {
            points_per_pasg,
            max_fiat_report_age_secs,
            max_session_duration_secs,
        } => {
            PASG_UTILITY_CONFIG.save(
                deps.storage,
                &PasgUtilityConfig {
                    points_per_pasg,
                    max_fiat_report_age_secs,
                    max_session_duration_secs,
                    updated_by_proposal: Some(proposal_id),
                },
            )?;
            response = response.add_attribute("proposal_action", "set_pasg_utility_config");
        }
        ProposalAction::StageAdminAction { action } => {
            validate_admin_action(&action)?;
            let payload_hash = hash_admin_action(&action)?;
            RATIFIED_ADMIN_ACTIONS.save(
                deps.storage,
                proposal_id,
                &RatifiedAdminAction {
                    proposal_id,
                    admin_multisig: config.admin_multisig.clone(),
                    action,
                    payload_hash: payload_hash.clone(),
                    ratified_at: now,
                },
            )?;
            response = response
                .add_attribute("proposal_action", "stage_admin_action")
                .add_attribute("ratified_admin_action", proposal_id.to_string())
                .add_attribute("admin_multisig", config.admin_multisig)
                .add_attribute("payload_hash", payload_hash);
        }
    }

    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(now);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    unlock_proposal_balances(deps.storage, proposal_id)?;

    Ok(response)
}

fn execute_close(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let snapshot = PROPOSAL_SNAPSHOTS.load(deps.storage, proposal_id)?;
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, &snapshot);

    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status == ProposalStatus::Passed {
        return Err(ContractError::ProposalNotPassed {});
    }
    if proposal.status == ProposalStatus::Open && now <= proposal.expires_at {
        return Err(ContractError::ProposalNotOpen {});
    }

    proposal.status = ProposalStatus::Rejected;
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    unlock_proposal_balances(deps.storage, proposal_id)?;

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("proposal_id", proposal_id.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&ConfigResponse {
            config: CONFIG.load(deps.storage)?,
        }),
        QueryMsg::Deposit { address } => {
            let addr = deps.api.addr_validate(&address)?;
            to_json_binary(&DepositResponse {
                address,
                deposited: DEPOSITS.may_load(deps.storage, &addr)?.unwrap_or_default(),
            })
        }
        QueryMsg::VotingPower { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let deposited = DEPOSITS.may_load(deps.storage, &addr)?.unwrap_or_default();
            let delegated_to = DELEGATIONS
                .may_load(deps.storage, &addr)?
                .map(|delegation| delegation.delegate.to_string());
            let incoming_delegated_power = delegated_power(deps, &addr)?;
            let effective_voting_power = resolve_effective_power(deps, &addr)?;
            to_json_binary(&VotingPowerResponse {
                address,
                deposited,
                delegated_to,
                incoming_delegated_power,
                effective_voting_power,
                locked_balance: locked_balance(deps, &addr)?,
            })
        }
        QueryMsg::PasgUtilityConfig {} => to_json_binary(&PasgUtilityConfigResponse {
            config: PASG_UTILITY_CONFIG.load(deps.storage)?,
        }),
        QueryMsg::Proposal { proposal_id } => {
            let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
            let config = CONFIG.load(deps.storage)?;
            let snapshot = PROPOSAL_SNAPSHOTS
                .may_load(deps.storage, proposal_id)?
                .unwrap_or(ProposalSnapshot {
                    proposal_id,
                    total_power: proposal.total_power_snapshot,
                    quorum_bps: config.quorum_bps,
                    pass_bps: config.pass_bps,
                    created_at: proposal.created_at,
                });
            let computed_status = computed_status(&proposal, env.block.time.seconds(), &snapshot);
            to_json_binary(&ProposalResponse {
                proposal,
                computed_status,
                snapshot,
            })
        }
        QueryMsg::Proposals { start_after, limit } => {
            let limit = limit
                .unwrap_or(DEFAULT_PAGE_LIMIT as u32)
                .min(MAX_PAGE_LIMIT as u32) as usize;
            let start = start_after.map(Bound::exclusive);
            let proposals = PROPOSALS
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| item.map(|(_, proposal)| proposal))
                .collect::<StdResult<Vec<_>>>()?;
            to_json_binary(&ProposalsResponse { proposals })
        }
        QueryMsg::Vote { proposal_id, voter } => {
            let voter = deps.api.addr_validate(&voter)?;
            to_json_binary(&VoteResponse {
                ballot: BALLOTS.may_load(deps.storage, (proposal_id, &voter))?,
            })
        }
        QueryMsg::Votes {
            proposal_id,
            start_after,
            limit,
        } => {
            let limit = limit
                .unwrap_or(DEFAULT_PAGE_LIMIT as u32)
                .min(MAX_PAGE_LIMIT as u32) as usize;
            let start_after = start_after
                .map(|value| deps.api.addr_validate(&value))
                .transpose()?;
            let start = start_after.as_ref().map(Bound::exclusive);
            let ballots = BALLOTS
                .prefix(proposal_id)
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| item.map(|(_, ballot)| ballot))
                .collect::<StdResult<Vec<_>>>()?;
            to_json_binary(&VotesResponse { ballots })
        }
        QueryMsg::RatifiedAdminAction { proposal_id } => {
            to_json_binary(&RatifiedAdminActionResponse {
                action: RATIFIED_ADMIN_ACTIONS.may_load(deps.storage, proposal_id)?,
            })
        }
    }
}

fn must_pay_native(info: &MessageInfo, denom: &str) -> Result<Uint128, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::MissingDeposit {});
    }
    let coin = info.funds.first().unwrap();
    if coin.denom != denom {
        return Err(ContractError::WrongDepositDenom {
            expected: denom.to_string(),
            actual: coin.denom.clone(),
        });
    }
    if info.funds.len() > 1 {
        return Err(ContractError::WrongDepositDenom {
            expected: denom.to_string(),
            actual: "multiple_denoms".to_string(),
        });
    }
    Ok(coin.amount)
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|inner| {
        let trimmed = inner.trim().to_string();
        (!trimmed.is_empty()).then_some(trimmed)
    })
}

fn validate_proposal_action(action: &ProposalAction) -> Result<(), ContractError> {
    match action {
        ProposalAction::SetPasgUtilityConfig { .. } => Ok(()),
        ProposalAction::StageAdminAction { action } => validate_admin_action(action),
    }
}

fn validate_admin_action(action: &AdminAction) -> Result<(), ContractError> {
    match action {
        AdminAction::StreamingBillingUpdateConfig {
            contract_addr,
            backend_operator,
            fiat_oracle,
            stripe_webhook_validator,
            paused,
        } => {
            validate_protocol_contract_addr(contract_addr)?;
            if backend_operator.is_none()
                && fiat_oracle.is_none()
                && stripe_webhook_validator.is_none()
                && paused.is_none()
            {
                return Err(ContractError::UnsupportedAdminAction {
                    reason: "streaming-billing update must change at least one field".to_string(),
                });
            }
            Ok(())
        }
        AdminAction::RegistryUpsertStakingValidator {
            contract_addr,
            operator_address,
            moniker,
            ..
        } => {
            validate_protocol_contract_addr(contract_addr)?;
            validate_staking_validator_metadata(operator_address, moniker)?;
            Ok(())
        }
        AdminAction::RegistryRemoveStakingValidator {
            contract_addr,
            operator_address,
        } => {
            validate_protocol_contract_addr(contract_addr)?;
            if operator_address.trim().is_empty() {
                return Err(ContractError::UnsupportedAdminAction {
                    reason: "registry staking validator removal requires an operator address"
                        .to_string(),
                });
            }
            Ok(())
        }
        AdminAction::MarketplaceV3UpdateConfig {
            contract_addr,
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            operators,
            paused,
            require_registration,
        } => {
            validate_protocol_contract_addr(contract_addr)?;
            if admin.is_none()
                && denom.is_none()
                && min_price.is_none()
                && trading_fee_bps.is_none()
                && max_trading_fee_bps.is_none()
                && fee_collector.is_none()
                && registry.is_none()
                && operators.is_none()
                && paused.is_none()
                && require_registration.is_none()
            {
                return Err(ContractError::UnsupportedAdminAction {
                    reason: "marketplace-v3 update must change at least one field".to_string(),
                });
            }
            Ok(())
        }
        AdminAction::AuctionEnglishUpdateConfig {
            contract_addr,
            admin,
            denom,
            min_price,
            trading_fee_bps,
            max_trading_fee_bps,
            fee_collector,
            registry,
            min_bid_increment_percent,
            min_duration,
            max_duration,
            extend_duration,
            paused,
            require_registration,
        } => {
            validate_protocol_contract_addr(contract_addr)?;
            if admin.is_none()
                && denom.is_none()
                && min_price.is_none()
                && trading_fee_bps.is_none()
                && max_trading_fee_bps.is_none()
                && fee_collector.is_none()
                && registry.is_none()
                && min_bid_increment_percent.is_none()
                && min_duration.is_none()
                && max_duration.is_none()
                && extend_duration.is_none()
                && paused.is_none()
                && require_registration.is_none()
            {
                return Err(ContractError::UnsupportedAdminAction {
                    reason: "auction-english update must change at least one field".to_string(),
                });
            }
            Ok(())
        }
    }
}

fn validate_protocol_contract_addr(contract_addr: &str) -> Result<(), ContractError> {
    let trimmed = contract_addr.trim();
    if trimmed.is_empty() {
        return Err(ContractError::ScopeViolation {
            reason: "contract_addr cannot be empty".to_string(),
        });
    }
    if trimmed.contains("://") || trimmed.contains("offchain") {
        return Err(ContractError::OffChainOperationForbidden {
            target: trimmed.to_string(),
        });
    }
    Ok(())
}

fn validate_staking_validator_metadata(
    operator_address: &str,
    moniker: &str,
) -> Result<(), ContractError> {
    if operator_address.trim().is_empty() {
        return Err(ContractError::UnsupportedAdminAction {
            reason: "registry staking validator update requires an operator address".to_string(),
        });
    }
    if moniker.trim().is_empty() {
        return Err(ContractError::UnsupportedAdminAction {
            reason: "registry staking validator update requires a moniker".to_string(),
        });
    }
    Ok(())
}

fn ensure_no_delegation_cycle(
    deps: Deps,
    delegator: &Addr,
    delegate: &Addr,
) -> Result<(), ContractError> {
    let mut current = delegate.clone();
    loop {
        if current == *delegator {
            return Err(ContractError::CycleInDelegation {});
        }
        let Some(next) = DELEGATIONS.may_load(deps.storage, &current)? else {
            return Ok(());
        };
        current = next.delegate;
    }
}

fn resolve_voting_addresses(deps: Deps, voter: &Addr) -> StdResult<Vec<Addr>> {
    let mut addresses = vec![voter.clone()];
    let incoming = delegated_addresses(deps, voter)?;
    for addr in incoming {
        if !addresses.iter().any(|existing| existing == &addr) {
            addresses.push(addr);
        }
    }
    Ok(addresses)
}

fn resolve_effective_power(deps: Deps, voter: &Addr) -> StdResult<Uint128> {
    if DELEGATIONS.may_load(deps.storage, voter)?.is_some() {
        return Ok(Uint128::zero());
    }

    let own = DEPOSITS.may_load(deps.storage, voter)?.unwrap_or_default();
    let incoming = delegated_power(deps, voter)?;
    Ok(own + incoming)
}

fn delegated_addresses(deps: Deps, delegate: &Addr) -> StdResult<Vec<Addr>> {
    DELEGATIONS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| match item {
            Ok((_, delegation)) if delegation.delegate == *delegate => {
                Some(Ok(delegation.delegator))
            }
            Ok(_) => None,
            Err(err) => Some(Err(err)),
        })
        .collect()
}

fn delegated_power(deps: Deps, delegate: &Addr) -> StdResult<Uint128> {
    let delegators = delegated_addresses(deps, delegate)?;
    let mut power = Uint128::zero();
    for delegator in delegators {
        power += DEPOSITS
            .may_load(deps.storage, &delegator)?
            .unwrap_or_default();
    }
    Ok(power)
}

fn snapshot_total_power(deps: Deps) -> StdResult<Uint128> {
    TOTAL_DEPOSITED.load(deps.storage)
}

fn proposal_meets_quorum(proposal: &Proposal, snapshot: &ProposalSnapshot) -> bool {
    let participation = proposal.yes_power + proposal.no_power;
    !participation.is_zero()
        && !snapshot.total_power.is_zero()
        && participation.u128() * BPS_SCALE
            >= snapshot.total_power.u128() * snapshot.quorum_bps as u128
}

fn proposal_meets_pass_threshold(proposal: &Proposal, snapshot: &ProposalSnapshot) -> bool {
    let participation = proposal.yes_power + proposal.no_power;
    !participation.is_zero()
        && proposal.yes_power.u128() * BPS_SCALE >= participation.u128() * snapshot.pass_bps as u128
}

fn lock_addresses_for_proposal(
    storage: &mut dyn cosmwasm_std::Storage,
    proposal_id: u64,
    addresses: &[Addr],
) -> StdResult<()> {
    for address in addresses {
        let amount = DEPOSITS.may_load(storage, address)?.unwrap_or_default();
        if amount.is_zero() {
            continue;
        }
        LOCKED_BALANCES.save(
            storage,
            (address, proposal_id),
            &LockedBalance {
                proposal_id,
                address: address.clone(),
                amount,
            },
        )?;
    }
    Ok(())
}

fn unlock_proposal_balances(
    storage: &mut dyn cosmwasm_std::Storage,
    proposal_id: u64,
) -> StdResult<()> {
    let keys = LOCKED_BALANCES
        .range(storage, None, None, Order::Ascending)
        .filter_map(|item| match item {
            Ok(((addr, locked_proposal_id), _)) if locked_proposal_id == proposal_id => {
                Some(Ok((addr, locked_proposal_id)))
            }
            Ok(_) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<StdResult<Vec<_>>>()?;

    for (addr, locked_proposal_id) in keys {
        LOCKED_BALANCES.remove(storage, (&addr, locked_proposal_id));
    }
    Ok(())
}

fn locked_balance(deps: Deps, address: &Addr) -> StdResult<Uint128> {
    let mut max_locked = Uint128::zero();
    for item in LOCKED_BALANCES
        .prefix(address)
        .range(deps.storage, None, None, Order::Ascending)
    {
        let (_, locked) = item?;
        if locked.amount > max_locked {
            max_locked = locked.amount;
        }
    }
    Ok(max_locked)
}

fn refresh_status(proposal: &mut Proposal, now: u64, snapshot: &ProposalSnapshot) {
    proposal.status = computed_status(proposal, now, snapshot);
}

fn computed_status(proposal: &Proposal, now: u64, snapshot: &ProposalSnapshot) -> ProposalStatus {
    if proposal.status == ProposalStatus::Executed {
        return ProposalStatus::Executed;
    }

    if proposal_meets_quorum(proposal, snapshot)
        && proposal_meets_pass_threshold(proposal, snapshot)
    {
        return ProposalStatus::Passed;
    }

    if now > proposal.expires_at {
        return ProposalStatus::Rejected;
    }

    ProposalStatus::Open
}

fn hash_admin_action(action: &AdminAction) -> StdResult<String> {
    let bytes = to_json_binary(action)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes.as_slice());
    Ok(format!("{:x}", hasher.finalize()))
}

#[allow(dead_code)]
fn decode_action(bytes: Binary) -> StdResult<AdminAction> {
    from_json(bytes)
}
