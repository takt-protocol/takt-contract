use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_initialized(
    env: &Env,
    admin: &Address,
    token: &Address,
    fee_bps: u32,
    treasury: &Address,
) {
    env.events().publish(
        (symbol_short!("init"), admin.clone()),
        (token.clone(), fee_bps, treasury.clone()),
    );
}

pub fn emit_engine_set(env: &Env, admin: &Address, engine: &Address) {
    env.events()
        .publish((symbol_short!("set_eng"), admin.clone()), engine.clone());
}

pub fn emit_deposited(env: &Env, owner: &Address, amount: i128, new_balance: i128) {
    env.events().publish(
        (symbol_short!("deposit"), owner.clone()),
        (amount, new_balance),
    );
}

pub fn emit_withdrawn(env: &Env, owner: &Address, amount: i128, remaining_balance: i128) {
    env.events().publish(
        (symbol_short!("withdraw"), owner.clone()),
        (amount, remaining_balance),
    );
}

pub fn emit_job_funded(env: &Env, job_id: u64, owner: &Address, new_escrow: i128) {
    env.events().publish(
        (symbol_short!("fund_job"), job_id),
        (owner.clone(), new_escrow),
    );
}

pub fn emit_job_topped_up(env: &Env, job_id: u64, owner: &Address, new_escrow: i128) {
    env.events().publish(
        (symbol_short!("top_up"), job_id),
        (owner.clone(), new_escrow),
    );
}

pub fn emit_payout(
    env: &Env,
    job_id: u64,
    keeper: &Address,
    keeper_reward: i128,
    protocol_fee: i128,
) {
    env.events().publish(
        (symbol_short!("payout"), job_id),
        (keeper.clone(), keeper_reward, protocol_fee),
    );
}

pub fn emit_refunded(env: &Env, job_id: u64, owner: &Address, refunded_amount: i128) {
    env.events().publish(
        (symbol_short!("refund"), job_id),
        (owner.clone(), refunded_amount),
    );
}
