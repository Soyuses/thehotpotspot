# 👨‍💻 Руководство разработчика - The Hot Pot Spot

## Содержание
1. [Настройка окружения](#настройка-окружения)
2. [Структура проекта](#структура-проекта)
3. [Архитектура кода](#архитектура-кода)
4. [Стандарты кодирования](#стандарты-кодирования)
5. [Тестирование](#тестирование)
6. [API разработка](#api-разработка)
7. [Работа с блокчейном](#работа-с-блокчейном)
8. [Интеграции](#интеграции)
9. [Отладка и профилирование](#отладка-и-профилирование)
10. [Лучшие практики](#лучшие-практики)

---

## 🛠 Настройка окружения

### Требования
- **Rust**: 1.70+ (stable)
- **PostgreSQL**: 13+
- **Node.js**: 16+ (для мобильного приложения)
- **Docker**: 20+ (опционально)

### Установка Rust
```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Проверка версии
rustc --version
cargo --version
```

### Настройка PostgreSQL
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql
brew services start postgresql

# Создание базы данных
sudo -u postgres createdb thehotpotspot
sudo -u postgres createuser --interactive
```

### Клонирование и сборка
```bash
# Клонирование репозитория
git clone <repository-url>
cd TheHotPotSpot

# Сборка проекта
cargo build

# Запуск тестов
cargo test

# Запуск в режиме разработки
cargo run --bin web_server
```

---

## 📁 Структура проекта

### Backend (Rust)
```
src/
├── lib.rs                    # Основная библиотека
├── main.rs                   # Точка входа
├── config.rs                 # Конфигурация
├── consensus.rs              # Алгоритм консенсуса
├── database.rs               # Работа с БД
├── franchise_network.rs      # Сеть франшиз
├── video_surveillance.rs     # Видеонаблюдение
├── hd_wallet.rs             # HD кошельки
├── kyc_aml.rs               # KYC/AML
├── observability.rs         # Мониторинг
├── pos_api.rs               # POS API
├── p2p_network.rs           # P2P сеть
├── ipfs_storage.rs          # IPFS
├── streaming_integration.rs # Стриминг
├── regulatory_exports.rs    # Экспорты
├── relayer_service.rs       # Relayer
├── chef_arm.rs              # Chef ARM
├── customer_streaming_arm.rs # Customer streaming
├── enhanced_streaming_manager.rs # Enhanced streaming
├── api_versioning.rs        # Версионирование API
├── bin/                     # Исполняемые файлы
│   ├── web_server.rs
│   └── simple_web_server.rs
└── tests/                   # Тесты
    ├── api.rs
    ├── blockchain.rs
    ├── database.rs
    └── integration.rs
```

### Frontend
```
web_interfaces/
├── index.html               # Главная страница
├── owner_dashboard.html     # Дашборд владельца
├── franchise_dashboard.html # Дашборд франшизы
├── customer_wallet.html     # Кошелек клиента
├── video_management_dashboard.html # Управление видео
└── api_test_interface.html  # Тестирование API

mobile_app/
├── src/
│   ├── components/          # UI компоненты
│   ├── screens/             # Экраны
│   ├── navigation/          # Навигация
│   ├── services/            # API сервисы
│   ├── store/               # Состояние
│   ├── types/               # TypeScript типы
│   └── utils/               # Утилиты
└── package.json
```

---

## 🏗 Архитектура кода

### Модульная архитектура
Каждый модуль имеет четко определенные границы и ответственность:

```rust
// Пример модуля
pub mod video_surveillance {
    pub struct VideoSurveillanceSystem {
        // Внутреннее состояние
    }
    
    impl VideoSurveillanceSystem {
        // Публичные методы
    }
    
    // Внутренние типы и функции
    mod internal {
        // Приватная логика
    }
}
```

### Принципы проектирования
1. **Single Responsibility**: Каждый модуль отвечает за одну область
2. **Dependency Injection**: Зависимости передаются через конструкторы
3. **Error Handling**: Использование Result<T, E> для обработки ошибок
4. **Async/Await**: Асинхронная обработка для I/O операций

### Паттерны проектирования
- **Builder Pattern**: Для создания сложных объектов
- **Factory Pattern**: Для создания экземпляров типов
- **Observer Pattern**: Для событий и уведомлений
- **Repository Pattern**: Для работы с данными

---

## 📝 Стандарты кодирования

### Rust Style Guide
```rust
// Именование
const MAX_RETRIES: u32 = 3;           // Константы - UPPER_SNAKE_CASE
struct UserData {                     // Типы - PascalCase
    user_id: String,                  // Поля - snake_case
}

impl UserData {
    pub fn new() -> Self {            // Методы - snake_case
        Self {
            user_id: String::new(),
        }
    }
}

// Документация
/// Создает новый экземпляр пользователя
/// 
/// # Примеры
/// ```
/// let user = UserData::new();
/// ```
pub fn create_user() -> Result<UserData, UserError> {
    // Реализация
}
```

### Обработка ошибок
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    
    #[error("Query failed: {0}")]
    QueryError(String),
    
    #[error("Transaction failed: {0}")]
    TransactionError(String),
}

// Использование
fn connect_to_database() -> Result<Connection, DatabaseError> {
    // Логика подключения
    Ok(connection)
}
```

### Логирование
```rust
use crate::observability::ObservabilityManager;

async fn process_transaction(
    obs: &ObservabilityManager,
    transaction: &Transaction
) -> Result<(), TransactionError> {
    obs.log(LogLevel::Info, "Processing transaction", "blockchain", None).await;
    
    // Обработка транзакции
    
    obs.log(LogLevel::Info, "Transaction processed successfully", "blockchain", None).await;
    Ok(())
}
```

---

## 🧪 Тестирование

### Типы тестов

#### Unit тесты
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let mut manager = HDWalletManager::new("test_seed".to_string());
        let wallet = manager.generate_node_wallet(1, WalletType::Franchise).unwrap();
        
        assert_eq!(wallet.wallet_type, WalletType::Franchise);
        assert_eq!(wallet.owner_id, "1");
    }
}
```

#### Integration тесты
```rust
#[tokio::test]
async fn test_database_operations() {
    let config = DatabaseConfig::default();
    let db = DatabaseManager::new(config).await.unwrap();
    
    let user = UserData {
        user_id: "test_user".to_string(),
        email: "test@example.com".to_string(),
        // ... другие поля
    };
    
    db.save_user(&user).await.unwrap();
    let retrieved = db.get_user("test_user").await.unwrap();
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().email, "test@example.com");
}
```

#### Property-based тесты
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_token_distribution(
        price in 1..10000u128,
        node_type in prop::sample::select(vec![NodeType::OWNER, NodeType::FRANCHISE])
    ) {
        let mut network = FranchiseNetwork::new("master".to_string());
        let node_id = network.register_node("owner".to_string(), node_type, "Tbilisi".to_string()).unwrap();
        
        let minting = network.record_sale(
            node_id,
            "sale_1".to_string(),
            price,
            "buyer_meta".to_string(),
            "pos_1".to_string(),
            vec![]
        ).unwrap();
        
        // Проверяем, что сумма распределения равна 100%
        let total = minting.owner_units + minting.buyer_units + minting.royalty_units;
        assert_eq!(total, 100);
    }
}
```

### Запуск тестов
```bash
# Все тесты
cargo test

# Конкретный тест
cargo test test_wallet_creation

# Тесты с выводом
cargo test -- --nocapture

# Бенчмарки
cargo bench
```

---

## 🌐 API разработка

### REST API структура
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}
```

### Обработка HTTP запросов
```rust
use tokio::net::TcpListener;
use std::collections::HashMap;

async fn handle_request(
    method: &str,
    path: &str,
    headers: HashMap<String, String>,
    body: Option<String>
) -> Result<String, String> {
    match (method, path) {
        ("GET", "/api/v1/health") => {
            Ok(serde_json::to_string(&ApiResponse::success("OK")).unwrap())
        },
        ("POST", "/api/v1/transactions") => {
            // Обработка создания транзакции
            Ok(serde_json::to_string(&ApiResponse::success("Transaction created")).unwrap())
        },
        _ => {
            Ok(serde_json::to_string(&ApiResponse::<()>::error("Not found".to_string())).unwrap())
        }
    }
}
```

### Версионирование API
```rust
pub mod api_versioning {
    pub const CURRENT_VERSION: &str = "v1";
    pub const SUPPORTED_VERSIONS: &[&str] = &["v1"];
    
    pub fn extract_version(path: &str) -> Option<String> {
        if let Some(version_part) = path.strip_prefix("/api/") {
            if let Some(version) = version_part.split('/').next() {
                return Some(version.to_string());
            }
        }
        None
    }
}
```

---

## ⛓ Работа с блокчейном

### Создание транзакций
```rust
use crate::consensus::{Transaction, TransactionType};

async fn create_transaction(
    node_id: u64,
    transaction_type: TransactionType,
    data: serde_json::Value
) -> Result<Transaction, String> {
    let mut transaction = Transaction::new(node_id, transaction_type, data);
    
    // Подписание транзакции
    let private_key = get_private_key(node_id).await?;
    transaction.sign(&private_key);
    
    Ok(transaction)
}
```

### Майнинг блоков
```rust
use crate::consensus::{Block, ConsensusAlgorithm};

async fn mine_block(
    consensus: &ConsensusAlgorithm,
    network: &FranchiseNetwork,
    transactions: Vec<Transaction>
) -> Result<Block, String> {
    let block_height = network.get_current_height() + 1;
    let previous_hash = network.get_last_block_hash();
    
    let mut block = Block::new(block_height, previous_hash, transactions);
    
    // Выбор валидаторов
    let validators = consensus.select_validators(network, block_height);
    
    // Валидация блока
    if consensus.validate_block(&block, &validators.selected_validators, network) {
        Ok(block)
    } else {
        Err("Block validation failed".to_string())
    }
}
```

### Работа с кошельками
```rust
use crate::hd_wallet::{HDWalletManager, WalletType};

async fn create_user_wallet(
    wallet_manager: &mut HDWalletManager,
    user_id: &str
) -> Result<HDWallet, HDWalletError> {
    wallet_manager.generate_customer_wallet(user_id.to_string())
}
```

---

## 🔗 Интеграции

### Twitch API
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchStream {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub title: String,
    pub viewer_count: u32,
}

pub struct TwitchClient {
    client_id: String,
    access_token: String,
}

impl TwitchClient {
    pub async fn start_stream(&self, title: &str) -> Result<TwitchStream, String> {
        // Реализация запуска стрима
        Ok(TwitchStream {
            id: "stream_123".to_string(),
            user_id: "user_456".to_string(),
            user_name: "hotpotspot".to_string(),
            title: title.to_string(),
            viewer_count: 0,
        })
    }
}
```

### YouTube API
```rust
pub struct YouTubeClient {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl YouTubeClient {
    pub async fn create_live_stream(&self, title: &str) -> Result<String, String> {
        // Реализация создания live стрима
        Ok("youtube_stream_123".to_string())
    }
}
```

### KYC провайдеры
```rust
pub struct KYCProvider {
    pub provider_id: String,
    pub name: String,
    pub api_endpoint: String,
    pub api_key: String,
}

impl KYCProvider {
    pub async fn verify_document(&self, document_data: &[u8]) -> Result<bool, String> {
        // Реализация верификации документа
        Ok(true)
    }
}
```

---

## 🐛 Отладка и профилирование

### Логирование для отладки
```rust
use crate::observability::{LogLevel, ObservabilityManager};

async fn debug_function(obs: &ObservabilityManager) {
    obs.log(LogLevel::Debug, "Function started", "module", None).await;
    
    // Код функции
    
    obs.log(LogLevel::Debug, "Function completed", "module", None).await;
}
```

### Профилирование производительности
```rust
use std::time::Instant;

async fn measure_performance() {
    let start = Instant::now();
    
    // Код для измерения
    
    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```

### Использование cargo flamegraph
```bash
# Установка flamegraph
cargo install flamegraph

# Профилирование
cargo flamegraph --bin web_server
```

---

## 💡 Лучшие практики

### Безопасность
1. **Валидация входных данных**: Всегда проверяйте пользовательский ввод
2. **Хеширование паролей**: Используйте bcrypt или Argon2
3. **HTTPS**: Всегда используйте зашифрованные соединения
4. **Rate Limiting**: Ограничивайте частоту запросов

### Производительность
1. **Асинхронность**: Используйте async/await для I/O операций
2. **Кэширование**: Кэшируйте часто используемые данные
3. **Connection Pooling**: Используйте пулы соединений для БД
4. **Lazy Loading**: Загружайте данные по требованию

### Поддерживаемость
1. **Документация**: Документируйте публичные API
2. **Тесты**: Покрывайте код тестами
3. **Логирование**: Логируйте важные события
4. **Мониторинг**: Отслеживайте метрики производительности

### Код-ревью
1. **Читаемость**: Код должен быть понятным
2. **Производительность**: Избегайте ненужных аллокаций
3. **Безопасность**: Проверяйте на уязвимости
4. **Тестируемость**: Код должен быть легко тестируемым

---

## 📚 Полезные ресурсы

### Документация
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serde Guide](https://serde.rs/)
- [PostgreSQL Rust Driver](https://docs.rs/tokio-postgres/)

### Инструменты
- [cargo-watch](https://crates.io/crates/cargo-watch) - Автоматическая пересборка
- [cargo-expand](https://crates.io/crates/cargo-expand) - Расширение макросов
- [cargo-audit](https://crates.io/crates/cargo-audit) - Проверка безопасности
- [cargo-clippy](https://github.com/rust-lang/rust-clippy) - Линтер

### Сообщество
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)

---

## 🚀 Следующие шаги

1. **Изучите код**: Начните с `src/lib.rs` и `src/main.rs`
2. **Запустите тесты**: Убедитесь, что все тесты проходят
3. **Изучите API**: Посмотрите на веб-интерфейсы
4. **Внесите изменения**: Создайте feature branch
5. **Напишите тесты**: Покройте новый код тестами
6. **Создайте PR**: Отправьте pull request

Удачи в разработке! 🎉
