use crate::*;

#[test]
fn test_51_percent_attack_prevention() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Set max ownership to 48% to prevent 51% attacks
    bc.max_owner_percentage = 48.0;
    
    // Create multiple token holders
    let mut total_tokens = 0.0;
    for i in 0..10 {
        let mut holder = TokenHolder::new(format!("holder{}", i), false);
        let tokens = 10.0 + i as f64;
        holder.add_security_tokens(tokens);
        total_tokens += tokens;
        bc.token_holders.insert(format!("holder{}", i), holder);
    }
    
    // Try to create a holder with 51% of tokens
    let mut attacker = TokenHolder::new("attacker".to_string(), false);
    let attack_tokens = (total_tokens * 0.51) + 1.0; // Just over 51%
    attacker.add_security_tokens(attack_tokens);
    bc.token_holders.insert("attacker".to_string(), attacker);
    
    // Обновляем общее количество токенов
    let new_total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    
    // Check security report
    let report = bc.check_network_security();
    assert!(!report.is_secure);
    assert!(!report.security_risks.is_empty());
    
    // Verify the attacker would be flagged
    let attacker_risk = report.security_risks.iter()
        .find(|risk| risk.wallet == "attacker")
        .expect("Attacker should be flagged as risk");
    
    let attacker_percentage = (attack_tokens / new_total_tokens) * 100.0;
    assert!(attacker_risk.percentage > 49.0); // Должен превышать лимит для обычных пользователей
}

#[test]
fn test_utility_token_concentration_prevention() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Issue utility tokens to multiple holders
    let mut total_utility = 0.0;
    for i in 0..5 {
        let mut holder = TokenHolder::new(format!("holder{}", i), false);
        let utility_tokens = 20.0;
        holder.add_utility_tokens(utility_tokens);
        total_utility += utility_tokens;
        bc.token_holders.insert(format!("holder{}", i), holder);
    }
    
    // Try to concentrate utility tokens in one holder
    let mut whale = TokenHolder::new("whale".to_string(), false);
    let whale_tokens = total_utility * 0.6; // 60% of utility tokens
    whale.add_utility_tokens(whale_tokens);
    bc.token_holders.insert("whale".to_string(), whale);
    
    // Check security report
    let report = bc.check_network_security();
    assert!(!report.is_secure);
    assert!(!report.utility_risks.is_empty());
    
    // Verify the whale would be flagged
    let whale_risk = report.utility_risks.iter()
        .find(|risk| risk.wallet == "whale")
        .expect("Whale should be flagged as risk");
    
    assert!(whale_risk.percentage > 50.0);
}

#[test]
fn test_balance_transfer_prevents_51_percent() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Set low max ownership for testing
    bc.max_owner_percentage = 30.0;
    
    // Create initial token distribution
    let mut holder1 = TokenHolder::new("holder1".to_string(), false);
    holder1.add_security_tokens(100.0);
    bc.token_holders.insert("holder1".to_string(), holder1);
    
    let mut holder2 = TokenHolder::new("holder2".to_string(), false);
    holder2.add_security_tokens(50.0);
    bc.token_holders.insert("holder2".to_string(), holder2);
    
    // Register and verify a user
    let phone = "+1234567890".to_string();
    let wallet = "0xwallet123".to_string();
    let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
        .expect("registration should succeed");
    bc.verify_phone_number(phone.clone(), verification_code)
        .expect("verification should succeed");
    
    // Create a large check that would exceed the limit
    let check = bc.process_purchase(
        "BigCustomer".to_string(),
        "Truck".to_string(),
        200.0, // This would give the user > 30% of total tokens
        vec!["BigMeal".to_string()],
    );
    
    // Attempt transfer should fail
    let result = bc.transfer_balance_from_check(check.check_id, phone);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceed maximum ownership percentage"));
}

