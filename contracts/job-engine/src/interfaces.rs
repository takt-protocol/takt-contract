use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "KeeperRegistryCrossClient")]
pub trait KeeperRegistryInterface {
    fn record_execution(env: Env, keeper: Address) -> u64;
    fn is_active_keeper(env: Env, keeper: Address) -> bool;
}

#[contractclient(name = "BountyVaultCrossClient")]
pub trait BountyVaultInterface {
    fn fund_job_from_engine(env: Env, owner: Address, job_id: u64, amount: i128) -> i128;
    fn payout_execution(
        env: Env,
        job_id: u64,
        keeper: Address,
        bounty_amount: i128,
    ) -> (i128, i128);
    fn refund_job(env: Env, job_id: u64, owner: Address) -> i128;
    fn get_job_escrow(env: Env, job_id: u64) -> i128;
}
