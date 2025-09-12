//! АРМ посетителя для трансляции трапезы
//! 
//! Позволяет посетителям включать трансляцию своей трапезы с согласием на обработку данных

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// АРМ посетителя для трансляции
pub struct CustomerStreamingARM {
    active_consents: Arc<RwLock<HashMap<String, CustomerConsent>>>,
    active_streams: Arc<RwLock<HashMap<String, CustomerStream>>>,
    privacy_policy_version: String,
    consent_templates: Arc<RwLock<HashMap<String, ConsentTemplate>>>,
}

/// Согласие посетителя на трансляцию
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerConsent {
    pub consent_id: String,
    pub customer_id: String,
    pub table_id: String,
    pub camera_id: String,
    pub customer_name: String,
    pub consent_timestamp: u64,
    pub privacy_policy_version: String,
    pub data_processing_consent: bool,
    pub streaming_consent: bool,
    pub face_recognition_consent: bool,
    pub data_retention_consent: bool,
    pub marketing_consent: bool,
    pub ip_address: String,
    pub user_agent: String,
    pub device_id: String,
    pub consent_signature: String, // Хеш подписи
}

/// Активная трансляция посетителя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerStream {
    pub stream_id: String,
    pub customer_id: String,
    pub table_id: String,
    pub camera_id: String,
    pub customer_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub is_active: bool,
    pub consent: CustomerConsent,
    pub overlay_enabled: bool,
    pub viewer_count: u32,
    pub quality: StreamQuality,
    pub platforms: Vec<StreamingPlatform>,
}

/// Качество стрима
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamQuality {
    Low,    // 480p
    Medium, // 720p
    High,   // 1080p
}

/// Платформы для стриминга
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamingPlatform {
    Twitch,
    YouTube,
    Facebook,
    Instagram,
}

/// Шаблон согласия
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentTemplate {
    pub template_id: String,
    pub language: String,
    pub title: String,
    pub content: String,
    pub required_consents: Vec<ConsentType>,
    pub optional_consents: Vec<ConsentType>,
    pub version: String,
    pub effective_date: u64,
}

/// Тип согласия
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsentType {
    DataProcessing,
    Streaming,
    FaceRecognition,
    DataRetention,
    Marketing,
}

/// Запрос на согласие
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub customer_id: String,
    pub table_id: String,
    pub camera_id: String,
    pub customer_name: String,
    pub language: String,
    pub ip_address: String,
    pub user_agent: String,
    pub device_id: String,
}

/// Ответ на запрос согласия
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentResponse {
    pub consent_id: String,
    pub template: ConsentTemplate,
    pub qr_code: String,
    pub expires_at: u64,
}

/// Подтверждение согласия
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentConfirmation {
    pub consent_id: String,
    pub customer_id: String,
    pub consent_signature: String,
    pub accepted_consents: Vec<ConsentType>,
    pub rejected_consents: Vec<ConsentType>,
    pub timestamp: u64,
}

