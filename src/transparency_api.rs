use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::transparency_reporting::{
    TransparencyReportingSystem, ReportGenerationRequest, ReportGenerationResponse,
    TransparencyReport, TransparencyNotification, ReportType,
};

/// API сервер для системы прозрачности и отчетности
pub struct TransparencyAPIServer {
    pub transparency_system: Arc<RwLock<TransparencyReportingSystem>>,
    pub cost_tracker: Arc<RwLock<crate::cost_tracking::CostTracker>>,
    pub tokenomics_manager: Arc<RwLock<crate::new_tokenomics::NewTokenomicsManager>>,
}

impl TransparencyAPIServer {
    pub fn new(
        transparency_system: Arc<RwLock<TransparencyReportingSystem>>,
        cost_tracker: Arc<RwLock<crate::cost_tracking::CostTracker>>,
        tokenomics_manager: Arc<RwLock<crate::new_tokenomics::NewTokenomicsManager>>,
    ) -> Self {
        Self {
            transparency_system,
            cost_tracker,
            tokenomics_manager,
        }
    }

    pub fn create_router(self) -> Router {
        Router::new()
            .route("/reports", post(Self::generate_report))
            .route("/reports", get(Self::get_all_reports))
            .route("/reports/:report_id", get(Self::get_report))
            .route("/reports/:report_id/publish", post(Self::publish_report))
            .route("/reports/:report_id/export/json", get(Self::get_report))
            .route("/reports/:report_id/export/csv", get(Self::export_report_csv))
            .route("/notifications", get(Self::get_notifications))
            .route("/notifications/:notification_id/acknowledge", post(Self::acknowledge_notification))
            .route("/dashboard", get(Self::get_dashboard_data))
            .route("/public/reports", get(Self::get_public_reports))
            .with_state(Arc::new(self))
    }

    /// Генерация отчета
    async fn generate_report(
        State(server): State<Arc<Self>>,
        Json(request): Json<ReportGenerationRequest>,
    ) -> Result<Json<ReportGenerationResponse>, StatusCode> {
        let mut transparency_system = server.transparency_system.write().await;
        let cost_tracker = server.cost_tracker.read().await;
        let tokenomics_manager = server.tokenomics_manager.read().await;

        // Mock sales data for demo
        let sales_data = HashMap::from([
            ("plov".to_string(), 1250.0),
            ("khinkali".to_string(), 980.0),
            ("khachapuri".to_string(), 1100.0),
            ("salad".to_string(), 450.0),
            ("tea".to_string(), 200.0),
        ]);

        let response = transparency_system
            .generate_report(request, &cost_tracker, &tokenomics_manager, &sales_data)
            .await;

        Ok(Json(response))
    }

    /// Получение всех отчетов
    async fn get_all_reports(
        State(server): State<Arc<Self>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<Vec<TransparencyReport>>, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        let mut reports: Vec<TransparencyReport> = transparency_system
            .get_all_reports()
            .into_iter()
            .cloned()
            .collect();

        // Фильтрация по типу отчета
        if let Some(report_type) = params.get("type") {
            if let Ok(parsed_type) = serde_json::from_str::<ReportType>(&format!("\"{}\"", report_type)) {
                reports.retain(|r| std::mem::discriminant(&r.report_type) == std::mem::discriminant(&parsed_type));
            }
        }

        // Фильтрация по статусу
        if let Some(status) = params.get("status") {
            reports.retain(|r| format!("{:?}", r.status).to_lowercase() == status.to_lowercase());
        }

        // Сортировка по дате создания
        reports.sort_by(|a, b| b.generated_at.cmp(&a.generated_at));

        Ok(Json(reports))
    }

    /// Получение отчета по ID
    async fn get_report(
        State(server): State<Arc<Self>>,
        Path(report_id): Path<String>,
    ) -> Result<Json<TransparencyReport>, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        
        match transparency_system.get_report(&report_id) {
            Some(report) => Ok(Json(report.clone())),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Публикация отчета
    async fn publish_report(
        State(server): State<Arc<Self>>,
        Path(report_id): Path<String>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        let mut transparency_system = server.transparency_system.write().await;
        
        match transparency_system.publish_report(&report_id) {
            Ok(_) => Ok(Json(serde_json::json!({
                "success": true,
                "message": "Отчет успешно опубликован"
            }))),
            Err(e) => {
                Ok(Json(serde_json::json!({
                    "success": false,
                    "error": e
                })))
            }
        }
    }


    /// Экспорт отчета в CSV
    async fn export_report_csv(
        State(server): State<Arc<Self>>,
        Path(report_id): Path<String>,
    ) -> Result<String, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        
        match transparency_system.export_report_csv(&report_id) {
            Ok(csv_data) => Ok(csv_data),
            Err(e) => {
                Err(StatusCode::NOT_FOUND)
            }
        }
    }

