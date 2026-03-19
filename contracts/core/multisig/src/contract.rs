use crate::error::ContractError;
use crate::msg::{
    CanExecuteResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, MemberResponse,
    MembersResponse, ProposalListItem, ProposalResponse, ProposalsResponse, QueryMsg, VoteResponse,
    VotesResponse,
};
use crate::state::{
    Ballot, Config, Member, Proposal, ProposalStatus, Vote, BALLOTS, CONFIG, MEMBERS, PROPOSALS,
    PROPOSAL_COUNT,
};
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Order,
    Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use std::collections::HashSet;

const CONTRACT_NAME: &str = "crates.io:multisig";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PAGE_LIMIT: usize = 30;
const MAX_PAGE_LIMIT: usize = 100;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let members = validate_members(deps.as_ref(), &msg.members)?;
    validate_threshold(msg.threshold, members.len() as u64)?;
    if msg.max_voting_period_secs == 0 {
        return Err(ContractError::InvalidVotingPeriod {});
    }

    let config = Config {
        threshold: msg.threshold,
        total_members: members.len() as u64,
        max_voting_period_secs: msg.max_voting_period_secs,
    };

    CONFIG.save(deps.storage, &config)?;
    PROPOSAL_COUNT.save(deps.storage, &0)?;

    let now = env.block.time.seconds();
    for member in members {
        MEMBERS.save(
            deps.storage,
            &member,
            &Member {
                addr: member.clone(),
                added_at: now,
            },
        )?;
    }

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("threshold", config.threshold.to_string())
        .add_attribute("total_members", config.total_members.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Propose {
            title,
            description,
            msgs,
        } => execute_propose(deps, env, info, title, description, msgs),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::Execute { proposal_id } => execute_execute(deps, env, proposal_id),
        ExecuteMsg::Close { proposal_id } => execute_close(deps, env, proposal_id),
        ExecuteMsg::UpdateMembers {
            members,
            threshold,
            max_voting_period_secs,
        } => execute_update_members(deps, env, info, members, threshold, max_voting_period_secs),
    }
}

fn execute_propose(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: Option<String>,
    msgs: Vec<cosmwasm_std::CosmosMsg<Empty>>,
) -> Result<Response, ContractError> {
    require_member(deps.as_ref(), &info.sender)?;
    if title.trim().is_empty() {
        return Err(ContractError::EmptyTitle {});
    }
    if msgs.is_empty() {
        return Err(ContractError::EmptyProposalMsgs {});
    }

    let config = CONFIG.load(deps.storage)?;
    let proposal_id =
        PROPOSAL_COUNT.update(deps.storage, |current| -> StdResult<_> { Ok(current + 1) })?;
    let now = env.block.time.seconds();

    let mut proposal = Proposal {
        id: proposal_id,
        title: title.trim().to_string(),
        description: description.and_then(normalize_optional_text),
        proposer: info.sender.clone(),
        msgs,
        status: ProposalStatus::Open,
        yes_votes: 1,
        no_votes: 0,
        threshold: config.threshold,
        total_members: config.total_members,
        expires_at: now + config.max_voting_period_secs,
        created_at: now,
        executed_at: None,
    };

    BALLOTS.save(
        deps.storage,
        (proposal_id, &info.sender),
        &Ballot {
            voter: info.sender.clone(),
            vote: Vote::Approve,
            voted_at: now,
        },
    )?;

    refresh_status(&mut proposal, now);
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
    require_member(deps.as_ref(), &info.sender)?;
    if BALLOTS.has(deps.storage, (proposal_id, &info.sender)) {
        return Err(ContractError::AlreadyVoted {});
    }

    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;

    refresh_status(&mut proposal, now);
    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status == ProposalStatus::Rejected {
        if now > proposal.expires_at {
            return Err(ContractError::ProposalExpired {});
        }
        return Err(ContractError::ProposalNotOpen {});
    }
    if proposal.status != ProposalStatus::Open {
        return Err(ContractError::ProposalNotOpen {});
    }

    match vote {
        Vote::Approve => proposal.yes_votes += 1,
        Vote::Reject => proposal.no_votes += 1,
    }

    BALLOTS.save(
        deps.storage,
        (proposal_id, &info.sender),
        &Ballot {
            voter: info.sender.clone(),
            vote: vote.clone(),
            voted_at: now,
        },
    )?;

    refresh_status(&mut proposal, now);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", info.sender)
        .add_attribute("vote", format!("{:?}", vote))
        .add_attribute("status", format!("{:?}", proposal.status)))
}

fn execute_execute(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;

    refresh_status(&mut proposal, now);
    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status != ProposalStatus::Passed {
        return Err(ContractError::ProposalNotPassed {});
    }

    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(now);
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_messages(proposal.msgs)
        .add_attribute("action", "execute")
        .add_attribute("proposal_id", proposal_id.to_string()))
}

