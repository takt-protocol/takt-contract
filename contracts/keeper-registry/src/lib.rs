#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::errors::RegistryError;
use crate::types::{KeeperInfo, KeeperStatus};

#[contract]
pub struct KeeperRegistry;

#[contractimpl]
impl KeeperRegistry {
    pub fn initialize(
        env: Env,
        admin: Address,
        stake_token: Address,
        min_stake: i128,
        unbond_period: u64,
    ) -> Result<(), RegistryError> {
        if storage::has_admin(&env) {
            return Err(RegistryError::AlreadyInitialized);
        }
        if min_stake <= 0 {
            return Err(RegistryError::InvalidAmount);
        }

        storage::set_admin(&env, &admin);
        storage::set_stake_token(&env, &stake_token);
        storage::set_min_stake(&env, min_stake);
        storage::set_unbond_period(&env, unbond_period);
        storage::set_total_keepers(&env, 0);
        storage::extend_instance_ttl(&env);

        events::emit_initialized(&env, &admin, &stake_token, min_stake, unbond_period);
        Ok(())
    }

    pub fn set_engine(env: Env, engine: Address) -> Result<(), RegistryError> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        storage::set_job_engine(&env, &engine);
        storage::extend_instance_ttl(&env);

        events::emit_engine_set(&env, &admin, &engine);
        Ok(())
    }

    pub fn register_keeper(
        env: Env,
        keeper: Address,
        stake_amount: i128,
    ) -> Result<(), RegistryError> {
        keeper.require_auth();

        let min_stake = storage::get_min_stake(&env)?;
        if stake_amount <= 0 {
            return Err(RegistryError::InvalidAmount);
        }
        if stake_amount < min_stake {
            return Err(RegistryError::InsufficientStake);
        }
        if storage::has_keeper(&env, &keeper) {
            return Err(RegistryError::KeeperAlreadyRegistered);
        }

        let token_addr = storage::get_stake_token(&env)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&keeper, &env.current_contract_address(), &stake_amount);

        let info = KeeperInfo {
            address: keeper.clone(),
            staked_amount: stake_amount,
            status: KeeperStatus::Active,
            registered_at: env.ledger().timestamp(),
            unbond_unlock_time: 0,
            total_executions: 0,
            slashes: 0,
        };

        let total = storage::get_total_keepers(&env)
            .checked_add(1)
            .ok_or(RegistryError::Overflow)?;
        storage::set_total_keepers(&env, total);
        storage::set_keeper(&env, &info);
        storage::extend_instance_ttl(&env);

        events::emit_keeper_registered(&env, &keeper, stake_amount);
        Ok(())
    }

    pub fn increase_stake(
        env: Env,
        keeper: Address,
        additional_amount: i128,
    ) -> Result<i128, RegistryError> {
        keeper.require_auth();

        if additional_amount <= 0 {
            return Err(RegistryError::InvalidAmount);
        }

        let mut info = storage::get_keeper(&env, &keeper)?;
        let min_stake = storage::get_min_stake(&env)?;
        let token_addr = storage::get_stake_token(&env)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&keeper, &env.current_contract_address(), &additional_amount);

        info.staked_amount = info
            .staked_amount
            .checked_add(additional_amount)
            .ok_or(RegistryError::Overflow)?;

        if info.status == KeeperStatus::Slashed && info.staked_amount >= min_stake {
            info.status = KeeperStatus::Active;
        }

        storage::set_keeper(&env, &info);
        storage::extend_instance_ttl(&env);

        events::emit_stake_increased(&env, &keeper, info.staked_amount);
        Ok(info.staked_amount)
    }

    pub fn request_unbond(env: Env, keeper: Address) -> Result<u64, RegistryError> {
        keeper.require_auth();

        let mut info = storage::get_keeper(&env, &keeper)?;
        if info.status != KeeperStatus::Active {
            return Err(RegistryError::KeeperNotActive);
        }

        let unbond_period = storage::get_unbond_period(&env)?;
        let unlock_time = env
            .ledger()
            .timestamp()
            .checked_add(unbond_period)
            .ok_or(RegistryError::Overflow)?;

        info.status = KeeperStatus::Unbonding;
        info.unbond_unlock_time = unlock_time;

        storage::set_keeper(&env, &info);
        storage::extend_instance_ttl(&env);

        events::emit_unbond_requested(&env, &keeper, unlock_time);
        Ok(unlock_time)
    }

    pub fn withdraw_stake(env: Env, keeper: Address) -> Result<i128, RegistryError> {
        keeper.require_auth();

        let info = storage::get_keeper(&env, &keeper)?;
        if info.status != KeeperStatus::Unbonding {
            return Err(RegistryError::KeeperNotUnbonding);
        }
        if env.ledger().timestamp() < info.unbond_unlock_time {
            return Err(RegistryError::CooldownNotElapsed);
        }

        let amount = info.staked_amount;
        storage::remove_keeper(&env, &keeper);

        let total = storage::get_total_keepers(&env).saturating_sub(1);
        storage::set_total_keepers(&env, total);

        let token_addr = storage::get_stake_token(&env)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &keeper, &amount);

        storage::extend_instance_ttl(&env);

        events::emit_stake_withdrawn(&env, &keeper, amount);
        Ok(amount)
    }

    pub fn record_execution(env: Env, keeper: Address) -> Result<u64, RegistryError> {
        let engine = storage::get_job_engine(&env)?;
        engine.require_auth();

        let mut info = storage::get_keeper(&env, &keeper)?;
        if info.status != KeeperStatus::Active {
            return Err(RegistryError::KeeperNotActive);
        }

        info.total_executions = info
            .total_executions
            .checked_add(1)
            .ok_or(RegistryError::Overflow)?;

        storage::set_keeper(&env, &info);
        storage::extend_instance_ttl(&env);

        events::emit_execution_recorded(&env, &keeper, info.total_executions);
        Ok(info.total_executions)
    }

    pub fn slash_keeper(
        env: Env,
        keeper: Address,
        slash_amount: i128,
        recipient: Address,
    ) -> Result<i128, RegistryError> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        if slash_amount <= 0 {
            return Err(RegistryError::InvalidAmount);
        }

        let mut info = storage::get_keeper(&env, &keeper)?;
        if slash_amount > info.staked_amount {
            return Err(RegistryError::InvalidAmount);
        }

        let min_stake = storage::get_min_stake(&env)?;
        info.staked_amount = info
            .staked_amount
            .checked_sub(slash_amount)
            .ok_or(RegistryError::Overflow)?;
        info.slashes = info.slashes.checked_add(1).ok_or(RegistryError::Overflow)?;

        if info.staked_amount < min_stake {
            info.status = KeeperStatus::Slashed;
        }

        storage::set_keeper(&env, &info);

        let token_addr = storage::get_stake_token(&env)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &recipient, &slash_amount);

        storage::extend_instance_ttl(&env);

        events::emit_keeper_slashed(&env, &keeper, slash_amount, info.staked_amount);
        Ok(info.staked_amount)
    }

    pub fn get_keeper(env: Env, keeper: Address) -> Result<KeeperInfo, RegistryError> {
        storage::get_keeper(&env, &keeper)
    }

    pub fn is_active_keeper(env: Env, keeper: Address) -> bool {
        match storage::get_keeper(&env, &keeper) {
            Ok(info) => info.status == KeeperStatus::Active,
            Err(_) => false,
        }
    }

    pub fn get_total_keepers(env: Env) -> u32 {
        storage::get_total_keepers(&env)
    }
}
