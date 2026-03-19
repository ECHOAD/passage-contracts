use crate::error::ContractError;
use crate::msg::{
    CanExecuteResponse, ConfigResponse, DelegationResponse, ExecuteMsg, ExecutionTargetsResponse,
    InstantiateMsg, ProposalListItem, ProposalResponse, ProposalsResponse, QueryMsg, VoteResponse,
    VotesResponse, VotingPowerResponse,
};
use crate::state::{
    Ballot, Config, Delegation, Proposal, ProposalAction, ProposalStatus, Vote,
    ALLOWED_EXECUTION_TARGETS, BALLOTS, BPS_SCALE, CONFIG, DELEGATIONS, OPEN_PROPOSAL_COUNT,
    PROPOSALS, PROPOSAL_COUNT, VOTING_BALANCES,
};
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

const CONTRACT_NAME: &str = "crates.io:multisig";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PAGE_LIMIT: usize = 30;
const MAX_PAGE_LIMIT: usize = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    validate_governance_config(
        msg.proposal_threshold,
        msg.quorum_bps,
        msg.approval_bps,
        msg.max_voting_period_secs,
    )?;

    let config = Config {
        pasg_denom: msg.pasg_denom,
        proposal_threshold: msg.proposal_threshold,
        quorum_bps: msg.quorum_bps,
        approval_bps: msg.approval_bps,
        max_voting_period_secs: msg.max_voting_period_secs,
    };

    CONFIG.save(deps.storage, &config)?;
    PROPOSAL_COUNT.save(deps.storage, &0)?;
    OPEN_PROPOSAL_COUNT.save(deps.storage, &0)?;

    for contract in msg.allowed_execute_contracts {
        let addr = deps.api.addr_validate(&contract)?;
        if ALLOWED_EXECUTION_TARGETS.has(deps.storage, &addr) {
            return Err(ContractError::DuplicateExecutionTarget { address: contract });
        }
        ALLOWED_EXECUTION_TARGETS.save(deps.storage, &addr, &true)?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("pasg_denom", config.pasg_denom)
        .add_attribute("proposal_threshold", config.proposal_threshold)
        .add_attribute("quorum_bps", config.quorum_bps.to_string())
        .add_attribute("approval_bps", config.approval_bps.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DepositVotingPower {} => execute_deposit_voting_power(deps, info),
        ExecuteMsg::WithdrawVotingPower { amount } => {
            execute_withdraw_voting_power(deps, info, amount)
        }
        ExecuteMsg::DelegateVotingPower { delegate } => {
            execute_delegate_voting_power(deps, env, info, delegate)
        }
        ExecuteMsg::UndelegateVotingPower {} => execute_undelegate_voting_power(deps, info),
        ExecuteMsg::Propose {
            title,
            description,
            actions,
        } => execute_propose(deps, env, info, title, description, actions),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::Execute { proposal_id } => execute_execute(deps, env, proposal_id),
        ExecuteMsg::Close { proposal_id } => execute_close(deps, env, proposal_id),
    }
}

fn execute_deposit_voting_power(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    ensure_governance_unlocked(deps.as_ref())?;
    let config = CONFIG.load(deps.storage)?;
    let amount = extract_native_amount(&info.funds, &config.pasg_denom)?;

    let new_balance =
        VOTING_BALANCES.update(deps.storage, &info.sender, |current| -> StdResult<_> {
            Ok(current.unwrap_or_default() + amount)
        })?;

    Ok(Response::new()
        .add_attribute("action", "deposit_voting_power")
        .add_attribute("voter", info.sender)
        .add_attribute("amount", amount)
        .add_attribute("balance", new_balance))
}

fn execute_withdraw_voting_power(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    ensure_governance_unlocked(deps.as_ref())?;
    let config = CONFIG.load(deps.storage)?;
    let balance = VOTING_BALANCES
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    if amount.is_zero() || amount > balance {
        return Err(ContractError::WithdrawAmountExceedsBalance {
            requested: amount,
            available: balance,
        });
    }

    let new_balance = balance - amount;
    if new_balance.is_zero() {
        VOTING_BALANCES.remove(deps.storage, &info.sender);
    } else {
        VOTING_BALANCES.save(deps.storage, &info.sender, &new_balance)?;
    }

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin::new(amount.u128(), config.pasg_denom)],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "withdraw_voting_power")
        .add_attribute("voter", info.sender)
        .add_attribute("amount", amount)
        .add_attribute("remaining_balance", new_balance))
}

