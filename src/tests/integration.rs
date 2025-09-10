use blockchain_project::*;
use std::time::Duration;
use tokio::time::sleep;

/// Интеграционные тесты для проверки взаимодействия между модулями
/// 
/// Эти тесты проверяют:
/// 1. Интеграцию между блокчейном и базой данных
/// 2. Взаимодействие KYC/AML с кошельками
/// 3. Интеграцию консенсуса с сетью
/// 4. Работу API с различными модулями

#[tokio::test]
async fn test_blockchain_database_integration() {
    println!("\n🔗 === ИНТЕГРАЦИОННЫЙ ТЕСТ: БЛОКЧЕЙН + БАЗА ДАННЫХ ===");
    
    // Создаем блокчейн
    let owner = "Alice".to_string();
    let mut blockchain = Blockchain::new(owner.clone());
    
    // Создаем менеджер базы данных
    let db_config = database::DatabaseConfig::new();
    let mut db_manager = database::DatabaseManager::new(db_config);
    
    // Создаем пользователя
    let user_data = database::UserData::new(
        "user123".to_string(),
        "john@example.com".to_string(),
        "0xwallet123".to_string()
    );
    
    // Сохраняем пользователя в базе данных
    db_manager.save_user_data(user_data.clone()).await.unwrap();
    
    // Создаем покупку в блокчейне
    let check = blockchain.process_purchase(
        "Customer".to_string(),
        "Truck".to_string(),
        50.0,
        vec!["Burger".to_string(), "Fries".to_string()],
    );
    
    // Проверяем, что чек создан
    assert!(!check.check_id.is_empty());
    assert_eq!(check.amount, 50.0);
    
    // Проверяем, что транзакция добавлена в блокчейн
    assert!(!blockchain.pending_transactions.is_empty());
    
    // Проверяем, что пользователь существует в базе данных
    let retrieved_user = db_manager.get_user_data("user123").await.unwrap();
    assert_eq!(retrieved_user.email, "john@example.com");
    
    println!("  ✅ Блокчейн и база данных успешно интегрированы");
}

#[tokio::test]
async fn test_kyc_wallet_integration() {
    println!("\n🔐 === ИНТЕГРАЦИОННЫЙ ТЕСТ: KYC/AML + КОШЕЛЬКИ ===");
    
    // Создаем менеджер KYC/AML
    let mut kyc_manager = kyc_aml::KYCAmlManager::new();
    
    // Создаем менеджер HD кошельков
    let mut wallet_manager = hd_wallet::HDWalletManager::new();
    
    // Создаем пользователя KYC
    let kyc_user = kyc_aml::KYCUser::new(
        "user123".to_string(),
        "john@example.com".to_string(),
        "+1234567890".to_string()
    );
    
    // Добавляем пользователя в KYC систему
    kyc_manager.add_user(kyc_user).unwrap();
    
    // Создаем кошелек для пользователя
    let wallet = hd_wallet::HDWallet::new(
        "user123_wallet".to_string(),
        hd_wallet::WalletType::Standard
    );
    
    // Добавляем кошелек в менеджер
    wallet_manager.add_wallet(wallet).unwrap();
    
    // Проверяем KYC статус
    let user = kyc_manager.get_user("user123").unwrap();
    assert!(matches!(user.kyc_status, kyc_aml::KYCStatus::Pending));
    
    // Завершаем KYC верификацию
    user.complete_verification(kyc_aml::KYCLevel::Intermediate).unwrap();
    
    // Проверяем, что кошелек активен после KYC
    let wallet = wallet_manager.get_wallet("user123_wallet").unwrap();
    assert!(matches!(wallet.status, hd_wallet::WalletStatus::Active));
    
    println!("  ✅ KYC/AML и кошельки успешно интегрированы");
}

