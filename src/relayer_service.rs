//! Relayer Service для The Hot Pot Spot
//! 
//! Обеспечивает безопасную обработку транзакций от POS систем с идемпотентностью,
//! защитой от повторных атак и интеграцией с блокчейном.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Utc};
use tokio::time::sleep;
use tokio::sync::RwLock;

/// Типы транзакций для relayer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelayerTransactionType {
    Sale,           // Продажа в ресторане
    Refund,         // Возврат средств
    TokenMint,      // Эмиссия токенов
    TokenTransfer,  // Перевод токенов
}

/// Статус транзакции в relayer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,        // Ожидает обработки
    Processing,     // В процессе обработки
    Completed,      // Успешно завершена
    Failed,         // Ошибка обработки
    Rejected,       // Отклонена
}

/// Структура транзакции для relayer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerTransaction {
    pub id: String,                     // Уникальный ID транзакции
    pub sale_id: String,                // ID продажи от POS
    pub node_id: u64,                   // ID ноды ресторана
    pub pos_id: String,                 // ID POS системы
    pub transaction_type: RelayerTransactionType,
    pub amount_subunits: u128,          // Сумма в subunits
    pub buyer_address: String,          // Адрес покупателя
    pub buyer_meta: String,             // Метаданные покупателя
    pub items: Vec<SaleItem>,           // Товары в заказе
    pub signature: String,              // Подпись от POS
    pub timestamp: u64,                 // Время создания
    pub status: TransactionStatus,      // Статус обработки
    pub blockchain_tx_hash: Option<String>, // Хеш транзакции в блокчейне
    pub error_message: Option<String>,  // Сообщение об ошибке
    pub retry_count: u32,               // Количество попыток
    pub max_retries: u32,               // Максимальное количество попыток
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItem {
    pub item_id: String,
    pub quantity: u32,
    pub price_subunits: u128,
}

/// Кэш идемпотентности для предотвращения дублирования
#[derive(Debug, Clone)]
pub struct IdempotencyCache {
    pub sale_id: String,
    pub processed_at: DateTime<Utc>,
    pub transaction_id: String,
    pub status: TransactionStatus,
}

/// Конфигурация relayer сервиса
#[derive(Debug, Clone)]
pub struct RelayerConfig {
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub transaction_timeout_ms: u64,
    pub batch_size: usize,
    pub gas_limit: u64,
    pub gas_price_gwei: u64,
    pub network_id: u64,
    pub contract_address: String,
}

impl Default for RelayerConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 5000, // 5 секунд
            transaction_timeout_ms: 30000, // 30 секунд
            batch_size: 10,
            gas_limit: 200000,
            gas_price_gwei: 20,
            network_id: 1, // Ethereum mainnet
            contract_address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }
}

/// Основной relayer сервис
pub struct RelayerService {
    config: RelayerConfig,
    pending_transactions: Arc<RwLock<HashMap<String, RelayerTransaction>>>,
    idempotency_cache: Arc<Mutex<HashMap<String, IdempotencyCache>>>,
    processed_transactions: Arc<RwLock<HashMap<String, RelayerTransaction>>>,
    statistics: Arc<Mutex<RelayerStatistics>>,
}

#[derive(Debug, Default)]
#[derive(Clone)]
pub struct RelayerStatistics {
    pub total_processed: u64,
    pub total_successful: u64,
    pub total_failed: u64,
    pub total_retries: u64,
    pub average_processing_time_ms: u64,
}

