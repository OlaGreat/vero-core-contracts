use soroban_sdk::{Address, Env};

use crate::reentrancy;
use crate::types::{ContractError, DataKey, Task};

pub fn register_task(env: &Env, admin: Address, task_id: u64) -> Result<(), ContractError> {
    admin.require_auth();

    reentrancy::lock(env)?;

    let key = DataKey::Task(task_id);
    if env.storage().instance().has(&key) {
        reentrancy::unlock(env);
        return Err(ContractError::NotAuthorized);
    }

    let task = Task {
        id: task_id,
        votes: 0,
        is_done: false,
        total_weight_accrued: 0,
        is_cancelled: false,
    };
    env.storage().instance().set(&key, &task);

    reentrancy::unlock(env);
    Ok(())
}

pub fn get_task(env: &Env, task_id: u64) -> Option<Task> {
    env.storage()
        .instance()
        .get(&DataKey::Task(task_id))
}

pub fn cancel_task(env: &Env, admin: Address, task_id: u64) -> Result<(), ContractError> {
    admin.require_auth();

    reentrancy::lock(env)?;

    let key = DataKey::Task(task_id);
    let mut task: Task = match env.storage().instance().get(&key) {
        Some(t) => t,
        None => {
            reentrancy::unlock(env);
            return Err(ContractError::NotAuthorized);
        }
    };

    task.is_cancelled = true;
    env.storage().instance().set(&key, &task);

    crate::events::emit_task_cancelled(env, task_id);

    reentrancy::unlock(env);
    Ok(())
}
