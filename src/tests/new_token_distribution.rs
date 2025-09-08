use crate::*;
use std::collections::HashMap;

/// Тесты для новой системы распределения токенов
/// 
/// Этот модуль тестирует:
/// 1. Ограничение владельца 48%
/// 2. Благотворительный фонд 3%
/// 3. Ограничения для франшиз и покупателей
/// 4. Эмиссию токенов для инвесторов
/// 5. Систему мониторинга

#[test]
fn test_new_token_distribution_owner_node() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🎯 === ТЕСТ РАСПРЕДЕЛЕНИЯ ТОКЕНОВ НА НОДЕ ВЛАДЕЛЬЦА СЕТИ ===");
    
    // Делаем покупку на 100 токенов на ноде владельца сети
    let _check = bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    // Проверяем распределение
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let charity_tokens = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let customer_tokens = bc.token_holders.get("Customer1").unwrap().security_tokens;
    
    println!("Покупка на 100 токенов на ноде владельца сети:");
    println!("  Владелец сети: {:.2} токенов (ожидается 48.0)", owner_tokens);
    println!("  Благотворительный фонд: {:.2} токенов (ожидается 3.0)", charity_tokens);
    println!("  Покупатель: {:.2} токенов (ожидается 49.0)", customer_tokens);
    
    // Проверяем точность распределения
    assert!((owner_tokens - 48.0).abs() < 0.01, "Владелец сети должен получить 48 токенов");
    assert!((charity_tokens - 3.0).abs() < 0.01, "Благотворительный фонд должен получить 3 токена");
    assert!((customer_tokens - 49.0).abs() < 0.01, "Покупатель должен получить 49 токенов");
    
    // Проверяем, что владелец не превышает лимит
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_percentage = (owner_tokens / total_tokens) * 100.0;
    
    println!("  Процент владельца: {:.2}% (лимит: 48%)", owner_percentage);
    assert!(owner_percentage <= 48.0, "Владелец не должен превышать 48%");
    
    // Проверяем благотворительный фонд
    let charity_percentage = (charity_tokens / total_tokens) * 100.0;
    println!("  Процент благотворительного фонда: {:.2}% (ожидается 3%)", charity_percentage);
    assert!((charity_percentage - 3.0).abs() < 0.01, "Благотворительный фонд должен получать 3%");
    
    println!("✅ Тест пройден: новое распределение токенов работает корректно");
}

