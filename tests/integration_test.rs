use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use blockchain_project::{
    tokenomics_config::TokenomicsConfig,
    new_tokenomics::{NewTokenomicsManager, SaleRecord, UtEvent, UtEventType, SaleStatus},
    security_checks::{SecurityValidator, SecurityValidationRequest, KycValidationRequest, TransactionType, DocumentType, DocumentInfo},
    governance_dao::{GovernanceDAO, CreateProposalRequest, VoteRequest, ProposalType, VoteChoice},
    viewer_arm::{ViewerARM, ViewerLoginRequest, KYCRegistrationRequest},
    kyc_aml::KYCAmlManager,
};

#[tokio::test]
async fn test_complete_system_flow() {
    println!("🧪 Testing Complete System Flow");
    
    // Initialize all components
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let kyc_manager = Arc::new(RwLock::new(KYCAmlManager::new()));
    let security_validator = Arc::new(RwLock::new(SecurityValidator::new(tokenomics_manager.clone(), tokenomics_config.clone())));
    let mut dao = GovernanceDAO::new(tokenomics_manager.clone(), tokenomics_config.clone());
    let mut viewer_arm = ViewerARM::new(tokenomics_config.clone(), kyc_manager.read().await.clone(), vec![]);

    // Test 1: POS Sale and ST Minting
    println!("📊 Test 1: POS Sale and ST Minting");
    let sale_record = SaleRecord {
        sale_id: "test_sale_001".to_string(),
        node_id: "node_tbilisi_001".to_string(),
        user_id: Some("user_001".to_string()),
        amount_gel: 25.0,
        st_units: 500, // 25.0 * 20 (1 GEL = 0.2 THP)
        check_address: "0xcheck1234567890abcdef1234567890abcdef123456".to_string(),
        activation_code_hash: "123456".to_string(),
        timestamp: Utc::now(),
        status: SaleStatus::Pending,
    };

    {
        let mut manager = tokenomics_manager.write().await;
        assert!(manager.add_sale(sale_record).is_ok());
    }

    // Test 2: UT Event Processing
    println!("🎥 Test 2: UT Event Processing");
    let ut_event = UtEvent {
        event_id: "event_001".to_string(),
        user_id: "user_001".to_string(),
        event_type: UtEventType::Streaming,
        units: 300, // 30 minutes * 10 UT per minute
        timestamp: Utc::now(),
        reference: "stream_001".to_string(),
        platform: "twitch".to_string(),
    };

    {
        let mut manager = tokenomics_manager.write().await;
        assert!(manager.add_ut_event(ut_event).is_ok());
    }

    // Test 3: Security Validation
    println!("🔒 Test 3: Security Validation");
    let security_request = SecurityValidationRequest {
        user_id: "user_001".to_string(),
        transaction_type: TransactionType::UtEarning,
        amount: 1000,
        ip_address: Some("203.0.113.1".to_string()),
        device_fingerprint: Some("device_fingerprint_123456789".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        timestamp: Utc::now(),
    };

    {
        let mut validator = security_validator.write().await;
        let response = validator.validate_security(security_request).await;
        assert!(response.is_valid);
        assert_eq!(response.risk_level, blockchain_project::security_checks::RiskLevel::Low);
    }

    // Test 4: KYC Validation
    println!("🔐 Test 4: KYC Validation");
    let kyc_request = KycValidationRequest {
        user_id: "user_001".to_string(),
        full_name: "John Doe".to_string(),
        date_of_birth: Utc::now() - chrono::Duration::days(365 * 25), // 25 years old
        nationality: "GE".to_string(),
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
            }
        ],
    };

    {
        let validator = security_validator.read().await;
        let response = validator.validate_kyc(kyc_request).await;
        assert!(response.is_valid);
        assert_eq!(response.kyc_status, blockchain_project::new_tokenomics::KycStatus::Pending);
    }

    // Test 5: DAO Proposal Creation
    println!("🗳️ Test 5: DAO Proposal Creation");
    let proposal_request = CreateProposalRequest {
        creator: "user_001".to_string(),
        title: "Test Proposal".to_string(),
        description: "This is a test proposal".to_string(),
        proposal_type: ProposalType::General { category: "test".to_string() },
        voting_duration_hours: 24,
    };

    let proposal_response = dao.create_proposal(proposal_request).await;
    assert!(proposal_response.success);
    let proposal_id = proposal_response.proposal_id.expect("Proposal ID should be present");
    assert!(!proposal_id.is_empty());

    // Test 6: DAO Voting
    println!("🗳️ Test 6: DAO Voting");
    let vote_request = VoteRequest {
        voter: "user_001".to_string(),
        proposal_id: proposal_id.clone(),
        choice: VoteChoice::Yes,
    };

    let vote_response = dao.cast_vote(vote_request).await;
    assert!(vote_response.success);

    // Test 7: Viewer ARM Login
    println!("🎮 Test 7: Viewer ARM Login");
    let login_request = ViewerLoginRequest {
        nickname: "test_user".to_string(),
        platform: "twitch".to_string(),
        phone: Some("+995 555 123 456".to_string()),
    };

    let login_response = viewer_arm.login_viewer(login_request).await;
    assert!(login_response.success);
    assert!(login_response.success);

    // Test 8: Conversion Round
    println!("🔄 Test 8: Conversion Round");
    {
        let mut manager = tokenomics_manager.write().await;
        let conversion_round = manager.trigger_conversion_round().unwrap();
        assert!(!conversion_round.round_id.is_empty());
        assert!(conversion_round.total_pool > 0);
    }

    // Test 9: Statistics
    println!("📊 Test 9: Statistics");
    {
        let manager = tokenomics_manager.read().await;
        let stats = manager.get_statistics().unwrap();
        assert!(stats.total_sales > 0);
        assert!(stats.total_ut_events > 0);
        assert!(stats.total_st_minted > 0);
        assert!(stats.total_ut_awarded > 0);
    }

    println!("✅ All tests passed! System is working correctly.");
}

