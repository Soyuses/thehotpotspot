use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Система прозрачности затрат и отчетности
#[derive(Debug, Clone)]
pub struct TransparencyReportingSystem {
    pub reports: HashMap<String, TransparencyReport>,
    pub notifications: Vec<TransparencyNotification>,
    pub config: TransparencyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyReport {
    pub report_id: String,
    pub report_type: ReportType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub generated_at: DateTime<Utc>,
    pub generated_by: String,
    pub status: ReportStatus,
    pub data: ReportData,
    pub summary: ReportSummary,
    pub token_emission: TokenEmissionData,
    pub cost_breakdown: CostBreakdown,
    pub revenue_analysis: RevenueAnalysis,
    pub efficiency_metrics: EfficiencyMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportStatus {
    Generating,
    Completed,
    Failed,
    Published,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub total_revenue: f64,
    pub total_costs: f64,
    pub net_profit: f64,
    pub profit_margin: f64,
    pub st_tokens_emitted: u64,
    pub ut_tokens_earned: u64,
    pub conversion_rounds: u32,
    pub dao_proposals: u32,
    pub active_users: u32,
    pub new_registrations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub key_highlights: Vec<String>,
    pub recommendations: Vec<String>,
    pub risks: Vec<String>,
    pub opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenEmissionData {
    pub st_emission_rate: f64, // THP per GEL
    pub total_st_emitted: u64,
    pub total_ut_earned: u64,
    pub conversion_pool_size: u64,
    pub reserved_st: u64,
    pub circulating_st: u64,
    pub emission_efficiency: f64, // ST per GEL of costs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub ingredient_costs: f64,
    pub labor_costs: f64,
    pub overhead_costs: f64,
    pub marketing_costs: f64,
    pub technology_costs: f64,
    pub other_costs: f64,
    pub cost_per_dish: f64,
    pub cost_per_token: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueAnalysis {
    pub total_sales: f64,
    pub average_order_value: f64,
    pub sales_by_category: HashMap<String, f64>,
    pub sales_by_location: HashMap<String, f64>,
    pub peak_hours: Vec<String>,
    pub customer_retention_rate: f64,
    pub revenue_per_token: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub cost_efficiency: f64, // Revenue / Costs
    pub token_efficiency: f64, // ST emitted per GEL revenue
    pub operational_efficiency: f64, // Orders per hour
    pub resource_utilization: f64, // Ingredient usage efficiency
    pub waste_percentage: f64,
    pub energy_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyNotification {
    pub notification_id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub priority: NotificationPriority,
    pub recipients: Vec<String>,
    pub status: NotificationStatus,
    pub action_required: bool,
    pub action_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    ReportGenerated,
    CostAnomaly,
    RevenueMilestone,
    TokenEmission,
    EfficiencyAlert,
    ComplianceReminder,
    SystemUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Read,
    Acknowledged,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyConfig {
    pub auto_report_generation: bool,
    pub report_frequency: ReportType,
    pub notification_enabled: bool,
    pub public_access: bool,
    pub data_retention_days: u32,
    pub cost_threshold_alerts: f64,
    pub revenue_threshold_alerts: f64,
    pub efficiency_threshold_alerts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportGenerationRequest {
    pub report_type: ReportType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub include_details: bool,
    pub include_recommendations: bool,
    pub generated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportGenerationResponse {
    pub success: bool,
    pub report_id: Option<String>,
    pub error: Option<String>,
    pub estimated_completion_time: Option<DateTime<Utc>>,
}

impl TransparencyReportingSystem {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
            notifications: Vec::new(),
            config: TransparencyConfig::default(),
        }
    }

    /// Генерация отчета о прозрачности
    pub async fn generate_report(
        &mut self,
        request: ReportGenerationRequest,
        cost_data: &crate::cost_tracking::CostTracker,
        tokenomics_data: &crate::new_tokenomics::NewTokenomicsManager,
        sales_data: &HashMap<String, f64>,
    ) -> ReportGenerationResponse {
        let report_id = Uuid::new_v4().to_string();
        
        // Создаем отчет
        let mut report = TransparencyReport {
            report_id: report_id.clone(),
            report_type: request.report_type.clone(),
            period_start: request.period_start,
            period_end: request.period_end,
            generated_at: Utc::now(),
            generated_by: request.generated_by,
            status: ReportStatus::Generating,
            data: ReportData::default(),
            summary: ReportSummary::default(),
            token_emission: TokenEmissionData::default(),
            cost_breakdown: CostBreakdown::default(),
            revenue_analysis: RevenueAnalysis::default(),
            efficiency_metrics: EfficiencyMetrics::default(),
        };

        // Собираем данные
        match self.collect_report_data(&mut report, cost_data, tokenomics_data, sales_data).await {
            Ok(_) => {
                report.status = ReportStatus::Completed;
                self.reports.insert(report_id.clone(), report);
                
                // Отправляем уведомление
                self.send_notification(TransparencyNotification {
                    notification_id: Uuid::new_v4().to_string(),
                    notification_type: NotificationType::ReportGenerated,
                    title: "Отчет о прозрачности сгенерирован".to_string(),
                    message: format!("Отчет {:?} за период {} - {} готов к просмотру", 
                        request.report_type, request.period_start, request.period_end),
                    created_at: Utc::now(),
                    priority: NotificationPriority::Medium,
                    recipients: vec!["admin".to_string(), "owner".to_string()],
                    status: NotificationStatus::Pending,
                    action_required: false,
                    action_url: Some(format!("/reports/{}", report_id)),
                });

                ReportGenerationResponse {
                    success: true,
                    report_id: Some(report_id),
                    error: None,
                    estimated_completion_time: None,
                }
            }
            Err(e) => {
                ReportGenerationResponse {
                    success: false,
                    report_id: None,
                    error: Some(e.to_string()),
                    estimated_completion_time: None,
                }
            }
        }
    }

    /// Сбор данных для отчета
    async fn collect_report_data(
        &self,
        report: &mut TransparencyReport,
        cost_data: &crate::cost_tracking::CostTracker,
        tokenomics_data: &crate::new_tokenomics::NewTokenomicsManager,
        sales_data: &HashMap<String, f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Mock данные о затратах для демо
        let total_costs = 5000.0;
        let ingredient_costs = 3000.0;
        let labor_costs = 1200.0;
        let overhead_costs = 500.0;
        let marketing_costs = 200.0;
        let technology_costs = 100.0;
        let other_costs = 0.0;
        let average_cost_per_dish = 5.0;
        let cost_per_token_emitted = 0.05;
        report.cost_breakdown = CostBreakdown {
            ingredient_costs,
            labor_costs,
            overhead_costs,
            marketing_costs,
            technology_costs,
            other_costs,
            cost_per_dish: average_cost_per_dish,
            cost_per_token: cost_per_token_emitted,
        };

        // Собираем данные о токеномике (mock для демо)
        let stats = tokenomics_data.get_statistics();
        let stats_data = stats.unwrap_or_else(|_| crate::new_tokenomics::ConversionStats {
            total_st_holders: 100,
            total_ut_holders: 200,
            total_sales: 50,
            total_ut_events: 300,
            total_st_minted: 1000,
            total_ut_awarded: 5000,
            total_rounds: 2,
            reserved_st: 500,
        });
        
        report.token_emission = TokenEmissionData {
            st_emission_rate: 0.2, // THP per GEL
            total_st_emitted: stats_data.total_st_minted as u64,
            total_ut_earned: stats_data.total_ut_awarded as u64,
            conversion_pool_size: stats_data.reserved_st as u64,
            reserved_st: stats_data.reserved_st as u64,
            circulating_st: (stats_data.total_st_minted - stats_data.reserved_st) as u64,
            emission_efficiency: stats_data.total_st_minted as f64 / total_costs,
        };

        // Собираем данные о выручке
        let total_revenue: f64 = sales_data.values().sum();
        report.revenue_analysis = RevenueAnalysis {
            total_sales: total_revenue,
            average_order_value: total_revenue / sales_data.len() as f64,
            sales_by_category: HashMap::new(), // TODO: Implement
            sales_by_location: HashMap::new(), // TODO: Implement
            peak_hours: Vec::new(), // TODO: Implement
            customer_retention_rate: 0.0, // TODO: Implement
            revenue_per_token: total_revenue / stats_data.total_st_minted as f64,
        };

        // Основные данные отчета
        report.data = ReportData {
            total_revenue,
            total_costs,
            net_profit: total_revenue - total_costs,
            profit_margin: (total_revenue - total_costs) / total_revenue,
            st_tokens_emitted: stats_data.total_st_minted as u64,
            ut_tokens_earned: stats_data.total_ut_awarded as u64,
            conversion_rounds: stats_data.total_rounds as u32,
            dao_proposals: 5, // Mock
            active_users: stats_data.total_st_holders + stats_data.total_ut_holders,
            new_registrations: 25, // Mock
        };

        // Метрики эффективности
        report.efficiency_metrics = EfficiencyMetrics {
            cost_efficiency: total_revenue / total_costs,
            token_efficiency: stats_data.total_st_minted as f64 / total_revenue,
            operational_efficiency: 0.0, // TODO: Implement
            resource_utilization: 0.0, // TODO: Implement
            waste_percentage: 0.0, // TODO: Implement
            energy_efficiency: 0.0, // TODO: Implement
        };

        // Генерируем сводку
        report.summary = self.generate_report_summary(report);

        Ok(())
    }

    /// Генерация сводки отчета
    fn generate_report_summary(&self, report: &TransparencyReport) -> ReportSummary {
        let mut highlights = Vec::new();
        let mut recommendations = Vec::new();
        let mut risks = Vec::new();
        let mut opportunities = Vec::new();

        // Ключевые моменты
        if report.data.profit_margin > 0.2 {
            highlights.push(format!("Высокая рентабельность: {:.1}%", report.data.profit_margin * 100.0));
        }
        if report.efficiency_metrics.cost_efficiency > 1.5 {
            highlights.push(format!("Эффективное управление затратами: {:.2}x", report.efficiency_metrics.cost_efficiency));
        }
        if report.data.st_tokens_emitted > 1000 {
            highlights.push(format!("Активная эмиссия токенов: {} ST", report.data.st_tokens_emitted));
        }

        // Рекомендации
        if report.cost_breakdown.ingredient_costs > report.data.total_revenue * 0.4 {
            recommendations.push("Оптимизировать закупку ингредиентов для снижения затрат");
        }
        if report.efficiency_metrics.token_efficiency < 0.15 {
            recommendations.push("Увеличить эффективность эмиссии токенов");
        }
        if report.data.new_registrations < 50 {
            recommendations.push("Улучшить программы лояльности для повышения удержания клиентов");
        }

        // Риски
        if report.data.profit_margin < 0.1 {
            risks.push("Низкая рентабельность может привести к финансовым проблемам");
        }
        if report.cost_breakdown.cost_per_token > 0.1 {
            risks.push("Высокая стоимость эмиссии токенов");
        }

        // Возможности
        if report.data.new_registrations > 100 {
            opportunities.push("Высокий рост пользовательской базы");
        }
        if report.token_emission.conversion_pool_size > 10000 {
            opportunities.push("Большой пул конвертации для UT держателей");
        }

        ReportSummary {
            key_highlights: highlights,
            recommendations: recommendations.into_iter().map(|s| s.to_string()).collect(),
            risks: risks.into_iter().map(|s| s.to_string()).collect(),
            opportunities: opportunities.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Отправка уведомления
    pub fn send_notification(&mut self, notification: TransparencyNotification) {
        self.notifications.push(notification);
    }

    /// Получение отчета по ID
    pub fn get_report(&self, report_id: &str) -> Option<&TransparencyReport> {
        self.reports.get(report_id)
    }

    /// Получение всех отчетов
    pub fn get_all_reports(&self) -> Vec<&TransparencyReport> {
        self.reports.values().collect()
    }

    /// Получение уведомлений
    pub fn get_notifications(&self, limit: Option<usize>) -> Vec<&TransparencyNotification> {
        let mut notifications: Vec<&TransparencyNotification> = self.notifications.iter().collect();
        notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        if let Some(limit) = limit {
            notifications.truncate(limit);
        }
        
        notifications
    }

    /// Публикация отчета
    pub fn publish_report(&mut self, report_id: &str) -> Result<(), String> {
        if let Some(report) = self.reports.get_mut(report_id) {
            report.status = ReportStatus::Published;
            Ok(())
        } else {
            Err("Отчет не найден".to_string())
        }
    }

    /// Экспорт отчета в JSON
    pub fn export_report_json(&self, report_id: &str) -> Result<String, String> {
        if let Some(report) = self.reports.get(report_id) {
            serde_json::to_string_pretty(report)
                .map_err(|e| format!("Ошибка сериализации: {}", e))
        } else {
            Err("Отчет не найден".to_string())
        }
    }

    /// Экспорт отчета в CSV
    pub fn export_report_csv(&self, report_id: &str) -> Result<String, String> {
        if let Some(report) = self.reports.get(report_id) {
            let mut csv = String::new();
            csv.push_str("Показатель,Значение\n");
            csv.push_str(&format!("Общая выручка,{:.2}\n", report.data.total_revenue));
            csv.push_str(&format!("Общие затраты,{:.2}\n", report.data.total_costs));
            csv.push_str(&format!("Чистая прибыль,{:.2}\n", report.data.net_profit));
            csv.push_str(&format!("Рентабельность,{:.2}%\n", report.data.profit_margin * 100.0));
            csv.push_str(&format!("ST токены эмитированы,{}\n", report.data.st_tokens_emitted));
            csv.push_str(&format!("UT токены заработаны,{}\n", report.data.ut_tokens_earned));
            csv.push_str(&format!("Эффективность затрат,{:.2}\n", report.efficiency_metrics.cost_efficiency));
            csv.push_str(&format!("Эффективность токенов,{:.2}\n", report.efficiency_metrics.token_efficiency));
            Ok(csv)
        } else {
            Err("Отчет не найден".to_string())
        }
    }
}

impl Default for TransparencyConfig {
    fn default() -> Self {
        Self {
            auto_report_generation: true,
            report_frequency: ReportType::Monthly,
            notification_enabled: true,
            public_access: false,
            data_retention_days: 365,
            cost_threshold_alerts: 1000.0,
            revenue_threshold_alerts: 5000.0,
            efficiency_threshold_alerts: 0.8,
        }
    }
}

impl Default for ReportData {
    fn default() -> Self {
        Self {
            total_revenue: 0.0,
            total_costs: 0.0,
            net_profit: 0.0,
            profit_margin: 0.0,
            st_tokens_emitted: 0,
            ut_tokens_earned: 0,
            conversion_rounds: 0,
            dao_proposals: 0,
            active_users: 0,
            new_registrations: 0,
        }
    }
}

impl Default for ReportSummary {
    fn default() -> Self {
        Self {
            key_highlights: Vec::new(),
            recommendations: Vec::new(),
            risks: Vec::new(),
            opportunities: Vec::new(),
        }
    }
}

impl Default for TokenEmissionData {
    fn default() -> Self {
        Self {
            st_emission_rate: 0.2,
            total_st_emitted: 0,
            total_ut_earned: 0,
            conversion_pool_size: 0,
            reserved_st: 0,
            circulating_st: 0,
            emission_efficiency: 0.0,
        }
    }
}

impl Default for CostBreakdown {
    fn default() -> Self {
        Self {
            ingredient_costs: 0.0,
            labor_costs: 0.0,
            overhead_costs: 0.0,
            marketing_costs: 0.0,
            technology_costs: 0.0,
            other_costs: 0.0,
            cost_per_dish: 0.0,
            cost_per_token: 0.0,
        }
    }
}

impl Default for RevenueAnalysis {
    fn default() -> Self {
        Self {
            total_sales: 0.0,
            average_order_value: 0.0,
            sales_by_category: HashMap::new(),
            sales_by_location: HashMap::new(),
            peak_hours: Vec::new(),
            customer_retention_rate: 0.0,
            revenue_per_token: 0.0,
        }
    }
}

impl Default for EfficiencyMetrics {
    fn default() -> Self {
        Self {
            cost_efficiency: 0.0,
            token_efficiency: 0.0,
            operational_efficiency: 0.0,
            resource_utilization: 0.0,
            waste_percentage: 0.0,
            energy_efficiency: 0.0,
        }
    }
}