#[tokio::test]
async fn test_consensus_network_integration() {
    println!("\n🌐 === ИНТЕГРАЦИОННЫЙ ТЕСТ: КОНСЕНСУС + СЕТЬ ===");
    
    // Создаем алгоритм консенсуса
    let mut consensus = consensus::ConsensusAlgorithm::new();
    
    // Создаем P2P узел
    let mut p2p_node = p2p_network::P2PNode::new("node1".to_string());
    
    // Регистрируем валидаторов
    let validator1 = consensus::Validator::new("validator1".to_string(), 100.0);
    let validator2 = consensus::Validator::new("validator2".to_string(), 100.0);
    let validator3 = consensus::Validator::new("validator3".to_string(), 100.0);
    
    consensus.register_validator(validator1).unwrap();
    consensus.register_validator(validator2).unwrap();
    consensus.register_validator(validator3).unwrap();
    
    // Создаем транзакцию
    let transaction = consensus::Transaction::new(
        "alice".to_string(),
        "bob".to_string(),
        10.0,
        consensus::TransactionType::Transfer
    );
    
    // Создаем блок
    let block = consensus::Block::new(1, "genesis".to_string(), vec![transaction]);
    
    // Достигаем консенсуса
    let consensus_result = consensus.reach_consensus(&block);
    assert!(consensus_result.is_ok());
    
    // Подключаем узел к сети
    p2p_node.connect_to_network("127.0.0.1:8080".to_string()).await.unwrap();
    
    // Проверяем, что узел подключен
    assert!(p2p_node.is_connected());
    
    println!("  ✅ Консенсус и сеть успешно интегрированы");
}

#[tokio::test]
async fn test_api_module_integration() {
    println!("\n🔌 === ИНТЕГРАЦИОННЫЙ ТЕСТ: API + МОДУЛИ ===");
    
    // Создаем блокчейн
    let owner = "Alice".to_string();
    let mut blockchain = Blockchain::new(owner.clone());
    
    // Создаем веб-сервер
    let mut web_server = web_server::WebServer::new("127.0.0.1:8080".to_string());
    
    // Создаем API запрос
    let api_request = ApiRequest::GetMenu;
    
    // Обрабатываем запрос через API
    let api_response = web_server.handle_api_request(api_request, &mut blockchain).await;
    
    // Проверяем, что ответ получен
    match api_response {
        Ok(response) => {
            match response {
                ApiResponse::Menu { items } => {
                    assert!(items.is_empty()); // Меню должно быть пустым изначально
                }
                _ => panic!("Unexpected response type"),
            }
        }
        Err(_) => panic!("API request failed"),
    }
    
    // Добавляем элемент в меню
    let menu_item = MenuItem::new(
        "Burger".to_string(),
        "Delicious burger".to_string(),
        10.0,
        "chef".to_string(),
        7
    );
    blockchain.menu_items.push(menu_item);
    
    // Повторно запрашиваем меню
    let api_request = ApiRequest::GetMenu;
    let api_response = web_server.handle_api_request(api_request, &mut blockchain).await;
    
    match api_response {
        Ok(response) => {
            match response {
                ApiResponse::Menu { items } => {
                    assert_eq!(items.len(), 1);
                    assert_eq!(items[0].name, "Burger");
                }
                _ => panic!("Unexpected response type"),
            }
        }
        Err(_) => panic!("API request failed"),
    }
    
    println!("  ✅ API и модули успешно интегрированы");
}

#[tokio::test]
async fn test_video_surveillance_integration() {
    println!("\n📹 === ИНТЕГРАЦИОННЫЙ ТЕСТ: ВИДЕОНАБЛЮДЕНИЕ + СИСТЕМА ===");
    
    // Создаем систему видеонаблюдения
    let mut video_system = video_surveillance::VideoSurveillanceSystem::new();
    
    // Создаем конфигурацию камеры
    let camera_config = video_surveillance::CameraConfig::new(
        "camera1".to_string(),
        "192.168.1.100".to_string(),
        video_surveillance::CameraType::Indoor
    );
    
    // Добавляем камеру в систему
    video_system.add_camera(camera_config).unwrap();
    
    // Создаем зону анонимизации
    let anonymization_zone = video_surveillance::AnonymizationZone::new(
        "zone1".to_string(),
        100, 100, 200, 200
    );
    
    // Добавляем зону в систему
    video_system.add_anonymization_zone(anonymization_zone).unwrap();
    
    // Создаем конфигурацию стриминга
    let streaming_config = video_surveillance::StreamingConfig::new(
        video_surveillance::StreamQuality::HD,
        30,
        1920,
        1080
    );
    
    // Настраиваем стриминг
    video_system.configure_streaming(streaming_config).unwrap();
    
    // Проверяем, что камера добавлена
    assert_eq!(video_system.cameras.len(), 1);
    assert_eq!(video_system.cameras[0].id, "camera1");
    
    // Проверяем, что зона анонимизации добавлена
    assert_eq!(video_system.anonymization_zones.len(), 1);
    assert_eq!(video_system.anonymization_zones[0].id, "zone1");
    
    // Проверяем конфигурацию стриминга
    assert!(matches!(video_system.streaming_config.quality, video_surveillance::StreamQuality::HD));
    assert_eq!(video_system.streaming_config.fps, 30);
    
    println!("  ✅ Система видеонаблюдения успешно интегрирована");
}

