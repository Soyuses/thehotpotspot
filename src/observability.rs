//! Observability module для The Hot Pot Spot
//! 
//! Обеспечивает структурированное логирование, метрики и мониторинг
//! для промышленного уровня приложения.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tokio::time::{interval, Duration as TokioDuration};

/// Уровни логирования
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

/// Структура лога
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub function: Option<String>,
    pub line: Option<u32>,
    pub thread_id: String,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub error: Option<ErrorInfo>,
}

/// Информация об ошибке
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub error_type: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub error_code: Option<String>,
}

/// Метрика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    pub metric_type: MetricType,
}

/// Тип метрики
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricType {
    Counter,    // Счетчик
    Gauge,      // Измеритель
    Histogram,  // Гистограмма
    Summary,    // Сводка
}

/// Алерт
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: String,
    pub name: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub status: AlertStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
}

/// Серьезность алерта
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Статус алерта
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Firing,
    Resolved,
    Suppressed,
}

/// Конфигурация observability
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub log_level: LogLevel,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub metrics_interval: Duration,
    pub log_buffer_size: usize,
    pub metrics_buffer_size: usize,
    pub prometheus_port: u16,
    pub jaeger_endpoint: Option<String>,
    pub elasticsearch_endpoint: Option<String>,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            enable_metrics: true,
            enable_tracing: true,
            metrics_interval: Duration::from_secs(60),
            log_buffer_size: 10000,
            metrics_buffer_size: 1000,
            prometheus_port: 9090,
            jaeger_endpoint: None,
            elasticsearch_endpoint: None,
        }
    }
}

/// Менеджер observability
#[derive(Clone)]
pub struct ObservabilityManager {
    config: ObservabilityConfig,
    logs: Arc<RwLock<Vec<LogEntry>>>,
    metrics: Arc<RwLock<Vec<Metric>>>,
    alerts: Arc<RwLock<Vec<Alert>>>,
    counters: Arc<RwLock<HashMap<String, f64>>>,
    gauges: Arc<RwLock<HashMap<String, f64>>>,
    histograms: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    request_counter: Arc<RwLock<u64>>,
    error_counter: Arc<RwLock<u64>>,
    active_connections: Arc<RwLock<u32>>,
}

impl ObservabilityManager {
    /// Создание нового менеджера observability
    pub fn new(config: ObservabilityConfig) -> Self {
        let manager = Self {
            config,
            logs: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
            request_counter: Arc::new(RwLock::new(0)),
            error_counter: Arc::new(RwLock::new(0)),
            active_connections: Arc::new(RwLock::new(0)),
        };

        // Запускаем фоновые задачи
        manager.start_background_tasks();

        manager
    }

    /// Логирование сообщения
    pub async fn log(&self, level: LogLevel, message: &str, module: &str, metadata: Option<HashMap<String, String>>) {
        if level < self.config.log_level {
            return;
        }

        let log_entry = LogEntry {
            timestamp: Utc::now(),
            level,
            message: message.to_string(),
            module: module.to_string(),
            function: None,
            line: None,
            thread_id: format!("{:?}", std::thread::current().id()),
            request_id: None,
            user_id: None,
            metadata: metadata.unwrap_or_default(),
            error: None,
        };

        let mut logs = self.logs.write().await;
        logs.push(log_entry);

        // Ограничиваем размер буфера
        if logs.len() > self.config.log_buffer_size {
            let excess = logs.len() - self.config.log_buffer_size;
            logs.drain(0..excess);
        }
    }

    /// Логирование ошибки
    pub async fn log_error(&self, message: &str, module: &str, error: ErrorInfo, metadata: Option<HashMap<String, String>>) {
        let log_entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Error,
            message: message.to_string(),
            module: module.to_string(),
            function: None,
            line: None,
            thread_id: format!("{:?}", std::thread::current().id()),
            request_id: None,
            user_id: None,
            metadata: metadata.unwrap_or_default(),
            error: Some(error),
        };

        let mut logs = self.logs.write().await;
        logs.push(log_entry);

        // Увеличиваем счетчик ошибок
        let mut error_counter = self.error_counter.write().await;
        *error_counter += 1;

