//! Anonymous Wallets Module for The Hot Pot Spot
//! 
//! This module handles anonymous wallet creation and management for the customer journey.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use hex;

/// Anonymous wallet for unclaimed tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousWallet {
    /// Unique wallet ID
    pub wallet_id: String,
    /// Wallet address (blockchain address)
    pub address: String,
    /// Associated check ID
    pub check_id: String,
    /// Associated sale ID
    pub sale_id: String,
    /// Node ID where the sale occurred
    pub node_id: String,
    /// Amount in GEL
    pub amount_gel: f64,
    /// ST tokens in this wallet
    pub st_tokens: u128,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Wallet status
    pub status: WalletStatus,
    /// Transfer timestamp (if transferred)
    pub transferred_at: Option<DateTime<Utc>>,
    /// User ID who claimed the wallet (if claimed)
    pub claimed_by: Option<String>,
    /// User's personal wallet address (if claimed)
    pub user_wallet_address: Option<String>,
}

/// Wallet status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletStatus {
    /// Wallet created but tokens not yet minted
    Created,
    /// Tokens minted and ready for claiming
    Active,
    /// Tokens transferred to user wallet
    Transferred,
    /// Wallet expired (not claimed within time limit)
    Expired,
    /// Wallet discarded by customer
    Discarded,
}

/// Anonymous wallet manager
#[derive(Debug, Clone)]
pub struct AnonymousWalletManager {
    /// Anonymous wallets storage
    pub wallets: HashMap<String, AnonymousWallet>,
    /// Configuration
    pub config: AnonymousWalletConfig,
}

/// Anonymous wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousWalletConfig {
    /// Wallet expiration time in hours
    pub expiration_hours: u32,
    /// Enable automatic cleanup of expired wallets
    pub auto_cleanup: bool,
    /// Maximum number of wallets to keep
    pub max_wallets: usize,
    /// Blockchain network ID
    pub network_id: u32,
}

impl Default for AnonymousWalletConfig {
    fn default() -> Self {
        Self {
            expiration_hours: 24,
            auto_cleanup: true,
            max_wallets: 10000,
            network_id: 1, // Mainnet
        }
    }
}

impl AnonymousWalletManager {
    /// Create new anonymous wallet manager
    pub fn new(config: AnonymousWalletConfig) -> Self {
        Self {
            wallets: HashMap::new(),
            config,
        }
    }

    /// Create a new anonymous wallet for a check
    pub fn create_wallet(&mut self, check_id: String, sale_id: String, node_id: String, amount_gel: f64, st_tokens: u128) -> Result<AnonymousWallet, String> {
        // Generate unique wallet ID
        let wallet_id = format!("anon_{}", Uuid::new_v4().to_string()[..8].to_uppercase());
        
        // Generate deterministic wallet address
        let address = self.generate_wallet_address(&wallet_id, &check_id)?;
        
        // Create anonymous wallet
        let wallet = AnonymousWallet {
            wallet_id: wallet_id.clone(),
            address,
            check_id,
            sale_id,
            node_id,
            amount_gel,
            st_tokens,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            status: WalletStatus::Created,
            transferred_at: None,
            claimed_by: None,
            user_wallet_address: None,
        };
        
        // Store wallet
        self.wallets.insert(wallet_id.clone(), wallet.clone());
        
        // Cleanup if needed
        if self.config.auto_cleanup {
            self.cleanup_expired_wallets();
        }
        
        Ok(wallet)
    }

    /// Activate wallet (mint tokens)
    pub fn activate_wallet(&mut self, wallet_id: &str) -> Result<(), String> {
        let wallet = self.wallets.get_mut(wallet_id)
            .ok_or_else(|| "Wallet not found".to_string())?;
        
        if wallet.status != WalletStatus::Created {
            return Err("Wallet is not in Created status".to_string());
        }
        
        wallet.status = WalletStatus::Active;
        wallet.last_activity = Utc::now();
        
        Ok(())
    }

    /// Transfer tokens from anonymous wallet to user wallet
    pub fn transfer_to_user(&mut self, wallet_id: &str, user_id: String, user_wallet_address: String) -> Result<u128, String> {
        let wallet = self.wallets.get_mut(wallet_id)
            .ok_or_else(|| "Wallet not found".to_string())?;
        
        // Validate wallet status
        if wallet.status != WalletStatus::Active {
            return Err("Wallet is not active for transfer".to_string());
        }
        
        // Check expiration
        let expiration_time = wallet.created_at + chrono::Duration::hours(self.config.expiration_hours as i64);
        if Utc::now() > expiration_time {
            wallet.status = WalletStatus::Expired;
            return Err("Wallet has expired".to_string());
        }
        
        // Transfer tokens
        let tokens_to_transfer = wallet.st_tokens;
        wallet.status = WalletStatus::Transferred;
        wallet.transferred_at = Some(Utc::now());
        wallet.claimed_by = Some(user_id);
        wallet.user_wallet_address = Some(user_wallet_address);
        wallet.last_activity = Utc::now();
        
        Ok(tokens_to_transfer)
    }

