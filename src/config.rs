//! Конфигурация проекта The Hot Pot Spot
//! 
//! Содержит все константы, связанные с валютой, токеномикой и регуляторными требованиями

/// Основная валюта проекта - Грузинский лари (GEL)
pub const CURRENCY: &str = "GEL";

/// Начальная цена токена THP в грузинских лари
/// 5.00 GEL = 500 subunits
pub const INITIAL_THP_PRICE_GEL: u128 = 500;

/// Масштаб для subunits (1 токен = 100 subunits)
/// Используется для точных расчетов без float арифметики
pub const SCALE: u128 = 100;

/// Максимальное количество десятичных знаков для GEL
pub const GEL_DECIMALS: u8 = 2;

/// Минимальная сумма транзакции в subunits (0.01 GEL)
pub const MIN_TRANSACTION_AMOUNT: u128 = 1;

/// Максимальная сумма транзакции в subunits (100,000 GEL)
pub const MAX_TRANSACTION_AMOUNT: u128 = 10_000_000;

/// Максимальный процент владения для одного участника (48%)
pub const MAX_OWNERSHIP_PERCENTAGE: u128 = 48;

/// Процент для благотворительного фонда (3%)
pub const CHARITY_PERCENTAGE: u128 = 3;

/// Процент для владельца сети от собственных нод (48%)
pub const MAIN_OWNER_OWN_NODE_PERCENTAGE: u128 = 48;

/// Процент для покупателя от собственных нод (49%)
pub const CUSTOMER_OWN_NODE_PERCENTAGE: u128 = 49;

/// Процент для владельца сети от франшизных нод (25%)
pub const MAIN_OWNER_FRANCHISE_PERCENTAGE: u128 = 25;

/// Процент для владельца франшизы (24%)
pub const FRANCHISE_OWNER_PERCENTAGE: u128 = 24;

/// Процент для покупателя от франшизных нод (48%)
pub const CUSTOMER_FRANCHISE_PERCENTAGE: u128 = 48;

/// Процент для инвесторов (49%)
pub const INVESTOR_PERCENTAGE: u128 = 49;

/// Регуляторные настройки
pub mod regulatory {
    /// Название регулятора в Грузии
    pub const REGULATOR_NAME: &str = "GSSS";
    
    /// Формат экспорта реестров
    pub const EXPORT_FORMAT: &str = "CSV";
    
    /// Периодичность отчетности (дни)
    pub const REPORTING_PERIOD_DAYS: u32 = 30;
    
    /// Минимальный размер эмиссии для регистрации (1000 GEL)
    pub const MIN_EMISSION_SIZE: u128 = 100_000; // 1000.00 GEL в subunits
}

/// Настройки безопасности
pub mod security {
    /// Минимальная длина пароля
    pub const MIN_PASSWORD_LENGTH: usize = 8;
    
    /// Максимальное количество неудачных попыток входа
    pub const MAX_LOGIN_ATTEMPTS: u32 = 3;
    
    /// Время блокировки после неудачных попыток (секунды)
    pub const LOCKOUT_DURATION_SECONDS: u64 = 300; // 5 минут
    
    /// Размер ключа для шифрования
    pub const ENCRYPTION_KEY_SIZE: usize = 256;
}

/// Настройки сети
pub mod network {
    /// Стандартный порт для API
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Стандартный порт для P2P
    pub const DEFAULT_P2P_PORT: u16 = 8081;
    
    /// Таймаут для P2P соединений (секунды)
    pub const P2P_TIMEOUT_SECONDS: u64 = 30;
    
    /// Максимальное количество подключений
    pub const MAX_CONNECTIONS: usize = 100;
}

/// Утилиты для работы с subunits
pub mod utils {
    use super::*;
    
    /// Конвертирует GEL в subunits
    /// 
    /// # Примеры
    /// ```
    /// assert_eq!(gel_to_subunits(5.0), 500);
    /// assert_eq!(gel_to_subunits(0.01), 1);
    /// ```
    pub fn gel_to_subunits(gel: f64) -> u128 {
        (gel * SCALE as f64) as u128
    }
    
    /// Конвертирует subunits в GEL
    /// 
    /// # Примеры
    /// ```
    /// assert_eq!(subunits_to_gel(500), 5.0);
    /// assert_eq!(subunits_to_gel(1), 0.01);
    /// ```
    pub fn subunits_to_gel(subunits: u128) -> f64 {
        subunits as f64 / SCALE as f64
    }
    
    /// Форматирует subunits как GEL строку
    /// 
    /// # Примеры
    /// ```
    /// assert_eq!(format_gel(500), "5.00 GEL");
    /// assert_eq!(format_gel(1), "0.01 GEL");
    /// ```
    pub fn format_gel(subunits: u128) -> String {
        format!("{:.2} {}", subunits_to_gel(subunits), CURRENCY)
    }
    
    /// Проверяет, что сумма находится в допустимых пределах
    pub fn is_valid_amount(subunits: u128) -> bool {
        subunits >= MIN_TRANSACTION_AMOUNT && subunits <= MAX_TRANSACTION_AMOUNT
    }
    
    /// Вычисляет процент от суммы
    /// 
    /// # Примеры
    /// ```
    /// assert_eq!(calculate_percentage(1000, 25), 250);
    /// assert_eq!(calculate_percentage(1000, 3), 30);
    /// ```
    pub fn calculate_percentage(amount: u128, percentage: u128) -> u128 {
        (amount * percentage) / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gel_conversion() {
        assert_eq!(utils::gel_to_subunits(5.0), 500);
        assert_eq!(utils::gel_to_subunits(0.01), 1);
        assert_eq!(utils::gel_to_subunits(100.0), 10000);
        
        assert_eq!(utils::subunits_to_gel(500), 5.0);
        assert_eq!(utils::subunits_to_gel(1), 0.01);
        assert_eq!(utils::subunits_to_gel(10000), 100.0);
    }
    
    #[test]
    fn test_formatting() {
        assert_eq!(utils::format_gel(500), "5.00 GEL");
        assert_eq!(utils::format_gel(1), "0.01 GEL");
        assert_eq!(utils::format_gel(10000), "100.00 GEL");
    }
    
    #[test]
    fn test_percentage_calculation() {
        assert_eq!(utils::calculate_percentage(1000, 25), 250);
        assert_eq!(utils::calculate_percentage(1000, 3), 30);
        assert_eq!(utils::calculate_percentage(1000, 48), 480);
    }
    
    #[test]
    fn test_amount_validation() {
        assert!(utils::is_valid_amount(1)); // 0.01 GEL
        assert!(utils::is_valid_amount(1000)); // 10.00 GEL
        assert!(utils::is_valid_amount(10_000_000)); // 100,000.00 GEL
        
        assert!(!utils::is_valid_amount(0)); // Слишком мало
        assert!(!utils::is_valid_amount(10_000_001)); // Слишком много
    }
}
