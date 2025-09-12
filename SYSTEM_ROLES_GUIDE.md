# 👥 Руководство по системным ролям - The Hot Pot Spot

## Содержание
1. [Обзор ролевой модели](#обзор-ролевой-модели)
2. [Административные роли](#административные-роли)
3. [Операционные роли](#операционные-роли)
4. [Пользовательские роли](#пользовательские-роли)
5. [Системные роли](#системные-роли)
6. [Разрешения и доступ](#разрешения-и-доступ)
7. [KYC/AML роли](#kycaml-роли)
8. [Блокчейн роли](#блокчейн-роли)
9. [Видеонаблюдение роли](#видеонаблюдение-роли)
10. [Управление ролями](#управление-ролями)

---

## 🎯 Обзор ролевой модели

The Hot Pot Spot использует комплексную ролевую модель на основе **Role-Based Access Control (RBAC)** с элементами **Attribute-Based Access Control (ABAC)** для обеспечения безопасности и соответствия регуляторным требованиям.

### Принципы ролевой модели
- **Principle of Least Privilege**: Минимальные необходимые права
- **Separation of Duties**: Разделение обязанностей
- **Audit Trail**: Полное логирование действий
- **Compliance**: Соответствие GDPR и грузинскому законодательству

### Иерархия ролей
```
SuperAdmin (Супер администратор)
├── Admin (Администратор)
├── Compliance (Сотрудник комплаенса)
├── MasterOwner (Владелец сети)
│   ├── FranchiseOwner (Владелец франшизы)
│   ├── POSOperator (Оператор POS)
│   └── Cashier (Кассир)
├── Customer (Покупатель)
├── Investor (Инвестор)
├── System (Системная роль)
└── Auditor (Аудитор)
```

---

## 🔧 Административные роли

### SuperAdmin (Супер администратор)
**Описание**: Высший уровень доступа с полными правами на управление системой.

**Основные обязанности**:
- Управление всей инфраструктурой системы
- Настройка глобальных параметров
- Управление другими администраторами
- Кризисное управление

**Разрешения**:
```rust
pub enum SuperAdminPermissions {
    // Системное управление
    ManageSystem,
    ManageInfrastructure,
    ManageGlobalSettings,
    
    // Управление пользователями
    ManageAllUsers,
    ManageAllRoles,
    AssignAnyRole,
    
    // Финансовые операции
    ManageAllFinancials,
    ProcessAnyTransaction,
    AccessAllWallets,
    
    // Безопасность
    ManageSecuritySettings,
    AccessAllAuditLogs,
    ManageEncryptionKeys,
    
    // Блокчейн
    ManageBlockchainNetwork,
    ManageConsensus,
    ManageAllNodes,
    
    // Видеонаблюдение
    ManageAllCameras,
    AccessAllVideoStreams,
    ManageVideoSettings,
}
```

**Интерфейсы доступа**:
- Административная панель
- API с полными правами
- Системные утилиты
- Мониторинг и алерты

### Admin (Администратор)
**Описание**: Администратор системы с ограниченными правами по сравнению с SuperAdmin.

**Основные обязанности**:
- Управление пользователями и ролями
- Мониторинг системы
- Обработка инцидентов
- Управление конфигурацией

**Разрешения**:
```rust
pub enum AdminPermissions {
    // Управление пользователями
    ManageUsers,
    ManageRoles,
    ViewUserData,
    
    // Мониторинг
    ViewSystemMetrics,
    ViewAuditLogs,
    ManageAlerts,
    
    // Конфигурация
    ManageApplicationSettings,
    ManageAPISettings,
    
    // Поддержка
    AccessSupportTools,
    ManageTickets,
}
```

**Ограничения**:
- Не может управлять системной инфраструктурой
- Не имеет доступа к финансовым операциям
- Не может изменять криптографические ключи

### Compliance (Сотрудник комплаенса)
**Описание**: Специалист по соответствию регуляторным требованиям.

**Основные обязанности**:
- Верификация KYC документов
- AML проверки
- Генерация регуляторных отчетов
- Мониторинг подозрительных операций

**Разрешения**:
```rust
pub enum CompliancePermissions {
    // KYC/AML
    VerifyKYC,
    ViewKYCData,
    ManageCompliance,
    GenerateReports,
    
    // Мониторинг
    ViewTransactionHistory,
    ViewUserActivity,
    FlagSuspiciousActivity,
    
    // Отчетность
    GenerateRegulatoryReports,
    ExportComplianceData,
    ManageComplianceSettings,
}
```

**Специальные права**:
- Доступ к персональным данным для верификации
- Возможность блокировки пользователей
- Генерация отчетов для регуляторов

---

## 🏪 Операционные роли

### MasterOwner (Владелец сети)
**Описание**: Владелец всей франшизной сети с правами на управление всеми нодами.

**Основные обязанности**:
- Управление сетью франшиз
- Настройка токеномики
- Управление POS системами
- Финансовое планирование

**Разрешения**:
```rust
pub enum MasterOwnerPermissions {
    // Управление сетью
    ManageAllNodes,
    ManageFranchiseNetwork,
    ConfigureTokenomics,
    
    // Финансы
    ViewAllFinancials,
    ProcessAllTransactions,
    ManageAllWallets,
    
    // POS системы
    ManageAllPOS,
    ConfigurePOSSettings,
    ViewPOSMetrics,
    
    // Меню и товары
    ManageGlobalMenu,
    SetPricing,
    ManageInventory,
}
```

**Токеномические права**:
- Получение 48% от собственных нод
- Получение 3% роялти от франшизных нод
- Управление благотворительным фондом

### FranchiseOwner (Владелец франшизы)
**Описание**: Владелец отдельной франшизной точки.

**Основные обязанности**:
- Управление своей франшизой
- Настройка меню
- Управление персоналом
- Финансовый контроль

**Разрешения**:
```rust
pub enum FranchiseOwnerPermissions {
    // Управление франшизой
    ManageOwnFranchise,
    ConfigureOwnMenu,
    ManageOwnStaff,
    
    // Финансы
    ViewOwnFinancials,
    ProcessOwnTransactions,
    ManageOwnWallet,
    
    // POS системы
    ManageOwnPOS,
    ViewOwnPOSMetrics,
    
    // Видеонаблюдение
    ManageOwnCameras,
    ViewOwnVideoStreams,
}
```

**Токеномические права**:
- Получение 48% от продаж своей франшизы
- Управление поварами и их токенами

### POSOperator (Оператор POS)
**Описание**: Оператор системы продаж в ресторане.

**Основные обязанности**:
- Обработка заказов
- Управление кассой
- Взаимодействие с клиентами
- Ведение отчетности

**Разрешения**:
```rust
pub enum POSOperatorPermissions {
    // POS операции
    ProcessOrders,
    ManageCashRegister,
    HandleRefunds,
    
    // Клиенты
    ViewCustomerData,
    ProcessCustomerPayments,
    
    // Отчетность
    ViewSalesReports,
    GenerateDailyReports,
}
```

**Ограничения**:
- Не может изменять настройки системы
- Ограниченный доступ к финансовым данным
- Не может управлять персоналом

### Cashier (Кассир)
**Описание**: Кассир с базовыми правами на обработку платежей.

**Основные обязанности**:
- Прием платежей
- Выдача чеков
- Базовая отчетность

**Разрешения**:
```rust
pub enum CashierPermissions {
    // Платежи
    ProcessPayments,
    IssueReceipts,
    HandleCash,
    
    // Базовая отчетность
    ViewOwnTransactions,
    GenerateBasicReports,
}
```

---

## 👤 Пользовательские роли

### Customer (Покупатель)
**Описание**: Обычный клиент ресторана с базовыми правами.

**Основные возможности**:
- Просмотр меню
- Создание заказов
- Управление кошельком
- Голосование за блюда

**Разрешения**:
```rust
pub enum CustomerPermissions {
    // Заказы
    CreateOrders,
    ViewOwnOrders,
    CancelOwnOrders,
    
    // Кошелек
    ViewOwnWallet,
    TransferTokens,
    ViewTransactionHistory,
    
    // Голосование
    VoteOnProposals,
    ViewVotingResults,
    
    // Профиль
    ViewOwnData,
    UpdateOwnProfile,
}
```

**Токеномические права**:
- Получение 25% токенов от покупок
- Участие в голосовании за новые блюда

### Investor (Инвестор)
**Описание**: Инвестор с расширенными правами на просмотр финансовых данных.

**Основные возможности**:
- Просмотр финансовых отчетов
- Анализ метрик сети
- Инвестирование в франшизы

**Разрешения**:
```rust
pub enum InvestorPermissions {
    // Финансы
    ViewFinancialReports,
    ViewNetworkMetrics,
    ViewInvestmentOpportunities,
    
    // Аналитика
    ViewAnalytics,
    ExportFinancialData,
    
    // Инвестиции
    MakeInvestments,
    ViewInvestmentPortfolio,
}
```

---

## 🤖 Системные роли

### System (Системная роль)
**Описание**: Внутренняя роль для системных процессов.

**Основные функции**:
- Автоматические процессы
- Интеграции с внешними системами
- Фоновые задачи

**Разрешения**:
```rust
pub enum SystemPermissions {
    // Автоматизация
    ProcessAutomatedTransactions,
    RunScheduledTasks,
    ManageSystemProcesses,
    
    // Интеграции
    AccessExternalAPIs,
    ProcessWebhooks,
    ManageIntegrations,
    
    // Данные
    AccessSystemData,
    ProcessSystemEvents,
}
```

### Auditor (Аудитор)
**Описание**: Внешний аудитор с правами только на чтение.

**Основные обязанности**:
- Аудит финансовых операций
- Проверка соответствия процедурам
- Генерация аудиторских отчетов

**Разрешения**:
```rust
pub enum AuditorPermissions {
    // Аудит
    ViewAllAuditLogs,
    ViewAllTransactions,
    ViewAllUserActivity,
    
    // Отчеты
    GenerateAuditReports,
    ExportAuditData,
    
    // Мониторинг
    ViewSystemMetrics,
    ViewComplianceReports,
}
```

**Ограничения**:
- Только чтение данных
- Не может изменять настройки
- Ограниченный период доступа

---

## 🔐 Разрешения и доступ

### Матрица разрешений

| Роль | Управление пользователями | Финансы | Блокчейн | Видео | KYC/AML |
|------|---------------------------|---------|----------|-------|---------|
| SuperAdmin | ✅ Полный | ✅ Полный | ✅ Полный | ✅ Полный | ✅ Полный |
| Admin | ✅ Ограниченный | ❌ | ❌ | ❌ | ✅ Просмотр |
| Compliance | ✅ KYC | ❌ | ❌ | ❌ | ✅ Полный |
| MasterOwner | ✅ Свои ноды | ✅ Свои | ✅ Свои | ✅ Свои | ❌ |
| FranchiseOwner | ✅ Своя франшиза | ✅ Своя | ✅ Своя | ✅ Своя | ❌ |
| POSOperator | ❌ | ✅ Ограниченный | ❌ | ❌ | ❌ |
| Cashier | ❌ | ✅ Базовый | ❌ | ❌ | ❌ |
| Customer | ❌ | ✅ Свой кошелек | ✅ Свои токены | ❌ | ❌ |
| Investor | ❌ | ✅ Просмотр | ✅ Просмотр | ❌ | ❌ |
| System | ✅ Системные | ✅ Автоматические | ✅ Автоматические | ✅ Автоматические | ✅ Автоматические |
| Auditor | ❌ | ✅ Просмотр | ✅ Просмотр | ✅ Просмотр | ✅ Просмотр |

### Уровни доступа

#### Уровень 1: Базовый доступ
- Просмотр публичной информации
- Создание заказов
- Управление собственным профилем

#### Уровень 2: Операционный доступ
- Обработка транзакций
- Управление POS системами
- Просмотр операционных данных

#### Уровень 3: Административный доступ
- Управление пользователями
- Настройка системы
- Просмотр всех данных

#### Уровень 4: Системный доступ
- Полный доступ к системе
- Управление инфраструктурой
- Криптографические операции

---

## 📋 KYC/AML роли

### KYC Specialist (Специалист KYC)
**Описание**: Специалист по верификации клиентов.

**Обязанности**:
- Верификация документов
- Проверка личности клиентов
- Обновление KYC статусов

**Разрешения**:
```rust
pub enum KYCSpecialistPermissions {
    // Документы
    ViewKYCData,
    VerifyDocuments,
    UpdateKYCStatus,
    
    // Клиенты
    ViewCustomerData,
    UpdateCustomerInfo,
    
    // Отчеты
    GenerateKYCReports,
    ExportKYCData,
}
```

### AML Analyst (AML аналитик)
**Описание**: Аналитик по противодействию отмыванию денег.

**Обязанности**:
- Мониторинг подозрительных операций
- AML проверки
- Генерация отчетов для регуляторов

**Разрешения**:
```rust
pub enum AMLAnalystPermissions {
    // Мониторинг
    ViewAllTransactions,
    FlagSuspiciousActivity,
    ViewRiskScores,
    
    // Проверки
    PerformAMLChecks,
    UpdateRiskAssessment,
    
    // Отчеты
    GenerateAMLReports,
    SubmitRegulatoryReports,
}
```

---

## ⛓ Блокчейн роли

### Validator (Валидатор)
**Описание**: Узел, участвующий в консенсусе блокчейна.

**Обязанности**:
- Валидация транзакций
- Участие в консенсусе
- Поддержание сети

**Права**:
```rust
pub enum ValidatorPermissions {
    // Консенсус
    ParticipateInConsensus,
    ValidateTransactions,
    ProposeBlocks,
    
    // Сеть
    MaintainNetwork,
    SyncBlockchain,
    
    // Награды
    ReceiveValidatorRewards,
    ViewValidatorMetrics,
}
```

### Node Operator (Оператор ноды)
**Описание**: Оператор блокчейн ноды.

**Обязанности**:
- Поддержание работы ноды
- Мониторинг производительности
- Обновление ПО

**Разрешения**:
```rust
pub enum NodeOperatorPermissions {
    // Нода
    ManageNode,
    ViewNodeMetrics,
    UpdateNodeSoftware,
    
    // Сеть
    ViewNetworkStatus,
    ManageConnections,
    
    // Логи
    ViewNodeLogs,
    ManageLogSettings,
}
```

---

## 📹 Видеонаблюдение роли

### Video Manager (Менеджер видео)
**Описание**: Менеджер системы видеонаблюдения.

**Обязанности**:
- Управление камерами
- Настройка стриминга
- Мониторинг записей

**Разрешения**:
```rust
pub enum VideoManagerPermissions {
    // Камеры
    ManageCameras,
    ConfigureCameraSettings,
    ViewCameraFeeds,
    
    // Стриминг
    ManageStreaming,
    ConfigureStreamSettings,
    ViewStreamMetrics,
    
    // Записи
    ViewRecordings,
    ManageRecordingSettings,
    ExportVideoData,
}
```

### Security Officer (Сотрудник безопасности)
**Описание**: Сотрудник службы безопасности.

**Обязанности**:
- Мониторинг безопасности
- Обработка инцидентов
- Управление доступом

**Разрешения**:
```rust
pub enum SecurityOfficerPermissions {
    // Безопасность
    ViewSecurityFeeds,
    ManageSecurityAlerts,
    HandleSecurityIncidents,
    
    // Доступ
    ManageAccessControl,
    ViewAccessLogs,
    
    // Инциденты
    CreateIncidentReports,
    ManageIncidentResponse,
}
```

---

## ⚙️ Управление ролями

### Назначение ролей
```rust
use crate::kyc_aml::{KYCAmlManager, UserRole, Permission};

async fn assign_role_example() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // Назначение роли администратора
    kyc_manager.assign_role(
        "user_123",
        UserRole::Admin,
        "super_admin_456".to_string(),
        None // Без истечения
    ).await?;
    
    // Проверка разрешений
    let has_permission = kyc_manager.has_permission(
        "user_123",
        &Permission::ManageUsers
    );
    
    println!("User has ManageUsers permission: {}", has_permission);
}
```

### Временные роли
```rust
use chrono::{Utc, Duration};

async fn assign_temporary_role() {
    let mut kyc_manager = KYCAmlManager::new();
    
    // Назначение временной роли (на 30 дней)
    let expires_at = Utc::now() + Duration::days(30);
    
    kyc_manager.assign_role(
        "auditor_789",
        UserRole::Auditor,
        "admin_123".to_string(),
        Some(expires_at)
    ).await?;
}
```

### Проверка доступа
```rust
async fn check_access_control() {
    let kyc_manager = KYCAmlManager::new();
    
    // Получение всех разрешений пользователя
    let permissions = kyc_manager.get_user_permissions("user_123");
    
    // Проверка конкретного разрешения
    if permissions.contains(&Permission::ManageUsers) {
        println!("User can manage users");
    }
    
    // Проверка роли
    if let Some(user) = kyc_manager.get_user("user_123") {
        // Проверка KYC статуса
        if user.kyc_status == KYCStatus::Verified {
            println!("User is KYC verified");
        }
    }
}
```

### Аудит ролей
```rust
async fn audit_role_changes() {
    let kyc_manager = KYCAmlManager::new();
    
    // Получение статистики ролей
    let stats = kyc_manager.get_kyc_statistics();
    
    println!("Total users: {}", stats.total_users);
    println!("Verified users: {}", stats.verified);
    println!("High risk users: {}", stats.high_risk);
    
    // Просмотр аудит логов
    let audit_logs = kyc_manager.get_audit_logs();
    for log in audit_logs {
        println!("Action: {} by user: {} at: {}", 
                 log.action, log.user_id, log.timestamp);
    }
}
```

---

## 🔄 Workflow управления ролями

### Процесс назначения роли
1. **Запрос роли**: Пользователь или администратор запрашивает назначение роли
2. **Проверка требований**: Система проверяет соответствие требованиям (KYC, опыт, etc.)
3. **Одобрение**: Соответствующий администратор одобряет запрос
4. **Назначение**: Роль назначается с указанием срока действия
5. **Уведомление**: Пользователь уведомляется о назначении роли
6. **Аудит**: Действие записывается в аудит лог

### Процесс отзыва роли
1. **Запрос отзыва**: Администратор инициирует отзыв роли
2. **Проверка зависимостей**: Система проверяет активные процессы пользователя
3. **Безопасное завершение**: Активные сессии завершаются безопасно
4. **Отзыв роли**: Роль отзывается из системы
5. **Уведомление**: Пользователь уведомляется об отзыве
6. **Аудит**: Действие записывается в аудит лог

### Процесс эскалации прав
1. **Запрос эскалации**: Пользователь запрашивает временные дополнительные права
2. **Обоснование**: Указывается причина и срок эскалации
3. **Одобрение**: Менеджер или администратор одобряет запрос
4. **Временное назначение**: Права назначаются на ограниченный срок
5. **Мониторинг**: Действия пользователя дополнительно мониторятся
6. **Автоматический отзыв**: Права автоматически отзываются по истечении срока

---

## 📊 Мониторинг и отчетность

### Метрики ролей
```rust
pub struct RoleMetrics {
    pub total_users: u32,
    pub users_by_role: HashMap<UserRole, u32>,
    pub active_sessions: u32,
    pub failed_login_attempts: u32,
    pub permission_denials: u32,
    pub role_changes_last_24h: u32,
}
```

### Отчеты по безопасности
- **Отчет по доступам**: Кто имеет доступ к каким ресурсам
- **Отчет по активности**: Активность пользователей по ролям
- **Отчет по нарушениям**: Попытки несанкционированного доступа
- **Отчет по соответствию**: Соответствие регуляторным требованиям

### Алерты безопасности
- **Подозрительная активность**: Необычные паттерны доступа
- **Эскалация привилегий**: Попытки получения дополнительных прав
- **Нарушение политик**: Действия, нарушающие корпоративные политики
- **KYC нарушения**: Проблемы с верификацией клиентов

---

## 🎯 Заключение

Ролевая модель The Hot Pot Spot обеспечивает:

- **Безопасность**: Четкое разделение прав и обязанностей
- **Соответствие**: Соответствие регуляторным требованиям
- **Гибкость**: Возможность настройки ролей под бизнес-потребности
- **Аудит**: Полное отслеживание всех действий пользователей
- **Масштабируемость**: Легкое добавление новых ролей и разрешений

Эта система ролей является основой для безопасной и эффективной работы всей платформы The Hot Pot Spot.
