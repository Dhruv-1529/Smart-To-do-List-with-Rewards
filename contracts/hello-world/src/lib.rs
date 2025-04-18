#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Vec, String, log};

#[derive(Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

#[contracttype]
pub enum TaskBook {
    Tasks,
    RewardPoints,
}

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    pub fn add_task(env: Env, description: String) {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TaskBook::Tasks).unwrap_or(Vec::new(&env));
        tasks.push_back(Task {
            description,
            completed: false,
        });
        env.storage().instance().set(&TaskBook::Tasks, &tasks);
    }

    pub fn complete_task(env: Env, index: u32) {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TaskBook::Tasks).unwrap_or(Vec::new(&env));
        if let Some(mut task) = tasks.get(index) {
            if !task.completed {
                task.completed = true;
                tasks.set(index, task);
                env.storage().instance().set(&TaskBook::Tasks, &tasks);

                // Reward user
                let mut points: u32 = env.storage().instance().get(&TaskBook::RewardPoints).unwrap_or(0);
                points += 10; // give 10 points per task
                env.storage().instance().set(&TaskBook::RewardPoints, &points);
                log!(&env, "Task completed. Earned 10 points.");
            }
        }
    }

    pub fn view_tasks(env: Env) -> Vec<Task> {
        env.storage().instance().get(&TaskBook::Tasks).unwrap_or(Vec::new(&env))
    }

    pub fn view_points(env: Env) -> u32 {
        env.storage().instance().get(&TaskBook::RewardPoints).unwrap_or(0)
    }
}
