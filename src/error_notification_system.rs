//! Система уведомлений об ошибках для ARM владельца сети
//! 
//! Обеспечивает отправку уведомлений об ошибках в ARM владельца сети
//! с целью достижения 0.001% ошибок и 100% покрытия тестами.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::observability::AlertSeverity;

/// Тип уведомления об ошибке
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorNotificationType {
    /// Критическая ошибка (требует немедленного вмешательства)
    Critical,
    /// Высокий приоритет (требует внимания в течение часа)
    High,
    /// Средний приоритет (требует внимания в течение дня)
    Medium,
    /// Низкий приоритет (информационное сообщение)
    Low,
    /// Ошибка тестирования (недостижение 100% покрытия)
    TestCoverage,
    /// Ошибка производительности (превышение 0.001% ошибок)
    Performance,
}

/// Уведомление об ошибке
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorNotification {
    pub id: String,
    pub notification_type: ErrorNotificationType,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub component: String,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
    pub stack_trace: Option<String>,
    pub user_impact: String,
    pub suggested_action: String,
}

/// Канал уведомлений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email уведомление
    Email(String),
    /// SMS уведомление
    SMS(String),
    /// Push уведомление
    Push(String),
    /// Webhook уведомление
    Webhook(String),
    /// Telegram уведомление
    Telegram(String),
    /// Slack уведомление
    Slack(String),
}

/// Конфигурация системы уведомлений
#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub owner_arm_id: String,
    pub channels: Vec<NotificationChannel>,
    pub critical_threshold: f64, // 0.001% = 0.00001
    pub test_coverage_threshold: f64, // 100% = 1.0
    pub notification_cooldown: u64, // секунды между уведомлениями
    pub max_notifications_per_hour: u32,
}

/// Статистика ошибок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStatistics {
    pub total_requests: u64,
    pub total_errors: u64,
    pub error_rate: f64,
    pub test_coverage: f64,
    pub last_notification_time: u64,
    pub notifications_sent_this_hour: u32,
    pub error_breakdown: HashMap<String, u64>,
}

/// Система уведомлений об ошибках
pub struct ErrorNotificationSystem {
    config: NotificationConfig,
    statistics: Arc<RwLock<ErrorStatistics>>,
    notification_sender: mpsc::UnboundedSender<ErrorNotification>,
    notification_receiver: Arc<RwLock<mpsc::UnboundedReceiver<ErrorNotification>>>,
    sent_notifications: Arc<RwLock<HashMap<String, u64>>>, // notification_id -> timestamp
}