fn execute_delegate_voting_power(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    delegate: String,
) -> Result<Response, ContractError> {
    ensure_governance_unlocked(deps.as_ref())?;
    let delegate_addr = deps.api.addr_validate(&delegate)?;
    if delegate_addr == info.sender {
        return Err(ContractError::CannotDelegateToSelf {});
    }
    if DELEGATIONS.has(deps.storage, &info.sender) {
        return Err(ContractError::AlreadyDelegated {});
    }
    if has_incoming_delegations(deps.as_ref(), &info.sender)? {
        return Err(ContractError::CannotDelegateWithIncomingPower {});
    }
    if DELEGATIONS.has(deps.storage, &delegate_addr) {
        return Err(ContractError::DelegationCycle {});
    }

    let amount = VOTING_BALANCES
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    if amount.is_zero() {
        return Err(ContractError::NoVotingPower {});
    }

    DELEGATIONS.save(deps.storage, &info.sender, &delegate_addr)?;

    Ok(Response::new()
        .add_attribute("action", "delegate_voting_power")
        .add_attribute("delegator", info.sender)
        .add_attribute("delegate", delegate_addr)
        .add_attribute("amount", amount)
        .add_attribute("updated_at", env.block.time.seconds().to_string()))
}

fn execute_undelegate_voting_power(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    ensure_governance_unlocked(deps.as_ref())?;
    let delegate = DELEGATIONS
        .may_load(deps.storage, &info.sender)?
        .ok_or(ContractError::NoDelegation {})?;
    DELEGATIONS.remove(deps.storage, &info.sender);

    Ok(Response::new()
        .add_attribute("action", "undelegate_voting_power")
        .add_attribute("delegator", info.sender)
        .add_attribute("delegate", delegate))
}

fn execute_propose(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: Option<String>,
    actions: Vec<ProposalAction>,
) -> Result<Response, ContractError> {
    if title.trim().is_empty() {
        return Err(ContractError::EmptyTitle {});
    }
    if actions.is_empty() {
        return Err(ContractError::EmptyProposalActions {});
    }

    let config = CONFIG.load(deps.storage)?;
    let proposer_power = effective_voting_power(deps.as_ref(), &info.sender)?;
    if proposer_power < config.proposal_threshold {
        return Err(ContractError::InsufficientProposalPower {});
    }

    for action in &actions {
        validate_proposal_action(deps.as_ref(), action)?;
    }

    let proposal_id =
        PROPOSAL_COUNT.update(deps.storage, |current| -> StdResult<_> { Ok(current + 1) })?;
    let now = env.block.time.seconds();
    let total_power_snapshot = total_voting_power(deps.as_ref())?;
    let proposal = Proposal {
        id: proposal_id,
        title: title.trim().to_string(),
        description: description.and_then(normalize_optional_text),
        proposer: info.sender.clone(),
        actions,
        status: ProposalStatus::Open,
        yes_power: Uint128::zero(),
        no_power: Uint128::zero(),
        total_power_snapshot,
        quorum_bps: config.quorum_bps,
        approval_bps: config.approval_bps,
        expires_at: now + config.max_voting_period_secs,
        created_at: now,
        executed_at: None,
    };
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    OPEN_PROPOSAL_COUNT.update(deps.storage, |count| -> StdResult<_> { Ok(count + 1) })?;

    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("proposer", info.sender)
        .add_attribute("total_power_snapshot", total_power_snapshot)
        .add_attribute("status", "Open"))
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

    let voting_power = effective_voting_power(deps.as_ref(), &info.sender)?;
    if voting_power.is_zero() {
        return Err(ContractError::NoVotingPower {});
    }

    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;
    let previous_status = computed_status(&proposal, now);
    if previous_status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if previous_status == ProposalStatus::Rejected {
        if now > proposal.expires_at {
            return Err(ContractError::ProposalExpired {});
        }
        return Err(ContractError::ProposalNotOpen {});
    }
    if previous_status != ProposalStatus::Open {
        return Err(ContractError::ProposalNotOpen {});
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

    let new_status = computed_status(&proposal, now);
    proposal.status = new_status.clone();
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    sync_open_proposal_counter(deps.storage, previous_status, new_status.clone())?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", info.sender)
        .add_attribute("vote", format!("{:?}", vote))
        .add_attribute("weight", voting_power)
        .add_attribute("status", format!("{:?}", new_status)))
}

