use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Конфигурация стриминга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub twitch: TwitchConfig,
    pub youtube: YouTubeConfig,
    pub default_quality: StreamQuality,
    pub max_concurrent_streams: u32,
}

/// Конфигурация Twitch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub channel_name: String,
    pub stream_key: String,
    pub webhook_secret: String,
}

/// Конфигурация YouTube
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub channel_id: String,
    pub stream_key: String,
    pub api_key: String,
}

/// Качество стрима
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamQuality {
    Low,    // 480p, 1.5 Mbps
    Medium, // 720p, 3 Mbps
    High,   // 1080p, 6 Mbps
}

impl StreamQuality {
    pub fn get_bitrate(&self) -> u32 {
        match self {
            StreamQuality::Low => 1500,
            StreamQuality::Medium => 3000,
            StreamQuality::High => 6000,
        }
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        match self {
            StreamQuality::Low => (854, 480),
            StreamQuality::Medium => (1280, 720),
            StreamQuality::High => (1920, 1080),
        }
    }
}

/// Расширенная конфигурация стриминга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStreamingConfig {
    pub twitch: TwitchConfig,
    pub youtube: YouTubeConfig,
    pub facebook: FacebookConfig,
    pub instagram: InstagramConfig,
    pub default_quality: StreamQuality,
    pub max_concurrent_streams: u32,
    pub auto_switch_cameras: bool,
    pub face_blurring_enabled: bool,
    pub overlay_settings: OverlaySettings,
}

/// Конфигурация Facebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacebookConfig {
    pub app_id: String,
    pub app_secret: String,
    pub access_token: String,
    pub page_id: String,
    pub stream_key: String,
}

/// Конфигурация Instagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstagramConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub user_id: String,
    pub stream_key: String,
}

/// Настройки оверлея
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlaySettings {
    pub show_chef_name: bool,
    pub show_customer_name: bool,
    pub show_timestamp: bool,
    pub show_logo: bool,
    pub logo_position: OverlayPosition,
    pub text_color: String,
    pub background_color: String,
    pub font_size: u32,
}

/// Позиция оверлея
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// Активная трансляция
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveStream {
    pub stream_id: String,
    pub stream_type: StreamType,
    pub platforms: Vec<StreamingPlatform>,
    pub quality: StreamQuality,
    pub layout: StreamLayout,
    pub active_cameras: Vec<String>,
    pub start_time: u64,
    pub is_live: bool,
    pub viewer_count: u32,
    pub chef_overlay: Option<ChefOverlay>,
    pub customer_overlays: Vec<CustomerOverlay>,
}

/// Тип трансляции
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamType {
    Kitchen,
    Customer,
    Security,
    Mixed,
}

/// Платформы для стриминга
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StreamingPlatform {
    Twitch,
    YouTube,
    Facebook,
    Instagram,
}

/// Макет трансляции
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamLayout {
    Single,
    SideBySide,
    Grid2x2,
    Grid3x3,
    PictureInPicture,
    Custom(Vec<StreamRegion>),
}

/// Область в макете трансляции
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamRegion {
    pub camera_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub z_index: u32,
}

/// Оверлей повара
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChefOverlay {
    pub chef_id: String,
    pub chef_name: String,
    pub camera_id: String,
    pub position: OverlayPosition,
    pub is_active: bool,
}

/// Оверлей посетителя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerOverlay {
    pub customer_id: String,
    pub customer_name: String,
    pub table_id: String,
    pub camera_id: String,
    pub position: OverlayPosition,
    pub is_active: bool,
}

/// Статистика трансляции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStatistics {
    pub stream_id: String,
    pub total_viewers: u32,
    pub peak_viewers: u32,
    pub average_viewers: u32,
    pub total_duration: u64,
    pub platform_stats: HashMap<StreamingPlatform, PlatformStats>,
}

/// Статистика платформы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStats {
    pub platform: StreamingPlatform,
    pub viewers: u32,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub engagement_rate: f32,
}

/// Статус стрима
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamStatus {
    Idle,
    Starting,
    Live,
    Ending,
    Error,
}