impl ErrorNotificationSystem {
    /// Создать новую систему уведомлений
    pub fn new(config: NotificationConfig) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Self {
            config,
            statistics: Arc::new(RwLock::new(ErrorStatistics {
                total_requests: 0,
                total_errors: 0,
                error_rate: 0.0,
                test_coverage: 0.0,
                last_notification_time: 0,
                notifications_sent_this_hour: 0,
                error_breakdown: HashMap::new(),
            })),
            notification_sender: sender,
            notification_receiver: Arc::new(RwLock::new(receiver)),
            sent_notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Запустить систему уведомлений
    pub async fn start(&self) -> Result<(), String> {
        let receiver = Arc::clone(&self.notification_receiver);
        let config = self.config.clone();
        let statistics = Arc::clone(&self.statistics);
        let sent_notifications = Arc::clone(&self.sent_notifications);

        tokio::spawn(async move {
            let mut receiver = receiver.write().await;
            while let Some(notification) = receiver.recv().await {
                Self::process_notification(
                    notification,
                    &config,
                    &statistics,
                    &sent_notifications,
                ).await;
            }
        });

        Ok(())
    }

    /// Обработать уведомление
    async fn process_notification(
        notification: ErrorNotification,
        config: &NotificationConfig,
        statistics: &Arc<RwLock<ErrorStatistics>>,
        sent_notifications: &Arc<RwLock<HashMap<String, u64>>>,
    ) {
        // Проверяем, не отправляли ли мы уже это уведомление
        let sent = sent_notifications.read().await;
        if let Some(last_sent) = sent.get(&notification.id) {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if current_time - last_sent < config.notification_cooldown {
                return; // Пропускаем уведомление из-за cooldown
            }
        }
        drop(sent);

        // Проверяем лимит уведомлений в час
        let stats = statistics.read().await;
        if stats.notifications_sent_this_hour >= config.max_notifications_per_hour {
            return; // Пропускаем уведомление из-за лимита
        }
        drop(stats);

        // Отправляем уведомление
        Self::send_notification(&notification, config).await;

        // Обновляем статистику
        let mut sent = sent_notifications.write().await;
        sent.insert(notification.id.clone(), SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs());

        let mut stats = statistics.write().await;
        stats.last_notification_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        stats.notifications_sent_this_hour += 1;
    }

    /// Отправить уведомление
    async fn send_notification(
        notification: &ErrorNotification,
        config: &NotificationConfig,
    ) {
        for channel in &config.channels {
            match channel {
                NotificationChannel::Email(email) => {
                    Self::send_email_notification(notification, email).await;
                },
                NotificationChannel::SMS(phone) => {
                    Self::send_sms_notification(notification, phone).await;
                },
                NotificationChannel::Push(device_id) => {
                    Self::send_push_notification(notification, device_id).await;
                },
                NotificationChannel::Webhook(url) => {
                    Self::send_webhook_notification(notification, url).await;
                },
                NotificationChannel::Telegram(chat_id) => {
                    Self::send_telegram_notification(notification, chat_id).await;
                },
                NotificationChannel::Slack(webhook_url) => {
                    Self::send_slack_notification(notification, webhook_url).await;
                },
            }
        }
    }

    /// Отправить email уведомление
    async fn send_email_notification(notification: &ErrorNotification, email: &str) {
        // Здесь должна быть интеграция с email сервисом
        println!("📧 EMAIL NOTIFICATION to {}: {} - {}", 
                 email, notification.title, notification.message);
    }

    /// Отправить SMS уведомление
    async fn send_sms_notification(notification: &ErrorNotification, phone: &str) {
        // Здесь должна быть интеграция с SMS сервисом
        println!("📱 SMS NOTIFICATION to {}: {} - {}", 
                 phone, notification.title, notification.message);
    }

    /// Отправить push уведомление
    async fn send_push_notification(notification: &ErrorNotification, device_id: &str) {
        // Здесь должна быть интеграция с push сервисом
        println!("🔔 PUSH NOTIFICATION to {}: {} - {}", 
                 device_id, notification.title, notification.message);
    }

    /// Отправить webhook уведомление
    async fn send_webhook_notification(notification: &ErrorNotification, url: &str) {
        // Здесь должна быть интеграция с webhook сервисом
        println!("🔗 WEBHOOK NOTIFICATION to {}: {} - {}", 
                 url, notification.title, notification.message);
    }

    /// Отправить Telegram уведомление
    async fn send_telegram_notification(notification: &ErrorNotification, chat_id: &str) {
        // Здесь должна быть интеграция с Telegram Bot API
        println!("📨 TELEGRAM NOTIFICATION to {}: {} - {}", 
                 chat_id, notification.title, notification.message);
    }

    /// Отправить Slack уведомление
    async fn send_slack_notification(notification: &ErrorNotification, webhook_url: &str) {
        // Здесь должна быть интеграция с Slack API
        println!("💬 SLACK NOTIFICATION to {}: {} - {}", 
                 webhook_url, notification.title, notification.message);
    }

    /// Зарегистрировать ошибку
    pub async fn register_error(
        &self,
        error_type: ErrorNotificationType,
        title: String,
        message: String,
        component: String,
        stack_trace: Option<String>,
        user_impact: String,
        suggested_action: String,
    ) -> Result<(), String> {
        let notification = ErrorNotification {
            id: format!("error_{}", SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            notification_type: error_type.clone(),
            severity: match error_type {
                ErrorNotificationType::Critical => AlertSeverity::Critical,
                ErrorNotificationType::High => AlertSeverity::Warning,
                ErrorNotificationType::Medium => AlertSeverity::Warning,
                ErrorNotificationType::Low => AlertSeverity::Info,
                ErrorNotificationType::TestCoverage => AlertSeverity::Warning,
                ErrorNotificationType::Performance => AlertSeverity::Critical,
            },
            title,
            message,
            component,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: HashMap::new(),
            stack_trace,
            user_impact,
            suggested_action,
        };

        // Обновляем статистику
        let mut stats = self.statistics.write().await;
        stats.total_errors += 1;
        stats.error_rate = stats.total_errors as f64 / stats.total_requests as f64;
        
        // Проверяем, превышен ли порог ошибок
        if stats.error_rate > self.config.critical_threshold {
            let _ = self.notification_sender.send(notification.clone());
        }

        // Проверяем покрытие тестами
        if stats.test_coverage < self.config.test_coverage_threshold {
            let coverage_notification = ErrorNotification {
                id: format!("coverage_{}", SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()),
                notification_type: ErrorNotificationType::TestCoverage,
                severity: AlertSeverity::Warning,
                title: "Test Coverage Below Target".to_string(),
                message: format!("Current test coverage: {:.2}%, Target: {:.2}%", 
                               stats.test_coverage * 100.0, 
                               self.config.test_coverage_threshold * 100.0),
                component: "Test System".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                metadata: HashMap::new(),
                stack_trace: None,
                user_impact: "Reduced code quality and reliability".to_string(),
                suggested_action: "Increase test coverage to 100%".to_string(),
            };
            let _ = self.notification_sender.send(coverage_notification);
        }

        Ok(())
    }

    /// Зарегистрировать успешный запрос
    pub async fn register_success(&self) -> Result<(), String> {
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.error_rate = stats.total_errors as f64 / stats.total_requests as f64;
        Ok(())
    }

    /// Обновить покрытие тестами
    pub async fn update_test_coverage(&self, coverage: f64) -> Result<(), String> {
        let mut stats = self.statistics.write().await;
        stats.test_coverage = coverage;
        
        if coverage < self.config.test_coverage_threshold {
            let notification = ErrorNotification {
                id: format!("coverage_{}", SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()),
                notification_type: ErrorNotificationType::TestCoverage,
                severity: AlertSeverity::Warning,
                title: "Test Coverage Below Target".to_string(),
                message: format!("Current test coverage: {:.2}%, Target: {:.2}%", 
                               coverage * 100.0, 
                               self.config.test_coverage_threshold * 100.0),
                component: "Test System".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                metadata: HashMap::new(),
                stack_trace: None,
                user_impact: "Reduced code quality and reliability".to_string(),
                suggested_action: "Increase test coverage to 100%".to_string(),
            };
            let _ = self.notification_sender.send(notification);
        }
        
        Ok(())
    }

    /// Получить статистику ошибок
    pub async fn get_statistics(&self) -> ErrorStatistics {
        self.statistics.read().await.clone()
    }

    /// Сбросить статистику
    pub async fn reset_statistics(&self) -> Result<(), String> {
        let mut stats = self.statistics.write().await;
        *stats = ErrorStatistics {
            total_requests: 0,
            total_errors: 0,
            error_rate: 0.0,
            test_coverage: 0.0,
            last_notification_time: 0,
            notifications_sent_this_hour: 0,
            error_breakdown: HashMap::new(),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> NotificationConfig {
        NotificationConfig {
            owner_arm_id: "owner_arm_123".to_string(),
            channels: vec![
                NotificationChannel::Email("owner@hotpot.com".to_string()),
                NotificationChannel::Telegram("123456789".to_string()),
            ],
            critical_threshold: 0.00001, // 0.001%
            test_coverage_threshold: 1.0, // 100%
            notification_cooldown: 300, // 5 минут
            max_notifications_per_hour: 10,
        }
    }

    #[tokio::test]
    async fn test_error_notification_system_creation() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config);
        
        // Проверяем, что система создана
        assert_eq!(system.config.owner_arm_id, "owner_arm_123");
        assert_eq!(system.config.channels.len(), 2);
    }

    #[tokio::test]
    async fn test_register_error() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config);
        
        let result = system.register_error(
            ErrorNotificationType::Critical,
            "Test Error".to_string(),
            "This is a test error".to_string(),
            "Test Component".to_string(),
            Some("Stack trace here".to_string()),
            "No user impact".to_string(),
            "Fix the error".to_string(),
        ).await;
        
        assert!(result.is_ok());
        
        let stats = system.get_statistics().await;
        assert_eq!(stats.total_errors, 1);
    }

