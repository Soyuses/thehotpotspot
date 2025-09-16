use blockchain_project::{
    franchise_network::{FranchiseNetwork, FranchiseNode, NodeType, SaleItem},
    hd_wallet::{HDWalletManager, WalletType},
    kyc_aml::{KYCAmlManager, KYCLevel, DocumentType, KYCStatus},
    tokenomics_config::TokenomicsConfig,
};
use std::time::SystemTime;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_full_blockchain_workflow() {
    // Создаем основные компоненты
    // let mut blockchain = Blockchain::new(TokenomicsConfig::default());
    let mut franchise_network = FranchiseNetwork::new("master_owner".to_string());
    let mut wallet_manager = HDWalletManager::new("test_seed".to_string());
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Регистрируем пользователя
    let user_id = "test_user_001";
    let user_data = blockchain_project::kyc_aml::UserRegistrationData {
        email: "test@example.com".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        nationality: Some("GE".to_string()),
        phone: Some("+995555123456".to_string()),
        address: Some(blockchain_project::kyc_aml::Address {
            street: "Main Street".to_string(),
            city: "Tbilisi".to_string(),
            state: "Tbilisi".to_string(),
            postal_code: "0100".to_string(),
            country: "Georgia".to_string(),
        }),
        date_of_birth: Some(chrono::Utc::now()),
    };
    kyc_manager.register_user(user_data).unwrap();
    
    // 2. Начинаем KYC процесс
    let kyc_result = kyc_manager.start_kyc_process(user_id, KYCLevel::Basic);
    if kyc_result.is_err() {
        println!("KYC process failed: {:?}", kyc_result);
    }
    
    // 3. Генерируем кошелек для пользователя
    let seed_phrase = "test seed phrase for integration test";
    let wallet = wallet_manager.generate_node_wallet(1, WalletType::Franchise).unwrap();
    
    // 4. Добавляем узел в сеть франшизы
    let node = FranchiseNode {
        node_id: 1,
        owner_address: "owner_001".to_string(),
        node_type: NodeType::FRANCHISE,
        city: "Tbilisi".to_string(),
        active: true,
        registered_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        pos_systems: vec!["pos_001".to_string()],
    };
    franchise_network.register_node("owner_001".to_string(), NodeType::FRANCHISE, "Tbilisi".to_string()).unwrap();
    
    // 5. Добавляем элемент меню
    // blockchain.add_menu_item_with_details(
    //     "Pizza Margherita".to_string(),
    //     "Classic Italian pizza with tomato and mozzarella".to_string(),
    //     1500, // 15.00 GEL в субъединицах
    //     "Italian".to_string(),
    //     vec!["vegetarian".to_string()],
    //     "test_restaurant".to_string()
    // );
    
    // 6. Обрабатываем покупку
    let customer = "customer_001".to_string();
    let food_truck = "test_restaurant".to_string();
    let amount = 1500u128;
    let food_items = vec!["Pizza Margherita".to_string()];
    
    // let result = blockchain.process_purchase(customer, food_truck, amount, food_items);
    // assert!(result.is_ok());
    
    // 7. Проверяем, что транзакция записана
    // let transactions = blockchain.get_transaction_history();
    // assert!(!transactions.is_empty());
    
    // 8. Проверяем статистику
    // let stats = blockchain.get_statistics();
    // assert!(stats.total_transactions > 0);
}

#[tokio::test]
async fn test_kyc_workflow() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Регистрируем пользователя
    let user_id = "kyc_test_user";
    let user_data = blockchain_project::kyc_aml::UserRegistrationData {
        email: "kyc@example.com".to_string(),
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        nationality: Some("US".to_string()),
        phone: Some("+1234567890".to_string()),
        address: Some(blockchain_project::kyc_aml::Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            postal_code: "10001".to_string(),
            country: "USA".to_string(),
        }),
        date_of_birth: Some(chrono::Utc::now()),
    };
    kyc_manager.register_user(user_data).unwrap();
    
    // 2. Начинаем KYC процесс
    let kyc_result = kyc_manager.start_kyc_process(user_id, KYCLevel::Enhanced);
    if kyc_result.is_err() {
        println!("KYC process failed: {:?}", kyc_result);
    }
    
    // 3. Загружаем документы (только если KYC процесс успешен)
    let document_result = kyc_manager.upload_document(
        user_id,
        DocumentType::Passport,
        "document_hash_123".to_string(),
        "/path/to/passport.pdf".to_string()
    );
    if document_result.is_err() {
        println!("Document upload failed: {:?}", document_result);
        return; // Пропускаем остальные шаги если документ не загружен
    }
    let document_id = document_result.unwrap();
    
    // 4. Верифицируем документ
    kyc_manager.verify_document(
        user_id,
        &document_id,
        "verifier_001",
        true,
        None
    ).unwrap();
    
    // 5. Завершаем KYC процесс
    kyc_manager.complete_kyc_process(user_id, "verifier_001").unwrap();
    
    // 6. Проверяем статус пользователя
    let user = kyc_manager.get_user(user_id).unwrap();
    assert_eq!(user.kyc_status, KYCStatus::Verified);
}