#[tokio::test]
async fn test_franchise_network_integration() {
    println!("\n🏪 === ИНТЕГРАЦИОННЫЙ ТЕСТ: ФРАНШИЗНАЯ СЕТЬ ===");
    
    // Создаем франшизную сеть
    let mut franchise_network = franchise_network::FranchiseNetwork::new();
    
    // Создаем узел франшизы
    let franchise_node = franchise_network::FranchiseNode::new(
        "franchise1".to_string(),
        "Franchise Location 1".to_string(),
        franchise_network::NodeType::Franchise
    );
    
    // Добавляем узел в сеть
    franchise_network.add_node(franchise_node).unwrap();
    
    // Создаем товар для продажи
    let sale_item = franchise_network::SaleItem::new(
        "item1".to_string(),
        "Burger".to_string(),
        10.0,
        "franchise1".to_string()
    );
    
    // Добавляем товар в узел
    franchise_network.add_sale_item(sale_item).unwrap();
    
    // Проверяем, что узел добавлен
    assert_eq!(franchise_network.nodes.len(), 1);
    assert_eq!(franchise_network.nodes[0].id, "franchise1");
    
    // Проверяем, что товар добавлен
    assert_eq!(franchise_network.sale_items.len(), 1);
    assert_eq!(franchise_network.sale_items[0].id, "item1");
    
    println!("  ✅ Франшизная сеть успешно интегрирована");
}

#[tokio::test]
async fn test_regulatory_exports_integration() {
    println!("\n📊 === ИНТЕГРАЦИОННЫЙ ТЕСТ: РЕГУЛЯТОРНЫЕ ЭКСПОРТЫ ===");
    
    // Создаем экспортер
    let mut exporter = regulatory_exports::RegulatoryExporter::new();
    
    // Создаем запись держателя
    let holder_entry = regulatory_exports::HolderRegistryEntry::new(
        "holder1".to_string(),
        "0xwallet123".to_string(),
        1000.0,
        "Security".to_string()
    );
    
    // Добавляем запись
    exporter.add_holder_entry(holder_entry).unwrap();
    
    // Создаем запись эмиссии
    let emission_entry = regulatory_exports::EmissionRegistryEntry::new(
        "emission1".to_string(),
        10000.0,
        "Security".to_string(),
        "Initial emission".to_string()
    );
    
    // Добавляем запись
    exporter.add_emission_entry(emission_entry).unwrap();
    
    // Создаем запись корпоративного действия
    let corporate_action = regulatory_exports::CorporateActionEntry::new(
        "action1".to_string(),
        "Dividend payment".to_string(),
        "2024-01-01".to_string()
    );
    
    // Добавляем запись
    exporter.add_corporate_action(corporate_action).unwrap();
    
    // Проверяем, что записи добавлены
    assert_eq!(exporter.holder_registry.len(), 1);
    assert_eq!(exporter.emission_registry.len(), 1);
    assert_eq!(exporter.corporate_actions.len(), 1);
    
    // Экспортируем данные в JSON
    let json_export = exporter.export_to_json().unwrap();
    assert!(!json_export.is_empty());
    
    // Экспортируем данные в CSV
    let csv_export = exporter.export_to_csv().unwrap();
    assert!(!csv_export.is_empty());
    
    println!("  ✅ Регуляторные экспорты успешно интегрированы");
}

