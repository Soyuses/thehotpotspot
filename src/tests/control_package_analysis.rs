use crate::*;
use std::collections::HashMap;

/// Анализ проблемы контрольного пакета и механизмов перераспределения
/// 
/// Этот модуль исследует:
/// 1. Как владелец накапливает контрольный пакет
/// 2. Возможности перераспределения токенов
/// 3. Механизмы децентрализации власти
/// 4. Нагрузочное тестирование различных сценариев

#[test]
fn test_owner_control_package_accumulation() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🔍 === АНАЛИЗ НАКОПЛЕНИЯ КОНТРОЛЬНОГО ПАКЕТА ВЛАДЕЛЬЦЕМ ===");
    
    // Симулируем множество покупок
    let mut total_purchases = 0.0;
    let mut owner_initial_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    
    println!("Начальные токены владельца: {:.2}", owner_initial_tokens);
    
    // Создаем 100 покупок
    for i in 0..100 {
        let purchase_amount = 10.0 + (i as f64 * 0.1); // От 10 до 20
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            purchase_amount,
            vec!["Meal".to_string()],
        );
        total_purchases += purchase_amount;
    }
    
    let owner_final_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_percentage = (owner_final_tokens / total_tokens) * 100.0;
    
    println!("Общая сумма покупок: {:.2}", total_purchases);
    println!("Финальные токены владельца: {:.2}", owner_final_tokens);
    println!("Общее количество токенов в сети: {:.2}", total_tokens);
    println!("Процент владения владельцем: {:.2}%", owner_percentage);
    
    // Проверяем, превышает ли владелец лимит
    let report = bc.check_network_security();
    let owner_risk = report.security_risks.iter()
        .find(|risk| risk.wallet == owner);
    
    if let Some(risk) = owner_risk {
        println!("⚠️  ВЛАДЕЛЕЦ ПРЕВЫШАЕТ ЛИМИТ: {:.2}%", risk.percentage);
        assert!(risk.percentage > bc.max_owner_percentage);
    } else {
        println!("✅ Владелец в пределах лимита: {:.2}%", owner_percentage);
    }
}

#[test]
fn test_token_redistribution_mechanisms() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🔄 === МЕХАНИЗМЫ ПЕРЕРАСПРЕДЕЛЕНИЯ ТОКЕНОВ ===");
    
    // Создаем начальную концентрацию у владельца
    for i in 0..50 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            20.0,
            vec!["Meal".to_string()],
        );
    }
    
    let initial_owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let initial_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let initial_percentage = (initial_owner_tokens / initial_total) * 100.0;
    
    println!("Начальный процент владельца: {:.2}%", initial_percentage);
    
    // Механизм 1: Создание новых держателей токенов через перенос баланса
    let mut redistributed_tokens = 0.0;
    
    for i in 0..20 {
        // Создаем чек для переноса
        let check = bc.process_purchase(
            format!("RedistCustomer{}", i),
            "Truck".to_string(),
            15.0,
            vec!["Meal".to_string()],
        );
        
        // Регистрируем нового пользователя
        let phone = format!("+123456789{}", i);
        let wallet = format!("0xredist{}", i);
        let verification_code = bc.register_user_with_phone(phone.clone(), wallet.clone())
            .expect("registration should succeed");
        bc.verify_phone_number(phone.clone(), verification_code)
            .expect("verification should succeed");
        
        // Переносим баланс (если не превышает лимиты)
        match bc.transfer_balance_from_check(check.check_id, phone) {
            Ok(_) => {
                redistributed_tokens += 15.0;
                println!("✅ Перенос {} токенов успешен", 15.0);
            }
            Err(e) => {
                println!("❌ Перенос заблокирован: {}", e);
            }
        }
    }
    
    let final_owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let final_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let final_percentage = (final_owner_tokens / final_total) * 100.0;
    
    println!("Перераспределено токенов: {:.2}", redistributed_tokens);
    println!("Финальный процент владельца: {:.2}%", final_percentage);
    println!("Снижение концентрации: {:.2}%", initial_percentage - final_percentage);
}

