use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStream {
    pub stream_id: String,
    pub stream_type: StreamType,
    pub source: StreamSource,
    pub status: StreamStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: StreamMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    Camera,
    YouTube,
    File,
    ScreenCapture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamSource {
    Camera { device_id: String, resolution: String },
    YouTube { video_id: String, url: String },
    File { file_path: String },
    ScreenCapture { display_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    Stopped,
    Starting,
    Running,
    Paused,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub title: String,
    pub description: Option<String>,
    pub duration: Option<u64>, // в секундах
    pub fps: Option<u32>,
    pub bitrate: Option<u32>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub auto_start: bool,
    pub loop_video: bool,
    pub quality: StreamQuality,
    pub overlay: Option<OverlayConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamQuality {
    Low,    // 480p
    Medium, // 720p
    High,   // 1080p
    Ultra,  // 4K
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub show_timestamp: bool,
    pub show_logo: bool,
    pub logo_path: Option<String>,
    pub text_overlay: Option<String>,
}

pub struct VideoStreamingManager {
    streams: Arc<RwLock<HashMap<String, VideoStream>>>,
    configs: Arc<RwLock<HashMap<String, StreamConfig>>>,
}

impl VideoStreamingManager {
    pub fn new() -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Создать новый видеопоток с камеры
    pub async fn create_camera_stream(
        &self,
        device_id: String,
        resolution: String,
        title: String,
        config: StreamConfig,
    ) -> Result<String, String> {
        let stream_id = Uuid::new_v4().to_string();
        
        let stream = VideoStream {
            stream_id: stream_id.clone(),
            stream_type: StreamType::Camera,
            source: StreamSource::Camera { device_id, resolution },
            status: StreamStatus::Stopped,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: StreamMetadata {
                title,
                description: None,
                duration: None,
                fps: Some(30),
                bitrate: Some(2500),
                resolution: Some("1920x1080".to_string()),
            },
        };

        let mut streams = self.streams.write().await;
        let mut configs = self.configs.write().await;
        
        streams.insert(stream_id.clone(), stream);
        configs.insert(stream_id.clone(), config);
        
        Ok(stream_id)
    }

    /// Создать поток с YouTube видео
    pub async fn create_youtube_stream(
        &self,
        video_id: String,
        url: String,
        title: String,
        config: StreamConfig,
    ) -> Result<String, String> {
        let stream_id = Uuid::new_v4().to_string();
        
        let stream = VideoStream {
            stream_id: stream_id.clone(),
            stream_type: StreamType::YouTube,
            source: StreamSource::YouTube { video_id, url },
            status: StreamStatus::Stopped,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: StreamMetadata {
                title,
                description: None,
                duration: None,
                fps: Some(30),
                bitrate: Some(5000),
                resolution: Some("1920x1080".to_string()),
            },
        };

        let mut streams = self.streams.write().await;
        let mut configs = self.configs.write().await;
        
        streams.insert(stream_id.clone(), stream);
        configs.insert(stream_id.clone(), config);
        
        Ok(stream_id)
    }

    /// Запустить видеопоток
    pub async fn start_stream(&self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.streams.write().await;
        
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.status = StreamStatus::Starting;
            stream.updated_at = Utc::now();
            
            // Здесь будет логика запуска реального потока
            // Для демонстрации просто меняем статус
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            stream.status = StreamStatus::Running;
            stream.updated_at = Utc::now();
            
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Остановить видеопоток
    pub async fn stop_stream(&self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.streams.write().await;
        
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.status = StreamStatus::Stopped;
            stream.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Получить информацию о потоке
    pub async fn get_stream(&self, stream_id: &str) -> Option<VideoStream> {
        let streams = self.streams.read().await;
        streams.get(stream_id).cloned()
    }

    /// Получить все потоки
    pub async fn get_all_streams(&self) -> Vec<VideoStream> {
        let streams = self.streams.read().await;
        streams.values().cloned().collect()
    }

    /// Получить список доступных камер
    pub async fn get_available_cameras(&self) -> Vec<CameraDevice> {
        // Здесь будет реальная логика обнаружения камер
        // Для демонстрации возвращаем моковые данные
        vec![
            CameraDevice {
                device_id: "camera_0".to_string(),
                name: "Встроенная камера".to_string(),
                resolution: "1920x1080".to_string(),
                fps: 30,
            },
            CameraDevice {
                device_id: "camera_1".to_string(),
                name: "USB камера".to_string(),
                resolution: "1280x720".to_string(),
                fps: 30,
            },
        ]
    }

    /// Валидация YouTube URL
    pub fn validate_youtube_url(&self, url: &str) -> Result<String, String> {
        if url.contains("youtube.com/watch?v=") || url.contains("youtu.be/") {
            let video_id = if url.contains("youtube.com/watch?v=") {
                url.split("v=").nth(1)
                    .and_then(|s| s.split('&').next())
                    .ok_or("Invalid YouTube URL")?
            } else {
                url.split("youtu.be/").nth(1)
                    .and_then(|s| s.split('?').next())
                    .ok_or("Invalid YouTube URL")?
            };
            
            Ok(video_id.to_string())
        } else {
            Err("Invalid YouTube URL format".to_string())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraDevice {
    pub device_id: String,
    pub name: String,
    pub resolution: String,
    pub fps: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamRequest {
    pub stream_type: StreamType,
    pub source: StreamSource,
    pub title: String,
    pub config: StreamConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResponse {
    pub stream_id: String,
    pub status: String,
    pub message: String,
}

impl Default for VideoStreamingManager {
    fn default() -> Self {
        Self::new()
    }
}

// Импорт для использования в других модулях
use std::sync::Arc;
