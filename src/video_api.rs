use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use crate::video_surveillance::{
    VideoSurveillanceSystem, VideoSurveillanceAPI, 
    CameraConfig, CameraType, AnonymizationZone,
    VideoConsent, ActiveRecording, CameraStats
};

/// API запросы для системы видеонаблюдения
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VideoAPIRequest {
    /// Получить согласие на видеозапись
    RequestConsent {
        customer_id: String,
        table_id: String,
        language: String,
    },
    /// Подтвердить согласие
    ConfirmConsent {
        customer_id: String,
        anonymization_preference: AnonymizationZone,
    },
    /// Начать запись
    StartRecording {
        camera_id: String,
        customer_id: Option<String>,
        table_id: Option<String>,
    },
    /// Остановить запись
    StopRecording {
        recording_id: String,
    },
    /// Получить активные записи
    GetActiveRecordings,
    /// Получить статистику камер
    GetCameraStats,
    /// Добавить камеру
    AddCamera {
        config: CameraConfig,
    },
    /// Получить конфигурацию камеры
    GetCameraConfig {
        camera_id: String,
    },
    /// Обновить конфигурацию камеры
    UpdateCameraConfig {
        camera_id: String,
        config: CameraConfig,
    },
    /// Удалить камеру
    RemoveCamera {
        camera_id: String,
    },
    /// Получить список камер
    GetCameras,
    /// Получить историю согласий
    GetConsentHistory {
        customer_id: String,
    },
    /// Отозвать согласие
    RevokeConsent {
        customer_id: String,
        recording_id: Option<String>,
    },
}

/// API ответы для системы видеонаблюдения
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VideoAPIResponse {
    /// Успешный ответ
    Success {
        data: serde_json::Value,
    },
    /// Ошибка
    Error {
        message: String,
        code: String,
    },
    /// Согласие запрошено
    ConsentRequested {
        consent_id: String,
        consent_text: String,
        max_duration_minutes: u32,
        anonymization_options: Vec<AnonymizationZone>,
    },
    /// Запись начата
    RecordingStarted {
        recording_id: String,
        camera_id: String,
        estimated_end_time: u64,
    },
    /// Запись остановлена
    RecordingStopped {
        recording_id: String,
        duration_seconds: u64,
    },
    /// Активные записи
    ActiveRecordings {
        recordings: Vec<ActiveRecording>,
    },
    /// Статистика камер
    CameraStats {
        stats: HashMap<String, CameraStats>,
    },
    /// Конфигурация камеры
    CameraConfig {
        config: CameraConfig,
    },
    /// Список камер
    Cameras {
        cameras: Vec<CameraConfig>,
    },
    /// История согласий
    ConsentHistory {
        consents: Vec<VideoConsent>,
    },
}

/// Обработчик API для системы видеонаблюдения
pub struct VideoAPIHandler {
    api: Arc<VideoSurveillanceAPI>,
    system: Arc<VideoSurveillanceSystem>,
}

impl VideoAPIHandler {
    pub fn new(system: Arc<VideoSurveillanceSystem>) -> Self {
        let api = Arc::new(VideoSurveillanceAPI::new(system.clone()));
        Self { api, system }
    }

