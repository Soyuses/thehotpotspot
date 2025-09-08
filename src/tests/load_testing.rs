use crate::*;
use std::time::Instant;
use std::collections::HashMap;

/// Нагрузочное тестирование системы безопасности и концентрации токенов
/// 
/// Этот модуль тестирует:
/// 1. Производительность системы при больших нагрузках
/// 2. Стабильность при множественных операциях
/// 3. Эффективность механизмов безопасности
/// 4. Масштабируемость системы

#[test]
fn test_high_volume_purchase_performance() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n⚡ === НАГРУЗОЧНОЕ ТЕСТИРОВАНИЕ: ВЫСОКИЙ ОБЪЕМ ПОКУПОК ===");
    
    let test_scenarios = vec![
        (100, "Низкая нагрузка"),
        (1000, "Средняя нагрузка"),
        (5000, "Высокая нагрузка"),
        (10000, "Экстремальная нагрузка"),
    ];
    
    for (num_purchases, scenario_name) in test_scenarios {
        println!("\n--- {}: {} покупок ---", scenario_name, num_purchases);
        
        let start_time = Instant::now();
        let mut bc_test = bc.clone();
        
        // Выполняем покупки
        for i in 0..num_purchases {
            let purchase_amount = 10.0 + (i as f64 % 50.0); // От 10 до 60
            bc_test.process_purchase(
                format!("LoadTest_Customer_{}", i),
                "Truck".to_string(),
                purchase_amount,
                vec!["Meal".to_string()],
            );
        }
        
        let purchase_time = start_time.elapsed();
        
        // Тестируем производительность проверки безопасности
        let security_start = Instant::now();
        let report = bc_test.check_network_security();
        let security_time = security_start.elapsed();
        
        // Анализируем результаты
        let total_tokens: f64 = bc_test.token_holders.values().map(|h| h.security_tokens).sum();
        let owner_tokens = bc_test.token_holders.get(&owner).unwrap().security_tokens;
        let owner_percentage = (owner_tokens / total_tokens) * 100.0;
        
        println!("  Время выполнения покупок: {:?}", purchase_time);
        println!("  Время проверки безопасности: {:?}", security_time);
        println!("  Покупок в секунду: {:.2}", num_purchases as f64 / purchase_time.as_secs_f64());
        println!("  Токены владельца: {:.2}", owner_tokens);
        println!("  Процент владельца: {:.2}%", owner_percentage);
        println!("  Безопасность: {}", if report.is_secure { "✅" } else { "⚠️" });
        println!("  Рисков: {}", report.security_risks.len());
        
        // Проверяем, что система остается стабильной
        assert!(bc_test.is_chain_valid());
        assert!(purchase_time.as_secs() < 60); // Не более минуты на тест
    }
}

#[test]
fn test_concurrent_balance_transfers() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🔄 === НАГРУЗОЧНОЕ ТЕСТИРОВАНИЕ: ПАРАЛЛЕЛЬНЫЕ ПЕРЕНОСЫ БАЛАНСА ===");
    
    // Создаем множество чеков для переноса
    let num_checks = 100;
    let mut checks = Vec::new();
    
    for i in 0..num_checks {
        let check = bc.process_purchase(
            format!("TransferCustomer{}", i),
            "Truck".to_string(),
            15.0,
            vec!["Meal".to_string()],
        );
        checks.push(check);
    }
    
    // Регистрируем пользователей
    let mut users = Vec::new();
    for i in 0..num_checks {
        let phone = format!("+123456789{}", i);
        let wallet = format!("0xwallet{}", i);
        let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
            .expect("registration should succeed");
        bc.verify_phone_number(phone.clone(), verification_code)
            .expect("verification should succeed");
        users.push((phone, wallet));
    }
    
    // Тестируем параллельные переносы
    let start_time = Instant::now();
    let mut successful_transfers = 0;
    let mut failed_transfers = 0;
    
    for (i, check) in checks.iter().enumerate() {
        let (phone, _) = &users[i];
        match bc.transfer_balance_from_check(check.check_id.clone(), phone.clone()) {
            Ok(_) => successful_transfers += 1,
            Err(_) => failed_transfers += 1,
        }
    }
    
    let transfer_time = start_time.elapsed();
    
    println!("  Время выполнения переносов: {:?}", transfer_time);
    println!("  Успешных переносов: {}", successful_transfers);
    println!("  Неудачных переносов: {}", failed_transfers);
    println!("  Переносов в секунду: {:.2}", num_checks as f64 / transfer_time.as_secs_f64());
    println!("  Процент успеха: {:.2}%", (successful_transfers as f64 / num_checks as f64) * 100.0);
    
    // Проверяем финальное состояние
    let report = bc.check_network_security();
    println!("  Финальная безопасность: {}", if report.is_secure { "✅" } else { "⚠️" });
    println!("  Количество рисков: {}", report.security_risks.len());
    
    // Проверяем, что система остается стабильной
    assert!(bc.is_chain_valid());
    assert!(transfer_time.as_secs() < 30); // Не более 30 секунд
}