fn execute_execute(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;

    let computed = computed_status(&proposal, now);
    if computed == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if computed != ProposalStatus::Passed {
        return Err(ContractError::ProposalNotPassed {});
    }

    let mut messages: Vec<CosmosMsg> = Vec::new();
    for action in &proposal.actions {
        match action {
            ProposalAction::UpdateGovernanceConfig {
                proposal_threshold,
                quorum_bps,
                approval_bps,
                max_voting_period_secs,
            } => {
                let mut config = CONFIG.load(deps.storage)?;
                if let Some(value) = proposal_threshold {
                    config.proposal_threshold = *value;
                }
                if let Some(value) = quorum_bps {
                    config.quorum_bps = *value;
                }
                if let Some(value) = approval_bps {
                    config.approval_bps = *value;
                }
                if let Some(value) = max_voting_period_secs {
                    config.max_voting_period_secs = *value;
                }
                validate_governance_config(
                    config.proposal_threshold,
                    config.quorum_bps,
                    config.approval_bps,
                    config.max_voting_period_secs,
                )?;
                CONFIG.save(deps.storage, &config)?;
            }
            ProposalAction::UpdateExecutionTargets { add, remove } => {
                for address in add {
                    let addr = deps.api.addr_validate(address)?;
                    ALLOWED_EXECUTION_TARGETS.save(deps.storage, &addr, &true)?;
                }
                for address in remove {
                    let addr = deps.api.addr_validate(address)?;
                    ALLOWED_EXECUTION_TARGETS.remove(deps.storage, &addr);
                }
            }
            ProposalAction::WasmExecute { contract_addr, msg } => {
                let addr = deps.api.addr_validate(contract_addr)?;
                if !ALLOWED_EXECUTION_TARGETS.has(deps.storage, &addr) {
                    return Err(ContractError::TargetNotAllowed {
                        contract_addr: contract_addr.clone(),
                    });
                }
                messages.push(
                    WasmMsg::Execute {
                        contract_addr: addr.to_string(),
                        msg: msg.clone(),
                        funds: vec![],
                    }
                    .into(),
                );
            }
        }
    }

    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(now);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "execute")
        .add_attribute("proposal_id", proposal_id.to_string()))
}

