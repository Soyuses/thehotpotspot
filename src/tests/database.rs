use blockchain_project::database::*;

#[cfg(test)]
mod database_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_database_manager_creation() {
        let config = DatabaseConfig::new();
        let manager = DatabaseManager::new(config);
        
        assert_eq!(manager.config.max_connections, 10);
        assert_eq!(manager.config.connection_timeout, 30);
        assert!(manager.connections.is_empty());
    }

    #[test]
    fn test_database_config_validation() {
        let config = DatabaseConfig::new();
        
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
        assert!(!config.database_name.is_empty());
        assert!(config.max_connections > 0);
        assert!(config.connection_timeout > 0);
    }

    #[test]
    fn test_user_data_creation() {
        let user_data = UserData::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "0xwallet123".to_string()
        );
        
        assert_eq!(user_data.user_id, "user123");
        assert_eq!(user_data.email, "john@example.com");
        assert_eq!(user_data.wallet_address, "0xwallet123");
        assert!(user_data.created_at > 0);
    }

    #[test]
    fn test_database_stats_initialization() {
        let stats = DatabaseStats::new();
        
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.failed_queries, 0);
        assert_eq!(stats.avg_query_time, 0.0);
    }

    #[test]
    fn test_cleanup_stats_initialization() {
        let cleanup_stats = CleanupStats::new();
        
        assert_eq!(cleanup_stats.records_deleted, 0);
        assert_eq!(cleanup_stats.tables_cleaned, 0);
        assert_eq!(cleanup_stats.cleanup_duration, 0);
    }

    #[test]
    fn test_database_error_types() {
        let connection_error = DatabaseError::ConnectionFailed("Connection timeout".to_string());
        let query_error = DatabaseError::QueryFailed("Invalid SQL".to_string());
        let config_error = DatabaseError::ConfigurationError("Invalid config".to_string());
        
        match connection_error {
            DatabaseError::ConnectionFailed(msg) => assert_eq!(msg, "Connection timeout"),
            _ => panic!("Wrong error type"),
        }
        
        match query_error {
            DatabaseError::QueryFailed(msg) => assert_eq!(msg, "Invalid SQL"),
            _ => panic!("Wrong error type"),
        }
        
        match config_error {
            DatabaseError::ConfigurationError(msg) => assert_eq!(msg, "Invalid config"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_user_data_serialization() {
        let user_data = UserData::new(
            "user123".to_string(),
            "john@example.com".to_string(),
            "0xwallet123".to_string()
        );
        
        let json = serde_json::to_string(&user_data).unwrap();
        let deserialized: UserData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(user_data.user_id, deserialized.user_id);
        assert_eq!(user_data.email, deserialized.email);
        assert_eq!(user_data.wallet_address, deserialized.wallet_address);
    }

    #[test]
    fn test_database_config_serialization() {
        let config = DatabaseConfig::new();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DatabaseConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.host, deserialized.host);
        assert_eq!(config.port, deserialized.port);
        assert_eq!(config.database_name, deserialized.database_name);
        assert_eq!(config.max_connections, deserialized.max_connections);
    }

    #[test]
    fn test_database_stats_updates() {
        let mut stats = DatabaseStats::new();
        
        // Simulate some database operations
        stats.total_connections = 5;
        stats.active_connections = 2;
        stats.total_queries = 100;
        stats.failed_queries = 5;
        stats.avg_query_time = 0.05;
        
        assert_eq!(stats.total_connections, 5);
        assert_eq!(stats.active_connections, 2);
        assert_eq!(stats.total_queries, 100);
        assert_eq!(stats.failed_queries, 5);
        assert_eq!(stats.avg_query_time, 0.05);
    }

    #[test]
    fn test_cleanup_stats_updates() {
        let mut cleanup_stats = CleanupStats::new();
        
        // Simulate cleanup operations
        cleanup_stats.records_deleted = 1000;
        cleanup_stats.tables_cleaned = 5;
        cleanup_stats.cleanup_duration = 30;
        
        assert_eq!(cleanup_stats.records_deleted, 1000);
        assert_eq!(cleanup_stats.tables_cleaned, 5);
        assert_eq!(cleanup_stats.cleanup_duration, 30);
    }
}
