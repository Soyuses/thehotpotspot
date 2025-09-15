//! New Relayer Service for The Hot Pot Spot
//! 
//! This module implements the relayer service for processing sales and minting ST tokens
//! according to the new tokenomics model.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::new_tokenomics::{NewTokenomicsManager, SaleRecord, StMinting, KycStatus};
use crate::new_database::{NewDatabaseManager, User, NewDatabaseConfig};
use crate::tokenomics_config::TokenomicsConfig;

/// Relayer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerConfig {
    /// Minimum amount in GEL for ST minting
    pub min_amount_gel: f64,
    /// Maximum amount in GEL per transaction
    pub max_amount_gel: f64,
    /// ST tokens per 1 GEL spent
    pub st_per_gel: u128,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay_seconds: u64,
    /// KYC required for ST transfers
    pub kyc_required: bool,
}

impl Default for RelayerConfig {
    fn default() -> Self {
        Self {
            min_amount_gel: 1.0,
            max_amount_gel: 1000.0,
            st_per_gel: 100, // 1 ST per 1 GEL (with scale 100)
            max_retries: 3,
            retry_delay_seconds: 5,
            kyc_required: true,
        }
    }
}

/// Sale request from POS system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleRequest {
    pub sale_id: String,
    pub node_id: String,
    pub customer_phone: Option<String>,
    pub amount_gel: f64,
    pub items: Vec<SaleItem>,
    pub timestamp: DateTime<Utc>,
}

/// Sale item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItem {
    pub item_id: String,
    pub name: String,
    pub price_gel: f64,
    pub quantity: u32,
}

/// Relayer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerResponse {
    pub success: bool,
    pub sale_id: String,
    pub check_address: String,
    pub activation_code: String,
    pub st_units: u128,
    pub error_message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Relayer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayerStats {
    pub total_sales: u64,
    pub successful_sales: u64,
    pub failed_sales: u64,
    pub total_st_minted: u128,
    pub total_amount_gel: f64,
    pub last_updated: DateTime<Utc>,
}

/// New Relayer Service
pub struct NewRelayerService {
    config: RelayerConfig,
    tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
    database: Arc<NewDatabaseManager>,
    stats: Arc<RwLock<RelayerStats>>,
}

impl NewRelayerService {
    /// Create a new relayer service
    pub fn new(
        config: RelayerConfig,
        tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
        database: Arc<NewDatabaseManager>,
    ) -> Self {
        let stats = Arc::new(RwLock::new(RelayerStats {
            total_sales: 0,
            successful_sales: 0,
            failed_sales: 0,
            total_st_minted: 0,
            total_amount_gel: 0.0,
            last_updated: Utc::now(),
        }));

        Self {
            config,
            tokenomics_manager,
            database,
            stats,
        }
    }

    /// Process sale request from POS system
    pub async fn process_sale(&self, request: SaleRequest) -> Result<RelayerResponse, String> {
        // Validate request
        self.validate_sale_request(&request)?;

        // Update statistics
        self.update_stats_total_sales().await;

        // Generate check address and activation code
        let check_address = self.generate_check_address(&request.sale_id);
        let activation_code = self.generate_activation_code();

        // Calculate ST units
        let st_units = self.calculate_st_units(request.amount_gel);

        // Create sale record in tokenomics manager
        let sale_record = SaleRecord {
            sale_id: request.sale_id.clone(),
            node_id: request.node_id.clone(),
            user_id: None, // Will be set when user claims
            amount_gel: request.amount_gel,
            st_units,
            check_address: check_address.clone(),
            activation_code_hash: format!("{:x}", md5::compute(&activation_code)),
            timestamp: request.timestamp,
            status: crate::new_tokenomics::SaleStatus::Pending,
        };

        // Add sale to tokenomics manager
        {
            let mut manager = self.tokenomics_manager.write().await;
            manager.add_sale(sale_record.clone())?;
        }

        // Save to database
        self.database.create_sale(
            request.sale_id.clone(),
            request.node_id.clone(),
            None, // user_id will be set when claimed
            request.amount_gel,
            st_units as i64,
            check_address.clone(),
            format!("{:x}", md5::compute(&activation_code)),
        ).await?;

        // Update statistics
        self.update_stats_successful_sale(st_units, request.amount_gel).await;

        Ok(RelayerResponse {
            success: true,
            sale_id: request.sale_id,
            check_address,
            activation_code,
            st_units,
            error_message: None,
            timestamp: Utc::now(),
        })
    }

