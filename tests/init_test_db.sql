-- Создание тестовой базы данных для blockchain проекта

-- Создание таблицы пользователей
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(255) PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    date_of_birth TIMESTAMP,
    nationality VARCHAR(10),
    address_street VARCHAR(255),
    address_city VARCHAR(100),
    address_state VARCHAR(100),
    address_postal_code VARCHAR(20),
    address_country VARCHAR(100),
    kyc_status VARCHAR(50) NOT NULL,
    kyc_level VARCHAR(50) NOT NULL,
    kyc_started_at TIMESTAMP,
    kyc_completed_at TIMESTAMP,
    kyc_expires_at TIMESTAMP,
    risk_score INTEGER DEFAULT 0,
    sanctions_check BOOLEAN DEFAULT FALSE,
    pep_status BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP
);

-- Создание таблицы аудит логов
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    success BOOLEAN NOT NULL,
    details TEXT,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
);

-- Создание таблицы чек-кошельков
CREATE TABLE IF NOT EXISTS check_wallets (
    wallet_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    amount_subunits BIGINT NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'GEL',
    is_activated BOOLEAN DEFAULT FALSE,
    activation_code VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    activated_at TIMESTAMP
);

-- Создание индексов для оптимизации
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_kyc_status ON users(kyc_status);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_check_wallets_user_id ON check_wallets(user_id);
CREATE INDEX IF NOT EXISTS idx_check_wallets_expires_at ON check_wallets(expires_at);

-- Вставка тестовых данных
INSERT INTO users (
    user_id, email, phone, first_name, last_name, 
    date_of_birth, nationality, address_street, address_city, 
    address_state, address_postal_code, address_country,
    kyc_status, kyc_level, kyc_started_at, kyc_completed_at, 
    kyc_expires_at, risk_score, sanctions_check, pep_status,
    created_at, updated_at
) VALUES (
    'test_user_001', 'test@example.com', '+995123456789', 'John', 'Doe',
    NOW() - INTERVAL '25 years', 'GE', '123 Main St', 'Tbilisi',
    'Tbilisi', '0100', 'Georgia',
    'Verified', 'Basic', NOW() - INTERVAL '1 day', NOW(),
    NOW() + INTERVAL '365 days', 25, FALSE, FALSE,
    NOW(), NOW()
) ON CONFLICT (user_id) DO NOTHING;