#[test]
fn test_memory_usage_under_load() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n💾 === НАГРУЗОЧНОЕ ТЕСТИРОВАНИЕ: ИСПОЛЬЗОВАНИЕ ПАМЯТИ ===");
    
    let test_sizes = vec![100, 500, 1000, 2000];
    
    for size in test_sizes {
        println!("\n--- Тест с {} элементами ---", size);
        
        let mut bc_test = bc.clone();
        
        // Создаем множество держателей токенов
        for i in 0..size {
            let mut holder = TokenHolder::new(format!("holder{}", i), false);
            holder.add_security_tokens(10.0 + (i as f64 % 100.0));
            bc_test.token_holders.insert(format!("holder{}", i), holder);
        }
        
        // Создаем множество авторизованных пользователей
        for i in 0..size {
            let phone = format!("+123456789{}", i);
            let wallet = format!("0xwallet{}", i);
            let user = AuthorizedUser::new(phone.clone(), wallet.clone());
            bc_test.authorized_users.insert(phone, user);
        }
        
        // Создаем множество записей переноса баланса
        for i in 0..size {
            let record = BalanceTransferRecord {
                transfer_id: format!("TRANSFER_{}", i),
                from_check_id: format!("CHECK_{}", i),
                from_wallet: format!("0xfrom{}", i),
                to_wallet: format!("0xto{}", i),
                to_phone: format!("+123456789{}", i),
                security_tokens_transferred: 10.0,
                utility_tokens_transferred: 1.0,
                timestamp: 1234567890 + i as u64,
                status: TransferStatus::Completed,
            };
            bc_test.balance_transfer_history.push(record);
        }
        
        // Тестируем производительность операций
        let start_time = Instant::now();
        let report = bc_test.check_network_security();
        let security_time = start_time.elapsed();
        
        let history_start = Instant::now();
        let history = bc_test.get_balance_transfer_history(Some(100));
        let history_time = history_start.elapsed();
        
        println!("  Держателей токенов: {}", bc_test.token_holders.len());
        println!("  Авторизованных пользователей: {}", bc_test.authorized_users.len());
        println!("  Записей переноса: {}", bc_test.balance_transfer_history.len());
        println!("  Время проверки безопасности: {:?}", security_time);
        println!("  Время получения истории: {:?}", history_time);
        println!("  Безопасность: {}", if report.is_secure { "✅" } else { "⚠️" });
        println!("  Рисков: {}", report.security_risks.len());
        
        // Проверяем, что система остается стабильной
        assert!(bc_test.is_chain_valid());
        assert!(security_time.as_millis() < 1000); // Не более 1 секунды
    }
}

#[test]
fn test_stress_testing_security_checks() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🛡️ === СТРЕСС-ТЕСТИРОВАНИЕ ПРОВЕРОК БЕЗОПАСНОСТИ ===");
    
    // Создаем сложную сеть с множественными рисками
    let mut bc_stress = bc.clone();
    
    // Создаем множество держателей с разными уровнями риска
    for i in 0..100 {
        let mut holder = TokenHolder::new(format!("risky_holder{}", i), false);
        let tokens = match i % 10 {
            0 => 1000.0, // Высокий риск
            1..=3 => 500.0, // Средний риск
            _ => 50.0, // Низкий риск
        };
        holder.add_security_tokens(tokens);
        bc_stress.token_holders.insert(format!("risky_holder{}", i), holder);
    }
    
    // Создаем "китов" с большим количеством токенов
    for i in 0..10 {
        let mut whale = TokenHolder::new(format!("whale{}", i), false);
        whale.add_security_tokens(2000.0 + (i as f64 * 100.0));
        bc_stress.token_holders.insert(format!("whale{}", i), whale);
    }
    
    // Тестируем производительность проверки безопасности
    let iterations = 1000;
    let start_time = Instant::now();
    
    for _ in 0..iterations {
        let report = bc_stress.check_network_security();
        // Проверяем, что отчет генерируется корректно
        assert!(report.total_security_tokens > 0.0);
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / iterations;
    
    println!("  Итераций: {}", iterations);
    println!("  Общее время: {:?}", total_time);
    println!("  Среднее время на проверку: {:?}", avg_time);
    println!("  Проверок в секунду: {:.2}", iterations as f64 / total_time.as_secs_f64());
    
    // Проверяем финальное состояние
    let final_report = bc_stress.check_network_security();
    println!("  Финальная безопасность: {}", if final_report.is_secure { "✅" } else { "⚠️" });
    println!("  Количество рисков: {}", final_report.security_risks.len());
    println!("  Общие токены: {:.2}", final_report.total_security_tokens);
    
    // Проверяем, что система остается стабильной
    assert!(bc_stress.is_chain_valid());
    assert!(avg_time.as_millis() < 10); // Не более 10мс на проверку
}