#[tokio::test]
async fn test_tokenomics_consistency() {
    println!("🧪 Testing Tokenomics Consistency");
    
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));

    // Test ST emission rate
    let gel_amount = 25.0;
    let expected_st = tokenomics_config.calculate_st_tokens(gel_amount);
    assert_eq!(expected_st, 500); // 25.0 * 20 = 500

    // Test UT earning rates
    assert_eq!(tokenomics_config.utility_token.ut_per_minute, 10);
    assert_eq!(tokenomics_config.utility_token.ut_per_comment, 5);
    assert_eq!(tokenomics_config.utility_token.ut_per_share, 20);
    assert_eq!(tokenomics_config.utility_token.ut_per_like, 2);

    // Test conversion pool
    let reserved_st = 1000;
    let pool_size = tokenomics_config.get_conversion_pool_size(reserved_st);
    assert_eq!(pool_size, 500); // 50% of 1000

    println!("✅ Tokenomics consistency tests passed!");
}

#[tokio::test]
async fn test_security_validation() {
    println!("🧪 Testing Security Validation");
    
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let mut security_validator = SecurityValidator::new(tokenomics_manager, tokenomics_config);

    // Test normal transaction
    let normal_request = SecurityValidationRequest {
        user_id: "user_001".to_string(),
        transaction_type: TransactionType::UtEarning,
        amount: 1000,
        ip_address: Some("203.0.113.1".to_string()),
        device_fingerprint: Some("device_fingerprint_123456789".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        timestamp: Utc::now(),
    };

    let response = security_validator.validate_security(normal_request).await;
    assert!(response.is_valid);
    assert_eq!(response.risk_level, blockchain_project::security_checks::RiskLevel::Low);

    // Test high-risk transaction
    let high_risk_request = SecurityValidationRequest {
        user_id: "user_002".to_string(),
        transaction_type: TransactionType::StMinting,
        amount: 100000, // High amount
        ip_address: Some("192.168.1.100".to_string()), // Private IP
        device_fingerprint: Some("short".to_string()), // Suspicious fingerprint
        user_agent: Some("Mozilla/5.0".to_string()),
        timestamp: Utc::now(),
    };

    let response = security_validator.validate_security(high_risk_request).await;
    assert!(response.risk_score > 0);
    assert!(response.risk_level != blockchain_project::security_checks::RiskLevel::Low);

    println!("✅ Security validation tests passed!");
}

#[tokio::test]
async fn test_dao_governance() {
    println!("🧪 Testing DAO Governance");
    
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let mut dao = GovernanceDAO::new(tokenomics_manager, tokenomics_config);

    // Create proposal
    let proposal_request = CreateProposalRequest {
        creator: "user_001".to_string(),
        title: "Test Proposal".to_string(),
        description: "This is a test proposal".to_string(),
        proposal_type: ProposalType::General { category: "test".to_string() },
        voting_duration_hours: 24,
    };

    let proposal_response = dao.create_proposal(proposal_request).await;
    assert!(proposal_response.success);
    let proposal_id = proposal_response.proposal_id.expect("Proposal ID should be present");
    assert!(!proposal_id.is_empty());

    // Vote on proposal
    let vote_request = VoteRequest {
        voter: "user_001".to_string(),
        proposal_id: proposal_id.clone(),
        choice: VoteChoice::Yes,
    };

    let vote_response = dao.cast_vote(vote_request).await;
    assert!(vote_response.success);

    // Check proposal status
    let proposal = dao.get_proposal(&proposal_id).unwrap();
    assert_eq!(proposal.status, blockchain_project::governance_dao::ProposalStatus::Active);

    println!("✅ DAO governance tests passed!");
}

#[tokio::test]
async fn test_viewer_arm() {
    println!("🧪 Testing Viewer ARM");
    
    let tokenomics_config = TokenomicsConfig::default();
    let tokenomics_manager = Arc::new(RwLock::new(NewTokenomicsManager::new(tokenomics_config.clone())));
    let kyc_manager = Arc::new(RwLock::new(KYCAmlManager::new()));
    let mut viewer_arm = ViewerARM::new(tokenomics_config.clone(), kyc_manager.read().await.clone(), vec![]);

    // Test login
    let login_request = ViewerLoginRequest {
        nickname: "test_user".to_string(),
        platform: "twitch".to_string(),
        phone: Some("+995 555 123 456".to_string()),
    };

    let login_response = viewer_arm.login_viewer(login_request).await;
    assert!(login_response.success);
    assert!(login_response.success);

    // Test registration
    let registration_request = KYCRegistrationRequest {
        session_id: "session_001".to_string(),
        full_name: "John Doe".to_string(),
        phone: "+995 555 123 456".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "SecurePassword123!".to_string(),
        tshirt_size: "L".to_string(),
        favorite_dish: "Plov".to_string(),
        qr_code: Some("qr123456789".to_string()),
    };

    let registration_response = viewer_arm.register_for_kyc(registration_request).await;
    assert!(registration_response.success);
    assert!(registration_response.success);

    // Test streaming activity
    use blockchain_project::viewer_arm::UTActivityRequest;
    let streaming_request = UTActivityRequest {
        session_id: "session_001".to_string(),
        reference: "stream_001".to_string(),
        count: Some(30),
        activity_type: "streaming".to_string(),
        duration_minutes: Some(30),
    };

    let streaming_response = viewer_arm.record_ut_activity(streaming_request).await;
    assert!(streaming_response.success);
    assert!(streaming_response.success);
    assert!(streaming_response.ut_earned > Some(0));

    println!("✅ Viewer ARM tests passed!");
}