impl CustomerStreamingARM {
    /// Создать новый АРМ посетителя
    pub fn new(privacy_policy_version: String) -> Self {
        Self {
            active_consents: Arc::new(RwLock::new(HashMap::new())),
            active_streams: Arc::new(RwLock::new(HashMap::new())),
            privacy_policy_version,
            consent_templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Запросить согласие на трансляцию
    pub async fn request_consent(&self, request: ConsentRequest) -> Result<ConsentResponse, String> {
        // Проверяем, что у нас есть шаблон для данного языка
        let templates = self.consent_templates.read().await;
        let template = templates.get(&request.language)
            .ok_or_else(|| format!("No consent template found for language: {}", request.language))?;

        let consent_id = format!("CONSENT_{}_{}", request.customer_id, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        let qr_code = format!("CONSENT_QR_{}_{}", consent_id, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        let expires_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600; // 1 час

        Ok(ConsentResponse {
            consent_id: consent_id.clone(),
            template: template.clone(),
            qr_code,
            expires_at,
        })
    }

    /// Подтвердить согласие
    pub async fn confirm_consent(&self, confirmation: ConsentConfirmation) -> Result<CustomerConsent, String> {
        // Проверяем, что согласие не истекло
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if now > confirmation.timestamp + 3600 {
            return Err("Consent request expired".to_string());
        }

        // Создаем запись согласия
        let consent = CustomerConsent {
            consent_id: confirmation.consent_id.clone(),
            customer_id: confirmation.customer_id.clone(),
            table_id: "".to_string(), // Будет заполнено из контекста
            camera_id: "".to_string(), // Будет заполнено из контекста
            customer_name: "".to_string(), // Будет заполнено из контекста
            consent_timestamp: confirmation.timestamp,
            privacy_policy_version: self.privacy_policy_version.clone(),
            data_processing_consent: confirmation.accepted_consents.contains(&ConsentType::DataProcessing),
            streaming_consent: confirmation.accepted_consents.contains(&ConsentType::Streaming),
            face_recognition_consent: confirmation.accepted_consents.contains(&ConsentType::FaceRecognition),
            data_retention_consent: confirmation.accepted_consents.contains(&ConsentType::DataRetention),
            marketing_consent: confirmation.accepted_consents.contains(&ConsentType::Marketing),
            ip_address: "".to_string(), // Будет заполнено из контекста
            user_agent: "".to_string(), // Будет заполнено из контекста
            device_id: "".to_string(), // Будет заполнено из контекста
            consent_signature: confirmation.consent_signature,
        };

        // Сохраняем согласие
        let mut consents = self.active_consents.write().await;
        consents.insert(confirmation.consent_id, consent.clone());

        Ok(consent)
    }

    /// Начать трансляцию посетителя
    pub async fn start_customer_stream(
        &self,
        customer_id: &str,
        table_id: &str,
        camera_id: &str,
        customer_name: &str,
        quality: StreamQuality,
        platforms: Vec<StreamingPlatform>,
    ) -> Result<CustomerStream, String> {
        // Проверяем, что у посетителя есть действующее согласие
        let consents = self.active_consents.read().await;
        let consent = consents.values()
            .find(|c| c.customer_id == customer_id && c.streaming_consent)
            .ok_or_else(|| "No valid streaming consent found".to_string())?;

        let stream_id = format!("CUSTOMER_STREAM_{}_{}", customer_id, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

        let stream = CustomerStream {
            stream_id: stream_id.clone(),
            customer_id: customer_id.to_string(),
            table_id: table_id.to_string(),
            camera_id: camera_id.to_string(),
            customer_name: customer_name.to_string(),
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            end_time: None,
            is_active: true,
            consent: consent.clone(),
            overlay_enabled: true,
            viewer_count: 0,
            quality,
            platforms,
        };

        // Сохраняем трансляцию
        let mut streams = self.active_streams.write().await;
        streams.insert(stream_id, stream.clone());

        Ok(stream)
    }

    /// Остановить трансляцию посетителя
    pub async fn stop_customer_stream(&self, customer_id: &str) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(customer_id) {
            stream.is_active = false;
            stream.end_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            Ok(())
        } else {
            Err(format!("No active stream found for customer: {}", customer_id))
        }
    }

    /// Получить все активные трансляции посетителей
    pub async fn get_active_customer_streams(&self) -> Vec<CustomerStream> {
        let streams = self.active_streams.read().await;
        streams.values().filter(|s| s.is_active).cloned().collect()
    }

    /// Получить трансляцию посетителя по ID
    pub async fn get_customer_stream(&self, customer_id: &str) -> Option<CustomerStream> {
        let streams = self.active_streams.read().await;
        streams.get(customer_id).cloned()
    }

    /// Обновить количество зрителей
    pub async fn update_viewer_count(&self, customer_id: &str, viewer_count: u32) -> Result<(), String> {
        let mut streams = self.active_streams.write().await;
        if let Some(stream) = streams.get_mut(customer_id) {
            stream.viewer_count = viewer_count;
            Ok(())
        } else {
            Err(format!("No stream found for customer: {}", customer_id))
        }
    }

    /// Добавить шаблон согласия
    pub async fn add_consent_template(&self, template: ConsentTemplate) -> Result<(), String> {
        let mut templates = self.consent_templates.write().await;
        templates.insert(template.language.clone(), template);
        Ok(())
    }

    /// Получить шаблон согласия по языку
    pub async fn get_consent_template(&self, language: &str) -> Option<ConsentTemplate> {
        let templates = self.consent_templates.read().await;
        templates.get(language).cloned()
    }

    /// Проверить, есть ли у посетителя действующее согласие
    pub async fn has_valid_consent(&self, customer_id: &str) -> bool {
        let consents = self.active_consents.read().await;
        consents.values().any(|c| c.customer_id == customer_id && c.streaming_consent)
    }

    /// Получить статистику трансляций
    pub async fn get_streaming_statistics(&self) -> StreamingStatistics {
        let streams = self.active_streams.read().await;
        let active_count = streams.values().filter(|s| s.is_active).count();
        let total_viewers: u32 = streams.values().map(|s| s.viewer_count).sum();
        let total_duration: u64 = streams.values()
            .map(|s| {
                let end_time = s.end_time.unwrap_or_else(|| 
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                end_time - s.start_time
            })
            .sum();

        StreamingStatistics {
            active_streams: active_count as u32,
            total_viewers,
            total_duration,
            total_streams: streams.len() as u32,
        }
    }

    /// Удалить истекшие согласия
    pub async fn cleanup_expired_consents(&self) -> u32 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut consents = self.active_consents.write().await;
        let expired_keys: Vec<String> = consents.iter()
            .filter(|(_, consent)| now - consent.consent_timestamp > 86400) // 24 часа
            .map(|(key, _)| key.clone())
            .collect();
        
        let count = expired_keys.len() as u32;
        for key in expired_keys {
            consents.remove(&key);
        }
        count
    }
}

/// Статистика трансляций
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingStatistics {
    pub active_streams: u32,
    pub total_viewers: u32,
    pub total_duration: u64,
    pub total_streams: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consent_workflow() {
        let arm = CustomerStreamingARM::new("1.0".to_string());

        // Добавляем шаблон согласия
        let template = ConsentTemplate {
            template_id: "template_1".to_string(),
            language: "en".to_string(),
            title: "Streaming Consent".to_string(),
            content: "Do you consent to streaming your meal?".to_string(),
            required_consents: vec![ConsentType::DataProcessing, ConsentType::Streaming],
            optional_consents: vec![ConsentType::Marketing],
            version: "1.0".to_string(),
            effective_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        arm.add_consent_template(template).await.unwrap();

        // Запрашиваем согласие
        let request = ConsentRequest {
            customer_id: "customer_1".to_string(),
            table_id: "table_1".to_string(),
            camera_id: "camera_1".to_string(),
            customer_name: "John Doe".to_string(),
            language: "en".to_string(),
            ip_address: "192.168.1.1".to_string(),
            user_agent: "Mozilla/5.0".to_string(),
            device_id: "device_1".to_string(),
        };

        let response = arm.request_consent(request).await.unwrap();
        assert!(!response.consent_id.is_empty());
        assert!(!response.qr_code.is_empty());

        // Подтверждаем согласие
        let confirmation = ConsentConfirmation {
            consent_id: response.consent_id.clone(),
            customer_id: "customer_1".to_string(),
            consent_signature: "signature_hash".to_string(),
            accepted_consents: vec![ConsentType::DataProcessing, ConsentType::Streaming],
            rejected_consents: vec![ConsentType::Marketing],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        let consent = arm.confirm_consent(confirmation).await.unwrap();
        assert_eq!(consent.customer_id, "customer_1");
        assert!(consent.streaming_consent);
        assert!(!consent.marketing_consent);
    }

    #[tokio::test]
    async fn test_customer_streaming() {
        let arm = CustomerStreamingARM::new("1.0".to_string());

        // Добавляем шаблон согласия
        let template = ConsentTemplate {
            template_id: "template_1".to_string(),
            language: "en".to_string(),
            title: "Streaming Consent".to_string(),
            content: "Do you consent to streaming your meal?".to_string(),
            required_consents: vec![ConsentType::DataProcessing, ConsentType::Streaming],
            optional_consents: vec![],
            version: "1.0".to_string(),
            effective_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        arm.add_consent_template(template).await.unwrap();

        // Создаем согласие напрямую для теста
        let consent = CustomerConsent {
            consent_id: "consent_1".to_string(),
            customer_id: "customer_1".to_string(),
            table_id: "table_1".to_string(),
            camera_id: "camera_1".to_string(),
            customer_name: "John Doe".to_string(),
            consent_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            privacy_policy_version: "1.0".to_string(),
            data_processing_consent: true,
            streaming_consent: true,
            face_recognition_consent: false,
            data_retention_consent: true,
            marketing_consent: false,
            ip_address: "192.168.1.1".to_string(),
            user_agent: "Mozilla/5.0".to_string(),
            device_id: "device_1".to_string(),
            consent_signature: "signature_hash".to_string(),
        };

        let mut consents = arm.active_consents.write().await;
        consents.insert("consent_1".to_string(), consent);
        drop(consents);

        // Начинаем трансляцию
        let stream = arm.start_customer_stream(
            "customer_1",
            "table_1",
            "camera_1",
            "John Doe",
            StreamQuality::Medium,
            vec![StreamingPlatform::YouTube],
        ).await.unwrap();

        assert_eq!(stream.customer_id, "customer_1");
        assert_eq!(stream.table_id, "table_1");
        assert!(stream.is_active);

        // Останавливаем трансляцию
        let result = arm.stop_customer_stream("customer_1").await;
        assert!(result.is_ok());

        // Проверяем, что трансляция остановлена
        let updated_stream = arm.get_customer_stream("customer_1").await.unwrap();
        assert!(!updated_stream.is_active);
        assert!(updated_stream.end_time.is_some());
    }
}

