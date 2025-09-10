use blockchain_project::consensus::*;

#[cfg(test)]
mod consensus_tests {
    use super::*;

    #[test]
    fn test_consensus_algorithm_creation() {
        let algorithm = ConsensusAlgorithm::new();
        assert_eq!(algorithm.min_stake, 10.0);
        assert_eq!(algorithm.block_time, 10);
        assert!(algorithm.validators.is_empty());
    }

    #[test]
    fn test_block_creation() {
        let block = Block::new(1, "previous_hash".to_string(), vec![]);
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "previous_hash");
        assert!(block.transactions.is_empty());
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "sender".to_string(),
            "receiver".to_string(),
            100.0,
            TransactionType::Transfer
        );
        assert_eq!(tx.sender, "sender");
        assert_eq!(tx.receiver, "receiver");
        assert_eq!(tx.amount, 100.0);
        assert!(matches!(tx.transaction_type, TransactionType::Transfer));
        assert!(!tx.id.is_empty());
    }

    #[test]
    fn test_validator_registration() {
        let mut algorithm = ConsensusAlgorithm::new();
        let validator = Validator::new("validator1".to_string(), 50.0);
        
        algorithm.register_validator(validator.clone()).unwrap();
        assert_eq!(algorithm.validators.len(), 1);
        assert_eq!(algorithm.validators[0].address, "validator1");
    }

    #[test]
    fn test_validator_stake_requirement() {
        let mut algorithm = ConsensusAlgorithm::new();
        let low_stake_validator = Validator::new("low_stake".to_string(), 5.0);
        
        // Should fail with insufficient stake
        let result = algorithm.register_validator(low_stake_validator);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("insufficient stake"));
    }

    #[test]
    fn test_block_validation() {
        let mut algorithm = ConsensusAlgorithm::new();
        
        // Create a valid block
        let transactions = vec![
            Transaction::new("alice".to_string(), "bob".to_string(), 10.0, TransactionType::Transfer),
        ];
        let block = Block::new(1, "genesis".to_string(), transactions);
        
        // Block should be valid
        assert!(algorithm.validate_block(&block).is_ok());
    }

    #[test]
    fn test_consensus_mechanism() {
        let mut algorithm = ConsensusAlgorithm::new();
        
        // Register validators
        let validator1 = Validator::new("validator1".to_string(), 100.0);
        let validator2 = Validator::new("validator2".to_string(), 100.0);
        let validator3 = Validator::new("validator3".to_string(), 100.0);
        
        algorithm.register_validator(validator1).unwrap();
        algorithm.register_validator(validator2).unwrap();
        algorithm.register_validator(validator3).unwrap();
        
        // Create a block
        let transactions = vec![
            Transaction::new("alice".to_string(), "bob".to_string(), 10.0, TransactionType::Transfer),
        ];
        let block = Block::new(1, "genesis".to_string(), transactions);
        
        // Test consensus
        let result = algorithm.reach_consensus(&block);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transaction_types() {
        let transfer_tx = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            10.0,
            TransactionType::Transfer
        );
        
        let stake_tx = Transaction::new(
            "alice".to_string(),
            "system".to_string(),
            50.0,
            TransactionType::Stake
        );
        
        let unstake_tx = Transaction::new(
            "alice".to_string(),
            "system".to_string(),
            25.0,
            TransactionType::Unstake
        );
        
        assert!(matches!(transfer_tx.transaction_type, TransactionType::Transfer));
        assert!(matches!(stake_tx.transaction_type, TransactionType::Stake));
        assert!(matches!(unstake_tx.transaction_type, TransactionType::Unstake));
    }

    #[test]
    fn test_block_hash_calculation() {
        let block1 = Block::new(1, "hash1".to_string(), vec![]);
        let block2 = Block::new(1, "hash2".to_string(), vec![]);
        
        // Different previous hashes should result in different block hashes
        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn test_consensus_with_insufficient_validators() {
        let mut algorithm = ConsensusAlgorithm::new();
        
        // Create a block
        let transactions = vec![
            Transaction::new("alice".to_string(), "bob".to_string(), 10.0, TransactionType::Transfer),
        ];
        let block = Block::new(1, "genesis".to_string(), transactions);
        
        // Try to reach consensus without validators
        let result = algorithm.reach_consensus(&block);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("insufficient validators"));
    }
}
