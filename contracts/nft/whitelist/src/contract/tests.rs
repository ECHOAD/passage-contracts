use super::*;
use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info},
    Attribute,
};

const ADMIN: &str = "admin";
const NATIVE_DENOM: &str = "ujuno";
const UNIT_AMOUNT: u128 = 100_000_000;

const START_TIME: Timestamp = Timestamp::from_nanos(1647032400000000000);
const END_TIME: Timestamp = START_TIME.plus_seconds(1);

fn setup_contract(deps: DepsMut) {
    let msg = InstantiateMsg {
        members: vec!["adsfsa".to_string()],
        start_time: START_TIME,
        end_time: END_TIME,
        unit_price: coin(UNIT_AMOUNT, NATIVE_DENOM),
        per_address_limit: 1,
        member_limit: 1000,
    };
    let info = mock_info(ADMIN, &[coin(100_000_000, "ujuno")]);
    let res = instantiate(deps, mock_env(), info.clone(), msg).unwrap();
    assert!(res.attributes[0].eq(&Attribute::new("action", "instantiate")));
    assert!(res.attributes[1].eq(&Attribute::new("contract_name", CONTRACT_NAME)));
    assert!(res.attributes[2].eq(&Attribute::new("contract_version", CONTRACT_VERSION)));
    assert!(res.attributes[3].eq(&Attribute::new("sender", info.sender.into_string())));
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());
}

#[test]
fn improper_initialization() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        members: vec!["adsfsa".to_string()],
        start_time: END_TIME,
        end_time: END_TIME,
        unit_price: coin(1, NATIVE_DENOM),
        per_address_limit: 1,
        member_limit: 1000,
    };
    let info = mock_info(ADMIN, &[coin(100_000_000, "ujuno")]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
}

#[test]
fn improper_initialization_dedup() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        members: vec![
            "adsfsa".to_string(),
            "adsfsa".to_string(),
            "adsfsa".to_string(),
        ],
        start_time: START_TIME,
        end_time: END_TIME,
        unit_price: coin(UNIT_AMOUNT, NATIVE_DENOM),
        per_address_limit: 1,
        member_limit: 1000,
    };
    let info = mock_info(ADMIN, &[coin(100_000_000, "ujuno")]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    let res = query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(1, res.num_members);
}

#[test]
fn check_start_time_after_end_time() {
    let msg = InstantiateMsg {
        members: vec!["adsfsa".to_string()],
        start_time: END_TIME,
        end_time: START_TIME,
        unit_price: coin(UNIT_AMOUNT, NATIVE_DENOM),
        per_address_limit: 1,
        member_limit: 1000,
    };
    let info = mock_info(ADMIN, &[coin(100_000_000, "ujuno")]);
    let mut deps = mock_dependencies();
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
}

#[test]
fn update_start_time() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());

    let new_start_time = START_TIME.minus_nanos(100);
    let msg = ExecuteMsg::UpdateStartTime(new_start_time);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.attributes.len(), 3);
    let res = query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(res.start_time, new_start_time);
}

#[test]
fn update_end_time() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());

    let new_end_time = START_TIME.plus_nanos(300);
    let msg = ExecuteMsg::UpdateEndTime(new_end_time);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.attributes.len(), 3);
    let res = query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(res.end_time, new_end_time);
}

#[test]
fn update_members() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());

    // dedupe addrs
    let add_msg = AddMembersMsg {
        to_add: vec!["adsfsa1".to_string(), "adsfsa1".to_string()],
    };
    let msg = ExecuteMsg::AddMembers(add_msg);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes.len(), 2);
    let res = query_members(deps.as_ref(), None, None).unwrap();
    assert_eq!(res.members.len(), 2);

    execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();

    let remove_msg = RemoveMembersMsg {
        to_remove: vec!["adsfsa1".to_string()],
    };
    let msg = ExecuteMsg::RemoveMembers(remove_msg);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.attributes.len(), 2);
    let res = query_members(deps.as_ref(), None, None).unwrap();
    assert_eq!(res.members.len(), 1);
}

#[test]
fn update_per_address_limit() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());

    let per_address_limit: u32 = 2;
    let msg = ExecuteMsg::UpdatePerAddressLimit(per_address_limit);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.attributes.len(), 2);
    let wl_config: ConfigResponse = query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(wl_config.per_address_limit, per_address_limit);
}

#[test]
fn query_members_pagination() {
    let mut deps = mock_dependencies();
    let mut members = vec![];
    for i in 0..150 {
        members.push(format!("juno1{}", i));
    }
    let msg = InstantiateMsg {
        members: members.clone(),
        start_time: START_TIME,
        end_time: END_TIME,
        unit_price: coin(UNIT_AMOUNT, NATIVE_DENOM),
        per_address_limit: 1,
        member_limit: 1000,
    };
    let info = mock_info(ADMIN, &[coin(100_000_000, "ujuno")]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let mut all_elements: Vec<String> = vec![];

    // enforcing a min
    let res = query_members(deps.as_ref(), None, None).unwrap();
    assert_eq!(res.members.len(), 25);

    // enforcing a max
    let res = query_members(deps.as_ref(), None, Some(125)).unwrap();
    assert_eq!(res.members.len(), 100);

    // first fetch
    let res = query_members(deps.as_ref(), None, Some(50)).unwrap();
    assert_eq!(res.members.len(), 50);
    all_elements.append(&mut res.members.clone());

    // second
    let res = query_members(
        deps.as_ref(),
        Some(res.members[res.members.len() - 1].clone()),
        Some(50),
    )
    .unwrap();
    assert_eq!(res.members.len(), 50);
    all_elements.append(&mut res.members.clone());

    // third
    let res = query_members(
        deps.as_ref(),
        Some(res.members[res.members.len() - 1].clone()),
        Some(50),
    )
    .unwrap();
    all_elements.append(&mut res.members.clone());
    assert_eq!(res.members.len(), 50);

    // check fetched items
    assert_eq!(all_elements.len(), 150);
    members.sort();
    all_elements.sort();
    assert_eq!(members, all_elements);
}

#[test]
fn increase_member_limit() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());
    let res = query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(1000, res.member_limit);

    let msg = ExecuteMsg::IncreaseMemberLimit(1001);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_ok());

    let msg = ExecuteMsg::IncreaseMemberLimit(1002);
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_ok());
}
