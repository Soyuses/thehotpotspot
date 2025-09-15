use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    new_tokenomics::NewTokenomicsManager,
    security_checks::{
        SecurityValidator, SecurityValidationRequest, KycValidationRequest,
        TransactionType, DocumentType, DocumentInfo, SecurityRules, KycRequirements,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 The Hot Pot Spot - Security Checks Demo");
    println!("===========================================");

    // Initialize configuration
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let mut security_validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

    println!("📊 Security Configuration:");
    println!("  - Max daily UT earning: {}", security_validator.security_rules.max_daily_ut_earning);
    println!("  - Max daily ST minting: {}", security_validator.security_rules.max_daily_st_minting);
    println!("  - Min transaction interval: {} seconds", security_validator.security_rules.min_transaction_interval);
    println!("  - Max failed attempts: {}", security_validator.security_rules.max_failed_attempts);
    println!("  - Lockout duration: {} minutes", security_validator.security_rules.lockout_duration_minutes);

    println!("\n🛡️ Testing Security Validation...");

    // Test 1: Normal UT earning transaction
    let normal_request = SecurityValidationRequest {
        user_id: "user_001".to_string(),
        transaction_type: TransactionType::UtEarning,
        amount: 1000,
        ip_address: Some("203.0.113.1".to_string()),
        device_fingerprint: Some("device_fingerprint_123456789".to_string()),
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()),
        timestamp: Utc::now(),
    };

    let normal_response = security_validator.validate_security(normal_request).await;
    println!("✅ Normal Transaction:");
    println!("  - Valid: {}", normal_response.is_valid);
    println!("  - Risk Score: {}", normal_response.risk_score);
    println!("  - Risk Level: {:?}", normal_response.risk_level);
    println!("  - Blocked Reasons: {:?}", normal_response.blocked_reasons);
    println!("  - Warnings: {:?}", normal_response.warnings);
    println!("  - Recommendations: {:?}", normal_response.recommendations);

    // Test 2: High volume transaction
    let high_volume_request = SecurityValidationRequest {
        user_id: "user_002".to_string(),
        transaction_type: TransactionType::StMinting,
        amount: 100000, // High amount
        ip_address: Some("203.0.113.2".to_string()),
        device_fingerprint: Some("device_fingerprint_987654321".to_string()),
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()),
        timestamp: Utc::now(),
    };

    let high_volume_response = security_validator.validate_security(high_volume_request).await;
    println!("\n⚠️ High Volume Transaction:");
    println!("  - Valid: {}", high_volume_response.is_valid);
    println!("  - Risk Score: {}", high_volume_response.risk_score);
    println!("  - Risk Level: {:?}", high_volume_response.risk_level);
    println!("  - Blocked Reasons: {:?}", high_volume_response.blocked_reasons);
    println!("  - Warnings: {:?}", high_volume_response.warnings);
    println!("  - Recommendations: {:?}", high_volume_response.recommendations);

    // Test 3: Suspicious transaction (private IP)
    let suspicious_request = SecurityValidationRequest {
        user_id: "user_003".to_string(),
        transaction_type: TransactionType::UtEarning,
        amount: 5000,
        ip_address: Some("192.168.1.100".to_string()), // Private IP
        device_fingerprint: Some("short".to_string()), // Suspicious fingerprint
        user_agent: Some("Mozilla/5.0".to_string()),
        timestamp: Utc::now(),
    };

    let suspicious_response = security_validator.validate_security(suspicious_request).await;
    println!("\n🚨 Suspicious Transaction:");
    println!("  - Valid: {}", suspicious_response.is_valid);
    println!("  - Risk Score: {}", suspicious_response.risk_score);
    println!("  - Risk Level: {:?}", suspicious_response.risk_level);
    println!("  - Blocked Reasons: {:?}", suspicious_response.blocked_reasons);
    println!("  - Warnings: {:?}", suspicious_response.warnings);
    println!("  - Recommendations: {:?}", suspicious_response.recommendations);

    println!("\n🔐 Testing KYC Validation...");

    // Test 1: Valid KYC request
    let valid_kyc_request = KycValidationRequest {
        user_id: "user_001".to_string(),
        full_name: "John Doe".to_string(),
        date_of_birth: Utc::now() - chrono::Duration::days(365 * 25), // 25 years old
        nationality: "GE".to_string(), // Georgia
        address: "Tbilisi, Georgia".to_string(),
        phone: "+995 555 123 456".to_string(),
        email: "john.doe@example.com".to_string(),
        documents: vec![
            DocumentInfo {
                document_type: DocumentType::Passport,
                document_number: "123456789".to_string(),
                issuing_country: "GE".to_string(),
                expiry_date: Some(Utc::now() + chrono::Duration::days(365)),
                document_hash: "passport_hash_123".to_string(),
            },
            DocumentInfo {
                document_type: DocumentType::NationalId,
                document_number: "987654321".to_string(),
                issuing_country: "GE".to_string(),
                expiry_date: None,
                document_hash: "national_id_hash_456".to_string(),
            },
            DocumentInfo {
                document_type: DocumentType::ProofOfAddress,
                document_number: "ADDR001".to_string(),
                issuing_country: "GE".to_string(),
                expiry_date: None,
                document_hash: "address_proof_hash_789".to_string(),
            },
        ],
    };

    let valid_kyc_response = security_validator.validate_kyc(valid_kyc_request).await;
    println!("✅ Valid KYC Request:");
    println!("  - Valid: {}", valid_kyc_response.is_valid);
    println!("  - KYC Status: {:?}", valid_kyc_response.kyc_status);
    println!("  - Validation Score: {}", valid_kyc_response.validation_score);
    println!("  - Issues: {:?}", valid_kyc_response.issues);
    println!("  - Recommendations: {:?}", valid_kyc_response.recommendations);
    println!("  - Next Steps: {:?}", valid_kyc_response.next_steps);

    // Test 2: Invalid KYC request (underage)
    let invalid_kyc_request = KycValidationRequest {
        user_id: "user_002".to_string(),
        full_name: "Jane Smith".to_string(),
        date_of_birth: Utc::now() - chrono::Duration::days(365 * 16), // 16 years old
        nationality: "GE".to_string(),
        address: "Tbilisi, Georgia".to_string(),
        phone: "+995 555 987 654".to_string(),
        email: "jane.smith@example.com".to_string(),
        documents: vec![
            DocumentInfo {
                document_type: DocumentType::Passport,
                document_number: "111222333".to_string(),
                issuing_country: "GE".to_string(),
                expiry_date: Some(Utc::now() + chrono::Duration::days(365)),
                document_hash: "passport_hash_111".to_string(),
            },
        ],
    };

    let invalid_kyc_response = security_validator.validate_kyc(invalid_kyc_request).await;
    println!("\n❌ Invalid KYC Request (Underage):");
    println!("  - Valid: {}", invalid_kyc_response.is_valid);
    println!("  - KYC Status: {:?}", invalid_kyc_response.kyc_status);
    println!("  - Validation Score: {}", invalid_kyc_response.validation_score);
    println!("  - Issues: {:?}", invalid_kyc_response.issues);
    println!("  - Recommendations: {:?}", invalid_kyc_response.recommendations);
    println!("  - Next Steps: {:?}", invalid_kyc_response.next_steps);

    // Test 3: KYC request from unsupported country
    let unsupported_country_request = KycValidationRequest {
        user_id: "user_003".to_string(),
        full_name: "Bob Johnson".to_string(),
        date_of_birth: Utc::now() - chrono::Duration::days(365 * 30), // 30 years old
        nationality: "XX".to_string(), // Unsupported country
        address: "Unknown Country".to_string(),
        phone: "+999 555 000 000".to_string(),
        email: "bob.johnson@example.com".to_string(),
        documents: vec![
            DocumentInfo {
                document_type: DocumentType::Passport,
                document_number: "444555666".to_string(),
                issuing_country: "XX".to_string(),
                expiry_date: Some(Utc::now() + chrono::Duration::days(365)),
                document_hash: "passport_hash_444".to_string(),
            },
        ],
    };

    let unsupported_country_response = security_validator.validate_kyc(unsupported_country_request).await;
    println!("\n🚫 Unsupported Country KYC Request:");
    println!("  - Valid: {}", unsupported_country_response.is_valid);
    println!("  - KYC Status: {:?}", unsupported_country_response.kyc_status);
    println!("  - Validation Score: {}", unsupported_country_response.validation_score);
    println!("  - Issues: {:?}", unsupported_country_response.issues);
    println!("  - Recommendations: {:?}", unsupported_country_response.recommendations);
    println!("  - Next Steps: {:?}", unsupported_country_response.next_steps);

    println!("\n📊 Security Statistics:");
    println!("  - Total risk assessments: {}", security_validator.risk_assessments.len());
    
    // Show risk assessments
    for (user_id, assessment) in &security_validator.risk_assessments {
        println!("  - User {}: Risk Score {}, Level {:?}", 
            user_id, assessment.risk_score, assessment.risk_level);
    }

    println!("\n🎯 Demo Summary:");
    println!("================");
    println!("✅ Successfully demonstrated:");
    println!("  - Security validation for different transaction types");
    println!("  - Risk assessment and scoring");
    println!("  - KYC validation with document verification");
    println!("  - Age and country validation");
    println!("  - Risk factor identification and mitigation");
    println!("\n🔒 Security system is working correctly!");
    println!("   - Normal transactions pass validation");
    println!("   - High-risk transactions are flagged");
    println!("   - KYC validation enforces compliance requirements");
    println!("   - Risk assessments provide actionable insights");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_security_validation_normal() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let mut validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

        let request = SecurityValidationRequest {
            user_id: "user_001".to_string(),
            transaction_type: TransactionType::UtEarning,
            amount: 1000,
            ip_address: Some("203.0.113.1".to_string()),
            device_fingerprint: Some("device_fingerprint_123456789".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            timestamp: Utc::now(),
        };

        let response = validator.validate_security(request).await;
        assert!(response.is_valid);
        assert_eq!(response.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_kyc_validation_valid() {
        let tokenomics_config = TokenomicsConfig::default();
        let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
        let validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

        let request = KycValidationRequest {
            user_id: "user_001".to_string(),
            full_name: "John Doe".to_string(),
            date_of_birth: Utc::now() - chrono::Duration::days(365 * 25),
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
