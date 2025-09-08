use crate::*;
use std::collections::HashMap;

/// Тесты для мобильного приложения
/// 
/// Этот модуль тестирует:
/// 1. API endpoints, используемые мобильным приложением
/// 2. Функциональность, специфичную для мобильных клиентов
/// 3. Интеграцию с блокчейном через мобильный API
/// 4. Обработку данных для мобильного интерфейса

#[test]
fn test_mobile_customer_wallet_api() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: КОШЕЛЕК КЛИЕНТА ===");
    
    // Создаем клиента с покупками
    let customer = "MobileCustomer1".to_string();
    bc.process_purchase(
        customer.clone(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    bc.process_purchase(
        customer.clone(),
        "Truck2".to_string(),
        50.0,
        vec!["Pizza".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Тестируем получение информации о кошельке клиента
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получена информация о кошельке клиента:");
            
            if let Some(customer_holder) = holders.get(&customer) {
                println!("  Клиент: {}", customer);
                println!("  Security токены: {:.2}", customer_holder.security_tokens);
                println!("  Utility токены: {:.2}", customer_holder.utility_tokens);
                println!("  Роль: {:?}", customer_holder.role);
                println!("  Авторизован: {}", customer_holder.is_authorized);
                
                assert!(customer_holder.security_tokens > 0.0, "Клиент должен иметь security токены");
                assert!(customer_holder.utility_tokens > 0.0, "Клиент должен иметь utility токены");
                assert_eq!(customer_holder.role, UserRole::Starter, "Клиент должен иметь роль Starter");
            } else {
                panic!("Клиент должен быть найден в держателях токенов");
            }
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}

#[test]
fn test_mobile_check_activation() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: АКТИВАЦИЯ ЧЕКА ===");
    
    // Создаем чек
    let check = bc.process_purchase(
        "MobileCustomer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    println!("Создан чек: {}", check.check_id);
    println!("Сумма чека: {:.2}", check.amount);
    println!("Активационный код: {}", check.activation_code);
    
    // Тестируем активацию чека
    let personal_data = PersonalData {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        phone: "+1234567890".to_string(),
    };
    
    let result = bc.activate_account(&check.check_id, &check.activation_code, personal_data.clone());
    
    match result {
        Ok(()) => {
            println!("✅ Чек успешно активирован");
            
            // Проверяем, что активация прошла успешно
            let api_server = ApiServer::new(bc);
            let request = ApiRequest::GetChecks;
            let response = api_server.process_request(request);
            
            match response {
                ApiResponse::Checks(checks) => {
                    if let Some(activated_check) = checks.iter().find(|c| c.check_id == check.check_id) {
                        println!("  Чек активирован: {}", activated_check.is_active);
                        assert!(activated_check.is_active, "Чек должен быть активирован");
                    } else {
                        panic!("Активированный чек должен быть найден");
                    }
                },
                _ => panic!("Ожидался ответ Checks"),
            }
        },
        Err(e) => {
            println!("❌ Ошибка активации чека: {}", e);
            panic!("Активация чека должна быть успешной");
        }
    }
}

#[test]
fn test_mobile_transaction_history() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: ИСТОРИЯ ТРАНЗАКЦИЙ ===");
    
    let customer = "MobileCustomer1".to_string();
    
    // Создаем несколько транзакций
    let check1 = bc.process_purchase(
        customer.clone(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let check2 = bc.process_purchase(
        customer.clone(),
        "Truck2".to_string(),
        75.0,
        vec!["Pizza".to_string()],
    );
    
    let check3 = bc.process_purchase(
        customer.clone(),
        "Truck1".to_string(),
        50.0,
        vec!["Salad".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Получаем историю транзакций
    let request = ApiRequest::GetTransactions;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::Transactions(transactions) => {
            println!("✅ Получена история транзакций:");
            
            // Фильтруем транзакции клиента
            let customer_transactions: Vec<_> = transactions.iter()
                .filter(|tx| tx.customer == customer)
                .collect();
            
            println!("  Транзакций клиента: {}", customer_transactions.len());
            
            for tx in &customer_transactions {
                println!("    {}: {} -> {} ({} токенов)", 
                    tx.transaction_id, tx.customer, tx.food_truck, tx.amount);
                println!("      Товары: {:?}", tx.food_items);
            }
            
            assert!(customer_transactions.len() >= 3, "Должно быть минимум 3 транзакции клиента");
            
            // Проверяем, что все наши транзакции присутствуют
            let transaction_ids: Vec<String> = customer_transactions.iter()
                .map(|tx| tx.transaction_id.clone())
                .collect();
            
            // Находим соответствующие транзакции по чекам
            let check_transactions: Vec<_> = transactions.iter()
                .filter(|tx| tx.check.as_ref().map_or(false, |c| 
                    c.check_id == check1.check_id || 
                    c.check_id == check2.check_id || 
                    c.check_id == check3.check_id))
                .collect();
            
            assert!(check_transactions.len() >= 3, "Должны быть найдены транзакции по чекам");
        },
        _ => panic!("Ожидался ответ Transactions"),
    }
}

#[test]
fn test_mobile_owner_dashboard() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: ДАШБОРД ВЛАДЕЛЬЦА ===");
    
    // Создаем активность в сети
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
    
    // Добавляем франшизную ноду
    let franchise_owner = "FranchiseOwner1".to_string();
    let franchise_node = "FranchiseTruck1".to_string();
    bc.add_franchise_node(franchise_node.clone(), franchise_owner.clone());
    
    bc.process_purchase(
        "Customer3".to_string(),
        franchise_node.clone(),
        50.0,
        vec!["Salad".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Получаем информацию для дашборда владельца
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получена информация для дашборда владельца:");
            
            // Анализируем данные для владельца
            let owner_holder = holders.get(&owner).unwrap();
            let total_tokens: f64 = holders.values().map(|h| h.security_tokens).sum();
            let owner_percentage = (owner_holder.security_tokens / total_tokens) * 100.0;
            
            println!("  Владелец: {}", owner);
            println!("  Security токены: {:.2}", owner_holder.security_tokens);
            println!("  Utility токены: {:.2}", owner_holder.utility_tokens);
            println!("  Процент владения: {:.2}%", owner_percentage);
            println!("  Роль: {:?}", owner_holder.role);
            
            // Проверяем франшизные ноды
            let franchise_holders: Vec<_> = holders.values()
                .filter(|h| h.is_franchise_owner)
                .collect();
            
            println!("  Владельцев франшиз: {}", franchise_holders.len());
            for holder in &franchise_holders {
                println!("    Франшиза: {:.2} security токенов", holder.security_tokens);
            }
            
            // Проверяем благотворительный фонд
            let charity_holder = holders.get(&api_server.blockchain.charity_fund.fund_id).unwrap();
            println!("  Благотворительный фонд: {:.2} токенов", charity_holder.security_tokens);
            
            assert!(owner_holder.security_tokens > 0.0, "Владелец должен иметь токены");
            assert!(owner_percentage <= 48.0, "Владелец не должен превышать 48%");
            assert!(!franchise_holders.is_empty(), "Должны быть владельцы франшиз");
            assert!(charity_holder.security_tokens > 0.0, "Фонд должен иметь токены");
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}

#[test]
fn test_mobile_franchise_dashboard() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: ДАШБОРД ФРАНЧАЙЗИ ===");
    
    // Добавляем франшизную ноду
    let franchise_owner = "FranchiseOwner1".to_string();
    let franchise_node = "FranchiseTruck1".to_string();
    bc.add_franchise_node(franchise_node.clone(), franchise_owner.clone());
    
    // Создаем активность на франшизной ноде
    bc.process_purchase(
        "Customer1".to_string(),
        franchise_node.clone(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    bc.process_purchase(
        "Customer2".to_string(),
        franchise_node.clone(),
        75.0,
        vec!["Pizza".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Получаем информацию для дашборда франчайзи
    let request = ApiRequest::GetTokenHolders;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::TokenHolders(holders) => {
            println!("✅ Получена информация для дашборда франчайзи:");
            
            if let Some(franchise_holder) = holders.get(&franchise_owner) {
                let total_tokens: f64 = holders.values().map(|h| h.security_tokens).sum();
                let franchise_percentage = (franchise_holder.security_tokens / total_tokens) * 100.0;
                
                println!("  Владелец франшизы: {}", franchise_owner);
                println!("  Security токены: {:.2}", franchise_holder.security_tokens);
                println!("  Utility токены: {:.2}", franchise_holder.utility_tokens);
                println!("  Процент владения: {:.2}%", franchise_percentage);
                println!("  Роль: {:?}", franchise_holder.role);
                println!("  Франшизные ноды: {:?}", franchise_holder.franchise_nodes);
                
                assert!(franchise_holder.security_tokens > 0.0, "Владелец франшизы должен иметь токены");
                assert!(franchise_holder.is_franchise_owner, "Должен быть помечен как владелец франшизы");
                assert!(franchise_holder.franchise_nodes.contains(&franchise_node), "Должна быть франшизная нода");
                
                // Проверяем, что владелец франшизы получил правильную долю
                let expected_tokens = (100.0 + 75.0) * 0.24; // 24% от покупок на франшизной ноде
                assert!((franchise_holder.security_tokens - expected_tokens).abs() < 0.01, 
                    "Владелец франшизы должен получить 24% от покупок на своей ноде");
            } else {
                panic!("Владелец франшизы должен быть найден");
            }
        },
        _ => panic!("Ожидался ответ TokenHolders"),
    }
}

#[test]
fn test_mobile_security_monitoring() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: МОНИТОРИНГ БЕЗОПАСНОСТИ ===");
    
    // Создаем ситуацию для мониторинга
    bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let api_server = ApiServer::new(bc);
    
    // Получаем отчет о безопасности
    let request = ApiRequest::GetNetworkSecurity;
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::NetworkSecurity(report) => {
            println!("✅ Получен отчет о безопасности для мобильного приложения:");
            println!("  Сеть безопасна: {}", report.is_secure);
            println!("  Общие security токены: {:.2}", report.total_security_tokens);
            println!("  Общие utility токены: {:.2}", report.total_utility_tokens);
            println!("  Количество рисков: {}", report.security_risks.len());
            println!("  Количество utility рисков: {}", report.utility_risks.len());
            
            // Проверяем структуру отчета
            assert!(report.total_security_tokens > 0.0, "Должны быть security токены");
            assert!(report.total_utility_tokens > 0.0, "Должны быть utility токены");
        },
        _ => panic!("Ожидался ответ NetworkSecurity"),
    }
    
    // Получаем алерты мониторинга
    let request = ApiRequest::GetMonitoringAlerts { limit: Some(10) };
    let response = api_server.process_request(request);
    
    match response {
        ApiResponse::MonitoringAlerts(alerts) => {
            println!("✅ Получены алерты мониторинга для мобильного приложения:");
            for alert in &alerts {
                println!("  {:?}: {} - {}", 
                    alert.alert_type, alert.severity, alert.message);
            }
            println!("  Общее количество алертов: {}", alerts.len());
        },
        _ => panic!("Ожидался ответ MonitoringAlerts"),
    }
}

#[test]
fn test_mobile_data_consistency() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📱 === ТЕСТ МОБИЛЬНОГО API: КОНСИСТЕНТНОСТЬ ДАННЫХ ===");
    
    // Создаем комплексную ситуацию
    let customer = "MobileCustomer1".to_string();
    let check = bc.process_purchase(
        customer.clone(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    // Активируем чек
    let personal_data = PersonalData {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        phone: "+1234567890".to_string(),
    };
    
    bc.activate_account(&check.check_id, &check.activation_code, personal_data).unwrap();
    
    let api_server = ApiServer::new(bc);
    
    // Проверяем консистентность данных между разными API endpoints
    let holders_request = ApiRequest::GetTokenHolders;
    let holders_response = api_server.process_request(holders_request);
    
    let checks_request = ApiRequest::GetChecks;
    let checks_response = api_server.process_request(checks_request);
    
    let transactions_request = ApiRequest::GetTransactions;
    let transactions_response = api_server.process_request(transactions_request);
    
    match (holders_response, checks_response, transactions_response) {
        (ApiResponse::TokenHolders(holders), ApiResponse::Checks(checks), ApiResponse::Transactions(transactions)) => {
            println!("✅ Проверка консистентности данных:");
            
            // Проверяем, что клиент есть в держателях токенов
            assert!(holders.contains_key(&customer), "Клиент должен быть в держателях токенов");
            
            // Проверяем, что чек активирован
            let activated_check = checks.iter().find(|c| c.check_id == check.check_id).unwrap();
            assert!(activated_check.is_active, "Чек должен быть активирован");
            
            // Проверяем, что транзакция существует
            let customer_transaction = transactions.iter().find(|t| t.customer == customer).unwrap();
            assert_eq!(customer_transaction.amount, 100.0, "Сумма транзакции должна совпадать");
            
            // Проверяем соответствие токенов
            let customer_holder = holders.get(&customer).unwrap();
            let expected_tokens = 100.0 * 0.49; // 49% от покупки
            assert!((customer_holder.security_tokens - expected_tokens).abs() < 0.01, 
                "Количество токенов клиента должно соответствовать покупке");
            
            println!("  ✅ Все данные консистентны между API endpoints");
        },
        _ => panic!("Ожидались корректные ответы от всех API endpoints"),
    }
}
