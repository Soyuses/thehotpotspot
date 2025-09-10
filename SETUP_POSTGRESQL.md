# 🐘 Настройка PostgreSQL для TheHotPotSpot

## 📋 Варианты установки PostgreSQL

### Вариант 1: Docker (Рекомендуется)

#### 1. Установка Docker Desktop
1. Скачайте Docker Desktop для Windows: https://docs.docker.com/desktop/windows/install/
2. Установите Docker Desktop
3. Запустите Docker Desktop
4. Убедитесь, что Docker запущен (иконка в системном трее)

#### 2. Запуск PostgreSQL в Docker
```bash
# Запуск тестовой PostgreSQL
docker-compose -f docker-compose.test.yml up -d postgres-test

# Проверка статуса
docker-compose -f docker-compose.test.yml ps

# Просмотр логов
docker-compose -f docker-compose.test.yml logs postgres-test
```

#### 3. Запуск тестов с PostgreSQL
```bash
# Windows PowerShell
.\scripts\run_tests_with_db.ps1

# Или вручную
$env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
cargo test --test test_database
```

### Вариант 2: Локальная установка PostgreSQL

#### 1. Установка PostgreSQL
1. Скачайте PostgreSQL: https://www.postgresql.org/download/windows/
2. Установите PostgreSQL с настройками по умолчанию
3. Запомните пароль для пользователя `postgres`

#### 2. Создание тестовой базы данных
```sql
-- Подключитесь к PostgreSQL как пользователь postgres
-- Создайте тестовую базу данных
CREATE DATABASE test_blockchain;

-- Создайте пользователя для тестов (опционально)
CREATE USER test_user WITH PASSWORD 'test_password';
GRANT ALL PRIVILEGES ON DATABASE test_blockchain TO test_user;
```

#### 3. Инициализация схемы
```bash
# Выполните SQL скрипт инициализации
psql -U postgres -d test_blockchain -f tests/init_test_db.sql
```

#### 4. Настройка переменных окружения
```bash
# Windows PowerShell
$env:DATABASE_URL = "postgresql://postgres:your_password@localhost:5432/test_blockchain"

# Windows CMD
set DATABASE_URL=postgresql://postgres:your_password@localhost:5432/test_blockchain

# Linux/macOS
export DATABASE_URL="postgresql://postgres:your_password@localhost:5432/test_blockchain"
```

### Вариант 3: Использование встроенной SQLite (Для разработки)

Если PostgreSQL недоступен, можно временно использовать SQLite:

#### 1. Добавить зависимость SQLite
```toml
# В Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
```

#### 2. Создать адаптер для SQLite
```rust
// Создать модуль database_sqlite.rs
// Адаптировать DatabaseManager для работы с SQLite
```

## 🧪 Запуск тестов

### Все тесты (включая PostgreSQL)
```bash
cargo test --all-targets
```

### Только unit тесты (без PostgreSQL)
```bash
cargo test --lib
```

### Тесты с PostgreSQL
```bash
# С Docker
docker-compose -f docker-compose.test.yml up -d
cargo test --test test_database

# С локальным PostgreSQL
cargo test --test test_database
```

### Property-based тесты
```bash
cargo test --test property_tests
```

### Интеграционные тесты
```bash
cargo test --test integration_tests
```

### Нагрузочные тесты
```bash
cargo bench --bench load_tests
```

## 🔧 Конфигурация

### Переменные окружения
```bash
# Основная база данных
DATABASE_URL=postgresql://postgres:password@localhost:5432/blockchain_db

# Тестовая база данных
TEST_DATABASE_URL=postgresql://postgres:password@localhost:5433/test_blockchain

# Настройки подключения
DB_HOST=localhost
DB_PORT=5432
DB_NAME=blockchain_db
DB_USER=postgres
DB_PASSWORD=your_password
DB_MAX_CONNECTIONS=10
DB_CONNECTION_TIMEOUT=30
```

