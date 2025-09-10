use blockchain_project::kyc_aml::*;

#[cfg(test)]
mod kyc_aml_tests {
    use super::*;

    #[test]
    fn test_kyc_aml_manager_creation() {
        let manager = KYCAmlManager::new();
        assert!(manager.users.is_empty());
        assert!(manager.roles.is_empty());
        assert_eq!(manager.user_count, 0);
    }

    #[test]
    fn test_kyc_user_creation() {
        let user = KYCUser::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "+1234567890".to_string()
        );
        
        assert_eq!(user.user_id, "user123");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.phone_number, "+1234567890");
        assert!(matches!(user.kyc_status, KYCStatus::Pending));
        assert!(matches!(user.kyc_level, KYCLevel::Basic));
        assert!(user.documents.is_empty());
    }

    #[test]
    fn test_kyc_status_enum() {
        let pending_status = KYCStatus::Pending;
        let verified_status = KYCStatus::Verified;
        let rejected_status = KYCStatus::Rejected;
        let expired_status = KYCStatus::Expired;
        
        assert!(matches!(pending_status, KYCStatus::Pending));
        assert!(matches!(verified_status, KYCStatus::Verified));
        assert!(matches!(rejected_status, KYCStatus::Rejected));
        assert!(matches!(expired_status, KYCStatus::Expired));
    }

    #[test]
    fn test_kyc_level_enum() {
        let basic_level = KYCLevel::Basic;
        let intermediate_level = KYCLevel::Intermediate;
        let advanced_level = KYCLevel::Advanced;
        
        assert!(matches!(basic_level, KYCLevel::Basic));
        assert!(matches!(intermediate_level, KYCLevel::Intermediate));
        assert!(matches!(advanced_level, KYCLevel::Advanced));
    }

    #[test]
    fn test_document_type_enum() {
        let passport = DocumentType::Passport;
        let driver_license = DocumentType::DriverLicense;
        let national_id = DocumentType::NationalId;
        let utility_bill = DocumentType::UtilityBill;
        
        assert!(matches!(passport, DocumentType::Passport));
        assert!(matches!(driver_license, DocumentType::DriverLicense));
        assert!(matches!(national_id, DocumentType::NationalId));
        assert!(matches!(utility_bill, DocumentType::UtilityBill));
    }

    #[test]
    fn test_document_status_enum() {
        let pending = DocumentStatus::Pending;
        let verified = DocumentStatus::Verified;
        let rejected = DocumentStatus::Rejected;
        
        assert!(matches!(pending, DocumentStatus::Pending));
        assert!(matches!(verified, DocumentStatus::Verified));
        assert!(matches!(rejected, DocumentStatus::Rejected));
    }

    #[test]
    fn test_user_role_creation() {
        let role = UserRole::new(
            "customer".to_string(),
            "Customer role".to_string()
        );
        
        assert_eq!(role.role_id, "customer");
        assert_eq!(role.name, "Customer role");
        assert!(role.permissions.is_empty());
    }

    #[test]
    fn test_permission_enum() {
        let read_permission = Permission::Read;
        let write_permission = Permission::Write;
        let admin_permission = Permission::Admin;
        
        assert!(matches!(read_permission, Permission::Read));
        assert!(matches!(write_permission, Permission::Write));
        assert!(matches!(admin_permission, Permission::Admin));
    }

    #[test]
    fn test_role_enum() {
        let customer_role = Role::Customer;
        let franchise_role = Role::Franchise;
        let owner_role = Role::Owner;
        let admin_role = Role::Admin;
        
        assert!(matches!(customer_role, Role::Customer));
        assert!(matches!(franchise_role, Role::Franchise));
        assert!(matches!(owner_role, Role::Owner));
        assert!(matches!(admin_role, Role::Admin));
    }

    #[test]
    fn test_user_role_assignment() {
        let assignment = UserRoleAssignment::new(
            "user123".to_string(),
            Role::Customer
        );
        
        assert_eq!(assignment.user_id, "user123");
        assert!(matches!(assignment.role, Role::Customer));
        assert!(assignment.assigned_at > 0);
    }

    #[test]
    fn test_kyc_statistics_creation() {
        let stats = KYCStatistics::new();
        
        assert_eq!(stats.total_users, 0);
        assert_eq!(stats.verified_users, 0);
        assert_eq!(stats.pending_users, 0);
        assert_eq!(stats.rejected_users, 0);
        assert_eq!(stats.expired_users, 0);
    }

    #[test]
    fn test_kyc_aml_error_types() {
        let user_not_found = KYCAmlError::UserNotFound("user123".to_string());
        let document_not_found = KYCAmlError::DocumentNotFound("doc123".to_string());
        let verification_failed = KYCAmlError::VerificationFailed("Verification failed".to_string());
        let invalid_document = KYCAmlError::InvalidDocument("Invalid document".to_string());
        
        match user_not_found {
            KYCAmlError::UserNotFound(id) => assert_eq!(id, "user123"),
            _ => panic!("Wrong error type"),
        }
        
        match document_not_found {
            KYCAmlError::DocumentNotFound(id) => assert_eq!(id, "doc123"),
            _ => panic!("Wrong error type"),
        }
        
        match verification_failed {
            KYCAmlError::VerificationFailed(msg) => assert_eq!(msg, "Verification failed"),
            _ => panic!("Wrong error type"),
        }
        
        match invalid_document {
            KYCAmlError::InvalidDocument(msg) => assert_eq!(msg, "Invalid document"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_user_registration_data() {
        let registration_data = UserRegistrationData::new(
            "john@example.com".to_string(),
            "+1234567890".to_string(),
            "John Doe".to_string()
        );
        
        assert_eq!(registration_data.email, "john@example.com");
        assert_eq!(registration_data.phone_number, "+1234567890");
        assert_eq!(registration_data.full_name, "John Doe");
    }

    #[test]
    fn test_address_creation() {
        let address = Address::new(
            "123 Main St".to_string(),
            "New York".to_string(),
            "NY".to_string(),
            "10001".to_string(),
            "USA".to_string()
        );
        
        assert_eq!(address.street, "123 Main St");
        assert_eq!(address.city, "New York");
        assert_eq!(address.state, "NY");
        assert_eq!(address.postal_code, "10001");
        assert_eq!(address.country, "USA");
    }

    #[test]
    fn test_audit_log_entry() {
        let audit_entry = AuditLogEntry::new(
            "user123".to_string(),
            "KYC verification completed".to_string(),
            "admin".to_string()
        );
        
        assert_eq!(audit_entry.user_id, "user123");
        assert_eq!(audit_entry.action, "KYC verification completed");
        assert_eq!(audit_entry.performed_by, "admin");
        assert!(audit_entry.timestamp > 0);
    }

    #[test]
    fn test_kyc_aml_manager_operations() {
        let mut manager = KYCAmlManager::new();
        
        // Create a user
        let user = KYCUser::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "+1234567890".to_string()
        );
        let user_id = user.user_id.clone();
        
        // Add user to manager
        manager.add_user(user).unwrap();
        assert_eq!(manager.user_count, 1);
        assert!(manager.users.contains_key(&user_id));
        
        // Get user from manager
        let retrieved_user = manager.get_user(&user_id).unwrap();
        assert_eq!(retrieved_user.email, "john@example.com");
        
        // Remove user from manager
        manager.remove_user(&user_id).unwrap();
        assert_eq!(manager.user_count, 0);
        assert!(!manager.users.contains_key(&user_id));
    }

    #[test]
    fn test_kyc_verification_process() {
        let mut user = KYCUser::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "+1234567890".to_string()
        );
        
        // Start verification
        user.start_verification().unwrap();
        assert!(matches!(user.kyc_status, KYCStatus::Pending));
        
        // Complete verification
        user.complete_verification(KYCLevel::Intermediate).unwrap();
        assert!(matches!(user.kyc_status, KYCStatus::Verified));
        assert!(matches!(user.kyc_level, KYCLevel::Intermediate));
    }

    #[test]
    fn test_document_management() {
        let mut user = KYCUser::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "+1234567890".to_string()
        );
        
        // Add document
        let document_id = user.add_document(
            DocumentType::Passport,
            "passport_data".to_string()
        ).unwrap();
        
        assert_eq!(user.documents.len(), 1);
        assert!(user.documents.contains_key(&document_id));
        
        // Verify document
        user.verify_document(&document_id, DocumentStatus::Verified).unwrap();
        let document = user.documents.get(&document_id).unwrap();
        assert!(matches!(document.status, DocumentStatus::Verified));
    }

    #[test]
    fn test_serialization() {
        let user = KYCUser::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "+1234567890".to_string()
        );
        
        let json = serde_json::to_string(&user).unwrap();
        let deserialized: KYCUser = serde_json::from_str(&json).unwrap();
        
        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.email, deserialized.email);
        assert_eq!(user.phone_number, deserialized.phone_number);
    }
}
