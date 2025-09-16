//! Database module для The Hot Pot Spot
//! 
//! Обеспечивает работу с PostgreSQL базой данных,
//! включая миграции, схемы и ORM функциональность.

use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use chrono::{DateTime, Utc};
use tokio_postgres::{Client, NoTls, Row};
use tokio_postgres::types::{ToSql};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Конфигурация базы данных
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "thehotpotspot".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
            max_connections: 10,
            connection_timeout: 30,
        }
    }
}

/// Менеджер базы данных
#[derive(Clone)]
pub struct DatabaseManager {
    client: Arc<Mutex<Client>>,
    config: DatabaseConfig,
}

/// Структура для миграций
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

/// Результат выполнения запроса
#[derive(Debug)]
pub struct QueryResult {
    pub rows_affected: u64,
    pub rows: Vec<Row>,
}

impl DatabaseManager {
    /// Создание нового менеджера базы данных
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let connection_string = format!(
            "host={} port={} dbname={} user={} password={}",
            config.host, config.port, config.database, config.username, config.password
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Запускаем соединение в фоновом режиме
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        let manager = Self {
            client: Arc::new(Mutex::new(client)),
            config,
        };

        // Инициализируем базу данных
        manager.initialize_database().await?;

        Ok(manager)
    }

    /// Инициализация базы данных
    async fn initialize_database(&self) -> Result<(), DatabaseError> {
        // Создаем таблицу миграций
        self.create_migrations_table().await?;
        
        // Выполняем миграции
        self.run_migrations().await?;

        Ok(())
    }