#[test]
fn test_scalability_limits() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n📈 === ТЕСТИРОВАНИЕ ПРЕДЕЛОВ МАСШТАБИРУЕМОСТИ ===");
    
    let scale_tests = vec![
        (1000, "1K элементов"),
        (5000, "5K элементов"),
        (10000, "10K элементов"),
        (20000, "20K элементов"),
    ];
    
    for (size, description) in scale_tests {
        println!("\n--- {} ---", description);
        
        let mut bc_scale = bc.clone();
        
        // Создаем элементы в указанном количестве
        let start_time = Instant::now();
        
        for i in 0..size {
            let mut holder = TokenHolder::new(format!("scale_holder{}", i), false);
            holder.add_security_tokens(10.0 + (i as f64 % 50.0));
            bc_scale.token_holders.insert(format!("scale_holder{}", i), holder);
        }
        
        let creation_time = start_time.elapsed();
        
        // Тестируем операции
        let ops_start = Instant::now();
        
        // Операция 1: Поиск держателя
        let search_start = Instant::now();
        let _ = bc_scale.token_holders.get("scale_holder5000");
        let search_time = search_start.elapsed();
        
        // Операция 2: Проверка безопасности
        let security_start = Instant::now();
        let report = bc_scale.check_network_security();
        let security_time = security_start.elapsed();
        
        // Операция 3: Обновление ролей
        let roles_start = Instant::now();
        bc_scale.update_roles();
        let roles_time = roles_start.elapsed();
        
        let ops_time = ops_start.elapsed();
        
        println!("  Время создания: {:?}", creation_time);
        println!("  Время поиска: {:?}", search_time);
        println!("  Время проверки безопасности: {:?}", security_time);
        println!("  Время обновления ролей: {:?}", roles_time);
        println!("  Общее время операций: {:?}", ops_time);
        println!("  Элементов в секунду: {:.2}", size as f64 / creation_time.as_secs_f64());
        println!("  Безопасность: {}", if report.is_secure { "✅" } else { "⚠️" });
        println!("  Рисков: {}", report.security_risks.len());
        
        // Проверяем, что система остается стабильной
        assert!(bc_scale.is_chain_valid());
        assert!(creation_time.as_secs() < 60); // Не более минуты на создание
        assert!(security_time.as_millis() < 5000); // Не более 5 секунд на проверку безопасности
    }
}