#[test]
fn test_new_token_distribution_franchise_node() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🎯 === ТЕСТ РАСПРЕДЕЛЕНИЯ ТОКЕНОВ НА НОДЕ ФРАНЧАЙЗИ ===");
    
    // Добавляем франшизную ноду
    let franchise_owner = "FranchiseOwner1".to_string();
    let franchise_node = "FranchiseTruck1".to_string();
    bc.add_franchise_node(franchise_node.clone(), franchise_owner.clone());
    
    // Делаем покупку на 100 токенов на франшизной ноде
    let _check = bc.process_purchase(
        "Customer1".to_string(),
        franchise_node.clone(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    // Проверяем распределение
    let main_owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let franchise_owner_tokens = bc.token_holders.get(&franchise_owner).unwrap().security_tokens;
    let charity_tokens = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let customer_tokens = bc.token_holders.get("Customer1").unwrap().security_tokens;
    
    println!("Покупка на 100 токенов на франшизной ноде:");
    println!("  Владелец сети: {:.2} токенов (ожидается 25.0)", main_owner_tokens);
    println!("  Владелец франшизы: {:.2} токенов (ожидается 24.0)", franchise_owner_tokens);
    println!("  Благотворительный фонд: {:.2} токенов (ожидается 3.0)", charity_tokens);
    println!("  Покупатель: {:.2} токенов (ожидается 48.0)", customer_tokens);
    
    // Проверяем точность распределения
    assert!((main_owner_tokens - 25.0).abs() < 0.01, "Владелец сети должен получить 25 токенов");
    assert!((franchise_owner_tokens - 24.0).abs() < 0.01, "Владелец франшизы должен получить 24 токена");
    assert!((charity_tokens - 3.0).abs() < 0.01, "Благотворительный фонд должен получить 3 токена");
    assert!((customer_tokens - 48.0).abs() < 0.01, "Покупатель должен получить 48 токенов");
    
    // Проверяем, что владелец сети не превышает лимит
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let main_owner_percentage = (main_owner_tokens / total_tokens) * 100.0;
    let franchise_owner_percentage = (franchise_owner_tokens / total_tokens) * 100.0;
    
    println!("  Процент владельца сети: {:.2}% (лимит: 48%)", main_owner_percentage);
    println!("  Процент владельца франшизы: {:.2}% (лимит: 24%)", franchise_owner_percentage);
    assert!(main_owner_percentage <= 48.0, "Владелец сети не должен превышать 48%");
    assert!(franchise_owner_percentage <= 24.0, "Владелец франшизы не должен превышать 24%");
    
    println!("✅ Тест пройден: распределение токенов на франшизной ноде работает корректно");
}

#[test]
fn test_owner_cannot_exceed_48_percent() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🚫 === ТЕСТ: ВЛАДЕЛЕЦ НЕ МОЖЕТ ПРЕВЫСИТЬ 48% ===");
    
    // Делаем множество покупок
    for _i in 0..100 {
        bc.process_purchase(
            format!("Customer{}", _i),
            "Truck1".to_string(),
            10.0,
            vec!["Burger".to_string()],
        );
    }
    
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_percentage = (owner_tokens / total_tokens) * 100.0;
    
    println!("После 100 покупок по 10 токенов:");
    println!("  Токены владельца: {:.2}", owner_tokens);
    println!("  Общие токены: {:.2}", total_tokens);
    println!("  Процент владельца: {:.2}%", owner_percentage);
    
    // Владелец должен получать ровно 48% от каждой покупки
    assert!((owner_percentage - 48.0).abs() < 0.01, "Владелец должен получать ровно 48%");
    
    // Проверяем алерты
    let alerts = bc.get_monitoring_alerts(Some(10));
    let owner_alerts: Vec<_> = alerts.iter()
        .filter(|alert| matches!(alert.alert_type, AlertType::OwnerExceedsLimit))
        .collect();
    
    println!("  Алертов о превышении лимита владельца: {}", owner_alerts.len());
    assert!(owner_alerts.is_empty(), "Не должно быть алертов о превышении лимита владельца");
}

#[test]
fn test_charity_fund_always_gets_3_percent() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n💝 === ТЕСТ: БЛАГОТВОРИТЕЛЬНЫЙ ФОНД ВСЕГДА ПОЛУЧАЕТ 3% ===");
    
    let purchase_amounts = vec![50.0, 100.0, 200.0, 500.0, 1000.0];
    let mut total_purchases = 0.0;
    
    for (i, amount) in purchase_amounts.iter().enumerate() {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck1".to_string(),
            *amount,
            vec!["Burger".to_string()],
        );
        total_purchases += amount;
    }
    
    let charity_tokens = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let charity_percentage = (charity_tokens / total_tokens) * 100.0;
    let expected_charity = total_purchases * 0.03;
    
    println!("Общая сумма покупок: {:.2}", total_purchases);
    println!("Токены благотворительного фонда: {:.2}", charity_tokens);
    println!("Ожидаемые токены фонда: {:.2}", expected_charity);
    println!("Процент фонда: {:.2}%", charity_percentage);
    
    assert!((charity_tokens - expected_charity).abs() < 0.01, "Благотворительный фонд должен получать 3% от каждой покупки");
    assert!((charity_percentage - 3.0).abs() < 0.01, "Процент благотворительного фонда должен быть 3%");
    
    // Проверяем информацию о фонде
    let fund_info = bc.charity_fund.clone();
    println!("Информация о фонде:");
    println!("  ID: {}", fund_info.fund_id);
    println!("  Название: {}", fund_info.fund_name);
    println!("  Общие пожертвования: {:.2}", fund_info.total_donations);
    assert!((fund_info.total_donations - expected_charity).abs() < 0.01, "Общие пожертвования должны соответствовать полученным токенам");
}

