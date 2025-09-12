# 🎯 Vision - The Hot Pot Spot

## Техническое состояние проекта

### Обзор проекта

**The Hot Pot Spot** - это комплексная блокчейн-система для управления сетью ресторанов франшизы с интегрированной системой видеонаблюдения, построенная на Rust. Проект представляет собой полнофункциональную экосистему, включающую блокчейн-ядро, веб-интерфейсы, мобильное приложение и систему видеонаблюдения.

---

## 🛠 Технологии

### Backend (Rust)
- **Язык**: Rust 2021 Edition
- **Async Runtime**: Tokio
- **Веб-сервер**: Custom HTTP server
- **База данных**: PostgreSQL с tokio-postgres
- **Сериализация**: Serde (JSON)
- **Криптография**: SHA2, hex encoding
- **Тестирование**: tokio-test, proptest, quickcheck, criterion

### Blockchain
- **Консенсус**: Proof of Stake + Reputation
- **Токеномика**: ERC20-совместимые токены
- **Смарт-контракты**: Solidity (OpenZeppelin)
- **Сеть**: P2P с IPFS для хранения
- **HD Wallets**: BIP32/BIP44 совместимые кошельки

### Frontend
- **Веб-интерфейсы**: HTML5, CSS3, JavaScript (Vanilla)
- **Мобильное приложение**: React Native 0.72.6
- **Навигация**: React Navigation 6.x
- **UI компоненты**: React Native Paper, React Native Elements
- **Состояние**: Zustand, React Query

### Видеонаблюдение
- **Стриминг**: Twitch, YouTube интеграция
- **Обработка видео**: Real-time с анонимизацией лиц
- **Согласие**: GDPR/грузинское законодательство
- **Камеры**: IP-камеры с различными типами (Security, Kitchen, Customer)

### DevOps & Infrastructure
- **Контейнеризация**: Docker, docker-compose
- **База данных**: PostgreSQL
- **Мониторинг**: Prometheus-совместимые метрики
- **Логирование**: Структурированные логи
- **CI/CD**: PowerShell скрипты

---

## 🏗 Принципы разработки

### Архитектурные принципы
1. **Модульность**: Каждый компонент изолирован и имеет четкие границы
2. **Асинхронность**: Полностью асинхронная архитектура на Tokio
3. **Безопасность**: KYC/AML соответствие, шифрование, аудит
4. **Масштабируемость**: Горизонтальное масштабирование через P2P сеть
5. **Отказоустойчивость**: Graceful degradation, retry механизмы

### Принципы кодирования
- **Rust Best Practices**: Ownership, borrowing, error handling
- **Type Safety**: Строгая типизация, compile-time проверки
- **Error Handling**: Result<T, E> паттерн, thiserror для кастомных ошибок
- **Testing**: Unit, integration, property-based тесты
- **Documentation**: Comprehensive rustdoc

### Принципы безопасности
- **Zero Trust**: Проверка всех входных данных
- **Principle of Least Privilege**: Минимальные необходимые права
- **Audit Trail**: Полное логирование всех операций
- **Data Protection**: GDPR compliance, анонимизация

---

## 📁 Структура проекта

```
TheHotPotSpot/
├── src/                          # Rust backend
│   ├── lib.rs                    # Основная библиотека
│   ├── main.rs                   # Точка входа
│   ├── config.rs                 # Конфигурация системы
│   ├── consensus.rs              # Алгоритм консенсуса
│   ├── database.rs               # Работа с БД
│   ├── franchise_network.rs      # Сеть франшиз
│   ├── video_surveillance.rs     # Видеонаблюдение
│   ├── hd_wallet.rs             # HD кошельки
│   ├── kyc_aml.rs               # KYC/AML система
│   ├── observability.rs         # Мониторинг и логи
│   ├── pos_api.rs               # POS API
│   ├── p2p_network.rs           # P2P сеть
│   ├── ipfs_storage.rs          # IPFS хранение
│   ├── streaming_integration.rs # Стриминг интеграция
│   ├── regulatory_exports.rs    # Регуляторные экспорты
│   ├── relayer_service.rs       # Relayer сервис
│   ├── chef_arm.rs              # Chef ARM
│   ├── customer_streaming_arm.rs # Customer streaming
│   ├── enhanced_streaming_manager.rs # Enhanced streaming
│   ├── api_versioning.rs        # Версионирование API
│   ├── bin/                     # Исполняемые файлы
│   └── tests/                   # Тесты
├── mobile_app/                   # React Native приложение
│   ├── src/
│   │   ├── components/          # UI компоненты
│   │   ├── screens/             # Экраны приложения
│   │   ├── navigation/          # Навигация
│   │   ├── services/            # API сервисы
│   │   ├── store/               # Управление состоянием
│   │   ├── types/               # TypeScript типы
│   │   └── utils/               # Утилиты
│   └── package.json
├── contracts/                    # Смарт-контракты
│   └── FranchiseToken.sol       # ERC20 токен
├── web_interfaces/              # HTML интерфейсы
│   ├── index.html               # Главная страница
│   ├── owner_dashboard.html     # Дашборд владельца
│   ├── franchise_dashboard.html # Дашборд франшизы
│   ├── customer_wallet.html     # Кошелек клиента
│   ├── video_management_dashboard.html # Управление видео
│   └── api_test_interface.html  # Тестирование API
├── scripts/                     # Скрипты автоматизации
├── tests/                       # Интеграционные тесты
├── benches/                     # Бенчмарки
└── docs/                        # Документация
```

