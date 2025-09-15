use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Datelike};
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::new_tokenomics::{NewTokenomicsManager, KycStatus};
use crate::tokenomics_config::TokenomicsConfig;

/// Security and KYC validation system
#[derive(Debug, Clone)]
pub struct SecurityValidator {
    /// Tokenomics manager
    pub tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
    /// Configuration
    pub config: TokenomicsConfig,
    /// Security rules
    pub security_rules: SecurityRules,
    /// KYC requirements
    pub kyc_requirements: KycRequirements,
    /// Risk assessments
    pub risk_assessments: HashMap<String, RiskAssessment>,
}

/// Security rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRules {
    /// Maximum daily UT earning per user
    pub max_daily_ut_earning: u128,
    /// Maximum daily ST minting per user
    pub max_daily_st_minting: u128,
    /// Minimum time between transactions (seconds)
    pub min_transaction_interval: u64,
    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration (minutes)
    pub lockout_duration_minutes: u32,
    /// Suspicious activity threshold
    pub suspicious_activity_threshold: u32,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Enable IP tracking
    pub enable_ip_tracking: bool,
    /// Enable device fingerprinting
    pub enable_device_fingerprinting: bool,
}

impl Default for SecurityRules {
    fn default() -> Self {
        Self {
            max_daily_ut_earning: 10000, // 10,000 UT per day
            max_daily_st_minting: 100000, // 100,000 ST per day
            min_transaction_interval: 5, // 5 seconds
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            suspicious_activity_threshold: 10,
            enable_rate_limiting: true,
            enable_ip_tracking: true,
            enable_device_fingerprinting: true,
        }
    }
}

/// KYC requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycRequirements {
    /// Minimum age for KYC
    pub min_age: u8,
    /// Required documents
    pub required_documents: Vec<DocumentType>,
    /// Supported countries
    pub supported_countries: Vec<String>,
    /// Excluded countries
    pub excluded_countries: Vec<String>,
    /// PEP (Politically Exposed Person) check required
    pub pep_check_required: bool,
    /// Sanctions check required
    pub sanctions_check_required: bool,
    /// AML (Anti-Money Laundering) check required
    pub aml_check_required: bool,
    /// KYC expiration period (days)
    pub kyc_expiration_days: u32,
}

impl Default for KycRequirements {
    fn default() -> Self {
        Self {
            min_age: 18,
            required_documents: vec![
                DocumentType::Passport,
                DocumentType::NationalId,
                DocumentType::ProofOfAddress,
            ],
            supported_countries: vec![
                "GE".to_string(), // Georgia
                "US".to_string(), // United States
                "EU".to_string(), // European Union
            ],
            excluded_countries: vec![
                "KP".to_string(), // North Korea
                "IR".to_string(), // Iran
                "SY".to_string(), // Syria
            ],
            pep_check_required: true,
            sanctions_check_required: true,
            aml_check_required: true,
            kyc_expiration_days: 365,
        }
    }
}

/// Document types for KYC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentType {
    Passport,
    NationalId,
    DriverLicense,
    ProofOfAddress,
    BankStatement,
    UtilityBill,
    TaxDocument,
    EmploymentCertificate,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// User ID
    pub user_id: String,
    /// Risk score (0-100)
    pub risk_score: u8,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Mitigation measures
    pub mitigation_measures: Vec<String>,
}

/// Risk level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor type
    pub factor_type: RiskFactorType,
    /// Description
    pub description: String,
    /// Impact score (0-100)
    pub impact_score: u8,
    /// Mitigation suggestion
    pub mitigation: Option<String>,
}

/// Risk factor types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskFactorType {
    /// High transaction volume
    HighVolume,
    /// Unusual transaction patterns
    UnusualPattern,
    /// Geographic risk
    GeographicRisk,
    /// PEP status
    PepStatus,
    /// Sanctions list
    SanctionsList,
    /// AML risk
    AmlRisk,
    /// Device risk
    DeviceRisk,
    /// IP risk
    IpRisk,
    /// Behavioral risk
    BehavioralRisk,
}