#[tokio::test]
async fn test_relayer_service_integration() {
    println!("\n🔄 === ИНТЕГРАЦИОННЫЙ ТЕСТ: RELAYER СЕРВИС ===");
    
    // Создаем конфигурацию relayer
    let config = relayer_service::RelayerConfig::new(
        "relayer1".to_string(),
        "127.0.0.1:8080".to_string()
    );
    
    // Создаем relayer сервис
    let mut relayer = relayer_service::RelayerService::new(config);
    
    // Создаем запрос на транзакцию
    let tx_request = relayer_service::RelayerTransactionRequest::new(
        "0xfrom".to_string(),
        "0xto".to_string(),
        100.0,
        "Transfer".to_string()
    );
    
    // Обрабатываем транзакцию
    let response = relayer.process_transaction(tx_request).await;
    
    // Проверяем, что транзакция обработана
    match response {
        Ok(relayer_response) => {
            assert!(!relayer_response.transaction_id.is_empty());
            assert!(matches!(relayer_response.status, relayer_service::RelayerTransactionStatus::Pending));
        }
        Err(_) => panic!("Transaction processing failed"),
    }
    
    // Проверяем статистику
    let stats = relayer.get_statistics();
    assert_eq!(stats.total_transactions, 1);
    assert_eq!(stats.pending_transactions, 1);
    
    println!("  ✅ Relayer сервис успешно интегрирован");
}

#[tokio::test]
async fn test_observability_integration() {
    println!("\n📈 === ИНТЕГРАЦИОННЫЙ ТЕСТ: OBSERVABILITY ===");
    
    // Создаем конфигурацию observability
    let config = observability::ObservabilityConfig::new();
    
    // Создаем менеджер observability
    let mut observability_manager = observability::ObservabilityManager::new(config);
    
    // Логируем событие
    observability_manager.log_event(
        observability::LogLevel::Info,
        "Test event".to_string(),
        "test_module".to_string()
    ).await.unwrap();
    
    // Записываем метрику
    observability_manager.record_metric(
        "test_metric".to_string(),
        100.0,
        observability::MetricType::Counter
    ).await.unwrap();
    
    // Создаем алерт
    let alert = observability_manager.create_alert(
        "test_alert".to_string(),
        "Test alert description".to_string(),
        observability::AlertSeverity::Warning
    ).unwrap();
    
    // Проверяем, что алерт создан
    assert_eq!(alert.id, "test_alert");
    assert!(matches!(alert.severity, observability::AlertSeverity::Warning));
    
    // Проверяем статистику
    let stats = observability_manager.get_statistics();
    assert!(stats.total_logs > 0);
    assert!(stats.total_metrics > 0);
    assert!(stats.total_alerts > 0);
    
    println!("  ✅ Observability успешно интегрирован");
}

#[tokio::test]
async fn test_api_versioning_integration() {
    println!("\n🔢 === ИНТЕГРАЦИОННЫЙ ТЕСТ: API VERSIONING ===");
    
    // Создаем конфигурацию API
    let api_config = api_versioning::ApiConfig::new();
    
    // Создаем менеджер версий API
    let mut version_manager = api_versioning::ApiVersionManager::new(api_config);
    
    // Создаем версию API
    let version = api_versioning::ApiVersion::new(
        "1.0.0".to_string(),
        "Initial API version".to_string(),
        api_versioning::VersionStatus::Active
    );
    
    // Добавляем версию
    version_manager.add_version(version).unwrap();
    
    // Создаем спецификацию OpenAPI
    let openapi_spec = api_versioning::OpenApiSpec::new(
        "1.0.0".to_string(),
        "Food Truck API".to_string(),
        "API for food truck blockchain system".to_string()
    );
    
    // Добавляем спецификацию
    version_manager.add_openapi_spec(openapi_spec).unwrap();
    
    // Проверяем, что версия добавлена
    assert_eq!(version_manager.versions.len(), 1);
    assert_eq!(version_manager.versions[0].version, "1.0.0");
    
    // Проверяем, что спецификация добавлена
    assert_eq!(version_manager.openapi_specs.len(), 1);
    assert_eq!(version_manager.openapi_specs[0].version, "1.0.0");
    
    // Получаем информацию о версии
    let version_info = version_manager.get_version_info("1.0.0").unwrap();
    assert_eq!(version_info.version, "1.0.0");
    assert!(matches!(version_info.status, api_versioning::VersionStatus::Active));
    
    println!("  ✅ API versioning успешно интегрирован");
}

