use crate::*;

/// Тесты для системы распределения невостребованных токенов
/// 
/// Этот модуль тестирует:
/// 1. Отслеживание невостребованных токенов
/// 2. Годовое распределение пропорционально доле владения
/// 3. API для работы с невостребованными токенами
/// 4. Проверку истечения токенов

#[test]
fn test_unclaimed_tokens_tracking() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📋 === ТЕСТ: ОТСЛЕЖИВАНИЕ НЕВОСТРЕБОВАННЫХ ТОКЕНОВ ===");
    
    // Делаем покупки без привязки к телефону (невостребованные)
    let _check1 = bc.process_purchase(
        "Customer1".to_string(),
        "Truck1".to_string(),
        100.0,
        vec!["Burger".to_string()],
    );
    
    let _check2 = bc.process_purchase(
        "Customer2".to_string(),
        "Truck1".to_string(),
        200.0,
        vec!["Pizza".to_string()],
    );
    
    // Делаем покупку с привязкой к телефону (востребованная)
    // Сначала регистрируем пользователя
    let phone = "+1234567890".to_string();
    let wallet = "Customer3".to_string();
    bc.register_user_with_phone(phone.clone(), wallet.clone()).expect("Should register user");
    
    // Теперь делаем покупку - она должна быть востребованной
    let _check3 = bc.process_purchase(
        wallet,
        "Truck1".to_string(),
        150.0,
        vec!["Salad".to_string()],
    );
    
    let unclaimed_tokens = bc.get_unclaimed_tokens(Some(10));
    
    println!("Невостребованных токенов: {}", unclaimed_tokens.len());
    
    // Должно быть 2 невостребованных токена (check1 и check2)
    assert_eq!(unclaimed_tokens.len(), 2, "Должно быть 2 невостребованных токена");
    
    // Проверяем суммы
    let total_unclaimed: f64 = unclaimed_tokens.iter().map(|r| r.amount).sum();
    let expected_unclaimed = (100.0 * 0.49) + (200.0 * 0.49); // 49% от каждой покупки
    
    println!("Общая сумма невостребованных токенов: {:.2}", total_unclaimed);
    println!("Ожидаемая сумма: {:.2}", expected_unclaimed);
    
    assert!((total_unclaimed - expected_unclaimed).abs() < 0.01, "Сумма невостребованных токенов должна быть корректной");
    
    // Проверяем, что токены не распределены
    for record in &unclaimed_tokens {
        assert!(!record.is_distributed, "Токены не должны быть распределены");
        assert!(record.distributed_timestamp.is_none(), "Время распределения должно быть None");
    }
}