---

## 🏛 Архитектура проекта

### Высокоуровневая архитектура

```mermaid
graph TB
    subgraph "Frontend Layer"
        WEB[Web Interfaces]
        MOBILE[Mobile App]
        API_CLIENT[API Clients]
    end
    
    subgraph "API Gateway"
        API[HTTP API Server]
        AUTH[Authentication]
        RATE_LIMIT[Rate Limiting]
    end
    
    subgraph "Core Services"
        BLOCKCHAIN[Blockchain Core]
        CONSENSUS[Consensus Engine]
        FRANCHISE[Franchise Network]
        VIDEO[Video Surveillance]
        KYC[KYC/AML System]
        WALLET[HD Wallet Manager]
    end
    
    subgraph "Storage Layer"
        POSTGRES[(PostgreSQL)]
        IPFS[IPFS Network]
        FILES[File Storage]
    end
    
    subgraph "External Services"
        TWITCH[Twitch API]
        YOUTUBE[YouTube API]
        KYC_PROVIDER[KYC Providers]
    end
    
    WEB --> API
    MOBILE --> API
    API_CLIENT --> API
    
    API --> AUTH
    API --> RATE_LIMIT
    API --> BLOCKCHAIN
    
    BLOCKCHAIN --> CONSENSUS
    BLOCKCHAIN --> FRANCHISE
    BLOCKCHAIN --> VIDEO
    BLOCKCHAIN --> KYC
    BLOCKCHAIN --> WALLET
    
    FRANCHISE --> POSTGRES
    KYC --> POSTGRES
    VIDEO --> IPFS
    VIDEO --> FILES
    
    VIDEO --> TWITCH
    VIDEO --> YOUTUBE
    KYC --> KYC_PROVIDER
```

### Микросервисная архитектура

1. **API Gateway Service**: Единая точка входа для всех клиентов
2. **Blockchain Core Service**: Управление блокчейном и консенсусом
3. **Franchise Network Service**: Управление сетью франшиз
4. **Video Surveillance Service**: Обработка видео и стриминг
5. **KYC/AML Service**: Верификация пользователей
6. **Wallet Service**: Управление кошельками
7. **POS Integration Service**: Интеграция с POS системами

---

## 🗄 Модель данных

### Основные сущности

#### Пользователи и роли
```rust
struct KYCUser {
    user_id: String,
    email: String,
    kyc_status: KYCStatus,
    kyc_level: KYCLevel,
    risk_score: u8,
    documents: Vec<KYCDocument>,
    roles: Vec<UserRole>,
}
```

#### Сеть франшиз
```rust
struct FranchiseNetwork {
    master_owner: String,
    nodes: HashMap<u64, FranchiseNode>,
    sales: Vec<Sale>,
    token_mintings: Vec<TokenMinting>,
    wallets: HashMap<String, Wallet>,
}
```

#### Видеонаблюдение
```rust
struct VideoSurveillanceSystem {
    cameras: HashMap<String, CameraConfig>,
    active_recordings: HashMap<String, ActiveRecording>,
    consent_records: HashMap<String, VideoConsent>,
    security_alerts: HashMap<String, SecurityAlert>,
}
```

#### Блокчейн
```rust
struct Block {
    height: u64,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    signatures: Vec<BlockSignature>,
}
```

### Схема базы данных

#### Основные таблицы
- `users` - Пользователи системы
- `documents` - KYC документы
- `roles` - Роли пользователей
- `permissions` - Разрешения
- `wallets` - HD кошельки
- `transactions` - Блокчейн транзакции
- `audit_logs` - Аудит логи

---

## ⛓ Работа с блокчейном

### Запуск блокчейна

