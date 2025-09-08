use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

/// Типы камер в системе видеонаблюдения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CameraType {
    /// Камеры вокруг заведения (внешние)
    External,
    /// Камеры в зонах производства
    Production,
    /// Камеры для откровений (специальная зона)
    Confession,
    /// Камеры за столами посетителей
    CustomerTable,
}

/// Статус камеры
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CameraStatus {
    Active,
    Inactive,
    Recording,
    Streaming,
    Maintenance,
}

/// Зоны анонимизации
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnonymizationZone {
    /// Полная анонимизация лиц
    FullFaceBlur,
    /// Замена лиц на выбранное изображение
    FaceReplacement,
    /// Без анонимизации (только с согласия)
    NoAnonymization,
}

/// Согласие на видеозапись
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConsent {
    pub customer_id: String,
    pub table_id: String,
    pub consent_given: bool,
    pub consent_timestamp: u64,
    pub max_duration_minutes: u32,
    pub anonymization_preference: AnonymizationZone,
    pub consent_text: String,
    pub ip_address: String,
    pub user_agent: String,
}

/// Конфигурация камеры
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub camera_id: String,
    pub camera_type: CameraType,
    pub location: String,
    pub ip_address: String,
    pub port: u16,
    pub resolution: (u32, u32),
    pub fps: u32,
    pub anonymization_zone: AnonymizationZone,
    pub requires_consent: bool,
    pub max_recording_duration: Option<Duration>,
    pub stream_to_twitch: bool,
    pub stream_to_youtube: bool,
}

/// Активная запись
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecording {
    pub recording_id: String,
    pub camera_id: String,
    pub customer_id: Option<String>,
    pub table_id: Option<String>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub duration_limit: Duration,
    pub consent: Option<VideoConsent>,
    pub is_streaming: bool,
    pub twitch_stream_id: Option<String>,
    pub youtube_stream_id: Option<String>,
}

/// Система видеонаблюдения
pub struct VideoSurveillanceSystem {
    cameras: Arc<RwLock<HashMap<String, CameraConfig>>>,
    active_recordings: Arc<RwLock<HashMap<String, ActiveRecording>>>,
    consent_records: Arc<RwLock<HashMap<String, VideoConsent>>>,
    face_replacement_images: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    streaming_config: StreamingConfig,
}

/// Конфигурация стриминга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    pub twitch_access_token: String,
    pub youtube_client_id: String,
    pub youtube_client_secret: String,
    pub youtube_refresh_token: String,
    pub stream_quality: StreamQuality,
    pub default_anonymization: AnonymizationZone,
}

/// Качество стрима
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamQuality {
    Low,    // 480p
    Medium, // 720p
    High,   // 1080p
}

impl VideoSurveillanceSystem {
    pub fn new(streaming_config: StreamingConfig) -> Self {
        Self {
            cameras: Arc::new(RwLock::new(HashMap::new())),
            active_recordings: Arc::new(RwLock::new(HashMap::new())),
            consent_records: Arc::new(RwLock::new(HashMap::new())),
            face_replacement_images: Arc::new(RwLock::new(HashMap::new())),
            streaming_config,
        }
    }

    /// Добавить камеру в систему
    pub async fn add_camera(&self, config: CameraConfig) -> Result<(), String> {
        let mut cameras = self.cameras.write().await;
        
        // Проверяем, что камера не дублируется
        if cameras.contains_key(&config.camera_id) {
            return Err(format!("Camera {} already exists", config.camera_id));
        }

        // Валидация конфигурации
        self.validate_camera_config(&config)?;
        
        cameras.insert(config.camera_id.clone(), config);
        Ok(())
    }

    /// Валидация конфигурации камеры
    fn validate_camera_config(&self, config: &CameraConfig) -> Result<(), String> {
        // Проверяем, что камеры за столами требуют согласие
        if config.camera_type == CameraType::CustomerTable && !config.requires_consent {
            return Err("Customer table cameras must require consent".to_string());
        }

        // Проверяем, что камеры в зонах производства не требуют согласие
        if config.camera_type == CameraType::Production && config.requires_consent {
            return Err("Production cameras should not require consent".to_string());
        }

        // Проверяем разрешение
        if config.resolution.0 < 320 || config.resolution.1 < 240 {
            return Err("Resolution too low".to_string());
        }

        Ok(())
    }

    /// Получить согласие на видеозапись
    pub async fn request_video_consent(
        &self,
        customer_id: String,
        table_id: String,
        consent_text: String,
        ip_address: String,
        user_agent: String,
    ) -> Result<VideoConsent, String> {
        let consent = VideoConsent {
            customer_id: customer_id.clone(),
            table_id: table_id.clone(),
            consent_given: false, // Пользователь должен подтвердить
            consent_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            max_duration_minutes: 30, // Максимум 30 минут
            anonymization_preference: AnonymizationZone::FaceReplacement,
            consent_text,
            ip_address,
            user_agent,
        };

        let mut consent_records = self.consent_records.write().await;
        consent_records.insert(customer_id.clone(), consent.clone());
        
        Ok(consent)
    }