/// Security validation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationRequest {
    /// User ID
    pub user_id: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Amount
    pub amount: u128,
    /// IP address
    pub ip_address: Option<String>,
    /// Device fingerprint
    pub device_fingerprint: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Transaction types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    /// UT earning
    UtEarning,
    /// ST minting
    StMinting,
    /// ST transfer
    StTransfer,
    /// UT conversion
    UtConversion,
    /// Proposal creation
    ProposalCreation,
    /// Voting
    Voting,
}

/// Security validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationResponse {
    /// Validation result
    pub is_valid: bool,
    /// Risk score
    pub risk_score: u8,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Blocked reasons
    pub blocked_reasons: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Requires additional verification
    pub requires_additional_verification: bool,
}

/// KYC validation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycValidationRequest {
    /// User ID
    pub user_id: String,
    /// Full name
    pub full_name: String,
    /// Date of birth
    pub date_of_birth: DateTime<Utc>,
    /// Nationality
    pub nationality: String,
    /// Address
    pub address: String,
    /// Phone number
    pub phone: String,
    /// Email
    pub email: String,
    /// Documents
    pub documents: Vec<DocumentInfo>,
}

/// Document information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    /// Document type
    pub document_type: DocumentType,
    /// Document number
    pub document_number: String,
    /// Issuing country
    pub issuing_country: String,
    /// Expiry date
    pub expiry_date: Option<DateTime<Utc>>,
    /// Document hash
    pub document_hash: String,
}

/// KYC validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycValidationResponse {
    /// Validation result
    pub is_valid: bool,
    /// KYC status
    pub kyc_status: KycStatus,
    /// Validation score
    pub validation_score: u8,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Next steps
    pub next_steps: Vec<String>,
}

impl SecurityValidator {
    /// Create a new security validator
    pub fn new(
        tokenomics_manager: Arc<RwLock<NewTokenomicsManager>>,
        config: TokenomicsConfig,
    ) -> Self {
        Self {
            tokenomics_manager,
            config,
            security_rules: SecurityRules::default(),
            kyc_requirements: KycRequirements::default(),
            risk_assessments: HashMap::new(),
        }
    }

    /// Validate security for a transaction
    pub async fn validate_security(&mut self, request: SecurityValidationRequest) -> SecurityValidationResponse {
        let mut blocked_reasons = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut risk_factors = Vec::new();
        let mut total_risk_score = 0u8;

        // Check rate limiting
        if self.security_rules.enable_rate_limiting {
            if let Err(reason) = self.check_rate_limiting(&request).await {
                blocked_reasons.push(reason);
            }
        }

        // Check daily limits
        if let Err(reason) = self.check_daily_limits(&request).await {
            blocked_reasons.push(reason);
        }

        // Check transaction patterns
        if let Some(risk_factor) = self.check_transaction_patterns(&request).await {
            risk_factors.push(risk_factor.clone());
            total_risk_score += risk_factor.impact_score;
        }

        // Check geographic risk
        if self.security_rules.enable_ip_tracking {
            if let Some(risk_factor) = self.check_geographic_risk(&request).await {
                risk_factors.push(risk_factor.clone());
                total_risk_score += risk_factor.impact_score;
            }
        }

        // Check device risk
        if self.security_rules.enable_device_fingerprinting {
            if let Some(risk_factor) = self.check_device_risk(&request).await {
                risk_factors.push(risk_factor.clone());
                total_risk_score += risk_factor.impact_score;
            }
        }

        // Determine risk level
        let risk_level = self.determine_risk_level(total_risk_score);
        
        // Generate recommendations
        if total_risk_score > 50 {
            recommendations.push("Consider additional verification".to_string());
        }
        if total_risk_score > 75 {
            recommendations.push("Manual review recommended".to_string());
        }

        // Check if validation passes
        let is_valid = blocked_reasons.is_empty() && total_risk_score < 90;

        // Update risk assessment
        if !risk_factors.is_empty() {
            let assessment = RiskAssessment {
                user_id: request.user_id.clone(),
                risk_score: total_risk_score,
                risk_level: risk_level.clone(),
                assessed_at: Utc::now(),
                risk_factors: risk_factors.clone(),
                mitigation_measures: recommendations.clone(),
            };
            self.risk_assessments.insert(request.user_id.clone(), assessment);
        }

        SecurityValidationResponse {
            is_valid,
            risk_score: total_risk_score,
            risk_level,
            blocked_reasons,
            warnings,
            recommendations,
            requires_additional_verification: total_risk_score > 70,
        }
    }