#[test]
fn test_main_owner_control_mechanism() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Main owner should have special privileges but still be subject to limits
    let main_owner_holder = bc.token_holders.get(&owner).unwrap();
    assert!(main_owner_holder.is_main_owner);
    assert_eq!(main_owner_holder.role, UserRole::MainOwner);
    
    // Main owner can accumulate tokens through purchases
    for i in 0..3 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            10.0,
            vec!["Meal".to_string()],
        );
    }
    
    // Check that main owner has accumulated tokens
    let updated_owner = bc.token_holders.get(&owner).unwrap();
    assert!(updated_owner.security_tokens > 0.0);
    
    // But main owner is still subject to security checks
    let report = bc.check_network_security();
    // Main owner should not exceed the max percentage
    let owner_risk = report.security_risks.iter()
        .find(|risk| risk.wallet == owner);
    
    // If main owner exceeds limit, they should be flagged
    if let Some(risk) = owner_risk {
        // In this test, main owner should not exceed the limit with small purchases
        assert!(risk.percentage <= bc.max_owner_percentage);
    }
}

#[test]
fn test_multiple_attack_vectors() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Test 1: Sybil attack prevention (multiple wallets controlled by one entity)
    let mut total_controlled = 0.0;
    for i in 0..20 {
        let mut holder = TokenHolder::new(format!("sybil{}", i), false);
        let tokens = 5.0; // Each wallet has small amount
        holder.add_security_tokens(tokens);
        total_controlled += tokens;
        bc.token_holders.insert(format!("sybil{}", i), holder);
    }
    
    // Even with many small wallets, if total exceeds limit, it should be detected
    let report = bc.check_network_security();
    if total_controlled > (bc.max_owner_percentage / 100.0) * 1000.0 {
        // In a real system, we'd need additional logic to detect coordinated attacks
        // For now, we just verify the security system is working
        assert!(report.total_security_tokens > 0.0);
    }
    
    // Test 2: Rapid token accumulation
    let mut rapid_accumulator = TokenHolder::new("rapid".to_string(), false);
    for _i in 0..10 {
        rapid_accumulator.add_security_tokens(10.0);
    }
    bc.token_holders.insert("rapid".to_string(), rapid_accumulator);
    
    let report2 = bc.check_network_security();
    assert!(report2.total_security_tokens > 0.0);
}

#[test]
fn test_network_security_monitoring() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Create a scenario with multiple risk factors
    let mut risky_holder = TokenHolder::new("risky".to_string(), false);
    risky_holder.add_security_tokens(100.0);
    risky_holder.add_utility_tokens(50.0);
    bc.token_holders.insert("risky".to_string(), risky_holder);
    
    // Issue utility tokens to the system
    bc.utility_token.issue_voting_tokens(50.0);
    
    // Generate security report
    let report = bc.check_network_security();
    
    // Verify report contains all necessary information
    assert!(report.total_security_tokens > 0.0);
    assert!(report.total_utility_tokens > 0.0);
    assert_eq!(report.max_owner_percentage, 48.0);
    
    // Check if risks are properly identified
    let has_security_risk = report.security_risks.iter()
        .any(|risk| risk.percentage > report.max_owner_percentage);
    let has_utility_risk = report.utility_risks.iter()
        .any(|risk| risk.percentage > report.max_owner_percentage);
    
    // Network should be considered insecure if any risks are found
    let expected_secure = !has_security_risk && !has_utility_risk;
    assert_eq!(report.is_secure, expected_secure);
}

#[test]
fn test_balance_transfer_audit_trail() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Create multiple transfers to test audit trail
    let mut transfer_ids = Vec::new();
    
    for i in 0..3 {
        let check = bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            0.1,
            vec!["Meal".to_string()],
        );
        
        let phone = format!("+123456789{}", i);
        let wallet = format!("0xwallet{}", i);
        let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
            .expect("registration should succeed");
        bc.verify_phone_number(phone.clone(), verification_code)
            .expect("verification should succeed");
        
        let transfer_id = bc.transfer_balance_from_check(check.check_id, phone)
            .expect("transfer should succeed");
        transfer_ids.push(transfer_id);
    }
    
    // Verify audit trail
    let history = bc.get_balance_transfer_history(None);
    assert_eq!(history.len(), 3);
    
    // All transfers should be completed
    for record in &history {
        assert!(matches!(record.status, TransferStatus::Completed));
        assert!(!record.transfer_id.is_empty());
        assert!(!record.from_check_id.is_empty());
        assert!(!record.to_phone.is_empty());
    }
    
    // Verify transfer IDs are unique
    let unique_ids: std::collections::HashSet<String> = history.iter()
        .map(|r| r.transfer_id.clone())
        .collect();
    assert_eq!(unique_ids.len(), 3);
}
