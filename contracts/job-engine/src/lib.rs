#![no_std]

pub mod errors;
pub mod events;
pub mod interfaces;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};

use crate::errors::EngineError;
use crate::interfaces::{BountyVaultCrossClient, KeeperRegistryCrossClient};
use crate::types::{ExecutionReceipt, Job, JobStatus, MIN_INTERVAL_SECONDS};

#[contract]
pub struct JobEngine;

#[contractimpl]
impl JobEngine {
    pub fn initialize(
        env: Env,
        admin: Address,
        keeper_registry: Address,
        bounty_vault: Address,
    ) -> Result<(), EngineError> {
        if storage::has_admin(&env) {
            return Err(EngineError::AlreadyInitialized);
        }

        storage::set_admin(&env, &admin);
        storage::set_keeper_registry(&env, &keeper_registry);
        storage::set_bounty_vault(&env, &bounty_vault);
        storage::set_next_job_id(&env, 1);
        storage::extend_instance_ttl(&env);

        events::emit_initialized(&env, &admin, &keeper_registry, &bounty_vault);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_job(
        env: Env,
        owner: Address,
        target_contract: Address,
        target_fn: Symbol,
        target_args: Vec<Val>,
        interval_seconds: u64,
        bounty_per_run: i128,
        max_runs: u32,
        initial_funding: i128,
    ) -> Result<u64, EngineError> {
        owner.require_auth();

        if interval_seconds < MIN_INTERVAL_SECONDS {
            return Err(EngineError::InvalidInterval);
        }
        if bounty_per_run <= 0 {
            return Err(EngineError::InvalidBounty);
        }
        if initial_funding < bounty_per_run {
            return Err(EngineError::InsufficientInitialFunding);
        }

        let vault_addr = storage::get_bounty_vault(&env)?;
        let job_id = storage::get_next_job_id(&env);
        let next_id = job_id.checked_add(1).ok_or(EngineError::Overflow)?;
        storage::set_next_job_id(&env, next_id);

        let vault_client = BountyVaultCrossClient::new(&env, &vault_addr);
        vault_client.fund_job_from_engine(&owner, &job_id, &initial_funding);

        let job = Job {
            id: job_id,
            owner: owner.clone(),
            target_contract: target_contract.clone(),
            target_fn: target_fn.clone(),
            target_args,
            interval_seconds,
            last_executed_at: 0,
            bounty_per_run,
            max_runs,
            completed_runs: 0,
            status: JobStatus::Active,
            created_at: env.ledger().timestamp(),
        };

        storage::set_job(&env, &job);
        storage::extend_instance_ttl(&env);

        events::emit_job_created(
            &env,
            job_id,
            &owner,
            &target_contract,
            &target_fn,
            interval_seconds,
            bounty_per_run,
        );
        Ok(job_id)
    }

    pub fn execute_job(
        env: Env,
        keeper: Address,
        job_id: u64,
    ) -> Result<ExecutionReceipt, EngineError> {
        keeper.require_auth();

        let mut job = storage::get_job(&env, job_id)?;
        if job.status != JobStatus::Active {
            return Err(EngineError::JobNotActive);
        }
        if job.max_runs > 0 && job.completed_runs >= job.max_runs {
            return Err(EngineError::MaxRunsReached);
        }

        let now = env.ledger().timestamp();
        if job.last_executed_at > 0 {
            let next_eligible = job
                .last_executed_at
                .checked_add(job.interval_seconds)
                .ok_or(EngineError::Overflow)?;
            if now < next_eligible {
                return Err(EngineError::IntervalNotElapsed);
            }
        }

        let registry_addr = storage::get_keeper_registry(&env)?;
        let registry_client = KeeperRegistryCrossClient::new(&env, &registry_addr);
        registry_client.record_execution(&keeper);

        let _result: Val = env.invoke_contract(
            &job.target_contract,
            &job.target_fn,
            job.target_args.clone(),
        );

        let vault_addr = storage::get_bounty_vault(&env)?;
        let vault_client = BountyVaultCrossClient::new(&env, &vault_addr);
        let (keeper_reward, protocol_fee) =
            vault_client.payout_execution(&job_id, &keeper, &job.bounty_per_run);

        job.last_executed_at = now;
        job.completed_runs = job
            .completed_runs
            .checked_add(1)
            .ok_or(EngineError::Overflow)?;

        if job.max_runs > 0 && job.completed_runs >= job.max_runs {
            job.status = JobStatus::Completed;
        }

        let receipt = ExecutionReceipt {
            job_id,
            keeper: keeper.clone(),
            executed_at: now,
            run_number: job.completed_runs,
            keeper_reward,
            protocol_fee,
        };

        storage::set_job(&env, &job);
        storage::set_last_receipt(&env, &receipt);
        storage::extend_instance_ttl(&env);

        events::emit_job_executed(
            &env,
            job_id,
            &keeper,
            job.completed_runs,
            keeper_reward,
            protocol_fee,
        );
        Ok(receipt)
    }

    pub fn pause_job(env: Env, owner: Address, job_id: u64) -> Result<(), EngineError> {
        owner.require_auth();

        let mut job = storage::get_job(&env, job_id)?;
        if job.owner != owner {
            return Err(EngineError::UnauthorizedOwner);
        }
        if job.status != JobStatus::Active {
            return Err(EngineError::JobNotActive);
        }

        job.status = JobStatus::Paused;
        storage::set_job(&env, &job);
        storage::extend_instance_ttl(&env);

        events::emit_job_paused(&env, job_id, &owner);
        Ok(())
    }

    pub fn resume_job(env: Env, owner: Address, job_id: u64) -> Result<(), EngineError> {
        owner.require_auth();

        let mut job = storage::get_job(&env, job_id)?;
        if job.owner != owner {
            return Err(EngineError::UnauthorizedOwner);
        }
        if job.status != JobStatus::Paused {
            return Err(EngineError::JobNotPaused);
        }

        job.status = JobStatus::Active;
        storage::set_job(&env, &job);
        storage::extend_instance_ttl(&env);

        events::emit_job_resumed(&env, job_id, &owner);
        Ok(())
    }

    pub fn cancel_job(env: Env, owner: Address, job_id: u64) -> Result<i128, EngineError> {
        owner.require_auth();

        let mut job = storage::get_job(&env, job_id)?;
        if job.owner != owner {
            return Err(EngineError::UnauthorizedOwner);
        }
        if job.status == JobStatus::Cancelled {
            return Err(EngineError::JobNotActive);
        }

        job.status = JobStatus::Cancelled;
        storage::set_job(&env, &job);

        let vault_addr = storage::get_bounty_vault(&env)?;
        let vault_client = BountyVaultCrossClient::new(&env, &vault_addr);
        let refunded = vault_client.refund_job(&job_id, &owner);

        storage::extend_instance_ttl(&env);

        events::emit_job_cancelled(&env, job_id, &owner, refunded);
        Ok(refunded)
    }

    pub fn is_executable(env: Env, job_id: u64) -> bool {
        let job = match storage::get_job(&env, job_id) {
            Ok(j) => j,
            Err(_) => return false,
        };
        if job.status != JobStatus::Active {
            return false;
        }
        if job.max_runs > 0 && job.completed_runs >= job.max_runs {
            return false;
        }

        let now = env.ledger().timestamp();
        if job.last_executed_at > 0 {
            match job.last_executed_at.checked_add(job.interval_seconds) {
                Some(next_eligible) if now < next_eligible => return false,
                None => return false,
                _ => {}
            }
        }

        let vault_addr = match storage::get_bounty_vault(&env) {
            Ok(addr) => addr,
            Err(_) => return false,
        };
        let vault_client = BountyVaultCrossClient::new(&env, &vault_addr);
        vault_client.get_job_escrow(&job_id) >= job.bounty_per_run
    }

    pub fn get_job(env: Env, job_id: u64) -> Result<Job, EngineError> {
        storage::get_job(&env, job_id)
    }

    pub fn get_last_receipt(env: Env, job_id: u64) -> Option<ExecutionReceipt> {
        storage::get_last_receipt(&env, job_id)
    }

    pub fn get_total_jobs(env: Env) -> u64 {
        storage::get_next_job_id(&env).saturating_sub(1)
    }
}
