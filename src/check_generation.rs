//! Check Generation Module for The Hot Pot Spot
//! 
//! This module handles the generation of physical checks with QR codes
//! for the customer journey implementation.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use qrcode::QrCode;
use image::{ImageBuffer, Rgb, RgbImage};
use std::io::Cursor;

/// Physical check with QR code for customer journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalCheck {
    /// Unique check ID
    pub check_id: String,
    /// Sale ID this check is associated with
    pub sale_id: String,
    /// Node ID where the sale occurred
    pub node_id: String,
    /// Anonymous wallet address for tokens
    pub wallet_address: String,
    /// Amount in GEL
    pub amount_gel: f64,
    /// ST tokens to be minted
    pub st_tokens: u128,
    /// QR code data
    pub qr_data: String,
    /// QR code image (base64 encoded)
    pub qr_image: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Check status
    pub status: CheckStatus,
    /// Claim timestamp (if claimed)
    pub claimed_at: Option<DateTime<Utc>>,
    /// User ID who claimed the check (if claimed)
    pub claimed_by: Option<String>,
}

/// Check status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckStatus {
    /// Check generated but not printed
    Generated,
    /// Check printed and given to customer
    Printed,
    /// Check claimed by customer via mobile app
    Claimed,
    /// Check expired (not claimed within time limit)
    Expired,
    /// Check discarded by customer
    Discarded,
}

/// QR code data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRCodeData {
    /// Check ID
    pub check_id: String,
    /// Wallet address
    pub wallet_address: String,
    /// Amount in GEL
    pub amount_gel: f64,
    /// ST tokens
    pub st_tokens: u128,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Mobile app download URL
    pub app_download_url: String,
    /// Activation code
    pub activation_code: String,
}

/// Check generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckGenerationRequest {
    /// Sale ID
    pub sale_id: String,
    /// Node ID
    pub node_id: String,
    /// Amount in GEL
    pub amount_gel: f64,
    /// ST tokens to mint
    pub st_tokens: u128,
    /// Customer phone (optional)
    pub customer_phone: Option<String>,
}

/// Check generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckGenerationResponse {
    /// Generated check
    pub check: PhysicalCheck,
    /// Success status
    pub success: bool,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Check claim request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckClaimRequest {
    /// QR code data
    pub qr_data: String,
    /// User ID claiming the check
    pub user_id: String,
    /// User's personal wallet address
    pub user_wallet: String,
}

/// Check claim response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckClaimResponse {
    /// Success status
    pub success: bool,
    /// Transferred ST tokens
    pub transferred_tokens: u128,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Check generation service
#[derive(Debug, Clone)]
pub struct CheckGenerationService {
    /// Configuration
    pub config: CheckGenerationConfig,
    /// Generated checks storage
    pub checks: HashMap<String, PhysicalCheck>,
    /// Mobile app download URL
    pub app_download_url: String,
}

/// Check generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckGenerationConfig {
    /// QR code size in pixels
    pub qr_size: u32,
    /// Check expiration time in hours
    pub expiration_hours: u32,
    /// Mobile app download URL
    pub app_download_url: String,
    /// Enable QR code generation
    pub enable_qr_generation: bool,
    /// QR code error correction level
    pub qr_error_correction: QRErrorCorrection,
}

/// QR code error correction levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QRErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

impl Default for CheckGenerationConfig {
    fn default() -> Self {
        Self {
            qr_size: 256,
            expiration_hours: 24,
            app_download_url: "https://thehotpotspot.com/mobile-app".to_string(),
            enable_qr_generation: true,
            qr_error_correction: QRErrorCorrection::Medium,
        }
    }
}

impl CheckGenerationService {
    /// Create new check generation service
    pub fn new(config: CheckGenerationConfig) -> Self {
        Self {
            config,
            checks: HashMap::new(),
            app_download_url: "https://thehotpotspot.com/mobile-app".to_string(),
        }
    }

