//! KYC/AML Integration для The Hot Pot Spot
//! 
//! Обеспечивает соответствие регуляторным требованиям Грузии,
//! включая верификацию пользователей и управление ролями.

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc, Duration, Datelike};
use sha2::{Sha256, Digest};
use hex;

/// Статус KYC верификации
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KYCStatus {
    NotStarted,     // Не начата
    Pending,        // В процессе
    Verified,       // Верифицирован
    Rejected,       // Отклонен
    Expired,        // Истек
    Suspended,      // Приостановлен
}

/// Уровень KYC верификации
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KYCLevel {
    Basic,          // Базовая верификация
    Enhanced,       // Расширенная верификация
    Premium,        // Премиум верификация
}

/// Типы документов для KYC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentType {
    Passport,       // Паспорт
    IdCard,         // ID карта
    DriverLicense,  // Водительские права
    UtilityBill,    // Счет за коммунальные услуги
    BankStatement,  // Банковская выписка
    ProofOfAddress, // Подтверждение адреса
}

/// Статус документа
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentStatus {
    Uploaded,       // Загружен
    UnderReview,    // На рассмотрении
    Approved,       // Одобрен
    Rejected,       // Отклонен
    Expired,        // Истек
}

/// Структура документа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCDocument {
    pub document_id: String,
    pub document_type: DocumentType,
    pub file_hash: String,        // Хеш файла для проверки целостности
    pub file_path: String,        // Путь к файлу
    pub uploaded_at: DateTime<Utc>,
    pub status: DocumentStatus,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<String>, // ID верификатора
    pub rejection_reason: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
}

/// Роли пользователей (RBAC)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserRole {
    // Административные роли
    SuperAdmin,     // Супер администратор
    Admin,          // Администратор
    Compliance,     // Сотрудник комплаенса
    
    // Операционные роли
    MasterOwner,    // Владелец сети
    FranchiseOwner, // Владелец франшизы
    POSOperator,    // Оператор POS
    Cashier,        // Кассир
    
    // Пользовательские роли
    Customer,       // Покупатель
    Investor,       // Инвестор
    
    // Системные роли
    System,         // Системная роль
    Auditor,        // Аудитор
}

/// Разрешения (ACL)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    // Административные разрешения
    ManageUsers,            // Управление пользователями
    ManageRoles,            // Управление ролями
    ViewAuditLogs,          // Просмотр аудит логов
    ManageSystem,           // Управление системой
    
    // KYC/AML разрешения
    VerifyKYC,              // Верификация KYC
    ViewKYCData,            // Просмотр KYC данных
    ManageCompliance,       // Управление комплаенсом
    GenerateReports,        // Генерация отчетов
    
    // Операционные разрешения
    ProcessTransactions,    // Обработка транзакций
    ManageNodes,            // Управление нодами
    ViewFinancials,         // Просмотр финансов
    ManageMenu,             // Управление меню
    
    // Пользовательские разрешения
    CreateOrders,           // Создание заказов
    ViewOwnData,            // Просмотр собственных данных
    TransferTokens,         // Перевод токенов
    VoteOnProposals,        // Голосование по предложениям
}