### Docker Compose конфигурация
```yaml
# docker-compose.test.yml
version: '3.8'
services:
  postgres-test:
    image: postgres:15
    environment:
      POSTGRES_DB: test_blockchain
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5433:5432"
    volumes:
      - postgres_test_data:/var/lib/postgresql/data
      - ./tests/init_test_db.sql:/docker-entrypoint-initdb.d/init_test_db.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres -d test_blockchain"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_test_data:
```

## 🚀 Автоматизация

### Скрипт для Windows PowerShell
```powershell
# scripts/run_tests_with_db.ps1
Write-Host "🚀 Запуск тестов с PostgreSQL..." -ForegroundColor Green

# Проверка Docker
try {
    docker --version | Out-Null
} catch {
    Write-Host "❌ Docker не установлен. Установите Docker Desktop." -ForegroundColor Red
    exit 1
}

# Запуск PostgreSQL
docker-compose -f docker-compose.test.yml up -d postgres-test

# Ожидание готовности
$timeout = 60
$counter = 0
do {
    Start-Sleep -Seconds 1
    $counter++
    $ready = docker-compose -f docker-compose.test.yml exec -T postgres-test pg_isready -U postgres -d test_blockchain 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ PostgreSQL готов!" -ForegroundColor Green
        break
    }
} while ($counter -lt $timeout)

# Запуск тестов
$env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
cargo test --test test_database

# Остановка PostgreSQL
docker-compose -f docker-compose.test.yml down
```

### Скрипт для Linux/macOS
```bash
#!/bin/bash
# scripts/run_tests_with_db.sh

echo "🚀 Запуск тестов с PostgreSQL..."

# Проверка Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker не установлен. Установите Docker."
    exit 1
fi

# Запуск PostgreSQL
docker-compose -f docker-compose.test.yml up -d postgres-test

# Ожидание готовности
timeout=60
counter=0
while [ $counter -lt $timeout ]; do
    if docker-compose -f docker-compose.test.yml exec -T postgres-test pg_isready -U postgres -d test_blockchain &> /dev/null; then
        echo "✅ PostgreSQL готов!"
        break
    fi
    sleep 1
    counter=$((counter + 1))
done

# Запуск тестов
export DATABASE_URL="postgresql://postgres:password@localhost:5433/test_blockchain"
cargo test --test test_database

# Остановка PostgreSQL
docker-compose -f docker-compose.test.yml down
```

## 🐛 Устранение проблем

### Проблема: Docker не запускается
**Решение:**
1. Убедитесь, что Docker Desktop установлен и запущен
2. Проверьте, что WSL2 включен (для Windows)
3. Перезапустите Docker Desktop

### Проблема: PostgreSQL не подключается
**Решение:**
1. Проверьте, что PostgreSQL запущен: `docker-compose ps`
2. Проверьте логи: `docker-compose logs postgres-test`
3. Убедитесь, что порт 5433 свободен

### Проблема: Тесты не находят базу данных
**Решение:**
1. Проверьте переменную DATABASE_URL
2. Убедитесь, что база данных создана
3. Проверьте права доступа пользователя

### Проблема: Медленные тесты
**Решение:**
1. Используйте SSD для Docker volumes
2. Увеличьте память для Docker Desktop
3. Оптимизируйте SQL запросы

## 📊 Мониторинг

### Проверка статуса PostgreSQL
```bash
# Статус контейнера
docker-compose -f docker-compose.test.yml ps

# Логи PostgreSQL
docker-compose -f docker-compose.test.yml logs postgres-test

# Подключение к базе данных
docker-compose -f docker-compose.test.yml exec postgres-test psql -U postgres -d test_blockchain
```

### Мониторинг производительности
```bash
# Статистика использования ресурсов
docker stats

# Логи приложения
cargo test --test test_database -- --nocapture
```

## 🎯 Следующие шаги

1. **Установите Docker Desktop** (рекомендуется)
2. **Запустите PostgreSQL** в Docker
3. **Выполните тесты** с базой данных
4. **Настройте CI/CD** для автоматического тестирования
5. **Оптимизируйте производительность** тестов

## 📞 Поддержка

При возникновении проблем:
1. Проверьте логи Docker: `docker-compose logs`
2. Убедитесь, что все зависимости установлены
3. Проверьте переменные окружения
4. Обратитесь к документации PostgreSQL