        // Ограничиваем размер буфера
        if logs.len() > self.config.log_buffer_size {
            let excess = logs.len() - self.config.log_buffer_size;
            logs.drain(0..excess);
        }
    }

    /// Увеличение счетчика
    pub async fn increment_counter(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let mut counters = self.counters.write().await;
        *counters.entry(name.to_string()).or_insert(0.0) += value;

        if self.config.enable_metrics {
            let metric = Metric {
                name: name.to_string(),
                value: *counters.get(name).unwrap(),
                timestamp: Utc::now(),
                labels: labels.unwrap_or_default(),
                metric_type: MetricType::Counter,
            };

            let mut metrics = self.metrics.write().await;
            metrics.push(metric);

            if metrics.len() > self.config.metrics_buffer_size {
                let excess = metrics.len() - self.config.metrics_buffer_size;
                metrics.drain(0..excess);
            }
        }
    }

    /// Установка значения измерителя
    pub async fn set_gauge(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let mut gauges = self.gauges.write().await;
        gauges.insert(name.to_string(), value);

        if self.config.enable_metrics {
            let metric = Metric {
                name: name.to_string(),
                value,
                timestamp: Utc::now(),
                labels: labels.unwrap_or_default(),
                metric_type: MetricType::Gauge,
            };

            let mut metrics = self.metrics.write().await;
            metrics.push(metric);

            if metrics.len() > self.config.metrics_buffer_size {
                let excess = metrics.len() - self.config.metrics_buffer_size;
                metrics.drain(0..excess);
            }
        }
    }

    /// Добавление значения в гистограмму
    pub async fn observe_histogram(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let mut histograms = self.histograms.write().await;
        histograms.entry(name.to_string()).or_insert_with(Vec::new).push(value);

        if self.config.enable_metrics {
            let metric = Metric {
                name: name.to_string(),
                value,
                timestamp: Utc::now(),
                labels: labels.unwrap_or_default(),
                metric_type: MetricType::Histogram,
            };

            let mut metrics = self.metrics.write().await;
            metrics.push(metric);

            if metrics.len() > self.config.metrics_buffer_size {
                let excess = metrics.len() - self.config.metrics_buffer_size;
                metrics.drain(0..excess);
            }
        }
    }

    /// Отслеживание запроса
    pub async fn track_request(&self, method: &str, path: &str, status_code: u16, duration_ms: u64) {
        // Увеличиваем счетчик запросов
        let mut request_counter = self.request_counter.write().await;
        *request_counter += 1;

        // Логируем запрос
        let mut metadata = HashMap::new();
        metadata.insert("method".to_string(), method.to_string());
        metadata.insert("path".to_string(), path.to_string());
        metadata.insert("status_code".to_string(), status_code.to_string());
        metadata.insert("duration_ms".to_string(), duration_ms.to_string());

        self.log(LogLevel::Info, &format!("Request: {} {} - {}ms", method, path, duration_ms), "http", Some(metadata.clone())).await;

        // Обновляем метрики
        self.increment_counter("http_requests_total", 1.0, Some(metadata.clone())).await;
        self.observe_histogram("http_request_duration_ms", duration_ms as f64, Some(metadata.clone())).await;

        // Проверяем на алерты
        if status_code >= 500 {
            self.create_alert(
                "high_error_rate",
                "High Error Rate",
                &format!("High error rate detected: {} {} returned {}", method, path, status_code),
                AlertSeverity::Warning,
                Some(metadata),
            ).await;
        }
    }

    /// Создание алерта
    pub async fn create_alert(&self, alert_id: &str, name: &str, description: &str, severity: AlertSeverity, labels: Option<HashMap<String, String>>) {
        let alert = Alert {
            alert_id: alert_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            severity: severity.clone(),
            status: AlertStatus::Firing,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            resolved_at: None,
            labels: labels.unwrap_or_default(),
            annotations: HashMap::new(),
        };

        let mut alerts = self.alerts.write().await;
        alerts.push(alert);

        // Логируем алерт
        let mut metadata = HashMap::new();
        metadata.insert("alert_id".to_string(), alert_id.to_string());
        metadata.insert("severity".to_string(), format!("{:?}", severity));
        
        self.log(LogLevel::Warn, &format!("Alert: {}", name), "alerts", Some(metadata)).await;
    }

    /// Разрешение алерта
    pub async fn resolve_alert(&self, alert_id: &str) {
        let mut alerts = self.alerts.write().await;
        for alert in alerts.iter_mut() {
            if alert.alert_id == alert_id && alert.status == AlertStatus::Firing {
                alert.status = AlertStatus::Resolved;
                alert.resolved_at = Some(Utc::now());
                alert.updated_at = Utc::now();
                break;
            }
        }
    }

    /// Получение логов
    pub async fn get_logs(&self, level: Option<LogLevel>, limit: Option<usize>) -> Vec<LogEntry> {
        let logs = self.logs.read().await;
        let mut filtered_logs: Vec<LogEntry> = logs.clone();

        // Фильтрация по уровню
        if let Some(min_level) = level {
            filtered_logs.retain(|log| log.level >= min_level);
        }

        // Ограничение количества
        if let Some(limit) = limit {
            filtered_logs.truncate(limit);
        }

        filtered_logs
    }

    /// Получение метрик
    pub async fn get_metrics(&self, metric_type: Option<MetricType>, limit: Option<usize>) -> Vec<Metric> {
        let metrics = self.metrics.read().await;
        let mut filtered_metrics: Vec<Metric> = metrics.clone();

        // Фильтрация по типу
        if let Some(metric_type) = metric_type {
            filtered_metrics.retain(|metric| metric.metric_type == metric_type);
        }

        // Ограничение количества
        if let Some(limit) = limit {
            filtered_metrics.truncate(limit);
        }

        filtered_metrics
    }

    /// Получение алертов
    pub async fn get_alerts(&self, status: Option<AlertStatus>, severity: Option<AlertSeverity>) -> Vec<Alert> {
        let alerts = self.alerts.read().await;
        let mut filtered_alerts: Vec<Alert> = alerts.clone();

        // Фильтрация по статусу
        if let Some(status) = status {
            filtered_alerts.retain(|alert| alert.status == status);
        }

        // Фильтрация по серьезности
        if let Some(severity) = severity {
            filtered_alerts.retain(|alert| alert.severity == severity);
        }

        filtered_alerts
    }

    /// Получение статистики
    pub async fn get_statistics(&self) -> ObservabilityStats {
        let logs = self.logs.read().await;
        let metrics = self.metrics.read().await;
        let alerts = self.alerts.read().await;
        let request_counter = self.request_counter.read().await;
        let error_counter = self.error_counter.read().await;
        let active_connections = self.active_connections.read().await;

        let mut log_counts = HashMap::new();
        for log in logs.iter() {
            *log_counts.entry(format!("{:?}", log.level)).or_insert(0) += 1;
        }

        let mut metric_counts = HashMap::new();
        for metric in metrics.iter() {
            *metric_counts.entry(format!("{:?}", metric.metric_type)).or_insert(0) += 1;
        }

        let mut alert_counts = HashMap::new();
        for alert in alerts.iter() {
            *alert_counts.entry(format!("{:?}", alert.severity)).or_insert(0) += 1;
        }

        ObservabilityStats {
            total_logs: logs.len() as u32,
            total_metrics: metrics.len() as u32,
            total_alerts: alerts.len() as u32,
            total_requests: *request_counter as u32,
            total_errors: *error_counter as u32,
            active_connections: *active_connections,
            log_counts,
            metric_counts,
            alert_counts,
        }
    }

    /// Генерация Prometheus метрик
    pub async fn generate_prometheus_metrics(&self) -> String {
        let mut output = String::new();
        
        // Счетчики
        let counters = self.counters.read().await;
        for (name, value) in counters.iter() {
            output.push_str(&format!("{} {}\n", name, value));
        }

        // Измерители
        let gauges = self.gauges.read().await;
        for (name, value) in gauges.iter() {
            output.push_str(&format!("{} {}\n", name, value));
        }

        // Гистограммы
        let histograms = self.histograms.read().await;
        for (name, values) in histograms.iter() {
            if !values.is_empty() {
                let sum: f64 = values.iter().sum();
                let count = values.len() as f64;
                let avg = sum / count;
                
                output.push_str(&format!("{}_sum {}\n", name, sum));
                output.push_str(&format!("{}_count {}\n", name, count));
                output.push_str(&format!("{}_avg {}\n", name, avg));
            }
        }

        // Системные метрики
        let request_counter = self.request_counter.read().await;
        let error_counter = self.error_counter.read().await;
        let active_connections = self.active_connections.read().await;

        output.push_str(&format!("http_requests_total {}\n", request_counter));
        output.push_str(&format!("http_errors_total {}\n", error_counter));
        output.push_str(&format!("active_connections {}\n", active_connections));

        output
    }

    /// Запуск фоновых задач
    fn start_background_tasks(&self) {
        let config = self.config.clone();
        let metrics = Arc::clone(&self.metrics);
        let _counters = Arc::clone(&self.counters);
        let _gauges = Arc::clone(&self.gauges);

        // Задача сбора метрик
        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(config.metrics_interval.as_secs()));
            
            loop {
                interval.tick().await;
                
                // Собираем системные метрики
                let system_metrics = collect_system_metrics().await;
                
                let mut metrics_guard = metrics.write().await;
                for metric in system_metrics {
                    metrics_guard.push(metric);
                    
                    if metrics_guard.len() > config.metrics_buffer_size {
                        let excess = metrics_guard.len() - config.metrics_buffer_size;
            metrics_guard.drain(0..excess);
                    }
                }
            }
        });
    }
}

