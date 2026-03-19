use crate::error::ContractError;
use crate::msg::{
    ConfigResponse, DepositResponse, ExecuteMsg, InstantiateMsg, PasgUtilityConfigResponse,
    ProposalAction, ProposalResponse, ProposalsResponse, QueryMsg, RatifiedAdminActionResponse,
    VoteResponse, VotesResponse,
};
use crate::state::{
    AdminAction, Ballot, Config, PasgUtilityConfig, Proposal, ProposalStatus, RatifiedAdminAction,
    Vote, BALLOTS, CONFIG, DEPOSITS, PASG_UTILITY_CONFIG, PROPOSALS, PROPOSAL_COUNT,
    RATIFIED_ADMIN_ACTIONS, TOTAL_DEPOSITED,
};
use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Uint128,
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
    if msg.quorum_bps == 0 || msg.quorum_bps > BPS_SCALE as u64 || msg.pass_bps == 0 || msg.pass_bps > BPS_SCALE as u64 {
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
        ExecuteMsg::Propose {
            title,
            description,
            action,
        } => execute_propose(deps, env, info, title, description, action),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::ExecuteProposal { proposal_id } => execute_execute_proposal(deps, env, proposal_id),
        ExecuteMsg::Close { proposal_id } => execute_close(deps, env, proposal_id),
    }
}

fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let amount = must_pay_native(&info, &config.native_denom)?;

    DEPOSITS.update(deps.storage, &info.sender, |current| -> StdResult<_> {
        Ok(current.unwrap_or_default() + amount)
    })?;
    TOTAL_DEPOSITED.update(deps.storage, |current| -> StdResult<_> { Ok(current + amount) })?;

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
    let deposited = DEPOSITS.may_load(deps.storage, &info.sender)?.unwrap_or_default();
    if deposited < amount || amount.is_zero() {
        return Err(ContractError::InsufficientVotingPower {
            required: amount,
            actual: deposited,
        });
    }

    let remaining = deposited.checked_sub(amount).unwrap();
    DEPOSITS.save(deps.storage, &info.sender, &remaining)?;
    TOTAL_DEPOSITED.update(deps.storage, |current| -> StdResult<_> {
        Ok(current.checked_sub(amount).unwrap())
    })?;

    Ok(Response::new()
        .add_message(cosmwasm_std::BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin::new(amount.u128(), config.native_denom)],
        })
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", amount))
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

    let config = CONFIG.load(deps.storage)?;
    let deposited = DEPOSITS.may_load(deps.storage, &info.sender)?.unwrap_or_default();
    if deposited < config.proposal_deposit {
        return Err(ContractError::InsufficientVotingPower {
            required: config.proposal_deposit,
            actual: deposited,
        });
    }

    let proposal_id = PROPOSAL_COUNT.update(deps.storage, |current| -> StdResult<_> { Ok(current + 1) })?;
    let now = env.block.time.seconds();
    let total_power = TOTAL_DEPOSITED.load(deps.storage)?;
    let mut proposal = Proposal {
        id: proposal_id,
        title: title.trim().to_string(),
        description: normalize_optional_text(description),
        proposer: info.sender.clone(),
        action,
        status: ProposalStatus::Open,
        yes_power: deposited,
        no_power: Uint128::zero(),
        total_power_snapshot: total_power,
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
            weight: deposited,
            voted_at: now,
        },
    )?;

    refresh_status(&mut proposal, now, config.quorum_bps, config.pass_bps);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("proposer", info.sender)
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

    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let config = CONFIG.load(deps.storage)?;
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, config.quorum_bps, config.pass_bps);
    if proposal.status != ProposalStatus::Open {
        return Err(if now > proposal.expires_at {
            ContractError::ProposalExpired {}
        } else {
            ContractError::ProposalNotOpen {}
        });
    }

    let deposited = DEPOSITS.may_load(deps.storage, &info.sender)?.unwrap_or_default();
    if deposited.is_zero() {
        return Err(ContractError::InsufficientVotingPower {
            required: Uint128::new(1),
            actual: deposited,
        });
    }

    match vote {
        Vote::Approve => proposal.yes_power += deposited,
        Vote::Reject => proposal.no_power += deposited,
    }

    BALLOTS.save(
        deps.storage,
        (proposal_id, &info.sender),
        &Ballot {
            voter: info.sender.clone(),
            vote: vote.clone(),
            weight: deposited,
            voted_at: now,
        },
    )?;

    refresh_status(&mut proposal, now, config.quorum_bps, config.pass_bps);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", info.sender)
        .add_attribute("vote", format!("{:?}", vote))
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
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, config.quorum_bps, config.pass_bps);
    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status != ProposalStatus::Passed {
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
                .add_attribute("admin_multisig", config.admin_multisig)
                .add_attribute("payload_hash", payload_hash);
        }
    }

    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(now);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(response)
}

