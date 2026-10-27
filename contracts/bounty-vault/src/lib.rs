#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::errors::VaultError;
use crate::types::{VaultConfig, BPS_DENOMINATOR, MAX_FEE_BPS};

#[contract]
pub struct BountyVault;

#[contractimpl]
impl BountyVault {
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        fee_bps: u32,
        treasury: Address,
    ) -> Result<(), VaultError> {
        if storage::has_config(&env) {
            return Err(VaultError::AlreadyInitialized);
        }
        if fee_bps > MAX_FEE_BPS {
            return Err(VaultError::FeeTooHigh);
        }

        let config = VaultConfig {
            admin: admin.clone(),
            token: token.clone(),
            fee_bps,
            treasury: treasury.clone(),
            job_engine: None,
        };

        storage::set_config(&env, &config);
        storage::extend_instance_ttl(&env);

        events::emit_initialized(&env, &admin, &token, fee_bps, &treasury);
        Ok(())
    }

    pub fn set_engine(env: Env, engine: Address) -> Result<(), VaultError> {
        let mut config = storage::get_config(&env)?;
        config.admin.require_auth();

        config.job_engine = Some(engine.clone());
        storage::set_config(&env, &config);
        storage::extend_instance_ttl(&env);

        events::emit_engine_set(&env, &config.admin, &engine);
        Ok(())
    }

    pub fn deposit(env: Env, owner: Address, amount: i128) -> Result<i128, VaultError> {
        owner.require_auth();

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let config = storage::get_config(&env)?;
        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(&owner, &env.current_contract_address(), &amount);

        let current_balance = storage::get_user_balance(&env, &owner);
        let new_balance = current_balance
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        storage::set_user_balance(&env, &owner, new_balance);
        storage::extend_instance_ttl(&env);

        events::emit_deposited(&env, &owner, amount, new_balance);
        Ok(new_balance)
    }

    pub fn withdraw(env: Env, owner: Address, amount: i128) -> Result<i128, VaultError> {
        owner.require_auth();

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let config = storage::get_config(&env)?;
        let current_balance = storage::get_user_balance(&env, &owner);
        if current_balance < amount {
            return Err(VaultError::InsufficientUserBalance);
        }

        let remaining_balance = current_balance
            .checked_sub(amount)
            .ok_or(VaultError::Overflow)?;
        storage::set_user_balance(&env, &owner, remaining_balance);

        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(&env.current_contract_address(), &owner, &amount);

        storage::extend_instance_ttl(&env);

        events::emit_withdrawn(&env, &owner, amount, remaining_balance);
        Ok(remaining_balance)
    }

    pub fn fund_job_from_engine(
        env: Env,
        owner: Address,
        job_id: u64,
        amount: i128,
    ) -> Result<i128, VaultError> {
        let config = storage::get_config(&env)?;
        let engine = config.job_engine.ok_or(VaultError::EngineNotSet)?;
        engine.require_auth();

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let current_balance = storage::get_user_balance(&env, &owner);
        if current_balance < amount {
            return Err(VaultError::InsufficientUserBalance);
        }

        let new_user_balance = current_balance
            .checked_sub(amount)
            .ok_or(VaultError::Overflow)?;
        storage::set_user_balance(&env, &owner, new_user_balance);

        let current_escrow = storage::get_job_escrow(&env, job_id);
        let new_escrow = current_escrow
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        storage::set_job_escrow(&env, job_id, new_escrow);
        storage::set_job_owner(&env, job_id, &owner);
        storage::extend_instance_ttl(&env);

        events::emit_job_funded(&env, job_id, &owner, new_escrow);
        Ok(new_escrow)
    }

    pub fn top_up_job(
        env: Env,
        owner: Address,
        job_id: u64,
        amount: i128,
    ) -> Result<i128, VaultError> {
        owner.require_auth();

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let _config = storage::get_config(&env)?;
        let current_balance = storage::get_user_balance(&env, &owner);
        if current_balance < amount {
            return Err(VaultError::InsufficientUserBalance);
        }

        let new_user_balance = current_balance
            .checked_sub(amount)
            .ok_or(VaultError::Overflow)?;
        storage::set_user_balance(&env, &owner, new_user_balance);

        let current_escrow = storage::get_job_escrow(&env, job_id);
        let new_escrow = current_escrow
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        storage::set_job_escrow(&env, job_id, new_escrow);
        storage::extend_instance_ttl(&env);

        events::emit_job_topped_up(&env, job_id, &owner, new_escrow);
        Ok(new_escrow)
    }

    pub fn payout_execution(
        env: Env,
        job_id: u64,
        keeper: Address,
        bounty_amount: i128,
    ) -> Result<(i128, i128), VaultError> {
        let config = storage::get_config(&env)?;
        let engine = config.job_engine.clone().ok_or(VaultError::EngineNotSet)?;
        engine.require_auth();

        if bounty_amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let current_escrow = storage::get_job_escrow(&env, job_id);
        if current_escrow < bounty_amount {
            return Err(VaultError::InsufficientJobEscrow);
        }

        let remaining_escrow = current_escrow
            .checked_sub(bounty_amount)
            .ok_or(VaultError::Overflow)?;
        storage::set_job_escrow(&env, job_id, remaining_escrow);

        let fee = bounty_amount
            .checked_mul(config.fee_bps as i128)
            .ok_or(VaultError::Overflow)?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(VaultError::Overflow)?;

        let keeper_reward = bounty_amount.checked_sub(fee).ok_or(VaultError::Overflow)?;

        let token_client = token::Client::new(&env, &config.token);
        if keeper_reward > 0 {
            token_client.transfer(&env.current_contract_address(), &keeper, &keeper_reward);
        }
        if fee > 0 {
            token_client.transfer(&env.current_contract_address(), &config.treasury, &fee);
        }

        storage::extend_instance_ttl(&env);

        events::emit_payout(&env, job_id, &keeper, keeper_reward, fee);
        Ok((keeper_reward, fee))
    }

    pub fn refund_job(env: Env, job_id: u64, owner: Address) -> Result<i128, VaultError> {
        let config = storage::get_config(&env)?;
        let engine = config.job_engine.ok_or(VaultError::EngineNotSet)?;
        engine.require_auth();

        let remaining_escrow = storage::get_job_escrow(&env, job_id);
        if remaining_escrow > 0 {
            storage::set_job_escrow(&env, job_id, 0);
            let current_balance = storage::get_user_balance(&env, &owner);
            let new_balance = current_balance
                .checked_add(remaining_escrow)
                .ok_or(VaultError::Overflow)?;
            storage::set_user_balance(&env, &owner, new_balance);
        }

        storage::extend_instance_ttl(&env);

        events::emit_refunded(&env, job_id, &owner, remaining_escrow);
        Ok(remaining_escrow)
    }

    pub fn get_user_balance(env: Env, owner: Address) -> i128 {
        storage::get_user_balance(&env, &owner)
    }

    pub fn get_job_escrow(env: Env, job_id: u64) -> i128 {
        storage::get_job_escrow(&env, job_id)
    }

    pub fn get_config(env: Env) -> Result<VaultConfig, VaultError> {
        storage::get_config(&env)
    }
}