    /// Get wallet by ID
    pub fn get_wallet(&self, wallet_id: &str) -> Option<&AnonymousWallet> {
        self.wallets.get(wallet_id)
    }

    /// Get wallet by check ID
    pub fn get_wallet_by_check(&self, check_id: &str) -> Option<&AnonymousWallet> {
        self.wallets.values().find(|w| w.check_id == check_id)
    }

    /// Get all active wallets
    pub fn get_active_wallets(&self) -> Vec<&AnonymousWallet> {
        self.wallets.values()
            .filter(|w| w.status == WalletStatus::Active)
            .collect()
    }

    /// Get all expired wallets
    pub fn get_expired_wallets(&self) -> Vec<&AnonymousWallet> {
        self.wallets.values()
            .filter(|w| w.status == WalletStatus::Expired)
            .collect()
    }

    /// Get all transferred wallets
    pub fn get_transferred_wallets(&self) -> Vec<&AnonymousWallet> {
        self.wallets.values()
            .filter(|w| w.status == WalletStatus::Transferred)
            .collect()
    }

    /// Mark wallet as discarded
    pub fn discard_wallet(&mut self, wallet_id: &str) -> Result<(), String> {
        let wallet = self.wallets.get_mut(wallet_id)
            .ok_or_else(|| "Wallet not found".to_string())?;
        
        if wallet.status != WalletStatus::Active {
            return Err("Wallet is not active for discarding".to_string());
        }
        
        wallet.status = WalletStatus::Discarded;
        wallet.last_activity = Utc::now();
        
        Ok(())
    }

    /// Cleanup expired wallets
    pub fn cleanup_expired_wallets(&mut self) {
        let now = Utc::now();
        let expiration_duration = chrono::Duration::hours(self.config.expiration_hours as i64);
        
        // Mark expired wallets
        for wallet in self.wallets.values_mut() {
            if wallet.status == WalletStatus::Active && 
               now > wallet.created_at + expiration_duration {
                wallet.status = WalletStatus::Expired;
            }
        }
        
        // Remove old transferred/discarded wallets if we exceed max_wallets
        if self.wallets.len() > self.config.max_wallets {
            let mut to_remove = Vec::new();
            
            // Collect old transferred/discarded wallets
            for (wallet_id, wallet) in &self.wallets {
                if (wallet.status == WalletStatus::Transferred || wallet.status == WalletStatus::Discarded) &&
                   now > wallet.last_activity + chrono::Duration::days(30) {
                    to_remove.push(wallet_id.clone());
                }
            }
            
            // Remove old wallets
            for wallet_id in to_remove {
                self.wallets.remove(&wallet_id);
            }
        }
    }

    /// Generate deterministic wallet address
    fn generate_wallet_address(&self, wallet_id: &str, check_id: &str) -> Result<String, String> {
        // Create deterministic input
        let input = format!("{}{}{}{}", wallet_id, check_id, self.config.network_id, "thehotpotspot");
        
        // Hash the input
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        
        // Convert to hex and take first 40 characters for address
        let address_hex = hex::encode(hash);
        let address = format!("0x{}", &address_hex[..40]);
        
        Ok(address)
    }

    /// Get wallet statistics
    pub fn get_statistics(&self) -> AnonymousWalletStatistics {
        let total_wallets = self.wallets.len();
        let created = self.wallets.values().filter(|w| w.status == WalletStatus::Created).count();
        let active = self.wallets.values().filter(|w| w.status == WalletStatus::Active).count();
        let transferred = self.wallets.values().filter(|w| w.status == WalletStatus::Transferred).count();
        let expired = self.wallets.values().filter(|w| w.status == WalletStatus::Expired).count();
        let discarded = self.wallets.values().filter(|w| w.status == WalletStatus::Discarded).count();
        
        let total_tokens = self.wallets.values()
            .filter(|w| w.status == WalletStatus::Transferred)
            .map(|w| w.st_tokens)
            .sum();
        
        let unclaimed_tokens = self.wallets.values()
            .filter(|w| w.status == WalletStatus::Active || w.status == WalletStatus::Expired)
            .map(|w| w.st_tokens)
            .sum();
        
        AnonymousWalletStatistics {
            total_wallets,
            created,
            active,
            transferred,
            expired,
            discarded,
            total_tokens_transferred: total_tokens,
            total_tokens_unclaimed: unclaimed_tokens,
        }
    }

    /// Get wallets for redistribution
    pub fn get_wallets_for_redistribution(&self) -> Vec<&AnonymousWallet> {
        self.wallets.values()
            .filter(|w| w.status == WalletStatus::Expired || w.status == WalletStatus::Discarded)
            .collect()
    }