fn execute_close(deps: DepsMut, env: Env, proposal_id: u64) -> Result<Response, ContractError> {
    let now = env.block.time.seconds();
    let mut proposal = PROPOSALS
        .may_load(deps.storage, proposal_id)?
        .ok_or(ContractError::ProposalNotFound { proposal_id })?;

    refresh_status(&mut proposal, now);
    if proposal.status == ProposalStatus::Executed {
        return Err(ContractError::ProposalAlreadyExecuted {});
    }
    if proposal.status == ProposalStatus::Passed {
        return Err(ContractError::ProposalNotPassed {});
    }
    if proposal.status == ProposalStatus::Open {
        return Err(ContractError::ProposalNotOpen {});
    }

    proposal.status = ProposalStatus::Rejected;
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("proposal_id", proposal_id.to_string()))
}

fn execute_update_members(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    members: Vec<String>,
    threshold: u64,
    max_voting_period_secs: Option<u64>,
) -> Result<Response, ContractError> {
    if info.sender != env.contract.address {
        return Err(ContractError::Unauthorized {});
    }

    let members = validate_members(deps.as_ref(), &members)?;
    validate_threshold(threshold, members.len() as u64)?;
    if let Some(value) = max_voting_period_secs {
        if value == 0 {
            return Err(ContractError::InvalidVotingPeriod {});
        }
    }

    let mut config = CONFIG.load(deps.storage)?;
    config.threshold = threshold;
    config.total_members = members.len() as u64;
    if let Some(value) = max_voting_period_secs {
        config.max_voting_period_secs = value;
    }

    let existing_members = MEMBERS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    for (addr, _) in existing_members {
        MEMBERS.remove(deps.storage, &addr);
    }

    let now = env.block.time.seconds();
    for member in members {
        MEMBERS.save(
            deps.storage,
            &member,
            &Member {
                addr: member.clone(),
                added_at: now,
            },
        )?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_members")
        .add_attribute("threshold", config.threshold.to_string())
        .add_attribute("total_members", config.total_members.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::Member { address } => to_json_binary(&query_member(deps, address)?),
        QueryMsg::Members { start_after, limit } => {
            to_json_binary(&query_members(deps, start_after, limit)?)
        }
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
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

fn query_member(deps: Deps, address: String) -> StdResult<MemberResponse> {
    let addr = deps.api.addr_validate(&address)?;
    Ok(MemberResponse {
        member: MEMBERS.may_load(deps.storage, &addr)?,
    })
}

fn query_members(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<MembersResponse> {
    let limit = limit
        .unwrap_or(DEFAULT_PAGE_LIMIT as u32)
        .min(MAX_PAGE_LIMIT as u32) as usize;
    let start_after = start_after
        .map(|value| deps.api.addr_validate(&value))
        .transpose()?;
    let start = start_after.as_ref().map(Bound::exclusive);

    let members = MEMBERS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, member)| member))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(MembersResponse { members })
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

fn computed_status(proposal: &Proposal, now: u64) -> ProposalStatus {
    if proposal.status == ProposalStatus::Executed {
        return ProposalStatus::Executed;
    }
    if proposal.yes_votes >= proposal.threshold {
        return ProposalStatus::Passed;
    }

    let possible_remaining = proposal
        .total_members
        .saturating_sub(proposal.yes_votes + proposal.no_votes);
    if proposal.yes_votes + possible_remaining < proposal.threshold {
        return ProposalStatus::Rejected;
    }
    if now > proposal.expires_at {
        return ProposalStatus::Rejected;
    }

    ProposalStatus::Open
}

fn refresh_status(proposal: &mut Proposal, now: u64) {
    proposal.status = computed_status(proposal, now);
}

fn require_member(deps: Deps, address: &Addr) -> Result<(), ContractError> {
    if !MEMBERS.has(deps.storage, address) {
        return Err(ContractError::NotMember {});
    }
    Ok(())
}

fn validate_members(deps: Deps, members: &[String]) -> Result<Vec<Addr>, ContractError> {
    if members.is_empty() {
        return Err(ContractError::EmptyMembers {});
    }

    let mut seen = HashSet::new();
    let mut validated = Vec::with_capacity(members.len());
    for member in members {
        let addr = deps.api.addr_validate(member)?;
        if !seen.insert(addr.clone()) {
            return Err(ContractError::DuplicateMember {
                address: member.clone(),
            });
        }
        validated.push(addr);
    }

    Ok(validated)
}

fn validate_threshold(threshold: u64, member_count: u64) -> Result<(), ContractError> {
    if threshold == 0 {
        return Err(ContractError::InvalidThreshold {});
    }
    if threshold > member_count {
        return Err(ContractError::ThresholdTooHigh {});
    }
    Ok(())
}

fn normalize_optional_text(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}