    /// Claim ST tokens from check
    pub async fn claim_st_tokens(
        &self,
        check_address: String,
        activation_code: String,
        user_phone: String,
        user_wallet: String,
    ) -> Result<StMinting, String> {
        // Get or create user
        let user = self.get_or_create_user(user_phone, user_wallet.clone()).await?;

        // Check KYC status if required
        if self.config.kyc_required && user.kyc_status != "verified" {
            return Err("KYC verification required for ST token claims".to_string());
        }

        // Convert KYC status
        let kyc_status = match user.kyc_status.as_str() {
            "not_required" => KycStatus::NotRequired,
            "pending" => KycStatus::Pending,
            "verified" => KycStatus::Verified,
            "rejected" => KycStatus::Rejected,
            "expired" => KycStatus::Expired,
            _ => KycStatus::Pending,
        };

        // Claim tokens from tokenomics manager
        let minting = {
            let mut manager = self.tokenomics_manager.write().await;
            manager.claim_st_tokens(check_address, activation_code, user_wallet, kyc_status)?
        };

        // Update sale status in database
        self.database.update_sale_status(&minting.sale_id, "claimed").await?;

        // Create ST minting record in database
        self.database.create_st_minting(
            minting.mint_id.clone(),
            minting.sale_id.clone(),
            minting.units as i64,
            minting.to_address.clone(),
            minting.transaction_hash.clone(),
        ).await?;

        Ok(minting)
    }

    /// Get relayer statistics
    pub async fn get_stats(&self) -> RelayerStats {
        self.stats.read().await.clone()
    }

    /// Reset statistics
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = RelayerStats {
            total_sales: 0,
            successful_sales: 0,
            failed_sales: 0,
            total_st_minted: 0,
            total_amount_gel: 0.0,
            last_updated: Utc::now(),
        };
    }

    /// Validate sale request
    fn validate_sale_request(&self, request: &SaleRequest) -> Result<(), String> {
        if request.amount_gel < self.config.min_amount_gel {
            return Err(format!("Amount too small: {} GEL (minimum: {} GEL)", 
                request.amount_gel, self.config.min_amount_gel));
        }

        if request.amount_gel > self.config.max_amount_gel {
            return Err(format!("Amount too large: {} GEL (maximum: {} GEL)", 
                request.amount_gel, self.config.max_amount_gel));
        }

        if request.sale_id.is_empty() {
            return Err("Sale ID cannot be empty".to_string());
        }

        if request.node_id.is_empty() {
            return Err("Node ID cannot be empty".to_string());
        }

        if request.items.is_empty() {
            return Err("Sale must contain at least one item".to_string());
        }

        // Validate items
        let total_items_amount: f64 = request.items.iter()
            .map(|item| item.price_gel * item.quantity as f64)
            .sum();

        if (total_items_amount - request.amount_gel).abs() > 0.01 {
            return Err("Items total amount does not match sale amount".to_string());
        }

        Ok(())
    }

    /// Calculate ST units based on amount
    fn calculate_st_units(&self, amount_gel: f64) -> u128 {
        (amount_gel * self.config.st_per_gel as f64) as u128
    }

    /// Generate check address
    fn generate_check_address(&self, sale_id: &str) -> String {
        format!("0x{}", hex::encode(&format!("check_{}", sale_id)))
    }

    /// Generate activation code
    fn generate_activation_code(&self) -> String {
        use fastrand;
        format!("{:06}", fastrand::u32(100000..999999))
    }

    /// Get or create user
    async fn get_or_create_user(&self, phone: String, wallet: String) -> Result<User, String> {
        // Hash phone number for privacy
        let phone_hash = format!("{:x}", md5::compute(&phone));

        // Try to get existing user
        if let Some(user) = self.database.get_user_by_phone(&phone_hash).await? {
            return Ok(user);
        }

        // Try to get user by wallet
        if let Some(user) = self.database.get_user_by_wallet(&wallet).await? {
            return Ok(user);
        }

        // Create new user
        self.database.create_user(
            phone_hash,
            Some(wallet),
            None, // full_name
            None, // email
            None, // t_shirt_size
            None, // favorite_dish
        ).await
    }

    /// Update total sales statistics
    async fn update_stats_total_sales(&self) {
        let mut stats = self.stats.write().await;
        stats.total_sales += 1;
        stats.last_updated = Utc::now();
    }

    /// Update successful sale statistics
    async fn update_stats_successful_sale(&self, st_units: u128, amount_gel: f64) {
        let mut stats = self.stats.write().await;
        stats.successful_sales += 1;
        stats.total_st_minted += st_units;
        stats.total_amount_gel += amount_gel;
        stats.last_updated = Utc::now();
    }

    /// Update failed sale statistics
    async fn update_stats_failed_sale(&self) {
        let mut stats = self.stats.write().await;
        stats.failed_sales += 1;
        stats.last_updated = Utc::now();
    }
}

