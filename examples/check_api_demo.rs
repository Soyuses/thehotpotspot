//! Check API Demo for The Hot Pot Spot
//! 
//! This example demonstrates the check generation and claiming workflow.

use blockchain_project::{
    check_generation::{
        CheckGenerationService, CheckGenerationRequest, CheckClaimRequest,
        CheckGenerationConfig, CheckStatus
    },
    check_api::demo_check_api
};

fn main() {
    println!("🔧 Демонстрация Check API для The Hot Pot Spot");
    println!("{}", "=".repeat(50));

    // Initialize check generation service
    let config = CheckGenerationConfig::default();
    let mut service = CheckGenerationService::new(config);

    // Demo 1: Generate a check for a sale
    println!("\n📋 Демо 1: Генерация чека для продажи");
    println!("{}", "-".repeat(30));

    let sale_request = CheckGenerationRequest {
        sale_id: "sale_2024_001".to_string(),
        node_id: "node_tbilisi_001".to_string(),
        amount_gel: 25.0,
        st_tokens: 500, // 25 GEL * 20 (1 GEL = 0.2 THP)
        customer_phone: None,
    };

    match service.generate_check(sale_request) {
        Ok(response) => {
            let check = &response.check;
            println!("✅ Чек успешно сгенерирован!");
            println!("   ID чека: {}", check.check_id);
            println!("   Сумма: {} GEL", check.amount_gel);
            println!("   Токены: {} ST", check.st_tokens);
            println!("   Кошелек: {}", check.wallet_address);
            println!("   Статус: {:?}", check.status);
            println!("   QR код: {} символов", check.qr_data.len());

            // Demo 2: Print the check (simulate POS printing)
            println!("\n🖨️  Демо 2: Печать чека");
            println!("{}", "-".repeat(30));

            match service.print_check(&check.check_id) {
                Ok(_) => {
                    println!("✅ Чек напечатан и выдан покупателю");
                    
                    // Demo 3: Customer claims the check
                    println!("\n📱 Демо 3: Активация чека покупателем");
                    println!("{}", "-".repeat(30));

                    let claim_request = CheckClaimRequest {
                        qr_data: check.qr_data.clone(),
                        user_id: "user_001".to_string(),
                        user_wallet: "0xuser_wallet_1234567890abcdef".to_string(),
                    };

                    match service.claim_check(claim_request) {
                        Ok(claim_response) => {
                            println!("✅ Чек успешно активирован!");
                            println!("   Переведено токенов: {} ST", claim_response.transferred_tokens);
                            println!("   На кошелек: 0xuser_wallet_1234567890abcdef");
                        }
                        Err(e) => {
                            println!("❌ Ошибка активации чека: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Ошибка печати чека: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Ошибка генерации чека: {}", e);
        }
    }

    // Demo 4: Generate multiple checks for different scenarios
    println!("\n🔄 Демо 4: Генерация нескольких чеков");
    println!("{}", "-".repeat(30));

    let scenarios = vec![
        ("sale_2024_002", "node_tbilisi_001", 15.0, 300), // 15 GEL = 3 THP
        ("sale_2024_003", "node_batumi_001", 50.0, 1000), // 50 GEL = 10 THP
        ("sale_2024_004", "node_kutaisi_001", 10.0, 200), // 10 GEL = 2 THP
    ];

    for (sale_id, node_id, amount, tokens) in scenarios {
        let request = CheckGenerationRequest {
            sale_id: sale_id.to_string(),
            node_id: node_id.to_string(),
            amount_gel: amount,
            st_tokens: tokens,
            customer_phone: None,
        };

        match service.generate_check(request) {
            Ok(response) => {
                println!("✅ Чек {}: {} GEL → {} ST", 
                    response.check.check_id, amount, tokens);
                
                // Simulate different customer behaviors
                match sale_id {
                    "sale_2024_002" => {
                        // Customer prints but doesn't claim
                        service.print_check(&response.check.check_id).unwrap();
                        println!("   📄 Напечатан, но не активирован");
                    }
                    "sale_2024_003" => {
                        // Customer prints and claims
                        service.print_check(&response.check.check_id).unwrap();
                        let claim_request = CheckClaimRequest {
                            qr_data: response.check.qr_data.clone(),
                            user_id: "user_002".to_string(),
                            user_wallet: "0xuser_wallet_abcdef1234567890".to_string(),
                        };
                        service.claim_check(claim_request).unwrap();
                        println!("   ✅ Напечатан и активирован");
                    }
                    "sale_2024_004" => {
                        // Customer discards the check
                        service.print_check(&response.check.check_id).unwrap();
                        service.discard_check(&response.check.check_id).unwrap();
                        println!("   🗑️  Напечатан, но выброшен");
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("❌ Ошибка генерации чека {}: {}", sale_id, e);
            }
        }
    }

    // Demo 5: Check statistics and analytics
    println!("\n📊 Демо 5: Статистика и аналитика");
    println!("{}", "-".repeat(30));

    let stats = service.get_statistics();
    println!("📈 Общая статистика чеков:");
    println!("   Всего чеков: {}", stats.total_checks);
    println!("   Сгенерировано: {}", stats.generated);
    println!("   Напечатано: {}", stats.printed);
    println!("   Активировано: {}", stats.claimed);
    println!("   Истекших: {}", stats.expired);
    println!("   Выброшенных: {}", stats.discarded);
    println!("   Токенов активировано: {} ST", stats.total_tokens_claimed);
    println!("   Токенов не активировано: {} ST", stats.total_tokens_unclaimed);

    // Demo 6: Unclaimed checks for redistribution
    println!("\n🔄 Демо 6: Невостребованные чеки");
    println!("{}", "-".repeat(30));

    let unclaimed = service.get_unclaimed_checks();
    println!("📋 Невостребованные чеки ({} шт.):", unclaimed.len());
    for check in unclaimed {
        println!("   {}: {} GEL → {} ST (статус: {:?})", 
            check.check_id, check.amount_gel, check.st_tokens, check.status);
    }

    let expired = service.get_expired_checks();
    println!("⏰ Истекшие чеки ({} шт.):", expired.len());
    for check in expired {
        println!("   {}: {} GEL → {} ST", 
            check.check_id, check.amount_gel, check.st_tokens);
    }

    // Demo 7: Customer journey simulation
    println!("\n👤 Демо 7: Симуляция пути покупателя");
    println!("{}", "-".repeat(30));

    println!("🛒 Покупатель делает заказ на 30 GEL");
    let customer_request = CheckGenerationRequest {
        sale_id: "customer_journey_001".to_string(),
        node_id: "node_tbilisi_001".to_string(),
        amount_gel: 30.0,
        st_tokens: 600, // 30 GEL * 20
        customer_phone: None,
    };

    match service.generate_check(customer_request) {
        Ok(response) => {
            println!("✅ Получен чек с QR-кодом");
            service.print_check(&response.check.check_id).unwrap();
            println!("📄 Чек напечатан и выдан покупателю");
            
            println!("\n🤔 Выбор покупателя:");
            println!("   Вариант А: Выбросить чек → токены остаются в обезличенном кошельке");
            println!("   Вариант Б: Сканировать QR-код → перейти к мобильному приложению");
            
            // Simulate customer choosing option B
            println!("\n📱 Покупатель выбирает вариант Б:");
            println!("   1. Сканирует QR-код");
            println!("   2. Переходит на страницу скачивания мобильного приложения");
            println!("   3. Скачивает приложение для своей ОС");
            println!("   4. Регистрируется в приложении");
            println!("   5. Сканирует QR-код в приложении");
            
            let claim_request = CheckClaimRequest {
                qr_data: response.check.qr_data.clone(),
                user_id: "customer_001".to_string(),
                user_wallet: "0xcustomer_wallet_1234567890abcdef".to_string(),
            };

            match service.claim_check(claim_request) {
                Ok(claim_response) => {
                    println!("✅ Токены успешно переведены на личный кошелек!");
                    println!("   Получено: {} ST токенов", claim_response.transferred_tokens);
                    println!("   Кошелек: 0xcustomer_wallet_1234567890abcdef");
                    println!("   Теперь покупатель может участвовать в DAO и получать дивиденды!");
                }
                Err(e) => {
                    println!("❌ Ошибка перевода токенов: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Ошибка генерации чека: {}", e);
        }
    }

    // Final statistics
    println!("\n📊 Финальная статистика:");
    println!("{}", "=".repeat(50));
    let final_stats = service.get_statistics();
    println!("Всего чеков: {}", final_stats.total_checks);
    println!("Активировано: {} ({}%)", 
        final_stats.claimed,
        if final_stats.total_checks > 0 {
            (final_stats.claimed * 100) / final_stats.total_checks
        } else { 0 }
    );
    println!("Не активировано: {} ({}%)", 
        final_stats.printed + final_stats.discarded,
        if final_stats.total_checks > 0 {
            ((final_stats.printed + final_stats.discarded) * 100) / final_stats.total_checks
        } else { 0 }
    );
    println!("Токенов в обращении: {} ST", final_stats.total_tokens_claimed);
    println!("Токенов для перераспределения: {} ST", final_stats.total_tokens_unclaimed);

    println!("\n🎉 Демонстрация завершена!");
    println!("💡 Ключевые особенности:");
    println!("   • Прозрачная эмиссия токенов (1 THP = 5 GEL затрат)");
    println!("   • Обезличенные кошельки для анонимности");
    println!("   • QR-коды для простой активации");
    println!("   • Автоматическое перераспределение невостребованных токенов");
    println!("   • Полная трассируемость в блокчейне");
}

