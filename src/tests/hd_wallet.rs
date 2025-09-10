use blockchain_project::hd_wallet::*;

#[cfg(test)]
mod hd_wallet_tests {
    use super::*;

    #[test]
    fn test_hd_wallet_manager_creation() {
        let manager = HDWalletManager::new();
        assert!(manager.wallets.is_empty());
        assert_eq!(manager.wallet_count, 0);
    }

    #[test]
    fn test_hd_wallet_creation() {
        let wallet = HDWallet::new("test_wallet".to_string(), WalletType::Standard);
        
        assert_eq!(wallet.name, "test_wallet");
        assert!(matches!(wallet.wallet_type, WalletType::Standard));
        assert!(matches!(wallet.status, WalletStatus::Active));
        assert!(!wallet.master_key.is_empty());
        assert_eq!(wallet.derived_keys.len(), 0);
    }

    #[test]
    fn test_check_wallet_creation() {
        let check_wallet = CheckWallet::new("check_123".to_string());
        
        assert_eq!(check_wallet.check_id, "check_123");
        assert!(matches!(check_wallet.status, WalletStatus::Active));
        assert!(!check_wallet.private_key.is_empty());
        assert!(!check_wallet.public_key.is_empty());
        assert_eq!(check_wallet.balance, 0.0);
    }

    #[test]
    fn test_wallet_type_enum() {
        let standard_wallet = WalletType::Standard;
        let check_wallet = WalletType::Check;
        let franchise_wallet = WalletType::Franchise;
        
        assert!(matches!(standard_wallet, WalletType::Standard));
        assert!(matches!(check_wallet, WalletType::Check));
        assert!(matches!(franchise_wallet, WalletType::Franchise));
    }

    #[test]
    fn test_wallet_status_enum() {
        let active_status = WalletStatus::Active;
        let inactive_status = WalletStatus::Inactive;
        let locked_status = WalletStatus::Locked;
        
        assert!(matches!(active_status, WalletStatus::Active));
        assert!(matches!(inactive_status, WalletStatus::Inactive));
        assert!(matches!(locked_status, WalletStatus::Locked));
    }

    #[test]
    fn test_wallet_statistics_creation() {
        let stats = WalletStatistics::new();
        
        assert_eq!(stats.total_wallets, 0);
        assert_eq!(stats.active_wallets, 0);
        assert_eq!(stats.inactive_wallets, 0);
        assert_eq!(stats.locked_wallets, 0);
        assert_eq!(stats.total_balance, 0.0);
    }

    #[test]
    fn test_hd_wallet_error_types() {
        let creation_error = HDWalletError::CreationFailed("Failed to create wallet".to_string());
        let derivation_error = HDWalletError::KeyDerivationFailed("Failed to derive key".to_string());
        let not_found_error = HDWalletError::WalletNotFound("wallet123".to_string());
        let invalid_input_error = HDWalletError::InvalidInput("Invalid input".to_string());
        
        match creation_error {
            HDWalletError::CreationFailed(msg) => assert_eq!(msg, "Failed to create wallet"),
            _ => panic!("Wrong error type"),
        }
        
        match derivation_error {
            HDWalletError::KeyDerivationFailed(msg) => assert_eq!(msg, "Failed to derive key"),
            _ => panic!("Wrong error type"),
        }
        
        match not_found_error {
            HDWalletError::WalletNotFound(id) => assert_eq!(id, "wallet123"),
            _ => panic!("Wrong error type"),
        }
        
        match invalid_input_error {
            HDWalletError::InvalidInput(msg) => assert_eq!(msg, "Invalid input"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_wallet_manager_operations() {
        let mut manager = HDWalletManager::new();
        
        // Create a wallet
        let wallet = HDWallet::new("test_wallet".to_string(), WalletType::Standard);
        let wallet_id = wallet.id.clone();
        
        // Add wallet to manager
        manager.add_wallet(wallet).unwrap();
        assert_eq!(manager.wallet_count, 1);
        assert!(manager.wallets.contains_key(&wallet_id));
        
        // Get wallet from manager
        let retrieved_wallet = manager.get_wallet(&wallet_id).unwrap();
        assert_eq!(retrieved_wallet.name, "test_wallet");
        
        // Remove wallet from manager
        manager.remove_wallet(&wallet_id).unwrap();
        assert_eq!(manager.wallet_count, 0);
        assert!(!manager.wallets.contains_key(&wallet_id));
    }

    #[test]
    fn test_wallet_key_derivation() {
        let mut wallet = HDWallet::new("test_wallet".to_string(), WalletType::Standard);
        
        // Derive a new key
        let derived_key = wallet.derive_key(0).unwrap();
        assert_eq!(wallet.derived_keys.len(), 1);
        assert_eq!(wallet.derived_keys[0], derived_key);
        
        // Derive another key
        let derived_key2 = wallet.derive_key(1).unwrap();
        assert_eq!(wallet.derived_keys.len(), 2);
        assert_ne!(derived_key, derived_key2);
    }

    #[test]
    fn test_check_wallet_operations() {
        let mut check_wallet = CheckWallet::new("check_123".to_string());
        
        // Test balance operations
        check_wallet.add_balance(100.0);
        assert_eq!(check_wallet.balance, 100.0);
        
        check_wallet.subtract_balance(30.0);
        assert_eq!(check_wallet.balance, 70.0);
        
        // Test status changes
        check_wallet.set_status(WalletStatus::Locked);
        assert!(matches!(check_wallet.status, WalletStatus::Locked));
    }

    #[test]
    fn test_wallet_statistics_updates() {
        let mut stats = WalletStatistics::new();
        
        // Simulate wallet operations
        stats.total_wallets = 10;
        stats.active_wallets = 8;
        stats.inactive_wallets = 1;
        stats.locked_wallets = 1;
        stats.total_balance = 1000.0;
        
        assert_eq!(stats.total_wallets, 10);
        assert_eq!(stats.active_wallets, 8);
        assert_eq!(stats.inactive_wallets, 1);
        assert_eq!(stats.locked_wallets, 1);
        assert_eq!(stats.total_balance, 1000.0);
    }

    #[test]
    fn test_wallet_serialization() {
        let wallet = HDWallet::new("test_wallet".to_string(), WalletType::Standard);
        let json = serde_json::to_string(&wallet).unwrap();
        let deserialized: HDWallet = serde_json::from_str(&json).unwrap();
        
        assert_eq!(wallet.name, deserialized.name);
        assert_eq!(wallet.wallet_type, deserialized.wallet_type);
        assert_eq!(wallet.status, deserialized.status);
    }

    #[test]
    fn test_check_wallet_serialization() {
        let check_wallet = CheckWallet::new("check_123".to_string());
        let json = serde_json::to_string(&check_wallet).unwrap();
        let deserialized: CheckWallet = serde_json::from_str(&json).unwrap();
        
        assert_eq!(check_wallet.check_id, deserialized.check_id);
        assert_eq!(check_wallet.status, deserialized.status);
        assert_eq!(check_wallet.balance, deserialized.balance);
    }
}
