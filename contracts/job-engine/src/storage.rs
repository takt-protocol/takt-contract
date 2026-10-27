use soroban_sdk::{Address, Env};

use crate::errors::EngineError;
use crate::types::{
    DataKey, ExecutionReceipt, Job, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
    PERSISTENT_BUMP_AMOUNT, PERSISTENT_LIFETIME_THRESHOLD,
};

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Result<Address, EngineError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(EngineError::NotInitialized)
}

pub fn set_keeper_registry(env: &Env, registry: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::KeeperRegistry, registry);
}

pub fn get_keeper_registry(env: &Env) -> Result<Address, EngineError> {
    env.storage()
        .instance()
        .get(&DataKey::KeeperRegistry)
        .ok_or(EngineError::NotInitialized)
}

pub fn set_bounty_vault(env: &Env, vault: &Address) {
    env.storage().instance().set(&DataKey::BountyVault, vault);
}

pub fn get_bounty_vault(env: &Env) -> Result<Address, EngineError> {
    env.storage()
        .instance()
        .get(&DataKey::BountyVault)
        .ok_or(EngineError::NotInitialized)
}

pub fn set_next_job_id(env: &Env, next_id: u64) {
    env.storage().instance().set(&DataKey::NextJobId, &next_id);
}

pub fn get_next_job_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextJobId)
        .unwrap_or(1)
}

pub fn set_job(env: &Env, job: &Job) {
    let key = DataKey::Job(job.id);
    env.storage().persistent().set(&key, job);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_job(env: &Env, job_id: u64) -> Result<Job, EngineError> {
    let key = DataKey::Job(job_id);
    let job: Job = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(EngineError::JobNotFound)?;
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
    Ok(job)
}

pub fn set_last_receipt(env: &Env, receipt: &ExecutionReceipt) {
    let key = DataKey::LastReceipt(receipt.job_id);
    env.storage().persistent().set(&key, receipt);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_last_receipt(env: &Env, job_id: u64) -> Option<ExecutionReceipt> {
    let key = DataKey::LastReceipt(job_id);
    let receipt: Option<ExecutionReceipt> = env.storage().persistent().get(&key);
    if receipt.is_some() {
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
    receipt
}