    /// Подтвердить согласие на видеозапись
    pub async fn confirm_video_consent(
        &self,
        customer_id: String,
        anonymization_preference: AnonymizationZone,
    ) -> Result<(), String> {
        let mut consent_records = self.consent_records.write().await;
        
        if let Some(consent) = consent_records.get_mut(&customer_id) {
            consent.consent_given = true;
            consent.anonymization_preference = anonymization_preference;
            consent.consent_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Ok(())
        } else {
            Err("Consent not found".to_string())
        }
    }

    /// Начать запись с камеры
    pub async fn start_recording(
        &self,
        camera_id: String,
        customer_id: Option<String>,
        table_id: Option<String>,
    ) -> Result<String, String> {
        let cameras = self.cameras.read().await;
        let camera = cameras.get(&camera_id)
            .ok_or_else(|| "Camera not found".to_string())?;

        // Проверяем, требуется ли согласие
        if camera.requires_consent {
            if let Some(ref cid) = customer_id {
                let consent_records = self.consent_records.read().await;
                let consent = consent_records.get(cid)
                    .ok_or_else(|| "Consent not found".to_string())?;
                
                if !consent.consent_given {
                    return Err("Consent not given".to_string());
                }
            } else {
                return Err("Customer ID required for consent-based recording".to_string());
            }
        }

        let recording_id = format!("REC_{}_{}", camera_id, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

        let consent = if let Some(ref cid) = customer_id {
            let consent_records = self.consent_records.read().await;
            consent_records.get(cid).cloned()
        } else {
            None
        };

        let recording = ActiveRecording {
            recording_id: recording_id.clone(),
            camera_id: camera_id.clone(),
            customer_id,
            table_id,
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            end_time: None,
            duration_limit: camera.max_recording_duration
                .unwrap_or(Duration::from_secs(30 * 60)), // 30 минут по умолчанию
            consent,
            is_streaming: camera.stream_to_twitch || camera.stream_to_youtube,
            twitch_stream_id: None,
            youtube_stream_id: None,
        };

        let mut active_recordings = self.active_recordings.write().await;
        active_recordings.insert(recording_id.clone(), recording);

        // Запускаем стрим, если настроено
        if camera.stream_to_twitch || camera.stream_to_youtube {
            self.start_streaming(&recording_id, &camera_id).await?;
        }

        Ok(recording_id)
    }

    /// Остановить запись
    pub async fn stop_recording(&self, recording_id: String) -> Result<(), String> {
        let mut active_recordings = self.active_recordings.write().await;
        
        if let Some(recording) = active_recordings.get_mut(&recording_id) {
            recording.end_time = Some(
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            );

            // Останавливаем стрим
            if recording.is_streaming {
                self.stop_streaming(&recording_id).await?;
            }

            Ok(())
        } else {
            Err("Recording not found".to_string())
        }
    }

    /// Начать стриминг
    async fn start_streaming(&self, recording_id: &str, camera_id: &str) -> Result<(), String> {
        // Здесь должна быть интеграция с Twitch и YouTube API
        // Пока что заглушка
        println!("Starting stream for recording {} from camera {}", recording_id, camera_id);
        Ok(())
    }

    /// Остановить стриминг
    async fn stop_streaming(&self, recording_id: &str) -> Result<(), String> {
        // Здесь должна быть интеграция с Twitch и YouTube API
        // Пока что заглушка
        println!("Stopping stream for recording {}", recording_id);
        Ok(())
    }

    /// Обработать видеокадр для анонимизации
    pub async fn process_video_frame(
        &self,
        frame_data: &[u8],
        camera_id: &str,
        anonymization_zone: &AnonymizationZone,
    ) -> Result<Vec<u8>, String> {
        match anonymization_zone {
            AnonymizationZone::FullFaceBlur => {
                self.blur_faces(frame_data).await
            },
            AnonymizationZone::FaceReplacement => {
                self.replace_faces(frame_data, camera_id).await
            },
            AnonymizationZone::NoAnonymization => {
                Ok(frame_data.to_vec())
            },
        }
    }

    /// Размытие лиц
    async fn blur_faces(&self, frame_data: &[u8]) -> Result<Vec<u8>, String> {
        // Здесь должна быть интеграция с библиотекой компьютерного зрения
        // для обнаружения и размытия лиц
        // Пока что возвращаем исходные данные
        Ok(frame_data.to_vec())
    }

    /// Замена лиц
    async fn replace_faces(&self, frame_data: &[u8], camera_id: &str) -> Result<Vec<u8>, String> {
        // Здесь должна быть интеграция с библиотекой компьютерного зрения
        // для обнаружения лиц и замены их на выбранное изображение
        // Пока что возвращаем исходные данные
        Ok(frame_data.to_vec())
    }

    /// Получить список активных записей
    pub async fn get_active_recordings(&self) -> Vec<ActiveRecording> {
        let active_recordings = self.active_recordings.read().await;
        active_recordings.values().cloned().collect()
    }

    /// Получить статистику по камерам
    pub async fn get_camera_statistics(&self) -> HashMap<String, CameraStats> {
        let cameras = self.cameras.read().await;
        let active_recordings = self.active_recordings.read().await;
        
        let mut stats = HashMap::new();
        
        for (camera_id, _) in cameras.iter() {
            let recording_count = active_recordings.values()
                .filter(|r| r.camera_id == *camera_id)
                .count();
            
            stats.insert(camera_id.clone(), CameraStats {
                camera_id: camera_id.clone(),
                active_recordings: recording_count,
                total_recordings: 0, // Нужно будет добавить историю
            });
        }
        
        stats
    }
}

/// Статистика камеры
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraStats {
    pub camera_id: String,
    pub active_recordings: usize,
    pub total_recordings: usize,
}

/// API для управления видеонаблюдением
pub struct VideoSurveillanceAPI {
    system: Arc<VideoSurveillanceSystem>,
}

impl VideoSurveillanceAPI {
    pub fn new(system: Arc<VideoSurveillanceSystem>) -> Self {
        Self { system }
    }

