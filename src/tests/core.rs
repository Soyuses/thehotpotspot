use crate::*;

#[test]
fn utility_token_issuing_increases_supply_and_power() {
    let mut token = UtilityToken::new("VOTE".to_string());
    let power1 = token.issue_voting_tokens(10.0);
    let power2 = token.issue_voting_tokens(5.5);
    assert_eq!(power1, 10.0);
    assert_eq!(power2, 5.5);
    assert!((token.total_supply - 15.5).abs() < f64::EPSILON);
}

#[test]
fn user_role_from_percentage_mapping() {
    assert_eq!(UserRole::from_percentage(0.0), UserRole::Unauthorized);
    assert_eq!(UserRole::from_percentage(1.0), UserRole::Unauthorized);
    assert_eq!(UserRole::from_percentage(1.0001), UserRole::Starter);
    assert_eq!(UserRole::from_percentage(5.1), UserRole::MiddlePlayer);
    assert_eq!(UserRole::from_percentage(10.1), UserRole::BigStack);
}

#[test]
fn check_creation_sets_expected_fields() {
    let items = vec!["Burger".to_string(), "Fries".to_string()];
    let check = Check::new(12.34, items.clone());
    assert!(!check.check_id.is_empty());
    assert!(check.qr_code.starts_with("QR_CODE_"));
    assert_eq!(check.amount, 12.34);
    assert_eq!(check.food_items, items);
    assert!(!check.is_activated);
    assert!(check.blockchain_account.starts_with("0x"));
}

#[test]
fn blockchain_account_activation_and_listing() {
    let mut account = BlockchainAccount::new("0xabc".to_string());
    assert!(matches!(account.status, AccountStatus::Sleep));

    let personal_data = PersonalData {
        name: "John".to_string(),
        email: "john@example.com".to_string(),
        phone: "+123".to_string(),
        wallet_address: Some("0xabc".to_string()),
    };

    account.activate(personal_data).expect("activation should succeed");
    assert!(matches!(account.status, AccountStatus::Active));
    assert!(account.activated_timestamp.is_some());

    account.list_for_sale().expect("listing active account should succeed");
    assert!(matches!(account.status, AccountStatus::ForSale));
}

#[test]
fn token_holder_add_check_and_activate_account_flow() {
    let mut holder = TokenHolder::new("holder1".to_string(), false);
    let check = Check::new(5.0, vec!["Item".to_string()]);
    let check_id = check.check_id.clone();
    let activation_code = check.activation_code.clone();
    let blockchain_account = check.blockchain_account.clone();
    holder.add_check(check);

    assert_eq!(holder.checks.len(), 1);
    assert!(holder.blockchain_accounts.contains_key(&blockchain_account));

    let personal = PersonalData {
        name: "Jane".to_string(),
        email: "jane@example.com".to_string(),
        phone: "+987".to_string(),
        wallet_address: Some("0xjane".to_string()),
    };

    holder
        .activate_account(&check_id, &activation_code, personal)
        .expect("activation should succeed");

    let acc = holder.blockchain_accounts.get(&blockchain_account).unwrap();
    assert!(matches!(acc.status, AccountStatus::Active));
}