/// Структура пользователя с KYC данными
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCUser {
    pub user_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub nationality: Option<String>,
    pub address: Option<Address>,
    pub kyc_status: KYCStatus,
    pub kyc_level: KYCLevel,
    pub kyc_started_at: Option<DateTime<Utc>>,
    pub kyc_completed_at: Option<DateTime<Utc>>,
    pub kyc_expires_at: Option<DateTime<Utc>>,
    pub documents: Vec<KYCDocument>,
    pub risk_score: u8,         // Оценка риска (0-100)
    pub sanctions_check: bool,  // Проверка санкций
    pub pep_status: bool,       // Статус PEP (Politically Exposed Person)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// Структура роли с разрешениями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub role_id: String,
    pub name: UserRole,
    pub permissions: HashSet<Permission>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Структура назначения роли пользователю
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleAssignment {
    pub assignment_id: String,
    pub user_id: String,
    pub role: UserRole,
    pub assigned_by: String,
    pub assigned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Менеджер KYC/AML системы
#[derive(Clone)]
pub struct KYCAmlManager {
    users: HashMap<String, KYCUser>,
    roles: HashMap<UserRole, Role>,
    role_assignments: HashMap<String, Vec<UserRoleAssignment>>,
    audit_log: Vec<AuditLogEntry>,
    kyc_providers: Vec<KYCProvider>,
}

/// Провайдер KYC услуг
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KYCProvider {
    pub provider_id: String,
    pub name: String,
    pub api_endpoint: String,
    pub api_key: String,
    pub is_active: bool,
    pub supported_countries: Vec<String>,
    pub supported_documents: Vec<DocumentType>,
}

/// Запись аудит лога
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub log_id: String,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub details: Option<String>,
}

impl KYCAmlManager {
    /// Создание нового менеджера KYC/AML
    pub fn new() -> Self {
        let mut manager = Self {
            users: HashMap::new(),
            roles: HashMap::new(),
            role_assignments: HashMap::new(),
            audit_log: Vec::new(),
            kyc_providers: Vec::new(),
        };
        
        // Инициализация стандартных ролей
        manager.initialize_default_roles();
        manager
    }

    /// Инициализация стандартных ролей
    fn initialize_default_roles(&mut self) {
        let roles = vec![
            Role {
                role_id: "super_admin".to_string(),
                name: UserRole::SuperAdmin,
                permissions: Permission::all_permissions(),
                description: "Супер администратор с полными правами".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Role {
                role_id: "admin".to_string(),
                name: UserRole::Admin,
                permissions: vec![
                    Permission::ManageUsers,
                    Permission::ViewKYCData,
                    Permission::ManageCompliance,
                    Permission::GenerateReports,
                    Permission::ViewAuditLogs,
                ].into_iter().collect(),
                description: "Администратор системы".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Role {
                role_id: "compliance".to_string(),
                name: UserRole::Compliance,
                permissions: vec![
                    Permission::VerifyKYC,
                    Permission::ViewKYCData,
                    Permission::ManageCompliance,
                    Permission::GenerateReports,
                ].into_iter().collect(),
                description: "Сотрудник комплаенса".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Role {
                role_id: "master_owner".to_string(),
                name: UserRole::MasterOwner,
                permissions: vec![
                    Permission::ManageNodes,
                    Permission::ViewFinancials,
                    Permission::ManageMenu,
                    Permission::ProcessTransactions,
                ].into_iter().collect(),
                description: "Владелец сети".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Role {
                role_id: "franchise_owner".to_string(),
                name: UserRole::FranchiseOwner,
                permissions: vec![
                    Permission::ViewFinancials,
                    Permission::ManageMenu,
                    Permission::ProcessTransactions,
                ].into_iter().collect(),
                description: "Владелец франшизы".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Role {
                role_id: "customer".to_string(),
                name: UserRole::Customer,
                permissions: vec![
                    Permission::CreateOrders,
                    Permission::ViewOwnData,
                    Permission::TransferTokens,
                    Permission::VoteOnProposals,
                ].into_iter().collect(),
                description: "Покупатель".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];

        for role in roles {
            self.roles.insert(role.name.clone(), role);
        }
    }

    /// Регистрация нового пользователя
    pub fn register_user(&mut self, user_data: UserRegistrationData) -> Result<String, KYCAmlError> {
        let user_id = self.generate_user_id(&user_data.email);
        
        let user = KYCUser {
            user_id: user_id.clone(),
            email: user_data.email,
            phone: user_data.phone,
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            date_of_birth: user_data.date_of_birth,
            nationality: user_data.nationality,
            address: user_data.address,
            kyc_status: KYCStatus::NotStarted,
            kyc_level: KYCLevel::Basic,
            kyc_started_at: None,
            kyc_completed_at: None,
            kyc_expires_at: None,
            documents: Vec::new(),
            risk_score: 0,
            sanctions_check: false,
            pep_status: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        };

        self.users.insert(user_id.clone(), user);
        
        // Назначаем базовую роль
        self.assign_role(&user_id, UserRole::Customer, "system".to_string(), None)?;
        
        self.log_audit_event(&user_id, "USER_REGISTERED", "user", true, Some("User registered successfully"));
        
        Ok(user_id)
    }

    /// Начало процесса KYC
    pub fn start_kyc_process(&mut self, user_id: &str, kyc_level: KYCLevel) -> Result<(), KYCAmlError> {
        let user = self.users.get_mut(user_id)
            .ok_or(KYCAmlError::UserNotFound)?;
        
        if user.kyc_status != KYCStatus::NotStarted {
            return Err(KYCAmlError::KYCAlreadyStarted);
        }

        user.kyc_status = KYCStatus::Pending;
        user.kyc_level = kyc_level.clone();
        user.kyc_started_at = Some(Utc::now());
        user.updated_at = Utc::now();

        let kyc_level_str = format!("{:?}", kyc_level);
        self.log_audit_event(user_id, "KYC_STARTED", "kyc", true, Some(&kyc_level_str));
        
        Ok(())
    }

    /// Загрузка документа для KYC
    pub fn upload_document(&mut self, user_id: &str, document_type: DocumentType, file_hash: String, file_path: String) -> Result<String, KYCAmlError> {
        // Генерируем document_id до получения mutable borrow
        let document_id = self.generate_document_id(user_id, &document_type);
        let document_type_str = format!("{:?}", document_type);
        
        let user = self.users.get_mut(user_id)
            .ok_or(KYCAmlError::UserNotFound)?;

        if user.kyc_status != KYCStatus::Pending {
            return Err(KYCAmlError::KYCNotInProgress);
        }
        
        let document = KYCDocument {
            document_id: document_id.clone(),
            document_type: document_type.clone(),
            file_hash,
            file_path,
            uploaded_at: Utc::now(),
            status: DocumentStatus::Uploaded,
            verified_at: None,
            verified_by: None,
            rejection_reason: None,
            expiry_date: Some(Utc::now() + Duration::days(365)), // Документы действительны 1 год
        };

        user.documents.push(document);
        user.updated_at = Utc::now();

        self.log_audit_event(user_id, "DOCUMENT_UPLOADED", "document", true, Some(&document_type_str));
        
        Ok(document_id)
    }

    /// Верификация документа
    pub fn verify_document(&mut self, user_id: &str, document_id: &str, verified_by: &str, approved: bool, rejection_reason: Option<String>) -> Result<(), KYCAmlError> {
        let user = self.users.get_mut(user_id)
            .ok_or(KYCAmlError::UserNotFound)?;

        let document = user.documents.iter_mut()
            .find(|doc| doc.document_id == document_id)
            .ok_or(KYCAmlError::DocumentNotFound)?;

        if approved {
            document.status = DocumentStatus::Approved;
            document.verified_at = Some(Utc::now());
            document.verified_by = Some(verified_by.to_string());
            document.rejection_reason = None;
        } else {
            document.status = DocumentStatus::Rejected;
            document.rejection_reason = rejection_reason;
        }

        user.updated_at = Utc::now();

        self.log_audit_event(user_id, "DOCUMENT_VERIFIED", "document", approved, Some(&format!("Document: {}, Approved: {}", document_id, approved)));
        
        Ok(())
    }

    /// Завершение процесса KYC
    pub fn complete_kyc_process(&mut self, user_id: &str, verified_by: &str) -> Result<(), KYCAmlError> {
        let user = self.users.get_mut(user_id)
            .ok_or(KYCAmlError::UserNotFound)?;

        if user.kyc_status != KYCStatus::Pending {
            return Err(KYCAmlError::KYCNotInProgress);
        }

        // Проверяем, что все необходимые документы одобрены
        let kyc_level = user.kyc_level.clone();
        let approved_docs: HashSet<DocumentType> = user.documents.iter()
            .filter(|doc| doc.status == DocumentStatus::Approved)
            .map(|doc| doc.document_type.clone())
            .collect();

        // Освобождаем borrow пользователя
        drop(user);

        // Получаем required_docs после освобождения borrow
        let required_docs = self.get_required_documents(&kyc_level);
        if !required_docs.iter().all(|doc_type| approved_docs.contains(doc_type)) {
            return Err(KYCAmlError::IncompleteDocuments);
        }

        // Выполняем проверки AML
        let mut user_clone = {
            let user = self.users.get(user_id).unwrap().clone();
            user
        };
        self.perform_aml_checks(&mut user_clone)?;

        // Обновляем пользователя
        let user = self.users.get_mut(user_id).unwrap();
        user.kyc_status = KYCStatus::Verified;
        user.kyc_completed_at = Some(Utc::now());
        user.kyc_expires_at = Some(Utc::now() + Duration::days(365)); // KYC действителен 1 год
        user.updated_at = Utc::now();

        self.log_audit_event(user_id, "KYC_COMPLETED", "kyc", true, Some("KYC verification completed successfully"));
        
        Ok(())
    }

    /// Выполнение AML проверок
    fn perform_aml_checks(&mut self, user: &mut KYCUser) -> Result<(), KYCAmlError> {
        // Проверка санкций (упрощенная версия)
        user.sanctions_check = self.check_sanctions(&user.first_name, &user.last_name, &user.nationality);
        
        // Проверка PEP статуса
        user.pep_status = self.check_pep_status(&user.first_name, &user.last_name, &user.nationality);
        
        // Расчет оценки риска
        user.risk_score = self.calculate_risk_score(user);
        
        Ok(())
    }

    /// Проверка санкций (упрощенная версия)
    fn check_sanctions(&self, first_name: &str, last_name: &str, nationality: &Option<String>) -> bool {
        // В реальной реализации здесь должен быть вызов API провайдера санкций
        // Пока возвращаем false (нет санкций)
        false
    }

    /// Проверка PEP статуса
    fn check_pep_status(&self, first_name: &str, last_name: &str, nationality: &Option<String>) -> bool {
        // В реальной реализации здесь должен быть вызов API провайдера PEP
        // Пока возвращаем false (не PEP)
        false
    }

    /// Расчет оценки риска
    fn calculate_risk_score(&self, user: &KYCUser) -> u8 {
        let mut score = 0u8;
        
        // Базовый риск
        score += 10;
        
        // Риск по национальности
        if let Some(nationality) = &user.nationality {
            if self.is_high_risk_country(nationality) {
                score += 30;
            }
        }
        
        // Риск по возрасту
        if let Some(dob) = user.date_of_birth {
            let age = Utc::now().year() - dob.year();
            if age < 18 || age > 80 {
                score += 20;
            }
        }
        
        // PEP статус
        if user.pep_status {
            score += 40;
        }
        
        // Санкции
        if user.sanctions_check {
            score += 50;
        }
        
        score.min(100)
    }

    /// Проверка высокорисковой страны
    fn is_high_risk_country(&self, nationality: &str) -> bool {
        let high_risk_countries = vec!["AF", "IR", "KP", "SY"]; // Упрощенный список
        high_risk_countries.contains(&nationality)
    }

    /// Получение необходимых документов для уровня KYC
    fn get_required_documents(&self, kyc_level: &KYCLevel) -> Vec<DocumentType> {
        match kyc_level {
            KYCLevel::Basic => vec![DocumentType::IdCard, DocumentType::ProofOfAddress],
            KYCLevel::Enhanced => vec![DocumentType::Passport, DocumentType::ProofOfAddress, DocumentType::BankStatement],
            KYCLevel::Premium => vec![DocumentType::Passport, DocumentType::ProofOfAddress, DocumentType::BankStatement, DocumentType::UtilityBill],
        }
    }

    /// Назначение роли пользователю
    pub fn assign_role(&mut self, user_id: &str, role: UserRole, assigned_by: String, expires_at: Option<DateTime<Utc>>) -> Result<(), KYCAmlError> {
        if !self.users.contains_key(user_id) {
            return Err(KYCAmlError::UserNotFound);
        }

        if !self.roles.contains_key(&role) {
            return Err(KYCAmlError::RoleNotFound);
        }

        let assignment = UserRoleAssignment {
            assignment_id: self.generate_assignment_id(user_id, &role),
            user_id: user_id.to_string(),
            role: role.clone(),
            assigned_by,
            assigned_at: Utc::now(),
            expires_at,
            is_active: true,
        };

        self.role_assignments.entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(assignment);

        self.log_audit_event(user_id, "ROLE_ASSIGNED", "role", true, Some(&format!("Role: {:?}", role)));
        
        Ok(())
    }

    /// Проверка разрешения пользователя
    pub fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        if let Some(assignments) = self.role_assignments.get(user_id) {
            for assignment in assignments {
                if assignment.is_active && (assignment.expires_at.is_none() || assignment.expires_at.unwrap() > Utc::now()) {
                    if let Some(role) = self.roles.get(&assignment.role) {
                        if role.permissions.contains(permission) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Получение всех разрешений пользователя
    pub fn get_user_permissions(&self, user_id: &str) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        
        if let Some(assignments) = self.role_assignments.get(user_id) {
            for assignment in assignments {
                if assignment.is_active && (assignment.expires_at.is_none() || assignment.expires_at.unwrap() > Utc::now()) {
                    if let Some(role) = self.roles.get(&assignment.role) {
                        permissions.extend(role.permissions.iter().cloned());
                    }
                }
            }
        }
        
        permissions
    }

    /// Получение информации о пользователе
    pub fn get_user(&self, user_id: &str) -> Option<&KYCUser> {
        self.users.get(user_id)
    }

    /// Получение статистики KYC
    pub fn get_kyc_statistics(&self) -> KYCStatistics {
        let mut stats = KYCStatistics::default();
        
        for user in self.users.values() {
            match user.kyc_status {
                KYCStatus::NotStarted => stats.not_started += 1,
                KYCStatus::Pending => stats.pending += 1,
                KYCStatus::Verified => stats.verified += 1,
                KYCStatus::Rejected => stats.rejected += 1,
                KYCStatus::Expired => stats.expired += 1,
                KYCStatus::Suspended => stats.suspended += 1,
            }
            
            if user.risk_score > 70 {
                stats.high_risk += 1;
            } else if user.risk_score > 30 {
                stats.medium_risk += 1;
            } else {
                stats.low_risk += 1;
            }
        }
        
        stats.total_users = self.users.len() as u32;
        stats.total_documents = self.users.values().map(|u| u.documents.len() as u32).sum();
        
        stats
    }

    /// Логирование аудит события
    fn log_audit_event(&mut self, user_id: &str, action: &str, resource: &str, success: bool, details: Option<&str>) {
        let log_entry = AuditLogEntry {
            log_id: self.generate_log_id(),
            user_id: user_id.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            timestamp: Utc::now(),
            ip_address: None,
            user_agent: None,
            success,
            details: details.map(|s| s.to_string()),
        };
        
        self.audit_log.push(log_entry);
    }

    /// Генерация ID пользователя
    fn generate_user_id(&self, email: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(email.as_bytes());
        hasher.update(Utc::now().timestamp().to_be_bytes());
        let hash = hasher.finalize();
        format!("user_{}", hex::encode(&hash[..8]))
    }

    /// Генерация ID документа
    fn generate_document_id(&self, user_id: &str, doc_type: &DocumentType) -> String {
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        hasher.update(format!("{:?}", doc_type).as_bytes());
        hasher.update(Utc::now().timestamp().to_be_bytes());
        let hash = hasher.finalize();
        format!("doc_{}", hex::encode(&hash[..8]))
    }

    /// Генерация ID назначения роли
    fn generate_assignment_id(&self, user_id: &str, role: &UserRole) -> String {
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        hasher.update(format!("{:?}", role).as_bytes());
        hasher.update(Utc::now().timestamp().to_be_bytes());
        let hash = hasher.finalize();
        format!("assign_{}", hex::encode(&hash[..8]))
    }

    /// Генерация ID лога
    fn generate_log_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(Utc::now().timestamp().to_be_bytes());
        hasher.update(fastrand::u64(..).to_be_bytes());
        let hash = hasher.finalize();
        format!("log_{}", hex::encode(&hash[..8]))
    }
}

/// Данные для регистрации пользователя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistrationData {
    pub email: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub nationality: Option<String>,
    pub address: Option<Address>,
}

/// Статистика KYC
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KYCStatistics {
    pub total_users: u32,
    pub total_documents: u32,
    pub not_started: u32,
    pub pending: u32,
    pub verified: u32,
    pub rejected: u32,
    pub expired: u32,
    pub suspended: u32,
    pub low_risk: u32,
    pub medium_risk: u32,
    pub high_risk: u32,
}

/// Ошибки KYC/AML системы
#[derive(Debug, thiserror::Error)]
pub enum KYCAmlError {
    #[error("User not found")]
    UserNotFound,
    
    #[error("Document not found")]
    DocumentNotFound,
    
    #[error("Role not found")]
    RoleNotFound,
    
    #[error("KYC already started")]
    KYCAlreadyStarted,
    
    #[error("KYC not in progress")]
    KYCNotInProgress,
    
    #[error("Incomplete documents")]
    IncompleteDocuments,
    
    #[error("Invalid document type")]
    InvalidDocumentType,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("User already exists")]
    UserAlreadyExists,
}

// Реализация для Permission
impl Permission {
    /// Получение всех разрешений
    pub fn all_permissions() -> HashSet<Permission> {
        vec![
            Permission::ManageUsers,
            Permission::ManageRoles,
            Permission::ViewAuditLogs,
            Permission::ManageSystem,
            Permission::VerifyKYC,
            Permission::ViewKYCData,
            Permission::ManageCompliance,
            Permission::GenerateReports,
            Permission::ProcessTransactions,
            Permission::ManageNodes,
            Permission::ViewFinancials,
            Permission::ManageMenu,
            Permission::CreateOrders,
            Permission::ViewOwnData,
            Permission::TransferTokens,
            Permission::VoteOnProposals,
        ].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_registration() {
        let mut manager = KYCAmlManager::new();
        
        let user_data = UserRegistrationData {
            email: "test@example.com".to_string(),
            phone: Some("+995123456789".to_string()),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: Some(Utc::now() - Duration::days(365 * 25)),
            nationality: Some("GE".to_string()),
            address: None,
        };
        
        let user_id = manager.register_user(user_data).unwrap();
        assert!(!user_id.is_empty());
        
        let user = manager.get_user(&user_id).unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.kyc_status, KYCStatus::NotStarted);
    }

    #[test]
    fn test_kyc_process() {
        let mut manager = KYCAmlManager::new();
        
        let user_data = UserRegistrationData {
            email: "test@example.com".to_string(),
            phone: None,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: None,
            nationality: None,
            address: None,
        };
        
        let user_id = manager.register_user(user_data).unwrap();
        
        // Начинаем KYC процесс
        manager.start_kyc_process(&user_id, KYCLevel::Basic).unwrap();
        
        let user = manager.get_user(&user_id).unwrap();
        assert_eq!(user.kyc_status, KYCStatus::Pending);
    }

    #[test]
    fn test_role_assignment() {
        let mut manager = KYCAmlManager::new();
        
        let user_data = UserRegistrationData {
            email: "admin@example.com".to_string(),
            phone: None,
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            date_of_birth: None,
            nationality: None,
            address: None,
        };
        
        let user_id = manager.register_user(user_data).unwrap();
        
        // Назначаем роль администратора
        manager.assign_role(&user_id, UserRole::Admin, "system".to_string(), None).unwrap();
        
        // Проверяем разрешения
        assert!(manager.has_permission(&user_id, &Permission::ManageUsers));
        assert!(manager.has_permission(&user_id, &Permission::ViewKYCData));
        assert!(!manager.has_permission(&user_id, &Permission::ManageSystem));
    }
}

