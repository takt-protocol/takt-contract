#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

use crate::{errors::RegistryError, types::KeeperStatus, KeeperRegistry, KeeperRegistryClient};

fn setup_test() -> (
    Env,
    KeeperRegistryClient<'static>,
    Address,
    Address,
    Address,
    TokenClient<'static>,
    StellarAssetClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let engine = Address::generate(&env);

    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_addr = token_contract.address();
    let token_client = TokenClient::new(&env, &token_addr);
    let token_admin_client = StellarAssetClient::new(&env, &token_addr);

    let contract_id = env.register(KeeperRegistry, ());
    let client = KeeperRegistryClient::new(&env, &contract_id);

    client.initialize(&admin, &token_addr, &1_000_000_000, &86_400);
    client.set_engine(&engine);

    (
        env,
        client,
        admin,
        engine,
        token_addr,
        token_client,
        token_admin_client,
    )
}

#[test]
fn test_initialize_and_register_keeper() {
    let (env, client, _admin, _engine, _token_addr, token_client, token_admin_client) =
        setup_test();

    let keeper = Address::generate(&env);
    token_admin_client.mint(&keeper, &2_000_000_000);

    client.register_keeper(&keeper, &1_500_000_000);

    assert!(client.is_active_keeper(&keeper));
    assert_eq!(client.get_total_keepers(), 1);
    assert_eq!(token_client.balance(&keeper), 500_000_000);

    let info = client.get_keeper(&keeper);
    assert_eq!(info.staked_amount, 1_500_000_000);
    assert_eq!(info.status, KeeperStatus::Active);
    assert_eq!(info.total_executions, 0);
}

#[test]
fn test_reject_insufficient_stake() {
    let (env, client, _admin, _engine, _token_addr, _token_client, token_admin_client) =
        setup_test();

    let keeper = Address::generate(&env);
    token_admin_client.mint(&keeper, &2_000_000_000);

    let res = client.try_register_keeper(&keeper, &500_000_000);
    assert_eq!(res, Err(Ok(RegistryError::InsufficientStake)));
}

#[test]
fn test_unbond_and_withdraw_after_cooldown() {
    let (env, client, _admin, _engine, _token_addr, token_client, token_admin_client) =
        setup_test();

    let keeper = Address::generate(&env);
    token_admin_client.mint(&keeper, &1_000_000_000);
    client.register_keeper(&keeper, &1_000_000_000);

    env.ledger().with_mut(|li| {
        li.timestamp = 1_000;
    });

    let unlock_time = client.request_unbond(&keeper);
    assert_eq!(unlock_time, 87_400);
    assert!(!client.is_active_keeper(&keeper));

    // Attempt early withdrawal before cooldown elapses
    env.ledger().with_mut(|li| {
        li.timestamp = 50_000;
    });
    let early_res = client.try_withdraw_stake(&keeper);
    assert_eq!(early_res, Err(Ok(RegistryError::CooldownNotElapsed)));

    // Advance ledger timestamp past unlock_time
    env.ledger().with_mut(|li| {
        li.timestamp = 87_401;
    });
    let withdrawn = client.withdraw_stake(&keeper);
    assert_eq!(withdrawn, 1_000_000_000);
    assert_eq!(token_client.balance(&keeper), 1_000_000_000);
    assert_eq!(client.get_total_keepers(), 0);
}

#[test]
fn test_record_execution_and_slash_recovery() {
    let (env, client, _admin, _engine, _token_addr, token_client, token_admin_client) =
        setup_test();

    let keeper = Address::generate(&env);
    let treasury = Address::generate(&env);
    token_admin_client.mint(&keeper, &2_000_000_000);

    client.register_keeper(&keeper, &1_200_000_000);

    let count = client.record_execution(&keeper);
    assert_eq!(count, 1);

    // Slash keeper below min_stake (1_200_000_000 - 400_000_000 = 800_000_000 < 1_000_000_000)
    let remaining = client.slash_keeper(&keeper, &400_000_000, &treasury);
    assert_eq!(remaining, 800_000_000);
    assert_eq!(token_client.balance(&treasury), 400_000_000);
    assert!(!client.is_active_keeper(&keeper));

    let slashed_info = client.get_keeper(&keeper);
    assert_eq!(slashed_info.status, KeeperStatus::Slashed);
    assert_eq!(slashed_info.slashes, 1);

    // Recover active status by topping up stake above min_stake
    let new_total = client.increase_stake(&keeper, &300_000_000);
    assert_eq!(new_total, 1_100_000_000);
    assert!(client.is_active_keeper(&keeper));
}

#[test]
fn test_update_min_stake_and_unbond_period_lifecycle() {
    let (env, client, _admin, _engine, _token_addr, _token_client, token_admin_client) =
        setup_test();

    // 1. Initial values
    assert_eq!(client.get_min_stake(), 1_000_000_000);
    assert_eq!(client.get_unbond_period(), 86_400);

    // 2. Register first keeper with original min stake
    let keeper1 = Address::generate(&env);
    token_admin_client.mint(&keeper1, &2_000_000_000);
    client.register_keeper(&keeper1, &1_000_000_000);
    assert!(client.is_active_keeper(&keeper1));

    // 3. Reject zero or negative min stake
    let zero_err = client.try_update_min_stake(&0);
    assert_eq!(zero_err, Err(Ok(RegistryError::InvalidAmount)));
    let neg_err = client.try_update_min_stake(&-100);
    assert_eq!(neg_err, Err(Ok(RegistryError::InvalidAmount)));

    // 4. Update min stake to 2_000_000_000
    client.update_min_stake(&2_000_000_000);
    assert_eq!(client.get_min_stake(), 2_000_000_000);

    // 5. Existing keeper1 remains active even though their stake is now below new min_stake
    assert!(client.is_active_keeper(&keeper1));

    // 6. New keeper registering with old 1_000_000_000 stake is rejected with InsufficientStake
    let keeper2 = Address::generate(&env);
    token_admin_client.mint(&keeper2, &3_000_000_000);
    let low_stake_err = client.try_register_keeper(&keeper2, &1_000_000_000);
    assert_eq!(low_stake_err, Err(Ok(RegistryError::InsufficientStake)));

    // New keeper registering with >= 2_000_000_000 succeeds
    client.register_keeper(&keeper2, &2_000_000_000);
    assert!(client.is_active_keeper(&keeper2));

    // 7. Update unbond period
    client.update_unbond_period(&172_800);
    assert_eq!(client.get_unbond_period(), 172_800);
}
