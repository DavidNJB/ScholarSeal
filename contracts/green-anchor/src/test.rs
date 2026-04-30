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