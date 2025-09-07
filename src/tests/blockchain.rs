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
        20.0,
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


