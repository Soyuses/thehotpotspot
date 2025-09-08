use crate::*;

#[test]
fn transaction_creation_generates_check() {
    let tx = Transaction::new(
        "A".to_string(),
        "B".to_string(),
        1.23,
        vec!["X".to_string()],
        1.0,
        0.1,
    );
    assert!(tx.check.is_some());
    assert!(!tx.transaction_id.is_empty());
}

#[test]
fn blockchain_genesis_and_basic_flows() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    assert_eq!(bc.chain.len(), 1); // genesis
    assert!(bc.is_chain_valid());

    // Process a purchase to give owner security tokens and queue tx
    let check = bc.process_purchase(
        "Customer".to_string(),
        "Truck".to_string(),
        50.0,
        vec!["Meal".to_string()],
    );
    assert!(!check.check_id.is_empty());
    assert!(!bc.pending_transactions.is_empty());

    // Make mining fast for tests
    bc.difficulty = 1;

    // Ensure validator has enough stake (owner gets 20 from purchase)
    bc.mine_block().expect("mining should succeed with stake >= min_stake");
    assert!(bc.chain.len() >= 2);
    assert!(bc.is_chain_valid());
}

#[test]
fn user_registration_and_phone_verification() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner);
    
    let phone = "+1234567890".to_string();
    let wallet = "0xwallet123".to_string();
    
    // Register user
    let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
        .expect("registration should succeed");
    
    assert!(!verification_code.is_empty());
    assert!(bc.authorized_users.contains_key(&phone));
    
    // Verify phone
    bc.verify_phone_number(phone.clone(), verification_code)
        .expect("verification should succeed");
    
    let user = bc.authorized_users.get(&phone).unwrap();
    assert!(user.is_verified);
}

#[test]
fn balance_transfer_from_check() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Create a purchase to generate a check (very small amount to avoid exceeding limits)
    let check = bc.process_purchase(
        "Customer".to_string(),
        "Truck".to_string(),
        0.1,
        vec!["Burger".to_string(), "Fries".to_string()],
    );
    
    // Register and verify a user
    let phone = "+1234567890".to_string();
    let wallet = "0xwallet123".to_string();
    let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
        .expect("registration should succeed");
    bc.verify_phone_number(phone.clone(), verification_code)
        .expect("verification should succeed");
    
    // Transfer balance from check
    let transfer_id = bc.transfer_balance_from_check(check.check_id.clone(), phone.clone())
        .expect("transfer should succeed");
    
    assert!(!transfer_id.is_empty());
    assert_eq!(bc.balance_transfer_history.len(), 1);
    
    // Check that tokens were transferred
    let to_holder = bc.token_holders.get(&wallet).unwrap();
    assert_eq!(to_holder.security_tokens, 10.0);
    assert_eq!(to_holder.utility_tokens, 1.0);
    
    // Check that original check is marked as claimed
    let owner_holder = bc.token_holders.get(&owner).unwrap();
    let original_check = owner_holder.checks.iter().find(|c| c.check_id == check.check_id).unwrap();
    assert!(original_check.is_claimed);
}

#[test]
fn balance_transfer_security_limits() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Set a low max ownership percentage for testing
    bc.max_owner_percentage = 10.0;
    bc.max_customer_percentage = 10.0; // Также ограничиваем лимит для клиентов
    
    // Create multiple purchases to accumulate tokens
    for i in 0..5 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            20.0,
            vec!["Meal".to_string()],
        );
    }
    
    // Register and verify a user
    let phone = "+1234567890".to_string();
    let wallet = "0xwallet123".to_string();
    let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
        .expect("registration should succeed");
    bc.verify_phone_number(phone.clone(), verification_code)
        .expect("verification should succeed");
    
    // Try to transfer a large amount that would exceed the limit
    let check = bc.process_purchase(
        "BigCustomer".to_string(),
        "Truck".to_string(),
        200.0, // This should exceed the 10% limit
        vec!["BigMeal".to_string()],
    );
    
    let result = bc.transfer_balance_from_check(check.check_id, phone);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceed maximum ownership percentage"));
}

#[test]
fn network_security_report() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Create some token holders with different amounts
    let mut holder1 = TokenHolder::new("holder1".to_string(), false);
    holder1.add_security_tokens(30.0);
    bc.token_holders.insert("holder1".to_string(), holder1);
    
    let mut holder2 = TokenHolder::new("holder2".to_string(), false);
    holder2.add_security_tokens(20.0);
    bc.token_holders.insert("holder2".to_string(), holder2);
    
    // Generate report
    let report = bc.check_network_security();
    
    assert_eq!(report.total_security_tokens, 50.0);
    assert_eq!(report.max_owner_percentage, 48.0);
    assert!(report.is_secure); // No one exceeds 48%
    
    // Add a holder with too many tokens
    let mut holder3 = TokenHolder::new("holder3".to_string(), false);
    holder3.add_security_tokens(200.0); // This will be > 49% of total (80%)
    bc.token_holders.insert("holder3".to_string(), holder3);
    
    let report_risky = bc.check_network_security();
    assert!(!report_risky.is_secure);
    assert!(!report_risky.security_risks.is_empty());
}

#[test]
fn balance_transfer_history() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Create multiple transfers (smaller amounts to avoid exceeding limits)
    for i in 0..3 {
        let check = bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            0.1 + i as f64 * 0.01,
            vec!["Meal".to_string()],
        );
        
        let phone = format!("+123456789{}", i);
        let wallet = format!("0xwallet{}", i);
        let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
            .expect("registration should succeed");
        bc.verify_phone_number(phone.clone(), verification_code)
            .expect("verification should succeed");
        
        bc.transfer_balance_from_check(check.check_id, phone)
            .expect("transfer should succeed");
    }
    
    let history = bc.get_balance_transfer_history(Some(2));
    assert_eq!(history.len(), 2); // Limited to 2 records
    assert!(history[0].timestamp >= history[1].timestamp); // Sorted by timestamp desc
}


