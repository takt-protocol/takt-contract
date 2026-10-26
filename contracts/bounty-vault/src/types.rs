use soroban_sdk::{contracttype, Address};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 17_280; // ~1 day
pub const INSTANCE_BUMP_AMOUNT: u32 = 518_400; // ~30 days
pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960; // ~7 days
pub const PERSISTENT_BUMP_AMOUNT: u32 = 518_400; // ~30 days

pub const MAX_FEE_BPS: u32 = 1_000; // Max 10% protocol fee
pub const BPS_DENOMINATOR: i128 = 10_000;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultConfig {
    pub admin: Address,
    pub token: Address,
    pub fee_bps: u32,
    pub treasury: Address,
    pub job_engine: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Config,
    UserBalance(Address),
    JobEscrow(u64),
    JobOwner(u64),
}