/// Сбор системных метрик
async fn collect_system_metrics() -> Vec<Metric> {
    let mut metrics = Vec::new();
    let timestamp = Utc::now();

    // Использование памяти
    if let Ok(memory_info) = get_memory_usage() {
        metrics.push(Metric {
            name: "memory_usage_bytes".to_string(),
            value: memory_info.used as f64,
            timestamp,
            labels: HashMap::new(),
            metric_type: MetricType::Gauge,
        });
    }

    // Использование CPU
    if let Ok(cpu_usage) = get_cpu_usage() {
        metrics.push(Metric {
            name: "cpu_usage_percent".to_string(),
            value: cpu_usage,
            timestamp,
            labels: HashMap::new(),
            metric_type: MetricType::Gauge,
        });
    }

    // Количество открытых файлов
    if let Ok(open_files) = get_open_files_count() {
        metrics.push(Metric {
            name: "open_files_count".to_string(),
            value: open_files as f64,
            timestamp,
            labels: HashMap::new(),
            metric_type: MetricType::Gauge,
        });
    }

    metrics
}

/// Получение информации об использовании памяти
fn get_memory_usage() -> Result<MemoryInfo, String> {
    // Упрощенная реализация для демонстрации
    Ok(MemoryInfo {
        total: 8 * 1024 * 1024 * 1024, // 8GB
        used: 2 * 1024 * 1024 * 1024,  // 2GB
        free: 6 * 1024 * 1024 * 1024,  // 6GB
    })
}

