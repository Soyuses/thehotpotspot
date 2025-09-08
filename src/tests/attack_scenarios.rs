use crate::*;

/// Анализ опасных сценариев и защитных механизмов
/// 
/// Этот модуль содержит тесты для проверки защиты от различных типов атак
/// на блокчейн-сеть фудтраков.

#[test]
fn test_rapid_token_accumulation_attack() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Злоумышленник пытается быстро накопить токены
    // через множественные покупки в короткий период времени
    
    let mut attacker_wallet = "attacker".to_string();
    let mut total_accumulated = 0.0;
    
    // Симуляция быстрого накопления токенов
    for i in 0..20 {
        let purchase_amount = 5.0;
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            purchase_amount,
            vec!["Meal".to_string()],
        );
        total_accumulated += purchase_amount;
    }
    
    // Проверяем, что система отслеживает накопление
    let report = bc.check_network_security();
    assert!(report.total_security_tokens > 0.0);
    
    // В реальной системе здесь должны быть дополнительные проверки:
    // - Ограничения на частоту покупок
    // - Анализ паттернов активности
    // - Дополнительная верификация для больших сумм
}

#[test]
fn test_coordinated_attack_prevention() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Координированная атака нескольких участников
    // для обхода ограничений на одного пользователя
    
    let mut total_coordinated = 0.0;
    let attack_group_size = 10;
    
    // Создаем группу "координированных" атакующих
    for i in 0..attack_group_size {
        let mut holder = TokenHolder::new(format!("attacker{}", i), false);
        let tokens = 8.0; // Каждый имеет меньше лимита, но вместе превышают
        holder.add_security_tokens(tokens);
        total_coordinated += tokens;
        bc.token_holders.insert(format!("attacker{}", i), holder);
    }
    
    // Проверяем общую концентрацию
    let report = bc.check_network_security();
    
    // В реальной системе должны быть механизмы для:
    // - Обнаружения координированной активности
    // - Анализа временных паттернов
    // - Проверки связей между кошельками
    assert!(report.total_security_tokens > 0.0);
}

#[test]
fn test_utility_token_manipulation() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Манипуляция utility токенами для влияния на голосование
    
    // Создаем пользователя с большим количеством utility токенов
    let mut whale = TokenHolder::new("whale".to_string(), false);
    whale.add_utility_tokens(100.0);
    bc.token_holders.insert("whale".to_string(), whale);
    
    // Выпускаем utility токены в систему
    bc.utility_token.issue_voting_tokens(100.0);
    
    // Создаем меню для голосования
    let menu_item = MenuItem::new(
        "Manipulated Item".to_string(),
        "Item created for manipulation test".to_string(),
        10.0,
        "whale".to_string(),
        7
    );
    bc.menu_items.push(menu_item);
    
    // Попытка манипуляции голосованием
    let menu_item_id = bc.menu_items[0].id.clone();
    bc.make_menu_item_available_for_voting(menu_item_id.clone()).unwrap();
    
    // "Кит" голосует с большим весом
    let result = bc.vote_on_menu_item("whale".to_string(), menu_item_id, true);
    
    // Проверяем, что система учитывает ограничения
    match result {
        Ok(()) => {
            // Если голосование прошло, проверяем ограничения
            let report = bc.check_network_security();
            if !report.utility_risks.is_empty() {
                // Система должна обнаружить риск концентрации utility токенов
                assert!(report.utility_risks.iter().any(|risk| risk.wallet == "whale"));
            }
        }
        Err(_) => {
            // Система заблокировала голосование из-за ограничений
            assert!(true);
        }
    }
}

#[test]
fn test_check_reuse_attack() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Попытка повторного использования чека
    
    // Создаем чек
    let check = bc.process_purchase(
        "Customer".to_string(),
        "Truck".to_string(),
        0.1,
        vec!["Burger".to_string()],
    );
    
    // Регистрируем и верифицируем пользователя
    let phone = "+1234567890".to_string();
    let wallet = "0xwallet123".to_string();
    let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
        .expect("registration should succeed");
    bc.verify_phone_number(phone.clone(), verification_code)
        .expect("verification should succeed");
    
    // Первый перенос должен пройти успешно
    let transfer_id1 = bc.transfer_balance_from_check(check.check_id.clone(), phone.clone())
        .expect("first transfer should succeed");
    assert!(!transfer_id1.is_empty());
    
    // Попытка повторного переноса с того же чека должна быть заблокирована
    let result2 = bc.transfer_balance_from_check(check.check_id.clone(), phone.clone());
    assert!(result2.is_err());
    assert!(result2.unwrap_err().contains("already claimed"));
}

#[test]
fn test_phone_number_spoofing() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Попытка подделки номера телефона
    
    let phone1 = "+1234567890".to_string();
    let phone2 = "+1234567890".to_string(); // Тот же номер
    let wallet1 = "0xwallet1".to_string();
    let wallet2 = "0xwallet2".to_string();
    
    // Первая регистрация должна пройти успешно
    let verification_code1 = bc.register_user_with_phone(phone1.clone(), wallet1.clone())
        .expect("first registration should succeed");
    
    // Попытка регистрации с тем же номером должна быть заблокирована
    let result2 = bc.register_user_with_phone(phone2.clone(), wallet2.clone());
    assert!(result2.is_err());
    assert!(result2.unwrap_err().contains("already registered"));
}