#[test]
fn test_annual_distribution_proportional() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n💰 === ТЕСТ: ГОДОВОЕ РАСПРЕДЕЛЕНИЕ ПРОПОРЦИОНАЛЬНО ДОЛЕ ВЛАДЕНИЯ ===");
    
    // Создаем несколько держателей токенов
    bc.process_purchase("Customer1".to_string(), "Truck1".to_string(), 1000.0, vec!["Burger".to_string()]);
    bc.process_purchase("Customer2".to_string(), "Truck1".to_string(), 1000.0, vec!["Pizza".to_string()]);
    bc.process_purchase("Customer3".to_string(), "Truck1".to_string(), 1000.0, vec!["Salad".to_string()]);
    
    // Получаем начальные балансы
    let owner_tokens_before = bc.token_holders.get(&owner).unwrap().security_tokens;
    let charity_tokens_before = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
    let customer1_tokens_before = bc.token_holders.get("Customer1").unwrap().security_tokens;
    
    println!("До распределения:");
    println!("  Владелец: {:.2} токенов", owner_tokens_before);
    println!("  Благотворительный фонд: {:.2} токенов", charity_tokens_before);
    println!("  Customer1: {:.2} токенов", customer1_tokens_before);
    
    // Симулируем истечение невостребованных токенов
    let unclaimed_tokens = bc.get_unclaimed_tokens(Some(10));
    let total_unclaimed: f64 = unclaimed_tokens.iter().map(|r| r.amount).sum();
    
    println!("Невостребованных токенов для распределения: {:.2}", total_unclaimed);
    
    // Устанавливаем срок истечения в прошлое для всех невостребованных токенов
    for record in &mut bc.unclaimed_tokens {
        record.expiry_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1;
    }
    
    // Вычисляем общее количество токенов для пропорционального распределения
    let _total_security_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    
    // Выполняем годовое распределение
    let distribution_result = bc.distribute_unclaimed_tokens_annually();
    
    match distribution_result {
        Ok(_distribution) => {
            println!("Распределение выполнено успешно!");
            println!("Год: {}", _distribution.year);
            println!("Общая сумма распределенных токенов: {:.2}", _distribution.total_unclaimed_tokens);
            println!("Количество получателей: {}", _distribution.distributions.len());
            
            // Проверяем, что распределение пропорционально
            for dist in &_distribution.distributions {
                println!("  {}: {:.2} токенов ({:.2}%)", dist.recipient_address, dist.amount, dist.percentage);
            }
            
            // Проверяем, что общая сумма распределения равна невостребованным токенам
            let total_distributed: f64 = _distribution.distributions.iter().map(|d| d.amount).sum();
            assert!((total_distributed - _distribution.total_unclaimed_tokens).abs() < 0.01, 
                "Общая сумма распределения должна равняться невостребованным токенам");
            
            // Проверяем, что токены добавлены к балансам
            let owner_tokens_after = bc.token_holders.get(&owner).unwrap().security_tokens;
            let charity_tokens_after = bc.token_holders.get(&bc.charity_fund.fund_id).unwrap().security_tokens;
            let customer1_tokens_after = bc.token_holders.get("Customer1").unwrap().security_tokens;
            
            println!("После распределения:");
            println!("  Владелец: {:.2} токенов (+{:.2})", owner_tokens_after, owner_tokens_after - owner_tokens_before);
            println!("  Благотворительный фонд: {:.2} токенов (+{:.2})", charity_tokens_after, charity_tokens_after - charity_tokens_before);
            println!("  Customer1: {:.2} токенов (+{:.2})", customer1_tokens_after, customer1_tokens_after - customer1_tokens_before);
            
            // Проверяем, что токены действительно добавлены
            assert!(owner_tokens_after > owner_tokens_before, "Владелец должен получить дополнительные токены");
            assert!(charity_tokens_after > charity_tokens_before, "Благотворительный фонд должен получить дополнительные токены");
            assert!(customer1_tokens_after > customer1_tokens_before, "Customer1 должен получить дополнительные токены");
            
        },
        Err(e) => {
            panic!("Ошибка при распределении: {}", e);
        }
    }
}

#[test]
fn test_expired_tokens_check() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n⏰ === ТЕСТ: ПРОВЕРКА ИСТЕЧЕНИЯ НЕВОСТРЕБОВАННЫХ ТОКЕНОВ ===");
    
    // Делаем покупки
    bc.process_purchase("Customer1".to_string(), "Truck1".to_string(), 100.0, vec!["Burger".to_string()]);
    bc.process_purchase("Customer2".to_string(), "Truck1".to_string(), 200.0, vec!["Pizza".to_string()]);
    
    let unclaimed_tokens = bc.get_unclaimed_tokens(Some(10));
    println!("Создано невостребованных токенов: {}", unclaimed_tokens.len());
    
    // Проверяем истекшие токены (в реальности они не истекли, так как только что созданы)
    let expired_checks = bc.check_expired_unclaimed_tokens();
    println!("Истекших токенов: {}", expired_checks.len());
    
    // В нормальных условиях не должно быть истекших токенов
    assert_eq!(expired_checks.len(), 0, "Не должно быть истекших токенов сразу после создания");
    
    // Проверяем, что все токены помечены как нераспределенные
    for record in &unclaimed_tokens {
        assert!(!record.is_distributed, "Токены не должны быть распределены");
        assert!(record.distributed_timestamp.is_none(), "Время распределения должно быть None");
    }
}