    /// Обработать API запрос
    pub async fn handle_request(&self, request: VideoAPIRequest) -> VideoAPIResponse {
        match request {
            VideoAPIRequest::RequestConsent { customer_id, table_id, language } => {
                self.handle_request_consent(customer_id, table_id, language).await
            },
            VideoAPIRequest::ConfirmConsent { customer_id, anonymization_preference } => {
                self.handle_confirm_consent(customer_id, anonymization_preference).await
            },
            VideoAPIRequest::StartRecording { camera_id, customer_id, table_id } => {
                self.handle_start_recording(camera_id, customer_id, table_id).await
            },
            VideoAPIRequest::StopRecording { recording_id } => {
                self.handle_stop_recording(recording_id).await
            },
            VideoAPIRequest::GetActiveRecordings => {
                self.handle_get_active_recordings().await
            },
            VideoAPIRequest::GetCameraStats => {
                self.handle_get_camera_stats().await
            },
            VideoAPIRequest::AddCamera { config } => {
                self.handle_add_camera(config).await
            },
            VideoAPIRequest::GetCameraConfig { camera_id } => {
                self.handle_get_camera_config(camera_id).await
            },
            VideoAPIRequest::UpdateCameraConfig { camera_id, config } => {
                self.handle_update_camera_config(camera_id, config).await
            },
            VideoAPIRequest::RemoveCamera { camera_id } => {
                self.handle_remove_camera(camera_id).await
            },
            VideoAPIRequest::GetCameras => {
                self.handle_get_cameras().await
            },
            VideoAPIRequest::GetConsentHistory { customer_id } => {
                self.handle_get_consent_history(customer_id).await
            },
            VideoAPIRequest::RevokeConsent { customer_id, recording_id } => {
                self.handle_revoke_consent(customer_id, recording_id).await
            },
        }
    }

    async fn handle_request_consent(
        &self,
        customer_id: String,
        table_id: String,
        language: String,
    ) -> VideoAPIResponse {
        match self.api.request_consent(
            customer_id,
            table_id,
            "127.0.0.1".to_string(), // TODO: Получать реальный IP
            "Web Browser".to_string(),
        ).await {
            Ok(consent_request) => VideoAPIResponse::ConsentRequested {
                consent_id: consent_request.consent_id,
                consent_text: consent_request.consent_text,
                max_duration_minutes: consent_request.max_duration_minutes,
                anonymization_options: consent_request.anonymization_options,
            },
            Err(e) => VideoAPIResponse::Error {
                message: e,
                code: "CONSENT_REQUEST_FAILED".to_string(),
            },
        }
    }

    async fn handle_confirm_consent(
        &self,
        customer_id: String,
        anonymization_preference: AnonymizationZone,
    ) -> VideoAPIResponse {
        match self.system.confirm_video_consent(customer_id, anonymization_preference).await {
            Ok(_) => VideoAPIResponse::Success {
                data: serde_json::json!({
                    "message": "Consent confirmed successfully"
                }),
            },
            Err(e) => VideoAPIResponse::Error {
                message: e,
                code: "CONSENT_CONFIRMATION_FAILED".to_string(),
            },
        }
    }

    async fn handle_start_recording(
        &self,
        camera_id: String,
        customer_id: Option<String>,
        table_id: Option<String>,
    ) -> VideoAPIResponse {
        match self.system.start_recording(camera_id.clone(), customer_id, table_id).await {
            Ok(recording_id) => {
                // Вычисляем предполагаемое время окончания (30 минут)
                let estimated_end_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 30 * 60;

                VideoAPIResponse::RecordingStarted {
                    recording_id,
                    camera_id,
                    estimated_end_time,
                }
            },
            Err(e) => VideoAPIResponse::Error {
                message: e,
                code: "RECORDING_START_FAILED".to_string(),
            },
        }
    }

    async fn handle_stop_recording(&self, recording_id: String) -> VideoAPIResponse {
        match self.system.stop_recording(recording_id.clone()).await {
            Ok(_) => {
                // Получаем информацию о записи для вычисления длительности
                let active_recordings = self.system.get_active_recordings().await;
                let duration = if let Some(recording) = active_recordings.iter()
                    .find(|r| r.recording_id == recording_id) {
                    if let Some(end_time) = recording.end_time {
                        end_time - recording.start_time
                    } else {
                        0
                    }
                } else {
                    0
                };

                VideoAPIResponse::RecordingStopped {
                    recording_id,
                    duration_seconds: duration,
                }
            },
            Err(e) => VideoAPIResponse::Error {
                message: e,
                code: "RECORDING_STOP_FAILED".to_string(),
            },
        }
    }