    /// Validate KYC information
    pub async fn validate_kyc(&self, request: KycValidationRequest) -> KycValidationResponse {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut next_steps = Vec::new();
        let mut validation_score = 100u8;

        // Check age requirement
        let age = self.calculate_age(request.date_of_birth);
        if age < self.kyc_requirements.min_age {
            issues.push(format!("User is under minimum age requirement ({})", self.kyc_requirements.min_age));
            validation_score -= 20;
        }

        // Check country support
        if !self.kyc_requirements.supported_countries.contains(&request.nationality) {
            issues.push("Country not supported for KYC".to_string());
            validation_score -= 30;
        }

        // Check excluded countries
        if self.kyc_requirements.excluded_countries.contains(&request.nationality) {
            issues.push("Country is excluded from KYC".to_string());
            validation_score -= 50;
        }

        // Check required documents
        let provided_doc_types: Vec<&DocumentType> = request.documents.iter().map(|d| &d.document_type).collect();
        for required_doc in &self.kyc_requirements.required_documents {
            if !provided_doc_types.contains(&required_doc) {
                issues.push(format!("Missing required document: {:?}", required_doc));
                validation_score -= 15;
            }
        }

        // Check document validity
        for document in &request.documents {
            if let Some(expiry_date) = document.expiry_date {
                if expiry_date < Utc::now() {
                    issues.push(format!("Document expired: {:?}", document.document_type));
                    validation_score -= 10;
                }
            }
        }

        // Generate recommendations
        if validation_score < 80 {
            recommendations.push("Provide additional documentation".to_string());
        }
        if validation_score < 60 {
            recommendations.push("Manual review required".to_string());
        }

        // Generate next steps
        if validation_score >= 80 {
            next_steps.push("KYC verification can proceed".to_string());
        } else {
            next_steps.push("Address identified issues".to_string());
            next_steps.push("Resubmit KYC application".to_string());
        }

        // Determine KYC status
        let kyc_status = if validation_score >= 80 {
            KycStatus::Pending
        } else if validation_score >= 60 {
            KycStatus::Pending
        } else {
            KycStatus::Rejected
        };

        KycValidationResponse {
            is_valid: validation_score >= 60,
            kyc_status,
            validation_score,
            issues,
            recommendations,
            next_steps,
        }
    }

    /// Check rate limiting
    async fn check_rate_limiting(&self, request: &SecurityValidationRequest) -> Result<(), String> {
        // In a real implementation, this would check against a rate limiting service
        // For now, we'll simulate a basic check
        Ok(())
    }

