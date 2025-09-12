//! Расширенный менеджер стриминга для The Hot Pot Spot
//! 
//! Обеспечивает управление трансляциями с кухни, камер безопасности и посетителей
//! с поддержкой множественных платформ и продвинутых функций.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::streaming_integration::{
    EnhancedStreamingConfig, ActiveStream, StreamType, StreamingPlatform, 
    StreamLayout, StreamRegion, ChefOverlay, CustomerOverlay, StreamStatistics,
    PlatformStats, StreamQuality, OverlayPosition
};

/// Расширенный менеджер стриминга
pub struct EnhancedStreamingManager {
    config: EnhancedStreamingConfig,
    active_streams: Arc<RwLock<HashMap<String, ActiveStream>>>,
    stream_statistics: Arc<RwLock<HashMap<String, StreamStatistics>>>,
    face_blurring_enabled: Arc<RwLock<bool>>,
    auto_switch_enabled: Arc<RwLock<bool>>,
}

impl EnhancedStreamingManager {
    /// Создать новый менеджер стриминга
    pub fn new(config: EnhancedStreamingConfig) -> Self {
        Self {
            config,
            active_streams: Arc::new(RwLock::new(HashMap::new())),
            stream_statistics: Arc::new(RwLock::new(HashMap::new())),
            face_blurring_enabled: Arc::new(RwLock::new(true)),
            auto_switch_enabled: Arc::new(RwLock::new(false)),
        }
    }

    /// Создать трансляцию кухни
    pub async fn create_kitchen_stream(
        &self,
        stream_id: &str,
        platforms: Vec<StreamingPlatform>,
        quality: StreamQuality,
        layout: StreamLayout,
        active_cameras: Vec<String>,
    ) -> Result<ActiveStream, String> {
        // Проверяем лимит одновременных трансляций
        let streams = self.active_streams.read().await;
        if streams.len() >= self.config.max_concurrent_streams as usize {
            return Err("Maximum concurrent streams reached".to_string());
        }

        let stream = ActiveStream {
            stream_id: stream_id.to_string(),
            stream_type: StreamType::Kitchen,
            platforms,
            quality,
            layout,
            active_cameras,
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            is_live: false,
            viewer_count: 0,
            chef_overlay: None,
            customer_overlays: Vec::new(),
        };

        let mut streams = self.active_streams.write().await;
        streams.insert(stream_id.to_string(), stream.clone());

        // Инициализируем статистику
        let mut stats = self.stream_statistics.write().await;
        stats.insert(stream_id.to_string(), StreamStatistics {
            stream_id: stream_id.to_string(),
            total_viewers: 0,
            peak_viewers: 0,
            average_viewers: 0,
            total_duration: 0,
            platform_stats: HashMap::new(),
        });

        Ok(stream)
    }

