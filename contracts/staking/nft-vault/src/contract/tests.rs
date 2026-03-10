use crate::{contract, state::Config};

use cosmwasm_std::{testing::mock_env, Addr, Uint128};
use std::collections::HashMap;
use sylvia::{
    cw_multi_test::{App as CwApp, IntoAddr},
    multitest::App,
};

#[test]
fn test_update_stake_amounts() {
    let app: App<CwApp> = App::default();
    let mut app_mut = app.app_mut();

    let user1 = "user1".into_addr();

    let collection1 = "collection1".into_addr();
    let collection2 = "collection2".into_addr();

    let nft_vault = contract::NftVaultContract::new();

    nft_vault
        .users_collection_staked_amounts
        .save(
            app_mut.storage_mut(),
            (user1.clone(), collection1.clone()),
            &4,
        )
        .unwrap();
    nft_vault
        .users_collection_staked_amounts
        .save(
            app_mut.storage_mut(),
            (user1.clone(), collection2.clone()),
            &7,
        )
        .unwrap();
    nft_vault
        .total_staked_amount
        .save(app_mut.storage_mut(), &Uint128::new(4), 0)
        .unwrap();

    let mut env = mock_env();
    env.block.height = 1;

    let config = Config::<Addr> {
        rewards_code_id: 0,
        unstaking_duration_sec: 60,
        collections: vec![collection1.clone(), collection2.clone()],
    };

    let mut collection_deltas: HashMap<Addr, i64> = HashMap::new();
    collection_deltas.insert(collection1.clone(), 6);
    collection_deltas.insert(collection2.clone(), 8);

    let _result = nft_vault.update_stake_amounts(
        app_mut.storage_mut(),
        &env,
        config,
        &user1,
        collection_deltas,
    );

    let user_collection_staked_amount_1 = nft_vault
        .users_collection_staked_amounts
        .load(app_mut.storage_mut(), (user1.clone(), collection1.clone()))
        .unwrap();
    assert_eq!(user_collection_staked_amount_1, 10);

    let user_collection_staked_amount_2 = nft_vault
        .users_collection_staked_amounts
        .load(app_mut.storage_mut(), (user1.clone(), collection2.clone()))
        .unwrap();
    assert_eq!(user_collection_staked_amount_2, 15);

    let total_staked_amount = nft_vault
        .total_staked_amount
        .load(app_mut.storage_mut())
        .unwrap();
    assert_eq!(total_staked_amount, Uint128::new(10));
}