fn execute_close(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;

    let previous_status = computed_status(&proposal, now);
    if previous_status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if previous_status == ProposalStatus::Passed {
        return Err(ContractError::ProposalNotPassed {});
    }
    if previous_status == ProposalStatus::Open {
        return Err(ContractError::ProposalNotOpen {});
    }

    proposal.status = ProposalStatus::Rejected;
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    sync_open_proposal_counter(deps.storage, previous_status, ProposalStatus::Rejected)?;

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("proposal_id", proposal_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::VotingPower { address } => to_json_binary(&query_voting_power(deps, address)?),
        QueryMsg::Delegation { address } => to_json_binary(&query_delegation(deps, address)?),
        QueryMsg::Proposal { proposal_id } => {
            to_json_binary(&query_proposal(deps, env, proposal_id)?)
        }
        QueryMsg::Proposals { start_after, limit } => {
            to_json_binary(&query_proposals(deps, env, start_after, limit)?)
        }
        QueryMsg::Vote { proposal_id, voter } => {
            to_json_binary(&query_vote(deps, proposal_id, voter)?)
        }
        QueryMsg::Votes {
            proposal_id,
            start_after,
            limit,
        } => to_json_binary(&query_votes(deps, proposal_id, start_after, limit)?),
        QueryMsg::CanExecute { proposal_id } => {
            to_json_binary(&query_can_execute(deps, env, proposal_id)?)
        }
        QueryMsg::ExecutionTargets { start_after, limit } => {
            to_json_binary(&query_execution_targets(deps, start_after, limit)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

fn query_voting_power(deps: Deps, address: String) -> StdResult<VotingPowerResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let deposited = VOTING_BALANCES
        .may_load(deps.storage, &addr)?
        .unwrap_or_default();
    let delegated_to = DELEGATIONS.may_load(deps.storage, &addr)?;
    let incoming = incoming_delegated_power(deps, &addr)?;
    let effective = effective_voting_power(deps, &addr)?;

    Ok(VotingPowerResponse {
        address,
        deposited,
        delegated_to: delegated_to.map(|addr| addr.to_string()),
        incoming_delegated_power: incoming,
        effective_voting_power: effective,
    })
}

fn query_delegation(deps: Deps, address: String) -> StdResult<DelegationResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let delegate = DELEGATIONS.may_load(deps.storage, &addr)?;
    let delegation = delegate.map(|delegate| Delegation {
        delegator: addr.clone(),
        amount: VOTING_BALANCES
            .may_load(deps.storage, &addr)
            .unwrap_or_default()
            .unwrap_or_default(),
        delegate,
        created_at: 0,
    });

    Ok(DelegationResponse { delegation })
}

fn query_proposal(deps: Deps, env: Env, proposal_id: u64) -> StdResult<ProposalResponse> {
    let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    let computed_status = computed_status(&proposal, env.block.time.seconds());
    Ok(ProposalResponse {
        proposal,
        computed_status,
    })
}

fn query_proposals(
    deps: Deps,
    env: Env,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ProposalsResponse> {
    let limit = limit
        .unwrap_or(DEFAULT_PAGE_LIMIT as u32)
        .min(MAX_PAGE_LIMIT as u32) as usize;
    let start = start_after.map(Bound::exclusive);
    let now = env.block.time.seconds();

    let proposals = PROPOSALS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            item.map(|(_, proposal)| ProposalListItem {
                computed_status: computed_status(&proposal, now),
                proposal,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(ProposalsResponse { proposals })
}

fn query_vote(deps: Deps, proposal_id: u64, voter: String) -> StdResult<VoteResponse> {
    let voter_addr = deps.api.addr_validate(&voter)?;
    Ok(VoteResponse {
        ballot: BALLOTS.may_load(deps.storage, (proposal_id, &voter_addr))?,
    })
}

fn query_votes(
    deps: Deps,
    proposal_id: u64,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<VotesResponse> {
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

    Ok(VotesResponse { ballots })
}

fn query_can_execute(deps: Deps, env: Env, proposal_id: u64) -> StdResult<CanExecuteResponse> {
    let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    Ok(CanExecuteResponse {
        proposal_id,
        executable: computed_status(&proposal, env.block.time.seconds()) == ProposalStatus::Passed,
    })
}

fn query_execution_targets(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ExecutionTargetsResponse> {
    let limit = limit
        .unwrap_or(DEFAULT_PAGE_LIMIT as u32)
        .min(MAX_PAGE_LIMIT as u32) as usize;
    let start_after = start_after
        .map(|value| deps.api.addr_validate(&value))
        .transpose()?;
    let start = start_after.as_ref().map(Bound::exclusive);

    let targets = ALLOWED_EXECUTION_TARGETS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(addr, _)| addr.to_string()))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(ExecutionTargetsResponse { targets })
}

fn extract_native_amount(funds: &[Coin], denom: &str) -> Result<Uint128, ContractError> {
    let amount = funds
        .iter()
        .find(|coin| coin.denom == denom)
        .map(|coin| coin.amount)
        .unwrap_or_default();
    if amount.is_zero() {
        return Err(ContractError::NoPasgFunds {});
    }
    if funds
        .iter()
        .any(|coin| coin.denom != denom && !coin.amount.is_zero())
    {
        return Err(ContractError::InvalidPasgPayment {
            denom: denom.to_string(),
        });
    }
    Ok(amount)
}

fn validate_governance_config(
    proposal_threshold: Uint128,
    quorum_bps: u64,
    approval_bps: u64,
    max_voting_period_secs: u64,
) -> Result<(), ContractError> {
    if proposal_threshold.is_zero() {
        return Err(ContractError::InvalidProposalThreshold {});
    }
    if max_voting_period_secs == 0 {
        return Err(ContractError::InvalidVotingPeriod {});
    }
    if quorum_bps == 0 || quorum_bps > BPS_SCALE {
        return Err(ContractError::InvalidQuorumBps {});
    }
    if approval_bps == 0 || approval_bps > BPS_SCALE {
        return Err(ContractError::InvalidApprovalBps {});
    }
    Ok(())
}

fn validate_proposal_action(deps: Deps, action: &ProposalAction) -> Result<(), ContractError> {
    match action {
        ProposalAction::UpdateGovernanceConfig {
            proposal_threshold,
            quorum_bps,
            approval_bps,
            max_voting_period_secs,
        } => {
            let current = CONFIG.load(deps.storage)?;
            validate_governance_config(
                proposal_threshold.unwrap_or(current.proposal_threshold),
                quorum_bps.unwrap_or(current.quorum_bps),
                approval_bps.unwrap_or(current.approval_bps),
                max_voting_period_secs.unwrap_or(current.max_voting_period_secs),
            )
        }
        ProposalAction::UpdateExecutionTargets { add, .. } => {
            for address in add {
                deps.api.addr_validate(address)?;
            }
            Ok(())
        }
        ProposalAction::WasmExecute { contract_addr, .. } => {
            let addr = deps.api.addr_validate(contract_addr)?;
            if !ALLOWED_EXECUTION_TARGETS.has(deps.storage, &addr) {
                return Err(ContractError::TargetNotAllowed {
                    contract_addr: contract_addr.clone(),
                });
            }
            Ok(())
        }
    }
}

fn effective_voting_power(deps: Deps, address: &Addr) -> StdResult<Uint128> {
    let own_balance = VOTING_BALANCES
        .may_load(deps.storage, address)?
        .unwrap_or_default();
    let delegated_out = DELEGATIONS.has(deps.storage, address);
    let incoming = incoming_delegated_power(deps, address)?;

    if delegated_out {
        Ok(Uint128::zero())
    } else {
        Ok(own_balance + incoming)
    }
}

fn incoming_delegated_power(deps: Deps, address: &Addr) -> StdResult<Uint128> {
    let mut total = Uint128::zero();
    let delegations = DELEGATIONS.range(deps.storage, None, None, Order::Ascending);
    for item in delegations {
        let (delegator, delegate) = item?;
        if delegate == *address {
            total += VOTING_BALANCES
                .may_load(deps.storage, &delegator)?
                .unwrap_or_default();
        }
    }
    Ok(total)
}

fn has_incoming_delegations(deps: Deps, address: &Addr) -> StdResult<bool> {
    Ok(!incoming_delegated_power(deps, address)?.is_zero())
}

fn total_voting_power(deps: Deps) -> StdResult<Uint128> {
    let mut total = Uint128::zero();
    for item in VOTING_BALANCES.range(deps.storage, None, None, Order::Ascending) {
        let (_, balance) = item?;
        total += balance;
    }
    Ok(total)
}

fn ensure_governance_unlocked(deps: Deps) -> Result<(), ContractError> {
    if OPEN_PROPOSAL_COUNT.load(deps.storage)? > 0 {
        return Err(ContractError::GovernancePowerLocked {});
    }
    Ok(())
}

fn computed_status(proposal: &Proposal, now: u64) -> ProposalStatus {
    if proposal.status == ProposalStatus::Executed {
        return ProposalStatus::Executed;
    }

    let participation = proposal.yes_power + proposal.no_power;
    if meets_quorum(
        proposal.total_power_snapshot,
        participation,
        proposal.quorum_bps,
    ) && meets_approval(participation, proposal.yes_power, proposal.approval_bps)
    {
        return ProposalStatus::Passed;
    }

    if now > proposal.expires_at {
        return ProposalStatus::Rejected;
    }

    ProposalStatus::Open
}

fn meets_quorum(total_power: Uint128, participation: Uint128, quorum_bps: u64) -> bool {
    participation.u128() * BPS_SCALE as u128 >= total_power.u128() * quorum_bps as u128
}

fn meets_approval(participation: Uint128, yes_power: Uint128, approval_bps: u64) -> bool {
    !participation.is_zero()
        && yes_power.u128() * BPS_SCALE as u128 >= participation.u128() * approval_bps as u128
}

fn sync_open_proposal_counter(
    storage: &mut dyn cosmwasm_std::Storage,
    previous_status: ProposalStatus,
    new_status: ProposalStatus,
) -> StdResult<()> {
    let was_open = previous_status == ProposalStatus::Open;
    let is_open = new_status == ProposalStatus::Open;
    if was_open && !is_open {
        OPEN_PROPOSAL_COUNT.update(storage, |count| -> StdResult<_> {
            Ok(count.saturating_sub(1))
        })?;
    }
    Ok(())
}

fn normalize_optional_text(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}
