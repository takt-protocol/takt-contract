use soroban_sdk::{contracttype, Address, Symbol, Val, Vec};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280; // ~1 day
pub const INSTANCE_BUMP_AMOUNT: u32 = 518_400; // ~30 days
pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960; // ~7 days
pub const PERSISTENT_BUMP_AMOUNT: u32 = 518_400; // ~30 days

pub const MIN_INTERVAL_SECONDS: u64 = 5;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobStatus {
    Active,
    Paused,
    Cancelled,
    Completed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub id: u64,
    pub owner: Address,
    pub target_contract: Address,
    pub target_fn: Symbol,
    pub target_args: Vec<Val>,
    pub interval_seconds: u64,
    pub last_executed_at: u64,
    pub bounty_per_run: i128,
    pub max_runs: u32,
    pub completed_runs: u32,
    pub status: JobStatus,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionReceipt {
    pub job_id: u64,
    pub keeper: Address,
    pub executed_at: u64,
    pub run_number: u32,
    pub keeper_reward: i128,
    pub protocol_fee: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    KeeperRegistry,
    BountyVault,
    NextJobId,
    Job(u64),
    LastReceipt(u64),
}
