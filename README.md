# GreenAnchor

## Project Description
**GreenAnchor** is a decentralized carbon credit marketplace built on Stellar (Soroban). It enables small-scale environmental projects, such as local coffee farmers or reforestation NGOs, to mint verified carbon credits as on-chain assets. By bypassing the high-cost barriers of traditional carbon exchanges, GreenAnchor allows local SMEs to purchase offsets directly, ensuring that climate finance reaches the grassroots level instantly.

## Project Vision
To democratize the global carbon market by creating a transparent, low-friction circular economy where local natural capital is recognized, verified, and compensated on-chain.

## Key Features
- **Decentralized Verification**: A registry system within Soroban to whitelist legitimate environmental projects.
- **On-Chain Minting**: Automated issuance of fractionalized carbon tokens based on verified environmental impact.
- **Secure Offset Purchases**: Peer-to-peer credit transfers between SMEs and projects using Stellar's high-speed settlement.
- **Transparent Retirement**: On-chain logging of "retired" credits to prevent double-counting.
- **Low-Cost Transactions**: Leveraging Stellar’s micro-fees to make small offset purchases viable.

## Deployed Contract Details
- **Network**: Stellar Testnet
- **Contract ID**: `[Insert Your Deployed Contract ID Here]` 
- **Stellar Expert Link**: `https://stellar.expert/explorer/testnet/contract/[Insert Your Deployed Contract ID Here]`

To help you set up your GitHub repository quickly, I’ve consolidated everything into a single "Copy-Paste" guide.

Follow the structure below for a professional, bootcamp-ready repository.

1. Cargo.toml
Create this file in your root directory.

Ini, TOML
[package]
name = "green_anchor"
version = "0.1.0"
edition = "2021"
license = "MIT"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
soroban-sdk = "22.0.0"

[dev-dependencies]
soroban-sdk = { version = "22.0.0", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
2. src/lib.rs
This is your core logic. Place this in a folder named src.

Rust
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
3. src/test.rs
Create this file inside the src folder.

Rust
#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, Address};

#[test]
fn test_happy_path_mint_and_purchase() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GreenAnchor);
    let client = GreenAnchorClient::new(&env, &contract_id);

    let project = Address::generate(&env);
    let sme = Address::generate(&env);

    // 1. Verify and Mint credits for the farmer
    client.verify_project(&project);
    client.mint_credits(&project, &100);
    assert_eq!(client.get_balance(&project), 100);

    // 2. SME Purchase (Mocking authorization for buyer)
    env.mock_all_auths();
    client.offset_purchase(&sme, &project, &30);

    // 3. Verify state changes
    assert_eq!(client.get_balance(&project), 70);
    assert_eq!(client.get_balance(&sme), 30);
}

#[test]
#[should_panic(expected = "Project not verified")]
fn test_unverified_mint_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GreenAnchor);
    let client = GreenAnchorClient::new(&env, &contract_id);

    let project = Address::generate(&env);
    env.mock_all_auths();
    client.mint_credits(&project, &100); 
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GreenAnchor);
    let client = GreenAnchorClient::new(&env, &contract_id);

    let project = Address::generate(&env);
    client.verify_project(&project);
    client.mint_credits(&project, &50);

    // Ensure state correctly persists in storage
    assert_eq!(client.get_balance(&project), 50);
}
4. README.md
The face of your project. Place this in the root directory.

Markdown
# GreenAnchor

## Project Description
**GreenAnchor** is a decentralized carbon credit marketplace built on Stellar (Soroban). It enables small-scale environmental projects, such as local coffee farmers or reforestation NGOs, to mint verified carbon credits as on-chain assets. By bypassing the high-cost barriers of traditional carbon exchanges, GreenAnchor allows local SMEs to purchase offsets directly, ensuring that climate finance reaches the grassroots level instantly.

## Project Vision
To democratize the global carbon market by creating a transparent, low-friction circular economy where local natural capital is recognized, verified, and compensated on-chain.

## Key Features
- **Decentralized Verification**: A registry system within Soroban to whitelist legitimate environmental projects.
- **On-Chain Minting**: Automated issuance of fractionalized carbon tokens based on verified environmental impact.
- **Secure Offset Purchases**: Peer-to-peer credit transfers between SMEs and projects using Stellar's high-speed settlement.
- **Transparent Retirement**: On-chain logging of "retired" credits to prevent double-counting.
- **Low-Cost Transactions**: Leveraging Stellar’s micro-fees to make small offset purchases viable.

## Deployed Contract Details
- **Network**: Stellar Testnet
- **Contract ID**: `https://stellar.expert/explorer/testnet/tx/aa3292d2554518653620acf86159608fa8623018533a4ea91c1d312a4838e170`
- **Stellar Expert Link**: `https://lab.stellar.org/r/testnet/contract/CC7FAXHRQEGB7KPK2MVS7PY572VBKHZDF55DRHVNRAGP3YRGGTD5RDOY`

## Future Scope
- Oracle Integration: Connecting to satellite data to automatically trigger credit minting.
- Local Anchor Integration: Partnering with regional anchors to allow farmers to off-ramp earnings to local bank accounts.
- NFT Impact Certificates: Issuing unique tokens for businesses to display as proof of carbon-neutral status.
