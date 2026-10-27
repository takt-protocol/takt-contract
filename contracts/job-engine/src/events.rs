use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn emit_initialized(
    env: &Env,
    admin: &Address,
    keeper_registry: &Address,
    bounty_vault: &Address,
) {
    env.events().publish(
        (symbol_short!("init"), admin.clone()),
        (keeper_registry.clone(), bounty_vault.clone()),
    );
}

pub fn emit_job_created(
    env: &Env,
    job_id: u64,
    owner: &Address,
    target_contract: &Address,
    target_fn: &Symbol,
    interval_seconds: u64,
    bounty_per_run: i128,
) {
    env.events().publish(
        (symbol_short!("job_new"), job_id),
        (
            owner.clone(),
            target_contract.clone(),
            target_fn.clone(),
            interval_seconds,
            bounty_per_run,
        ),
    );
}

pub fn emit_job_executed(
    env: &Env,
    job_id: u64,
    keeper: &Address,
    run_number: u32,
    keeper_reward: i128,
    protocol_fee: i128,
) {
    env.events().publish(
        (symbol_short!("job_exec"), job_id),
        (keeper.clone(), run_number, keeper_reward, protocol_fee),
    );
}

pub fn emit_job_paused(env: &Env, job_id: u64, owner: &Address) {
    env.events()
        .publish((symbol_short!("job_paus"), job_id), owner.clone());
}

pub fn emit_job_resumed(env: &Env, job_id: u64, owner: &Address) {
    env.events()
        .publish((symbol_short!("job_resm"), job_id), owner.clone());
}

pub fn emit_job_cancelled(env: &Env, job_id: u64, owner: &Address, refunded_amount: i128) {
    env.events().publish(
        (symbol_short!("job_cncl"), job_id),
        (owner.clone(), refunded_amount),
    );
}
