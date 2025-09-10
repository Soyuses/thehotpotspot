use blockchain_project::config::*;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_currency_config_creation() {
        let config = CurrencyConfig::new();
        assert_eq!(config.base_currency, "USD");
        assert_eq!(config.decimal_places, 2);
        assert!(config.supported_currencies.contains(&"USD".to_string()));
    }

    #[test]
    fn test_currency_conversion() {
        let config = CurrencyConfig::new();
        
        // Test USD to USD conversion (should be 1:1)
        let usd_amount = config.convert_currency(100.0, "USD", "USD").unwrap();
        assert_eq!(usd_amount, 100.0);
        
        // Test with unsupported currency
        let result = config.convert_currency(100.0, "EUR", "USD");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation() {
        let mut config = CurrencyConfig::new();
        
        // Test valid currency
        assert!(config.is_currency_supported("USD"));
        
        // Test invalid currency
        assert!(!config.is_currency_supported("INVALID"));
    }

    #[test]
    fn test_config_serialization() {
        let config = CurrencyConfig::new();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CurrencyConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.base_currency, deserialized.base_currency);
        assert_eq!(config.decimal_places, deserialized.decimal_places);
    }
}