#[test]
fn test_network_growth_simulation() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌱 === СИМУЛЯЦИЯ РОСТА СЕТИ ===");
    
    let growth_phases = vec![
        (100, "Фаза 1: Стартап (100 пользователей)"),
        (500, "Фаза 2: Рост (500 пользователей)"),
        (1000, "Фаза 3: Расширение (1K пользователей)"),
        (5000, "Фаза 4: Масштабирование (5K пользователей)"),
        (10000, "Фаза 5: Зрелость (10K пользователей)"),
    ];
    
    for (target_size, phase_name) in growth_phases {
        println!("\n--- {} ---", phase_name);
        
        // Добавляем пользователей до целевого размера
        let current_size = bc.token_holders.len();
        let users_to_add = target_size - current_size;
        
        let start_time = Instant::now();
        
        for i in 0..users_to_add {
            let mut holder = TokenHolder::new(format!("growth_user{}", i), false);
            holder.add_security_tokens(10.0 + (i as f64 % 100.0));
            bc.token_holders.insert(format!("growth_user{}", i), holder);
        }
        
        let growth_time = start_time.elapsed();
        
        // Анализируем состояние сети
        let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let owner_percentage = (owner_tokens / total_tokens) * 100.0;
        
        let report = bc.check_network_security();
        
        println!("  Пользователей: {}", bc.token_holders.len());
        println!("  Время роста: {:?}", growth_time);
        println!("  Общие токены: {:.2}", total_tokens);
        println!("  Процент владельца: {:.2}%", owner_percentage);
        println!("  Безопасность: {}", if report.is_secure { "✅" } else { "⚠️" });
        println!("  Рисков: {}", report.security_risks.len());
        
        // Анализируем распределение власти
        let mut power_distribution = HashMap::new();
        for holder in bc.token_holders.values() {
            let percentage = (holder.security_tokens / total_tokens) * 100.0;
            let category = match percentage {
                p if p >= 10.0 => "10%+",
                p if p >= 5.0 => "5-10%",
                p if p >= 1.0 => "1-5%",
                _ => "<1%",
            };
            *power_distribution.entry(category).or_insert(0) += 1;
        }
        
        println!("  Распределение власти:");
        for (category, count) in power_distribution {
            println!("    {}: {} пользователей", category, count);
        }
        
        // Проверяем, что система остается стабильной
        assert!(bc.is_chain_valid());
        assert!(growth_time.as_secs() < 30); // Не более 30 секунд на фазу
    }
}

#[test]
fn test_extreme_scenarios() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🚨 === ЭКСТРЕМАЛЬНЫЕ СЦЕНАРИИ ===");
    
    // Сценарий 1: Один "кит" с 90% токенов
    println!("\n--- Сценарий 1: Кит с 90% токенов ---");
    let mut whale = TokenHolder::new("mega_whale".to_string(), false);
    whale.add_security_tokens(9000.0);
    bc.token_holders.insert("mega_whale".to_string(), whale);
    
    // Добавляем мелких держателей
    for i in 0..1000 {
        let mut holder = TokenHolder::new(format!("small_holder{}", i), false);
        holder.add_security_tokens(1.0);
        bc.token_holders.insert(format!("small_holder{}", i), holder);
    }
    
    let report1 = bc.check_network_security();
    println!("  Безопасность: {}", if report1.is_secure { "✅" } else { "🚨 ОПАСНО" });
    println!("  Рисков: {}", report1.security_risks.len());
    
    // Сценарий 2: Равномерное распределение
    println!("\n--- Сценарий 2: Равномерное распределение ---");
    let mut bc_even = Blockchain::new(owner.clone());
    
    for i in 0..1000 {
        let mut holder = TokenHolder::new(format!("even_holder{}", i), false);
        holder.add_security_tokens(10.0);
        bc_even.token_holders.insert(format!("even_holder{}", i), holder);
    }
    
    let report2 = bc_even.check_network_security();
    println!("  Безопасность: {}", if report2.is_secure { "✅" } else { "⚠️" });
    println!("  Рисков: {}", report2.security_risks.len());
    
    // Сценарий 3: Два крупных игрока
    println!("\n--- Сценарий 3: Два крупных игрока ---");
    let mut bc_duopoly = Blockchain::new(owner.clone());
    
    let mut player1 = TokenHolder::new("player1".to_string(), false);
    player1.add_security_tokens(4000.0);
    bc_duopoly.token_holders.insert("player1".to_string(), player1);
    
    let mut player2 = TokenHolder::new("player2".to_string(), false);
    player2.add_security_tokens(4000.0);
    bc_duopoly.token_holders.insert("player2".to_string(), player2);
    
    // Добавляем мелких игроков
    for i in 0..100 {
        let mut holder = TokenHolder::new(format!("small_player{}", i), false);
        holder.add_security_tokens(20.0);
        bc_duopoly.token_holders.insert(format!("small_player{}", i), holder);
    }
    
    let report3 = bc_duopoly.check_network_security();
    println!("  Безопасность: {}", if report3.is_secure { "✅" } else { "⚠️" });
    println!("  Рисков: {}", report3.security_risks.len());
    
    // Проверяем, что все системы остаются стабильными
    assert!(bc.is_chain_valid());
    assert!(bc_even.is_chain_valid());
    assert!(bc_duopoly.is_chain_valid());
}
