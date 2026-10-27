use soroban_sdk::{Address, Env};

use crate::errors::VaultError;
use crate::types::{
    DataKey, VaultConfig, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
    PERSISTENT_BUMP_AMOUNT, PERSISTENT_LIFETIME_THRESHOLD,
};

pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn has_config(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Config)
}

pub fn set_config(env: &Env, config: &VaultConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn get_config(env: &Env) -> Result<VaultConfig, VaultError> {
    env.storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(VaultError::NotInitialized)
}

pub fn get_user_balance(env: &Env, owner: &Address) -> i128 {
    let key = DataKey::UserBalance(owner.clone());
    let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    if balance > 0 {
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
    balance
}

pub fn set_user_balance(env: &Env, owner: &Address, balance: i128) {
    let key = DataKey::UserBalance(owner.clone());
    env.storage().persistent().set(&key, &balance);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_job_escrow(env: &Env, job_id: u64) -> i128 {
    let key = DataKey::JobEscrow(job_id);
    let escrow: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    if escrow > 0 {
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
    escrow
}

pub fn set_job_escrow(env: &Env, job_id: u64, amount: i128) {
    let key = DataKey::JobEscrow(job_id);
    env.storage().persistent().set(&key, &amount);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn set_job_owner(env: &Env, job_id: u64, owner: &Address) {
    let key = DataKey::JobOwner(job_id);
    env.storage().persistent().set(&key, owner);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_job_owner(env: &Env, job_id: u64) -> Option<Address> {
    let key = DataKey::JobOwner(job_id);
    env.storage().persistent().get(&key)
}
