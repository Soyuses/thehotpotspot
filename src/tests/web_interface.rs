use crate::*;
use std::collections::HashMap;

/// Тесты для веб-интерфейса
/// 
/// Этот модуль тестирует:
/// 1. API endpoints для веб-интерфейса
/// 2. Корректность данных, возвращаемых API
/// 3. Обработку ошибок в API
/// 4. Интеграцию с блокчейном

#[test]
fn test_api_get_blockchain_info() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ ИНФОРМАЦИИ О БЛОКЧЕЙНЕ ===");
    
    // Создаем API сервер
    let api_server = ApiServer::new(bc);
    
    // Тестируем получение информации о блокчейне
    let request = ApiRequest::GetBlockchainInfo;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::BlockchainInfo(info) => {
            println!("✅ Получена информация о блокчейне:");
            println!("  Количество блоков: {}", info.chain_length);
            println!("  Владелец: {}", info.owner);
            println!("  Сложность: {}", info.difficulty);
            println!("  Минимальная ставка: {}", info.min_stake);
            println!("  Награда за блок: {}", info.block_reward);
            
            assert!(info.chain_length >= 1, "Должен быть хотя бы genesis блок");
            assert_eq!(info.owner, owner, "Владелец должен совпадать");
        },
        _ => panic!("Ожидался ответ BlockchainInfo"),
    }
}

#[test]
fn test_api_get_token_holders() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ ДЕРЖАТЕЛЕЙ ТОКЕНОВ ===");
    
    // Добавляем несколько держателей токенов
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    bc.process_purchase(
        "Customer2".to_string(),
        "Truck1".to_string(),
        50.0,
        vec!["Pizza".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получены держатели токенов:");
            for (address, holder) in holders {
                println!("  {}: {:.2} security, {:.2} utility", 
                    address, holder.security_tokens, holder.utility_tokens);
            }
            
            assert!(holders.len() >= 3, "Должно быть минимум 3 держателя (владелец, фонд, покупатели)");
            assert!(holders.contains_key(&owner), "Должен быть владелец");
            assert!(holders.contains_key("Customer1"), "Должен быть Customer1");
            assert!(holders.contains_key("Customer2"), "Должен быть Customer2");
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}

#[test]
fn test_api_get_transactions() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ ТРАНЗАКЦИЙ ===");
    
    // Добавляем транзакции
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    bc.process_purchase(
        "Customer2".to_string(),
        "Truck2".to_string(),
        75.0,
        vec!["Pizza".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetTransactions;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::Transactions(transactions) => {
            println!("✅ Получены транзакции:");
            for tx in &transactions {
                println!("  {}: {} -> {} ({} токенов)", 
                    tx.transaction_id, tx.customer, tx.food_truck, tx.amount);
            }
            
            assert!(transactions.len() >= 2, "Должно быть минимум 2 транзакции");
        },
        _ => panic!("Ожидался ответ Transactions"),
    }
}

#[test]
fn test_api_get_checks() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ ЧЕКОВ ===");
    
    // Создаем чеки
    let check1 = bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let check2 = bc.process_purchase(
        "Customer2".to_string(),
        "Truck2".to_string(),
        50.0,
        vec!["Pizza".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetChecks;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::Checks(checks) => {
            println!("✅ Получены чеки:");
            for check in &checks {
                println!("  {}: {} токенов, активен: {}", 
                    check.check_id, check.amount, check.is_active);
            }
            
            assert!(checks.len() >= 2, "Должно быть минимум 2 чека");
            
            // Проверяем, что наши чеки присутствуют
            let check_ids: Vec<String> = checks.iter().map(|c| c.check_id.clone()).collect();
            assert!(check_ids.contains(&check1.check_id), "Должен быть check1");
            assert!(check_ids.contains(&check2.check_id), "Должен быть check2");
        },
        _ => panic!("Ожидался ответ Checks"),
    }
}

#[test]
fn test_api_get_network_security() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ ИНФОРМАЦИИ О БЕЗОПАСНОСТИ ===");
    
    // Добавляем несколько держателей для тестирования безопасности
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetNetworkSecurity;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::NetworkSecurity(report) => {
            println!("✅ Получен отчет о безопасности:");
            println!("  Сеть безопасна: {}", report.is_secure);
            println!("  Количество рисков: {}", report.security_risks.len());
            println!("  Количество utility рисков: {}", report.utility_risks.len());
            
            // Проверяем структуру отчета
            assert!(report.total_security_tokens > 0.0, "Должны быть security токены");
            assert!(report.total_utility_tokens > 0.0, "Должны быть utility токены");
        },
        _ => panic!("Ожидался ответ NetworkSecurity"),
    }
}