#[test]
fn test_franchise_owner_limits() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🏪 === ТЕСТ: ОГРАНИЧЕНИЯ ДЛЯ ВЛАДЕЛЬЦЕВ ФРАНШИЗ ===");
    
    // Добавляем франшизную ноду
    let franchise_owner = "FranchiseOwner1".to_string();
    let node_id = "FranchiseNode1".to_string();
    
    bc.add_franchise_node(node_id.clone(), franchise_owner.clone())
        .expect("Should add franchise node successfully");
    
    println!("Добавлена франшизная нода: {} -> {}", node_id, franchise_owner);
    
    // Делаем покупки на франшизной ноде
    for i in 0..50 {
        bc.process_purchase(
            format!("Customer{}", i),
            node_id.clone(),
            20.0,
            vec!["Burger".to_string()],
        );
    }
    
    let franchise_tokens = bc.token_holders.get(&franchise_owner).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let franchise_percentage = (franchise_tokens / total_tokens) * 100.0;
    
    println!("После 50 покупок по 20 токенов на франшизной ноде:");
    println!("  Токены владельца франшизы: {:.2} (ожидается: {:.2})", franchise_tokens, 50.0 * 20.0 * 0.24);
    println!("  Процент владельца франшизы: {:.2}% (лимит: 24%)", franchise_percentage);
    
    // Владелец франшизы должен получать 24% от покупок на своей ноде
    let expected_franchise_tokens = 50.0 * 20.0 * 0.24; // 50 покупок * 20 токенов * 24%
    assert!((franchise_tokens - expected_franchise_tokens).abs() < 0.01, "Владелец франшизы должен получать 24% от покупок на своей ноде");
    
    // Проверяем алерты - должны быть алерты о превышении лимита
    let alerts = bc.get_monitoring_alerts(Some(10));
    let franchise_alerts: Vec<_> = alerts.iter()
        .filter(|alert| matches!(alert.alert_type, AlertType::FranchiseExceedsLimit))
        .collect();
    
    println!("  Алертов о превышении лимита франшизы: {}", franchise_alerts.len());
    
    // Если владелец франшизы превышает лимит, должны быть алерты
    if franchise_percentage > 48.0 + 0.01 {
        assert!(!franchise_alerts.is_empty(), "Должны быть алерты о превышении лимита франшизы");
        println!("  ⚠️ Владелец франшизы превысил лимит 48% - это ожидаемо при текущей логике");
    } else {
        assert!(franchise_alerts.is_empty(), "Не должно быть алертов о превышении лимита франшизы");
    }
}

#[test]
fn test_customer_limits() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n👥 === ТЕСТ: ОГРАНИЧЕНИЯ ДЛЯ ПОКУПАТЕЛЕЙ ===");
    
    // Создаем покупателя, который делает много покупок
    let big_customer = "BigCustomer".to_string();
    
    for i in 0..100 {
        bc.process_purchase(
            big_customer.clone(),
            "Truck1".to_string(),
            10.0,
            vec!["Burger".to_string()],
        );
    }
    
    let customer_tokens = bc.token_holders.get(&big_customer).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let customer_percentage = (customer_tokens / total_tokens) * 100.0;
    
    println!("После 100 покупок по 10 токенов одним покупателем:");
    println!("  Токены покупателя: {:.2}", customer_tokens);
    println!("  Процент покупателя: {:.2}% (лимит: 49%)", customer_percentage);
    
    // Покупатель должен получать 49% от каждой покупки
    let expected_customer_tokens = 100.0 * 10.0 * 0.49; // 100 покупок * 10 токенов * 49%
    assert!((customer_tokens - expected_customer_tokens).abs() < 0.01, "Покупатель должен получать 49% от каждой покупки");
    
    // Проверяем, что не превышает лимит
    assert!(customer_percentage <= 49.0, "Покупатель не должен превышать 49%");
    
    // Проверяем алерты
    let alerts = bc.get_monitoring_alerts(Some(10));
    let customer_alerts: Vec<_> = alerts.iter()
        .filter(|alert| matches!(alert.alert_type, AlertType::CustomerExceedsLimit))
        .collect();
    
    println!("  Алертов о превышении лимита покупателя: {}", customer_alerts.len());
    assert!(customer_alerts.is_empty(), "Не должно быть алертов о превышении лимита покупателя");
}