#[test]
fn test_coordinated_group_attack_on_control() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n👥 === КООРДИНИРОВАННАЯ АТАКА ГРУППЫ НА КОНТРОЛЬ ===");
    
    // Создаем начальную ситуацию с концентрацией у владельца
    for i in 0..30 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            25.0,
            vec!["Meal".to_string()],
        );
    }
    
    let initial_owner_percentage = {
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        (owner_tokens / total) * 100.0
    };
    
    println!("Начальный процент владельца: {:.2}%", initial_owner_percentage);
    
    // Создаем координированную группу атакующих
    let group_size = 10;
    let mut group_total_tokens = 0.0;
    
    for i in 0..group_size {
        // Каждый участник группы делает покупки
        for j in 0..5 {
            let purchase_amount = 20.0;
            bc.process_purchase(
                format!("Group{}_Customer{}", i, j),
                "Truck".to_string(),
                purchase_amount,
                vec!["Meal".to_string()],
            );
            group_total_tokens += purchase_amount;
        }
        
        // Создаем отдельного держателя токенов для группы
        let mut group_member = TokenHolder::new(format!("group_member{}", i), false);
        group_member.add_security_tokens(100.0); // Каждый участник группы получает токены
        bc.token_holders.insert(format!("group_member{}", i), group_member);
    }
    
    // Анализируем распределение власти
    let final_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let owner_percentage = (owner_tokens / final_total) * 100.0;
    
    // Считаем общую долю группы
    let group_tokens: f64 = (0..group_size)
        .map(|i| bc.token_holders.get(&format!("group_member{}", i)).unwrap().security_tokens)
        .sum();
    let group_percentage = (group_tokens / final_total) * 100.0;
    
    println!("Токены группы: {:.2}", group_tokens);
    println!("Процент группы: {:.2}%", group_percentage);
    println!("Финальный процент владельца: {:.2}%", owner_percentage);
    
    // Проверяем, может ли группа получить контроль
    if group_percentage > owner_percentage {
        println!("🚨 ГРУППА ПОЛУЧИЛА БОЛЬШЕ ТОКЕНОВ ЧЕМ ВЛАДЕЛЕЦ!");
    } else {
        println!("✅ Владелец сохраняет доминирующее положение");
    }
    
    // Проверяем ограничения безопасности
    let report = bc.check_network_security();
    println!("Риски безопасности: {}", report.security_risks.len());
    for risk in &report.security_risks {
        println!("  - {}: {:.2}% ({})", risk.wallet, risk.percentage, risk.token_type);
    }
}

#[test]
fn test_load_testing_token_concentration() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n⚡ === НАГРУЗОЧНОЕ ТЕСТИРОВАНИЕ КОНЦЕНТРАЦИИ ТОКЕНОВ ===");
    
    let scenarios = vec![
        ("Малые покупки", 1000, 5.0),
        ("Средние покупки", 500, 20.0),
        ("Крупные покупки", 100, 100.0),
        ("Смешанные покупки", 200, 0.0), // 0 означает случайные суммы
    ];
    
    for (scenario_name, num_purchases, amount) in scenarios {
        println!("\n--- Сценарий: {} ---", scenario_name);
        
        let mut bc_scenario = bc.clone();
        let mut total_purchases = 0.0;
        
        for i in 0..num_purchases {
            let purchase_amount = if amount == 0.0 {
                // Случайные суммы от 1 до 50
                fastrand::f64() * 49.0 + 1.0
            } else {
                amount
            };
            
            bc_scenario.process_purchase(
                format!("LoadTest_Customer_{}_{}", scenario_name, i),
                "Truck".to_string(),
                purchase_amount,
                vec!["Meal".to_string()],
            );
            total_purchases += purchase_amount;
        }
        
        let owner_tokens = bc_scenario.token_holders.get(&owner).unwrap().security_tokens;
        let total_tokens: f64 = bc_scenario.token_holders.values().map(|h| h.security_tokens).sum();
        let owner_percentage = (owner_tokens / total_tokens) * 100.0;
        
        let report = bc_scenario.check_network_security();
        let is_secure = report.is_secure;
        let risks_count = report.security_risks.len();
        
        println!("  Покупок: {}", num_purchases);
        println!("  Общая сумма: {:.2}", total_purchases);
        println!("  Токены владельца: {:.2}", owner_tokens);
        println!("  Процент владельца: {:.2}%", owner_percentage);
        println!("  Безопасность: {}", if is_secure { "✅ Безопасно" } else { "⚠️ Риски" });
        println!("  Количество рисков: {}", risks_count);
        
        // Анализируем распределение токенов
        let mut distribution = HashMap::new();
        for holder in bc_scenario.token_holders.values() {
            let percentage = (holder.security_tokens / total_tokens) * 100.0;
            let range = match percentage {
                p if p >= 10.0 => "10%+",
                p if p >= 5.0 => "5-10%",
                p if p >= 1.0 => "1-5%",
                _ => "<1%",
            };
            *distribution.entry(range).or_insert(0) += 1;
        }
        
        println!("  Распределение держателей:");
        for (range, count) in distribution {
            println!("    {}: {} держателей", range, count);
        }
    }
}