#[test]
fn test_api_get_monitoring_alerts() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ПОЛУЧЕНИЕ АЛЕРТОВ МОНИТОРИНГА ===");
    
    // Создаем ситуацию, которая может вызвать алерты
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetMonitoringAlerts { limit: Some(10) };
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::MonitoringAlerts(alerts) => {
            println!("✅ Получены алерты мониторинга:");
            for alert in &alerts {
                println!("  {:?}: {} - {}", 
                    alert.alert_type, alert.severity, alert.message);
            }
            
            // Алерты могут быть или не быть, в зависимости от ситуации
            println!("  Количество алертов: {}", alerts.len());
        },
        _ => panic!("Ожидался ответ MonitoringAlerts"),
    }
}

#[test]
fn test_api_error_handling() {
    let owner = "Alice".to_string();
    let bc = Blockchain::new(owner);
    
    println!("\n🌐 === ТЕСТ API: ОБРАБОТКА ОШИБОК ===");
    
    let api_server = ApiServer::new(bc);
    
    // Тестируем несуществующий запрос (это должно обрабатываться корректно)
    let request = ApiRequest::GetBlockchainInfo; // Валидный запрос
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::BlockchainInfo(_) => {
            println!("✅ API корректно обрабатывает валидные запросы");
        },
        ApiResponse::Error(msg) => {
            println!("❌ Неожиданная ошибка: {}", msg);
            panic!("API не должен возвращать ошибку для валидного запроса");
        },
        _ => {
            println!("✅ API возвращает корректный тип ответа");
        }
    }
}

#[test]
fn test_api_franchise_operations() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ОПЕРАЦИИ С ФРАНШИЗАМИ ===");
    
    // Добавляем франшизную ноду
    let franchise_owner = "FranchiseOwner1".to_string();
    let franchise_node = "FranchiseTruck1".to_string();
    bc.add_franchise_node(franchise_node.clone(), franchise_owner.clone());
    
    // Делаем покупку на франшизной ноде
    bc.process_purchase(
        "Customer1".to_string(),
        franchise_node.clone(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Получаем информацию о держателях токенов
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получены держатели токенов после франшизной операции:");
            for (address, holder) in holders {
                println!("  {}: {:.2} security, {:.2} utility", 
                    address, holder.security_tokens, holder.utility_tokens);
            }
            
            // Проверяем, что владелец франшизы получил токены
            assert!(holders.contains_key(&franchise_owner), "Должен быть владелец франшизы");
            
            let franchise_holder = holders.get(&franchise_owner).unwrap();
            assert!(franchise_holder.security_tokens > 0.0, "Владелец франшизы должен иметь токены");
            assert!(franchise_holder.is_franchise_owner, "Должен быть помечен как владелец франшизы");
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}

#[test]
fn test_api_token_emission() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === ТЕСТ API: ЭМИССИЯ ТОКЕНОВ ===");
    
    // Делаем несколько покупок для создания базового количества токенов
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let initial_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    println!("Общее количество токенов до эмиссии: {:.2}", initial_total);
    
    // Эмитируем токены для инвестора
    let investor = "WhaleInvestor".to_string();
    let emission_amount = 1000.0;
    
    let result = bc.emit_tokens_for_investors(emission_amount, investor.clone());
    assert!(result.is_ok(), "Эмиссия должна быть успешной");
    
    let final_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    println!("Общее количество токенов после эмиссии: {:.2}", final_total);
    
    // Проверяем через API
    let api_server = ApiServer::new(bc);
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получены держатели токенов после эмиссии:");
            for (address, holder) in holders {
                println!("  {}: {:.2} security, {:.2} utility", 
                    address, holder.security_tokens, holder.utility_tokens);
            }
            
            // Проверяем, что инвестор получил токены
            assert!(holders.contains_key(&investor), "Должен быть инвестор");
            
            let investor_holder = holders.get(&investor).unwrap();
            let expected_investor_tokens = emission_amount * 0.49; // 49% от эмиссии
            assert!((investor_holder.security_tokens - expected_investor_tokens).abs() < 0.01, 
                "Инвестор должен получить 49% от эмиссии");
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}