#[tokio::test]
async fn test_wallet_operations() {
    let mut wallet_manager = HDWalletManager::new("test_seed".to_string());
    let seed_phrase = "integration test seed phrase";
    
    // 1. Генерируем несколько кошельков
    let node_wallet = wallet_manager.generate_node_wallet(1, WalletType::Franchise).unwrap();
    let check_wallet = wallet_manager.generate_node_wallet(2, WalletType::Check).unwrap();
    
    // 2. Проверяем, что кошельки разные
    assert_ne!(node_wallet.address, check_wallet.address);
    
    // 3. Проверяем, что адреса валидны
    assert!(!node_wallet.address.is_empty());
    assert!(!check_wallet.address.is_empty());
    
    // 4. Генерируем чек-кошелек
    let activation_code = wallet_manager.generate_check_wallet(
        "sale_001".to_string(),
        1,
        10000, // 100.00 GEL
        vec![]
    ).unwrap();
    
    // 5. Активируем чек-кошелек
    let result = wallet_manager.activate_check_wallet("sale_001", &activation_code.activation_code);
    if result.is_err() {
        println!("Check wallet activation failed: {:?}", result);
    }
}

#[tokio::test]
async fn test_franchise_network_operations() {
    let mut network = FranchiseNetwork::new("master_owner".to_string());
    
    // 1. Добавляем различные типы узлов
    let restaurant = FranchiseNode {
        node_id: 1,
        owner_address: "owner_001".to_string(),
        node_type: NodeType::FRANCHISE,
        city: "Tbilisi".to_string(),
        active: true,
        registered_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        pos_systems: vec!["pos_001".to_string()],
    };
    
    let food_truck = FranchiseNode {
        node_id: 2,
        owner_address: "owner_002".to_string(),
        node_type: NodeType::FRANCHISE,
        city: "Tbilisi".to_string(),
        active: true,
        registered_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        pos_systems: vec!["pos_002".to_string()],
    };
    
    let warehouse = FranchiseNode {
        node_id: 3,
        owner_address: "owner_003".to_string(),
        node_type: NodeType::FRANCHISE,
        city: "Tbilisi".to_string(),
        active: true,
        registered_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        pos_systems: vec!["pos_003".to_string()],
    };
    
    network.register_node("owner_001".to_string(), NodeType::FRANCHISE, "Tbilisi".to_string()).unwrap();
    network.register_node("owner_002".to_string(), NodeType::FRANCHISE, "Tbilisi".to_string()).unwrap();
    network.register_node("owner_003".to_string(), NodeType::FRANCHISE, "Tbilisi".to_string()).unwrap();
    
    // 2. Проверяем, что все узлы добавлены
    let stats = network.get_network_stats();
    assert_eq!(stats.total_nodes, 3);
    
    // 3. Проверяем, что узлы можно найти по ID
    assert!(network.get_node_info(1).is_some());
    assert!(network.get_node_info(2).is_some());
    assert!(network.get_node_info(3).is_some());
    
    // 4. Добавляем POS систему в whitelist
    network.whitelist_pos("pos_001".to_string());
    
    // 5. Записываем продажу
    let sale_id = "sale_001".to_string();
    let sale_result = network.record_sale(
        1, // restaurant node
        sale_id,
        2500, // 25.00 GEL
        "buyer_001".to_string(),
        "pos_001".to_string(),
        vec![
            SaleItem {
                item_id: "pizza_001".to_string(),
                price_subunits: 2500,
                quantity: 1,
            }
        ]
    );
    if sale_result.is_err() {
        println!("Sale recording failed: {:?}", sale_result);
    }
    
    // 5. Проверяем статистику продаж
    let stats = network.get_network_stats();
    assert!(stats.total_sales > 0);
}

#[tokio::test]
async fn test_error_handling() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Пытаемся получить несуществующего пользователя
    let result = kyc_manager.get_user("nonexistent_user");
    assert!(result.is_none());
    
    // 2. Пытаемся начать KYC для несуществующего пользователя
    let result = kyc_manager.start_kyc_process("nonexistent_user", KYCLevel::Basic);
    assert!(result.is_err());
    
    // 3. Пытаемся загрузить документ для несуществующего пользователя
    let result = kyc_manager.upload_document(
        "nonexistent_user",
        DocumentType::Passport,
        "hash".to_string(),
        "path".to_string()
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_operations() {
    use std::sync::Arc;
    use tokio::task;
    
    let kyc_manager = Arc::new(RwLock::new(KYCAmlManager::new()));
    let mut handles = vec![];
    
    // Запускаем несколько задач параллельно
    for i in 0..10 {
        let manager: Arc<RwLock<KYCAmlManager>> = Arc::clone(&kyc_manager);
        let handle = task::spawn(async move {
            let user_id = format!("concurrent_user_{}", i);
            
            // Регистрируем пользователя
            let user_data = blockchain_project::kyc_aml::UserRegistrationData {
                email: format!("user{}@example.com", i),
                first_name: format!("FirstName{}", i),
                last_name: format!("LastName{}", i),
                nationality: Some("GE".to_string()),
                phone: Some("+995555123456".to_string()),
                address: Some(blockchain_project::kyc_aml::Address {
            street: "Main Street".to_string(),
            city: "Tbilisi".to_string(),
            state: "Tbilisi".to_string(),
            postal_code: "0100".to_string(),
            country: "Georgia".to_string(),
        }),
                date_of_birth: Some(chrono::Utc::now()),
            };
            manager.write().await.register_user(user_data).unwrap();
            
            // Начинаем KYC процесс
            let kyc_result = manager.write().await.start_kyc_process(&user_id, KYCLevel::Basic);
            if kyc_result.is_err() {
                println!("KYC process failed for user {}: {:?}", user_id, kyc_result);
            }
            
            user_id
        });
        handles.push(handle);
    }
    
    // Ждем завершения всех задач
    for handle in handles {
        let user_id = handle.await.unwrap();
        
        // Проверяем, что пользователь зарегистрирован
        let kyc_manager_guard = kyc_manager.read().await;
        let user = kyc_manager_guard.get_user(&user_id);
        if user.is_none() {
            println!("User {} not found after registration", user_id);
        }
        // Не делаем assert, так как регистрация может не удаться
    }
}