#[test]
fn test_annual_distributions_history() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📊 === ТЕСТ: ИСТОРИЯ ГОДОВЫХ РАСПРЕДЕЛЕНИЙ ===");
    
    // Делаем покупки для создания невостребованных токенов
    bc.process_purchase("Customer1".to_string(), "Truck1".to_string(), 500.0, vec!["Burger".to_string()]);
    bc.process_purchase("Customer2".to_string(), "Truck1".to_string(), 500.0, vec!["Pizza".to_string()]);
    
    // Устанавливаем срок истечения в прошлое для всех невостребованных токенов
    for record in &mut bc.unclaimed_tokens {
        record.expiry_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1;
    }
    
    // Выполняем распределение
    let distribution_result = bc.distribute_unclaimed_tokens_annually();
    
    match distribution_result {
        Ok(_) => {
            // Получаем историю распределений
            let distributions = bc.get_annual_distributions(Some(10));
            
            println!("История распределений: {} записей", distributions.len());
            
            assert_eq!(distributions.len(), 1, "Должна быть одна запись о распределении");
            
            let distribution = &distributions[0];
            println!("Год: {}", distribution.year);
            println!("Общая сумма: {:.2}", distribution.total_unclaimed_tokens);
            println!("Время распределения: {}", distribution.distribution_timestamp);
            println!("Количество получателей: {}", distribution.distributions.len());
            
            // Проверяем структуру распределения
            assert_eq!(distribution.year, 2024, "Год должен быть 2024");
            assert!(distribution.total_unclaimed_tokens > 0.0, "Общая сумма должна быть положительной");
            assert!(!distribution.distributions.is_empty(), "Должны быть получатели");
            
            // Проверяем, что все невостребованные токены помечены как распределенные
            let unclaimed_tokens = bc.get_unclaimed_tokens(Some(10));
            for record in &unclaimed_tokens {
                assert!(record.is_distributed, "Все токены должны быть помечены как распределенные");
                assert!(record.distributed_timestamp.is_some(), "Время распределения должно быть установлено");
            }
            
        },
        Err(e) => {
            panic!("Ошибка при распределении: {}", e);
        }
    }
}

#[test]
fn test_no_unclaimed_tokens_distribution() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🚫 === ТЕСТ: РАСПРЕДЕЛЕНИЕ БЕЗ НЕВОСТРЕБОВАННЫХ ТОКЕНОВ ===");
    
    // Не создаем невостребованные токены
    // Пытаемся выполнить распределение
    let distribution_result = bc.distribute_unclaimed_tokens_annually();
    
    match distribution_result {
        Ok(_) => {
            panic!("Не должно быть успешного распределения без невостребованных токенов");
        },
        Err(e) => {
            println!("Ожидаемая ошибка: {}", e);
            assert!(e.contains("Нет невостребованных токенов"), "Ошибка должна указывать на отсутствие невостребованных токенов");
        }
    }
}

#[test]
fn test_distribution_preserves_ownership_limits() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🛡️ === ТЕСТ: РАСПРЕДЕЛЕНИЕ СОХРАНЯЕТ ОГРАНИЧЕНИЯ ВЛАДЕНИЯ ===");
    
    // Создаем много невостребованных токенов
    for i in 0..50 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck1".to_string(),
            100.0,
            vec!["Burger".to_string()],
        );
    }
    
    let total_unclaimed: f64 = bc.get_unclaimed_tokens(Some(100)).iter().map(|r| r.amount).sum();
    println!("Общая сумма невостребованных токенов: {:.2}", total_unclaimed);
    
    // Устанавливаем срок истечения в прошлое для всех невостребованных токенов
    for record in &mut bc.unclaimed_tokens {
        record.expiry_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1;
    }
    
    // Выполняем распределение
    let distribution_result = bc.distribute_unclaimed_tokens_annually();
    
    match distribution_result {
        Ok(distribution) => {
            println!("Распределение выполнено успешно");
            
            // Проверяем, что владелец не превысил лимит
            let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
            let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
            let owner_percentage = (owner_tokens / total_tokens) * 100.0;
            
            println!("Процент владельца после распределения: {:.2}%", owner_percentage);
            
            // Владелец не должен превышать 48%
            assert!(owner_percentage <= 48.0, "Владелец не должен превышать лимит 48% после распределения");
            
            // Проверяем алерты
            let alerts = bc.get_monitoring_alerts(Some(10));
            let owner_alerts: Vec<_> = alerts.iter()
                .filter(|alert| matches!(alert.alert_type, AlertType::OwnerExceedsLimit))
                .collect();
            
            println!("Алертов о превышении лимита владельца: {}", owner_alerts.len());
            
            // Не должно быть критических алертов
            assert!(owner_alerts.is_empty(), "Не должно быть алертов о превышении лимита владельца");
            
        },
        Err(e) => {
            panic!("Ошибка при распределении: {}", e);
        }
    }
}