    async fn handle_get_active_recordings(&self) -> VideoAPIResponse {
        let recordings = self.system.get_active_recordings().await;
        VideoAPIResponse::ActiveRecordings { recordings }
    }

    async fn handle_get_camera_stats(&self) -> VideoAPIResponse {
        let stats = self.system.get_camera_statistics().await;
        VideoAPIResponse::CameraStats { stats }
    }

    async fn handle_add_camera(&self, config: CameraConfig) -> VideoAPIResponse {
        match self.system.add_camera(config.clone()).await {
            Ok(_) => VideoAPIResponse::Success {
                data: serde_json::json!({
                    "message": "Camera added successfully",
                    "camera_id": config.camera_id
                }),
            },
            Err(e) => VideoAPIResponse::Error {
                message: e,
                code: "CAMERA_ADD_FAILED".to_string(),
            },
        }
    }

    async fn handle_get_camera_config(&self, camera_id: String) -> VideoAPIResponse {
        // TODO: Реализовать получение конфигурации камеры
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }

    async fn handle_update_camera_config(
        &self,
        camera_id: String,
        config: CameraConfig,
    ) -> VideoAPIResponse {
        // TODO: Реализовать обновление конфигурации камеры
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }

    async fn handle_remove_camera(&self, camera_id: String) -> VideoAPIResponse {
        // TODO: Реализовать удаление камеры
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }

    async fn handle_get_cameras(&self) -> VideoAPIResponse {
        // TODO: Реализовать получение списка камер
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }

    async fn handle_get_consent_history(&self, customer_id: String) -> VideoAPIResponse {
        // TODO: Реализовать получение истории согласий
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }

    async fn handle_revoke_consent(
        &self,
        customer_id: String,
        recording_id: Option<String>,
    ) -> VideoAPIResponse {
        // TODO: Реализовать отзыв согласия
        VideoAPIResponse::Error {
            message: "Not implemented".to_string(),
            code: "NOT_IMPLEMENTED".to_string(),
        }
    }
}

/// HTTP обработчик для API видеонаблюдения
pub struct VideoHTTPHandler {
    api_handler: Arc<VideoAPIHandler>,
}

impl VideoHTTPHandler {
    pub fn new(api_handler: Arc<VideoAPIHandler>) -> Self {
        Self { api_handler }
    }

    /// Обработать HTTP запрос
    pub async fn handle_http_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, String> {
        match (method, path) {
            ("POST", "/api/video-consent") => {
                self.handle_consent_request(body).await
            },
            ("POST", "/api/video-consent/confirm") => {
                self.handle_consent_confirm(body).await
            },
            ("POST", "/api/video-recording/start") => {
                self.handle_recording_start(body).await
            },
            ("POST", "/api/video-recording/stop") => {
                self.handle_recording_stop(body).await
            },
            ("GET", "/api/video-recording/active") => {
                self.handle_get_active_recordings().await
            },
            ("GET", "/api/video-cameras/stats") => {
                self.handle_get_camera_stats().await
            },
            ("POST", "/api/video-cameras") => {
                self.handle_add_camera(body).await
            },
            _ => Err("Not found".to_string()),
        }
    }

