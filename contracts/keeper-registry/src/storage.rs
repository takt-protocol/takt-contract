use soroban_sdk::{Address, Env};

use crate::errors::RegistryError;
use crate::types::{
    DataKey, KeeperInfo, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT,
    PERSISTENT_LIFETIME_THRESHOLD,
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

pub fn get_admin(env: &Env) -> Result<Address, RegistryError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(RegistryError::NotInitialized)
}

pub fn set_stake_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::StakeToken, token);
}

pub fn get_stake_token(env: &Env) -> Result<Address, RegistryError> {
    env.storage()
        .instance()
        .get(&DataKey::StakeToken)
        .ok_or(RegistryError::NotInitialized)
}

pub fn set_min_stake(env: &Env, min_stake: i128) {
    env.storage().instance().set(&DataKey::MinStake, &min_stake);
}

pub fn get_min_stake(env: &Env) -> Result<i128, RegistryError> {
    env.storage()
        .instance()
        .get(&DataKey::MinStake)
        .ok_or(RegistryError::NotInitialized)
}

pub fn set_unbond_period(env: &Env, period: u64) {
    env.storage()
        .instance()
        .set(&DataKey::UnbondPeriod, &period);
}

pub fn get_unbond_period(env: &Env) -> Result<u64, RegistryError> {
    env.storage()
        .instance()
        .get(&DataKey::UnbondPeriod)
        .ok_or(RegistryError::NotInitialized)
}

pub fn set_job_engine(env: &Env, engine: &Address) {
    env.storage().instance().set(&DataKey::JobEngine, engine);
}

pub fn get_job_engine(env: &Env) -> Result<Address, RegistryError> {
    env.storage()
        .instance()
        .get(&DataKey::JobEngine)
        .ok_or(RegistryError::UnauthorizedEngine)
}

pub fn set_total_keepers(env: &Env, count: u32) {
    env.storage().instance().set(&DataKey::TotalKeepers, &count);
}

pub fn get_total_keepers(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::TotalKeepers)
        .unwrap_or(0)
}

pub fn has_keeper(env: &Env, keeper: &Address) -> bool {
    let key = DataKey::Keeper(keeper.clone());
    env.storage().persistent().has(&key)
}

pub fn set_keeper(env: &Env, info: &KeeperInfo) {
    let key = DataKey::Keeper(info.address.clone());
    env.storage().persistent().set(&key, info);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

pub fn get_keeper(env: &Env, keeper: &Address) -> Result<KeeperInfo, RegistryError> {
    let key = DataKey::Keeper(keeper.clone());
    let info: KeeperInfo = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(RegistryError::KeeperNotFound)?;
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
    Ok(info)
}

pub fn remove_keeper(env: &Env, keeper: &Address) {
    let key = DataKey::Keeper(keeper.clone());
    env.storage().persistent().remove(&key);
}
