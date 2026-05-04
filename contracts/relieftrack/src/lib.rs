#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Address, Symbol, Vec, Map
};

#[contract]
pub struct ReliefTrack;

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Disbursements, // Map<u32, Disbursement>
    Counter,
}

/// Struct representing a single disbursement
#[contracttype]
#[derive(Clone)]
pub struct Disbursement {
    pub recipient: Address,
    pub amount: i128,
}

#[contractimpl]
impl ReliefTrack {

    /// Initialize contract with NGO admin
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Counter, &0u32);
    }

    /// Record and execute a disbursement
    /// MVP: NGO sends funds → contract logs it
    pub fn disburse(env: Env, caller: Address, recipient: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        // Only NGO admin can disburse
        if caller != admin {
            panic!("Unauthorized");
        }

        let mut counter: u32 = env.storage().instance().get(&DataKey::Counter).unwrap();

        let mut records: Map<u32, Disbursement> =
            env.storage().instance().get(&DataKey::Disbursements)
            .unwrap_or(Map::new(&env));

        // Create record
        let entry = Disbursement {
            recipient: recipient.clone(),
            amount,
        };

        records.set(counter, entry);
        counter += 1;

        env.storage().instance().set(&DataKey::Disbursements, &records);
        env.storage().instance().set(&DataKey::Counter, &counter);

        // NOTE: In real deployment, integrate token transfer:
        // token::Client::new(&env, &token_id).transfer(&caller, &recipient, &amount);
    }

    /// Get a specific disbursement
    pub fn get(env: Env, id: u32) -> Disbursement {
        let records: Map<u32, Disbursement> =
            env.storage().instance().get(&DataKey::Disbursements).unwrap();

        records.get(id).unwrap()
    }

    /// Get total number of disbursements
    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }
}