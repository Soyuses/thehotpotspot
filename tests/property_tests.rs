use proptest::prelude::*;
use blockchain_project::franchise_network::{FranchiseNetwork, FranchiseNode, NodeType};
use blockchain_project::hd_wallet::{HDWalletManager, WalletType};
use blockchain_project::kyc_aml::{KYCAmlManager, KYCLevel, DocumentType};
use std::collections::HashMap;

proptest! {
    #[test]
    fn test_franchise_network_consistency(
        nodes in prop::collection::vec(
            (1u64..1000u64, prop::sample::select(vec![NodeType::Owner, NodeType::Franchise])),
            1..50
        )
    ) {
        let mut network = FranchiseNetwork::new("master_owner".to_string());
        
        // Добавляем узлы
        for (node_id, node_type) in nodes {
            let node = FranchiseNode {
                node_id: node_id.to_string(),
                owner_address: format!("owner_{}", node_id),
                city: format!("City_{}", node_id),
                active: true,
                registered_at: std::time::SystemTime::now(),
                pos_systems: vec![],
            };
            
            network.register_node(node.node_id.clone(), node.owner_address.clone(), node.city.clone());
        }
        
        // Проверяем, что все узлы добавлены
        let stats = network.get_network_stats();
        assert_eq!(stats.total_nodes, nodes.len() as u32);
        
        // Проверяем, что каждый узел можно найти
        for (node_id, _) in &nodes {
            assert!(network.get_node_info(&node_id.to_string()).is_some());
        }
    }

    #[test]
    fn test_wallet_generation_consistency(
        seed_phrase in "[a-z ]{10,100}",
        wallet_count in 1u32..20u32
    ) {
        let mut wallet_manager = HDWalletManager::new("test_seed".to_string());
        
        // Генерируем кошельки
        let mut wallets = Vec::new();
        for i in 0..wallet_count {
            let wallet_type = if i % 2 == 0 { WalletType::Node } else { WalletType::Check };
            let wallet = wallet_manager.generate_node_wallet(&seed_phrase, wallet_type);
            wallets.push(wallet);
        }
        
        // Проверяем, что все кошельки уникальны
        let addresses: Vec<String> = wallets.iter().map(|w| w.address.clone()).collect();
        let unique_addresses: std::collections::HashSet<String> = addresses.iter().cloned().collect();
        assert_eq!(addresses.len(), unique_addresses.len());
        
        // Проверяем, что все кошельки имеют валидные адреса
        for wallet in &wallets {
            assert!(!wallet.address.is_empty());
            assert!(wallet.address.len() > 20); // Минимальная длина адреса
        }
    }

    #[test]
    fn test_kyc_process_consistency(
        user_data in prop::collection::vec(
            (
                "[a-zA-Z]{3,20}",
                "[a-zA-Z]{3,20}",
                "[a-zA-Z]{2,10}",
                prop::sample::select(vec![KYCLevel::Basic, KYCLevel::Enhanced, KYCLevel::Premium])
            ),
            1..10
        )
    ) {
        let mut kyc_manager = KYCAmlManager::new();
        
        for (first_name, last_name, nationality, kyc_level) in user_data {
            let user_id = format!("user_{}_{}", first_name, last_name);
            
            // Регистрируем пользователя
            let user_data = UserRegistrationData {
                user_id: user_id.clone(),
                email: format!("{}@example.com", user_id),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                nationality: Some(nationality.clone()),
            };
            kyc_manager.register_user(user_data).unwrap();
            
            // Начинаем KYC процесс
            kyc_manager.start_kyc_process(&user_id, kyc_level.clone()).unwrap();
            
            // Проверяем, что пользователь зарегистрирован
            let user = kyc_manager.get_user(&user_id).unwrap();
            let user_data = user;
            assert_eq!(user_data.first_name, first_name);
            assert_eq!(user_data.last_name, last_name);
            assert_eq!(user_data.kyc_level, kyc_level);
        }
    }

    #[test]
    fn test_token_arithmetic_consistency(
        amounts in prop::collection::vec(1u128..1000000u128, 1..100)
    ) {
        let mut total = 0u128;
        
        for amount in amounts {
            // Проверяем, что сложение не вызывает переполнение
            let new_total = total.saturating_add(amount);
            assert!(new_total >= total);
            
            // Проверяем, что деление работает корректно
            if amount > 0 {
                let percentage = (amount * 100) / (new_total.max(1));
                assert!(percentage <= 100);
            }
            
            total = new_total;
        }
        
        // Проверяем, что общая сумма не превышает разумные пределы
        assert!(total <= u128::MAX / 2);
    }

    #[test]
    fn test_string_validation_consistency(
        input in "[a-zA-Z0-9@._-]{1,100}"
    ) {
        // Проверяем валидацию email
        let is_valid_email = input.contains('@') && 
                            input.contains('.') && 
                            input.len() > 5 && 
                            input.len() < 100;
        
        if is_valid_email {
            // Если это похоже на email, проверяем базовую структуру
            let parts: Vec<&str> = input.split('@').collect();
            assert_eq!(parts.len(), 2);
            assert!(!parts[0].is_empty());
            assert!(!parts[1].is_empty());
        }
        
        // Проверяем, что строка не содержит недопустимых символов
        assert!(!input.contains(' '));
        assert!(!input.contains('\n'));
        assert!(!input.contains('\r'));
        assert!(!input.contains('\t'));
    }
}

// QuickCheck тесты для дополнительной проверки
#[cfg(test)]
mod quickcheck_tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    // use blockchain_project::config::Amount; // Amount не существует в config

    #[quickcheck]
    fn test_amount_conversion_roundtrip(amount: u128) -> TestResult {
        if amount > 1_000_000_000_000_000_000 {
            return TestResult::discard();
        }
        
        let gel_amount = Amount::from_subunits(amount);
        let converted_back = gel_amount.to_subunits();
        
        TestResult::from_bool(amount == converted_back)
    }

    #[quickcheck]
    fn test_amount_formatting_consistency(amount: u128) -> TestResult {
        if amount > 1_000_000_000_000_000_000 {
            return TestResult::discard();
        }
        
        let gel_amount = Amount::from_subunits(amount);
        let formatted = gel_amount.to_gel_string();
        
        // Проверяем, что форматированная строка содержит только цифры, точку и пробелы
        let is_valid = formatted.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ' ');
        
        TestResult::from_bool(is_valid)
    }
}