#[test]
fn test_decentralization_mechanisms() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🌐 === МЕХАНИЗМЫ ДЕЦЕНТРАЛИЗАЦИИ ===");
    
    // Создаем начальную концентрацию
    for i in 0..40 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            30.0,
            vec!["Meal".to_string()],
        );
    }
    
    let initial_owner_percentage = {
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        (owner_tokens / total) * 100.0
    };
    
    println!("Начальная концентрация владельца: {:.2}%", initial_owner_percentage);
    
    // Механизм 1: Автоматическое перераспределение части токенов
    let redistribution_percentage = 0.1; // 10% от токенов владельца
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let tokens_to_redistribute = owner_tokens * redistribution_percentage;
    
    // Создаем новых держателей токенов
    let num_new_holders = 20;
    let tokens_per_holder = tokens_to_redistribute / num_new_holders as f64;
    
    for i in 0..num_new_holders {
        let mut new_holder = TokenHolder::new(format!("decentralized_holder{}", i), false);
        new_holder.add_security_tokens(tokens_per_holder);
        bc.token_holders.insert(format!("decentralized_holder{}", i), new_holder);
    }
    
    // Уменьшаем токены владельца
    if let Some(owner_holder) = bc.token_holders.get_mut(&owner) {
        owner_holder.security_tokens -= tokens_to_redistribute;
    }
    
    let final_owner_percentage = {
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        (owner_tokens / total) * 100.0
    };
    
    println!("Перераспределено токенов: {:.2}", tokens_to_redistribute);
    println!("Создано новых держателей: {}", num_new_holders);
    println!("Финальная концентрация владельца: {:.2}%", final_owner_percentage);
    println!("Снижение концентрации: {:.2}%", initial_owner_percentage - final_owner_percentage);
    
    // Механизм 2: Стимулирование децентрализации через бонусы
    let decentralization_bonus = 0.05; // 5% бонус за децентрализацию
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let bonus_tokens = total_tokens * decentralization_bonus;
    
    // Распределяем бонус между всеми держателями (кроме владельца)
    let non_owner_addresses: Vec<String> = bc.token_holders.iter()
        .filter(|(addr, _)| **addr != owner)
        .map(|(addr, _)| addr.clone())
        .collect();
    
    let bonus_per_holder = bonus_tokens / non_owner_addresses.len() as f64;
    
    for addr in non_owner_addresses {
        if let Some(holder_mut) = bc.token_holders.get_mut(&addr) {
            holder_mut.add_security_tokens(bonus_per_holder);
        }
    }
    
    let final_owner_percentage_with_bonus = {
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        (owner_tokens / total) * 100.0
    };
    
    println!("Бонус за децентрализацию: {:.2}", bonus_tokens);
    println!("Финальная концентрация с бонусом: {:.2}%", final_owner_percentage_with_bonus);
    
    // Проверяем безопасность
    let report = bc.check_network_security();
    println!("Безопасность сети: {}", if report.is_secure { "✅ Безопасно" } else { "⚠️ Риски" });
    println!("Количество рисков: {}", report.security_risks.len());
}