/// Информация о стриме
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub stream_id: String,
    pub camera_id: String,
    pub platform: StreamingPlatform,
    pub status: StreamStatus,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub viewer_count: u32,
    pub quality: StreamQuality,
    pub stream_url: Option<String>,
    pub chat_url: Option<String>,
    pub error_message: Option<String>,
}


/// Менеджер стриминга
pub struct StreamingManager {
    config: StreamingConfig,
    active_streams: Arc<RwLock<HashMap<String, StreamInfo>>>,
    twitch_client: TwitchClient,
    youtube_client: YouTubeClient,
}

impl StreamingManager {
    pub fn new(config: StreamingConfig) -> Self {
        let twitch_client = TwitchClient::new(config.twitch.clone());
        let youtube_client = YouTubeClient::new(config.youtube.clone());
        
        Self {
            config,
            active_streams: Arc::new(RwLock::new(HashMap::new())),
            twitch_client,
            youtube_client,
        }
    }

    /// Начать стрим
    pub async fn start_stream(
        &self,
        camera_id: String,
        platform: StreamingPlatform,
        quality: Option<StreamQuality>,
    ) -> Result<String, String> {
        let stream_id = format!("STREAM_{}_{}", camera_id, 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs());

        let quality = quality.unwrap_or(self.config.default_quality.clone());

        // Проверяем лимит одновременных стримов
        let active_count = self.get_active_stream_count().await;
        if active_count >= self.config.max_concurrent_streams {
            return Err("Maximum concurrent streams reached".to_string());
        }

        let mut stream_info = StreamInfo {
            stream_id: stream_id.clone(),
            camera_id: camera_id.clone(),
            platform: platform.clone(),
            status: StreamStatus::Starting,
            start_time: None,
            end_time: None,
            viewer_count: 0,
            quality: quality.clone(),
            stream_url: None,
            chat_url: None,
            error_message: None,
        };

        // Запускаем стрим на выбранных платформах
        match platform {
            StreamingPlatform::Twitch => {
                match self.twitch_client.start_stream(&stream_id, &quality).await {
                    Ok(twitch_info) => {
                        stream_info.stream_url = Some(twitch_info.stream_url);
                        stream_info.chat_url = Some(twitch_info.chat_url);
                        stream_info.status = StreamStatus::Live;
                        stream_info.start_time = Some(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );
                    },
                    Err(e) => {
                        stream_info.status = StreamStatus::Error;
                        stream_info.error_message = Some(e);
                    }
                }
            },
            StreamingPlatform::YouTube => {
                match self.youtube_client.start_stream(&stream_id, &quality).await {
                    Ok(youtube_info) => {
                        stream_info.stream_url = Some(youtube_info.stream_url);
                        stream_info.chat_url = Some(youtube_info.chat_url);
                        stream_info.status = StreamStatus::Live;
                        stream_info.start_time = Some(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        );
                    },
                    Err(e) => {
                        stream_info.status = StreamStatus::Error;
                        stream_info.error_message = Some(e);
                    }
                }
            },
            StreamingPlatform::Facebook => {
                // Логика для Facebook (заглушка)
                stream_info.status = StreamStatus::Error;
                stream_info.error_message = Some("Facebook streaming not implemented yet".to_string());
            },
            StreamingPlatform::Instagram => {
                // Логика для Instagram (заглушка)
                stream_info.status = StreamStatus::Error;
                stream_info.error_message = Some("Instagram streaming not implemented yet".to_string());
            }
        }

        // Сохраняем информацию о стриме
        let mut active_streams = self.active_streams.write().await;
        active_streams.insert(stream_id.clone(), stream_info);

        Ok(stream_id)
    }