    /// Calculate total tokens available for redistribution
    pub fn get_redistribution_tokens(&self) -> u128 {
        self.wallets.values()
            .filter(|w| w.status == WalletStatus::Expired || w.status == WalletStatus::Discarded)
            .map(|w| w.st_tokens)
            .sum()
    }
}

/// Anonymous wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousWalletStatistics {
    /// Total number of wallets
    pub total_wallets: usize,
    /// Number of created wallets
    pub created: usize,
    /// Number of active wallets
    pub active: usize,
    /// Number of transferred wallets
    pub transferred: usize,
    /// Number of expired wallets
    pub expired: usize,
    /// Number of discarded wallets
    pub discarded: usize,
    /// Total tokens transferred
    pub total_tokens_transferred: u128,
    /// Total tokens unclaimed
    pub total_tokens_unclaimed: u128,
}

/// Wallet transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransferRequest {
    /// Anonymous wallet ID
    pub wallet_id: String,
    /// User ID claiming the wallet
    pub user_id: String,
    /// User's personal wallet address
    pub user_wallet_address: String,
}

/// Wallet transfer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransferResponse {
    /// Success status
    pub success: bool,
    /// Transferred tokens
    pub transferred_tokens: u128,
    /// Error message (if any)
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymous_wallet_creation() {
        let config = AnonymousWalletConfig::default();
        let mut manager = AnonymousWalletManager::new(config);
        
        let wallet = manager.create_wallet(
            "check_001".to_string(),
            "sale_001".to_string(),
            "node_001".to_string(),
            25.0,
            500
        ).unwrap();
        
        assert_eq!(wallet.check_id, "check_001");
        assert_eq!(wallet.amount_gel, 25.0);
        assert_eq!(wallet.st_tokens, 500);
        assert_eq!(wallet.status, WalletStatus::Created);
        assert!(wallet.address.starts_with("0x"));
    }

    #[test]
    fn test_wallet_activation() {
        let config = AnonymousWalletConfig::default();
        let mut manager = AnonymousWalletManager::new(config);
        
        let wallet = manager.create_wallet(
            "check_001".to_string(),
            "sale_001".to_string(),
            "node_001".to_string(),
            25.0,
            500
        ).unwrap();
        
        manager.activate_wallet(&wallet.wallet_id).unwrap();
        
        let activated_wallet = manager.get_wallet(&wallet.wallet_id).unwrap();
        assert_eq!(activated_wallet.status, WalletStatus::Active);
    }

    #[test]
    fn test_wallet_transfer() {
        let config = AnonymousWalletConfig::default();
        let mut manager = AnonymousWalletManager::new(config);
        
        let wallet = manager.create_wallet(
            "check_001".to_string(),
            "sale_001".to_string(),
            "node_001".to_string(),
            25.0,
            500
        ).unwrap();
        
        manager.activate_wallet(&wallet.wallet_id).unwrap();
        
        let transferred_tokens = manager.transfer_to_user(
            &wallet.wallet_id,
            "user_001".to_string(),
            "0xuser_wallet".to_string()
        ).unwrap();
        
        assert_eq!(transferred_tokens, 500);
        
        let transferred_wallet = manager.get_wallet(&wallet.wallet_id).unwrap();
        assert_eq!(transferred_wallet.status, WalletStatus::Transferred);
        assert_eq!(transferred_wallet.claimed_by, Some("user_001".to_string()));
        assert_eq!(transferred_wallet.user_wallet_address, Some("0xuser_wallet".to_string()));
    }

    #[test]
    fn test_wallet_statistics() {
        let config = AnonymousWalletConfig::default();
        let mut manager = AnonymousWalletManager::new(config);
        
        // Create multiple wallets
        for i in 0..5 {
            let wallet = manager.create_wallet(
                format!("check_{}", i),
                format!("sale_{}", i),
                "node_001".to_string(),
                25.0,
                500
            ).unwrap();
            
            manager.activate_wallet(&wallet.wallet_id).unwrap();
        }
        
        // Transfer some wallets
        let wallet_ids: Vec<String> = manager.get_active_wallets().into_iter()
            .map(|w| w.wallet_id.clone())
            .take(3)
            .collect();
        for (i, wallet_id) in wallet_ids.iter().enumerate() {
            let _ = manager.transfer_to_user(
                wallet_id,
                format!("user_{}", i),
                format!("0xuser_wallet_{}", i)
            );
        }
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_wallets, 5);
        assert_eq!(stats.active, 2);
        assert_eq!(stats.transferred, 3);
        assert_eq!(stats.total_tokens_transferred, 1500);
        assert_eq!(stats.total_tokens_unclaimed, 1000);
    }
}