#[test]
fn test_what_if_51_percent_always_goes_to_owner() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n🚨 === СЦЕНАРИЙ: 51% ВСЕГДА ПОПАДАЕТ ВЛАДЕЛЬЦУ ===");
    
    // Модифицируем логику process_purchase для симуляции
    // В реальной системе это было бы опасно!
    
    let mut total_purchases = 0.0;
    let mut owner_accumulation = 0.0;
    
    // Симулируем 100 покупок, где владелец получает 51% от каждой
    for i in 0..100 {
        let purchase_amount = 20.0;
        total_purchases += purchase_amount;
        
        // Владелец получает 51% от каждой покупки
        let owner_share = purchase_amount * 0.51;
        owner_accumulation += owner_share;
        
        // Остальные 49% распределяются между другими участниками
        let remaining = purchase_amount * 0.49;
        
        // Создаем других держателей токенов
        let mut other_holder = TokenHolder::new(format!("other_holder{}", i), false);
        other_holder.add_security_tokens(remaining);
        bc.token_holders.insert(format!("other_holder{}", i), other_holder);
    }
    
    // Обновляем токены владельца
    if let Some(owner_holder) = bc.token_holders.get_mut(&owner) {
        owner_holder.security_tokens += owner_accumulation;
    }
    
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let total_tokens: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_percentage = (owner_tokens / total_tokens) * 100.0;
    
    println!("Общая сумма покупок: {:.2}", total_purchases);
    println!("Накоплено владельцем: {:.2}", owner_accumulation);
    println!("Общее количество токенов: {:.2}", total_tokens);
    println!("Процент владения владельцем: {:.2}%", owner_percentage);
    
    // Анализируем последствия
    if owner_percentage > 50.0 {
        println!("🚨 КРИТИЧЕСКАЯ СИТУАЦИЯ: Владелец контролирует {:.2}% токенов!", owner_percentage);
        println!("   Это означает полный контроль над сетью!");
        println!("   Возможности владельца:");
        println!("   - Отмена любых транзакций");
        println!("   - Создание форков сети");
        println!("   - Манипулирование консенсусом");
        println!("   - Блокировка других участников");
    }
    
    // Проверяем, можно ли это исправить
    println!("\n🔧 ВОЗМОЖНЫЕ РЕШЕНИЯ:");
    
    // Решение 1: Принудительное перераспределение
    let redistribution_needed = owner_tokens - (total_tokens * 0.49); // До 49%
    println!("1. Принудительное перераспределение: нужно перераспределить {:.2} токенов", redistribution_needed);
    
    // Решение 2: Создание новых токенов для других участников
    let new_tokens_needed = (owner_tokens / 0.49) - total_tokens;
    println!("2. Создание новых токенов: нужно создать {:.2} новых токенов", new_tokens_needed);
    
    // Решение 3: Сжигание части токенов владельца
    let burn_needed = owner_tokens - (total_tokens * 0.49);
    println!("3. Сжигание токенов владельца: нужно сжечь {:.2} токенов", burn_needed);
    
    // Решение 4: Изменение правил консенсуса
    println!("4. Изменение консенсуса: переход на Proof-of-Stake с ограничениями");
    
    // Проверяем безопасность
    let report = bc.check_network_security();
    println!("\n📊 ОТЧЕТ БЕЗОПАСНОСТИ:");
    println!("Безопасность: {}", if report.is_secure { "✅ Безопасно" } else { "🚨 ОПАСНО" });
    println!("Риски: {}", report.security_risks.len());
    
    for risk in &report.security_risks {
        println!("  - {}: {:.2}% ({})", risk.wallet, risk.percentage, risk.token_type);
    }
}