fn execute_close(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let config = CONFIG.load(deps.storage)?;
    let now = env.block.time.seconds();
    refresh_status(&mut proposal, now, config.quorum_bps, config.pass_bps);

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
        QueryMsg::PasgUtilityConfig {} => to_json_binary(&PasgUtilityConfigResponse {
            config: PASG_UTILITY_CONFIG.load(deps.storage)?,
        }),
        QueryMsg::Proposal { proposal_id } => {
            let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
            let config = CONFIG.load(deps.storage)?;
            let computed_status = computed_status(
                &proposal,
                env.block.time.seconds(),
                config.quorum_bps,
                config.pass_bps,
            );
            to_json_binary(&ProposalResponse {
                proposal,
                computed_status,
            })
        }
        QueryMsg::Proposals { start_after, limit } => {
            let limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT as u32).min(MAX_PAGE_LIMIT as u32) as usize;
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
            let limit = limit.unwrap_or(DEFAULT_PAGE_LIMIT as u32).min(MAX_PAGE_LIMIT as u32) as usize;
            let start_after = start_after.map(|value| deps.api.addr_validate(&value)).transpose()?;
            let start = start_after.as_ref().map(Bound::exclusive);
            let ballots = BALLOTS
                .prefix(proposal_id)
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| item.map(|(_, ballot)| ballot))
                .collect::<StdResult<Vec<_>>>()?;
            to_json_binary(&VotesResponse { ballots })
        }
        QueryMsg::RatifiedAdminAction { proposal_id } => to_json_binary(&RatifiedAdminActionResponse {
            action: RATIFIED_ADMIN_ACTIONS.may_load(deps.storage, proposal_id)?,
        }),
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

fn refresh_status(proposal: &mut Proposal, now: u64, quorum_bps: u64, pass_bps: u64) {
    proposal.status = computed_status(proposal, now, quorum_bps, pass_bps);
}

fn computed_status(proposal: &Proposal, now: u64, quorum_bps: u64, pass_bps: u64) -> ProposalStatus {
    if proposal.status == ProposalStatus::Executed {
        return ProposalStatus::Executed;
    }

    let participation = proposal.yes_power + proposal.no_power;
    if thresholds_met(
        proposal.yes_power,
        participation,
        proposal.total_power_snapshot,
        quorum_bps,
        pass_bps,
    ) {
        return ProposalStatus::Passed;
    }

    if now > proposal.expires_at {
        return ProposalStatus::Rejected;
    }

    ProposalStatus::Open
}

fn thresholds_met(
    yes_power: Uint128,
    participation: Uint128,
    total_power_snapshot: Uint128,
    quorum_bps: u64,
    pass_bps: u64,
) -> bool {
    if participation.is_zero() || total_power_snapshot.is_zero() {
        return false;
    }

    let quorum_ok = participation.u128() * BPS_SCALE >= total_power_snapshot.u128() * quorum_bps as u128;
    let pass_ok = yes_power.u128() * BPS_SCALE >= participation.u128() * pass_bps as u128;
    quorum_ok && pass_ok
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