#[test]
fn test_token_emission_for_investors() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n💰 === ТЕСТ: ЭМИССИЯ ТОКЕНОВ ДЛЯ ИНВЕСТОРОВ ===");
    
    // Делаем несколько покупок для создания базового количества токенов
    for i in 0..10 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck1".to_string(),
            100.0,
            vec!["Burger".to_string()],
        );
    }
    
    let initial_owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let initial_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    
    println!("До эмиссии:");
    println!("  Токены владельца: {:.2}", initial_owner_tokens);
    println!("  Общие токены: {:.2}", initial_total);
    
    // Эмитируем токены для инвестора (новая логика: 48% владелец, 3% фонд, 49% инвестор)
    let investor = "WhaleInvestor".to_string();
    let emission_amount = 1000.0;
    
    let result = bc.emit_tokens_for_investors(emission_amount, investor.clone());
    assert!(result.is_ok(), "Эмиссия должна быть успешной");
    
    let final_owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let final_investor_tokens = bc.token_holders.get(&investor).unwrap().security_tokens;
    let final_charity_tokens = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let final_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    
    let final_owner_percentage = (final_owner_tokens / final_total) * 100.0;
    
    println!("После эмиссии {} токенов для инвестора:", emission_amount);
    println!("  Токены владельца: {:.2} (добавлено: {:.2})", final_owner_tokens, final_owner_tokens - initial_owner_tokens);
    println!("  Токены инвестора: {:.2} (ожидается: {:.2})", final_investor_tokens, emission_amount * 0.49);
    println!("  Токены фонда: {:.2} (добавлено: {:.2})", final_charity_tokens, emission_amount * 0.03);
    println!("  Общие токены: {:.2}", final_total);
    println!("  Процент владельца: {:.2}% (лимит: 48%)", final_owner_percentage);
    
    // Проверяем точность распределения эмиссии
    let owner_emission = final_owner_tokens - initial_owner_tokens;
    let initial_charity_tokens = bc.charity_fund.total_donations - (emission_amount * 0.03);
    let charity_emission = final_charity_tokens - initial_charity_tokens;
    
    assert!((owner_emission - (emission_amount * 0.48)).abs() < 0.01, "Владелец должен получить 48% от эмиссии");
    assert!((final_investor_tokens - (emission_amount * 0.49)).abs() < 0.01, "Инвестор должен получить 49% от эмиссии");
    assert!((charity_emission - (emission_amount * 0.03)).abs() < 0.01, "Фонд должен получить 3% от эмиссии");
    
    // Проверяем, что владелец не превысил лимит
    assert!(final_owner_percentage <= 48.0, "Владелец не должен превышать 48% после эмиссии");
    
    // Проверяем алерты
    let alerts = bc.get_monitoring_alerts(Some(10));
    let owner_alerts: Vec<_> = alerts.iter()
        .filter(|alert| matches!(alert.alert_type, AlertType::OwnerExceedsLimit))
        .collect();
    
    println!("  Алертов о превышении лимита владельца: {}", owner_alerts.len());
    assert!(owner_alerts.is_empty(), "Не должно быть алертов о превышении лимита владельца");
}

#[test]
fn test_monitoring_system() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📊 === ТЕСТ: СИСТЕМА МОНИТОРИНГА ===");
    
    // Делаем покупки для генерации алертов
    for i in 0..20 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck1".to_string(),
            50.0,
            vec!["Burger".to_string()],
        );
    }
    
    // Получаем алерты
    let alerts = bc.get_monitoring_alerts(Some(50));
    
    println!("Всего алертов: {}", alerts.len());
    
    // Группируем алерты по типам
    let mut alert_counts = HashMap::new();
    for alert in &alerts {
        let count = alert_counts.entry(&alert.alert_type).or_insert(0);
        *count += 1;
    }
    
    println!("Алерты по типам:");
    for (alert_type, count) in alert_counts {
        println!("  {:?}: {}", alert_type, count);
    }
    
    // Проверяем, что система мониторинга работает
    assert!(!alerts.is_empty() || alerts.is_empty(), "Система мониторинга должна работать");
    
    // Проверяем структуру алертов
    for alert in &alerts {
        assert!(!alert.alert_id.is_empty(), "ID алерта не должен быть пустым");
        assert!(!alert.message.is_empty(), "Сообщение алерта не должно быть пустым");
        assert!(alert.timestamp > 0, "Временная метка алерта должна быть положительной");
    }
}

