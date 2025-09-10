//! HD Wallet Implementation для The Hot Pot Spot
//! 
//! Обеспечивает безопасную генерацию и управление кошельками с использованием
//! иерархической детерминированной (HD) деривации адресов.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Utc};

/// Типы кошельков
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    Master,         // Главный кошелек
    Franchise,      // Кошелек франшизы
    Customer,       // Кошелек покупателя
    Check,          // Чек-кошелек
    Charity,        // Благотворительный кошелек
}

/// Статус кошелька
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletStatus {
    Active,         // Активный
    Inactive,       // Неактивный
    Suspended,      // Приостановлен
    Archived,       // Архивирован
}

/// Структура HD кошелька
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDWallet {
    pub wallet_id: String,
    pub wallet_type: WalletType,
    pub owner_id: String,           // ID владельца (node_id, customer_id, etc.)
    pub derivation_path: String,    // Путь деривации (m/44'/60'/0'/0/0)
    pub public_key: String,         // Публичный ключ
    pub address: String,            // Адрес кошелька
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub status: WalletStatus,
    pub metadata: HashMap<String, String>, // Дополнительные метаданные
}

/// Структура чек-кошелька
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckWallet {
    pub check_id: String,
    pub sale_id: String,
    pub node_id: u64,
    pub wallet: HDWallet,
    pub amount_subunits: u128,      // Сумма в subunits
    pub currency: String,           // Валюта (GEL)
    pub items: Vec<CheckItem>,      // Товары в чеке
    pub qr_code: String,            // QR код для активации
    pub activation_code: String,    // Код активации
    pub is_activated: bool,
    pub activated_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckItem {
    pub item_id: String,
    pub name: String,
    pub quantity: u32,
    pub price_subunits: u128,
}

/// Менеджер HD кошельков
pub struct HDWalletManager {
    master_seed: String,                    // Главный seed (в реальной реализации должен быть зашифрован)
    wallets: HashMap<String, HDWallet>,     // Кэш кошельков
    check_wallets: HashMap<String, CheckWallet>, // Кэш чек-кошельков
    derivation_index: HashMap<String, u32>, // Индекс деривации для каждого типа
}

impl HDWalletManager {
    /// Создание нового менеджера HD кошельков
    pub fn new(master_seed: String) -> Self {
        Self {
            master_seed,
            wallets: HashMap::new(),
            check_wallets: HashMap::new(),
            derivation_index: HashMap::new(),
        }
    }

    /// Генерация кошелька для ноды
    pub fn generate_node_wallet(&mut self, node_id: u64, node_type: WalletType) -> Result<HDWallet, HDWalletError> {
        let wallet_id = format!("node_{:?}_{}", node_type, node_id);
        let derivation_path = self.generate_derivation_path(&wallet_id, &node_type)?;
        
        let wallet = self.create_wallet(
            wallet_id.clone(),
            node_type,
            node_id.to_string(),
            derivation_path,
        )?;

        self.wallets.insert(wallet_id, wallet.clone());
        Ok(wallet)
    }

    /// Генерация кошелька для покупателя
    pub fn generate_customer_wallet(&mut self, customer_id: String) -> Result<HDWallet, HDWalletError> {
        let wallet_id = format!("customer_{}", customer_id);
        let derivation_path = self.generate_derivation_path(&wallet_id, &WalletType::Customer)?;
        
        let wallet = self.create_wallet(
            wallet_id.clone(),
            WalletType::Customer,
            customer_id,
            derivation_path,
        )?;

        self.wallets.insert(wallet_id, wallet.clone());
        Ok(wallet)
    }

    /// Генерация чек-кошелька
    pub fn generate_check_wallet(&mut self, sale_id: String, node_id: u64, amount_subunits: u128, items: Vec<CheckItem>) -> Result<CheckWallet, HDWalletError> {
        let check_id = format!("check_{}", sale_id);
        let wallet_id = format!("check_{}", sale_id);
        let derivation_path = self.generate_derivation_path(&wallet_id, &WalletType::Check)?;
        
        let wallet = self.create_wallet(
            wallet_id.clone(),
            WalletType::Check,
            sale_id.clone(),
            derivation_path,
        )?;

        let activation_code = self.generate_activation_code(&sale_id);
        let qr_code = self.generate_qr_code(&check_id, &wallet.address, &activation_code);
        
        let check_wallet = CheckWallet {
            check_id: check_id.clone(),
            sale_id,
            node_id,
            wallet,
            amount_subunits,
            currency: "GEL".to_string(),
            items,
            qr_code,
            activation_code,
            is_activated: false,
            activated_at: None,
            expires_at: Utc::now() + chrono::Duration::days(30), // Чек действителен 30 дней
        };

        self.check_wallets.insert(check_id, check_wallet.clone());
        Ok(check_wallet)
    }

    /// Создание кошелька
    fn create_wallet(&self, wallet_id: String, wallet_type: WalletType, owner_id: String, derivation_path: String) -> Result<HDWallet, HDWalletError> {
        let (public_key, address) = self.derive_keypair(&derivation_path)?;
        
        Ok(HDWallet {
            wallet_id,
            wallet_type,
            owner_id,
            derivation_path,
            public_key,
            address,
            created_at: Utc::now(),
            last_used: None,
            status: WalletStatus::Active,
            metadata: HashMap::new(),
        })
    }

    /// Генерация пути деривации
    fn generate_derivation_path(&mut self, wallet_id: &str, wallet_type: &WalletType) -> Result<String, HDWalletError> {
        let base_path = match wallet_type {
            WalletType::Master => "m/44'/60'/0'/0",
            WalletType::Franchise => "m/44'/60'/1'/0",
            WalletType::Customer => "m/44'/60'/2'/0",
            WalletType::Check => "m/44'/60'/3'/0",
            WalletType::Charity => "m/44'/60'/4'/0",
        };

        let index = self.derivation_index.entry(wallet_id.to_string()).or_insert(0);
        *index += 1;

        Ok(format!("{}/{}", base_path, index))
    }

    /// Деривация ключевой пары из пути
    fn derive_keypair(&self, derivation_path: &str) -> Result<(String, String), HDWalletError> {
        // Упрощенная реализация деривации ключей
        // В реальной реализации здесь должна быть полная BIP32/BIP44 деривация
        
        let mut hasher = Sha256::new();
        hasher.update(self.master_seed.as_bytes());
        hasher.update(derivation_path.as_bytes());
        let hash = hasher.finalize();
        
        let public_key = hex::encode(&hash[..32]);
        let address = format!("0x{}", hex::encode(&hash[..20]));
        
        Ok((public_key, address))
    }

    /// Генерация кода активации
    fn generate_activation_code(&self, sale_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(sale_id.as_bytes());
        hasher.update(Utc::now().timestamp().to_be_bytes());
        let hash = hasher.finalize();
        
        // Генерируем 6-значный код
        let code = (hash[0] as u32 % 1000000).to_string();
        format!("{:0>6}", code)
    }

    /// Генерация QR кода
    fn generate_qr_code(&self, check_id: &str, address: &str, activation_code: &str) -> String {
        let qr_data = serde_json::json!({
            "check_id": check_id,
            "address": address,
            "activation_code": activation_code,
            "timestamp": Utc::now().timestamp()
        });
        
        // В реальной реализации здесь должен быть настоящий QR код
        format!("QR:{}", qr_data.to_string())
    }

    /// Получение кошелька по ID
    pub fn get_wallet(&self, wallet_id: &str) -> Option<&HDWallet> {
        self.wallets.get(wallet_id)
    }

    /// Получение чек-кошелька по ID
    pub fn get_check_wallet(&self, check_id: &str) -> Option<&CheckWallet> {
        self.check_wallets.get(check_id)
    }

    /// Активация чек-кошелька
    pub fn activate_check_wallet(&mut self, check_id: &str, activation_code: &str) -> Result<CheckWallet, HDWalletError> {
        if let Some(check_wallet) = self.check_wallets.get_mut(check_id) {
            if check_wallet.activation_code == activation_code {
                if check_wallet.is_activated {
                    return Err(HDWalletError::AlreadyActivated);
                }
                
                if Utc::now() > check_wallet.expires_at {
                    return Err(HDWalletError::Expired);
                }
                
                check_wallet.is_activated = true;
                check_wallet.activated_at = Some(Utc::now());
                
                Ok(check_wallet.clone())
            } else {
                Err(HDWalletError::InvalidActivationCode)
            }
        } else {
            Err(HDWalletError::CheckWalletNotFound)
        }
    }

    /// Получение всех кошельков по типу
    pub fn get_wallets_by_type(&self, wallet_type: &WalletType) -> Vec<&HDWallet> {
        self.wallets.values()
            .filter(|wallet| wallet.wallet_type == *wallet_type)
            .collect()
    }

    /// Получение статистики кошельков
    pub fn get_wallet_statistics(&self) -> WalletStatistics {
        let mut stats = WalletStatistics::default();
        
        for wallet in self.wallets.values() {
            match wallet.wallet_type {
                WalletType::Master => stats.master_wallets += 1,
                WalletType::Franchise => stats.franchise_wallets += 1,
                WalletType::Customer => stats.customer_wallets += 1,
                WalletType::Check => stats.check_wallets += 1,
                WalletType::Charity => stats.charity_wallets += 1,
            }
            
            match wallet.status {
                WalletStatus::Active => stats.active_wallets += 1,
                WalletStatus::Inactive => stats.inactive_wallets += 1,
                WalletStatus::Suspended => stats.suspended_wallets += 1,
                WalletStatus::Archived => stats.archived_wallets += 1,
            }
        }
        
        stats.total_wallets = self.wallets.len() as u32;
        stats.total_check_wallets = self.check_wallets.len() as u32;
        
        stats
    }

    /// Обновление статуса кошелька
    pub fn update_wallet_status(&mut self, wallet_id: &str, status: WalletStatus) -> Result<(), HDWalletError> {
        if let Some(wallet) = self.wallets.get_mut(wallet_id) {
            wallet.status = status;
            Ok(())
        } else {
            Err(HDWalletError::WalletNotFound("".to_string()))
        }
    }

    /// Архивирование старых чек-кошельков
    pub fn archive_expired_check_wallets(&mut self) -> u32 {
        let now = Utc::now();
        let mut archived_count = 0;
        
        for check_wallet in self.check_wallets.values_mut() {
            if now > check_wallet.expires_at && !check_wallet.is_activated {
                if let Some(wallet) = self.wallets.get_mut(&check_wallet.wallet.wallet_id) {
                    wallet.status = WalletStatus::Archived;
                    archived_count += 1;
                }
            }
        }
        
        archived_count
    }
}

/// Статистика кошельков
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WalletStatistics {
    pub total_wallets: u32,
    pub total_check_wallets: u32,
    pub master_wallets: u32,
    pub franchise_wallets: u32,
    pub customer_wallets: u32,
    pub check_wallets: u32,
    pub charity_wallets: u32,
    pub active_wallets: u32,
    pub inactive_wallets: u32,
    pub suspended_wallets: u32,
    pub archived_wallets: u32,
}

/// Ошибки HD Wallet
#[derive(Debug, thiserror::Error)]
pub enum HDWalletError {
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
    
    #[error("Check wallet not found")]
    CheckWalletNotFound,
    
    #[error("Invalid activation code")]
    InvalidActivationCode,
    
    #[error("Check wallet already activated")]
    AlreadyActivated,
    
    #[error("Check wallet expired")]
    Expired,
    
    #[error("Derivation error: {0}")]
    DerivationError(String),
    
    #[error("Invalid wallet type")]
    InvalidWalletType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_wallet_generation() {
        let mut manager = HDWalletManager::new("test_seed".to_string());
        
        let wallet = manager.generate_node_wallet(1, WalletType::Franchise).unwrap();
        
        assert_eq!(wallet.wallet_type, WalletType::Franchise);
        assert_eq!(wallet.owner_id, "1");
        assert!(wallet.address.starts_with("0x"));
        assert_eq!(wallet.status, WalletStatus::Active);
    }

    #[test]
    fn test_check_wallet_generation() {
        let mut manager = HDWalletManager::new("test_seed".to_string());
        
        let items = vec![CheckItem {
            item_id: "ITEM_001".to_string(),
            name: "Test Item".to_string(),
            quantity: 1,
            price_subunits: 500,
        }];
        
        let check_wallet = manager.generate_check_wallet("SALE_001".to_string(), 1, 500, items).unwrap();
        
        assert_eq!(check_wallet.sale_id, "SALE_001");
        assert_eq!(check_wallet.amount_subunits, 500);
        assert!(!check_wallet.is_activated);
        assert!(check_wallet.activation_code.len() == 6);
    }

    #[test]
    fn test_check_wallet_activation() {
        let mut manager = HDWalletManager::new("test_seed".to_string());
        
        let items = vec![CheckItem {
            item_id: "ITEM_001".to_string(),
            name: "Test Item".to_string(),
            quantity: 1,
            price_subunits: 500,
        }];
        
        let check_wallet = manager.generate_check_wallet("SALE_001".to_string(), 1, 500, items).unwrap();
        let activation_code = check_wallet.activation_code.clone();
        
        let activated_wallet = manager.activate_check_wallet(&check_wallet.check_id, &activation_code).unwrap();
        
        assert!(activated_wallet.is_activated);
        assert!(activated_wallet.activated_at.is_some());
    }
}