    /// Начать трансляцию
    pub async fn start_stream(&self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.is_live = true;
            stream.start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Остановить трансляцию
    pub async fn stop_stream(&self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.is_live = false;
            
            // Обновляем статистику
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - stream.start_time;
            let mut stats = self.stream_statistics.write().await;
            if let Some(stat) = stats.get_mut(stream_id) {
                stat.total_duration = duration;
            }
            
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Добавить оверлей повара
    pub async fn add_chef_overlay(
        &self,
        stream_id: &str,
        chef_id: &str,
        chef_name: &str,
        camera_id: &str,
        position: OverlayPosition,
    ) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            if stream.stream_type != StreamType::Kitchen {
                return Err("Chef overlay can only be added to kitchen streams".to_string());
            }

            let overlay = ChefOverlay {
                chef_id: chef_id.to_string(),
                chef_name: chef_name.to_string(),
                camera_id: camera_id.to_string(),
                position,
                is_active: true,
            };

            stream.chef_overlay = Some(overlay);
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Добавить оверлей посетителя
    pub async fn add_customer_overlay(
        &self,
        stream_id: &str,
        customer_id: &str,
        customer_name: &str,
        table_id: &str,
        camera_id: &str,
        position: OverlayPosition,
    ) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            let overlay = CustomerOverlay {
                customer_id: customer_id.to_string(),
                customer_name: customer_name.to_string(),
                table_id: table_id.to_string(),
                camera_id: camera_id.to_string(),
                position,
                is_active: true,
            };

            stream.customer_overlays.push(overlay);
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Переключить камеру в трансляции
    pub async fn switch_camera(
        &self,
        stream_id: &str,
        camera_id: &str,
        layout: Option<StreamLayout>,
    ) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            // Обновляем активные камеры
            if stream.active_cameras.len() == 1 {
                stream.active_cameras[0] = camera_id.to_string();
            } else {
                // Для многокамерных макетов добавляем камеру
                if !stream.active_cameras.contains(&camera_id.to_string()) {
                    stream.active_cameras.push(camera_id.to_string());
                }
            }

            // Обновляем макет если указан
            if let Some(new_layout) = layout {
                stream.layout = new_layout;
            }

            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Обновить макет трансляции
    pub async fn update_layout(&self, stream_id: &str, layout: StreamLayout) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.layout = layout;
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Получить все активные трансляции
    pub async fn get_active_streams(&self) -> Vec<ActiveStream> {
        let streams = self.active_streams.read().await;
        streams.values().filter(|s| s.is_live).cloned().collect()
    }

    /// Получить трансляцию по ID
    pub async fn get_stream(&self, stream_id: &str) -> Option<ActiveStream> {
        let streams = self.active_streams.read().await;
        streams.get(stream_id).cloned()
    }

    /// Обновить количество зрителей
    pub async fn update_viewer_count(&self, stream_id: &str, viewer_count: u32) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.viewer_count = viewer_count;
            
            // Обновляем статистику
            let mut stats = self.stream_statistics.write().await;
            if let Some(stat) = stats.get_mut(stream_id) {
                stat.total_viewers = viewer_count;
                if viewer_count > stat.peak_viewers {
                    stat.peak_viewers = viewer_count;
                }
            }
            
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Получить статистику трансляции
    pub async fn get_stream_statistics(&self, stream_id: &str) -> Option<StreamStatistics> {
        let stats = self.stream_statistics.read().await;
        stats.get(stream_id).cloned()
    }

    /// Включить/выключить размытие лиц
    pub async fn set_face_blurring(&self, enabled: bool) {
        let mut face_blurring = self.face_blurring_enabled.write().await;
        *face_blurring = enabled;
    }

    /// Проверить, включено ли размытие лиц
    pub async fn is_face_blurring_enabled(&self) -> bool {
        let face_blurring = self.face_blurring_enabled.read().await;
        *face_blurring
    }

    /// Включить/выключить автоматическое переключение камер
    pub async fn set_auto_switch(&self, enabled: bool) {
        let mut auto_switch = self.auto_switch_enabled.write().await;
        *auto_switch = enabled;
    }

    /// Проверить, включено ли автоматическое переключение камер
    pub async fn is_auto_switch_enabled(&self) -> bool {
        let auto_switch = self.auto_switch_enabled.read().await;
        *auto_switch
    }

    /// Создать смешанную трансляцию (кухня + посетители)
    pub async fn create_mixed_stream(
        &self,
        stream_id: &str,
        platforms: Vec<StreamingPlatform>,
        quality: StreamQuality,
        kitchen_cameras: Vec<String>,
        customer_cameras: Vec<String>,
    ) -> Result<ActiveStream, String> {
        let mut active_cameras = kitchen_cameras;
        active_cameras.extend(customer_cameras);

        let layout = if active_cameras.len() <= 4 {
            StreamLayout::Grid2x2
        } else {
            StreamLayout::Grid3x3
        };

        let stream = ActiveStream {
            stream_id: stream_id.to_string(),
            stream_type: StreamType::Mixed,
            platforms,
            quality,
            layout,
            active_cameras,
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            is_live: false,
            viewer_count: 0,
            chef_overlay: None,
            customer_overlays: Vec::new(),
        };

        let mut streams = self.active_streams.write().await;
        streams.insert(stream_id.to_string(), stream.clone());

        Ok(stream)
    }

    /// Удалить трансляцию
    pub async fn remove_stream(&self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if streams.remove(stream_id).is_some() {
            let mut stats = self.stream_statistics.write().await;
            stats.remove(stream_id);
            Ok(())
        } else {
            Err(format!("Stream {} not found", stream_id))
        }
    }

    /// Получить конфигурацию
    pub fn get_config(&self) -> &EnhancedStreamingConfig {
        &self.config
    }

    /// Обновить конфигурацию
    pub async fn update_config(&mut self, config: EnhancedStreamingConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming_integration::{TwitchConfig, YouTubeConfig, FacebookConfig, InstagramConfig, OverlaySettings};

    fn create_test_config() -> EnhancedStreamingConfig {
        EnhancedStreamingConfig {
            twitch: TwitchConfig {
                client_id: "test_client_id".to_string(),
                client_secret: "test_client_secret".to_string(),
                access_token: "test_access_token".to_string(),
                refresh_token: "test_refresh_token".to_string(),
                channel_name: "test_channel".to_string(),
                stream_key: "test_stream_key".to_string(),
                webhook_secret: "test_webhook_secret".to_string(),
            },
            youtube: YouTubeConfig {
                client_id: "test_youtube_client_id".to_string(),
                client_secret: "test_youtube_client_secret".to_string(),
                refresh_token: "test_youtube_refresh_token".to_string(),
                channel_id: "test_channel_id".to_string(),
                stream_key: "test_youtube_stream_key".to_string(),
                api_key: "test_api_key".to_string(),
            },
            facebook: FacebookConfig {
                app_id: "test_app_id".to_string(),
                app_secret: "test_app_secret".to_string(),
                access_token: "test_facebook_access_token".to_string(),
                page_id: "test_page_id".to_string(),
                stream_key: "test_facebook_stream_key".to_string(),
            },
            instagram: InstagramConfig {
                client_id: "test_instagram_client_id".to_string(),
                client_secret: "test_instagram_client_secret".to_string(),
                access_token: "test_instagram_access_token".to_string(),
                user_id: "test_user_id".to_string(),
                stream_key: "test_instagram_stream_key".to_string(),
            },
            default_quality: StreamQuality::Medium,
            max_concurrent_streams: 5,
            auto_switch_cameras: false,
            face_blurring_enabled: true,
            overlay_settings: OverlaySettings {
                show_chef_name: true,
                show_customer_name: true,
                show_timestamp: true,
                show_logo: true,
                logo_position: OverlayPosition::TopRight,
                text_color: "#FFFFFF".to_string(),
                background_color: "#000000".to_string(),
                font_size: 16,
            },
        }
    }

    #[tokio::test]
    async fn test_create_kitchen_stream() {
        let manager = EnhancedStreamingManager::new(create_test_config());
        
        let result = manager.create_kitchen_stream(
            "test_stream",
            vec![StreamingPlatform::Twitch, StreamingPlatform::YouTube],
            StreamQuality::High,
            StreamLayout::Single,
            vec!["camera1".to_string()],
        ).await;

        assert!(result.is_ok());
        let stream = result.unwrap();
        assert_eq!(stream.stream_id, "test_stream");
        assert_eq!(stream.stream_type, StreamType::Kitchen);
        assert_eq!(stream.platforms.len(), 2);
    }

    #[tokio::test]
    async fn test_add_chef_overlay() {
        let manager = EnhancedStreamingManager::new(create_test_config());
        
        // Создаем трансляцию
        manager.create_kitchen_stream(
            "test_stream",
            vec![StreamingPlatform::Twitch],
            StreamQuality::Medium,
            StreamLayout::Single,
            vec!["camera1".to_string()],
        ).await.unwrap();

        // Добавляем оверлей повара
        let result = manager.add_chef_overlay(
            "test_stream",
            "chef1",
            "Chef John",
            "camera1",
            OverlayPosition::BottomLeft,
        ).await;

        assert!(result.is_ok());

        // Проверяем, что оверлей добавлен
        let stream = manager.get_stream("test_stream").await.unwrap();
        assert!(stream.chef_overlay.is_some());
        let overlay = stream.chef_overlay.unwrap();
        assert_eq!(overlay.chef_name, "Chef John");
    }

    #[tokio::test]
    async fn test_switch_camera() {
        let manager = EnhancedStreamingManager::new(create_test_config());
        
        // Создаем трансляцию
        manager.create_kitchen_stream(
            "test_stream",
            vec![StreamingPlatform::Twitch],
            StreamQuality::Medium,
            StreamLayout::Single,
            vec!["camera1".to_string()],
        ).await.unwrap();

        // Переключаем камеру
        let result = manager.switch_camera(
            "test_stream",
            "camera2",
            None,
        ).await;

        assert!(result.is_ok());

        // Проверяем, что камера переключена
        let stream = manager.get_stream("test_stream").await.unwrap();
        assert_eq!(stream.active_cameras[0], "camera2");
    }

    #[tokio::test]
    async fn test_create_mixed_stream() {
        let manager = EnhancedStreamingManager::new(create_test_config());
        
        let result = manager.create_mixed_stream(
            "mixed_stream",
            vec![StreamingPlatform::YouTube],
            StreamQuality::High,
            vec!["kitchen_cam1".to_string(), "kitchen_cam2".to_string()],
            vec!["table_cam1".to_string(), "table_cam2".to_string()],
        ).await;

        assert!(result.is_ok());
        let stream = result.unwrap();
        assert_eq!(stream.stream_type, StreamType::Mixed);
        assert_eq!(stream.active_cameras.len(), 4);
        assert_eq!(stream.layout, StreamLayout::Grid2x2);
    }
}