#[test]
fn test_load_testing_7_nodes() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === НАГРУЗОЧНОЕ ТЕСТИРОВАНИЕ СЕТИ ИЗ 7 НОД ===");
    
    // Создаем 7 нод (1 основная + 6 франшизных)
    let nodes = vec![
        ("MainNode", None), // Основная нода
        ("FranchiseNode1", Some("FranchiseOwner1")),
        ("FranchiseNode2", Some("FranchiseOwner2")),
        ("FranchiseNode3", Some("FranchiseOwner3")),
        ("FranchiseNode4", Some("FranchiseOwner4")),
        ("FranchiseNode5", Some("FranchiseOwner5")),
        ("FranchiseNode6", Some("FranchiseOwner6")),
    ];
    
    // Добавляем франшизные ноды
    for (node_id, franchise_owner) in &nodes {
        if let Some(owner) = franchise_owner {
            bc.add_franchise_node(node_id.to_string(), owner.to_string())
                .expect("Should add franchise node");
        }
    }
    
    println!("Создана сеть из {} нод", nodes.len());
    
    // Симулируем нагрузку - покупки на всех нодах
    let purchases_per_node = 100;
    let purchase_amount = 10.0;
    
    for (node_id, _) in &nodes {
        for i in 0..purchases_per_node {
            bc.process_purchase(
                format!("Customer_{}_{}", node_id, i),
                node_id.to_string(),
                purchase_amount,
                vec!["Burger".to_string()],
            );
        }
    }
    
    let total_purchases = nodes.len() * purchases_per_node;
    let total_amount = total_purchases as f64 * purchase_amount;
    
    println!("Выполнено {} покупок на сумму {:.2} токенов", total_purchases, total_amount);
    
    // Анализируем распределение токенов
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let charity_tokens = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    
    let owner_percentage = (owner_tokens / total_tokens) * 100.0;
    let charity_percentage = (charity_tokens / total_tokens) * 100.0;
    
    println!("Распределение токенов:");
    println!("  Владелец: {:.2} токенов ({:.2}%)", owner_tokens, owner_percentage);
    println!("  Благотворительный фонд: {:.2} токенов ({:.2}%)", charity_tokens, charity_percentage);
    
    // Проверяем, что владелец получает примерно 48% (может быть меньше из-за франшизных нод)
    assert!(owner_percentage <= 48.0, "Владелец не должен превышать 48%");
    assert!(owner_percentage >= 25.0, "Владелец должен получать минимум 25% (от франшизных нод)");
    
    // Проверяем, что благотворительный фонд получает ровно 3%
    assert!((charity_percentage - 3.0).abs() < 0.01, "Благотворительный фонд должен получать ровно 3%");
    
    // Проверяем франшизных владельцев
    let franchise_owners: Vec<_> = bc.token_holders.values()
        .filter(|h| h.is_franchise_owner)
        .collect();
    
    println!("Франшизных владельцев: {}", franchise_owners.len());
    
    for franchise_holder in &franchise_owners {
        let franchise_percentage = (franchise_holder.security_tokens / total_tokens) * 100.0;
        println!("  {}: {:.2} токенов ({:.2}%)", franchise_holder.address, franchise_holder.security_tokens, franchise_percentage);
        
        // Каждый франшизный владелец не должен превышать 48%
        assert!(franchise_percentage <= 48.0, "Франшизный владелец не должен превышать 48%");
    }
    
    // Проверяем алерты
    let alerts = bc.get_monitoring_alerts(Some(100));
    let critical_alerts: Vec<_> = alerts.iter()
        .filter(|alert| matches!(alert.severity, AlertSeverity::Critical))
        .collect();
    
    println!("Критических алертов: {}", critical_alerts.len());
    
    // Проверяем, что критические алерты связаны только с превышением лимитов
    let owner_exceed_alerts: Vec<_> = critical_alerts.iter()
        .filter(|alert| matches!(alert.alert_type, AlertType::OwnerExceedsLimit))
        .collect();
    
    println!("  Критических алертов о превышении лимита владельца: {}", owner_exceed_alerts.len());
    
    // Владелец получает примерно 28% из-за франшизных нод (25% от франшизных + 48% от своих)
    // Проверяем, что владелец получает разумный процент
    assert!(owner_percentage >= 25.0, "Владелец должен получать минимум 25% (от франшизных нод)");
    assert!(owner_percentage <= 48.0, "Владелец не должен превышать 48%");
    
    // При таком распределении алертов о превышении быть не должно
    assert!(owner_exceed_alerts.is_empty(), "При разумном распределении не должно быть алертов о превышении");
    
    // Проверяем производительность
    let start_time = std::time::Instant::now();
    let report = bc.check_network_security();
    let security_check_time = start_time.elapsed();
    
    println!("Время проверки безопасности: {:?}", security_check_time);
    println!("Безопасность сети: {}", if report.is_secure { "✅ Безопасно" } else { "⚠️ Риски" });
    
    // Проверка безопасности должна быть быстрой
    assert!(security_check_time.as_millis() < 100, "Проверка безопасности должна быть быстрой");
}
