use soroban_sdk::{contracttype, Address};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280; // ~1 day
pub const INSTANCE_BUMP_AMOUNT: u32 = 518_400; // ~30 days
pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960; // ~7 days
pub const PERSISTENT_BUMP_AMOUNT: u32 = 518_400; // ~30 days

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeeperStatus {
    Active,
    Unbonding,
    Slashed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeeperInfo {
    pub address: Address,
    pub staked_amount: i128,
    pub status: KeeperStatus,
    pub registered_at: u64,
    pub unbond_unlock_time: u64,
    pub total_executions: u64,
    pub slashes: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    StakeToken,
    MinStake,
    UnbondPeriod,
    JobEngine,
    TotalKeepers,
    Keeper(Address),
}