#[test]
fn test_group_coordination_to_take_control() {
    let owner = "Alice".to_string();
    let mut bc = Blockchain::new(owner.clone());
    
    println!("\n👥 === ПОПЫТКА ГРУППЫ ЗАХВАТИТЬ КОНТРОЛЬ ===");
    
    // Создаем начальную ситуацию
    for i in 0..20 {
        bc.process_purchase(
            format!("Customer{}", i),
            "Truck".to_string(),
            25.0,
            vec!["Meal".to_string()],
        );
    }
    
    let initial_owner_percentage = {
        let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
        let total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
        (owner_tokens / total) * 100.0
    };
    
    println!("Начальный процент владельца: {:.2}%", initial_owner_percentage);
    
    // Создаем координированную группу
    let group_size = 5;
    let mut group_total_tokens = 0.0;
    
    // Стратегия 1: Массовые покупки
    println!("\n--- Стратегия 1: Массовые покупки ---");
    for i in 0..group_size {
        for j in 0..20 { // Каждый участник делает 20 покупок
            let purchase_amount = 30.0;
            bc.process_purchase(
                format!("Group{}_Purchase{}", i, j),
                "Truck".to_string(),
                purchase_amount,
                vec!["Meal".to_string()],
            );
            group_total_tokens += purchase_amount;
        }
    }
    
    // Стратегия 2: Создание множественных кошельков
    println!("--- Стратегия 2: Множественные кошельки ---");
    for i in 0..group_size {
        for j in 0..10 { // Каждый участник создает 10 кошельков
            let mut wallet = TokenHolder::new(format!("group{}_wallet{}", i, j), false);
            wallet.add_security_tokens(50.0); // Каждый кошелек получает токены
            bc.token_holders.insert(format!("group{}_wallet{}", i, j), wallet);
        }
    }
    
    // Стратегия 3: Координация через перенос баланса
    println!("--- Стратегия 3: Координация через переносы ---");
    for i in 0..group_size {
        // Создаем чеки для переноса
        for j in 0..5 {
            let check = bc.process_purchase(
                format!("Group{}_Transfer{}", i, j),
                "Truck".to_string(),
                20.0,
                vec!["Meal".to_string()],
            );
            
            // Регистрируем пользователя группы
            let phone = format!("+123456789{}{}", i, j);
            let wallet = format!("0xgroup{}_wallet{}", i, j);
            
            if let Ok(verification_code) = bc.register_user_with_phone(phone.clone(), wallet.clone()) {
                if let Ok(_) = bc.verify_phone_number(phone.clone(), verification_code) {
                    // Пытаемся перенести баланс
                    match bc.transfer_balance_from_check(check.check_id, phone) {
                        Ok(_) => println!("  ✅ Перенос успешен для группы {}", i),
                        Err(e) => println!("  ❌ Перенос заблокирован: {}", e),
                    }
                }
            }
        }
    }
    
    // Анализируем результат
    let final_total: f64 = bc.token_holders.values().map(|h| h.security_tokens).sum();
    let owner_tokens = bc.token_holders.get(&owner).unwrap().security_tokens;
    let owner_percentage = (owner_tokens / final_total) * 100.0;
    
    // Считаем общую долю группы
    let group_tokens: f64 = bc.token_holders.iter()
        .filter(|(addr, _)| addr.starts_with("group"))
        .map(|(_, holder)| holder.security_tokens)
        .sum();
    let group_percentage = (group_tokens / final_total) * 100.0;
    
    println!("\n📊 РЕЗУЛЬТАТЫ КООРДИНИРОВАННОЙ АТАКИ:");
    println!("Токены группы: {:.2}", group_tokens);
    println!("Процент группы: {:.2}%", group_percentage);
    println!("Токены владельца: {:.2}", owner_tokens);
    println!("Процент владельца: {:.2}%", owner_percentage);
    
    if group_percentage > owner_percentage {
        println!("🚨 ГРУППА УСПЕШНО ЗАХВАТИЛА КОНТРОЛЬ!");
        println!("   Группа контролирует {:.2}% токенов", group_percentage);
        println!("   Владелец контролирует только {:.2}% токенов", owner_percentage);
    } else if group_percentage > 30.0 {
        println!("⚠️  ГРУППА СТАЛА ЗНАЧИТЕЛЬНОЙ СИЛОЙ");
        println!("   Группа контролирует {:.2}% токенов", group_percentage);
        println!("   Это может создать угрозу для стабильности сети");
    } else {
        println!("✅ Владелец сохраняет контроль");
        println!("   Группа контролирует только {:.2}% токенов", group_percentage);
    }
    
    // Проверяем ограничения безопасности
    let report = bc.check_network_security();
    println!("\n🛡️ ПРОВЕРКА БЕЗОПАСНОСТИ:");
    println!("Безопасность: {}", if report.is_secure { "✅ Безопасно" } else { "⚠️ Риски" });
    println!("Количество рисков: {}", report.security_risks.len());
    
    for risk in &report.security_risks {
        println!("  - {}: {:.2}% ({})", risk.wallet, risk.percentage, risk.token_type);
    }
    
    // Анализируем распределение власти
    let mut power_distribution = HashMap::new();
    for (addr, holder) in &bc.token_holders {
        let percentage = (holder.security_tokens / final_total) * 100.0;
        let category = if addr == &owner {
            "Владелец"
        } else if addr.starts_with("group") {
            "Группа"
        } else {
            "Другие"
        };
        *power_distribution.entry(category).or_insert(0.0) += percentage;
    }
    
    println!("\n📈 РАСПРЕДЕЛЕНИЕ ВЛАСТИ:");
    for (category, percentage) in power_distribution {
        println!("  {}: {:.2}%", category, percentage);
    }
}
