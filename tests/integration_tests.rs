use blockchain_project::*;
use std::time::SystemTime;

#[tokio::test]
async fn test_full_blockchain_workflow() {
    // Создаем основные компоненты
    let mut blockchain = Blockchain::new();
    let mut franchise_network = FranchiseNetwork::new();
    let mut wallet_manager = HDWalletManager::new();
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Регистрируем пользователя
    let user_id = "test_user_001";
    kyc_manager.register_user(
        user_id,
        "test@example.com",
        "John",
        "Doe",
        Some("GE".to_string())
    ).unwrap();
    
    // 2. Начинаем KYC процесс
    kyc_manager.start_kyc_process(user_id, KYCLevel::Basic).unwrap();
    
    // 3. Генерируем кошелек для пользователя
    let seed_phrase = "test seed phrase for integration test";
    let wallet = wallet_manager.generate_wallet(seed_phrase, WalletType::Node);
    
    // 4. Добавляем узел в сеть франшизы
    let node = FranchiseNode {
        id: 1,
        name: "Test Restaurant".to_string(),
        node_type: NodeType::Restaurant,
        location: "Tbilisi, Georgia".to_string(),
        capacity: 100,
        current_load: 0,
        is_active: true,
        created_at: SystemTime::now(),
        last_updated: SystemTime::now(),
    };
    franchise_network.add_node(node);
    
    // 5. Добавляем элемент меню
    blockchain.add_menu_item_with_details(
        "Pizza Margherita".to_string(),
        "Classic Italian pizza with tomato and mozzarella".to_string(),
        1500, // 15.00 GEL в субъединицах
        "Italian".to_string(),
        vec!["vegetarian".to_string()],
        "test_restaurant".to_string()
    );
    
    // 6. Обрабатываем покупку
    let customer = "customer_001".to_string();
    let food_truck = "test_restaurant".to_string();
    let amount = 1500u128;
    let food_items = vec!["Pizza Margherita".to_string()];
    
    let result = blockchain.process_purchase(customer, food_truck, amount, food_items);
    assert!(result.is_ok());
    
    // 7. Проверяем, что транзакция записана
    let transactions = blockchain.get_transaction_history();
    assert!(!transactions.is_empty());
    
    // 8. Проверяем статистику
    let stats = blockchain.get_statistics();
    assert!(stats.total_transactions > 0);
}

#[tokio::test]
async fn test_kyc_workflow() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Регистрируем пользователя
    let user_id = "kyc_test_user";
    kyc_manager.register_user(
        user_id,
        "kyc@example.com",
        "Jane",
        "Smith",
        Some("US".to_string())
    ).unwrap();
    
    // 2. Начинаем KYC процесс
    kyc_manager.start_kyc_process(user_id, KYCLevel::Enhanced).unwrap();
    
    // 3. Загружаем документы
    let document_id = kyc_manager.upload_document(
        user_id,
        DocumentType::Passport,
        "document_hash_123".to_string(),
        "/path/to/passport.pdf".to_string()
    ).unwrap();
    
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
    let user = kyc_manager.get_user(user_id).unwrap().unwrap();
    assert_eq!(user.kyc_status, KYCStatus::Verified);
}