    /// Generate a new physical check with QR code
    pub fn generate_check(&mut self, request: CheckGenerationRequest) -> Result<CheckGenerationResponse, String> {
        // Generate unique check ID
        let check_id = format!("check_{}", Uuid::new_v4().to_string()[..8].to_uppercase());
        
        // Generate anonymous wallet address
        let wallet_address = self.generate_anonymous_wallet(&check_id);
        
        // Generate activation code
        let activation_code = self.generate_activation_code();
        
        // Create QR code data
        let qr_data = QRCodeData {
            check_id: check_id.clone(),
            wallet_address: wallet_address.clone(),
            amount_gel: request.amount_gel,
            st_tokens: request.st_tokens,
            timestamp: Utc::now(),
            app_download_url: self.app_download_url.clone(),
            activation_code: activation_code.clone(),
        };
        
        // Generate QR code image
        let qr_image = if self.config.enable_qr_generation {
            self.generate_qr_code_image(&qr_data)?
        } else {
            "".to_string()
        };
        
        // Create physical check
        let check = PhysicalCheck {
            check_id: check_id.clone(),
            sale_id: request.sale_id,
            node_id: request.node_id,
            wallet_address,
            amount_gel: request.amount_gel,
            st_tokens: request.st_tokens,
            qr_data: serde_json::to_string(&qr_data).map_err(|e| format!("Failed to serialize QR data: {}", e))?,
            qr_image,
            created_at: Utc::now(),
            status: CheckStatus::Generated,
            claimed_at: None,
            claimed_by: None,
        };
        
        // Store check
        self.checks.insert(check_id.clone(), check.clone());
        
        Ok(CheckGenerationResponse {
            check,
            success: true,
            error: None,
        })
    }

    /// Claim a check using QR code data
    pub fn claim_check(&mut self, request: CheckClaimRequest) -> Result<CheckClaimResponse, String> {
        // Parse QR code data
        let qr_data: QRCodeData = serde_json::from_str(&request.qr_data)
            .map_err(|e| format!("Invalid QR code data: {}", e))?;
        
        // Find check
        let check = self.checks.get_mut(&qr_data.check_id)
            .ok_or_else(|| "Check not found".to_string())?;
        
        // Validate check status
        if check.status != CheckStatus::Printed {
            return Err("Check is not available for claiming".to_string());
        }
        
        // Check expiration
        let expiration_time = check.created_at + chrono::Duration::hours(self.config.expiration_hours as i64);
        if Utc::now() > expiration_time {
            check.status = CheckStatus::Expired;
            return Err("Check has expired".to_string());
        }
        
        // Update check status
        check.status = CheckStatus::Claimed;
        check.claimed_at = Some(Utc::now());
        check.claimed_by = Some(request.user_id);
        
        Ok(CheckClaimResponse {
            success: true,
            transferred_tokens: check.st_tokens,
            error: None,
        })
    }

    /// Get check by ID
    pub fn get_check(&self, check_id: &str) -> Option<&PhysicalCheck> {
        self.checks.get(check_id)
    }

    /// Get all unclaimed checks
    pub fn get_unclaimed_checks(&self) -> Vec<&PhysicalCheck> {
        self.checks.values()
            .filter(|check| check.status == CheckStatus::Printed)
            .collect()
    }

    /// Get expired checks for redistribution
    pub fn get_expired_checks(&self) -> Vec<&PhysicalCheck> {
        self.checks.values()
            .filter(|check| check.status == CheckStatus::Expired)
            .collect()
    }

    /// Generate anonymous wallet address
    fn generate_anonymous_wallet(&self, check_id: &str) -> String {
        // In a real implementation, this would generate a proper blockchain address
        // For now, we'll create a deterministic address based on check ID
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        check_id.hash(&mut hasher);
        let hash = hasher.finish();
        format!("0x{:040x}", hash)
    }

    /// Generate activation code
    fn generate_activation_code(&self) -> String {
        // Generate 6-digit activation code
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(100000..999999))
    }

    /// Generate QR code image
    fn generate_qr_code_image(&self, qr_data: &QRCodeData) -> Result<String, String> {
        // Serialize QR data
        let qr_string = serde_json::to_string(qr_data)
            .map_err(|e| format!("Failed to serialize QR data: {}", e))?;
        
        // Generate QR code
        let code = QrCode::new(&qr_string)
            .map_err(|e| format!("Failed to generate QR code: {}", e))?;
        
        // Convert to image
        let image = code.render::<Rgb<u8>>().build();
        
        // Convert to base64
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            image::DynamicImage::ImageRgb8(image)
                .write_to(&mut cursor, image::ImageOutputFormat::Png)
                .map_err(|e| format!("Failed to encode image: {}", e))?;
        }
        
        use base64::{Engine as _, engine::general_purpose};
        Ok(general_purpose::STANDARD.encode(&buffer))
    }

    /// Print check (update status to Printed)
    pub fn print_check(&mut self, check_id: &str) -> Result<(), String> {
        let check = self.checks.get_mut(check_id)
            .ok_or_else(|| "Check not found".to_string())?;
        
        if check.status != CheckStatus::Generated {
            return Err("Check is not in Generated status".to_string());
        }
        
        check.status = CheckStatus::Printed;
        Ok(())
    }

    /// Mark check as discarded
    pub fn discard_check(&mut self, check_id: &str) -> Result<(), String> {
        let check = self.checks.get_mut(check_id)
            .ok_or_else(|| "Check not found".to_string())?;
        
        if check.status != CheckStatus::Printed {
            return Err("Check is not in Printed status".to_string());
        }
        
        check.status = CheckStatus::Discarded;
        Ok(())
    }

    /// Get check statistics
    pub fn get_statistics(&self) -> CheckStatistics {
        let total_checks = self.checks.len();
        let generated = self.checks.values().filter(|c| c.status == CheckStatus::Generated).count();
        let printed = self.checks.values().filter(|c| c.status == CheckStatus::Printed).count();
        let claimed = self.checks.values().filter(|c| c.status == CheckStatus::Claimed).count();
        let expired = self.checks.values().filter(|c| c.status == CheckStatus::Expired).count();
        let discarded = self.checks.values().filter(|c| c.status == CheckStatus::Discarded).count();
        
        let total_tokens = self.checks.values()
            .filter(|c| c.status == CheckStatus::Claimed)
            .map(|c| c.st_tokens)
            .sum();
        
        let unclaimed_tokens = self.checks.values()
            .filter(|c| c.status == CheckStatus::Printed || c.status == CheckStatus::Expired)
            .map(|c| c.st_tokens)
            .sum();
        
        CheckStatistics {
            total_checks,
            generated,
            printed,
            claimed,
            expired,
            discarded,
            total_tokens_claimed: total_tokens,
            total_tokens_unclaimed: unclaimed_tokens,
        }
    }
}