/// Получение использования CPU
fn get_cpu_usage() -> Result<f64, String> {
    // Упрощенная реализация для демонстрации
    Ok(25.5) // 25.5%
}

/// Получение количества открытых файлов
fn get_open_files_count() -> Result<u32, String> {
    // Упрощенная реализация для демонстрации
    Ok(1024)
}

/// Информация о памяти
#[derive(Debug)]
struct MemoryInfo {
    total: u64,
    used: u64,
    free: u64,
}

/// Статистика observability
#[derive(Debug, Serialize, Deserialize)]
pub struct ObservabilityStats {
    pub total_logs: u32,
    pub total_metrics: u32,
    pub total_alerts: u32,
    pub total_requests: u32,
    pub total_errors: u32,
    pub active_connections: u32,
    pub log_counts: HashMap<String, u32>,
    pub metric_counts: HashMap<String, u32>,
    pub alert_counts: HashMap<String, u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logging() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);
        
        manager.log(LogLevel::Info, "Test message", "test_module", None).await;
        
        let logs = manager.get_logs(None, None).await;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Test message");
    }

    #[tokio::test]
    async fn test_metrics() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);
        
        manager.increment_counter("test_counter", 1.0, None).await;
        manager.set_gauge("test_gauge", 42.0, None).await;
        
        let metrics = manager.get_metrics(None, None).await;
        assert!(metrics.len() >= 2);
    }

    #[tokio::test]
    async fn test_alerts() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);
        
        manager.create_alert("test_alert", "Test Alert", "Test description", AlertSeverity::Warning, None).await;
        
        let alerts = manager.get_alerts(None, None).await;
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].name, "Test Alert");
    }
}