    /// Получить согласие на видеозапись (для веб-интерфейса)
    pub async fn request_consent(
        &self,
        customer_id: String,
        table_id: String,
        ip_address: String,
        user_agent: String,
    ) -> Result<VideoConsentRequest, String> {
        let consent_text = self.get_consent_text().await;
        
        let consent = self.system.request_video_consent(
            customer_id,
            table_id,
            consent_text,
            ip_address,
            user_agent,
        ).await?;

        Ok(VideoConsentRequest {
            consent_id: consent.customer_id.clone(),
            consent_text: consent.consent_text,
            max_duration_minutes: consent.max_duration_minutes,
            anonymization_options: vec![
                AnonymizationZone::FullFaceBlur,
                AnonymizationZone::FaceReplacement,
                AnonymizationZone::NoAnonymization,
            ],
        })
    }

    /// Получить текст согласия согласно законодательству Грузии
    async fn get_consent_text(&self) -> String {
        // Текст должен быть на грузинском и русском языках
        "მე ვეთანხმები ჩემი ვიდეო ჩაწერას The Hot Pot Spot-ში ჩემი ვიზიტის დროს. 
        ვიდეო შეიძლება გამოყენებულ იქნას უსაფრთხოების მიზნებისთვის და შეიძლება 
        გადაცემული იქნეს Twitch და YouTube-ზე. ჩემი ღვთისმშობლი უფლებები 
        დაცულია ქართული კანონმდებლობის შესაბამისად.

        Я соглашаюсь на видеозапись моего пребывания в The Hot Pot Spot. 
        Видео может использоваться в целях безопасности и может транслироваться 
        на Twitch и YouTube. Мои права защищены в соответствии с грузинским законодательством.".to_string()
    }
}

/// Запрос на согласие
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConsentRequest {
    pub consent_id: String,
    pub consent_text: String,
    pub max_duration_minutes: u32,
    pub anonymization_options: Vec<AnonymizationZone>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_camera_addition() {
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

        let system = VideoSurveillanceSystem::new(streaming_config);
        
        let camera_config = CameraConfig {
            camera_id: "CAM_001".to_string(),
            camera_type: CameraType::CustomerTable,
            location: "Table 1".to_string(),
            ip_address: "192.168.1.100".to_string(),
            port: 8080,
            resolution: (1920, 1080),
            fps: 30,
            anonymization_zone: AnonymizationZone::FaceReplacement,
            requires_consent: true,
            max_recording_duration: Some(Duration::from_secs(30 * 60)),
            stream_to_twitch: true,
            stream_to_youtube: false,
        };

        let result = system.add_camera(camera_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_consent_workflow() {
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

        let system = VideoSurveillanceSystem::new(streaming_config);
        
        // Запрашиваем согласие
        let consent = system.request_video_consent(
            "CUSTOMER_001".to_string(),
            "TABLE_001".to_string(),
            "Test consent text".to_string(),
            "192.168.1.1".to_string(),
            "Mozilla/5.0".to_string(),
        ).await.unwrap();

        assert!(!consent.consent_given);

        // Подтверждаем согласие
        let result = system.confirm_video_consent(
            "CUSTOMER_001".to_string(),
            AnonymizationZone::FaceReplacement,
        ).await;

        assert!(result.is_ok());
    }
}