#[tokio::test]
async fn test_wallet_operations() {
    let mut wallet_manager = HDWalletManager::new();
    let seed_phrase = "integration test seed phrase";
    
    // 1. Генерируем несколько кошельков
    let node_wallet = wallet_manager.generate_wallet(seed_phrase, WalletType::Node);
    let check_wallet = wallet_manager.generate_wallet(seed_phrase, WalletType::Check);
    
    // 2. Проверяем, что кошельки разные
    assert_ne!(node_wallet.address, check_wallet.address);
    
    // 3. Проверяем, что адреса валидны
    assert!(!node_wallet.address.is_empty());
    assert!(!check_wallet.address.is_empty());
    
    // 4. Генерируем чек-кошелек
    let activation_code = wallet_manager.generate_check_wallet(
        "user_001",
        10000, // 100.00 GEL
        "GEL"
    ).unwrap();
    
    // 5. Активируем чек-кошелек
    let result = wallet_manager.activate_check_wallet(&activation_code);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_franchise_network_operations() {
    let mut network = FranchiseNetwork::new();
    
    // 1. Добавляем различные типы узлов
    let restaurant = FranchiseNode {
        id: 1,
        name: "Main Restaurant".to_string(),
        node_type: NodeType::Restaurant,
        location: "Tbilisi Center".to_string(),
        capacity: 200,
        current_load: 0,
        is_active: true,
        created_at: SystemTime::now(),
        last_updated: SystemTime::now(),
    };
    
    let food_truck = FranchiseNode {
        id: 2,
        name: "Mobile Food Truck".to_string(),
        node_type: NodeType::FoodTruck,
        location: "Rustaveli Avenue".to_string(),
        capacity: 50,
        current_load: 0,
        is_active: true,
        created_at: SystemTime::now(),
        last_updated: SystemTime::now(),
    };
    
    let warehouse = FranchiseNode {
        id: 3,
        name: "Central Warehouse".to_string(),
        node_type: NodeType::Warehouse,
        location: "Industrial Zone".to_string(),
        capacity: 1000,
        current_load: 0,
        is_active: true,
        created_at: SystemTime::now(),
        last_updated: SystemTime::now(),
    };
    
    network.add_node(restaurant);
    network.add_node(food_truck);
    network.add_node(warehouse);
    
    // 2. Проверяем, что все узлы добавлены
    let all_nodes = network.get_all_nodes();
    assert_eq!(all_nodes.len(), 3);
    
    // 3. Проверяем фильтрацию по типу
    let restaurants = network.get_nodes_by_type(NodeType::Restaurant);
    assert_eq!(restaurants.len(), 1);
    
    let food_trucks = network.get_nodes_by_type(NodeType::FoodTruck);
    assert_eq!(food_trucks.len(), 1);
    
    let warehouses = network.get_nodes_by_type(NodeType::Warehouse);
    assert_eq!(warehouses.len(), 1);
    
    // 4. Записываем продажу
    let sale_id = "sale_001".to_string();
    network.record_sale(
        1, // restaurant node
        sale_id,
        2500, // 25.00 GEL
        vec![
            SaleItem {
                item_name: "Pizza".to_string(),
                quantity: 2,
                price_subunits: 2500,
            }
        ]
    );
    
    // 5. Проверяем статистику продаж
    let stats = network.get_sales_statistics();
    assert!(stats.total_sales > 0);
}

#[tokio::test]
async fn test_error_handling() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // 1. Пытаемся получить несуществующего пользователя
    let result = kyc_manager.get_user("nonexistent_user");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    
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
    
    let kyc_manager = Arc::new(KYCAmlManager::new());
    let mut handles = vec![];
    
    // Запускаем несколько задач параллельно
    for i in 0..10 {
        let manager = Arc::clone(&kyc_manager);
        let handle = task::spawn(async move {
            let user_id = format!("concurrent_user_{}", i);
            
            // Регистрируем пользователя
            manager.register_user(
                &user_id,
                &format!("user{}@example.com", i),
                &format!("FirstName{}", i),
                &format!("LastName{}", i),
                Some("GE".to_string())
            ).unwrap();
            
            // Начинаем KYC процесс
            manager.start_kyc_process(&user_id, KYCLevel::Basic).unwrap();
            
            user_id
        });
        handles.push(handle);
    }
    
    // Ждем завершения всех задач
    for handle in handles {
        let user_id = handle.await.unwrap();
        
        // Проверяем, что пользователь зарегистрирован
        let user = kyc_manager.get_user(&user_id).unwrap();
        assert!(user.is_some());
    }
}