/// Check statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckStatistics {
    /// Total number of checks
    pub total_checks: usize,
    /// Number of generated checks
    pub generated: usize,
    /// Number of printed checks
    pub printed: usize,
    /// Number of claimed checks
    pub claimed: usize,
    /// Number of expired checks
    pub expired: usize,
    /// Number of discarded checks
    pub discarded: usize,
    /// Total tokens claimed
    pub total_tokens_claimed: u128,
    /// Total tokens unclaimed
    pub total_tokens_unclaimed: u128,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_generation() {
        let config = CheckGenerationConfig::default();
        let mut service = CheckGenerationService::new(config);
        
        let request = CheckGenerationRequest {
            sale_id: "sale_001".to_string(),
            node_id: "node_001".to_string(),
            amount_gel: 25.0,
            st_tokens: 500,
            customer_phone: None,
        };
        
        let response = service.generate_check(request).unwrap();
        assert!(response.success);
        assert_eq!(response.check.amount_gel, 25.0);
        assert_eq!(response.check.st_tokens, 500);
        assert_eq!(response.check.status, CheckStatus::Generated);
    }

    #[test]
    fn test_check_claiming() {
        let config = CheckGenerationConfig::default();
        let mut service = CheckGenerationService::new(config);
        
        // Generate check
        let request = CheckGenerationRequest {
            sale_id: "sale_001".to_string(),
            node_id: "node_001".to_string(),
            amount_gel: 25.0,
            st_tokens: 500,
            customer_phone: None,
        };
        
        let response = service.generate_check(request).unwrap();
        let check_id = response.check.check_id.clone();
        
        // Print check
        service.print_check(&check_id).unwrap();
        
        // Claim check
        let claim_request = CheckClaimRequest {
            qr_data: response.check.qr_data.clone(),
            user_id: "user_001".to_string(),
            user_wallet: "0xuser_wallet".to_string(),
        };
        
        let claim_response = service.claim_check(claim_request).unwrap();
        assert!(claim_response.success);
        assert_eq!(claim_response.transferred_tokens, 500);
        
        // Verify check status
        let check = service.get_check(&check_id).unwrap();
        assert_eq!(check.status, CheckStatus::Claimed);
        assert!(check.claimed_at.is_some());
        assert_eq!(check.claimed_by, Some("user_001".to_string()));
    }

    #[test]
    fn test_check_statistics() {
        let config = CheckGenerationConfig::default();
        let mut service = CheckGenerationService::new(config);
        
        // Generate multiple checks
        for i in 0..5 {
            let request = CheckGenerationRequest {
                sale_id: format!("sale_{}", i),
                node_id: "node_001".to_string(),
                amount_gel: 25.0,
                st_tokens: 500,
                customer_phone: None,
            };
            
            let response = service.generate_check(request).unwrap();
            service.print_check(&response.check.check_id).unwrap();
        }
        
        let stats = service.get_statistics();
        assert_eq!(stats.total_checks, 5);
        assert_eq!(stats.generated, 0);
        assert_eq!(stats.printed, 5);
        assert_eq!(stats.claimed, 0);
        assert_eq!(stats.total_tokens_unclaimed, 2500);
    }
}