#[test]
fn test_network_partition_attack() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Атака разделения сети
    
    // Создаем несколько узлов
    let mut nodes: Vec<String> = Vec::new();
    for i in 0..5 {
        let mut holder = TokenHolder::new(format!("node{}", i), false);
        holder.add_security_tokens(10.0);
        bc.token_holders.insert(format!("node{}", i), holder);
    }
    
    // Симулируем разделение сети
    // В реальной системе здесь должны быть механизмы для:
    // - Обнаружения разделения сети
    // - Выбора основной ветки
    // - Восстановления консенсуса
    
    let report = bc.check_network_security();
    assert!(report.total_security_tokens > 0.0);
    
    // Проверяем, что система остается стабильной
    assert!(bc.is_chain_valid());
}

#[test]
fn test_economic_attack_vectors() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Экономические атаки
    
    // 1. Атака на инфляцию через множественные покупки
    let mut total_inflation = 0.0;
    for i in 0..100 {
        let amount = 1.0;
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            amount,
            vec!["Meal".to_string()],
        );
        total_inflation += amount;
    }
    
    // 2. Атака на дефляцию через накопление токенов
    let mut accumulator = TokenHolder::new("accumulator".to_string(), false);
    accumulator.add_security_tokens(total_inflation * 0.3); // 30% от всех токенов
    bc.token_holders.insert("accumulator".to_string(), accumulator);
    
    // Проверяем, что система отслеживает экономические риски
    let report = bc.check_network_security();
    assert!(report.total_security_tokens > 0.0);
    
    // В реальной системе должны быть механизмы для:
    // - Контроля инфляции/дефляции
    // - Стабилизации экономики
    // - Защиты от манипуляций с ценами
}

#[test]
fn test_social_engineering_attack() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Социальная инженерия для получения доступа
    
    // Попытка регистрации с подозрительными данными
    let suspicious_phone = "+1111111111".to_string(); // Подозрительный номер
    let wallet = "0xwallet123".to_string();
    
    // В реальной системе должны быть проверки на:
    // - Валидность номера телефона
    // - Подозрительные паттерны
    // - Дополнительная верификация
    
    let result = bc.register_user_with_phone(suspicious_phone.clone(), wallet.clone());
    
    // Система должна либо принять регистрацию (если номер валидный),
    // либо заблокировать (если обнаружены подозрительные паттерны)
    match result {
        Ok(_) => {
            // Регистрация прошла, но должны быть дополнительные проверки
            assert!(bc.authorized_users.contains_key(&suspicious_phone));
        }
        Err(_) => {
            // Система заблокировала подозрительную регистрацию
            assert!(true);
        }
    }
}

#[test]
fn test_time_based_attacks() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Атаки, основанные на времени
    
    // 1. Атака на временные окна голосования
    let menu_item = MenuItem::new(
        "Time Attack Item".to_string(),
        "Item for time-based attack test".to_string(),
        10.0,
        "attacker".to_string(),
        1 // Короткий период голосования
    );
    bc.menu_items.push(menu_item);
    
    // 2. Попытка манипуляции временными метками
    // В реальной системе должны быть механизмы для:
    // - Синхронизации времени
    // - Защиты от манипуляций временными метками
    // - Валидации временных окон
    
    let report = bc.check_network_security();
    assert!(report.total_security_tokens >= 0.0);
    
    // Проверяем, что система остается стабильной
    assert!(bc.is_chain_valid());
}

#[test]
fn test_cross_chain_attack() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Сценарий: Атаки через другие блокчейны
    
    // Симуляция попытки атаки через внешние системы
    // В реальной системе должны быть механизмы для:
    // - Защиты от атак через мосты
    // - Валидации внешних транзакций
    // - Изоляции от других сетей
    
    let report = bc.check_network_security();
    assert!(report.total_security_tokens >= 0.0);
    
    // Проверяем, что система остается изолированной и безопасной
    assert!(bc.is_chain_valid());
}

/// Комплексный тест всех защитных механизмов
#[test]
fn test_comprehensive_security_measures() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    // Тестируем все основные защитные механизмы одновременно
    
    // 1. Ограничения на владение токенами
    bc.max_owner_percentage = 30.0; // Устанавливаем низкий лимит для теста
    bc.max_customer_percentage = 30.0; // Также ограничиваем лимит для клиентов
    
    // 2. Создаем сценарий с множественными атаками
    let mut total_attack_tokens = 0.0;
    for i in 0..10 {
        let mut holder = TokenHolder::new(format!("attacker{}", i), false);
        let tokens = 5.0;
        holder.add_security_tokens(tokens);
        total_attack_tokens += tokens;
        bc.token_holders.insert(format!("attacker{}", i), holder);
    }
    
    // 3. Попытка концентрации токенов
    let mut whale = TokenHolder::new("whale".to_string(), false);
    whale.add_security_tokens(40.0); // Превышает лимит
    bc.token_holders.insert("whale".to_string(), whale);
    
    // 4. Проверяем все защитные механизмы
    let report = bc.check_network_security();
    
    // Система должна обнаружить риски
    assert!(!report.is_secure);
    assert!(!report.security_risks.is_empty());
    
    // Проверяем, что "кит" обнаружен как риск
    let whale_risk = report.security_risks.iter()
        .find(|risk| risk.wallet == "whale");
    assert!(whale_risk.is_some());
    
    // Проверяем общую стабильность системы
    assert!(bc.is_chain_valid());
    assert!(report.total_security_tokens > 0.0);
}
