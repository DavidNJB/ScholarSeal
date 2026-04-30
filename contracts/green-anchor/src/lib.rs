#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, log};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),  // Tracks carbon credits held by an address
    Verified(Address), // Tracks if a project is authorized to mint
}

#[contract]
pub struct GreenAnchor;

#[contractimpl]
impl GreenAnchor {
    /// Registers a local project as verified to mint credits.
    pub fn verify_project(env: Env, project: Address) {
        env.storage().instance().set(&DataKey::Verified(project.clone()), &true);
        log!(&env, "Project verified successfully", project);
    }

    /// Mints carbon credits (1 token = 1 ton CO2) for a verified project.
    pub fn mint_credits(env: Env, project: Address, amount: i128) {
        project.require_auth();
        
        let is_verified: bool = env.storage().instance().get(&DataKey::Verified(project.clone())).unwrap_or(false);
        if !is_verified {
            panic!("Project not verified for carbon credit issuance");
        }

        let key = DataKey::Balance(project.clone());
        let current_balance: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(current_balance + amount));
    }

    /// Transfers credits from project to SME (The Purchase/Offset).
    pub fn offset_purchase(env: Env, buyer: Address, project: Address, amount: i128) {
        buyer.require_auth();

        let project_key = DataKey::Balance(project.clone());
        let buyer_key = DataKey::Balance(buyer.clone());

        let project_bal: i128 = env.storage().instance().get(&project_key).unwrap_or(0);
        if project_bal < amount {
            panic!("Insufficient credits available in project storage");
        }

        env.storage().instance().set(&project_key, &(project_bal - amount));
        let buyer_bal: i128 = env.storage().instance().get(&buyer_key).unwrap_or(0);
        env.storage().instance().set(&buyer_key, &(buyer_bal + amount));
        
        log!(&env, "Offset credits transferred to SME", buyer, amount);
    }

    /// Returns the credit balance for a specific account.
    pub fn get_balance(env: Env, account: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(account)).unwrap_or(0)
    }
}

mod test;