impl RelayerService {
    /// Создание нового relayer сервиса
    pub fn new(config: RelayerConfig) -> Self {
        Self {
            config,
            pending_transactions: Arc::new(RwLock::new(HashMap::new())),
            idempotency_cache: Arc::new(Mutex::new(HashMap::new())),
            processed_transactions: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(RelayerStatistics::default())),
        }
    }

    /// Обработка новой транзакции от POS системы
    pub async fn process_transaction(&self, request: RelayerTransactionRequest) -> Result<RelayerResponse, RelayerError> {
        // 1. Проверка идемпотентности
        if self.is_duplicate(&request.sale_id).await {
            return Ok(RelayerResponse {
                transaction_id: self.get_cached_transaction_id(&request.sale_id).await,
                status: TransactionStatus::Completed,
                message: "Transaction already processed".to_string(),
                blockchain_tx_hash: None,
            });
        }

        // 2. Валидация запроса
        self.validate_request(&request)?;

        // 3. Создание транзакции
        let transaction = self.create_transaction(request).await?;

        // 4. Добавление в кэш идемпотентности
        self.add_to_idempotency_cache(&transaction).await;

        // 5. Добавление в очередь обработки
        self.add_to_pending_queue(&transaction).await;

        // 6. Запуск асинхронной обработки
        // TODO: Fix Send trait issue - need to restructure to avoid &mut self in async context
        // self.process_transaction_async(transaction.clone()).await;

        Ok(RelayerResponse {
            transaction_id: transaction.id.clone(),
            status: TransactionStatus::Pending,
            message: "Transaction queued for processing".to_string(),
            blockchain_tx_hash: None,
        })
    }

    /// Проверка на дублирование транзакции
    async fn is_duplicate(&self, sale_id: &str) -> bool {
        let cache = self.idempotency_cache.lock().unwrap();
        cache.contains_key(sale_id)
    }

    /// Получение ID кэшированной транзакции
    async fn get_cached_transaction_id(&self, sale_id: &str) -> String {
        let cache = self.idempotency_cache.lock().unwrap();
        cache.get(sale_id)
            .map(|entry| entry.transaction_id.clone())
            .unwrap_or_default()
    }

    /// Валидация запроса
    fn validate_request(&self, request: &RelayerTransactionRequest) -> Result<(), RelayerError> {
        if request.sale_id.is_empty() {
            return Err(RelayerError::ValidationError("Sale ID cannot be empty".to_string()));
        }

        if request.amount_subunits == 0 {
            return Err(RelayerError::ValidationError("Amount must be greater than 0".to_string()));
        }

        if request.buyer_address.is_empty() {
            return Err(RelayerError::ValidationError("Buyer address cannot be empty".to_string()));
        }

        if request.items.is_empty() {
            return Err(RelayerError::ValidationError("Items list cannot be empty".to_string()));
        }

        // Проверка подписи (упрощенная версия)
        if !self.verify_signature(request) {
            return Err(RelayerError::ValidationError("Invalid signature".to_string()));
        }

        Ok(())
    }

    /// Проверка подписи (упрощенная версия)
    fn verify_signature(&self, request: &RelayerTransactionRequest) -> bool {
        // В реальной реализации здесь должна быть проверка криптографической подписи
        // от POS системы
        !request.signature.is_empty()
    }

    /// Создание транзакции
    async fn create_transaction(&self, request: RelayerTransactionRequest) -> Result<RelayerTransaction, RelayerError> {
        let transaction_id = self.generate_transaction_id(&request).await;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(RelayerTransaction {
            id: transaction_id,
            sale_id: request.sale_id,
            node_id: request.node_id,
            pos_id: request.pos_id,
            transaction_type: RelayerTransactionType::Sale,
            amount_subunits: request.amount_subunits,
            buyer_address: request.buyer_address,
            buyer_meta: request.buyer_meta,
            items: request.items,
            signature: request.signature,
            timestamp,
            status: TransactionStatus::Pending,
            blockchain_tx_hash: None,
            error_message: None,
            retry_count: 0,
            max_retries: self.config.max_retries,
        })
    }

    /// Генерация уникального ID транзакции
    async fn generate_transaction_id(&self, request: &RelayerTransactionRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.sale_id.as_bytes());
        hasher.update(request.pos_id.as_bytes());
        hasher.update(request.timestamp.to_be_bytes());
        hasher.update(request.amount_subunits.to_be_bytes());
        
        let hash = hasher.finalize();
        format!("relayer_{}", hex::encode(&hash[..8]))
    }

    /// Добавление в кэш идемпотентности
    async fn add_to_idempotency_cache(&self, transaction: &RelayerTransaction) {
        let cache_entry = IdempotencyCache {
            sale_id: transaction.sale_id.clone(),
            processed_at: Utc::now(),
            transaction_id: transaction.id.clone(),
            status: transaction.status.clone(),
        };

        let mut cache = self.idempotency_cache.lock().unwrap();
        cache.insert(transaction.sale_id.clone(), cache_entry);
    }

    /// Добавление в очередь обработки
    async fn add_to_pending_queue(&self, transaction: &RelayerTransaction) {
        let mut pending = self.pending_transactions.write().await;
        pending.insert(transaction.id.clone(), transaction.clone());
    }

    /// Асинхронная обработка транзакции
    async fn process_transaction_async(&mut self, mut transaction: RelayerTransaction) {
        let transaction_id = transaction.id.clone();
        
        // Обновляем статус на "обрабатывается"
        {
            let mut pending = self.pending_transactions.write().await;
            if let Some(tx) = pending.get_mut(&transaction_id) {
                tx.status = TransactionStatus::Processing;
            }
        }

        // Симуляция обработки блокчейн транзакции
        let result = self.execute_blockchain_transaction(&transaction).await;

        match result {
            Ok(blockchain_tx_hash) => {
                // Успешная обработка
                transaction.status = TransactionStatus::Completed;
                transaction.blockchain_tx_hash = Some(blockchain_tx_hash);
                
                // Обновляем статистику
                self.update_statistics(true, 0).await;
            }
            Err(error) => {
                // Ошибка обработки
                transaction.retry_count += 1;
                transaction.error_message = Some(error.to_string());

                if transaction.retry_count >= transaction.max_retries {
                    transaction.status = TransactionStatus::Failed;
                    self.update_statistics(false, transaction.retry_count).await;
                } else {
                    transaction.status = TransactionStatus::Pending;
                    self.update_statistics(false, transaction.retry_count).await;
                    
                    // TODO: Fix Send trait issue - need to restructure to avoid &mut self in async context
                    // Планируем повторную попытку
                    // let delay = Duration::from_millis(self.config.retry_delay_ms);
                    // let service_clone = self.clone();
                    // let tx_clone = transaction.clone();
                    // tokio::spawn(async move {
                    //     sleep(delay).await;
                    //     service_clone.process_transaction_async(tx_clone).await;
                    // });
                }
            }
        }

        // Перемещаем в обработанные транзакции
        {
            let mut pending = self.pending_transactions.write().await;
            pending.remove(&transaction_id);
        }

        {
            let mut processed = self.processed_transactions.write().await;
            processed.insert(transaction_id, transaction);
        }
    }

    /// Выполнение блокчейн транзакции
    async fn execute_blockchain_transaction(&self, transaction: &RelayerTransaction) -> Result<String, RelayerError> {
        // Симуляция вызова смарт-контракта
        // В реальной реализации здесь будет:
        // 1. Подготовка данных для контракта
        // 2. Оценка газа
        // 3. Подписание транзакции
        // 4. Отправка в сеть
        // 5. Ожидание подтверждения

        // Симуляция задержки сети
        sleep(Duration::from_millis(1000)).await;

        // Симуляция успешного выполнения
        let mut hasher = Sha256::new();
        hasher.update(transaction.id.as_bytes());
        hasher.update(transaction.sale_id.as_bytes());
        hasher.update(transaction.timestamp.to_be_bytes());
        
        let hash = hasher.finalize();
        Ok(format!("0x{}", hex::encode(&hash[..20])))
    }

    /// Обновление статистики
    async fn update_statistics(&self, success: bool, retry_count: u32) {
        let mut stats = self.statistics.lock().unwrap();
        stats.total_processed += 1;
        stats.total_retries += retry_count as u64;
        
        if success {
            stats.total_successful += 1;
        } else {
            stats.total_failed += 1;
        }
    }

    /// Получение статуса транзакции
    pub async fn get_transaction_status(&self, transaction_id: &str) -> Option<RelayerTransaction> {
        // Проверяем в обработанных транзакциях
        {
            let processed = self.processed_transactions.read().await;
            if let Some(tx) = processed.get(transaction_id) {
                return Some(tx.clone());
            }
        }

        // Проверяем в ожидающих транзакциях
        {
            let pending = self.pending_transactions.read().await;
            if let Some(tx) = pending.get(transaction_id) {
                return Some(tx.clone());
            }
        }

        None
    }

    /// Получение статистики
    pub async fn get_statistics(&self) -> RelayerStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Очистка старых записей из кэша идемпотентности
    pub async fn cleanup_old_cache_entries(&self, max_age_hours: u64) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        
        let mut cache = self.idempotency_cache.lock().unwrap();
        cache.retain(|_, entry| entry.processed_at > cutoff_time);
    }
}