/// Relayer service error
#[derive(Debug, thiserror::Error)]
pub enum RelayerError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Tokenomics error: {0}")]
    Tokenomics(String),
    #[error("KYC error: {0}")]
    Kyc(String),
    #[error("Network error: {0}")]
    Network(String),
}

impl From<String> for RelayerError {
    fn from(err: String) -> Self {
        RelayerError::Validation(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_relayer_config_default() {
        let config = RelayerConfig::default();
        assert_eq!(config.min_amount_gel, 1.0);
        assert_eq!(config.max_amount_gel, 1000.0);
        assert_eq!(config.st_per_gel, 100);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.kyc_required, true);
    }

    #[test]
    fn test_sale_request_validation() {
        let config = RelayerConfig::default();
        let service = NewRelayerService::new(
            config,
            Arc::new(RwLock::new(NewTokenomicsManager::new(TokenomicsConfig::default()))),
            Arc::new(NewDatabaseManager::new(NewDatabaseConfig::default()).await.unwrap()),
        );

        let valid_request = SaleRequest {
            sale_id: "test_sale_001".to_string(),
            node_id: "node_001".to_string(),
            customer_phone: Some("+995123456789".to_string()),
            amount_gel: 10.0,
            items: vec![SaleItem {
                item_id: "item_001".to_string(),
                name: "Test Item".to_string(),
                price_gel: 10.0,
                quantity: 1,
            }],
            timestamp: Utc::now(),
        };

        assert!(service.validate_sale_request(&valid_request).is_ok());
    }

    #[test]
    fn test_st_units_calculation() {
        let config = RelayerConfig::default();
        let service = NewRelayerService::new(
            config,
            Arc::new(RwLock::new(NewTokenomicsManager::new(TokenomicsConfig::default()))),
            Arc::new(NewDatabaseManager::new(NewDatabaseConfig::default()).await.unwrap()),
        );

        assert_eq!(service.calculate_st_units(1.0), 100);
        assert_eq!(service.calculate_st_units(5.0), 500);
        assert_eq!(service.calculate_st_units(10.0), 1000);
    }

    #[test]
    fn test_check_address_generation() {
        let config = RelayerConfig::default();
        let service = NewRelayerService::new(
            config,
            Arc::new(RwLock::new(NewTokenomicsManager::new(TokenomicsConfig::default()))),
            Arc::new(NewDatabaseManager::new(NewDatabaseConfig::default()).await.unwrap()),
        );

        let address = service.generate_check_address("test_sale_001");
        assert!(address.starts_with("0x"));
        assert!(address.len() > 10);
    }

    #[test]
    fn test_activation_code_generation() {
        let config = RelayerConfig::default();
        let service = NewRelayerService::new(
            config,
            Arc::new(RwLock::new(NewTokenomicsManager::new(TokenomicsConfig::default()))),
            Arc::new(NewDatabaseManager::new(NewDatabaseConfig::default()).await.unwrap()),
        );

        let code = service.generate_activation_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }
}