#### Локальная разработка
```bash
# Запуск PostgreSQL
docker-compose up -d postgres

# Миграции БД
cargo run --bin database_migrate

# Запуск ноды
cargo run --bin blockchain_node -- --config config.toml
```

#### Продакшн развертывание
```bash
# Сборка релизной версии
cargo build --release

# Запуск с systemd
sudo systemctl start thehotpotspot-node
```

### Поддержка и обслуживание

#### Мониторинг нод
- **Health Checks**: HTTP endpoints для проверки состояния
- **Metrics**: Prometheus метрики для мониторинга
- **Logs**: Структурированные логи в JSON формате
- **Alerts**: Автоматические уведомления о проблемах

#### Обновление нод
1. **Rolling Updates**: Поэтапное обновление нод
2. **Backward Compatibility**: Обратная совместимость API
3. **Database Migrations**: Автоматические миграции БД
4. **Rollback Strategy**: План отката при проблемах

#### Резервное копирование
- **Database Backups**: Ежедневные бэкапы PostgreSQL
- **Wallet Backups**: Шифрованные бэкапы seed фраз
- **Configuration Backups**: Версионирование конфигураций

---

## 📊 Мониторинг стабильности блокчейна

### Ключевые метрики

#### Сетевые метрики
- **Node Uptime**: Время работы нод
- **Network Latency**: Задержка в сети
- **Consensus Time**: Время достижения консенсуса
- **Block Production Rate**: Скорость создания блоков

#### Бизнес метрики
- **Transaction Volume**: Объем транзакций
- **Active Users**: Активные пользователи
- **Token Circulation**: Обращение токенов
- **Franchise Growth**: Рост сети франшиз

#### Технические метрики
- **Memory Usage**: Использование памяти
- **CPU Usage**: Использование CPU
- **Disk I/O**: Дисковые операции
- **Database Performance**: Производительность БД

### Система алертов

#### Критические алерты
- **Node Down**: Нода недоступна
- **Consensus Failure**: Сбой консенсуса
- **Database Connection Lost**: Потеря соединения с БД
- **High Error Rate**: Высокий уровень ошибок

#### Предупреждающие алерты
- **High Memory Usage**: Высокое использование памяти
- **Slow Response Time**: Медленное время отклика
- **Low Disk Space**: Мало места на диске
- **Unusual Transaction Pattern**: Необычные паттерны транзакций

---

## 🎬 Сценарии работы

### Сценарий 1: Регистрация новой франшизы
1. **Владелец франшизы** подает заявку через веб-интерфейс
2. **KYC процесс** - верификация документов
3. **Создание ноды** в блокчейне
4. **Генерация HD кошелька** для франшизы
5. **Активация POS систем** и интеграция
6. **Начало работы** и первые продажи

### Сценарий 2: Обработка продажи
1. **Клиент** делает заказ через POS систему
2. **Создание транзакции** в блокчейне
3. **Эмиссия токенов** согласно токеномике
4. **Распределение токенов** между участниками
5. **Обновление балансов** кошельков
6. **Запись в аудит лог**

### Сценарий 3: Видеонаблюдение
1. **Клиент** дает согласие на видеозапись
2. **Активация камер** за столом
3. **Обработка видео** с анонимизацией лиц
4. **Стриминг** на Twitch/YouTube (опционально)
5. **Сохранение записи** в IPFS
6. **Автоматическое отключение** по таймауту

### Сценарий 4: KYC верификация
1. **Пользователь** регистрируется в системе
2. **Загрузка документов** для верификации
3. **Автоматическая проверка** документов
4. **Ручная верификация** сотрудником комплаенса
5. **AML проверки** (санкции, PEP)
6. **Присвоение роли** и разрешений

---

## 🚀 Принципы и механизмы деплоя

### Состояние деплоя

#### Текущее состояние
- ✅ **Локальная разработка**: Полностью настроена
- ✅ **Docker контейнеризация**: Готова
- ✅ **База данных**: PostgreSQL настроена
- ⚠️ **Продакшн деплой**: В процессе настройки
- ⚠️ **CI/CD пайплайн**: Частично реализован
- ❌ **Kubernetes**: Не реализован
- ❌ **Load Balancing**: Не настроен

#### Планы развития
1. **Kubernetes кластер** для оркестрации
2. **Terraform** для инфраструктуры как код
3. **GitHub Actions** для CI/CD
4. **Monitoring stack** (Prometheus + Grafana)
5. **Log aggregation** (ELK Stack)

### Механизмы деплоя

#### Blue-Green Deployment
```bash
# Подготовка новой версии
cargo build --release
docker build -t thehotpotspot:v2.0.0 .

# Переключение трафика
kubectl apply -f k8s/green-deployment.yaml
kubectl patch service api -p '{"spec":{"selector":{"version":"v2.0.0"}}}'

# Откат при проблемах
kubectl patch service api -p '{"spec":{"selector":{"version":"v1.0.0"}}}'
```