    /// Получение уведомлений
    async fn get_notifications(
        State(server): State<Arc<Self>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<Vec<TransparencyNotification>>, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        
        let limit = params.get("limit")
            .and_then(|l| l.parse::<usize>().ok());
        
        let notifications: Vec<TransparencyNotification> = transparency_system
            .get_notifications(limit)
            .into_iter()
            .cloned()
            .collect();

        Ok(Json(notifications))
    }

    /// Подтверждение уведомления
    async fn acknowledge_notification(
        State(server): State<Arc<Self>>,
        Path(notification_id): Path<String>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        let mut transparency_system = server.transparency_system.write().await;
        
        if let Some(notification) = transparency_system.notifications
            .iter_mut()
            .find(|n| n.notification_id == notification_id) {
            notification.status = crate::transparency_reporting::NotificationStatus::Acknowledged;
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Уведомление подтверждено"
            })))
        } else {
            Ok(Json(serde_json::json!({
                "success": false,
                "error": "Уведомление не найдено"
            })))
        }
    }

    /// Получение данных для дашборда
    async fn get_dashboard_data(
        State(server): State<Arc<Self>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        let cost_tracker = server.cost_tracker.read().await;
        let tokenomics_manager = server.tokenomics_manager.read().await;

        // Получаем последние отчеты
        let recent_reports: Vec<&TransparencyReport> = transparency_system
            .get_all_reports()
            .into_iter()
            .take(5)
            .collect();

        // Получаем активные уведомления
        let active_notifications: Vec<&TransparencyNotification> = transparency_system
            .notifications
            .iter()
            .filter(|n| matches!(n.status, crate::transparency_reporting::NotificationStatus::Pending))
            .take(10)
            .collect();

        // Получаем статистику
        let stats = tokenomics_manager.get_statistics();

        let dashboard_data = serde_json::json!({
            "recent_reports": recent_reports,
            "active_notifications": active_notifications,
            "statistics": {
                "total_reports": transparency_system.reports.len(),
                "pending_notifications": active_notifications.len(),
                "total_st_emitted": stats.as_ref().map(|s| s.total_st_minted).unwrap_or(0),
                "total_ut_earned": stats.as_ref().map(|s| s.total_ut_awarded).unwrap_or(0),
                "conversion_rounds": stats.as_ref().map(|s| s.total_rounds).unwrap_or(0),
                "dao_proposals": 5, // Mock
            },
            "config": transparency_system.config,
        });

        Ok(Json(dashboard_data))
    }

    /// Получение публичных отчетов
    async fn get_public_reports(
        State(server): State<Arc<Self>>,
    ) -> Result<Json<Vec<TransparencyReport>>, StatusCode> {
        let transparency_system = server.transparency_system.read().await;
        
        let public_reports: Vec<TransparencyReport> = transparency_system
            .get_all_reports()
            .into_iter()
            .filter(|r| matches!(r.status, crate::transparency_reporting::ReportStatus::Published))
            .cloned()
            .collect();

        Ok(Json(public_reports))
    }
}

/// Структуры для API запросов
#[derive(Debug, Deserialize)]
pub struct ReportFilterParams {
    pub report_type: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ReportListResponse {
    pub reports: Vec<TransparencyReport>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<TransparencyNotification>,
    pub total: usize,
    pub unread_count: usize,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub recent_reports: Vec<TransparencyReport>,
    pub active_notifications: Vec<TransparencyNotification>,
    pub statistics: DashboardStatistics,
    pub config: crate::transparency_reporting::TransparencyConfig,
}

#[derive(Debug, Serialize)]
pub struct DashboardStatistics {
    pub total_reports: usize,
    pub pending_notifications: usize,
    pub total_st_emitted: u64,
    pub total_ut_earned: u64,
    pub conversion_rounds: u32,
    pub dao_proposals: u32,
    pub last_report_date: Option<DateTime<Utc>>,
    pub system_health: SystemHealth,
}

#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub status: String,
    pub uptime: u64,
    pub last_backup: Option<DateTime<Utc>>,
    pub error_rate: f64,
    pub performance_score: f64,
}
