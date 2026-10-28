#![cfg(test)]

use bounty_vault::{BountyVault, BountyVaultClient};
use keeper_registry::{KeeperRegistry, KeeperRegistryClient};
use soroban_sdk::{
    contract, contractimpl, symbol_short,
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

use crate::{errors::EngineError, types::JobStatus, JobEngine, JobEngineClient};

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    pub fn increment(env: Env, step: u32) -> u32 {
        let key = symbol_short!("count");
        let current: u32 = env.storage().instance().get(&key).unwrap_or(0);
        let next = current + step;
        env.storage().instance().set(&key, &next);
        next
    }

    pub fn get_count(env: Env) -> u32 {
        let key = symbol_short!("count");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}

struct TestFixture<'a> {
    env: Env,
    engine: JobEngineClient<'a>,
    registry: KeeperRegistryClient<'a>,
    vault: BountyVaultClient<'a>,
    counter: CounterContractClient<'a>,
    counter_addr: Address,
    token_client: TokenClient<'a>,
    token_admin_client: StellarAssetClient<'a>,
    treasury: Address,
}

fn setup_fixture() -> TestFixture<'static> {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_addr = token_contract.address();
    let token_client = TokenClient::new(&env, &token_addr);
    let token_admin_client = StellarAssetClient::new(&env, &token_addr);

    let registry_addr = env.register(KeeperRegistry, ());
    let registry = KeeperRegistryClient::new(&env, &registry_addr);

    let vault_addr = env.register(BountyVault, ());
    let vault = BountyVaultClient::new(&env, &vault_addr);

    let engine_addr = env.register(JobEngine, ());
    let engine = JobEngineClient::new(&env, &engine_addr);

    let counter_addr = env.register(CounterContract, ());
    let counter = CounterContractClient::new(&env, &counter_addr);

    // Initialize in strict dependency order
    registry.initialize(&admin, &token_addr, &1_000_000, &3_600);
    vault.initialize(&admin, &token_addr, &500, &treasury); // 5% protocol fee
    engine.initialize(&admin, &registry_addr, &vault_addr);

    // Authorize JobEngine in both dependency contracts
    registry.set_engine(&engine_addr);
    vault.set_engine(&engine_addr);

    TestFixture {
        env,
        engine,
        registry,
        vault,
        counter,
        counter_addr,
        token_client,
        token_admin_client,
        treasury,
    }
}

#[test]
fn test_end_to_end_job_creation_and_scheduled_execution() {
    let f = setup_fixture();

    let owner = Address::generate(&f.env);
    let keeper = Address::generate(&f.env);

    f.token_admin_client.mint(&owner, &5_000_000);
    f.token_admin_client.mint(&keeper, &2_000_000);

    // Register keeper with 1_000_000 stake
    f.registry.register_keeper(&keeper, &1_000_000);

    // Owner deposits 1_000_000 into BountyVault
    f.vault.deposit(&owner, &1_000_000);

    f.env.ledger().with_mut(|li| {
        li.timestamp = 1_000;
    });

    let args: Vec<Val> = vec![&f.env, 5u32.into_val(&f.env)];
    let job_id = f.engine.create_job(
        &owner,
        &f.counter_addr,
        &Symbol::new(&f.env, "increment"),
        &args,
        &60,      // every 60 seconds
        &200_000, // 200_000 bounty per run
        &2,       // max 2 runs
        &400_000, // initial escrow for 2 runs
    );

    assert_eq!(job_id, 1);
    assert_eq!(f.engine.get_total_jobs(), 1);
    assert!(f.engine.is_executable(&job_id));

    // Run #1 at t = 1000
    let receipt1 = f.engine.execute_job(&keeper, &job_id);
    assert_eq!(receipt1.run_number, 1);
    assert_eq!(receipt1.keeper_reward, 190_000);
    assert_eq!(receipt1.protocol_fee, 10_000);
    assert_eq!(f.counter.get_count(), 5);
    assert_eq!(f.registry.get_keeper(&keeper).total_executions, 1);

    // Immediate second execution at t = 1020 must fail with IntervalNotElapsed
    f.env.ledger().with_mut(|li| {
        li.timestamp = 1_020;
    });
    assert!(!f.engine.is_executable(&job_id));
    let early_err = f.engine.try_execute_job(&keeper, &job_id);
    assert_eq!(early_err, Err(Ok(EngineError::IntervalNotElapsed)));

    // Advance time to t = 1060 and execute Run #2 (reaches max_runs = 2)
    f.env.ledger().with_mut(|li| {
        li.timestamp = 1_060;
    });
    assert!(f.engine.is_executable(&job_id));
    let receipt2 = f.engine.execute_job(&keeper, &job_id);
    assert_eq!(receipt2.run_number, 2);
    assert_eq!(f.counter.get_count(), 10);

    let completed_job = f.engine.get_job(&job_id);
    assert_eq!(completed_job.status, JobStatus::Completed);
    assert!(!f.engine.is_executable(&job_id));
    assert_eq!(f.token_client.balance(&f.treasury), 20_000);
}

#[test]
fn test_pause_resume_and_cancel_with_escrow_refund() {
    let f = setup_fixture();

    let owner = Address::generate(&f.env);
    f.token_admin_client.mint(&owner, &1_000_000);
    f.vault.deposit(&owner, &1_000_000);

    let args: Vec<Val> = vec![&f.env, 1u32.into_val(&f.env)];
    let job_id = f.engine.create_job(
        &owner,
        &f.counter_addr,
        &Symbol::new(&f.env, "increment"),
        &args,
        &30,
        &100_000,
        &0, // unlimited runs until escrow depleted
        &500_000,
    );

    assert_eq!(f.vault.get_user_balance(&owner), 500_000);
    assert_eq!(f.vault.get_job_escrow(&job_id), 500_000);

    f.engine.pause_job(&owner, &job_id);
    assert_eq!(f.engine.get_job(&job_id).status, JobStatus::Paused);
    assert!(!f.engine.is_executable(&job_id));

    f.engine.resume_job(&owner, &job_id);
    assert_eq!(f.engine.get_job(&job_id).status, JobStatus::Active);
    assert!(f.engine.is_executable(&job_id));

    // Cancel job and verify remaining 500_000 escrow is returned to owner's vault balance
    let refunded = f.engine.cancel_job(&owner, &job_id);
    assert_eq!(refunded, 500_000);
    assert_eq!(f.vault.get_job_escrow(&job_id), 0);
    assert_eq!(f.vault.get_user_balance(&owner), 1_000_000);
    assert_eq!(f.engine.get_job(&job_id).status, JobStatus::Cancelled);
}
