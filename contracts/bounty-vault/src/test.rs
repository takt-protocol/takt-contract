#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

use crate::{errors::VaultError, BountyVault, BountyVaultClient};

fn setup_test() -> (
    Env,
    BountyVaultClient<'static>,
    Address,
    Address,
    Address,
    TokenClient<'static>,
    StellarAssetClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    let engine = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_addr = token_contract.address();
    let token_client = TokenClient::new(&env, &token_addr);
    let token_admin_client = StellarAssetClient::new(&env, &token_addr);

    let contract_id = env.register(BountyVault, ());
    let client = BountyVaultClient::new(&env, &contract_id);

    // 500 bps = 5% protocol fee
    client.initialize(&admin, &token_addr, &500, &treasury);
    client.set_engine(&engine);

    (
        env,
        client,
        admin,
        treasury,
        engine,
        token_client,
        token_admin_client,
    )
}

#[test]
fn test_reject_excessive_fee_bps() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    let token = Address::generate(&env);

    let contract_id = env.register(BountyVault, ());
    let client = BountyVaultClient::new(&env, &contract_id);

    let res = client.try_initialize(&admin, &token, &1_500, &treasury);
    assert_eq!(res, Err(Ok(VaultError::FeeTooHigh)));
}

#[test]
fn test_deposit_and_withdraw() {
    let (env, client, _admin, _treasury, _engine, token_client, token_admin_client) = setup_test();

    let user = Address::generate(&env);
    token_admin_client.mint(&user, &1_000_000);

    let balance = client.deposit(&user, &600_000);
    assert_eq!(balance, 600_000);
    assert_eq!(client.get_user_balance(&user), 600_000);
    assert_eq!(token_client.balance(&user), 400_000);

    let remaining = client.withdraw(&user, &250_000);
    assert_eq!(remaining, 350_000);
    assert_eq!(token_client.balance(&user), 650_000);

    let over_withdraw = client.try_withdraw(&user, &400_000);
    assert_eq!(over_withdraw, Err(Ok(VaultError::InsufficientUserBalance)));
}

#[test]
fn test_fund_job_payout_with_fee_split_and_refund() {
    let (env, client, _admin, treasury, _engine, token_client, token_admin_client) = setup_test();

    let owner = Address::generate(&env);
    let keeper = Address::generate(&env);
    token_admin_client.mint(&owner, &1_000_000);

    client.deposit(&owner, &1_000_000);

    // Lock 500_000 for job #1 via engine
    let escrow = client.fund_job_from_engine(&owner, &1, &500_000);
    assert_eq!(escrow, 500_000);
    assert_eq!(client.get_user_balance(&owner), 500_000);

    // Owner tops up job #1 by another 100_000
    let updated_escrow = client.top_up_job(&owner, &1, &100_000);
    assert_eq!(updated_escrow, 600_000);
    assert_eq!(client.get_user_balance(&owner), 400_000);

    // Execute payout of 200_000 (5% fee = 10_000 to treasury, 190_000 to keeper)
    let (keeper_reward, fee) = client.payout_execution(&1, &keeper, &200_000);
    assert_eq!(keeper_reward, 190_000);
    assert_eq!(fee, 10_000);
    assert_eq!(token_client.balance(&keeper), 190_000);
    assert_eq!(token_client.balance(&treasury), 10_000);
    assert_eq!(client.get_job_escrow(&1), 400_000);

    // Refund remaining 400_000 on job cancellation back to owner's vault balance
    let refunded = client.refund_job(&1, &owner);
    assert_eq!(refunded, 400_000);
    assert_eq!(client.get_job_escrow(&1), 0);
    assert_eq!(client.get_user_balance(&owner), 800_000);
}
