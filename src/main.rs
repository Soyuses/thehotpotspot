     #[cfg(test)]
use std::fmt;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
#[cfg(test)]
use qrcode::{QrCode, render::svg};
use hex;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// Импортируем конфигурацию валюты
use blockchain_project::config::{self, utils as config_utils};

// Импорты для модулей блокчейна
use blockchain_project::simple_server::SimpleServer;
use blockchain_project::web_server::WebServer;
use blockchain_project::franchise_network::{FranchiseNetwork, NodeType, SaleItem};
use blockchain_project::pos_api::PosApiServer;
use blockchain_project::consensus::{ConsensusAlgorithm, Block as ConsensusBlock, Transaction as ConsensusTransaction, TransactionType};
use blockchain_project::p2p_network::P2PNode;
use blockchain_project::ipfs_storage::IPFSStorage;

// Импорты для системы видеонаблюдения
use blockchain_project::video_surveillance::{
    VideoSurveillanceSystem, CameraConfig, CameraType, AnonymizationZone,
    StreamingConfig, StreamQuality
};
use blockchain_project::video_api::{VideoAPIHandler, VideoHTTPHandler};
use blockchain_project::streaming_integration::{
    TwitchConfig, YouTubeConfig
};
use blockchain_project::enhanced_web_server::EnhancedWebServer;

// Импорты для регуляторных экспортов
use blockchain_project::regulatory_exports::{
    RegulatoryExporter, ExportFormat, HolderRegistryEntry, EmissionRegistryEntry, 
    CorporateActionEntry, HolderUpdate
};

// Импорты для relayer сервиса
use blockchain_project::relayer_service::{
    RelayerService, RelayerConfig, RelayerTransactionRequest, RelayerResponse,
    RelayerError, TransactionStatus as RelayerTransactionStatus
};

// Импорты для HD wallet
use blockchain_project::hd_wallet::{
    HDWalletManager, HDWallet, CheckWallet, WalletType, WalletStatus,
    WalletStatistics, HDWalletError
};

// Импорты для KYC/AML
use blockchain_project::kyc_aml::{
    KYCAmlManager, KYCUser, KYCStatus, KYCLevel, DocumentType, DocumentStatus,
    UserRole, Permission, Role, UserRoleAssignment, KYCStatistics, KYCAmlError,
    UserRegistrationData, Address, AuditLogEntry
};

// Импорты для базы данных
use blockchain_project::database::{
    DatabaseManager, DatabaseConfig, DatabaseStats, CleanupStats, DatabaseError, UserData
};

// Импорты для observability
use blockchain_project::observability::{
    ObservabilityManager, ObservabilityConfig, LogLevel, MetricType, AlertSeverity, AlertStatus,
    ObservabilityStats, ErrorInfo
};

// Импорты для API versioning
use blockchain_project::api_versioning::{
    ApiVersionManager, ApiConfig, ApiVersion, VersionStatus, VersionInfo, VersionWarning,
    VersionStatistics, OpenApiSpec, ChangeType, ChangeImpact
};

// Utility Token for voting
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UtilityToken {
    symbol: String,
    total_supply: f64,
    voting_power_per_token: f64,
}

impl UtilityToken {
    fn new(symbol: String) -> Self {
        UtilityToken {
            symbol,
            total_supply: 0.0,
            voting_power_per_token: 1.0,
        }
    }

    fn issue_voting_tokens(&mut self, amount: f64) -> f64 {
        self.total_supply += amount;
        amount * self.voting_power_per_token
    }
}

// User Roles based on token holdings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum UserRole {
    Unauthorized,
    Starter,      // > 1% of total supply
    MiddlePlayer, // > 5% of total supply
    BigStack,     // > 10% of total supply
    MainOwner,    // Special role
}

impl UserRole {
    fn from_percentage(percentage: f64) -> Self {
        match percentage {
            p if p > 10.0 => UserRole::BigStack,
            p if p > 5.0 => UserRole::MiddlePlayer,
            p if p > 1.0 => UserRole::Starter,
            _ => UserRole::Unauthorized,
        }
    }
}

// Check structure for account activation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Check {
    check_id: String,
    qr_code: String,
    activation_code: String,
    amount_subunits: u128, // Сумма в subunits (1/100 GEL)
    currency: String, // Валюта (GEL)
    food_items: Vec<String>,
    timestamp: u64,
    is_activated: bool,
    blockchain_account: String,
    phone_number: Option<String>, // Номер телефона для авторизации
    is_claimed: bool, // Был ли чек уже использован для переноса баланса
}

impl Check {
    fn new(amount_gel: f64, food_items: Vec<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let amount_subunits = config_utils::gel_to_subunits(amount_gel);
        let check_id = Self::generate_check_id(amount_subunits, &food_items, timestamp);
        let activation_code = Self::generate_activation_code();
        let blockchain_account = Self::generate_blockchain_account();
        
        // Generate QR code data
        let qr_data = format!("{}|{}|{}", check_id, activation_code, blockchain_account);
        let qr_code = Self::generate_qr_code(&qr_data);
        
        Check {
            check_id,
            qr_code,
            activation_code,
            amount_subunits,
            currency: config::CURRENCY.to_string(),
            food_items,
            timestamp,
            is_activated: false,
            blockchain_account,
            phone_number: None,
            is_claimed: false,
        }
    }

    fn new_with_phone(amount: f64, food_items: Vec<String>, phone_number: String) -> Self {
        let mut check = Self::new(amount, food_items);
        check.phone_number = Some(phone_number);
        check
    }

    fn generate_check_id(amount_subunits: u128, food_items: &[String], timestamp: u64) -> String {
        let data = format!("{}{}{}", amount_subunits, food_items.join(""), timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(&hasher.finalize()[..16])
    }

    fn generate_activation_code() -> String {
        let mut rng = fastrand::Rng::new();
        format!("{:06}", rng.u32(100000..999999))
    }

    fn generate_blockchain_account() -> String {
        let mut rng = fastrand::Rng::new();
        let mut account = String::new();
        for _ in 0..8 {
            account.push_str(&format!("{:x}", rng.u32(0..16)));
        }
        format!("0x{}", account)
    }

    fn generate_qr_code(data: &str) -> String {
        // In a real implementation, this would generate an actual QR code
        // For now, we'll return a placeholder
        format!("QR_CODE_{}", data)
    }
}

// Blockchain Account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockchainAccount {
    address: String,
    status: AccountStatus,
    security_tokens: u128, // в subunits
    utility_tokens: u128, // в subunits
    personal_data: Option<PersonalData>,
    created_timestamp: u64,
    activated_timestamp: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum AccountStatus {
    Sleep,      // 100% tokens go to owner
    Active,     // Normal operation
    ForSale,    // Account is listed for sale
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonalData {
    name: String,
    email: String,
    phone: String,
    wallet_address: Option<String>,
}

impl BlockchainAccount {
    fn new(address: String) -> Self {
        BlockchainAccount {
            address,
            status: AccountStatus::Sleep,
            security_tokens: 0,
            utility_tokens: 0,
            personal_data: None,
            created_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            activated_timestamp: None,
        }
    }

    fn activate(&mut self, personal_data: PersonalData) -> Result<(), String> {
        if self.status != AccountStatus::Sleep {
            return Err("Account is not in sleep status".to_string());
        }
        
        self.status = AccountStatus::Active;
        self.personal_data = Some(personal_data);
        self.activated_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(())
    }

    fn list_for_sale(&mut self) -> Result<(), String> {
        if self.status != AccountStatus::Active {
            return Err("Only active accounts can be listed for sale".to_string());
        }
        self.status = AccountStatus::ForSale;
        Ok(())
    }
}

// Authorized User with phone verification
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthorizedUser {
    phone_number: String,
    wallet_address: String,
    verification_code: String,
    is_verified: bool,
    created_timestamp: u64,
    last_login_timestamp: Option<u64>,
}

impl AuthorizedUser {
    fn new(phone_number: String, wallet_address: String) -> Self {
        let verification_code = Self::generate_verification_code();
        AuthorizedUser {
            phone_number,
            wallet_address,
            verification_code,
            is_verified: false,
            created_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login_timestamp: None,
        }
    }

    fn generate_verification_code() -> String {
        let mut rng = fastrand::Rng::new();
        format!("{:06}", rng.u32(100000..999999))
    }

    fn verify(&mut self, code: &str) -> Result<(), String> {
        if self.verification_code == code {
            self.is_verified = true;
            self.last_login_timestamp = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            Ok(())
        } else {
            Err("Invalid verification code".to_string())
        }
    }
}

// Charity Fund for owner's family
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CharityFund {
    fund_id: String,
    fund_name: String,
    owner_family: String,
    total_donations: u128, // в subunits
    created_timestamp: u64,
    is_active: bool,
}

impl CharityFund {
    fn new(owner_family: String) -> Self {
        let fund_id = format!("CHARITY_{}", hex::encode(&owner_family.as_bytes()));
        CharityFund {
            fund_id,
            fund_name: format!("Благотворительный фонд семьи {}", owner_family),
            owner_family,
            total_donations: 0,
            created_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            is_active: true,
        }
    }

    fn add_donation(&mut self, amount: f64) {
        self.total_donations += amount;
    }
}

// Enhanced Token Holder with roles
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenHolder {
    address: String,
    security_tokens: u128, // в subunits
    utility_tokens: u128, // в subunits
    role: UserRole,
    is_main_owner: bool,
    is_charity_fund: bool, // Является ли благотворительным фондом
    is_franchise_owner: bool, // Является ли владельцем франшизы
    checks: Vec<Check>,
    blockchain_accounts: HashMap<String, BlockchainAccount>,
    phone_number: Option<String>, // Номер телефона для авторизации
    is_authorized: bool, // Авторизован ли пользователь по телефону
    franchise_nodes: Vec<String>, // Список нод франшизы (если применимо)
}

impl TokenHolder {
    fn new(address: String, is_main_owner: bool) -> Self {
        TokenHolder {
            address,
            security_tokens: 0,
            utility_tokens: 0,
            role: if is_main_owner { UserRole::MainOwner } else { UserRole::Unauthorized },
            is_main_owner,
            is_charity_fund: false,
            is_franchise_owner: false,
            checks: vec![],
            blockchain_accounts: HashMap::new(),
            phone_number: None,
            is_authorized: false,
            franchise_nodes: vec![],
        }
    }

    fn new_charity_fund(address: String, fund_name: String) -> Self {
        TokenHolder {
            address,
            security_tokens: 0,
            utility_tokens: 0,
            role: UserRole::MainOwner, // Благотворительный фонд имеет особый статус
            is_main_owner: false,
            is_charity_fund: true,
            is_franchise_owner: false,
            checks: vec![],
            blockchain_accounts: HashMap::new(),
            phone_number: None,
            is_authorized: true, // Фонд всегда авторизован
            franchise_nodes: vec![],
        }
    }

    fn new_franchise_owner(address: String, franchise_nodes: Vec<String>) -> Self {
        TokenHolder {
            address,
            security_tokens: 0,
            utility_tokens: 0,
            role: UserRole::Unauthorized,
            is_main_owner: false,
            is_charity_fund: false,
            is_franchise_owner: true,
            checks: vec![],
            blockchain_accounts: HashMap::new(),
            phone_number: None,
            is_authorized: false,
            franchise_nodes,
        }
    }

    fn authorize_with_phone(&mut self, phone_number: String) {
        self.phone_number = Some(phone_number);
        self.is_authorized = true;
    }

    fn add_security_tokens(&mut self, amount: u128) {
        self.security_tokens += amount;
        self.update_role();
    }

    fn add_utility_tokens(&mut self, amount: u128) {
        self.utility_tokens += amount;
    }

    fn update_role(&mut self) {
        if self.is_main_owner {
            self.role = UserRole::MainOwner;
        } else {
            // Role will be calculated based on percentage of total supply
            // This will be updated by the blockchain
        }
    }

    fn add_check(&mut self, check: Check) {
        self.checks.push(check.clone());
        // Add blockchain account to holder's accounts
        let account = BlockchainAccount::new(check.blockchain_account.clone());
        self.blockchain_accounts.insert(check.blockchain_account.clone(), account);
    }

    fn activate_account(&mut self, check_id: &str, activation_code: &str, personal_data: PersonalData) -> Result<(), String> {
        // Find the check
        if let Some(check) = self.checks.iter_mut().find(|c| c.check_id == check_id) {
            if check.activation_code != activation_code {
                return Err("Invalid activation code".to_string());
            }
            if check.is_activated {
                return Err("Check already activated".to_string());
            }
            
            check.is_activated = true;
            
            // Activate the blockchain account
            if let Some(account) = self.blockchain_accounts.get_mut(&check.blockchain_account) {
                account.activate(personal_data)?;
            }
            
            Ok(())
        } else {
            Err("Check not found".to_string())
        }
    }
}

// Ингредиент с количеством и калориями
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ingredient {
    name: String,
    amount_grams: f64,
    calories: f64,
}

// Позиция меню с полной информацией
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    name: String,
    description: String,
    price_subunits: u128, // Цена в subunits (1/100 GEL)
    currency: String, // Валюта (GEL)
    availability: u32, // количество доступных штук
    priority_rank: u32, // приоритетность (1-10, где 10 - высший)
    cooking_time_minutes: u32, // время готовки в минутах
    ingredients: Vec<Ingredient>,
    total_calories: f64,
    suggested_by: String,
    votes_for: f64,
    votes_against: f64,
    status: MenuItemStatus,
    created_timestamp: u64,
    voting_ends: u64,
    is_available_for_voting: bool, // доступно ли для голосования
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum MenuItemStatus {
    Proposed,
    Voting,
    Approved,
    Rejected,
    Active,
}

// Заказ блюда
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    menu_item_id: String,
    quantity: u32,
}

// Статус заказа
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum OrderStatus {
    Pending,      // на рассмотрении
    Confirmed,    // подтвержден
    Cancelled,    // отменен
    Completed,    // выполнен
}

// Заказ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    id: String,
    customer_wallet: String,
    items: Vec<OrderItem>,
    total_amount_subunits: u128, // Сумма в subunits (1/100 GEL)
    currency: String, // Валюта (GEL)
    delivery_time_minutes: u32, // когда может приехать курьер
    status: OrderStatus,
    created_timestamp: u64,
    confirmed_timestamp: Option<u64>,
    cancellation_reason: Option<String>,
    tokens_issued_subunits: u128, // количество токенов в subunits, выданных за заказ
}

#[cfg_attr(test, allow(dead_code))]
impl MenuItem {
    fn new(name: String, description: String, price_subunits: u128, suggested_by: String, voting_duration_days: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        MenuItem {
            id: Self::generate_id(&name, &suggested_by, timestamp),
            name,
            description,
            price_subunits,
            currency: config::CURRENCY.to_string(),
            availability: 0,
            priority_rank: 5, // средний приоритет по умолчанию
            cooking_time_minutes: 15, // 15 минут по умолчанию
            ingredients: vec![],
            total_calories: 0.0,
            suggested_by,
            votes_for: 0.0,
            votes_against: 0.0,
            status: MenuItemStatus::Proposed,
            created_timestamp: timestamp,
            voting_ends: timestamp + (voting_duration_days * 24 * 60 * 60),
            is_available_for_voting: false,
        }
    }

    fn new_with_details(
        name: String, 
        description: String, 
        price_subunits: u128, // Цена в subunits (1/100 GEL)
        availability: u32,
        priority_rank: u32,
        cooking_time_minutes: u32,
        ingredients: Vec<Ingredient>,
        suggested_by: String, 
        voting_duration_days: u64
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let total_calories: f64 = ingredients.iter().map(|i| i.calories).sum();
        
        MenuItem {
            id: Self::generate_id(&name, &suggested_by, timestamp),
            name,
            description,
            price_subunits,
            currency: config::CURRENCY.to_string(),
            availability,
            priority_rank,
            cooking_time_minutes,
            ingredients,
            total_calories,
            suggested_by,
            votes_for: 0.0,
            votes_against: 0.0,
            status: MenuItemStatus::Proposed,
            created_timestamp: timestamp,
            voting_ends: timestamp + (voting_duration_days * 24 * 60 * 60),
            is_available_for_voting: false,
        }
    }

    fn generate_id(name: &str, suggested_by: &str, timestamp: u64) -> String {
        let data = format!("{}{}{}", name, suggested_by, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(&hasher.finalize()[..8])
    }

    fn start_voting(&mut self) {
        self.status = MenuItemStatus::Voting;
    }

    fn vote(&mut self, utility_tokens: f64, vote_for: bool) -> Result<(), String> {
        if self.status != MenuItemStatus::Voting {
            return Err("Voting is not active for this item".to_string());
        }
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if current_time > self.voting_ends {
            return Err("Voting period has ended".to_string());
        }
        
        if vote_for {
            self.votes_for += utility_tokens;
        } else {
            self.votes_against += utility_tokens;
        }
        Ok(())
    }

    fn finalize_vote(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if current_time > self.voting_ends {
            if self.votes_for > self.votes_against {
                self.status = MenuItemStatus::Approved;
            } else {
                self.status = MenuItemStatus::Rejected;
            }
        }
    }

    fn make_available_for_voting(&mut self) {
        self.is_available_for_voting = true;
        self.status = MenuItemStatus::Voting;
    }
}

impl Order {
    fn new(customer_wallet: String, items: Vec<OrderItem>, delivery_time_minutes: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Временная заглушка - будет пересчитано в create_order
        let total_amount_subunits = 0u128;
        
        Order {
            id: Self::generate_order_id(&customer_wallet, timestamp),
            customer_wallet,
            items,
            total_amount_subunits,
            currency: config::CURRENCY.to_string(),
            delivery_time_minutes,
            status: OrderStatus::Pending,
            created_timestamp: timestamp,
            confirmed_timestamp: None,
            cancellation_reason: None,
            tokens_issued_subunits: 0,
        }
    }

    fn generate_order_id(customer_wallet: &str, timestamp: u64) -> String {
        let data = format!("{}{}", customer_wallet, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("ORDER_{}", hex::encode(&hasher.finalize()[..8]))
    }

    fn confirm(&mut self, tokens_issued: f64) {
        self.status = OrderStatus::Confirmed;
        self.confirmed_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        self.tokens_issued_subunits = tokens_issued as u128;
    }

    fn cancel(&mut self, reason: String) {
        self.status = OrderStatus::Cancelled;
        self.cancellation_reason = Some(reason);
    }
}

// Transaction with check generation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount_subunits: u128, // Сумма в subunits (1/100 GEL)
    currency: String, // Валюта (GEL)
    food_items: Vec<String>,
    timestamp: u64,
    transaction_id: String,
    check: Option<Check>,
    security_tokens_issued_subunits: u128, // Security токены в subunits
    utility_tokens_issued_subunits: u128, // Utility токены в subunits
}

impl Transaction {
    fn new(from: String, to: String, amount_subunits: u128, food_items: Vec<String>, 
           security_tokens: u128, utility_tokens: u128) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let transaction_id = Self::generate_transaction_id(&from, &to, amount_subunits, &food_items, timestamp);
        
        // Generate check for the transaction
        let check = Check::new(config::utils::subunits_to_gel(amount_subunits), food_items.clone());
        
        Transaction {
            from,
            to,
            amount_subunits,
            currency: config::CURRENCY.to_string(),
            food_items,
            timestamp,
            transaction_id,
            check: Some(check),
            security_tokens_issued_subunits: security_tokens as u128,
            utility_tokens_issued_subunits: utility_tokens as u128,
        }
    }

