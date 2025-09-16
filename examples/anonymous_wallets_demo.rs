//! Anonymous Wallets Demo for The Hot Pot Spot
//! 
//! This example demonstrates the anonymous wallet system for unclaimed tokens.

use blockchain_project::anonymous_wallets::{
    AnonymousWalletManager, AnonymousWalletConfig, WalletStatus, WalletTransferRequest
};

fn main() {
    println!("🔐 Демонстрация системы обезличенных кошельков");
    println!("{}", "=".repeat(50));

    // Initialize anonymous wallet manager
    let config = AnonymousWalletConfig::default();
    let mut manager = AnonymousWalletManager::new(config);

    // Demo 1: Create anonymous wallets for different sales
    println!("\n📋 Демо 1: Создание обезличенных кошельков");
    println!("{}", "-".repeat(30));

    let sales = vec![
        ("sale_001", "check_001", "node_tbilisi_001", 25.0, 500),
        ("sale_002", "check_002", "node_batumi_001", 15.0, 300),
        ("sale_003", "check_003", "node_kutaisi_001", 50.0, 1000),
        ("sale_004", "check_004", "node_tbilisi_001", 10.0, 200),
        ("sale_005", "check_005", "node_batumi_001", 30.0, 600),
    ];

    let mut created_wallets = Vec::new();

    for (sale_id, check_id, node_id, amount, tokens) in sales {
        match manager.create_wallet(
            check_id.to_string(),
            sale_id.to_string(),
            node_id.to_string(),
            amount,
            tokens
        ) {
            Ok(wallet) => {
                println!("✅ Создан кошелек {} для чека {}", wallet.wallet_id, check_id);
                println!("   Адрес: {}", wallet.address);
                println!("   Сумма: {} GEL → {} ST", amount, tokens);
                created_wallets.push(wallet);
            }
            Err(e) => {
                println!("❌ Ошибка создания кошелька для {}: {}", check_id, e);
            }
        }
    }

    // Demo 2: Activate wallets (mint tokens)
    println!("\n🪙 Демо 2: Активация кошельков (эмиссия токенов)");
    println!("{}", "-".repeat(30));

    for wallet in &created_wallets {
        match manager.activate_wallet(&wallet.wallet_id) {
            Ok(_) => {
                println!("✅ Кошелек {} активирован", wallet.wallet_id);
            }
            Err(e) => {
                println!("❌ Ошибка активации кошелька {}: {}", wallet.wallet_id, e);
            }
        }
    }

    // Demo 3: Customer journey simulation
    println!("\n👤 Демо 3: Симуляция пути покупателя");
    println!("{}", "-".repeat(30));

    let customer_scenarios = vec![
        ("check_001", "user_001", "0xuser_wallet_1234567890abcdef", "Активирует чек"),
        ("check_002", "user_002", "0xuser_wallet_abcdef1234567890", "Активирует чек"),
        ("check_003", "user_003", "0xuser_wallet_9876543210fedcba", "Активирует чек"),
        ("check_004", "", "", "Выбрасывает чек"),
        ("check_005", "user_005", "0xuser_wallet_fedcba0987654321", "Активирует чек"),
    ];

    for (check_id, user_id, user_wallet, action) in customer_scenarios {
        println!("\n🛒 Покупатель с чеком {}: {}", check_id, action);
        
        if let Some(wallet) = manager.get_wallet_by_check(check_id) {
            let wallet_id = wallet.wallet_id.clone();
            if action == "Выбрасывает чек" {
                // Customer discards the check
                match manager.discard_wallet(&wallet_id) {
                    Ok(_) => {
                        println!("   🗑️  Чек выброшен, кошелек помечен как discarded");
                    }
                    Err(e) => {
                        println!("   ❌ Ошибка при выбрасывании чека: {}", e);
                    }
                }
            } else {
                // Customer claims the check
                match manager.transfer_to_user(
                    &wallet_id,
                    user_id.to_string(),
                    user_wallet.to_string()
                ) {
                    Ok(transferred_tokens) => {
                        println!("   ✅ Токены переведены на личный кошелек");
                        println!("   💰 Получено: {} ST токенов", transferred_tokens);
                        println!("   🔗 Кошелек: {}", user_wallet);
                    }
                    Err(e) => {
                        println!("   ❌ Ошибка перевода токенов: {}", e);
                    }
                }
            }
        } else {
            println!("   ❌ Кошелек для чека {} не найден", check_id);
        }
    }

    // Demo 4: Wallet statistics
    println!("\n📊 Демо 4: Статистика кошельков");
    println!("{}", "-".repeat(30));

    let stats = manager.get_statistics();
    println!("📈 Общая статистика обезличенных кошельков:");
    println!("   Всего кошельков: {}", stats.total_wallets);
    println!("   Создано: {}", stats.created);
    println!("   Активных: {}", stats.active);
    println!("   Переведено: {}", stats.transferred);
    println!("   Истекших: {}", stats.expired);
    println!("   Выброшенных: {}", stats.discarded);
    println!("   Токенов переведено: {} ST", stats.total_tokens_transferred);
    println!("   Токенов не востребовано: {} ST", stats.total_tokens_unclaimed);

    // Demo 5: Redistribution analysis
    println!("\n🔄 Демо 5: Анализ перераспределения");
    println!("{}", "-".repeat(30));

    let redistribution_wallets = manager.get_wallets_for_redistribution();
    let redistribution_tokens = manager.get_redistribution_tokens();

    println!("📋 Кошельки для перераспределения ({} шт.):", redistribution_wallets.len());
    for wallet in &redistribution_wallets {
        println!("   {}: {} ST (статус: {:?})", 
            wallet.wallet_id, wallet.st_tokens, wallet.status);
    }

    println!("💰 Общая сумма токенов для перераспределения: {} ST", redistribution_tokens);

    if redistribution_tokens > 0 {
        println!("\n🎯 Сценарии перераспределения:");
        println!("   1. Распределение между активными участниками DAO");
        println!("   2. Добавление в резервный фонд");
        println!("   3. Использование для развития экосистемы");
        println!("   4. Пожертвования в благотворительные фонды");
    }

    // Demo 6: Wallet lifecycle
    println!("\n🔄 Демо 6: Жизненный цикл кошелька");
    println!("{}", "-".repeat(30));

    println!("📝 Этапы жизненного цикла обезличенного кошелька:");
    println!("   1. 🆕 Создание - кошелек создается при генерации чека");
    println!("   2. 🪙 Активация - токены эмитируются в кошелек");
    println!("   3. ⏳ Ожидание - кошелек ждет активации покупателем");
    println!("   4. ✅ Передача - токены переводятся на личный кошелек");
    println!("   5. 🗑️  Выбрасывание - покупатель выбрасывает чек");
    println!("   6. ⏰ Истечение - кошелек истекает по времени");
    println!("   7. 🔄 Перераспределение - невостребованные токены перераспределяются");

    // Demo 7: Security and privacy features
    println!("\n🔒 Демо 7: Безопасность и приватность");
    println!("{}", "-".repeat(30));

    println!("🛡️  Особенности безопасности:");
    println!("   • Детерминистические адреса на основе check_id и wallet_id");
    println!("   • Автоматическое истечение неактивных кошельков");
    println!("   • Отсутствие связи с личными данными до активации");
    println!("   • Полная трассируемость в блокчейне");
    println!("   • Защита от двойного использования");

    println!("\n🔐 Особенности приватности:");
    println!("   • Анонимность до момента активации");
    println!("   • Невозможность связать чек с покупателем без QR-кода");
    println!("   • Автоматическая очистка старых данных");
    println!("   • Соответствие требованиям GDPR");

    // Demo 8: Integration with other systems
    println!("\n🔗 Демо 8: Интеграция с другими системами");
    println!("{}", "-".repeat(30));

    println!("🤝 Интеграция с:");
    println!("   • Check Generation API - создание кошельков при генерации чеков");
    println!("   • Tokenomics Manager - эмиссия и управление токенами");
    println!("   • DAO Governance - участие в перераспределении");
    println!("   • KYC/AML System - проверка при активации");
    println!("   • Mobile App - интерфейс для пользователей");

    // Final statistics
    println!("\n📊 Финальная статистика:");
    println!("{}", "=".repeat(50));
    let final_stats = manager.get_statistics();
    println!("Всего кошельков: {}", final_stats.total_wallets);
    println!("Успешно активировано: {} ({}%)", 
        final_stats.transferred,
        if final_stats.total_wallets > 0 {
            (final_stats.transferred * 100) / final_stats.total_wallets
        } else { 0 }
    );
    println!("Выброшено/истекло: {} ({}%)", 
        final_stats.discarded + final_stats.expired,
        if final_stats.total_wallets > 0 {
            ((final_stats.discarded + final_stats.expired) * 100) / final_stats.total_wallets
        } else { 0 }
    );
    println!("Токенов в обращении: {} ST", final_stats.total_tokens_transferred);
    println!("Токенов для перераспределения: {} ST", final_stats.total_tokens_unclaimed);

    println!("\n🎉 Демонстрация завершена!");
    println!("💡 Ключевые преимущества обезличенных кошельков:");
    println!("   • Максимальная приватность покупателей");
    println!("   • Автоматическое управление невостребованными токенами");
    println!("   • Прозрачность и трассируемость");
    println!("   • Интеграция с экосистемой The Hot Pot Spot");
    println!("   • Соответствие регуляторным требованиям");
}