    /// Остановить стрим
    pub async fn stop_stream(&self, stream_id: String) -> Result<(), String> {
        let mut active_streams = self.active_streams.write().await;
        
        if let Some(stream_info) = active_streams.get_mut(&stream_id) {
            stream_info.status = StreamStatus::Ending;
            stream_info.end_time = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );

            // Останавливаем стрим на платформах
            match stream_info.platform {
                StreamingPlatform::Twitch => {
                    if let Err(e) = self.twitch_client.stop_stream(&stream_id).await {
                        eprintln!("Error stopping Twitch stream: {}", e);
                    }
                },
                StreamingPlatform::YouTube => {
                    if let Err(e) = self.youtube_client.stop_stream(&stream_id).await {
                        eprintln!("Error stopping YouTube stream: {}", e);
                    }
                },
                StreamingPlatform::Facebook => {
                    // Facebook streaming not implemented yet
                    eprintln!("Facebook streaming not implemented yet");
                },
                StreamingPlatform::Instagram => {
                    // Instagram streaming not implemented yet
                    eprintln!("Instagram streaming not implemented yet");
                }
            }

            stream_info.status = StreamStatus::Idle;
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Получить информацию о стриме
    pub async fn get_stream_info(&self, stream_id: String) -> Option<StreamInfo> {
        let active_streams = self.active_streams.read().await;
        active_streams.get(&stream_id).cloned()
    }

    /// Получить все активные стримы
    pub async fn get_active_streams(&self) -> Vec<StreamInfo> {
        let active_streams = self.active_streams.read().await;
        active_streams.values()
            .filter(|s| s.status == StreamStatus::Live)
            .cloned()
            .collect()
    }

    /// Получить количество активных стримов
    async fn get_active_stream_count(&self) -> u32 {
        let active_streams = self.active_streams.read().await;
        active_streams.values()
            .filter(|s| s.status == StreamStatus::Live)
            .count() as u32
    }

    /// Обновить статистику стрима
    pub async fn update_stream_stats(&self, stream_id: String, viewer_count: u32) -> Result<(), String> {
        let mut active_streams = self.active_streams.write().await;
        
        if let Some(stream_info) = active_streams.get_mut(&stream_id) {
            stream_info.viewer_count = viewer_count;
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Получить статистику по платформам
    pub async fn get_platform_stats(&self) -> HashMap<StreamingPlatform, PlatformStats> {
        let active_streams = self.active_streams.read().await;
        let mut stats = HashMap::new();

        let mut twitch_streams = 0;
        let mut youtube_streams = 0;
        let mut total_viewers = 0;

        for stream in active_streams.values() {
            if stream.status == StreamStatus::Live {
                match stream.platform {
                    StreamingPlatform::Twitch => {
                        twitch_streams += 1;
                        total_viewers += stream.viewer_count;
                    },
                    StreamingPlatform::YouTube => {
                        youtube_streams += 1;
                        total_viewers += stream.viewer_count;
                    },
                    StreamingPlatform::Facebook => {
                        // Facebook streaming not implemented yet
                    },
                    StreamingPlatform::Instagram => {
                        // Instagram streaming not implemented yet
                    }
                }
            }
        }

        stats.insert(StreamingPlatform::Twitch, PlatformStats {
            platform: StreamingPlatform::Twitch,
            viewers: total_viewers / 2, // Примерное распределение
            likes: 0,
            comments: 0,
            shares: 0,
            engagement_rate: 0.0,
        });

        stats.insert(StreamingPlatform::YouTube, PlatformStats {
            platform: StreamingPlatform::YouTube,
            viewers: total_viewers / 2,
            likes: 0,
            comments: 0,
            shares: 0,
            engagement_rate: 0.0,
        });

        stats
    }
}


/// Клиент для работы с Twitch API
pub struct TwitchClient {
    config: TwitchConfig,
}

impl TwitchClient {
    pub fn new(config: TwitchConfig) -> Self {
        Self { config }
    }

    pub async fn start_stream(
        &self,
        _stream_id: &str,
        _quality: &StreamQuality,
    ) -> Result<TwitchStreamInfo, String> {
        // Здесь должна быть реальная интеграция с Twitch API
        // Пока что возвращаем заглушку
        
        let stream_url = format!("rtmp://live.twitch.tv/live/{}", self.config.stream_key);
        let chat_url = format!("https://www.twitch.tv/{}", self.config.channel_name);

        Ok(TwitchStreamInfo {
            stream_url,
            chat_url,
            stream_key: self.config.stream_key.clone(),
        })
    }

    pub async fn stop_stream(&self, stream_id: &str) -> Result<(), String> {
        // Здесь должна быть реальная интеграция с Twitch API
        println!("Stopping Twitch stream: {}", stream_id);
        Ok(())
    }

    pub async fn get_viewer_count(&self, stream_id: &str) -> Result<u32, String> {
        // Здесь должна быть реальная интеграция с Twitch API
        Ok(0)
    }
}

/// Информация о Twitch стриме
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchStreamInfo {
    pub stream_url: String,
    pub chat_url: String,
    pub stream_key: String,
}

/// Клиент для работы с YouTube API
pub struct YouTubeClient {
    config: YouTubeConfig,
}

impl YouTubeClient {
    pub fn new(config: YouTubeConfig) -> Self {
        Self { config }
    }

    pub async fn start_stream(
        &self,
        _stream_id: &str,
        _quality: &StreamQuality,
    ) -> Result<YouTubeStreamInfo, String> {
        // Здесь должна быть реальная интеграция с YouTube API
        // Пока что возвращаем заглушку
        
        let stream_url = format!("rtmp://a.rtmp.youtube.com/live2/{}", self.config.stream_key);
        let chat_url = format!("https://www.youtube.com/channel/{}/live", self.config.channel_id);

        Ok(YouTubeStreamInfo {
            stream_url,
            chat_url,
            stream_key: self.config.stream_key.clone(),
        })
    }

    pub async fn stop_stream(&self, stream_id: &str) -> Result<(), String> {
        // Здесь должна быть реальная интеграция с YouTube API
        println!("Stopping YouTube stream: {}", stream_id);
        Ok(())
    }

    pub async fn get_viewer_count(&self, stream_id: &str) -> Result<u32, String> {
        // Здесь должна быть реальная интеграция с YouTube API
        Ok(0)
    }
}

/// Информация о YouTube стриме
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeStreamInfo {
    pub stream_url: String,
    pub chat_url: String,
    pub stream_key: String,
}

/// Обработчик веб-хуков от платформ стриминга
pub struct StreamingWebhookHandler {
    streaming_manager: Arc<StreamingManager>,
}

impl StreamingWebhookHandler {
    pub fn new(streaming_manager: Arc<StreamingManager>) -> Self {
        Self { streaming_manager }
    }

    /// Обработать веб-хук от Twitch
    pub async fn handle_twitch_webhook(
        &self,
        payload: &str,
        _signature: &str,
    ) -> Result<(), String> {
        // Здесь должна быть валидация подписи и обработка веб-хука
        println!("Received Twitch webhook: {}", payload);
        Ok(())
    }

    /// Обработать веб-хук от YouTube
    pub async fn handle_youtube_webhook(
        &self,
        payload: &str,
        _signature: &str,
    ) -> Result<(), String> {
        // Здесь должна быть валидация подписи и обработка веб-хука
        println!("Received YouTube webhook: {}", payload);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_streaming_manager() {
        let twitch_config = TwitchConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            access_token: "test_access_token".to_string(),
            refresh_token: "test_refresh_token".to_string(),
            channel_name: "test_channel".to_string(),
            stream_key: "test_stream_key".to_string(),
            webhook_secret: "test_webhook_secret".to_string(),
        };

        let youtube_config = YouTubeConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_client_secret".to_string(),
            refresh_token: "test_refresh_token".to_string(),
            channel_id: "test_channel_id".to_string(),
            stream_key: "test_stream_key".to_string(),
            api_key: "test_api_key".to_string(),
        };

        let streaming_config = StreamingConfig {
            twitch: twitch_config,
            youtube: youtube_config,
            default_quality: StreamQuality::Medium,
            max_concurrent_streams: 5,
        };

        let manager = StreamingManager::new(streaming_config);

        // Тест запуска стрима
        let stream_id = manager.start_stream(
            "CAM_001".to_string(),
            StreamingPlatform::Twitch,
            Some(StreamQuality::High),
        ).await.unwrap();

        assert!(!stream_id.is_empty());

        // Тест получения информации о стриме
        let stream_info = manager.get_stream_info(stream_id.clone()).await;
        assert!(stream_info.is_some());

        // Тест остановки стрима
        let result = manager.stop_stream(stream_id).await;
        assert!(result.is_ok());
    }
}
