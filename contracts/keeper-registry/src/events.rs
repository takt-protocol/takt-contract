use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_initialized(
    env: &Env,
    admin: &Address,
    stake_token: &Address,
    min_stake: i128,
    unbond_period: u64,
) {
    env.events().publish(
        (symbol_short!("init"), admin.clone()),
        (stake_token.clone(), min_stake, unbond_period),
    );
}

pub fn emit_engine_set(env: &Env, admin: &Address, engine: &Address) {
    env.events()
        .publish((symbol_short!("set_eng"), admin.clone()), engine.clone());
}

pub fn emit_keeper_registered(env: &Env, keeper: &Address, stake_amount: i128) {
    env.events()
        .publish((symbol_short!("reg_kpr"), keeper.clone()), stake_amount);
}

pub fn emit_stake_increased(env: &Env, keeper: &Address, new_total: i128) {
    env.events()
        .publish((symbol_short!("inc_stk"), keeper.clone()), new_total);
}

pub fn emit_unbond_requested(env: &Env, keeper: &Address, unlock_time: u64) {
    env.events()
        .publish((symbol_short!("unbond"), keeper.clone()), unlock_time);
}

pub fn emit_stake_withdrawn(env: &Env, keeper: &Address, amount: i128) {
    env.events()
        .publish((symbol_short!("withdraw"), keeper.clone()), amount);
}

pub fn emit_execution_recorded(env: &Env, keeper: &Address, total_executions: u64) {
    env.events().publish(
        (symbol_short!("exec_rec"), keeper.clone()),
        total_executions,
    );
}

pub fn emit_keeper_slashed(env: &Env, keeper: &Address, slash_amount: i128, remaining_stake: i128) {
    env.events().publish(
        (symbol_short!("slashed"), keeper.clone()),
        (slash_amount, remaining_stake),
    );
}
