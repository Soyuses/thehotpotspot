use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use blockchain_project::{
    transparency_reporting::{
        TransparencyReportingSystem, ReportGenerationRequest, ReportType,
        TransparencyConfig, NotificationType, NotificationPriority,
    },
    cost_tracking::CostTracker,
    new_tokenomics::NewTokenomicsManager,
    tokenomics_config::TokenomicsConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Демонстрация системы прозрачности и отчетности");
    println!("==================================================");

    // Инициализация системы прозрачности
    let mut transparency_system = TransparencyReportingSystem::new();
    transparency_system.config = TransparencyConfig {
        auto_report_generation: true,
        report_frequency: ReportType::Monthly,
        notification_enabled: true,
        public_access: true,
        data_retention_days: 365,
        cost_threshold_alerts: 1000.0,
        revenue_threshold_alerts: 5000.0,
        efficiency_threshold_alerts: 0.8,
    };

    // Инициализация системы учета затрат
    let cost_tracker = CostTracker::new();
    
    // Инициализация токеномики
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = NewTokenomicsManager::new(tokenomics_config);

    // Mock данные о продажах
    let sales_data = HashMap::from([
        ("plov".to_string(), 1250.0),
        ("khinkali".to_string(), 980.0),
        ("khachapuri".to_string(), 1100.0),
        ("salad".to_string(), 450.0),
        ("tea".to_string(), 200.0),
    ]);

    println!("\n📊 Генерация ежемесячного отчета о прозрачности");
    println!("------------------------------------------------");

    // Создание запроса на генерацию отчета
    let report_request = ReportGenerationRequest {
        report_type: ReportType::Monthly,
        period_start: Utc::now() - chrono::Duration::days(30),
        period_end: Utc::now(),
        include_details: true,
        include_recommendations: true,
        generated_by: "admin".to_string(),
    };

    // Генерация отчета
    let report_response = transparency_system
        .generate_report(
            report_request,
            &cost_tracker,
            &tokenomics_manager,
            &sales_data,
        )
        .await;

    if report_response.success {
        println!("✅ Отчет успешно сгенерирован!");
        if let Some(report_id) = &report_response.report_id {
            println!("   ID отчета: {}", report_id);
        }
        
        // Получение сгенерированного отчета
        if let Some(ref report_id) = report_response.report_id {
            if let Some(report) = transparency_system.get_report(&report_id) {
                println!("\n📋 Детали отчета:");
                println!("   Тип: {:?}", report.report_type);
                println!("   Период: {} - {}", report.period_start, report.period_end);
                println!("   Статус: {:?}", report.status);
                
                println!("\n💰 Финансовые показатели:");
                println!("   Общая выручка: {:.2} GEL", report.data.total_revenue);
                println!("   Общие затраты: {:.2} GEL", report.data.total_costs);
                println!("   Чистая прибыль: {:.2} GEL", report.data.net_profit);
                println!("   Рентабельность: {:.2}%", report.data.profit_margin * 100.0);
                
                println!("\n🪙 Токеномика:");
                println!("   ST токены эмитированы: {}", report.data.st_tokens_emitted);
                println!("   UT токены заработаны: {}", report.data.ut_tokens_earned);
                println!("   Раунды конвертации: {}", report.data.conversion_rounds);
                println!("   DAO предложения: {}", report.data.dao_proposals);
                
                println!("\n⚡ Эффективность:");
                println!("   Эффективность затрат: {:.2}x", report.efficiency_metrics.cost_efficiency);
                println!("   Эффективность токенов: {:.2}", report.efficiency_metrics.token_efficiency);
                println!("   Ставка эмиссии: {:.2} THP/GEL", report.token_emission.st_emission_rate);
                
                println!("\n💡 Ключевые моменты:");
                for highlight in &report.summary.key_highlights {
                    println!("   • {}", highlight);
                }
                
                println!("\n📈 Рекомендации:");
                for recommendation in &report.summary.recommendations {
                    println!("   • {}", recommendation);
                }
                
                println!("\n⚠️ Риски:");
                for risk in &report.summary.risks {
                    println!("   • {}", risk);
                }
                
                println!("\n🚀 Возможности:");
                for opportunity in &report.summary.opportunities {
                    println!("   • {}", opportunity);
                }
            }
        }
    } else {
        println!("❌ Ошибка генерации отчета: {}", report_response.error.unwrap());
    }

    println!("\n🔔 Система уведомлений");
    println!("----------------------");

    // Отправка уведомлений
    transparency_system.send_notification(blockchain_project::transparency_reporting::TransparencyNotification {
        notification_id: uuid::Uuid::new_v4().to_string(),
        notification_type: NotificationType::ReportGenerated,
        title: "Отчет о прозрачности сгенерирован".to_string(),
        message: "Ежемесячный отчет о прозрачности готов к просмотру".to_string(),
        created_at: Utc::now(),
        priority: NotificationPriority::Medium,
        recipients: vec!["admin".to_string(), "owner".to_string()],
        status: blockchain_project::transparency_reporting::NotificationStatus::Pending,
        action_required: false,
        action_url: Some("/reports/monthly".to_string()),
    });

    transparency_system.send_notification(blockchain_project::transparency_reporting::TransparencyNotification {
        notification_id: uuid::Uuid::new_v4().to_string(),
        notification_type: NotificationType::CostAnomaly,
        title: "Аномалия в затратах".to_string(),
        message: "Обнаружено превышение затрат на ингредиенты на 15%".to_string(),
        created_at: Utc::now(),
        priority: NotificationPriority::High,
        recipients: vec!["admin".to_string(), "finance".to_string()],
        status: blockchain_project::transparency_reporting::NotificationStatus::Pending,
        action_required: true,
        action_url: Some("/costs/analysis".to_string()),
    });

    // Отображение уведомлений
    let notifications = transparency_system.get_notifications(Some(10));
    println!("📬 Активные уведомления ({}):", notifications.len());
    for notification in notifications {
        println!("   🔔 {} - {}", notification.title, notification.message);
        println!("      Приоритет: {:?}, Статус: {:?}", notification.priority, notification.status);
        if notification.action_required {
            println!("      ⚠️ Требуется действие: {}", notification.action_url.as_ref().unwrap_or(&"N/A".to_string()));
        }
    }

    println!("\n📊 Экспорт отчетов");
    println!("------------------");

    // Экспорт отчета в JSON
    if let Some(ref report_id) = report_response.report_id {
        match transparency_system.export_report_json(&report_id) {
            Ok(json_data) => {
                println!("✅ JSON экспорт успешен");
                println!("   Размер: {} байт", json_data.len());
            }
            Err(e) => {
                println!("❌ Ошибка JSON экспорта: {}", e);
            }
        }

        // Экспорт отчета в CSV
        match transparency_system.export_report_csv(&report_id) {
            Ok(csv_data) => {
                println!("✅ CSV экспорт успешен");
                println!("   Размер: {} байт", csv_data.len());
                println!("   Данные:\n{}", csv_data);
            }
            Err(e) => {
                println!("❌ Ошибка CSV экспорта: {}", e);
            }
        }
    }

    println!("\n🌐 Публикация отчета");
    println!("--------------------");

    // Публикация отчета
    if let Some(ref report_id) = report_response.report_id {
        match transparency_system.publish_report(&report_id) {
            Ok(_) => {
                println!("✅ Отчет успешно опубликован для публичного доступа");
            }
            Err(e) => {
                println!("❌ Ошибка публикации: {}", e);
            }
        }
    }

    println!("\n📈 Статистика системы");
    println!("--------------------");
    println!("   Всего отчетов: {}", transparency_system.reports.len());
    println!("   Всего уведомлений: {}", transparency_system.notifications.len());
    println!("   Автогенерация отчетов: {}", transparency_system.config.auto_report_generation);
    println!("   Частота отчетов: {:?}", transparency_system.config.report_frequency);
    println!("   Публичный доступ: {}", transparency_system.config.public_access);

    println!("\n🎯 Демонстрация завершена!");
    println!("Система прозрачности и отчетности готова к использованию.");

    Ok(())
}