/// Комплексный интеграционный тест всех модулей
#[tokio::test]
async fn test_comprehensive_system_integration() {
    println!("\n🚀 === КОМПЛЕКСНЫЙ ИНТЕГРАЦИОННЫЙ ТЕСТ ===");
    
    // Создаем все основные компоненты системы
    let owner = "Alice".to_string();
    let mut blockchain = Blockchain::new(owner.clone());
    
    let mut kyc_manager = kyc_aml::KYCAmlManager::new();
    let mut wallet_manager = hd_wallet::HDWalletManager::new();
    let mut db_manager = database::DatabaseManager::new(database::DatabaseConfig::new());
    let mut video_system = video_surveillance::VideoSurveillanceSystem::new();
    let mut franchise_network = franchise_network::FranchiseNetwork::new();
    let mut observability_manager = observability::ObservabilityManager::new(observability::ObservabilityConfig::new());
    
    // 1. Регистрируем пользователя в KYC
    let kyc_user = kyc_aml::KYCUser::new(
        "user123".to_string(),
        "john@example.com".to_string(),
        "+1234567890".to_string()
    );
    kyc_manager.add_user(kyc_user).unwrap();
    
    // 2. Создаем кошелек для пользователя
    let wallet = hd_wallet::HDWallet::new(
        "user123_wallet".to_string(),
        hd_wallet::WalletType::Standard
    );
    wallet_manager.add_wallet(wallet).unwrap();
    
    // 3. Сохраняем данные пользователя в базе данных
    let user_data = database::UserData::new(
        "user123".to_string(),
        "john@example.com".to_string(),
        "0xwallet123".to_string()
    );
    db_manager.save_user_data(user_data).await.unwrap();
    
    // 4. Создаем франшизный узел
    let franchise_node = franchise_network::FranchiseNode::new(
        "franchise1".to_string(),
        "Franchise Location 1".to_string(),
        franchise_network::NodeType::Franchise
    );
    franchise_network.add_node(franchise_node).unwrap();
    
    // 5. Настраиваем видеонаблюдение
    let camera_config = video_surveillance::CameraConfig::new(
        "camera1".to_string(),
        "192.168.1.100".to_string(),
        video_surveillance::CameraType::Indoor
    );
    video_system.add_camera(camera_config).unwrap();
    
    // 6. Создаем покупку в блокчейне
    let check = blockchain.process_purchase(
        "Customer".to_string(),
        "Truck".to_string(),
        50.0,
        vec!["Burger".to_string(), "Fries".to_string()],
    );
    
    // 7. Логируем событие в observability
    observability_manager.log_event(
        observability::LogLevel::Info,
        "Purchase completed".to_string(),
        "blockchain".to_string()
    ).await.unwrap();
    
    // Проверяем, что все компоненты работают вместе
    assert_eq!(kyc_manager.user_count, 1);
    assert_eq!(wallet_manager.wallet_count, 1);
    assert_eq!(franchise_network.nodes.len(), 1);
    assert_eq!(video_system.cameras.len(), 1);
    assert!(!check.check_id.is_empty());
    
    // Проверяем статистику observability
    let stats = observability_manager.get_statistics();
    assert!(stats.total_logs > 0);
    
    println!("  ✅ Все модули успешно интегрированы и работают вместе");
    println!("  📊 Статистика системы:");
    println!("    - KYC пользователей: {}", kyc_manager.user_count);
    println!("    - Кошельков: {}", wallet_manager.wallet_count);
    println!("    - Франшизных узлов: {}", franchise_network.nodes.len());
    println!("    - Камер: {}", video_system.cameras.len());
    println!("    - Логов: {}", stats.total_logs);
}