    #[tokio::test]
    async fn test_register_success() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config);
        
        let result = system.register_success().await;
        assert!(result.is_ok());
        
        let stats = system.get_statistics().await;
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.error_rate, 0.0);
    }

    #[tokio::test]
    async fn test_error_rate_calculation() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config);
        
        // Регистрируем 1000 успешных запросов
        for _ in 0..1000 {
            system.register_success().await.unwrap();
        }
        
        // Регистрируем 1 ошибку
        system.register_error(
            ErrorNotificationType::Critical,
            "Test Error".to_string(),
            "This is a test error".to_string(),
            "Test Component".to_string(),
            None,
            "No user impact".to_string(),
            "Fix the error".to_string(),
        ).await.unwrap();
        
        let stats = system.get_statistics().await;
        assert_eq!(stats.total_requests, 1000);
        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.error_rate, 0.001); // 0.1%
    }

    #[tokio::test]
    async fn test_test_coverage_notification() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config);
        
        // Устанавливаем покрытие ниже целевого
        let result = system.update_test_coverage(0.85).await; // 85%
        assert!(result.is_ok());
        
        let stats = system.get_statistics().await;
        assert_eq!(stats.test_coverage, 0.85);
    }

    #[tokio::test]
    async fn test_critical_threshold_exceeded() {
        let config = create_test_config();
        let system = ErrorNotificationSystem::new(config.clone());
        
        // Регистрируем 1000 успешных запросов
        for _ in 0..1000 {
            system.register_success().await.unwrap();
        }
        
        // Регистрируем 2 ошибки (0.2% > 0.001%)
        for i in 0..2 {
            system.register_error(
                ErrorNotificationType::Critical,
                format!("Test Error {}", i),
                "This is a test error".to_string(),
                "Test Component".to_string(),
                None,
                "No user impact".to_string(),
                "Fix the error".to_string(),
            ).await.unwrap();
        }
        
        let stats = system.get_statistics().await;
        assert!(stats.error_rate > config.critical_threshold);
    }
}
