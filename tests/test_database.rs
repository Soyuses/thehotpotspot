use blockchain_project::database::{DatabaseManager, DatabaseConfig, UserData};
use std::time::SystemTime;
use tokio;

#[tokio::test]
async fn test_database_connection() {
    // Создаем тестовую конфигурацию
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "test_blockchain".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        max_connections: 10,
        connection_timeout: 30,
    };

    // Пытаемся подключиться к тестовой БД
    let result = DatabaseManager::new(config).await;
    
    // Если PostgreSQL недоступен, пропускаем тест
    if result.is_err() {
        println!("PostgreSQL недоступен, пропускаем тест");
        return;
    }
    
    let db = result.unwrap();
    
    // Тестируем подключение
    let connection_result = db.test_connection().await;
    assert!(connection_result.is_ok(), "Не удалось подключиться к базе данных");
}

#[tokio::test]
async fn test_user_operations() {
    // Создаем тестовую конфигурацию
    let config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "test_blockchain".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        max_connections: 10,
        connection_timeout: 30,
    };

    // Пытаемся подключиться к тестовой БД
    let result = DatabaseManager::new(config).await;
    
    // Если PostgreSQL недоступен, пропускаем тест
    if result.is_err() {
        println!("PostgreSQL недоступен, пропускаем тест");
        return;
    }
    
    let db = result.unwrap();
    
    // Создаем тестового пользователя
    let user = UserData {
        user_id: "test_user_001".to_string(),
        email: "test@example.com".to_string(),
        phone: Some("+995123456789".to_string()),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        date_of_birth: Some(SystemTime::now()),
        nationality: Some("GE".to_string()),
        address_street: Some("123 Main St".to_string()),
        address_city: Some("Tbilisi".to_string()),
        address_state: Some("Tbilisi".to_string()),
        address_postal_code: Some("0100".to_string()),
        address_country: Some("Georgia".to_string()),
        kyc_status: "Verified".to_string(),
        kyc_level: "Basic".to_string(),
        kyc_started_at: Some(SystemTime::now()),
        kyc_completed_at: Some(SystemTime::now()),
        kyc_expires_at: Some(SystemTime::now()),
        risk_score: 25,
        sanctions_check: false,
        pep_status: false,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
        last_login: None,
    };
    
    // Сохраняем пользователя
    let save_result = db.save_user(&user).await;
    assert!(save_result.is_ok(), "Не удалось сохранить пользователя");
    
    // Получаем пользователя
    let get_result = db.get_user("test_user_001").await;
    assert!(get_result.is_ok(), "Не удалось получить пользователя");
    
    let retrieved_user = get_result.unwrap();
    assert!(retrieved_user.is_some(), "Пользователь не найден");
    
    // Проверяем данные
    let user_data = retrieved_user.unwrap();
    assert_eq!(user_data.email, "test@example.com");
    assert_eq!(user_data.first_name, "John");
    assert_eq!(user_data.last_name, "Doe");
}