    fn generate_transaction_id(from: &str, to: &str, amount_subunits: u128, food_items: &[String], timestamp: u64) -> String {
        let data = format!("{}{}{}{}{}", from, to, amount_subunits, food_items.join(""), timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}

// Enhanced Blockchain with new token distribution rules
#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    token_holders: HashMap<String, TokenHolder>,
    pending_transactions: Vec<Transaction>,
    utility_token: UtilityToken,
    pub menu_items: Vec<MenuItem>,
    pub orders: Vec<Order>,
    #[allow(dead_code)]
    smart_contracts: Vec<SmartContract>,
    voting_history: Vec<VotingRecord>,
    blockchain_history: Vec<BlockchainOrderRecord>,
    authorized_users: HashMap<String, AuthorizedUser>, // phone_number -> AuthorizedUser
    balance_transfer_history: Vec<BalanceTransferRecord>,
    charity_fund: CharityFund, // Благотворительный фонд семьи владельца
    main_owner: String,
    difficulty: usize,
    min_stake: f64,
    block_reward: f64,
    // Новые ограничения на владение токенами
    max_owner_percentage: f64, // Максимум 48% для владельца
    max_franchise_percentage: f64, // Максимум 24% для владельцев франшиз (все вместе)
    max_customer_percentage: f64, // Максимум 49% для покупателей
    charity_percentage: f64, // 3% для благотворительного фонда
    franchise_nodes: HashMap<String, String>, // node_id -> franchise_owner_address
    monitoring_alerts: Vec<MonitoringAlert>, // Система мониторинга
    unclaimed_tokens: Vec<UnclaimedTokensRecord>, // Невостребованные токены
    annual_distributions: Vec<AnnualDistribution>, // История годовых распределений
    current_year: u32, // Текущий год для отслеживания
    regulatory_exporter: RegulatoryExporter, // Регуляторные экспорты
    relayer_service: RelayerService, // Relayer сервис для обработки транзакций
    hd_wallet_manager: HDWalletManager, // HD wallet менеджер
    kyc_aml_manager: KYCAmlManager, // KYC/AML менеджер
    database_manager: Option<DatabaseManager>, // Database менеджер (опциональный для совместимости)
    observability_manager: ObservabilityManager, // Observability менеджер
    api_version_manager: ApiVersionManager, // API versioning менеджер
}

#[cfg_attr(test, allow(dead_code))]
impl Blockchain {
    fn new(main_owner: String) -> Self {
        let genesis_block = Block::new(
            0,
            vec![],
            "0".to_string(),
            "Genesis".to_string(),
            0.0,
        );
        
        let utility_token = UtilityToken::new("VOTE".to_string());
        
        // Создаем благотворительный фонд семьи владельца
        let charity_fund = CharityFund::new(main_owner.clone());
        let charity_address = charity_fund.fund_id.clone();
        
        let mut token_holders = HashMap::new();
        token_holders.insert(main_owner.clone(), TokenHolder::new(main_owner.clone(), true));
        token_holders.insert(charity_address.clone(), TokenHolder::new_charity_fund(charity_address, charity_fund.fund_name.clone()));
        
        Blockchain {
            chain: vec![genesis_block],
            token_holders,
            pending_transactions: vec![],
            utility_token,
            menu_items: vec![],
            orders: vec![],
            smart_contracts: vec![],
            voting_history: vec![],
            blockchain_history: vec![],
            authorized_users: HashMap::new(),
            balance_transfer_history: vec![],
            charity_fund,
            main_owner,
            difficulty: 4,
            min_stake: 10.0,
            block_reward: 5.0,
            // Новые ограничения
            max_owner_percentage: 48.0, // Максимум 48% для владельца сети
            max_franchise_percentage: 24.0, // Максимум 24% для владельцев франшиз (все вместе)
            max_customer_percentage: 49.0, // Максимум 49% для покупателей
            charity_percentage: 3.0, // 3% для благотворительного фонда
            franchise_nodes: HashMap::new(),
            monitoring_alerts: vec![],
            unclaimed_tokens: vec![],
            annual_distributions: vec![],
            current_year: 2024, // Текущий год
            regulatory_exporter: RegulatoryExporter::new(),
            relayer_service: RelayerService::new(RelayerConfig::default()),
            hd_wallet_manager: HDWalletManager::new("thehotpotspot_master_seed_2024".to_string()),
            kyc_aml_manager: KYCAmlManager::new(),
            database_manager: None, // Инициализируется отдельно при необходимости
            observability_manager: ObservabilityManager::new(ObservabilityConfig::default()),
            api_version_manager: ApiVersionManager::new(ApiConfig::default()),
        }
    }

    fn process_purchase(&mut self, customer: String, food_truck: String, amount_subunits: u128, food_items: Vec<String>) -> Check {
        // Новая логика распределения токенов (1:1 utility токены):
        // Нода владельца сети: 48% владелец сети, 3% фонд, 49% покупатель
        // Нода франчайзи: 25% владелец сети, 24% франчайзи, 3% фонд, 49% покупатель
        // Utility токены: 1:1 с security токенами для всех участников
        
        let is_franchise_node = self.franchise_nodes.contains_key(&food_truck);
        
        let (main_owner_tokens, franchise_owner_tokens, charity_tokens, customer_tokens) = if is_franchise_node {
            // Для франшизной ноды: 25% + 24% + 3% + 48% = 100%
            let main_owner_tokens = config::utils::calculate_percentage(amount_subunits, 25); // 25% владельцу сети
            let franchise_owner_tokens = config::utils::calculate_percentage(amount_subunits, 24); // 24% владельцу франшизы
            let charity_tokens = config::utils::calculate_percentage(amount_subunits, 3); // 3% фонду
            let customer_tokens = config::utils::calculate_percentage(amount_subunits, 48); // 48% покупателю
            (main_owner_tokens, franchise_owner_tokens, charity_tokens, customer_tokens)
        } else {
            // Для ноды владельца сети: 48% + 3% + 49% = 100%
            let main_owner_tokens = config::utils::calculate_percentage(amount_subunits, 48); // 48% владельцу сети
            let franchise_owner_tokens = 0; // 0% для франшизы
            let charity_tokens = config::utils::calculate_percentage(amount_subunits, 3); // 3% фонду
            let customer_tokens = config::utils::calculate_percentage(amount_subunits, 49); // 49% покупателю
            (main_owner_tokens, franchise_owner_tokens, charity_tokens, customer_tokens)
        };
        
        let utility_tokens = amount_subunits; // 1:1 utility токенов для голосования (100%)
        
        // Create transaction with check
        let transaction = Transaction::new(
            customer.clone(),
            food_truck.clone(),
            amount_subunits,
            food_items.clone(),
            customer_tokens, // Покупатель получает свою долю
            utility_tokens,
        );
        
        let check = transaction.check.as_ref().unwrap().clone();
        
        // Распределяем токены согласно новым правилам
        
        // 1. Владелец сети получает свою долю (48% для своих нод, 25% для франшизных)
        if !self.token_holders.contains_key(&self.main_owner) {
            let mut new_holder = TokenHolder::new(self.main_owner.clone(), true);
            new_holder.add_security_tokens(main_owner_tokens);
            new_holder.add_check(check.clone());
            self.token_holders.insert(self.main_owner.clone(), new_holder);
        } else {
            if let Some(holder) = self.token_holders.get_mut(&self.main_owner) {
                holder.add_security_tokens(main_owner_tokens);
                holder.add_check(check.clone());
            }
        }
        
        // 2. Владелец франшизы получает свою долю (только для франшизных нод)
        if is_franchise_node && franchise_owner_tokens > 0 {
            let franchise_owner = self.franchise_nodes.get(&food_truck).unwrap().clone();
            if !self.token_holders.contains_key(&franchise_owner) {
                let mut new_holder = TokenHolder::new_franchise_owner(franchise_owner.clone(), vec![food_truck.clone()]);
                new_holder.add_security_tokens(franchise_owner_tokens);
                self.token_holders.insert(franchise_owner.clone(), new_holder);
            } else {
                if let Some(holder) = self.token_holders.get_mut(&franchise_owner) {
                    holder.add_security_tokens(franchise_owner_tokens);
                }
            }
        }
        
        // 3. Благотворительный фонд получает 3%
        let charity_address = self.charity_fund.fund_id.clone();
        if let Some(charity_holder) = self.token_holders.get_mut(&charity_address) {
            charity_holder.add_security_tokens(charity_tokens);
        }
        self.charity_fund.add_donation(charity_tokens);
        
        // 4. Покупатель получает свою долю
        if !self.token_holders.contains_key(&customer) {
            let mut new_holder = TokenHolder::new(customer.clone(), false);
            new_holder.add_security_tokens(customer_tokens);
            self.token_holders.insert(customer.clone(), new_holder);
        } else {
            if let Some(holder) = self.token_holders.get_mut(&customer) {
                holder.add_security_tokens(customer_tokens);
            }
        }
        
        // Issue utility tokens for voting
        let voting_power = self.utility_token.issue_voting_tokens(utility_tokens);
        
        // Utility токены распределяются пропорционально security токенам
        let total_security = main_owner_tokens + franchise_owner_tokens + charity_tokens + customer_tokens;
        let main_owner_utility = (main_owner_tokens / total_security) * voting_power;
        let franchise_owner_utility = (franchise_owner_tokens / total_security) * voting_power;
        let charity_utility = (charity_tokens / total_security) * voting_power;
        let customer_utility = (customer_tokens / total_security) * voting_power;
        
        // Добавляем utility токены
        if let Some(main_owner_holder) = self.token_holders.get_mut(&self.main_owner) {
            main_owner_holder.add_utility_tokens(main_owner_utility);
        }
        
        if is_franchise_node && franchise_owner_tokens > 0.0 {
            let franchise_owner = self.franchise_nodes.get(&food_truck).unwrap().clone();
            if let Some(franchise_holder) = self.token_holders.get_mut(&franchise_owner) {
                franchise_holder.add_utility_tokens(franchise_owner_utility);
            }
        }
        
        if let Some(charity_holder) = self.token_holders.get_mut(&charity_address) {
            charity_holder.add_utility_tokens(charity_utility);
        }
        
        if let Some(customer_holder) = self.token_holders.get_mut(&customer) {
            customer_holder.add_utility_tokens(customer_utility);
        }
        
        // Проверяем ограничения и создаем алерты
        self.check_token_limits_and_create_alerts();
        
        // Добавляем запись о невостребованных токенах (если покупатель не зарегистрирован)
        // Проверяем, что покупатель не зарегистрирован в системе
        let is_customer_registered = self.authorized_users.values()
            .any(|user| user.wallet_address == customer);
        if !is_customer_registered {
            let expiry_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + (365 * 24 * 60 * 60); // 1 год до истечения
            
            let unclaimed_record = UnclaimedTokensRecord {
                check_id: check.check_id.clone(),
                amount: customer_tokens,
                created_timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                expiry_timestamp,
                is_distributed: false,
                distributed_timestamp: None,
            };
            
            self.unclaimed_tokens.push(unclaimed_record);
        }
        
        self.add_transaction(transaction);
        check
    }

    fn activate_account(&mut self, check_id: &str, activation_code: &str, personal_data: PersonalData) -> Result<(), String> {
        // Find the holder who has this check
        for holder in self.token_holders.values_mut() {
            if let Ok(()) = holder.activate_account(check_id, activation_code, personal_data.clone()) {
                // In a real system tokens would be transferred here. For now, success is enough.
                    return Ok(());
            }
        }
        Err("Check not found or invalid activation code".to_string())
    }

    fn suggest_menu_item(&mut self, name: String, description: String, price_subunits: u128, suggested_by: String) -> Result<(), String> {
        // Only main owner and big stacks can suggest menu items
        if let Some(holder) = self.token_holders.get(&suggested_by) {
            if holder.role != UserRole::MainOwner && holder.role != UserRole::BigStack {
                return Err("Only main owner and big stacks can suggest menu items".to_string());
            }
        } else {
            return Err("Invalid suggester address".to_string());
        }
        
        let menu_item = MenuItem::new(name, description, price_subunits, suggested_by, 7); // 7 days voting
        self.menu_items.push(menu_item);
        Ok(())
    }

    fn add_menu_item_with_details(&mut self, name: String, description: String, price_subunits: u128, 
                                 availability: u32, priority_rank: u32, cooking_time_minutes: u32,
                                 ingredients: Vec<Ingredient>, suggested_by: String) -> Result<(), String> {
        // Only main owner can add detailed menu items
        if let Some(holder) = self.token_holders.get(&suggested_by) {
            if holder.role != UserRole::MainOwner {
                return Err("Only main owner can add detailed menu items".to_string());
            }
        } else {
            return Err("Invalid suggester address".to_string());
        }
        
        let menu_item = MenuItem::new_with_details(
            name, description, price_subunits, availability, priority_rank, 
            cooking_time_minutes, ingredients, suggested_by, 7
        );
        self.menu_items.push(menu_item);
        Ok(())
    }

    fn make_menu_item_available_for_voting(&mut self, menu_item_id: String) -> Result<(), String> {
        if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == menu_item_id) {
            menu_item.make_available_for_voting();
            Ok(())
        } else {
            Err("Menu item not found".to_string())
        }
    }

    fn create_order(&mut self, customer_wallet: String, items: Vec<OrderItem>, delivery_time_minutes: u32) -> Result<Order, String> {
        // Проверяем доступность товаров
        for order_item in &items {
            if let Some(menu_item) = self.menu_items.iter().find(|item| item.id == order_item.menu_item_id) {
                if menu_item.availability < order_item.quantity {
                    return Err(format!("Not enough {} available. Requested: {}, Available: {}", 
                        menu_item.name, order_item.quantity, menu_item.availability));
                }
            } else {
                return Err(format!("Menu item {} not found", order_item.menu_item_id));
            }
        }

        let mut order = Order::new(customer_wallet, items, delivery_time_minutes);
        
        // Рассчитываем правильную сумму заказа
        let mut total_amount_subunits = 0u128;
        for order_item in &order.items {
            if let Some(menu_item) = self.menu_items.iter().find(|item| item.id == order_item.menu_item_id) {
                total_amount_subunits += menu_item.price_subunits * order_item.quantity as u128;
            }
        }
        order.total_amount_subunits = total_amount_subunits;
        
        self.orders.push(order.clone());
        Ok(order)
    }

    pub fn confirm_order(&mut self, order_id: String) -> Result<(), String> {
        let idx = self.orders.iter().position(|o| o.id == order_id).ok_or("Order not found".to_string())?;
        let (security_tokens, utility_tokens, customer_wallet, items_clone);
        {
            let order = &self.orders[idx];
            if order.status != OrderStatus::Pending {
                return Err("Order is not pending".to_string());
            }
            security_tokens = order.total_amount_subunits as f64;
            utility_tokens = order.total_amount_subunits as f64;
            customer_wallet = order.customer_wallet.clone();
            items_clone = order.items.clone();
        }

        // update balances
        if let Some(holder) = self.token_holders.get_mut(&customer_wallet) {
                holder.add_security_tokens(security_tokens);
                holder.add_utility_tokens(utility_tokens);
            } else {
            let mut new_holder = TokenHolder::new(customer_wallet.clone(), false);
                new_holder.add_security_tokens(security_tokens);
                new_holder.add_utility_tokens(utility_tokens);
            self.token_holders.insert(customer_wallet.clone(), new_holder);
            }

        // update availability
        for order_item in &items_clone {
                if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == order_item.menu_item_id) {
                    menu_item.availability -= order_item.quantity;
                }
            }