/// Запрос на обработку транзакции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerTransactionRequest {
    pub sale_id: String,
    pub node_id: u64,
    pub pos_id: String,
    pub amount_subunits: u128,
    pub buyer_address: String,
    pub buyer_meta: String,
    pub items: Vec<SaleItem>,
    pub signature: String,
    pub timestamp: u64,
}

/// Ответ relayer сервиса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerResponse {
    pub transaction_id: String,
    pub status: TransactionStatus,
    pub message: String,
    pub blockchain_tx_hash: Option<String>,
}

/// Ошибки relayer сервиса
#[derive(Debug, thiserror::Error)]
pub enum RelayerError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

// Реализация Clone для RelayerService
impl Clone for RelayerService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            pending_transactions: Arc::clone(&self.pending_transactions),
            idempotency_cache: Arc::clone(&self.idempotency_cache),
            processed_transactions: Arc::clone(&self.processed_transactions),
            statistics: Arc::clone(&self.statistics),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_relayer_transaction_processing() {
        let config = RelayerConfig::default();
        let relayer = RelayerService::new(config);

        let request = RelayerTransactionRequest {
            sale_id: "SALE_001".to_string(),
            node_id: 1,
            pos_id: "POS_001".to_string(),
            amount_subunits: 500, // 5.00 GEL
            buyer_address: "0x1234567890abcdef".to_string(),
            buyer_meta: "Customer data".to_string(),
            items: vec![SaleItem {
                item_id: "ITEM_001".to_string(),
                quantity: 1,
                price_subunits: 500,
            }],
            signature: "signature_data".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        let response = relayer.process_transaction(request).await;
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.status, TransactionStatus::Pending);
        assert!(!response.transaction_id.is_empty());
    }

    #[tokio::test]
    async fn test_idempotency() {
        let config = RelayerConfig::default();
        let relayer = RelayerService::new(config);

        let request = RelayerTransactionRequest {
            sale_id: "SALE_002".to_string(),
            node_id: 1,
            pos_id: "POS_001".to_string(),
            amount_subunits: 500,
            buyer_address: "0x1234567890abcdef".to_string(),
            buyer_meta: "Customer data".to_string(),
            items: vec![SaleItem {
                item_id: "ITEM_001".to_string(),
                quantity: 1,
                price_subunits: 500,
            }],
            signature: "signature_data".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        // Первая обработка
        let response1 = relayer.process_transaction(request.clone()).await.unwrap();
        
        // Вторая обработка того же запроса
        let response2 = relayer.process_transaction(request).await.unwrap();
        
        // Должны получить тот же transaction_id
        assert_eq!(response1.transaction_id, response2.transaction_id);
        assert_eq!(response2.status, TransactionStatus::Completed);
    }
}