    async fn handle_consent_request(&self, body: Option<&str>) -> Result<String, String> {
        let request_data: serde_json::Value = serde_json::from_str(
            body.ok_or("Missing request body")?
        ).map_err(|e| format!("Invalid JSON: {}", e))?;

        let customer_id = request_data["customer_id"]
            .as_str()
            .ok_or("Missing customer_id")?
            .to_string();
        
        let table_id = request_data["table_id"]
            .as_str()
            .ok_or("Missing table_id")?
            .to_string();
        
        let language = request_data["language"]
            .as_str()
            .unwrap_or("ru")
            .to_string();

        let request = VideoAPIRequest::RequestConsent {
            customer_id,
            table_id,
            language,
        };

        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_consent_confirm(&self, body: Option<&str>) -> Result<String, String> {
        let request_data: serde_json::Value = serde_json::from_str(
            body.ok_or("Missing request body")?
        ).map_err(|e| format!("Invalid JSON: {}", e))?;

        let customer_id = request_data["customer_id"]
            .as_str()
            .ok_or("Missing customer_id")?
            .to_string();
        
        let anonymization_preference = match request_data["anonymization_preference"]
            .as_str()
            .unwrap_or("replace") {
            "blur" => AnonymizationZone::FullFaceBlur,
            "replace" => AnonymizationZone::FaceReplacement,
            "none" => AnonymizationZone::NoAnonymization,
            _ => AnonymizationZone::FaceReplacement,
        };

        let request = VideoAPIRequest::ConfirmConsent {
            customer_id,
            anonymization_preference,
        };

        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_recording_start(&self, body: Option<&str>) -> Result<String, String> {
        let request_data: serde_json::Value = serde_json::from_str(
            body.ok_or("Missing request body")?
        ).map_err(|e| format!("Invalid JSON: {}", e))?;

        let camera_id = request_data["camera_id"]
            .as_str()
            .ok_or("Missing camera_id")?
            .to_string();
        
        let customer_id = request_data["customer_id"]
            .as_str()
            .map(|s| s.to_string());
        
        let table_id = request_data["table_id"]
            .as_str()
            .map(|s| s.to_string());

        let request = VideoAPIRequest::StartRecording {
            camera_id,
            customer_id,
            table_id,
        };

        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_recording_stop(&self, body: Option<&str>) -> Result<String, String> {
        let request_data: serde_json::Value = serde_json::from_str(
            body.ok_or("Missing request body")?
        ).map_err(|e| format!("Invalid JSON: {}", e))?;

        let recording_id = request_data["recording_id"]
            .as_str()
            .ok_or("Missing recording_id")?
            .to_string();

        let request = VideoAPIRequest::StopRecording { recording_id };
        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_get_active_recordings(&self) -> Result<String, String> {
        let request = VideoAPIRequest::GetActiveRecordings;
        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_get_camera_stats(&self) -> Result<String, String> {
        let request = VideoAPIRequest::GetCameraStats;
        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }

    async fn handle_add_camera(&self, body: Option<&str>) -> Result<String, String> {
        let config: CameraConfig = serde_json::from_str(
            body.ok_or("Missing request body")?
        ).map_err(|e| format!("Invalid JSON: {}", e))?;

        let request = VideoAPIRequest::AddCamera { config };
        let response = self.api_handler.handle_request(request).await;
        Ok(serde_json::to_string(&response).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::video_surveillance::{StreamingConfig, StreamQuality};

    #[tokio::test]
    async fn test_video_api_handler() {
        let streaming_config = StreamingConfig {
            twitch_client_id: "test".to_string(),
            twitch_client_secret: "test".to_string(),
            twitch_access_token: "test".to_string(),
            youtube_client_id: "test".to_string(),
            youtube_client_secret: "test".to_string(),
            youtube_refresh_token: "test".to_string(),
            stream_quality: StreamQuality::Medium,
            default_anonymization: AnonymizationZone::FaceReplacement,
        };

        let system = Arc::new(VideoSurveillanceSystem::new(streaming_config));
        let api_handler = Arc::new(VideoAPIHandler::new(system));
        let http_handler = VideoHTTPHandler::new(api_handler);

        // Тест запроса согласия
        let consent_request = r#"{
            "customer_id": "CUSTOMER_001",
            "table_id": "TABLE_001",
            "language": "ru"
        }"#;

        let result = http_handler.handle_http_request(
            "POST",
            "/api/video-consent",
            Some(consent_request),
        ).await;

        assert!(result.is_ok());
    }
}