    /// Check daily limits
    async fn check_daily_limits(&self, request: &SecurityValidationRequest) -> Result<(), String> {
        let tokenomics_manager = self.tokenomics_manager.read().await;
        
        match request.transaction_type {
            TransactionType::UtEarning => {
                // Check daily UT earning limit
                // In a real implementation, this would check against daily UT earning records
                if request.amount > self.security_rules.max_daily_ut_earning {
                    return Err(format!("Daily UT earning limit exceeded: {} > {}", 
                        request.amount, self.security_rules.max_daily_ut_earning));
                }
            }
            TransactionType::StMinting => {
                // Check daily ST minting limit
                if request.amount > self.security_rules.max_daily_st_minting {
                    return Err(format!("Daily ST minting limit exceeded: {} > {}", 
                        request.amount, self.security_rules.max_daily_st_minting));
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    /// Check transaction patterns
    async fn check_transaction_patterns(&self, request: &SecurityValidationRequest) -> Option<RiskFactor> {
        // In a real implementation, this would analyze transaction history
        // For now, we'll simulate some basic checks
        
        if request.amount > 50000 {
            return Some(RiskFactor {
                factor_type: RiskFactorType::HighVolume,
                description: "High transaction volume detected".to_string(),
                impact_score: 20,
                mitigation: Some("Consider additional verification".to_string()),
            });
        }
        
        None
    }

    /// Check geographic risk
    async fn check_geographic_risk(&self, request: &SecurityValidationRequest) -> Option<RiskFactor> {
        // In a real implementation, this would check IP geolocation
        if let Some(ip) = &request.ip_address {
            // Simulate some high-risk countries
            if ip.starts_with("192.168.") || ip.starts_with("10.") {
                return Some(RiskFactor {
                    factor_type: RiskFactorType::IpRisk,
                    description: "Private IP address detected".to_string(),
                    impact_score: 10,
                    mitigation: Some("Use public IP address".to_string()),
                });
            }
        }
        
        None
    }

    /// Check device risk
    async fn check_device_risk(&self, request: &SecurityValidationRequest) -> Option<RiskFactor> {
        // In a real implementation, this would analyze device fingerprint
        if let Some(fingerprint) = &request.device_fingerprint {
            if fingerprint.len() < 10 {
                return Some(RiskFactor {
                    factor_type: RiskFactorType::DeviceRisk,
                    description: "Suspicious device fingerprint".to_string(),
                    impact_score: 15,
                    mitigation: Some("Update device information".to_string()),
                });
            }
        }
        
        None
    }

    /// Determine risk level based on score
    fn determine_risk_level(&self, score: u8) -> RiskLevel {
        match score {
            0..=25 => RiskLevel::Low,
            26..=50 => RiskLevel::Medium,
            51..=75 => RiskLevel::High,
            76..=100 => RiskLevel::Critical,
            _ => RiskLevel::Critical, // Handle any unexpected values
        }
    }

    /// Calculate age from date of birth
    fn calculate_age(&self, date_of_birth: DateTime<Utc>) -> u8 {
        let now = Utc::now();
        let age = now.year() - date_of_birth.year();
        if now.month() < date_of_birth.month() || 
           (now.month() == date_of_birth.month() && now.day() < date_of_birth.day()) {
            (age - 1) as u8
        } else {
            age as u8
        }
    }

    /// Get risk assessment for user
    pub fn get_risk_assessment(&self, user_id: &str) -> Option<&RiskAssessment> {
        self.risk_assessments.get(user_id)
    }

    /// Update security rules
    pub fn update_security_rules(&mut self, new_rules: SecurityRules) {
        self.security_rules = new_rules;
    }

    /// Update KYC requirements
    pub fn update_kyc_requirements(&mut self, new_requirements: KycRequirements) {
        self.kyc_requirements = new_requirements;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenomics_config::TokenomicsConfig;

    #[tokio::test]
    async fn test_security_validation() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let mut validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

        let request = SecurityValidationRequest {
            user_id: "user_001".to_string(),
            transaction_type: TransactionType::UtEarning,
            amount: 1000,
            ip_address: Some("192.168.1.1".to_string()),
            device_fingerprint: Some("device123".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            timestamp: Utc::now(),
        };

        let response = validator.validate_security(request).await;
        assert!(response.is_valid);
        assert_eq!(response.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_kyc_validation() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

        let request = KycValidationRequest {
            user_id: "user_001".to_string(),
            full_name: "John Doe".to_string(),
            date_of_birth: Utc::now() - chrono::Duration::days(365 * 25), // 25 years old
            nationality: "GE".to_string(),
            address: "Tbilisi, Georgia".to_string(),
            phone: "+995 555 123 456".to_string(),
            email: "john@example.com".to_string(),
            documents: vec![
                DocumentInfo {
                    document_type: DocumentType::Passport,
                    document_number: "123456789".to_string(),
                    issuing_country: "GE".to_string(),
                    expiry_date: Some(Utc::now() + chrono::Duration::days(365)),
                    document_hash: "hash123".to_string(),
                }
            ],
        };

        let response = validator.validate_kyc(request).await;
        assert!(response.is_valid);
        assert_eq!(response.kyc_status, KycStatus::Pending);
    }
}