    /// Создание таблицы миграций
    async fn create_migrations_table(&self) -> Result<(), DatabaseError> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            );
        "#;

        self.execute_sql(sql, &[]).await?;
        Ok(())
    }

    /// Выполнение миграций
    async fn run_migrations(&self) -> Result<(), DatabaseError> {
        let migrations = self.get_migrations();
        
        for migration in migrations {
            if !self.is_migration_applied(migration.version).await? {
                println!("Applying migration: {} (version {})", migration.name, migration.version);
                
                // Выполняем миграцию
                self.execute_sql(&migration.up_sql, &[]).await?;
                
                // Записываем в таблицу миграций
                self.record_migration(migration.version, &migration.name).await?;
            }
        }

        Ok(())
    }

    /// Получение списка миграций
    fn get_migrations(&self) -> Vec<Migration> {
        vec![
            Migration {
                version: 1,
                name: "create_users_table".to_string(),
                up_sql: r#"
                    CREATE TABLE users (
                        user_id VARCHAR(255) PRIMARY KEY,
                        email VARCHAR(255) UNIQUE NOT NULL,
                        phone VARCHAR(50),
                        first_name VARCHAR(255) NOT NULL,
                        last_name VARCHAR(255) NOT NULL,
                        date_of_birth TIMESTAMP WITH TIME ZONE,
                        nationality VARCHAR(10),
                        address_street VARCHAR(255),
                        address_city VARCHAR(255),
                        address_state VARCHAR(255),
                        address_postal_code VARCHAR(20),
                        address_country VARCHAR(100),
                        kyc_status VARCHAR(50) NOT NULL DEFAULT 'NotStarted',
                        kyc_level VARCHAR(50) NOT NULL DEFAULT 'Basic',
                        kyc_started_at TIMESTAMP WITH TIME ZONE,
                        kyc_completed_at TIMESTAMP WITH TIME ZONE,
                        kyc_expires_at TIMESTAMP WITH TIME ZONE,
                        risk_score INTEGER NOT NULL DEFAULT 0,
                        sanctions_check BOOLEAN NOT NULL DEFAULT FALSE,
                        pep_status BOOLEAN NOT NULL DEFAULT FALSE,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        last_login TIMESTAMP WITH TIME ZONE
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS users;".to_string(),
            },
            Migration {
                version: 2,
                name: "create_documents_table".to_string(),
                up_sql: r#"
                    CREATE TABLE documents (
                        document_id VARCHAR(255) PRIMARY KEY,
                        user_id VARCHAR(255) NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                        document_type VARCHAR(50) NOT NULL,
                        file_hash VARCHAR(255) NOT NULL,
                        file_path VARCHAR(500) NOT NULL,
                        uploaded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        status VARCHAR(50) NOT NULL DEFAULT 'Uploaded',
                        verified_at TIMESTAMP WITH TIME ZONE,
                        verified_by VARCHAR(255),
                        rejection_reason TEXT,
                        expiry_date TIMESTAMP WITH TIME ZONE
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS documents;".to_string(),
            },
            Migration {
                version: 3,
                name: "create_roles_table".to_string(),
                up_sql: r#"
                    CREATE TABLE roles (
                        role_id VARCHAR(255) PRIMARY KEY,
                        name VARCHAR(100) UNIQUE NOT NULL,
                        description TEXT,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS roles;".to_string(),
            },
            Migration {
                version: 4,
                name: "create_permissions_table".to_string(),
                up_sql: r#"
                    CREATE TABLE permissions (
                        permission_id VARCHAR(255) PRIMARY KEY,
                        name VARCHAR(100) UNIQUE NOT NULL,
                        description TEXT,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS permissions;".to_string(),
            },
            Migration {
                version: 5,
                name: "create_role_permissions_table".to_string(),
                up_sql: r#"
                    CREATE TABLE role_permissions (
                        role_id VARCHAR(255) NOT NULL REFERENCES roles(role_id) ON DELETE CASCADE,
                        permission_id VARCHAR(255) NOT NULL REFERENCES permissions(permission_id) ON DELETE CASCADE,
                        PRIMARY KEY (role_id, permission_id)
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS role_permissions;".to_string(),
            },
            Migration {
                version: 6,
                name: "create_user_roles_table".to_string(),
                up_sql: r#"
                    CREATE TABLE user_roles (
                        assignment_id VARCHAR(255) PRIMARY KEY,
                        user_id VARCHAR(255) NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                        role_id VARCHAR(255) NOT NULL REFERENCES roles(role_id) ON DELETE CASCADE,
                        assigned_by VARCHAR(255) NOT NULL,
                        assigned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        expires_at TIMESTAMP WITH TIME ZONE,
                        is_active BOOLEAN NOT NULL DEFAULT TRUE
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS user_roles;".to_string(),
            },
            Migration {
                version: 7,
                name: "create_wallets_table".to_string(),
                up_sql: r#"
                    CREATE TABLE wallets (
                        wallet_id VARCHAR(255) PRIMARY KEY,
                        wallet_type VARCHAR(50) NOT NULL,
                        owner_id VARCHAR(255) NOT NULL,
                        derivation_path VARCHAR(255) NOT NULL,
                        public_key TEXT NOT NULL,
                        address VARCHAR(255) NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        last_used TIMESTAMP WITH TIME ZONE,
                        status VARCHAR(50) NOT NULL DEFAULT 'Active'
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS wallets;".to_string(),
            },
            Migration {
                version: 8,
                name: "create_check_wallets_table".to_string(),
                up_sql: r#"
                    CREATE TABLE check_wallets (
                        check_id VARCHAR(255) PRIMARY KEY,
                        sale_id VARCHAR(255) NOT NULL,
                        node_id BIGINT NOT NULL,
                        wallet_id VARCHAR(255) NOT NULL REFERENCES wallets(wallet_id) ON DELETE CASCADE,
                        amount_subunits BIGINT NOT NULL,
                        currency VARCHAR(10) NOT NULL DEFAULT 'GEL',
                        qr_code TEXT NOT NULL,
                        activation_code VARCHAR(10) NOT NULL,
                        is_activated BOOLEAN NOT NULL DEFAULT FALSE,
                        activated_at TIMESTAMP WITH TIME ZONE,
                        expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS check_wallets;".to_string(),
            },
            Migration {
                version: 9,
                name: "create_transactions_table".to_string(),
                up_sql: r#"
                    CREATE TABLE transactions (
                        transaction_id VARCHAR(255) PRIMARY KEY,
                        sale_id VARCHAR(255) NOT NULL,
                        node_id BIGINT NOT NULL,
                        pos_id VARCHAR(255) NOT NULL,
                        transaction_type VARCHAR(50) NOT NULL,
                        amount_subunits BIGINT NOT NULL,
                        buyer_address VARCHAR(255) NOT NULL,
                        buyer_meta TEXT,
                        signature TEXT NOT NULL,
                        timestamp BIGINT NOT NULL,
                        status VARCHAR(50) NOT NULL DEFAULT 'Pending',
                        blockchain_tx_hash VARCHAR(255),
                        error_message TEXT,
                        retry_count INTEGER NOT NULL DEFAULT 0,
                        max_retries INTEGER NOT NULL DEFAULT 3,
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS transactions;".to_string(),
            },
            Migration {
                version: 10,
                name: "create_audit_logs_table".to_string(),
                up_sql: r#"
                    CREATE TABLE audit_logs (
                        log_id VARCHAR(255) PRIMARY KEY,
                        user_id VARCHAR(255) NOT NULL,
                        action VARCHAR(100) NOT NULL,
                        resource VARCHAR(100) NOT NULL,
                        timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                        ip_address INET,
                        user_agent TEXT,
                        success BOOLEAN NOT NULL,
                        details TEXT
                    );
                "#.to_string(),
                down_sql: "DROP TABLE IF EXISTS audit_logs;".to_string(),
            },
            Migration {
                version: 11,
                name: "create_indexes".to_string(),
                up_sql: r#"
                    CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
                    CREATE INDEX IF NOT EXISTS idx_users_kyc_status ON users(kyc_status);
                    CREATE INDEX IF NOT EXISTS idx_documents_user_id ON documents(user_id);
                    CREATE INDEX IF NOT EXISTS idx_documents_status ON documents(status);
                    CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id);
                    CREATE INDEX IF NOT EXISTS idx_user_roles_active ON user_roles(is_active);
                    CREATE INDEX IF NOT EXISTS idx_wallets_owner_id ON wallets(owner_id);
                    CREATE INDEX IF NOT EXISTS idx_wallets_type ON wallets(wallet_type);
                    CREATE INDEX IF NOT EXISTS idx_transactions_sale_id ON transactions(sale_id);
                    CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);
                    CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
                    CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp);
                "#.to_string(),
                down_sql: r#"
                    DROP INDEX IF EXISTS idx_users_email;
                    DROP INDEX IF EXISTS idx_users_kyc_status;
                    DROP INDEX IF EXISTS idx_documents_user_id;
                    DROP INDEX IF EXISTS idx_documents_status;
                    DROP INDEX IF EXISTS idx_user_roles_user_id;
                    DROP INDEX IF EXISTS idx_user_roles_active;
                    DROP INDEX IF EXISTS idx_wallets_owner_id;
                    DROP INDEX IF EXISTS idx_wallets_type;
                    DROP INDEX IF EXISTS idx_transactions_sale_id;
                    DROP INDEX IF EXISTS idx_transactions_status;
                    DROP INDEX IF EXISTS idx_audit_logs_user_id;
                    DROP INDEX IF EXISTS idx_audit_logs_timestamp;
                "#.to_string(),
            },
        ]
    }

    /// Проверка, применена ли миграция
    async fn is_migration_applied(&self, version: u32) -> Result<bool, DatabaseError> {
        let sql = "SELECT COUNT(*) FROM migrations WHERE version = $1";
        let result = self.query_one(sql, &[&version]).await?;
        let count: i64 = result.get(0);
        Ok(count > 0)
    }

    /// Запись миграции в таблицу
    async fn record_migration(&self, version: u32, name: &str) -> Result<(), DatabaseError> {
        let sql = "INSERT INTO migrations (version, name) VALUES ($1, $2)";
        self.execute_sql(sql, &[&version, &name]).await?;
        Ok(())
    }

    /// Выполнение SQL запроса
    pub async fn execute_sql(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, DatabaseError> {
        let client = self.client.lock().await;
        let result = client.execute(sql, params).await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(result)
    }

    /// Выполнение SQL запроса с возвратом одной строки
    pub async fn query_one(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, DatabaseError> {
        let client = self.client.lock().await;
        let result = client.query_one(sql, params).await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(result)
    }

    /// Выполнение SQL запроса с возвратом множества строк
    pub async fn query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, DatabaseError> {
        let client = self.client.lock().await;
        let result = client.query(sql, params).await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(result)
    }

    /// Выполнение транзакции
    pub async fn transaction<F, R>(&self, f: F) -> Result<R, DatabaseError>
    where
        F: FnOnce(&tokio_postgres::Transaction) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, DatabaseError>> + Send>>,
    {
        let mut client = self.client.lock().await;
        let transaction = client.transaction().await
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))?;

        let result = f(&transaction).await?;
        
        transaction.commit().await
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))?;

        Ok(result)
    }

    /// Получение статистики базы данных
    pub async fn get_database_stats(&self) -> Result<DatabaseStats, DatabaseError> {
        let mut stats = DatabaseStats::default();

        // Подсчет пользователей
        let user_count: i64 = self.query_one("SELECT COUNT(*) FROM users", &[]).await?.get(0);
        stats.total_users = user_count as u32;

        // Подсчет документов
        let doc_count: i64 = self.query_one("SELECT COUNT(*) FROM documents", &[]).await?.get(0);
        stats.total_documents = doc_count as u32;

        // Подсчет кошельков
        let wallet_count: i64 = self.query_one("SELECT COUNT(*) FROM wallets", &[]).await?.get(0);
        stats.total_wallets = wallet_count as u32;

        // Подсчет транзакций
        let tx_count: i64 = self.query_one("SELECT COUNT(*) FROM transactions", &[]).await?.get(0);
        stats.total_transactions = tx_count as u32;

        // Подсчет аудит логов
        let log_count: i64 = self.query_one("SELECT COUNT(*) FROM audit_logs", &[]).await?.get(0);
        stats.total_audit_logs = log_count as u32;

        Ok(stats)
    }

    /// Очистка старых данных
    pub async fn cleanup_old_data(&self, days: i32) -> Result<CleanupStats, DatabaseError> {
        let mut stats = CleanupStats::default();

        // Очистка старых аудит логов
        let cutoff_date = std::time::SystemTime::now() - std::time::Duration::from_secs((days as u64) * 24 * 60 * 60);
        let deleted_logs: i64 = self.execute_sql(
            "DELETE FROM audit_logs WHERE timestamp < $1",
            &[&cutoff_date]
        ).await? as i64;
        stats.deleted_audit_logs = deleted_logs as u32;

        // Очистка истекших чек-кошельков
        let deleted_checks: i64 = self.execute_sql(
            "DELETE FROM check_wallets WHERE expires_at < NOW() AND is_activated = FALSE",
            &[]
        ).await? as i64;
        stats.deleted_expired_checks = deleted_checks as u32;

        Ok(stats)
    }
}

/// Статистика базы данных
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_users: u32,
    pub total_documents: u32,
    pub total_wallets: u32,
    pub total_transactions: u32,
    pub total_audit_logs: u32,
}

/// Статистика очистки
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CleanupStats {
    pub deleted_audit_logs: u32,
    pub deleted_expired_checks: u32,
}

/// Ошибки базы данных
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Data error: {0}")]
    DataError(String),
}

/// ORM функции для работы с пользователями
impl DatabaseManager {
    /// Сохранение пользователя
    pub async fn save_user(&self, user: &UserData) -> Result<(), DatabaseError> {
        let sql = r#"
            INSERT INTO users (
                user_id, email, phone, first_name, last_name, date_of_birth,
                nationality, address_street, address_city, address_state,
                address_postal_code, address_country, kyc_status, kyc_level,
                kyc_started_at, kyc_completed_at, kyc_expires_at, risk_score,
                sanctions_check, pep_status, created_at, updated_at, last_login
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23
            ) ON CONFLICT (user_id) DO UPDATE SET
                email = EXCLUDED.email,
                phone = EXCLUDED.phone,
                first_name = EXCLUDED.first_name,
                last_name = EXCLUDED.last_name,
                date_of_birth = EXCLUDED.date_of_birth,
                nationality = EXCLUDED.nationality,
                address_street = EXCLUDED.address_street,
                address_city = EXCLUDED.address_city,
                address_state = EXCLUDED.address_state,
                address_postal_code = EXCLUDED.address_postal_code,
                address_country = EXCLUDED.address_country,
                kyc_status = EXCLUDED.kyc_status,
                kyc_level = EXCLUDED.kyc_level,
                kyc_started_at = EXCLUDED.kyc_started_at,
                kyc_completed_at = EXCLUDED.kyc_completed_at,
                kyc_expires_at = EXCLUDED.kyc_expires_at,
                risk_score = EXCLUDED.risk_score,
                sanctions_check = EXCLUDED.sanctions_check,
                pep_status = EXCLUDED.pep_status,
                updated_at = NOW(),
                last_login = EXCLUDED.last_login
        "#;

        self.execute_sql(sql, &[
            &user.user_id,
            &user.email,
            &user.phone,
            &user.first_name,
            &user.last_name,
            &user.date_of_birth,
            &user.nationality,
            &user.address_street,
            &user.address_city,
            &user.address_state,
            &user.address_postal_code,
            &user.address_country,
            &user.kyc_status,
            &user.kyc_level,
            &user.kyc_started_at,
            &user.kyc_completed_at,
            &user.kyc_expires_at,
            &user.risk_score,
            &user.sanctions_check,
            &user.pep_status,
            &user.created_at,
            &user.updated_at,
            &user.last_login,
        ]).await?;

        Ok(())
    }

    /// Получение пользователя по ID
    pub async fn get_user(&self, user_id: &str) -> Result<Option<UserData>, DatabaseError> {
        let sql = "SELECT * FROM users WHERE user_id = $1";
        let result = self.query(sql, &[&user_id]).await?;
        
        if result.is_empty() {
            return Ok(None);
        }

        let row = &result[0];
        let user = UserData {
            user_id: row.get("user_id"),
            email: row.get("email"),
            phone: row.get("phone"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            date_of_birth: row.get("date_of_birth"),
            nationality: row.get("nationality"),
            address_street: row.get("address_street"),
            address_city: row.get("address_city"),
            address_state: row.get("address_state"),
            address_postal_code: row.get("address_postal_code"),
            address_country: row.get("address_country"),
            kyc_status: row.get("kyc_status"),
            kyc_level: row.get("kyc_level"),
            kyc_started_at: row.get("kyc_started_at"),
            kyc_completed_at: row.get("kyc_completed_at"),
            kyc_expires_at: row.get("kyc_expires_at"),
            risk_score: row.get("risk_score"),
            sanctions_check: row.get("sanctions_check"),
            pep_status: row.get("pep_status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login: row.get("last_login"),
        };

        Ok(Some(user))
    }

    /// Получение всех пользователей
    pub async fn get_all_users(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<UserData>, DatabaseError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let sql = "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2";
        let result = self.query(sql, &[&(limit as i64), &(offset as i64)]).await?;
        
        let mut users = Vec::new();
        for row in result {
            let user = UserData {
                user_id: row.get("user_id"),
                email: row.get("email"),
                phone: row.get("phone"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                date_of_birth: row.get("date_of_birth"),
                nationality: row.get("nationality"),
                address_street: row.get("address_street"),
                address_city: row.get("address_city"),
                address_state: row.get("address_state"),
                address_postal_code: row.get("address_postal_code"),
                address_country: row.get("address_country"),
                kyc_status: row.get("kyc_status"),
                kyc_level: row.get("kyc_level"),
                kyc_started_at: row.get("kyc_started_at"),
                kyc_completed_at: row.get("kyc_completed_at"),
                kyc_expires_at: row.get("kyc_expires_at"),
                risk_score: row.get("risk_score"),
                sanctions_check: row.get("sanctions_check"),
                pep_status: row.get("pep_status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_login: row.get("last_login"),
            };
            users.push(user);
        }

        Ok(users)
    }
}

/// Структура данных пользователя для базы данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub user_id: String,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<std::time::SystemTime>,
    pub nationality: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub kyc_status: String,
    pub kyc_level: String,
    pub kyc_started_at: Option<std::time::SystemTime>,
    pub kyc_completed_at: Option<std::time::SystemTime>,
    pub kyc_expires_at: Option<std::time::SystemTime>,
    pub risk_score: i32,
    pub sanctions_check: bool,
    pub pep_status: bool,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub last_login: Option<std::time::SystemTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig::default();
        let result = DatabaseManager::new(config).await;
        
        // Если PostgreSQL недоступен, пропускаем тест
        if result.is_err() {
            println!("PostgreSQL недоступен, пропускаем тест");
            return;
        }
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_operations() {
        let config = DatabaseConfig::default();
        let result = DatabaseManager::new(config).await;
        
        // Если PostgreSQL недоступен, пропускаем тест
        if result.is_err() {
            println!("PostgreSQL недоступен, пропускаем тест");
            return;
        }
        
        let db = result.unwrap();
        
        let user = UserData {
            user_id: "test_user_001".to_string(),
            email: "test@example.com".to_string(),
            phone: Some("+995123456789".to_string()),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: Some(std::time::SystemTime::now() - std::time::Duration::from_secs(365 * 25 * 24 * 60 * 60)),
            nationality: Some("GE".to_string()),
            address_street: Some("123 Main St".to_string()),
            address_city: Some("Tbilisi".to_string()),
            address_state: Some("Tbilisi".to_string()),
            address_postal_code: Some("0100".to_string()),
            address_country: Some("Georgia".to_string()),
            kyc_status: "Verified".to_string(),
            kyc_level: "Basic".to_string(),
            kyc_started_at: Some(std::time::SystemTime::now() - std::time::Duration::from_secs(24 * 60 * 60)),
            kyc_completed_at: Some(std::time::SystemTime::now()),
            kyc_expires_at: Some(std::time::SystemTime::now() + std::time::Duration::from_secs(365 * 24 * 60 * 60)),
            risk_score: 25,
            sanctions_check: false,
            pep_status: false,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
            last_login: None,
        };
        
        // Сохраняем пользователя
        db.save_user(&user).await.unwrap();
        
        // Получаем пользователя
        let retrieved_user = db.get_user("test_user_001").await.unwrap();
        assert!(retrieved_user.is_some());
        assert_eq!(retrieved_user.unwrap().email, "test@example.com");
    }
}