#### Rolling Updates
```bash
# Обновление с проверками
kubectl set image deployment/api api=thehotpotspot:v2.0.0
kubectl rollout status deployment/api
kubectl rollout undo deployment/api  # При необходимости
```

#### Database Migrations
```bash
# Автоматические миграции
cargo run --bin migrate -- --up
cargo run --bin migrate -- --down  # Откат
```

---

## ⚙️ Реализация подхода к конфигурированию

### Конфигурационная система

#### Иерархия конфигурации
1. **Default values** (в коде)
2. **Environment variables** (переменные окружения)
3. **Config files** (TOML файлы)
4. **Command line arguments** (аргументы командной строки)

#### Структура конфигурации
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub blockchain: BlockchainConfig,
    pub video: VideoConfig,
    pub kyc: KYCConfig,
    pub observability: ObservabilityConfig,
}
```

#### Примеры конфигурации

**config.toml**
```toml
[database]
host = "localhost"
port = 5432
database = "thehotpotspot"
username = "postgres"
password = "password"

[blockchain]
consensus_algorithm = "proof_of_stake"
min_validators = 3
max_validators = 7

[video]
twitch_client_id = "your_twitch_id"
youtube_client_id = "your_youtube_id"
default_anonymization = "face_replacement"

[observability]
log_level = "info"
enable_metrics = true
prometheus_port = 9090
```

**Environment Variables**
```bash
export DATABASE_URL="postgresql://user:pass@localhost/db"
export BLOCKCHAIN_NETWORK="mainnet"
export VIDEO_TWITCH_CLIENT_ID="your_id"
export OBSERVABILITY_LOG_LEVEL="debug"
```

### Управление секретами

#### Локальная разработка
```bash
# .env файл (не в git)
DATABASE_PASSWORD=secret_password
TWITCH_CLIENT_SECRET=secret_key
```

#### Продакшн
- **Kubernetes Secrets**: Для контейнеров
- **HashiCorp Vault**: Для централизованного управления
- **AWS Secrets Manager**: Для облачных деплоев

---

## 📝 Реализация подхода к логгированию

### Структурированное логирование

#### Формат логов
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "message": "Transaction processed successfully",
  "module": "blockchain",
  "function": "process_transaction",
  "request_id": "req_123456",
  "user_id": "user_789",
  "metadata": {
    "transaction_id": "tx_abc123",
    "amount": "500",
    "currency": "GEL"
  }
}
```

#### Уровни логирования
- **TRACE**: Детальная отладочная информация
- **DEBUG**: Отладочная информация
- **INFO**: Общая информация о работе
- **WARN**: Предупреждения
- **ERROR**: Ошибки
- **FATAL**: Критические ошибки

#### Контекстная информация
- **Request ID**: Для трассировки запросов
- **User ID**: Для аудита пользователей
- **Module/Function**: Для локализации проблем
- **Metadata**: Дополнительная контекстная информация

### Централизованное логирование

#### Локальная разработка
```bash
# Вывод в консоль с форматированием
RUST_LOG=debug cargo run
```

#### Продакшн
- **ELK Stack**: Elasticsearch + Logstash + Kibana
- **Fluentd**: Сбор и отправка логов
- **Prometheus**: Метрики из логов
- **Grafana**: Визуализация логов и метрик

### Аудит и соответствие

#### Аудит логи
```rust
struct AuditLogEntry {
    log_id: String,
    user_id: String,
    action: String,
    resource: String,
    timestamp: DateTime<Utc>,
    ip_address: Option<String>,
    success: bool,
    details: Option<String>,
}
```

#### Соответствие требованиям
- **GDPR**: Право на забвение, анонимизация
- **Грузинское законодательство**: Локальные требования
- **Financial Regulations**: Соответствие финансовым нормам

---

## 🎯 Заключение

**The Hot Pot Spot** представляет собой зрелую, хорошо спроектированную систему с четкой архитектурой, современными технологиями и комплексным подходом к разработке. Проект демонстрирует:

- **Высокое качество кода** с использованием лучших практик Rust
- **Модульную архитектуру** с четким разделением ответственности
- **Комплексную систему безопасности** с KYC/AML соответствием
- **Инновационные решения** в области блокчейна и видеонаблюдения
- **Готовность к масштабированию** и промышленному использованию

Система готова к дальнейшему развитию и может служить основой для создания полноценной экосистемы франшизных ресторанов с блокчейн-интеграцией.