        let total_tokens = security_tokens + utility_tokens;
        let (order_id_clone, order_created_ts, customer_wallet_clone, total_amount_clone, tokens_issued_clone, status_clone);
        {
            let order_mut = &mut self.orders[idx];
            order_mut.confirm(total_tokens);
            order_id_clone = order_mut.id.clone();
            order_created_ts = order_mut.created_timestamp;
            customer_wallet_clone = order_mut.customer_wallet.clone();
            total_amount_clone = order_mut.total_amount;
            tokens_issued_clone = order_mut.tokens_issued_subunits as f64;
            status_clone = match order_mut.status {
                OrderStatus::Pending => "Pending".to_string(),
                OrderStatus::Confirmed => "Confirmed".to_string(),
                OrderStatus::Cancelled => "Cancelled".to_string(),
                OrderStatus::Completed => "Completed".to_string(),
            };
        }
        self.blockchain_history.push(BlockchainOrderRecord {
            order_id: order_id_clone,
            customer_wallet: customer_wallet_clone,
            total_amount: total_amount_clone,
            tokens_issued: tokens_issued_clone,
            timestamp: order_created_ts,
            status: status_clone,
        });
            Ok(())
    }

    pub fn cancel_order(&mut self, order_id: String, reason: String) -> Result<(), String> {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return Err("Order is not pending".to_string());
            }
            order.cancel(reason);
            Ok(())
        } else {
            Err("Order not found".to_string())
        }
    }

    // Децентрализованные смарт-контракты
    fn create_purchase_contract(&mut self, customer: String, amount: f64) -> Result<String, String> {
        let conditions = ContractConditions {
            min_tokens_required: 0.0,
            expiration_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 3600 // 1 час
            ),
            required_participants: vec![customer.clone(), self.main_owner.clone()],
            auto_execute: true,
        };

        let contract = SmartContract::new(
            ContractType::PurchaseContract,
            customer.clone(),
            conditions
        );

        let contract_id = contract.contract_id.clone();
        self.smart_contracts.push(contract);
        Ok(contract_id)
    }

    fn create_voting_contract(&mut self, voter: String, menu_item_id: String) -> Result<String, String> {
        let conditions = ContractConditions {
            min_tokens_required: 1.0, // Минимум 1 utility токен
            expiration_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 86400 // 24 часа
            ),
            required_participants: vec![voter.clone()],
            auto_execute: false,
        };

        let contract = SmartContract::new(
            ContractType::VotingContract,
            voter.clone(),
            conditions
        );

        let contract_id = contract.contract_id.clone();
        self.smart_contracts.push(contract);
        Ok(contract_id)
    }

    fn execute_voting_contract(&mut self, contract_id: String, voter: String, vote_for: bool) -> Result<(), String> {
        if let Some(contract) = self.smart_contracts.iter_mut().find(|c| c.contract_id == contract_id) {
            // Проверяем баланс utility токенов
            if let Some(holder) = self.token_holders.get(&voter) {
                if holder.utility_tokens < contract.conditions.min_tokens_required {
                    return Err("Insufficient utility tokens for voting".to_string());
                }
            } else {
                return Err("Voter not found".to_string());
            }

            // Выполняем контракт
            let action = if vote_for { "vote_for" } else { "vote_against" };
            let tokens_used = contract.conditions.min_tokens_required;
            
            contract.execute(voter.clone(), action.to_string(), tokens_used)?;

            // Записываем в историю голосований
            let voting_record = VotingRecord {
                voter_wallet: voter.clone(),
                menu_item_id: "menu_item_from_contract".to_string(), // В реальной реализации это будет из контракта
                menu_item_name: "Menu Item".to_string(),
                vote_weight: tokens_used,
                vote_for,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            self.voting_history.push(voting_record);
            Ok(())
        } else {
            Err("Contract not found".to_string())
        }
    }

    // Методы для получения истории
    fn get_blockchain_history(&self, limit: Option<u32>) -> Vec<BlockchainOrderRecord> {
        let limit = limit.unwrap_or(1000);
        let mut history = self.blockchain_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history.truncate(limit as usize);
        history
    }

    fn get_voting_history(&self) -> Vec<VotingRecord> {
        let mut history = self.voting_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history
    }

    fn add_to_blockchain_history(&mut self, order: &Order) {
        let record = BlockchainOrderRecord {
            order_id: order.id.clone(),
            customer_wallet: order.customer_wallet.clone(),
            total_amount: order.total_amount_subunits as f64,
            tokens_issued: order.tokens_issued_subunits as f64,
            timestamp: order.created_timestamp,
            status: match order.status {
                OrderStatus::Pending => "Pending".to_string(),
                OrderStatus::Confirmed => "Confirmed".to_string(),
                OrderStatus::Cancelled => "Cancelled".to_string(),
                OrderStatus::Completed => "Completed".to_string(),
            },
        };
        self.blockchain_history.push(record);
    }

    fn vote_on_menu_item(&mut self, voter: String, menu_item_id: String, vote_for: bool) -> Result<(), String> {
        let voting_power = if let Some(holder) = self.token_holders.get(&voter) {
            holder.utility_tokens
        } else {
            return Err("Voter not found".to_string());
        };
        
        if voting_power <= 0.0 {
            return Err("No voting power available".to_string());
        }
        
        if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == menu_item_id) {
            menu_item.vote(voting_power, vote_for)
        } else {
            Err("Menu item not found".to_string())
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn select_validator(&self) -> Option<String> {
        if self.token_holders.is_empty() {
            return None;
        }

        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        if total_security_tokens == 0.0 {
            return None;
        }
        
        let mut rng = fastrand::Rng::new();
        let random_value = rng.f64() * total_security_tokens;
        
        let mut current_sum = 0.0;
        for (address, holder) in &self.token_holders {
            current_sum += holder.security_tokens;
            if random_value <= current_sum {
                return Some(address.clone());
            }
        }
        
        None
    }

    fn mine_block(&mut self) -> Result<(), String> {
        if self.pending_transactions.is_empty() {
            return Err("No pending transactions to mine".to_string());
        }

        let validator_address = self.select_validator()
            .ok_or("No validators available")?;
        
        let validator = self.token_holders.get(&validator_address)
            .ok_or("Validator not found")?;
        
        if validator.security_tokens < self.min_stake {
            return Err("Validator security tokens too low".to_string());
        }

        let transactions = self.pending_transactions.drain(..).collect();
        let prev_hash = self.chain.last().unwrap().hash.clone();
        
        let mut new_block = Block::new(
            self.chain.len() as u32,
            transactions,
            prev_hash,
            validator_address.clone(),
            validator.security_tokens,
        );

        // Add block reward before mining so hash includes it
        let reward_transaction = Transaction::new(
            "Blockchain".to_string(),
            validator_address.clone(),
            self.block_reward,
            vec!["Block Reward".to_string()],
            0.0,
            0.0,
        );
        new_block.transactions.push(reward_transaction);

        new_block.mine(self.difficulty);

        // Update validator rewards
        if let Some(validator) = self.token_holders.get_mut(&validator_address) {
            validator.add_security_tokens(self.block_reward);
        }

        self.chain.push(new_block);
        Ok(())
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.hash != current.calculate_hash() {
                return false;
            }
            
            if current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }

    fn update_roles(&mut self) {
        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        
        for holder in self.token_holders.values_mut() {
            if !holder.is_main_owner {
                let percentage = (holder.security_tokens / total_security_tokens) * 100.0;
                holder.role = UserRole::from_percentage(percentage);
            }
        }
    }

    // Авторизация пользователя по номеру телефона
    fn register_user_with_phone(&mut self, phone_number: String, wallet_address: String) -> Result<String, String> {
        if self.authorized_users.contains_key(&phone_number) {
            return Err("Phone number already registered".to_string());
        }

        let authorized_user = AuthorizedUser::new(phone_number.clone(), wallet_address.clone());
        let verification_code = authorized_user.verification_code.clone();
        
        self.authorized_users.insert(phone_number.clone(), authorized_user);
        
        // Создаем или обновляем TokenHolder
        if let Some(holder) = self.token_holders.get_mut(&wallet_address) {
            holder.authorize_with_phone(phone_number);
        } else {
            let mut holder = TokenHolder::new(wallet_address.clone(), false);
            holder.authorize_with_phone(phone_number);
            self.token_holders.insert(wallet_address, holder);
        }
        
        Ok(verification_code)
    }

    // Подтверждение номера телефона
    fn verify_phone_number(&mut self, phone_number: String, verification_code: String) -> Result<(), String> {
        if let Some(user) = self.authorized_users.get_mut(&phone_number) {
            user.verify(&verification_code)?;
            Ok(())
        } else {
            Err("Phone number not found".to_string())
        }
    }

    // Перенос баланса с неавторизованного кошелька на авторизованный
    fn transfer_balance_from_check(&mut self, check_id: String, to_phone_number: String) -> Result<String, String> {
        // Проверяем, что получатель авторизован
        let authorized_user = self.authorized_users.get(&to_phone_number)
            .ok_or("Phone number not authorized")?;
        
        if !authorized_user.is_verified {
            return Err("Phone number not verified".to_string());
        }

        // Находим чек в системе
        let mut found_check: Option<Check> = None;
        let mut from_wallet: Option<String> = None;
        
        for (wallet, holder) in &self.token_holders {
            if let Some(check) = holder.checks.iter().find(|c| c.check_id == check_id) {
                if check.is_claimed {
                    return Err("Check already claimed".to_string());
                }
                if check.is_activated {
                    return Err("Check already activated".to_string());
                }
                found_check = Some(check.clone());
                from_wallet = Some(wallet.clone());
                break;
            }
        }

        let check = found_check.ok_or("Check not found")?;
        let from_wallet = from_wallet.unwrap();

        // Проверяем ограничения на владение токенами
        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        let total_utility_tokens = self.utility_token.total_supply;
        
        let security_tokens_to_transfer = check.amount_subunits as f64;
        let utility_tokens_to_transfer = check.amount_subunits as f64;

        // Проверяем, не превысит ли перенос максимальную долю владения
        if let Some(to_holder) = self.token_holders.get(&authorized_user.wallet_address) {
            // Токены уже существуют в системе, поэтому общее количество не меняется
            let new_security_percentage = ((to_holder.security_tokens + security_tokens_to_transfer) / total_security_tokens) * 100.0;
            let new_utility_percentage = ((to_holder.utility_tokens + utility_tokens_to_transfer) / total_utility_tokens) * 100.0;
            
            // Определяем максимальный лимит в зависимости от роли пользователя
            let max_percentage = if to_holder.is_main_owner {
                self.max_owner_percentage
            } else if to_holder.is_franchise_owner {
                self.max_franchise_percentage
            } else {
                self.max_customer_percentage
            };
            
            if new_security_percentage > max_percentage + 0.01 || new_utility_percentage > max_percentage + 0.01 {
                return Err(format!("Transfer would exceed maximum ownership percentage of {}%", max_percentage));
            }
        }

        // Создаем запись о переносе
        let transfer_id = Self::generate_transfer_id(&check_id, &to_phone_number);
        let transfer_record = BalanceTransferRecord {
            transfer_id: transfer_id.clone(),
            from_check_id: check_id.clone(),
            from_wallet: from_wallet.clone(),
            to_wallet: authorized_user.wallet_address.clone(),
            to_phone: to_phone_number.clone(),
            security_tokens_transferred: security_tokens_to_transfer,
            utility_tokens_transferred: utility_tokens_to_transfer,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: TransferStatus::Pending,
        };

        // Выполняем перенос
        // Удаляем токены с исходного кошелька
        if let Some(from_holder) = self.token_holders.get_mut(&from_wallet) {
            from_holder.security_tokens -= security_tokens_to_transfer;
            from_holder.utility_tokens -= utility_tokens_to_transfer;
            
            // Помечаем чек как использованный
            if let Some(check) = from_holder.checks.iter_mut().find(|c| c.check_id == check_id) {
                check.is_claimed = true;
            }
        }

        // Добавляем токены на целевой кошелек
        if let Some(to_holder) = self.token_holders.get_mut(&authorized_user.wallet_address) {
            to_holder.add_security_tokens(security_tokens_to_transfer);
            to_holder.add_utility_tokens(utility_tokens_to_transfer);
        } else {
            let mut new_holder = TokenHolder::new(authorized_user.wallet_address.clone(), false);
            new_holder.authorize_with_phone(to_phone_number.clone());
            new_holder.add_security_tokens(security_tokens_to_transfer);
            new_holder.add_utility_tokens(utility_tokens_to_transfer);
            self.token_holders.insert(authorized_user.wallet_address.clone(), new_holder);
        }

        // Обновляем статус переноса
        let mut final_record = transfer_record;
        final_record.status = TransferStatus::Completed;
        self.balance_transfer_history.push(final_record);

        // Обновляем роли
        self.update_roles();

        Ok(transfer_id)
    }

    fn generate_transfer_id(check_id: &str, phone_number: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let data = format!("{}{}{}", check_id, phone_number, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("TRANSFER_{}", hex::encode(&hasher.finalize()[..8]))
    }

    // Получение истории переносов баланса
    fn get_balance_transfer_history(&self, limit: Option<u32>) -> Vec<BalanceTransferRecord> {
        let limit = limit.unwrap_or(100);
        let mut history = self.balance_transfer_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history.truncate(limit as usize);
        history
    }

    // Проверка ограничений токенов и создание алертов
    fn check_token_limits_and_create_alerts(&mut self) {
        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        let total_utility_tokens = self.utility_token.total_supply;
        
        // Собираем информацию о держателях токенов
        let mut alerts_to_create = Vec::new();
        
        for (address, holder) in &self.token_holders {
            let security_percentage = (holder.security_tokens / total_security_tokens) * 100.0;
            let utility_percentage = (holder.utility_tokens / total_utility_tokens) * 100.0;
            
            // Проверяем ограничения для владельца
            if holder.is_main_owner && security_percentage > self.max_owner_percentage + 0.01 {
                alerts_to_create.push((
                    AlertType::OwnerExceedsLimit,
                    AlertSeverity::Critical,
                    format!("Владелец превысил лимит: {:.2}% > {:.2}%", security_percentage, self.max_owner_percentage),
                    Some(address.clone()),
                    Some(security_percentage),
                ));
            }
            
            // Проверяем ограничения для владельцев франшиз
            if holder.is_franchise_owner && security_percentage > self.max_franchise_percentage + 0.01 {
                alerts_to_create.push((
                    AlertType::FranchiseExceedsLimit,
                    AlertSeverity::High,
                    format!("Владелец франшизы превысил лимит: {:.2}% > {:.2}%", security_percentage, self.max_franchise_percentage),
                    Some(address.clone()),
                    Some(security_percentage),
                ));
            }
            
            // Проверяем ограничения для покупателей
            if !holder.is_main_owner && !holder.is_charity_fund && !holder.is_franchise_owner && security_percentage > self.max_customer_percentage + 0.01 {
                alerts_to_create.push((
                    AlertType::CustomerExceedsLimit,
                    AlertSeverity::High,
                    format!("Покупатель превысил лимит: {:.2}% > {:.2}%", security_percentage, self.max_customer_percentage),
                    Some(address.clone()),
                    Some(security_percentage),
                ));
            }
            
            // Проверяем концентрацию utility токенов
            if utility_percentage > 30.0 {
                alerts_to_create.push((
                    AlertType::TokenConcentration,
                    AlertSeverity::Medium,
                    format!("Высокая концентрация utility токенов: {:.2}%", utility_percentage),
                    Some(address.clone()),
                    Some(utility_percentage),
                ));
            }
        }
        
        // Проверяем благотворительный фонд
        let charity_percentage = (self.charity_fund.total_donations / total_security_tokens) * 100.0;
        if charity_percentage < self.charity_percentage * 0.8 { // Если меньше 80% от ожидаемого
            alerts_to_create.push((
                AlertType::CharityFundLow,
                AlertSeverity::Low,
                format!("Благотворительный фонд получает меньше ожидаемого: {:.2}% < {:.2}%", charity_percentage, self.charity_percentage),
                Some(self.charity_fund.fund_id.clone()),
                Some(charity_percentage),
            ));
        }
        
        // Создаем алерты
        for (alert_type, severity, message, affected_wallet, percentage) in alerts_to_create {
            self.create_alert(alert_type, severity, message, affected_wallet, percentage);
        }
    }
    
    // Создание алерта мониторинга
    fn create_alert(&mut self, alert_type: AlertType, severity: AlertSeverity, message: String, affected_wallet: Option<String>, percentage: Option<f64>) {
        let alert_id = format!("ALERT_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        let alert = MonitoringAlert {
            alert_id,
            alert_type,
            severity,
            message,
            affected_wallet,
            percentage,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            is_resolved: false,
        };
        self.monitoring_alerts.push(alert);
    }
    
    // Добавление франшизной ноды
    fn add_franchise_node(&mut self, node_id: String, franchise_owner: String) -> Result<(), String> {
        if self.franchise_nodes.contains_key(&node_id) {
            return Err("Node already exists".to_string());
        }
        
        self.franchise_nodes.insert(node_id.clone(), franchise_owner.clone());
        
        // Создаем держателя токенов для владельца франшизы, если его еще нет
        if !self.token_holders.contains_key(&franchise_owner) {
            let mut franchise_holder = TokenHolder::new_franchise_owner(franchise_owner.clone(), vec![node_id]);
            self.token_holders.insert(franchise_owner, franchise_holder);
        } else {
            // Добавляем ноду к существующему владельцу франшизы
            if let Some(holder) = self.token_holders.get_mut(&franchise_owner) {
                holder.franchise_nodes.push(node_id);
            }
        }
        
        Ok(())
    }
    
    // Эмиссия токенов для привлечения китов-инвесторов (1:1 utility токены)
    fn emit_tokens_for_investors(&mut self, amount_gel: f64, investor_address: String) -> Result<(), String> {
        // Новая логика эмиссии для "китов": 48% владелец сети, 3% фонд, 49% инвестор
        // Utility токены: 1:1 с security токенами
        let main_owner_tokens = amount_gel * 0.48; // 48% владельцу сети
        let charity_tokens = amount_gel * 0.03; // 3% фонду
        let investor_tokens = amount_gel * 0.49; // 49% инвестору
        
        let utility_tokens = amount_gel * 1.0; // 1:1 utility токенов для голосования (100%)
        
        // Проверяем, что владелец не превысит лимит после эмиссии
        let current_owner_tokens = self.token_holders.get(&self.main_owner).map(|h| h.security_tokens).unwrap_or(0.0);
        let total_tokens: f64 = self.token_holders.values().map(|h| h.security_tokens).sum();
        let new_total = total_tokens + main_owner_tokens + charity_tokens + investor_tokens;
        let new_owner_percentage = ((current_owner_tokens + main_owner_tokens) / new_total) * 100.0;
        
        if new_owner_percentage > self.max_owner_percentage + 0.01 {
            return Err(format!("Эмиссия приведет к превышению лимита владельца: {:.2}% > {:.2}%", new_owner_percentage, self.max_owner_percentage));
        }
        
        // 1. Владелец сети получает 48%
        if !self.token_holders.contains_key(&self.main_owner) {
            let mut new_holder = TokenHolder::new(self.main_owner.clone(), true);
            new_holder.add_security_tokens(main_owner_tokens);
            self.token_holders.insert(self.main_owner.clone(), new_holder);
        } else {
            if let Some(holder) = self.token_holders.get_mut(&self.main_owner) {
                holder.add_security_tokens(main_owner_tokens);
            }
        }
        
        // 2. Благотворительный фонд получает 3%
        let charity_address = self.charity_fund.fund_id.clone();
        if let Some(charity_holder) = self.token_holders.get_mut(&charity_address) {
            charity_holder.add_security_tokens(charity_tokens);
        }
        self.charity_fund.add_donation(charity_tokens);
        
        // 3. Инвестор получает 49%
        if !self.token_holders.contains_key(&investor_address) {
            let mut investor_holder = TokenHolder::new(investor_address.clone(), false);
            investor_holder.add_security_tokens(investor_tokens);
            self.token_holders.insert(investor_address.clone(), investor_holder);
        } else {
            if let Some(holder) = self.token_holders.get_mut(&investor_address) {
                holder.add_security_tokens(investor_tokens);
            }
        }
        
        // Issue utility tokens for voting
        let voting_power = self.utility_token.issue_voting_tokens(utility_tokens);
        
        // Utility токены распределяются пропорционально security токенам
        let total_security = main_owner_tokens + charity_tokens + investor_tokens;
        let main_owner_utility = (main_owner_tokens / total_security) * voting_power;
        let charity_utility = (charity_tokens / total_security) * voting_power;
        let investor_utility = (investor_tokens / total_security) * voting_power;
        
        // Добавляем utility токены
        if let Some(main_owner_holder) = self.token_holders.get_mut(&self.main_owner) {
            main_owner_holder.add_utility_tokens(main_owner_utility);
        }
        
        if let Some(charity_holder) = self.token_holders.get_mut(&charity_address) {
            charity_holder.add_utility_tokens(charity_utility);
        }
        
        if let Some(investor_holder) = self.token_holders.get_mut(&investor_address) {
            investor_holder.add_utility_tokens(investor_utility);
        }
        
        // Проверяем ограничения после эмиссии
        self.check_token_limits_and_create_alerts();
        
        Ok(())
    }
    
    // Получение алертов мониторинга
    fn get_monitoring_alerts(&self, limit: Option<u32>) -> Vec<MonitoringAlert> {
        let limit = limit.unwrap_or(100);
        let mut alerts = self.monitoring_alerts.clone();
        alerts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        alerts.truncate(limit as usize);
        alerts
    }
    
    // Распределение невостребованных токенов в конце года
    fn distribute_unclaimed_tokens_annually(&mut self) -> Result<AnnualDistribution, String> {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Находим все невостребованные токены, которые истекли
        let mut unclaimed_to_distribute: Vec<UnclaimedTokensRecord> = self.unclaimed_tokens
            .iter()
            .filter(|record| !record.is_distributed && record.expiry_timestamp <= current_timestamp)
            .cloned()
            .collect();
        
        if unclaimed_to_distribute.is_empty() {
            return Err("Нет невостребованных токенов для распределения".to_string());
        }
        
        let total_unclaimed: f64 = unclaimed_to_distribute.iter().map(|r| r.amount).sum();
        
        // Вычисляем общее количество токенов для пропорционального распределения
        let total_security_tokens: f64 = self.token_holders.values().map(|h| h.security_tokens).sum();
        
        let mut distributions = Vec::new();
        
        // Собираем информацию о держателях токенов
        let mut holder_info = Vec::new();
        for (address, holder) in &self.token_holders {
            let holder_percentage = (holder.security_tokens / total_security_tokens) * 100.0;
            let distribution_amount = (holder_percentage / 100.0) * total_unclaimed;
            
            if distribution_amount > 0.0 {
                let recipient_type = if holder.is_main_owner {
                    RecipientType::MainOwner
                } else if holder.is_charity_fund {
                    RecipientType::CharityFund
                } else if holder.is_franchise_owner {
                    RecipientType::FranchiseOwner
                } else {
                    RecipientType::Customer
                };
                
                distributions.push(TokenDistribution {
                    recipient_address: address.clone(),
                    amount: distribution_amount,
                    percentage: holder_percentage,
                    recipient_type,
                });
                
                holder_info.push((address.clone(), distribution_amount));
            }
        }
        
        // Добавляем токены держателям
        for (address, amount) in holder_info {
            if let Some(holder_mut) = self.token_holders.get_mut(&address) {
                holder_mut.add_security_tokens(amount);
            }
        }
        
        // Создаем запись о годовом распределении
        let annual_distribution = AnnualDistribution {
            year: self.current_year,
            total_unclaimed_tokens: total_unclaimed,
            distribution_timestamp: current_timestamp,
            distributions: distributions.clone(),
        };
        
        // Отмечаем токены как распределенные
        for unclaimed_record in &mut self.unclaimed_tokens {
            if !unclaimed_record.is_distributed && unclaimed_record.expiry_timestamp <= current_timestamp {
                unclaimed_record.is_distributed = true;
                unclaimed_record.distributed_timestamp = Some(current_timestamp);
            }
        }
        
        self.annual_distributions.push(annual_distribution.clone());
        
        // Проверяем ограничения после распределения
        self.check_token_limits_and_create_alerts();
        
        Ok(annual_distribution)
    }
    
    // Получение невостребованных токенов
    fn get_unclaimed_tokens(&self, limit: Option<u32>) -> Vec<UnclaimedTokensRecord> {
        let limit = limit.unwrap_or(100);
        let mut unclaimed = self.unclaimed_tokens.clone();
        unclaimed.sort_by(|a, b| b.created_timestamp.cmp(&a.created_timestamp));
        unclaimed.truncate(limit as usize);
        unclaimed
    }
    
    // Получение истории годовых распределений
    fn get_annual_distributions(&self, limit: Option<u32>) -> Vec<AnnualDistribution> {
        let limit = limit.unwrap_or(10);
        let mut distributions = self.annual_distributions.clone();
        distributions.sort_by(|a, b| b.year.cmp(&a.year));
        distributions.truncate(limit as usize);
        distributions
    }
    
    // Проверка истечения невостребованных токенов
    fn check_expired_unclaimed_tokens(&mut self) -> Vec<String> {
        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut expired_checks = Vec::new();
        
        for record in &self.unclaimed_tokens {
            if !record.is_distributed && record.expiry_timestamp <= current_timestamp {
                expired_checks.push(record.check_id.clone());
            }
        }
        
        expired_checks
    }
    
    // Проверка безопасности сети
    fn check_network_security(&self) -> NetworkSecurityReport {
        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        let total_utility_tokens = self.utility_token.total_supply;
        
        let mut security_risks = Vec::new();
        let mut utility_risks = Vec::new();
        
        for (address, holder) in &self.token_holders {
            let security_percentage = (holder.security_tokens / total_security_tokens) * 100.0;
            let utility_percentage = (holder.utility_tokens / total_utility_tokens) * 100.0;
            
            // Определяем максимальный лимит в зависимости от роли пользователя
            let max_percentage = if holder.is_main_owner {
                self.max_owner_percentage
            } else if holder.is_franchise_owner {
                self.max_franchise_percentage
            } else {
                self.max_customer_percentage
            };
            
            if security_percentage > max_percentage + 0.01 {
                security_risks.push(OwnershipRisk {
                    wallet: address.clone(),
                    percentage: security_percentage,
                    token_type: "Security".to_string(),
                });
            }
            
            if utility_percentage > max_percentage + 0.01 {
                utility_risks.push(OwnershipRisk {
                    wallet: address.clone(),
                    percentage: utility_percentage,
                    token_type: "Utility".to_string(),
                });
            }
        }
        
        NetworkSecurityReport {
            total_security_tokens,
            total_utility_tokens,
            max_owner_percentage: self.max_owner_percentage,
            security_risks: security_risks.clone(),
            utility_risks: utility_risks.clone(),
            is_secure: security_risks.is_empty() && utility_risks.is_empty(),
        }
    }
}

// Block structure (simplified for this example)
#[derive(Debug, Clone)]
struct Block {
    index: u32,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    timestamp: u64,
    validator: String,
    stake_used: f64,
    nonce: u64,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, prev_hash: String, validator: String, stake_used: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            index,
            transactions,
            prev_hash,
            hash: String::new(),
            timestamp,
            validator,
            stake_used,
            nonce: 0,
        };
        
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let tx_concat: String = self.transactions.iter().map(|t| t.transaction_id.as_str()).collect();
        let data = format!(
            "{}{}{}{}{}{}{}",
            self.index,
            tx_concat,
            self.prev_hash,
            self.timestamp,
            self.validator,
            self.stake_used,
            self.nonce
        );
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

// API Request/Response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayerSaleItem {
    pub item_id: String,
    pub quantity: u32,
    pub price_subunits: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckItemRequest {
    pub item_id: String,
    pub name: String,
    pub quantity: u32,
    pub price_subunits: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddressRequest {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseUserInfo {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub kyc_status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogEntryInfo {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub module: String,
    pub thread_id: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MetricInfo {
    pub name: String,
    pub value: f64,
    pub timestamp: String,
    pub metric_type: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlertInfo {
    pub alert_id: String,
    pub name: String,
    pub description: String,
    pub severity: String,
    pub status: String,
    pub created_at: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChangelogEntryInfo {
    pub version: String,
    pub date: String,
    pub changes: Vec<ChangeEntryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChangeEntryInfo {
    pub change_type: String,
    pub description: String,
    pub impact: String,
    pub affected_endpoints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiRequest {
    GetMenu,
    GetMenuItem { id: String },
    CreateOrder { 
        customer_wallet: String, 
        items: Vec<OrderItem>, 
        delivery_time_minutes: u32 
    },
    CancelOrder { 
        order_id: String, 
        reason: String, 
        customer_wallet: String 
    },
    GetOrderStatus { order_id: String },
    GetWalletBalance { wallet: String },
    GetBlockchainHistory { limit: Option<u32> },
    GetVotingHistory,
    VoteOnMenuItem { 
        voter_wallet: String, 
        menu_item_id: String, 
        vote_for: bool 
    },
    AddMenuItem {
        name: String,
        description: String,
        price_subunits: u128, // Цена в subunits (1/100 GEL)
        availability: u32,
        priority_rank: u32,
        cooking_time_minutes: u32,
        ingredients: Vec<Ingredient>,
        suggested_by: String,
    },
    MakeItemAvailableForVoting { menu_item_id: String },
    ConfirmOrder { order_id: String },
    RegisterUserWithPhone { phone_number: String, wallet_address: String },
    VerifyPhoneNumber { phone_number: String, verification_code: String },
    TransferBalanceFromCheck { check_id: String, to_phone_number: String },
    GetBalanceTransferHistory { limit: Option<u32> },
    GetNetworkSecurityReport,
    AddFranchiseNode { node_id: String, franchise_owner: String },
    // Regulatory Export API endpoints
    ExportHoldersRegistry { format: String }, // "CSV" or "JSON"
    ExportEmissionsRegistry { format: String },
    ExportCorporateActionsRegistry { format: String },
    GenerateRegulatoryReport { format: String },
    // Relayer Service API endpoints
    ProcessRelayerTransaction {
        sale_id: String,
        node_id: u64,
        pos_id: String,
        amount_subunits: u128,
        buyer_address: String,
        buyer_meta: String,
        items: Vec<RelayerSaleItem>,
        signature: String,
        timestamp: u64,
    },
    GetRelayerTransactionStatus { transaction_id: String },
    GetRelayerStatistics,
    // HD Wallet API endpoints
    GenerateNodeWallet { node_id: u64, wallet_type: String },
    GenerateCustomerWallet { customer_id: String },
    GenerateCheckWallet { 
        sale_id: String, 
        node_id: u64, 
        amount_subunits: u128, 
        items: Vec<CheckItemRequest> 
    },
    ActivateCheckWallet { check_id: String, activation_code: String },
    GetWalletInfo { wallet_id: String },
    GetCheckWalletInfo { check_id: String },
    GetWalletStatistics,
    // KYC/AML API endpoints
    RegisterUser {
        email: String,
        phone: Option<String>,
        first_name: String,
        last_name: String,
        date_of_birth: Option<String>,
        nationality: Option<String>,
        address: Option<AddressRequest>,
    },
    StartKYCProcess { user_id: String, kyc_level: String },
    UploadDocument {
        user_id: String,
        document_type: String,
        file_hash: String,
        file_path: String,
    },
    VerifyDocument {
        user_id: String,
        document_id: String,
        verified_by: String,
        approved: bool,
        rejection_reason: Option<String>,
    },
    CompleteKYCProcess { user_id: String, verified_by: String },
    AssignRole {
        user_id: String,
        role: String,
        assigned_by: String,
        expires_at: Option<String>,
    },
    CheckPermission { user_id: String, permission: String },
    GetUserInfo { user_id: String },
    GetKYCStatistics,
    // Database API endpoints
    InitializeDatabase { 
        host: String, 
        port: u16, 
        database: String, 
        username: String, 
        password: String 
    },
    GetDatabaseStats,
    CleanupOldData { days: i32 },
    GetUserFromDatabase { user_id: String },
    GetAllUsersFromDatabase { limit: Option<u32>, offset: Option<u32> },
    // Observability API endpoints
    GetLogs { level: Option<String>, limit: Option<u32> },
    GetMetrics { metric_type: Option<String>, limit: Option<u32> },
    GetAlerts { status: Option<String>, severity: Option<String> },
    GetObservabilityStats,
    GeneratePrometheusMetrics,
    CreateAlert { 
        alert_id: String, 
        name: String, 
        description: String, 
        severity: String 
    },
    ResolveAlert { alert_id: String },
    // API Versioning endpoints
    GetApiVersion,
    GetSupportedVersions,
    GetVersionInfo { version: String },
    GetVersionWarning { version: String },
    GetChangelog { version: Option<String> },
    GetVersionStatistics,
    GenerateOpenApiSpec { version: String },
    CheckVersionCompatibility { version1: String, version2: String },
    EmitTokensForInvestors { amount_gel: f64, investor_address: String },
    GetMonitoringAlerts { limit: Option<u32> },
    GetCharityFundInfo,
    DistributeUnclaimedTokensAnnually,
    GetUnclaimedTokens { limit: Option<u32> },
    GetAnnualDistributions { limit: Option<u32> },
    CheckExpiredUnclaimedTokens,
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    Menu { items: Vec<MenuItem> },
    MenuItem { item: MenuItem },
    OrderCreated { order: Order },
    OrderCancelled { success: bool },
    OrderStatus { order: Order },
    WalletBalance { 
        wallet: String, 
        security_tokens: f64, 
        utility_tokens: f64 
    },
    BlockchainHistory { 
        orders: Vec<BlockchainOrderRecord> 
    },
    VotingHistory { 
        votes: Vec<VotingRecord> 
    },
    VoteResult { success: bool },
    MenuItemAdded { success: bool },
    ItemAvailableForVoting { success: bool },
    OrderConfirmed { success: bool },
    UserRegistered { verification_code: String },
    PhoneVerified { success: bool },
    BalanceTransferred { transfer_id: String },
    BalanceTransferHistory { transfers: Vec<BalanceTransferRecord> },
    NetworkSecurityReport { report: NetworkSecurityReport },
    FranchiseNodeAdded { success: bool },
    TokensEmitted { success: bool },
    MonitoringAlerts { alerts: Vec<MonitoringAlert> },
    CharityFundInfo { fund: CharityFund },
    UnclaimedTokensDistributed { distribution: AnnualDistribution },
    UnclaimedTokens { tokens: Vec<UnclaimedTokensRecord> },
    AnnualDistributions { distributions: Vec<AnnualDistribution> },
    ExpiredUnclaimedTokens { expired_checks: Vec<String> },
    RegulatoryExport { data: String, format: String },
    // Relayer Service responses
    RelayerTransactionResponse { 
        transaction_id: String, 
        status: String, 
        message: String, 
        blockchain_tx_hash: Option<String> 
    },
    RelayerTransactionStatus { 
        transaction_id: String, 
        status: String, 
        blockchain_tx_hash: Option<String>, 
        error_message: Option<String> 
    },
    RelayerStatistics { 
        total_processed: u64, 
        total_successful: u64, 
        total_failed: u64, 
        total_retries: u64, 
        average_processing_time_ms: u64 
    },
    // HD Wallet responses
    WalletGenerated { 
        wallet_id: String, 
        address: String, 
        derivation_path: String, 
        public_key: String 
    },
    CheckWalletGenerated { 
        check_id: String, 
        wallet_address: String, 
        qr_code: String, 
        activation_code: String, 
        expires_at: String 
    },
    CheckWalletActivated { 
        check_id: String, 
        wallet_address: String, 
        activated_at: String 
    },
    WalletInfo { 
        wallet_id: String, 
        wallet_type: String, 
        address: String, 
        status: String, 
        created_at: String 
    },
    CheckWalletInfo { 
        check_id: String, 
        wallet_address: String, 
        amount_subunits: u128, 
        is_activated: bool, 
        expires_at: String 
    },
    WalletStatistics { 
        total_wallets: u32, 
        total_check_wallets: u32, 
        active_wallets: u32, 
        inactive_wallets: u32 
    },
    // KYC/AML responses
    UserRegistered { 
        user_id: String, 
        email: String, 
        kyc_status: String 
    },
    KYCProcessStarted { 
        user_id: String, 
        kyc_level: String, 
        status: String 
    },
    DocumentUploaded { 
        document_id: String, 
        user_id: String, 
        document_type: String 
    },
    DocumentVerified { 
        document_id: String, 
        user_id: String, 
        approved: bool 
    },
    KYCProcessCompleted { 
        user_id: String, 
        kyc_status: String, 
        risk_score: u8 
    },
    RoleAssigned { 
        user_id: String, 
        role: String, 
        assigned_by: String 
    },
    PermissionCheck { 
        user_id: String, 
        permission: String, 
        has_permission: bool 
    },
    UserInfo { 
        user_id: String, 
        email: String, 
        first_name: String, 
        last_name: String, 
        kyc_status: String, 
        kyc_level: String, 
        risk_score: u8 
    },
    KYCStatistics { 
        total_users: u32, 
        total_documents: u32, 
        verified: u32, 
        pending: u32, 
        rejected: u32, 
        high_risk: u32 
    },
    // Database responses
    DatabaseInitialized { 
        message: String, 
        config: String 
    },
    DatabaseStats { 
        total_users: u32, 
        total_documents: u32, 
        total_wallets: u32, 
        total_transactions: u32, 
        total_audit_logs: u32 
    },
    CleanupCompleted { 
        deleted_audit_logs: u32, 
        deleted_expired_checks: u32 
    },
    DatabaseUserInfo { 
        user_id: String, 
        email: String, 
        first_name: String, 
        last_name: String, 
        kyc_status: String, 
        created_at: String 
    },
    DatabaseUsersList { 
        users: Vec<DatabaseUserInfo>, 
        total_count: u32 
    },
    // Observability responses
    LogsList { 
        logs: Vec<LogEntryInfo>, 
        total_count: u32 
    },
    MetricsList { 
        metrics: Vec<MetricInfo>, 
        total_count: u32 
    },
    AlertsList { 
        alerts: Vec<AlertInfo>, 
        total_count: u32 
    },
    ObservabilityStats { 
        total_logs: u32, 
        total_metrics: u32, 
        total_alerts: u32, 
        total_requests: u32, 
        total_errors: u32, 
        active_connections: u32 
    },
    PrometheusMetrics { 
        metrics: String 
    },
    AlertCreated { 
        alert_id: String, 
        name: String, 
        status: String 
    },
    AlertResolved { 
        alert_id: String, 
        status: String 
    },
    // API Versioning responses
    ApiVersionInfo { 
        version: String, 
        status: String, 
        release_date: String 
    },
    SupportedVersions { 
        versions: Vec<String>, 
        current_version: String 
    },
    VersionDetails { 
        version: String, 
        status: String, 
        release_date: String, 
        deprecation_date: Option<String>, 
        retirement_date: Option<String>, 
        new_features: Vec<String>, 
        breaking_changes: Vec<String>, 
        bug_fixes: Vec<String> 
    },
    VersionWarning { 
        warning_type: String, 
        message: String, 
        retirement_date: Option<String> 
    },
    Changelog { 
        entries: Vec<ChangelogEntryInfo>, 
        total_count: u32 
    },
    VersionStats { 
        total_versions: u32, 
        current_version: String, 
        supported_versions: u32, 
        deprecated_versions: u32, 
        retired_versions: u32, 
        total_endpoints: u32, 
        deprecated_endpoints: u32 
    },
    OpenApiSpec { 
        spec: String, 
        version: String 
    },
    VersionCompatibility { 
        version1: String, 
        version2: String, 
        compatible: bool, 
        reason: Option<String> 
    },
    Error { message: String },
}

// Структуры для истории
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockchainOrderRecord {
    order_id: String,
    customer_wallet: String,
    total_amount: f64, // Оставляем f64 для совместимости с API
    tokens_issued: f64, // Оставляем f64 для совместимости с API
    timestamp: u64,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VotingRecord {
    voter_wallet: String,
    menu_item_id: String,
    menu_item_name: String,
    vote_weight: f64,
    vote_for: bool,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BalanceTransferRecord {
    transfer_id: String,
    from_check_id: String,
    from_wallet: String,
    to_wallet: String,
    to_phone: String,
    security_tokens_transferred: f64, // Оставляем f64 для совместимости с API
    utility_tokens_transferred: f64, // Оставляем f64 для совместимости с API
    timestamp: u64,
    status: TransferStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum TransferStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkSecurityReport {
    total_security_tokens: f64,
    total_utility_tokens: f64,
    max_owner_percentage: f64,
    security_risks: Vec<OwnershipRisk>,
    utility_risks: Vec<OwnershipRisk>,
    is_secure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OwnershipRisk {
    wallet: String,
    percentage: f64,
    token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MonitoringAlert {
    alert_id: String,
    alert_type: AlertType,
    severity: AlertSeverity,
    message: String,
    affected_wallet: Option<String>,
    percentage: Option<f64>,
    timestamp: u64,
    is_resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnclaimedTokensRecord {
    check_id: String,
    amount: f64, // Оставляем f64 для совместимости с API
    created_timestamp: u64,
    expiry_timestamp: u64,
    is_distributed: bool,
    distributed_timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnnualDistribution {
    year: u32,
    total_unclaimed_tokens: f64, // Оставляем f64 для совместимости с API
    distribution_timestamp: u64,
    distributions: Vec<TokenDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenDistribution {
    recipient_address: String,
    amount: f64, // Оставляем f64 для совместимости с API
    percentage: f64,
    recipient_type: RecipientType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum RecipientType {
    MainOwner,
    CharityFund,
    FranchiseOwner,
    Customer,
    Investor,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
enum AlertType {
    OwnerExceedsLimit,
    FranchiseExceedsLimit,
    CustomerExceedsLimit,
    CharityFundLow,
    CoordinatedAttack,
    TokenConcentration,
    NetworkAnomaly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// Децентрализованные смарт-контракты
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SmartContract {
    contract_id: String,
    contract_type: ContractType,
    creator: String,
    created_timestamp: u64,
    status: ContractStatus,
    participants: Vec<String>,
    conditions: ContractConditions,
    execution_history: Vec<ContractExecution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ContractType {
    PurchaseContract,
    VotingContract,
    OrderContract,
    TokenContract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ContractStatus {
    Active,
    Executed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContractConditions {
    min_tokens_required: f64,
    expiration_time: Option<u64>,
    required_participants: Vec<String>,
    auto_execute: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContractExecution {
    executor: String,
    action: String,
    timestamp: u64,
    result: String,
    tokens_used: f64,
}

#[cfg_attr(test, allow(dead_code))]
impl SmartContract {
    fn new(contract_type: ContractType, creator: String, conditions: ContractConditions) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let contract_id = Self::generate_contract_id(&creator, timestamp);
        
        SmartContract {
            contract_id,
            contract_type,
            creator: creator.clone(),
            created_timestamp: timestamp,
            status: ContractStatus::Active,
            participants: vec![creator],
            conditions,
            execution_history: vec![],
        }
    }

    fn generate_contract_id(creator: &str, timestamp: u64) -> String {
        let data = format!("{}{}", creator, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("CONTRACT_{}", hex::encode(&hasher.finalize()[..8]))
    }

    fn execute(&mut self, executor: String, action: String, tokens_used: f64) -> Result<String, String> {
        if self.status != ContractStatus::Active {
            return Err("Contract is not active".to_string());
        }

        // Проверяем условия выполнения
        if !self.participants.contains(&executor) {
            return Err("Executor is not a participant".to_string());
        }

        let result = format!("Action '{}' executed by {}", action, executor);
        
        let execution = ContractExecution {
            executor: executor.clone(),
            action,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            result: result.clone(),
            tokens_used,
        };

        self.execution_history.push(execution);
        
        // Если контракт настроен на автоматическое выполнение
        if self.conditions.auto_execute {
            self.status = ContractStatus::Executed;
        }

        Ok(result)
    }
}

// API Server
struct ApiServer {
    blockchain: Arc<Mutex<Blockchain>>,
    port: u16,
}

impl ApiServer {
    fn new(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Self {
        ApiServer { blockchain, port }
    }

    fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");
        
        println!("🌐 API Server started on port {}", self.port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let blockchain_clone = Arc::clone(&self.blockchain);
                    thread::spawn(move || {
                        Self::handle_client(stream, blockchain_clone);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 8192];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let req_raw = String::from_utf8_lossy(&buffer[..size]).to_string();

                // Very basic HTTP parsing
                let headers_end = req_raw.find("\r\n\r\n").unwrap_or(req_raw.len());
                let (head, body_part) = req_raw.split_at(headers_end);
                let body = if body_part.starts_with("\r\n\r\n") { &body_part[4..] } else { body_part };

                let method = if let Some(space) = head.find(' ') { &head[..space] } else { "" };

                // CORS preflight
                if method == "OPTIONS" {
                    let resp = "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                    let _ = stream.write_all(resp.as_bytes());
                    return;
                }

                // Expect POST with JSON body containing our ApiRequest enum
                let api_result = match serde_json::from_str::<ApiRequest>(body) {
                    Ok(req) => Self::process_request(req, blockchain),
                    Err(_) => {
                        // Fallback compatibility: accept {"Variant": {...}} and unit variants as {"Variant": {}}
                        match serde_json::from_str::<serde_json::Value>(body) {
                            Ok(val) => {
                                if let Some(obj) = val.as_object() {
                                    if obj.contains_key("GetMenu") {
                                        Self::process_request(ApiRequest::GetMenu, blockchain)
                                    } else if let Some(params) = obj.get("GetBlockchainHistory") {
                                        let limit = params.get("limit").and_then(|v| v.as_u64()).map(|v| v as u32);
                                        Self::process_request(ApiRequest::GetBlockchainHistory { limit }, blockchain)
                                    } else if obj.contains_key("GetVotingHistory") {
                                        Self::process_request(ApiRequest::GetVotingHistory, blockchain)
                                    } else if let Some(params) = obj.get("MakeItemAvailableForVoting") {
                                        if let Some(menu_item_id) = params.get("menu_item_id").and_then(|v| v.as_str()) {
                                            Self::process_request(ApiRequest::MakeItemAvailableForVoting { menu_item_id: menu_item_id.to_string() }, blockchain)
                                        } else {
                                            ApiResponse::Error { message: "Invalid MakeItemAvailableForVoting payload".to_string() }
                                        }
                                    } else if let Some(params) = obj.get("ConfirmOrder") {
                                        if let Some(order_id) = params.get("order_id").and_then(|v| v.as_str()) {
                                            Self::process_request(ApiRequest::ConfirmOrder { order_id: order_id.to_string() }, blockchain)
                                        } else {
                                            ApiResponse::Error { message: "Invalid ConfirmOrder payload".to_string() }
                                        }
                                    } else if let Some(params) = obj.get("CancelOrder") {
                                        let order_id = params.get("order_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let reason = params.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let customer_wallet = params.get("customer_wallet").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        if order_id.is_empty() || customer_wallet.is_empty() {
                                            ApiResponse::Error { message: "Invalid CancelOrder payload".to_string() }
                                        } else {
                                            Self::process_request(ApiRequest::CancelOrder { order_id, reason, customer_wallet }, blockchain)
                                        }
                                    } else if let Some(params) = obj.get("AddMenuItem") {
                                        // Map incoming ingredient fields amount -> amount_grams
                                        let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let description = params.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let price_gel = params.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let price_subunits = config::utils::gel_to_subunits(price_gel);
                                        let availability = params.get("availability").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let priority_rank = params.get("priority_rank").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let cooking_time_minutes = params.get("cooking_time_minutes").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let suggested_by = params.get("suggested_by").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let ingredients = params.get("ingredients").and_then(|v| v.as_array()).map(|arr| {
                                            arr.iter().filter_map(|ing| {
                                                let name = ing.get("name").and_then(|v| v.as_str())?.to_string();
                                                let amount = ing.get("amount").or_else(|| ing.get("amount_grams")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                let calories = ing.get("calories").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                Some(Ingredient { name, amount_grams: amount, calories })
                                            }).collect::<Vec<_>>()
                                        }).unwrap_or_default();

                                        Self::process_request(
                                            ApiRequest::AddMenuItem {
                                                name, description, price_subunits, availability, priority_rank, cooking_time_minutes, ingredients, suggested_by
                                            },
                                            blockchain,
                                        )
                                    } else if let Some(params) = obj.get("ProcessRelayerTransaction") {
                                        let sale_id = params.get("sale_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let node_id = params.get("node_id").and_then(|v| v.as_u64()).unwrap_or(0) as u64;
                                        let pos_id = params.get("pos_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let amount_gel = params.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let amount_subunits = config::utils::gel_to_subunits(amount_gel);
                                        let buyer_address = params.get("buyer_address").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let buyer_meta = params.get("buyer_meta").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let signature = params.get("signature").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let timestamp = params.get("timestamp").and_then(|v| v.as_u64()).unwrap_or(0) as u64;
                                        let items = params.get("items").and_then(|v| v.as_array()).map(|arr| {
                                            arr.iter().filter_map(|item| {
                                                let item_id = item.get("item_id").and_then(|v| v.as_str())?.to_string();
                                                let quantity = item.get("quantity").and_then(|v| v.as_u64())? as u32;
                                                let price_gel = item.get("price").and_then(|v| v.as_f64())?;
                                                let price_subunits = config::utils::gel_to_subunits(price_gel);
                                                Some(RelayerSaleItem { item_id, quantity, price_subunits })
                                            }).collect::<Vec<_>>()
                                        }).unwrap_or_default();

                                        if sale_id.is_empty() || node_id == 0 || pos_id.is_empty() || buyer_address.is_empty() {
                                            ApiResponse::Error { message: "Invalid ProcessRelayerTransaction payload".to_string() }
                                        } else {
                                            Self::process_request(
                                                ApiRequest::ProcessRelayerTransaction {
                                                    sale_id, node_id, pos_id, amount_subunits, buyer_address, buyer_meta, items, signature, timestamp
                                                },
                                                blockchain,
                                            )
                                        }
                                    } else if let Some(params) = obj.get("VoteOnMenuItem") {
                                        let voter_wallet = params.get("voter_wallet").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let menu_item_id = params.get("menu_item_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let vote_for = params.get("vote_for").and_then(|v| v.as_bool()).unwrap_or(true);
                                        if voter_wallet.is_empty() || menu_item_id.is_empty() {
                                            ApiResponse::Error { message: "Invalid VoteOnMenuItem payload".to_string() }
                                        } else {
                                            Self::process_request(ApiRequest::VoteOnMenuItem { voter_wallet, menu_item_id, vote_for }, blockchain)
                                        }
                                    } else {
                                        ApiResponse::Error { message: "Unknown API request".to_string() }
                                    }
                                } else {
                                    ApiResponse::Error { message: "Invalid request payload".to_string() }
                                }
                            }
                            Err(e) => ApiResponse::Error { message: format!("Invalid request format: {}", e) },
                        }
                    }
                };

                let response_json = serde_json::to_string(&api_result)
                    .unwrap_or_else(|_| r#"{"Error": {"message": "Serialization error"}}"#.to_string());

                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\n\r\n{}",
                    response_json.len(),
                    response_json
                );
                let _ = stream.write_all(resp.as_bytes());
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
            }
        }
    }

    fn process_request(request: ApiRequest, blockchain: Arc<Mutex<Blockchain>>) -> ApiResponse {
        let mut blockchain_guard = blockchain.lock().unwrap();
        
        match request {
            ApiRequest::GetMenu => {
                let items = blockchain_guard.menu_items.clone();
                ApiResponse::Menu { items }
            }
            
            ApiRequest::GetMenuItem { id } => {
                if let Some(item) = blockchain_guard.menu_items.iter().find(|item| item.id == id) {
                    ApiResponse::MenuItem { item: item.clone() }
                } else {
                    ApiResponse::Error { message: "Menu item not found".to_string() }
                }
            }
            
            ApiRequest::CreateOrder { customer_wallet, items, delivery_time_minutes } => {
                match blockchain_guard.create_order(customer_wallet, items, delivery_time_minutes) {
                    Ok(order) => ApiResponse::OrderCreated { order },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::CancelOrder { order_id, reason, customer_wallet } => {
                // Проверяем, что заказ принадлежит этому кошельку
                if let Some(order) = blockchain_guard.orders.iter().find(|o| o.id == order_id) {
                    if order.customer_wallet == customer_wallet {
                        match blockchain_guard.cancel_order(order_id, reason) {
                            Ok(()) => ApiResponse::OrderCancelled { success: true },
                            Err(e) => ApiResponse::Error { message: e },
                        }
                    } else {
                        ApiResponse::Error { message: "Order does not belong to this wallet".to_string() }
                    }
                } else {
                    ApiResponse::Error { message: "Order not found".to_string() }
                }
            }
            
            ApiRequest::GetOrderStatus { order_id } => {
                if let Some(order) = blockchain_guard.orders.iter().find(|o| o.id == order_id) {
                    ApiResponse::OrderStatus { order: order.clone() }
                } else {
                    ApiResponse::Error { message: "Order not found".to_string() }
                }
            }
            
            ApiRequest::GetWalletBalance { wallet } => {
                if let Some(holder) = blockchain_guard.token_holders.get(&wallet) {
                    ApiResponse::WalletBalance {
                        wallet,
                        security_tokens: holder.security_tokens,
                        utility_tokens: holder.utility_tokens,
                    }
                } else {
                    ApiResponse::WalletBalance {
                        wallet,
                        security_tokens: 0.0,
                        utility_tokens: 0.0,
                    }
                }
            }
            
            ApiRequest::GetBlockchainHistory { limit } => {
                let orders = blockchain_guard.get_blockchain_history(limit);
                ApiResponse::BlockchainHistory { orders }
            }
            
            ApiRequest::GetVotingHistory => {
                let votes = blockchain_guard.get_voting_history();
                ApiResponse::VotingHistory { votes }
            }
            
            ApiRequest::VoteOnMenuItem { voter_wallet, menu_item_id, vote_for } => {
                match blockchain_guard.vote_on_menu_item(voter_wallet, menu_item_id, vote_for) {
                    Ok(()) => ApiResponse::VoteResult { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::AddMenuItem { name, description, price_subunits, availability, priority_rank, cooking_time_minutes, ingredients, suggested_by } => {
                match blockchain_guard.add_menu_item_with_details(
                    name, description, price_subunits, availability, priority_rank, 
                    cooking_time_minutes, ingredients, suggested_by
                ) {
                    Ok(()) => ApiResponse::MenuItemAdded { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::MakeItemAvailableForVoting { menu_item_id } => {
                match blockchain_guard.make_menu_item_available_for_voting(menu_item_id) {
                    Ok(()) => ApiResponse::ItemAvailableForVoting { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::ConfirmOrder { order_id } => {
                match blockchain_guard.confirm_order(order_id) {
                    Ok(()) => ApiResponse::OrderConfirmed { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::RegisterUserWithPhone { phone_number, wallet_address } => {
                match blockchain_guard.register_user_with_phone(phone_number, wallet_address) {
                    Ok(verification_code) => ApiResponse::UserRegistered { verification_code },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::VerifyPhoneNumber { phone_number, verification_code } => {
                match blockchain_guard.verify_phone_number(phone_number, verification_code) {
                    Ok(()) => ApiResponse::PhoneVerified { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::TransferBalanceFromCheck { check_id, to_phone_number } => {
                match blockchain_guard.transfer_balance_from_check(check_id, to_phone_number) {
                    Ok(transfer_id) => ApiResponse::BalanceTransferred { transfer_id },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::GetBalanceTransferHistory { limit } => {
                let transfers = blockchain_guard.get_balance_transfer_history(limit);
                ApiResponse::BalanceTransferHistory { transfers }
            }
            
            ApiRequest::GetNetworkSecurityReport => {
                let report = blockchain_guard.check_network_security();
                ApiResponse::NetworkSecurityReport { report }
            }
            
            ApiRequest::AddFranchiseNode { node_id, franchise_owner } => {
                match blockchain_guard.add_franchise_node(node_id, franchise_owner) {
                    Ok(()) => ApiResponse::FranchiseNodeAdded { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::EmitTokensForInvestors { amount_gel, investor_address } => {
                match blockchain_guard.emit_tokens_for_investors(amount_gel, investor_address) {
                    Ok(()) => ApiResponse::TokensEmitted { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::GetMonitoringAlerts { limit } => {
                let alerts = blockchain_guard.get_monitoring_alerts(limit);
                ApiResponse::MonitoringAlerts { alerts }
            }
            
            ApiRequest::GetCharityFundInfo => {
                let fund = blockchain_guard.charity_fund.clone();
                ApiResponse::CharityFundInfo { fund }
            }
            
            ApiRequest::DistributeUnclaimedTokensAnnually => {
                match blockchain_guard.distribute_unclaimed_tokens_annually() {
                    Ok(distribution) => ApiResponse::UnclaimedTokensDistributed { distribution },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::GetUnclaimedTokens { limit } => {
                let tokens = blockchain_guard.get_unclaimed_tokens(limit);
                ApiResponse::UnclaimedTokens { tokens }
            }
            
            ApiRequest::GetAnnualDistributions { limit } => {
                let distributions = blockchain_guard.get_annual_distributions(limit);
                ApiResponse::AnnualDistributions { distributions }
            }
            
            ApiRequest::CheckExpiredUnclaimedTokens => {
                let expired_checks = blockchain_guard.check_expired_unclaimed_tokens();
                ApiResponse::ExpiredUnclaimedTokens { expired_checks }
            }
            
            // Regulatory Export API endpoints
            ApiRequest::ExportHoldersRegistry { format } => {
                let export_format = match format.to_uppercase().as_str() {
                    "CSV" => ExportFormat::CSV,
                    "JSON" => ExportFormat::JSON,
                    _ => return ApiResponse::Error { message: "Invalid format. Use CSV or JSON".to_string() }
                };
                
                match export_format {
                    ExportFormat::CSV => {
                        let csv_data = blockchain_guard.regulatory_exporter.export_holders_csv();
                        ApiResponse::RegulatoryExport { data: csv_data, format: "CSV".to_string() }
                    },
                    ExportFormat::JSON => {
                        match blockchain_guard.regulatory_exporter.export_holders_json() {
                            Ok(json_data) => ApiResponse::RegulatoryExport { data: json_data, format: "JSON".to_string() },
                            Err(e) => ApiResponse::Error { message: e }
                        }
                    },
                    _ => ApiResponse::Error { message: "Unsupported format".to_string() }
                }
            }
            
            ApiRequest::ExportEmissionsRegistry { format } => {
                let export_format = match format.to_uppercase().as_str() {
                    "CSV" => ExportFormat::CSV,
                    "JSON" => ExportFormat::JSON,
                    _ => return ApiResponse::Error { message: "Invalid format. Use CSV or JSON".to_string() }
                };
                
                match export_format {
                    ExportFormat::CSV => {
                        let csv_data = blockchain_guard.regulatory_exporter.export_emissions_csv();
                        ApiResponse::RegulatoryExport { data: csv_data, format: "CSV".to_string() }
                    },
                    ExportFormat::JSON => {
                        match blockchain_guard.regulatory_exporter.export_emissions_json() {
                            Ok(json_data) => ApiResponse::RegulatoryExport { data: json_data, format: "JSON".to_string() },
                            Err(e) => ApiResponse::Error { message: e }
                        }
                    },
                    _ => ApiResponse::Error { message: "Unsupported format".to_string() }
                }
            }
            
            ApiRequest::ExportCorporateActionsRegistry { format } => {
                let export_format = match format.to_uppercase().as_str() {
                    "CSV" => ExportFormat::CSV,
                    "JSON" => ExportFormat::JSON,
                    _ => return ApiResponse::Error { message: "Invalid format. Use CSV or JSON".to_string() }
                };
                
                match export_format {
                    ExportFormat::CSV => {
                        let csv_data = blockchain_guard.regulatory_exporter.export_corporate_actions_csv();
                        ApiResponse::RegulatoryExport { data: csv_data, format: "CSV".to_string() }
                    },
                    ExportFormat::JSON => {
                        match blockchain_guard.regulatory_exporter.export_corporate_actions_json() {
                            Ok(json_data) => ApiResponse::RegulatoryExport { data: json_data, format: "JSON".to_string() },
                            Err(e) => ApiResponse::Error { message: e }
                        }
                    },
                    _ => ApiResponse::Error { message: "Unsupported format".to_string() }
                }
            }
            
            ApiRequest::GenerateRegulatoryReport { format } => {
                let export_format = match format.to_uppercase().as_str() {
                    "CSV" => ExportFormat::CSV,
                    "JSON" => ExportFormat::JSON,
                    _ => return ApiResponse::Error { message: "Invalid format. Use CSV or JSON".to_string() }
                };
                
                match blockchain_guard.regulatory_exporter.generate_regulatory_report(export_format) {
                    Ok(report_data) => ApiResponse::RegulatoryExport { data: report_data, format: format.to_uppercase() },
                    Err(e) => ApiResponse::Error { message: e }
                }
            }
            
            // Relayer Service API endpoints
            ApiRequest::ProcessRelayerTransaction { 
                sale_id, node_id, pos_id, amount_subunits, buyer_address, 
                buyer_meta, items, signature, timestamp 
            } => {
                let request = RelayerTransactionRequest {
                    sale_id,
                    node_id,
                    pos_id,
                    amount_subunits,
                    buyer_address,
                    buyer_meta,
                    items: items.into_iter().map(|item| blockchain_project::relayer_service::SaleItem {
                        item_id: item.item_id,
                        quantity: item.quantity,
                        price_subunits: item.price_subunits,
                    }).collect(),
                    signature,
                    timestamp,
                };
                
                match blockchain_guard.relayer_service.process_transaction(request).await {
                    Ok(response) => ApiResponse::RelayerTransactionResponse {
                        transaction_id: response.transaction_id,
                        status: format!("{:?}", response.status),
                        message: response.message,
                        blockchain_tx_hash: response.blockchain_tx_hash,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::GetRelayerTransactionStatus { transaction_id } => {
                match blockchain_guard.relayer_service.get_transaction_status(&transaction_id).await {
                    Some(transaction) => ApiResponse::RelayerTransactionStatus {
                        transaction_id: transaction.id,
                        status: format!("{:?}", transaction.status),
                        blockchain_tx_hash: transaction.blockchain_tx_hash,
                        error_message: transaction.error_message,
                    },
                    None => ApiResponse::Error { message: "Transaction not found".to_string() }
                }
            }
            
            ApiRequest::GetRelayerStatistics => {
                let stats = blockchain_guard.relayer_service.get_statistics().await;
                ApiResponse::RelayerStatistics {
                    total_processed: stats.total_processed,
                    total_successful: stats.total_successful,
                    total_failed: stats.total_failed,
                    total_retries: stats.total_retries,
                    average_processing_time_ms: stats.average_processing_time_ms,
                }
            }
            
            // HD Wallet API endpoints
            ApiRequest::GenerateNodeWallet { node_id, wallet_type } => {
                let wallet_type_enum = match wallet_type.to_uppercase().as_str() {
                    "MASTER" => WalletType::Master,
                    "FRANCHISE" => WalletType::Franchise,
                    _ => return ApiResponse::Error { message: "Invalid wallet type. Use MASTER or FRANCHISE".to_string() }
                };
                
                match blockchain_guard.hd_wallet_manager.generate_node_wallet(node_id, wallet_type_enum) {
                    Ok(wallet) => ApiResponse::WalletGenerated {
                        wallet_id: wallet.wallet_id,
                        address: wallet.address,
                        derivation_path: wallet.derivation_path,
                        public_key: wallet.public_key,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::GenerateCustomerWallet { customer_id } => {
                match blockchain_guard.hd_wallet_manager.generate_customer_wallet(customer_id) {
                    Ok(wallet) => ApiResponse::WalletGenerated {
                        wallet_id: wallet.wallet_id,
                        address: wallet.address,
                        derivation_path: wallet.derivation_path,
                        public_key: wallet.public_key,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::GenerateCheckWallet { sale_id, node_id, amount_subunits, items } => {
                let check_items: Vec<blockchain_project::hd_wallet::CheckItem> = items.into_iter().map(|item| {
                    blockchain_project::hd_wallet::CheckItem {
                        item_id: item.item_id,
                        name: item.name,
                        quantity: item.quantity,
                        price_subunits: item.price_subunits,
                    }
                }).collect();
                
                match blockchain_guard.hd_wallet_manager.generate_check_wallet(sale_id, node_id, amount_subunits, check_items) {
                    Ok(check_wallet) => ApiResponse::CheckWalletGenerated {
                        check_id: check_wallet.check_id,
                        wallet_address: check_wallet.wallet.address,
                        qr_code: check_wallet.qr_code,
                        activation_code: check_wallet.activation_code,
                        expires_at: check_wallet.expires_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::ActivateCheckWallet { check_id, activation_code } => {
                match blockchain_guard.hd_wallet_manager.activate_check_wallet(&check_id, &activation_code) {
                    Ok(check_wallet) => ApiResponse::CheckWalletActivated {
                        check_id: check_wallet.check_id,
                        wallet_address: check_wallet.wallet.address,
                        activated_at: check_wallet.activated_at.unwrap().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::GetWalletInfo { wallet_id } => {
                match blockchain_guard.hd_wallet_manager.get_wallet(&wallet_id) {
                    Some(wallet) => ApiResponse::WalletInfo {
                        wallet_id: wallet.wallet_id.clone(),
                        wallet_type: format!("{:?}", wallet.wallet_type),
                        address: wallet.address.clone(),
                        status: format!("{:?}", wallet.status),
                        created_at: wallet.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    None => ApiResponse::Error { message: "Wallet not found".to_string() }
                }
            }
            
            ApiRequest::GetCheckWalletInfo { check_id } => {
                match blockchain_guard.hd_wallet_manager.get_check_wallet(&check_id) {
                    Some(check_wallet) => ApiResponse::CheckWalletInfo {
                        check_id: check_wallet.check_id.clone(),
                        wallet_address: check_wallet.wallet.address.clone(),
                        amount_subunits: check_wallet.amount_subunits,
                        is_activated: check_wallet.is_activated,
                        expires_at: check_wallet.expires_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    },
                    None => ApiResponse::Error { message: "Check wallet not found".to_string() }
                }
            }
            
            ApiRequest::GetWalletStatistics => {
                match blockchain_guard.hd_wallet_manager.get_wallet_statistics() {
                    Ok(stats) => ApiResponse::WalletStatistics {
                        total_wallets: stats.total_wallets,
                        total_check_wallets: stats.total_check_wallets,
                        active_wallets: stats.active_wallets,
                        inactive_wallets: stats.inactive_wallets,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            // KYC/AML API endpoints
            ApiRequest::RegisterUser { 
                email, phone, first_name, last_name, date_of_birth, nationality, address 
            } => {
                let date_of_birth_parsed = if let Some(dob_str) = date_of_birth {
                    chrono::DateTime::parse_from_rfc3339(&dob_str).ok().map(|dt| dt.with_timezone(&Utc))
                } else {
                    None
                };
                
                let address_parsed = address.map(|addr| Address {
                    street: addr.street,
                    city: addr.city,
                    state: addr.state,
                    postal_code: addr.postal_code,
                    country: addr.country,
                });
                
                let user_data = UserRegistrationData {
                    email,
                    phone,
                    first_name,
                    last_name,
                    date_of_birth: date_of_birth_parsed,
                    nationality,
                    address: address_parsed,
                };
                
                match blockchain_guard.kyc_aml_manager.register_user(user_data) {
                    Ok(user_id) => {
                        if let Some(user) = blockchain_guard.kyc_aml_manager.get_user(&user_id) {
                            ApiResponse::UserRegistered {
                                user_id,
                                email: user.email.clone(),
                                kyc_status: format!("{:?}", user.kyc_status),
                            }
                        } else {
                            ApiResponse::Error { message: "User not found after registration".to_string() }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::StartKYCProcess { user_id, kyc_level } => {
                let kyc_level_enum = match kyc_level.to_uppercase().as_str() {
                    "BASIC" => KYCLevel::Basic,
                    "ENHANCED" => KYCLevel::Enhanced,
                    "PREMIUM" => KYCLevel::Premium,
                    _ => return ApiResponse::Error { message: "Invalid KYC level. Use BASIC, ENHANCED, or PREMIUM".to_string() }
                };
                
                match blockchain_guard.kyc_aml_manager.start_kyc_process(&user_id, kyc_level_enum) {
                    Ok(()) => {
                        if let Some(user) = blockchain_guard.kyc_aml_manager.get_user(&user_id) {
                            ApiResponse::KYCProcessStarted {
                                user_id,
                                kyc_level,
                                status: format!("{:?}", user.kyc_status),
                            }
                        } else {
                            ApiResponse::Error { message: "User not found".to_string() }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::UploadDocument { user_id, document_type, file_hash, file_path } => {
                let document_type_enum = match document_type.to_uppercase().as_str() {
                    "PASSPORT" => DocumentType::Passport,
                    "IDCARD" => DocumentType::IdCard,
                    "DRIVERLICENSE" => DocumentType::DriverLicense,
                    "UTILITYBILL" => DocumentType::UtilityBill,
                    "BANKSTATEMENT" => DocumentType::BankStatement,
                    "PROOFOFADDRESS" => DocumentType::ProofOfAddress,
                    _ => return ApiResponse::Error { message: "Invalid document type".to_string() }
                };
                
                match blockchain_guard.kyc_aml_manager.upload_document(&user_id, document_type_enum, file_hash, file_path) {
                    Ok(document_id) => ApiResponse::DocumentUploaded {
                        document_id,
                        user_id,
                        document_type,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::VerifyDocument { user_id, document_id, verified_by, approved, rejection_reason } => {
                match blockchain_guard.kyc_aml_manager.verify_document(&user_id, &document_id, &verified_by, approved, rejection_reason) {
                    Ok(()) => ApiResponse::DocumentVerified {
                        document_id,
                        user_id,
                        approved,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::CompleteKYCProcess { user_id, verified_by } => {
                match blockchain_guard.kyc_aml_manager.complete_kyc_process(&user_id, &verified_by) {
                    Ok(()) => {
                        if let Some(user) = blockchain_guard.kyc_aml_manager.get_user(&user_id) {
                            ApiResponse::KYCProcessCompleted {
                                user_id,
                                kyc_status: format!("{:?}", user.kyc_status),
                                risk_score: user.risk_score,
                            }
                        } else {
                            ApiResponse::Error { message: "User not found".to_string() }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::AssignRole { user_id, role, assigned_by, expires_at } => {
                let role_enum = match role.to_uppercase().as_str() {
                    "SUPERADMIN" => UserRole::SuperAdmin,
                    "ADMIN" => UserRole::Admin,
                    "COMPLIANCE" => UserRole::Compliance,
                    "MASTEROWNER" => UserRole::MasterOwner,
                    "FRANCHISEOWNER" => UserRole::FranchiseOwner,
                    "POSOPERATOR" => UserRole::POSOperator,
                    "CASHIER" => UserRole::Cashier,
                    "CUSTOMER" => UserRole::Customer,
                    "INVESTOR" => UserRole::Investor,
                    "SYSTEM" => UserRole::System,
                    "AUDITOR" => UserRole::Auditor,
                    _ => return ApiResponse::Error { message: "Invalid role".to_string() }
                };
                
                let expires_at_parsed = if let Some(exp_str) = expires_at {
                    chrono::DateTime::parse_from_rfc3339(&exp_str).ok().map(|dt| dt.with_timezone(&Utc))
                } else {
                    None
                };
                
                match blockchain_guard.kyc_aml_manager.assign_role(&user_id, role_enum, assigned_by.clone(), expires_at_parsed) {
                    Ok(()) => ApiResponse::RoleAssigned {
                        user_id,
                        role,
                        assigned_by,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::CheckPermission { user_id, permission } => {
                let permission_enum = match permission.to_uppercase().as_str() {
                    "MANAGEUSERS" => Permission::ManageUsers,
                    "MANAGEROLES" => Permission::ManageRoles,
                    "VIEWAUDITLOGS" => Permission::ViewAuditLogs,
                    "MANAGESYSTEM" => Permission::ManageSystem,
                    "VERIFYKYC" => Permission::VerifyKYC,
                    "VIEWKYCDATA" => Permission::ViewKYCData,
                    "MANAGECOMPLIANCE" => Permission::ManageCompliance,
                    "GENERATEREPORTS" => Permission::GenerateReports,
                    "PROCESSTRANSACTIONS" => Permission::ProcessTransactions,
                    "MANAGENODES" => Permission::ManageNodes,
                    "VIEWFINANCIALS" => Permission::ViewFinancials,
                    "MANAGEMENU" => Permission::ManageMenu,
                    "CREATEORDERS" => Permission::CreateOrders,
                    "VIEWOWNDATA" => Permission::ViewOwnData,
                    "TRANSFERTOKENS" => Permission::TransferTokens,
                    "VOTEONPROPOSALS" => Permission::VoteOnProposals,
                    _ => return ApiResponse::Error { message: "Invalid permission".to_string() }
                };
                
                let has_permission = blockchain_guard.kyc_aml_manager.has_permission(&user_id, &permission_enum);
                
                ApiResponse::PermissionCheck {
                    user_id,
                    permission,
                    has_permission,
                }
            }
            
            ApiRequest::GetUserInfo { user_id } => {
                match blockchain_guard.kyc_aml_manager.get_user(&user_id) {
                    Some(user) => ApiResponse::UserInfo {
                        user_id: user.user_id.clone(),
                        email: user.email.clone(),
                        first_name: user.first_name.clone(),
                        last_name: user.last_name.clone(),
                        kyc_status: format!("{:?}", user.kyc_status),
                        kyc_level: format!("{:?}", user.kyc_level),
                        risk_score: user.risk_score,
                    },
                    None => ApiResponse::Error { message: "User not found".to_string() }
                }
            }
            
            ApiRequest::GetKYCStatistics => {
                match blockchain_guard.kyc_aml_manager.get_kyc_statistics() {
                    Ok(stats) => ApiResponse::KYCStatistics {
                        total_users: stats.total_users,
                        total_documents: stats.total_documents,
                        verified: stats.verified,
                        pending: stats.pending,
                        rejected: stats.rejected,
                        high_risk: stats.high_risk,
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            // Database API endpoints
            ApiRequest::InitializeDatabase { host, port, database, username, password } => {
                let config = DatabaseConfig {
                    host,
                    port,
                    database,
                    username,
                    password,
                    max_connections: 10,
                    connection_timeout: 30,
                };
                
                match DatabaseManager::new(config.clone()).await {
                    Ok(db_manager) => {
                        blockchain_guard.database_manager = Some(db_manager);
                        ApiResponse::DatabaseInitialized {
                            message: "Database initialized successfully".to_string(),
                            config: format!("{}:{}@{}:{}", config.username, "***", config.host, config.port),
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e.to_string() }
                }
            }
            
            ApiRequest::GetDatabaseStats => {
                match &blockchain_guard.database_manager {
                    Some(db_manager) => {
                        match db_manager.get_database_stats().await {
                            Ok(stats) => ApiResponse::DatabaseStats {
                                total_users: stats.total_users,
                                total_documents: stats.total_documents,
                                total_wallets: stats.total_wallets,
                                total_transactions: stats.total_transactions,
                                total_audit_logs: stats.total_audit_logs,
                            },
                            Err(e) => ApiResponse::Error { message: e.to_string() }
                        }
                    },
                    None => ApiResponse::Error { message: "Database not initialized".to_string() }
                }
            }
            
            ApiRequest::CleanupOldData { days } => {
                match &blockchain_guard.database_manager {
                    Some(db_manager) => {
                        match db_manager.cleanup_old_data(days).await {
                            Ok(stats) => ApiResponse::CleanupCompleted {
                                deleted_audit_logs: stats.deleted_audit_logs,
                                deleted_expired_checks: stats.deleted_expired_checks,
                            },
                            Err(e) => ApiResponse::Error { message: e.to_string() }
                        }
                    },
                    None => ApiResponse::Error { message: "Database not initialized".to_string() }
                }
            }
            
            ApiRequest::GetUserFromDatabase { user_id } => {
                match &blockchain_guard.database_manager {
                    Some(db_manager) => {
                        match db_manager.get_user(&user_id).await {
                            Ok(Some(user)) => ApiResponse::DatabaseUserInfo {
                                user_id: user.user_id,
                                email: user.email,
                                first_name: user.first_name,
                                last_name: user.last_name,
                                kyc_status: user.kyc_status,
                                created_at: user.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                            },
                            Ok(None) => ApiResponse::Error { message: "User not found".to_string() },
                            Err(e) => ApiResponse::Error { message: e.to_string() }
                        }
                    },
                    None => ApiResponse::Error { message: "Database not initialized".to_string() }
                }
            }
            
            ApiRequest::GetAllUsersFromDatabase { limit, offset } => {
                match &blockchain_guard.database_manager {
                    Some(db_manager) => {
                        match db_manager.get_all_users(limit, offset).await {
                            Ok(users) => {
                                let db_users: Vec<DatabaseUserInfo> = users.into_iter().map(|user| {
                                    DatabaseUserInfo {
                                        user_id: user.user_id,
                                        email: user.email,
                                        first_name: user.first_name,
                                        last_name: user.last_name,
                                        kyc_status: user.kyc_status,
                                        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                                    }
                                }).collect();
                                
                                ApiResponse::DatabaseUsersList {
                                    users: db_users,
                                    total_count: users.len() as u32,
                                }
                            },
                            Err(e) => ApiResponse::Error { message: e.to_string() }
                        }
                    },
                    None => ApiResponse::Error { message: "Database not initialized".to_string() }
                }
            }
            
            // Observability API endpoints
            ApiRequest::GetLogs { level, limit } => {
                let log_level = if let Some(level_str) = level {
                    match level_str.to_uppercase().as_str() {
                        "TRACE" => Some(LogLevel::Trace),
                        "DEBUG" => Some(LogLevel::Debug),
                        "INFO" => Some(LogLevel::Info),
                        "WARN" => Some(LogLevel::Warn),
                        "ERROR" => Some(LogLevel::Error),
                        "FATAL" => Some(LogLevel::Fatal),
                        _ => None,
                    }
                } else {
                    None
                };
                
                let logs = blockchain_guard.observability_manager.get_logs(log_level, limit.map(|l| l as usize)).await;
                let log_infos: Vec<LogEntryInfo> = logs.into_iter().map(|log| {
                    LogEntryInfo {
                        timestamp: log.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        level: format!("{:?}", log.level),
                        message: log.message,
                        module: log.module,
                        thread_id: log.thread_id,
                        metadata: log.metadata,
                    }
                }).collect();
                
                ApiResponse::LogsList {
                    logs: log_infos.clone(),
                    total_count: log_infos.len() as u32,
                }
            }
            
            ApiRequest::GetMetrics { metric_type, limit } => {
                let metric_type_enum = if let Some(type_str) = metric_type {
                    match type_str.to_uppercase().as_str() {
                        "COUNTER" => Some(MetricType::Counter),
                        "GAUGE" => Some(MetricType::Gauge),
                        "HISTOGRAM" => Some(MetricType::Histogram),
                        "SUMMARY" => Some(MetricType::Summary),
                        _ => None,
                    }
                } else {
                    None
                };
                
                let metrics = blockchain_guard.observability_manager.get_metrics(metric_type_enum, limit.map(|l| l as usize)).await;
                let metric_infos: Vec<MetricInfo> = metrics.into_iter().map(|metric| {
                    MetricInfo {
                        name: metric.name,
                        value: metric.value,
                        timestamp: metric.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        metric_type: format!("{:?}", metric.metric_type),
                        labels: metric.labels,
                    }
                }).collect();
                
                ApiResponse::MetricsList {
                    metrics: metric_infos.clone(),
                    total_count: metric_infos.len() as u32,
                }
            }
            
            ApiRequest::GetAlerts { status, severity } => {
                let alert_status = if let Some(status_str) = status {
                    match status_str.to_uppercase().as_str() {
                        "FIRING" => Some(AlertStatus::Firing),
                        "RESOLVED" => Some(AlertStatus::Resolved),
                        "SUPPRESSED" => Some(AlertStatus::Suppressed),
                        _ => None,
                    }
                } else {
                    None
                };
                
                let alert_severity = if let Some(severity_str) = severity {
                    match severity_str.to_uppercase().as_str() {
                        "INFO" => Some(AlertSeverity::Info),
                        "WARNING" => Some(AlertSeverity::Warning),
                        "CRITICAL" => Some(AlertSeverity::Critical),
                        "EMERGENCY" => Some(AlertSeverity::Emergency),
                        _ => None,
                    }
                } else {
                    None
                };
                
                let alerts = blockchain_guard.observability_manager.get_alerts(alert_status, alert_severity).await;
                let alert_infos: Vec<AlertInfo> = alerts.into_iter().map(|alert| {
                    AlertInfo {
                        alert_id: alert.alert_id,
                        name: alert.name,
                        description: alert.description,
                        severity: format!("{:?}", alert.severity),
                        status: format!("{:?}", alert.status),
                        created_at: alert.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        labels: alert.labels,
                    }
                }).collect();
                
                ApiResponse::AlertsList {
                    alerts: alert_infos.clone(),
                    total_count: alert_infos.len() as u32,
                }
            }
            
            ApiRequest::GetObservabilityStats => {
                let stats = blockchain_guard.observability_manager.get_statistics().await;
                ApiResponse::ObservabilityStats {
                    total_logs: stats.total_logs,
                    total_metrics: stats.total_metrics,
                    total_alerts: stats.total_alerts,
                    total_requests: stats.total_requests,
                    total_errors: stats.total_errors,
                    active_connections: stats.active_connections,
                }
            }
            
            ApiRequest::GeneratePrometheusMetrics => {
                let metrics = blockchain_guard.observability_manager.generate_prometheus_metrics().await;
                ApiResponse::PrometheusMetrics { metrics }
            }
            
            ApiRequest::CreateAlert { alert_id, name, description, severity } => {
                let alert_severity = match severity.to_uppercase().as_str() {
                    "INFO" => AlertSeverity::Info,
                    "WARNING" => AlertSeverity::Warning,
                    "CRITICAL" => AlertSeverity::Critical,
                    "EMERGENCY" => AlertSeverity::Emergency,
                    _ => return ApiResponse::Error { message: "Invalid severity level".to_string() }
                };
                
                blockchain_guard.observability_manager.create_alert(&alert_id, &name, &description, alert_severity, None).await;
                
                ApiResponse::AlertCreated {
                    alert_id,
                    name,
                    status: "Firing".to_string(),
                }
            }
            
            ApiRequest::ResolveAlert { alert_id } => {
                blockchain_guard.observability_manager.resolve_alert(&alert_id).await;
                
                ApiResponse::AlertResolved {
                    alert_id,
                    status: "Resolved".to_string(),
                }
            }
            
            // API Versioning endpoints
            ApiRequest::GetApiVersion => {
                let current_version = blockchain_guard.api_version_manager.get_current_version();
                let version_info = blockchain_guard.api_version_manager.get_version_info(current_version);
                
                if let Some(info) = version_info {
                    ApiResponse::ApiVersionInfo {
                        version: current_version.to_string(),
                        status: format!("{:?}", info.status),
                        release_date: info.release_date.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    }
                } else {
                    ApiResponse::Error { message: "Version info not found".to_string() }
                }
            }
            
            ApiRequest::GetSupportedVersions => {
                let supported_versions = blockchain_guard.api_version_manager.get_supported_versions();
                let current_version = blockchain_guard.api_version_manager.get_current_version();
                
                let version_strings: Vec<String> = supported_versions.iter().map(|v| v.to_string()).collect();
                
                ApiResponse::SupportedVersions {
                    versions: version_strings,
                    current_version: current_version.to_string(),
                }
            }
            
            ApiRequest::GetVersionInfo { version } => {
                match ApiVersion::from_string(&version) {
                    Ok(api_version) => {
                        if let Some(info) = blockchain_guard.api_version_manager.get_version_info(&api_version) {
                            ApiResponse::VersionDetails {
                                version: api_version.to_string(),
                                status: format!("{:?}", info.status),
                                release_date: info.release_date.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                                deprecation_date: info.deprecation_date.map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
                                retirement_date: info.retirement_date.map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
                                new_features: info.new_features.clone(),
                                breaking_changes: info.breaking_changes.clone(),
                                bug_fixes: info.bug_fixes.clone(),
                            }
                        } else {
                            ApiResponse::Error { message: "Version info not found".to_string() }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e }
                }
            }
            
            ApiRequest::GetVersionWarning { version } => {
                match ApiVersion::from_string(&version) {
                    Ok(api_version) => {
                        if let Some(warning) = blockchain_guard.api_version_manager.get_version_warning(&api_version) {
                            ApiResponse::VersionWarning {
                                warning_type: format!("{:?}", warning.warning_type),
                                message: warning.message,
                                retirement_date: warning.retirement_date.map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
                            }
                        } else {
                            ApiResponse::Error { message: "No warning for this version".to_string() }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e }
                }
            }
            
            ApiRequest::GetChangelog { version } => {
                let api_version = if let Some(version_str) = version {
                    match ApiVersion::from_string(&version_str) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                } else {
                    None
                };
                
                let changelog_entries = blockchain_guard.api_version_manager.get_changelog(api_version.as_ref());
                let entry_infos: Vec<ChangelogEntryInfo> = changelog_entries.into_iter().map(|entry| {
                    let changes: Vec<ChangeEntryInfo> = entry.changes.iter().map(|change| {
                        ChangeEntryInfo {
                            change_type: format!("{:?}", change.change_type),
                            description: change.description.clone(),
                            impact: format!("{:?}", change.impact),
                            affected_endpoints: change.affected_endpoints.clone(),
                        }
                    }).collect();
                    
                    ChangelogEntryInfo {
                        version: entry.version.to_string(),
                        date: entry.date.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                        changes,
                    }
                }).collect();
                
                ApiResponse::Changelog {
                    entries: entry_infos.clone(),
                    total_count: entry_infos.len() as u32,
                }
            }
            
            ApiRequest::GetVersionStatistics => {
                let stats = blockchain_guard.api_version_manager.get_version_statistics();
                ApiResponse::VersionStats {
                    total_versions: stats.total_versions,
                    current_version: stats.current_version,
                    supported_versions: stats.supported_versions,
                    deprecated_versions: stats.deprecated_versions,
                    retired_versions: stats.retired_versions,
                    total_endpoints: stats.total_endpoints,
                    deprecated_endpoints: stats.deprecated_endpoints,
                }
            }
            
            ApiRequest::GenerateOpenApiSpec { version } => {
                match ApiVersion::from_string(&version) {
                    Ok(api_version) => {
                        match blockchain_guard.api_version_manager.generate_openapi_spec(&api_version) {
                            Ok(spec) => {
                                match serde_json::to_string_pretty(&spec) {
                                    Ok(spec_json) => ApiResponse::OpenApiSpec {
                                        spec: spec_json,
                                        version: api_version.to_string(),
                                    },
                                    Err(e) => ApiResponse::Error { message: format!("Failed to serialize OpenAPI spec: {}", e) }
                                }
                            },
                            Err(e) => ApiResponse::Error { message: e }
                        }
                    },
                    Err(e) => ApiResponse::Error { message: e }
                }
            }
            
            ApiRequest::CheckVersionCompatibility { version1, version2 } => {
                match (ApiVersion::from_string(&version1), ApiVersion::from_string(&version2)) {
                    (Ok(v1), Ok(v2)) => {
                        let compatible = blockchain_guard.api_version_manager.are_versions_compatible(&v1, &v2);
                        let reason = if !compatible {
                            Some("Major versions must match for compatibility".to_string())
                        } else {
                            None
                        };
                        
                        ApiResponse::VersionCompatibility {
                            version1: v1.to_string(),
                            version2: v2.to_string(),
                            compatible,
                            reason,
                        }
                    },
                    (Err(e), _) => ApiResponse::Error { message: format!("Invalid version1: {}", e) },
                    (_, Err(e)) => ApiResponse::Error { message: format!("Invalid version2: {}", e) },
                }
            }
        }
    }
}

// UI System
struct UI {
    current_user: Option<String>,
    blockchain: Blockchain,
}

#[cfg_attr(test, allow(dead_code))]
impl UI {
    fn new(blockchain: Blockchain) -> Self {
        UI {
            current_user: None,
            blockchain,
        }
    }

    fn show_main_menu(&self) {
        println!("\n🍔 Fast Food Truck Blockchain UI 🍔");
        println!("=====================================");
        
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                println!("Logged in as: {}", user);
                println!("Role: {:?}", holder.role);
                println!("Security Tokens: {:.2}", holder.security_tokens);
                println!("Utility Tokens: {:.2}", holder.utility_tokens);
                println!("Checks: {}", holder.checks.len());
                println!("Blockchain Accounts: {}", holder.blockchain_accounts.len());
            }
        } else {
            println!("Not logged in");
        }
        
        println!("\nOptions:");
        println!("1. Login with QR Code");
        println!("2. Login with Check Number");
        println!("3. View Menu Items");
        println!("4. Suggest Menu Item");
        println!("5. Add Detailed Menu Item");
        println!("6. Make Item Available for Voting");
        println!("7. Vote on Menu Items");
        println!("8. View Orders");
        println!("9. Confirm Order");
        println!("10. View My Checks");
        println!("11. Activate Account");
        println!("12. List Account for Sale");
        println!("13. View Blockchain Status");
        println!("14. Start API Server");
        println!("0. Exit");
    }

    fn show_unauthorized_ui(&self) {
        println!("\n👤 Unauthorized User Interface");
        println!("===============================");
        println!("You don't have any tokens yet.");
        println!("Make a purchase to get started!");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Make Purchase (Demo)");
        println!("3. Back to Main Menu");
    }

    fn show_starter_ui(&self) {
        println!("\n🌟 Starter User Interface");
        println!("==========================");
        println!("You have 1-5% of security tokens!");
        println!("You can vote on menu items.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Vote on Menu Items");
        println!("3. View My Checks");
        println!("4. Activate Account");
        println!("5. Back to Main Menu");
    }

    fn show_middle_player_ui(&self) {
        println!("\n🎯 Middle Player Interface");
        println!("===========================");
        println!("You have 5-10% of security tokens!");
        println!("You can vote and have more influence.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Vote on Menu Items");
        println!("3. View My Checks");
        println!("4. Activate Account");
        println!("5. List Account for Sale");
        println!("6. Back to Main Menu");
    }

    fn show_big_stack_ui(&self) {
        println!("\n💎 Big Stack Interface");
        println!("=======================");
        println!("You have 10%+ of security tokens!");
        println!("You can suggest menu items and vote.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Suggest Menu Item");
        println!("3. Vote on Menu Items");
        println!("4. View My Checks");
        println!("5. Activate Account");
        println!("6. List Account for Sale");
        println!("7. Back to Main Menu");
    }

    fn show_main_owner_ui(&self) {
        println!("\n👑 Main Owner Interface");
        println!("========================");
        println!("You are the main owner!");
        println!("You control the blockchain and can suggest menu items.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Suggest Menu Item");
        println!("3. Vote on Menu Items");
        println!("4. View All Checks");
        println!("5. View Blockchain Status");
        println!("6. Mine Block");
        println!("7. Back to Main Menu");
    }

    fn login_with_qr(&mut self) {
        println!("\n📱 Login with QR Code");
        println!("=====================");
        println!("Scan the QR code from your check:");
        
        // Simulate QR code scanning
        let qr_data = "check_id_123|123456|0xabcd1234";
        println!("QR Data: {}", qr_data);
        
        // Parse QR data
        let parts: Vec<&str> = qr_data.split('|').collect();
        if parts.len() == 3 {
            let check_id = parts[0];
            let activation_code = parts[1];
            let account = parts[2];
            
            println!("Check ID: {}", check_id);
            println!("Activation Code: {}", activation_code);
            println!("Account: {}", account);
            
            // Find the holder with this check
            for (address, holder) in &self.blockchain.token_holders {
                if holder.checks.iter().any(|c| c.check_id == check_id) {
                    self.current_user = Some(address.clone());
                    println!("✅ Login successful!");
                    return;
                }
            }
        }
        
        println!("❌ Invalid QR code or check not found");
    }

    fn login_with_check(&mut self) {
        println!("\n🧾 Login with Check Number");
        println!("==========================");
        println!("Enter your check ID:");
        
        // Simulate user input
        let check_id = "check_id_123";
        println!("Check ID: {}", check_id);
        
        // Find the holder with this check
        for (address, holder) in &self.blockchain.token_holders {
            if holder.checks.iter().any(|c| c.check_id == check_id) {
                self.current_user = Some(address.clone());
                println!("✅ Login successful!");
                return;
            }
        }
        
        println!("❌ Check not found");
    }

    fn view_menu_items(&self) {
        println!("\n🍽️ Menu Items");
        println!("=============");
        
        if self.blockchain.menu_items.is_empty() {
            println!("No menu items available.");
            return;
        }
        
        for (i, item) in self.blockchain.menu_items.iter().enumerate() {
            let status_str = match item.status {
                MenuItemStatus::Proposed => "Proposed",
                MenuItemStatus::Voting => "Voting",
                MenuItemStatus::Approved => "Approved",
                MenuItemStatus::Rejected => "Rejected",
                MenuItemStatus::Active => "Active",
            };
            
            println!("{}. {} - {}", i + 1, item.name, item.description);
            println!("   Status: {}", status_str);
            println!("   Suggested by: {}", item.suggested_by);
            println!("   Votes: For {:.2}, Against {:.2}", item.votes_for, item.votes_against);
            println!();
        }
    }

    fn suggest_menu_item(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner && holder.role != UserRole::BigStack {
                    println!("❌ Only main owners and big stacks can suggest menu items");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n💡 Suggest Menu Item");
        println!("====================");
        
        // Simulate user input
        let name = "Vegan Pizza";
        let description = "Plant-based pizza with vegan cheese";
        let price = 15.99;
        
        println!("Name: {}", name);
        println!("Description: {}", description);
        println!("Price: ${:.2}", price);
        
        if let Some(user) = &self.current_user {
            match self.blockchain.suggest_menu_item(name.to_string(), description.to_string(), price, user.clone()) {
                Ok(()) => println!("✅ Menu item suggested successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn add_detailed_menu_item(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can add detailed menu items");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🍽️ Add Detailed Menu Item");
        println!("==========================");
        
        // Simulate detailed menu item creation
        let name = "Gourmet Burger";
        let description = "Premium beef burger with special sauce";
        let price = 18.50;
        let availability = 20;
        let priority_rank = 8;
        let cooking_time_minutes = 12;
        
        let ingredients = vec![
            Ingredient { name: "Beef Patty".to_string(), amount_grams: 200.0, calories: 350.0 },
            Ingredient { name: "Bun".to_string(), amount_grams: 80.0, calories: 200.0 },
            Ingredient { name: "Cheese".to_string(), amount_grams: 30.0, calories: 120.0 },
            Ingredient { name: "Lettuce".to_string(), amount_grams: 20.0, calories: 5.0 },
            Ingredient { name: "Tomato".to_string(), amount_grams: 25.0, calories: 10.0 },
        ];
        
        println!("Name: {}", name);
        println!("Description: {}", description);
        println!("Price: ${:.2}", price);
        println!("Availability: {} pieces", availability);
        println!("Priority Rank: {}/10", priority_rank);
        println!("Cooking Time: {} minutes", cooking_time_minutes);
        println!("Ingredients: {} items", ingredients.len());
        
        if let Some(user) = &self.current_user {
            match self.blockchain.add_menu_item_with_details(
                name.to_string(), 
                description.to_string(), 
                price, 
                availability, 
                priority_rank, 
                cooking_time_minutes, 
                ingredients, 
                user.clone()
            ) {
                Ok(()) => println!("✅ Detailed menu item added successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn make_item_available_for_voting(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can make items available for voting");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🗳️ Make Item Available for Voting");
        println!("==================================");
        
        // Show items that can be made available for voting
        let proposed_items: Vec<_> = self.blockchain.menu_items.iter()
            .filter(|item| item.status == MenuItemStatus::Proposed && !item.is_available_for_voting)
            .collect();
        
        if proposed_items.is_empty() {
            println!("No items available for voting.");
            return;
        }
        
        for (i, item) in proposed_items.iter().enumerate() {
            println!("{}. {} - {}", i + 1, item.name, item.description);
        }
        
        // Simulate selecting first item
        if let Some(item) = proposed_items.first() {
            println!("Making {} available for voting...", item.name);
            match self.blockchain.make_menu_item_available_for_voting(item.id.clone()) {
                Ok(()) => println!("✅ Item is now available for voting!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn view_orders(&self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can view all orders");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n📋 All Orders");
        println!("=============");
        
        if self.blockchain.orders.is_empty() {
            println!("No orders found.");
            return;
        }
        
        for (i, order) in self.blockchain.orders.iter().enumerate() {
            let status_str = match order.status {
                OrderStatus::Pending => "Pending",
                OrderStatus::Confirmed => "Confirmed",
                OrderStatus::Cancelled => "Cancelled",
                OrderStatus::Completed => "Completed",
            };
            
            println!("{}. Order ID: {}", i + 1, order.id);
            println!("   Customer: {}", order.customer_wallet);
            println!("   Amount: {}", config_utils::format_gel(order.total_amount_subunits));
            println!("   Status: {}", status_str);
            println!("   Delivery Time: {} minutes", order.delivery_time_minutes);
            println!("   Items: {} items", order.items.len());
            if let Some(reason) = &order.cancellation_reason {
                println!("   Cancellation Reason: {}", reason);
            }
            println!();
        }
    }

    fn confirm_order(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can confirm orders");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n✅ Confirm Order");
        println!("================");
        
        let pending_orders: Vec<_> = self.blockchain.orders.iter()
            .filter(|order| order.status == OrderStatus::Pending)
            .collect();
        
        if pending_orders.is_empty() {
            println!("No pending orders found.");
            return;
        }
        
        for (i, order) in pending_orders.iter().enumerate() {
            println!("{}. Order ID: {} - Customer: {} - Amount: {}", 
                i + 1, order.id, order.customer_wallet, config_utils::format_gel(order.total_amount_subunits));
        }
        
        // Simulate confirming first order
        if let Some(order) = pending_orders.first() {
            println!("Confirming order: {}", order.id);
            match self.blockchain.confirm_order(order.id.clone()) {
                Ok(()) => println!("✅ Order confirmed successfully! Tokens issued to customer."),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn vote_on_menu_items(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.utility_tokens <= 0.0 {
                    println!("❌ You don't have any voting power");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🗳️ Vote on Menu Items");
        println!("====================");
        
        let voting_items: Vec<_> = self.blockchain.menu_items.iter()
            .filter(|item| item.status == MenuItemStatus::Voting)
            .collect();
        
        if voting_items.is_empty() {
            println!("No items currently open for voting.");
            return;
        }
        
        for (i, item) in voting_items.iter().enumerate() {
            println!("{}. {} - {}", i + 1, item.name, item.description);
        }
        
        // Simulate voting on first item
        if let Some(item) = voting_items.first() {
            if let Some(user) = &self.current_user {
                match self.blockchain.vote_on_menu_item(user.clone(), item.id.clone(), true) {
                    Ok(()) => println!("✅ Vote cast successfully!"),
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
        }
    }

    fn view_my_checks(&self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                println!("\n🧾 My Checks");
                println!("============");
                
                if holder.checks.is_empty() {
                    println!("No checks found.");
                    return;
                }
                
                for (i, check) in holder.checks.iter().enumerate() {
                    println!("{}. Check ID: {}", i + 1, check.check_id);
                    println!("   Amount: {}", config_utils::format_gel(check.amount_subunits));
                    println!("   Food Items: {}", check.food_items.join(", "));
                    println!("   Activation Code: {}", check.activation_code);
                    println!("   Status: {}", if check.is_activated { "Activated" } else { "Not Activated" });
                    println!("   QR Code: {}", check.qr_code);
                    println!();
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn activate_account(&mut self) {
        if let Some(user) = &self.current_user {
            println!("\n🔓 Activate Account");
            println!("==================");
            
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                let inactive_checks: Vec<_> = holder.checks.iter()
                    .filter(|c| !c.is_activated)
                    .collect();
                
                if inactive_checks.is_empty() {
                    println!("No inactive checks found.");
                    return;
                }
                
                // Simulate activating first check
                if let Some(check) = inactive_checks.first() {
                    let personal_data = PersonalData {
                        name: "John Doe".to_string(),
                        email: "john@example.com".to_string(),
                        phone: "+1234567890".to_string(),
                        wallet_address: Some("0x1234567890abcdef".to_string()),
                    };
                    
                    println!("Activating check: {}", check.check_id);
                    println!("Personal data: {:?}", personal_data);
                    
                    let check_id = check.check_id.clone();
                    let activation_code = check.activation_code.clone();
                    match self.blockchain.activate_account(&check_id, &activation_code, personal_data) {
                        Ok(()) => println!("✅ Account activated successfully!"),
                        Err(e) => println!("❌ Error: {}", e),
                    }
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn list_account_for_sale(&mut self) {
        if let Some(user) = &self.current_user {
            println!("\n💰 List Account for Sale");
            println!("=======================");
            
            if let Some(holder) = self.blockchain.token_holders.get_mut(user) {
                let active_addresses: Vec<String> = holder.blockchain_accounts
                    .values()
                    .filter(|acc| acc.status == AccountStatus::Active)
                    .map(|acc| acc.address.clone())
                    .collect();
                
                if active_addresses.is_empty() {
                    println!("No active accounts to list for sale.");
                    return;
                }
                
                // Simulate listing first account
                let addr = &active_addresses[0];
                println!("Listing account: {}", addr);
                    
                if let Some(acc) = holder.blockchain_accounts.get_mut(addr) {
                        match acc.list_for_sale() {
                            Ok(()) => println!("✅ Account listed for sale!"),
                            Err(e) => println!("❌ Error: {}", e),
                    }
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn view_blockchain_status(&self) {
        println!("\n🔗 Blockchain Status");
        println!("===================");
        println!("Chain valid: {}", self.blockchain.is_chain_valid());
        println!("Total blocks: {}", self.blockchain.chain.len());
        println!("Total security tokens: {:.2}", 
            self.blockchain.token_holders.values().map(|h| h.security_tokens).sum::<f64>());
        println!("Total utility tokens: {:.2}", self.blockchain.utility_token.total_supply);
        println!("Main owner: {}", self.blockchain.main_owner);
        
        println!("\nToken Holders:");
        for (address, holder) in &self.blockchain.token_holders {
            println!("  {}: {:?} - Security: {:.2}, Utility: {:.2}", 
                address, holder.role, holder.security_tokens, holder.utility_tokens);
        }
    }

    fn mine_block(&mut self) {
        println!("\n⛏️ Mining Block");
        println!("==============");
        
        match self.blockchain.mine_block() {
            Ok(()) => println!("✅ Block mined successfully!"),
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    fn run(&mut self) {
        loop {
            self.show_main_menu();
            
            // Simulate user choice
            let choice = 1; // This would normally be user input
            
            match choice {
                1 => self.login_with_qr(),
                2 => self.login_with_check(),
                3 => self.view_menu_items(),
                4 => self.suggest_menu_item(),
                5 => self.add_detailed_menu_item(),
                6 => self.make_item_available_for_voting(),
                7 => self.vote_on_menu_items(),
                8 => self.view_orders(),
                9 => self.confirm_order(),
                10 => self.view_my_checks(),
                11 => self.activate_account(),
                12 => self.list_account_for_sale(),
                13 => self.view_blockchain_status(),
                14 => {
                    println!("🌐 Starting API Server...");
                    let blockchain_arc = Arc::new(Mutex::new(self.blockchain.clone()));
                    let api_server = SimpleServer::new(3000);
                    // api_server.start(); // Запуск в отдельном потоке
                },
                0 => {
                    println!("👋 Goodbye!");
                    break;
                }
                _ => println!("❌ Invalid choice"),
            }
            
            // Show role-specific UI
            if let Some(user) = &self.current_user {
                if let Some(holder) = self.blockchain.token_holders.get(user) {
                    match holder.role {
                        UserRole::Unauthorized => self.show_unauthorized_ui(),
                        UserRole::Starter => self.show_starter_ui(),
                        UserRole::MiddlePlayer => self.show_middle_player_ui(),
                        UserRole::BigStack => self.show_big_stack_ui(),
                        UserRole::MainOwner => self.show_main_owner_ui(),
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🍔 Fast Food Truck Blockchain with Security Tokens & 1:1 Utility Tokens 🍔\n");
    
    // Initialize blockchain with main owner
    let main_owner = "MainOwner_Alice".to_string();
    let mut blockchain = Blockchain::new(main_owner.clone());
    
    // Initialize video surveillance system
    println!("Initializing video surveillance system...");
    let twitch_config = TwitchConfig {
        client_id: "your_twitch_client_id".to_string(),
        client_secret: "your_twitch_client_secret".to_string(),
        access_token: "your_twitch_access_token".to_string(),
        refresh_token: "your_twitch_refresh_token".to_string(),
        channel_name: "hotpotspot_georgia".to_string(),
        stream_key: "your_twitch_stream_key".to_string(),
        webhook_secret: "your_webhook_secret".to_string(),
    };
    
    let youtube_config = YouTubeConfig {
        client_id: "your_youtube_client_id".to_string(),
        client_secret: "your_youtube_client_secret".to_string(),
        refresh_token: "your_youtube_refresh_token".to_string(),
        channel_id: "your_youtube_channel_id".to_string(),
        stream_key: "your_youtube_stream_key".to_string(),
        api_key: "your_youtube_api_key".to_string(),
    };
    
    let streaming_config = StreamingConfig {
        twitch_client_id: twitch_config.client_id,
        twitch_client_secret: twitch_config.client_secret,
        twitch_access_token: twitch_config.access_token,
        youtube_client_id: youtube_config.client_id,
        youtube_client_secret: youtube_config.client_secret,
        youtube_refresh_token: youtube_config.refresh_token,
        stream_quality: StreamQuality::Medium,
        default_anonymization: AnonymizationZone::FaceReplacement,
    };
    
    let video_system = Arc::new(VideoSurveillanceSystem::new(streaming_config));
    let video_api_handler = Arc::new(VideoAPIHandler::new(video_system.clone()));
    let video_http_handler = Arc::new(VideoHTTPHandler::new(video_api_handler));
    
    // Add sample cameras
    println!("Adding sample cameras...");
    let sample_cameras = vec![
        CameraConfig {
            camera_id: "CAM_EXT_001".to_string(),
            camera_type: CameraType::External,
            location: "Main Entrance".to_string(),
            ip_address: "192.168.1.100".to_string(),
            port: 8080,
            resolution: (1920, 1080),
            fps: 30,
            anonymization_zone: AnonymizationZone::FullFaceBlur,
            requires_consent: false,
            max_recording_duration: None,
            stream_to_twitch: true,
            stream_to_youtube: false,
        },
        CameraConfig {
            camera_id: "CAM_PROD_001".to_string(),
            camera_type: CameraType::Production,
            location: "Kitchen Area".to_string(),
            ip_address: "192.168.1.101".to_string(),
            port: 8080,
            resolution: (1280, 720),
            fps: 30,
            anonymization_zone: AnonymizationZone::FullFaceBlur,
            requires_consent: false,
            max_recording_duration: None,
            stream_to_twitch: true,
            stream_to_youtube: true,
        },
        CameraConfig {
            camera_id: "CAM_TABLE_001".to_string(),
            camera_type: CameraType::CustomerTable,
            location: "Table 1".to_string(),
            ip_address: "192.168.1.102".to_string(),
            port: 8080,
            resolution: (1920, 1080),
            fps: 30,
            anonymization_zone: AnonymizationZone::FaceReplacement,
            requires_consent: true,
            max_recording_duration: Some(std::time::Duration::from_secs(30 * 60)), // 30 minutes
            stream_to_twitch: true,
            stream_to_youtube: true,
        },
    ];
    
    for camera in sample_cameras {
        if let Err(e) = video_system.add_camera(camera).await {
            println!("Warning: Failed to add camera: {}", e);
        }
    }
    
    println!("Video surveillance system initialized successfully!");
    
    // Добавляем примеры меню с полной информацией
    println!("Adding sample menu items...");
    
    let burger_ingredients = vec![
        Ingredient { name: "Beef Patty".to_string(), amount_grams: 200.0, calories: 350.0 },
        Ingredient { name: "Bun".to_string(), amount_grams: 80.0, calories: 200.0 },
        Ingredient { name: "Cheese".to_string(), amount_grams: 30.0, calories: 120.0 },
        Ingredient { name: "Lettuce".to_string(), amount_grams: 20.0, calories: 5.0 },
        Ingredient { name: "Tomato".to_string(), amount_grams: 25.0, calories: 10.0 },
    ];
    
    let pizza_ingredients = vec![
        Ingredient { name: "Pizza Dough".to_string(), amount_grams: 150.0, calories: 300.0 },
        Ingredient { name: "Tomato Sauce".to_string(), amount_grams: 50.0, calories: 25.0 },
        Ingredient { name: "Mozzarella".to_string(), amount_grams: 80.0, calories: 200.0 },
        Ingredient { name: "Pepperoni".to_string(), amount_grams: 40.0, calories: 150.0 },
    ];
    
    let _ = blockchain.add_menu_item_with_details(
        "Classic Burger".to_string(),
        "Traditional beef burger with fresh ingredients".to_string(),
        12.99,
        15,
        8,
        10,
        burger_ingredients,
        main_owner.clone()
    );
    
    let _ = blockchain.add_menu_item_with_details(
        "Pepperoni Pizza".to_string(),
        "Classic pepperoni pizza with mozzarella cheese".to_string(),
        16.99,
        8,
        9,
        15,
        pizza_ingredients,
        main_owner.clone()
    );
    
    // Simulate some purchases to generate checks
    println!("Processing food purchases and generating checks...");
    let purchases = vec![
        ("Customer_John".to_string(), "Truck_Alice".to_string(), 12.50, vec!["Burger Combo".to_string()]),
        ("Customer_Sarah".to_string(), "Truck_Bob".to_string(), 8.75, vec!["Taco Plate".to_string()]),
        ("Customer_Mike".to_string(), "Truck_Charlie".to_string(), 15.00, vec!["Pizza Slice".to_string()]),
        ("Customer_Lisa".to_string(), "Truck_Alice".to_string(), 6.25, vec!["Hot Dog".to_string()]),
    ];
    
    for (customer, truck, amount, food_items) in purchases {
        let check = blockchain.process_purchase(customer, truck, amount, food_items);
        println!("Generated check: {} for {}", check.check_id, config_utils::format_gel(check.amount_subunits));
    }
    
    // Создаем пример заказа
    println!("Creating sample order...");
    let order_items = vec![
        OrderItem { menu_item_id: blockchain.menu_items[0].id.clone(), quantity: 2 },
        OrderItem { menu_item_id: blockchain.menu_items[1].id.clone(), quantity: 1 },
    ];
    
    match blockchain.create_order("Customer_John".to_string(), order_items, 30) {
        Ok(order) => println!("Created order: {} for {}", order.id, config_utils::format_gel(order.total_amount_subunits)),
        Err(e) => println!("Failed to create order: {}", e),
    }
    
    // Mine some blocks
    println!("\nMining blocks...");
    for i in 0..2 {
        match blockchain.mine_block() {
            Ok(()) => println!("Block {} mined successfully!", i + 1),
            Err(e) => println!("Failed to mine block {}: {}", i + 1, e),
        }
    }
    
    // Update roles
    blockchain.update_roles();

    // Optional: start API server only (no interactive UI) when API_ONLY=1
    if env::var("API_ONLY").map(|v| v == "1").unwrap_or(false) {
        println!("🌐 Starting API Server (API_ONLY mode) on port 3000...");
        let blockchain_arc = Arc::new(Mutex::new(blockchain.clone()));
        let api_server = ApiServer::new(blockchain_arc, 3000);
        api_server.start();
        return;
    }

    // Optional: start Franchise Network API when FRANCHISE_API=1
    if env::var("FRANCHISE_API").map(|v| v == "1").unwrap_or(false) {
        println!("🏪 Starting Franchise Network API on port 3001...");
        
        // Создаем франшизную сеть
        let franchise_network = Arc::new(Mutex::new(FranchiseNetwork::new("master_owner_georgia".to_string())));
        
        // Демонстрация работы сети
        demo_franchise_network(&franchise_network);
        
        // Запускаем POS API сервер
        let pos_api_server = PosApiServer::new(franchise_network, 3001);
        pos_api_server.start();
        return;
    }

    // Optional: start P2P Network when P2P_NETWORK=1
    if env::var("P2P_NETWORK").map(|v| v == "1").unwrap_or(false) {
        println!("🌐 Starting P2P Network...");
        
        // Создаем франшизную сеть
        let franchise_network = Arc::new(Mutex::new(FranchiseNetwork::new("master_owner_georgia".to_string())));
        
        // Демонстрация работы сети
        demo_franchise_network(&franchise_network);
        
        // Создаем P2P узел
        let node_id = env::var("NODE_ID").unwrap_or_else(|_| "1".to_string()).parse::<u64>().unwrap_or(1);
        let port = env::var("P2P_PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
        let address: std::net::SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        
        let p2p_node = P2PNode::new(node_id, address, franchise_network);
        
        println!("🚀 Starting P2P Node {} on {}", node_id, address);
        p2p_node.start();
        return;
    }

    // Optional: start Full Decentralized Network when FULL_DECENTRALIZED=1
    if env::var("FULL_DECENTRALIZED").map(|v| v == "1").unwrap_or(false) {
        println!("🌐 Starting Full Decentralized Network...");
        
        // Создаем франшизную сеть
        let franchise_network = Arc::new(Mutex::new(FranchiseNetwork::new("master_owner_georgia".to_string())));
        
        // Демонстрация работы сети
        demo_franchise_network(&franchise_network);
        
        // Создаем IPFS хранилище
        let mut ipfs_storage = IPFSStorage::new("https://ipfs.io/ipfs/".to_string());
        
        // Демонстрация IPFS
        demo_ipfs_storage(&mut ipfs_storage, &franchise_network);
        
        // Создаем P2P узел с IPFS
        let node_id = env::var("NODE_ID").unwrap_or_else(|_| "1".to_string()).parse::<u64>().unwrap_or(1);
        let port = env::var("P2P_PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
        let address = format!("127.0.0.1:{}", port).parse().unwrap();
        
        let p2p_node = P2PNode::new(node_id, address, franchise_network);
        
        println!("🚀 Starting Full Decentralized Node {} on {}", node_id, address);
        p2p_node.start();
        return;
    }
    
    // Start UI
    let mut ui = UI::new(blockchain);
    ui.run();
}

// Демонстрация работы франшизной сети
fn demo_franchise_network(franchise_network: &Arc<Mutex<FranchiseNetwork>>) {
    println!("\n🏪 === FRANCHISE NETWORK DEMO ===");
    
    let mut network = franchise_network.lock().unwrap();
    
    // 1. Регистрируем POS системы
    network.whitelist_pos("POS_Tbilisi_001".to_string());
    network.whitelist_pos("POS_Batumi_001".to_string());
    network.whitelist_pos("POS_Kutaisi_001".to_string());
    println!("✅ Whitelisted POS systems");
    
    // 2. Регистрируем ноды
    let node1 = network.register_node(
        "owner_tbilisi_central".to_string(),
        NodeType::OWNER,
        "Tbilisi".to_string()
    ).unwrap();
    println!("✅ Registered OWNER node {} in Tbilisi", node1);
    
    let node2 = network.register_node(
        "franchisee_batumi".to_string(),
        NodeType::FRANCHISE,
        "Batumi".to_string()
    ).unwrap();
    println!("✅ Registered FRANCHISE node {} in Batumi", node2);
    
    let node3 = network.register_node(
        "franchisee_kutaisi".to_string(),
        NodeType::FRANCHISE,
        "Kutaisi".to_string()
    ).unwrap();
    println!("✅ Registered FRANCHISE node {} in Kutaisi", node3);
    
    // 3. Демонстрируем продажи
    println!("\n💰 === SALES DEMONSTRATION ===");
    
    // Продажа в собственной точке (OWNER)
    let sale1 = network.record_sale(
        node1,
        "sale_tbilisi_001".to_string(),
        25.50,
        "Customer: John Doe, Phone: +995123456789".to_string(),
        "POS_Tbilisi_001".to_string(),
        vec![
            SaleItem { item_id: "khinkali_001".to_string(), quantity: 10, price: 15.0 },
            SaleItem { item_id: "khachapuri_001".to_string(), quantity: 2, price: 10.5 },
        ]
    ).unwrap();
    
    println!("🍽️  Sale in OWNER node {}: {} GEL", node1, 25.50);
    println!("   → Owner gets: {} subunits (0.51 tokens)", sale1.owner_units);
    println!("   → Buyer gets: {} subunits (0.49 tokens)", sale1.buyer_units);
    println!("   → Royalty: {} subunits", sale1.royalty_units);
    
    // Продажа во франшизной точке (FRANCHISE)
    let sale2 = network.record_sale(
        node2,
        "sale_batumi_001".to_string(),
        18.75,
        "Customer: Maria Garcia, Email: maria@example.com".to_string(),
        "POS_Batumi_001".to_string(),
        vec![
            SaleItem { item_id: "lobiani_001".to_string(), quantity: 3, price: 12.0 },
            SaleItem { item_id: "mtsvadi_001".to_string(), quantity: 1, price: 6.75 },
        ]
    ).unwrap();
    
    println!("\n🍽️  Sale in FRANCHISE node {}: {} GEL", node2, 18.75);
    println!("   → Franchisee gets: {} subunits (0.48 tokens)", sale2.owner_units);
    println!("   → Master owner gets: {} subunits (0.03 tokens royalty)", sale2.royalty_units);
    println!("   → Buyer gets: {} subunits (0.49 tokens)", sale2.buyer_units);
    
    // Еще одна продажа
    let sale3 = network.record_sale(
        node3,
        "sale_kutaisi_001".to_string(),
        32.00,
        "Customer: Ahmed Hassan, Table: 5".to_string(),
        "POS_Kutaisi_001".to_string(),
        vec![
            SaleItem { item_id: "chakapuli_001".to_string(), quantity: 1, price: 20.0 },
            SaleItem { item_id: "mchadi_001".to_string(), quantity: 4, price: 12.0 },
        ]
    ).unwrap();
    
    println!("\n🍽️  Sale in FRANCHISE node {}: {} GEL", node3, 32.00);
    println!("   → Franchisee gets: {} subunits (0.48 tokens)", sale3.owner_units);
    println!("   → Master owner gets: {} subunits (0.03 tokens royalty)", sale3.royalty_units);
    println!("   → Buyer gets: {} subunits (0.49 tokens)", sale3.buyer_units);
    
    // 4. Показываем статистику
    let stats = network.get_network_stats();
    println!("\n📊 === NETWORK STATISTICS ===");
    println!("   Total nodes: {}", stats.total_nodes);
    println!("   Active nodes: {}", stats.active_nodes);
    println!("   Total sales: {}", stats.total_sales);
    println!("   Total tokens minted: {} subunits ({} tokens)", 
             stats.total_tokens_minted, 
             stats.total_tokens_minted as f64 / 100.0);
    println!("   Master owner balance: {} subunits ({} tokens)", 
             stats.master_owner_balance,
             stats.master_owner_balance as f64 / 100.0);
    
    // 5. Показываем балансы кошельков
    println!("\n💳 === WALLET BALANCES ===");
    println!("   Master owner: {} subunits", network.get_wallet_balance("master_owner_georgia"));
    println!("   Tbilisi owner: {} subunits", network.get_wallet_balance("owner_tbilisi_central"));
    println!("   Batumi franchisee: {} subunits", network.get_wallet_balance("franchisee_batumi"));
    println!("   Kutaisi franchisee: {} subunits", network.get_wallet_balance("franchisee_kutaisi"));
    
    println!("\n🌐 Franchise Network API is ready on port 3001!");
    println!("   Try: curl -X POST http://localhost:3001/ -H 'Content-Type: application/json' -d '{{\"GetNetworkStats\": null}}'");
}

// Демонстрация IPFS хранилища
fn demo_ipfs_storage(ipfs_storage: &mut IPFSStorage, franchise_network: &Arc<Mutex<FranchiseNetwork>>) {
    println!("\n📦 === IPFS STORAGE DEMO ===");
    
    // 1. Создаем пример меню
    let menu_data = blockchain_project::ipfs_storage::MenuData {
        items: vec![
            blockchain_project::ipfs_storage::MenuItem {
                id: "khinkali_001".to_string(),
                name: "Хинкали".to_string(),
                description: "Традиционные грузинские хинкали с мясом".to_string(),
                price: 1.5,
                category: "Основные блюда".to_string(),
                ingredients: vec!["мука".to_string(), "говядина".to_string(), "лук".to_string()],
                image_hash: Some("QmKhinkaliImage".to_string()),
                nutritional_info: blockchain_project::ipfs_storage::NutritionalInfo {
                    calories: 250,
                    protein: 15.0,
                    carbs: 30.0,
                    fat: 8.0,
                    fiber: 2.0,
                },
            },
            blockchain_project::ipfs_storage::MenuItem {
                id: "khachapuri_001".to_string(),
                name: "Хачапури".to_string(),
                description: "Грузинский сырный хлеб".to_string(),
                price: 5.0,
                category: "Основные блюда".to_string(),
                ingredients: vec!["мука".to_string(), "сыр".to_string(), "яйцо".to_string()],
                image_hash: Some("QmKhachapuriImage".to_string()),
                nutritional_info: blockchain_project::ipfs_storage::NutritionalInfo {
                    calories: 400,
                    protein: 20.0,
                    carbs: 35.0,
                    fat: 18.0,
                    fiber: 3.0,
                },
            },
        ],
        categories: vec!["Основные блюда".to_string(), "Закуски".to_string(), "Напитки".to_string()],
        last_updated: chrono::Utc::now().timestamp() as u64,
        version: 1,
    };
    
    // Сохраняем меню в IPFS
    match ipfs_storage.store_menu(&menu_data) {
        Ok(hash) => println!("✅ Menu stored in IPFS: {}", hash),
        Err(e) => println!("❌ Failed to store menu: {}", e),
    }
    
    // 2. Создаем отчет о продажах
    let network = franchise_network.lock().unwrap();
    match ipfs_storage.create_sales_report(1, &network, 30) {
        Ok(report) => {
            println!("📊 Created sales report for node 1:");
            println!("   Total sales: {}", report.total_sales);
            println!("   Total revenue: {:.2} GEL", report.total_revenue);
            println!("   Top items: {}", report.top_items.len());
            
            // Сохраняем отчет в IPFS
            match ipfs_storage.store_sales_report(&report) {
                Ok(hash) => println!("✅ Sales report stored in IPFS: {}", hash),
                Err(e) => println!("❌ Failed to store sales report: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to create sales report: {}", e),
    }
    
    // 3. Создаем глобальный отчет сети
    match ipfs_storage.create_network_report(&network) {
        Ok(network_report) => {
            println!("🌐 Created network report:");
            println!("   Total nodes: {}", network_report.total_nodes);
            println!("   Active nodes: {}", network_report.active_nodes);
            println!("   Total sales: {}", network_report.total_sales);
            println!("   Total revenue: {:.2} GEL", network_report.total_revenue);
            println!("   Cities: {}", network_report.city_breakdown.len());
        }
        Err(e) => println!("❌ Failed to create network report: {}", e),
    }
    
    // 4. Синхронизация с IPFS сетью
    match ipfs_storage.sync_with_network() {
        Ok(new_hashes) => {
            println!("🔄 Synced with IPFS network, found {} new files", new_hashes.len());
        }
        Err(e) => println!("❌ Failed to sync with IPFS: {}", e),
    }
    
    // 5. Показываем статистику хранилища
    let stats = ipfs_storage.get_storage_stats();
    println!("\n📈 === IPFS STORAGE STATS ===");
    println!("   Total files: {}", stats.total_files);
    println!("   Total size: {} bytes", stats.total_size);
    println!("   Pinned hashes: {}", stats.pinned_hashes);
    println!("   Gateway URL: {}", stats.gateway_url);
    
    drop(network);
    
    // Запускаем веб-сервер для обслуживания HTML интерфейсов и API
    println!("\n🌐 === ЗАПУСК ВЕБ-СЕРВЕРА ===");
    let enhanced_web_server = EnhancedWebServer::new(8080);
    
    // Запускаем веб-сервер в отдельном потоке
    thread::spawn(move || {
        enhanced_web_server.start();
    });
    
    println!("✅ Enhanced веб-сервер запущен на http://127.0.0.1:8080");
    println!("📱 Доступные интерфейсы:");
    println!("   • Главная страница: http://127.0.0.1:8080/");
    println!("   • Владелец сети: http://127.0.0.1:8080/owner_dashboard.html");
    println!("   • Владелец франшизы: http://127.0.0.1:8080/franchise_dashboard.html");
    println!("   • Покупатель: http://127.0.0.1:8080/customer_wallet.html");
    println!("   • Старый интерфейс владельца: http://127.0.0.1:8080/restaurant_owner.html");
    println!("   • Старый интерфейс кошелька: http://127.0.0.1:8080/wallet_interface.html");
    println!("🎥 Система видеонаблюдения:");
    println!("   • Панель управления: http://127.0.0.1:8080/video_management_dashboard.html");
    println!("   • Интерфейс согласия: http://127.0.0.1:8080/video_consent_interface.html");
    println!("   • Тестирование API: http://127.0.0.1:8080/api_test_interface.html");
    println!("🔧 API эндпоинты:");
    println!("   • POST /api/video-consent - Запрос согласия");
    println!("   • POST /api/video-consent/confirm - Подтверждение согласия");
    println!("   • POST /api/video-recording/start - Начать запись");
    println!("   • POST /api/video-recording/stop - Остановить запись");
    println!("   • GET /api/video-recording/active - Активные записи");
    println!("   • GET /api/video-cameras/stats - Статистика камер");
    println!("   • POST /api/video-cameras - Добавить камеру");
    
    // Оставляем программу запущенной
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    // Тесты находятся в отдельных файлах в папке src/tests/
    // Они будут автоматически подхвачены cargo test
}
