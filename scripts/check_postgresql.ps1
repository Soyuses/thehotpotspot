# Скрипт для проверки доступности PostgreSQL

Write-Host "🔍 Проверка доступности PostgreSQL..." -ForegroundColor Yellow

# Проверка Docker
Write-Host "📦 Проверка Docker..." -ForegroundColor Cyan
try {
    $dockerVersion = docker --version
    Write-Host "✅ Docker найден: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker не найден. Установите Docker Desktop:" -ForegroundColor Red
    Write-Host "   https://docs.docker.com/desktop/windows/install/" -ForegroundColor Yellow
    exit 1
}

# Проверка docker-compose
Write-Host "🔧 Проверка docker-compose..." -ForegroundColor Cyan
try {
    $composeVersion = docker-compose --version
    Write-Host "✅ docker-compose найден: $composeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ docker-compose не найден" -ForegroundColor Red
    exit 1
}

# Проверка файла docker-compose.test.yml
Write-Host "📄 Проверка конфигурации..." -ForegroundColor Cyan
if (Test-Path "docker-compose.test.yml") {
    Write-Host "✅ docker-compose.test.yml найден" -ForegroundColor Green
} else {
    Write-Host "❌ docker-compose.test.yml не найден" -ForegroundColor Red
    exit 1
}

# Попытка запуска PostgreSQL
Write-Host "🚀 Запуск PostgreSQL..." -ForegroundColor Cyan
try {
    docker-compose -f docker-compose.test.yml up -d postgres-test
    Write-Host "✅ PostgreSQL запущен" -ForegroundColor Green
} catch {
    Write-Host "❌ Не удалось запустить PostgreSQL" -ForegroundColor Red
    Write-Host "   Проверьте, что Docker Desktop запущен" -ForegroundColor Yellow
    exit 1
}

# Ожидание готовности PostgreSQL
Write-Host "⏳ Ожидание готовности PostgreSQL..." -ForegroundColor Cyan
$timeout = 60
$counter = 0
$ready = $false

do {
    Start-Sleep -Seconds 2
    $counter += 2
    Write-Host "   Проверка... ($counter/$timeout сек)" -ForegroundColor Gray
    
    try {
        $result = docker-compose -f docker-compose.test.yml exec -T postgres-test pg_isready -U postgres -d test_blockchain 2>$null
        if ($LASTEXITCODE -eq 0) {
            $ready = $true
            Write-Host "✅ PostgreSQL готов!" -ForegroundColor Green
            break
        }
    } catch {
        # Игнорируем ошибки во время проверки
    }
} while ($counter -lt $timeout)

if (-not $ready) {
    Write-Host "❌ PostgreSQL не готов в течение $timeout секунд" -ForegroundColor Red
    Write-Host "   Проверьте логи: docker-compose -f docker-compose.test.yml logs postgres-test" -ForegroundColor Yellow
    exit 1
}

# Проверка подключения к базе данных
Write-Host "🔗 Проверка подключения к базе данных..." -ForegroundColor Cyan
try {
    $env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
    $result = docker-compose -f docker-compose.test.yml exec -T postgres-test psql -U postgres -d test_blockchain -c "SELECT 1;" 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Подключение к базе данных успешно" -ForegroundColor Green
    } else {
        Write-Host "❌ Не удалось подключиться к базе данных" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Ошибка при проверке подключения" -ForegroundColor Red
}

# Проверка таблиц
Write-Host "📊 Проверка структуры базы данных..." -ForegroundColor Cyan
try {
    $tables = docker-compose -f docker-compose.test.yml exec -T postgres-test psql -U postgres -d test_blockchain -t -c "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';" 2>$null
    if ($LASTEXITCODE -eq 0 -and $tables) {
        Write-Host "✅ Таблицы найдены:" -ForegroundColor Green
        $tables | ForEach-Object { Write-Host "   - $($_.Trim())" -ForegroundColor Gray }
    } else {
        Write-Host "⚠️  Таблицы не найдены. Возможно, нужно выполнить инициализацию." -ForegroundColor Yellow
    }
} catch {
    Write-Host "❌ Ошибка при проверке таблиц" -ForegroundColor Red
}

# Запуск тестов
Write-Host "🧪 Запуск тестов с PostgreSQL..." -ForegroundColor Cyan
try {
    $env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
    cargo test --test test_database -- --nocapture
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Тесты с PostgreSQL прошли успешно!" -ForegroundColor Green
    } else {
        Write-Host "❌ Тесты с PostgreSQL завершились с ошибками" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Ошибка при запуске тестов" -ForegroundColor Red
}

# Остановка PostgreSQL
Write-Host "🛑 Остановка PostgreSQL..." -ForegroundColor Cyan
try {
    docker-compose -f docker-compose.test.yml down
    Write-Host "✅ PostgreSQL остановлен" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Не удалось остановить PostgreSQL" -ForegroundColor Yellow
}

Write-Host "🎉 Проверка завершена!" -ForegroundColor Green
Write-Host "📋 Следующие шаги:" -ForegroundColor Cyan
Write-Host "   1. Установите Docker Desktop (если не установлен)" -ForegroundColor Gray
Write-Host "   2. Запустите: .\scripts\run_tests_with_db.ps1" -ForegroundColor Gray
Write-Host "   3. Или вручную: docker-compose -f docker-compose.test.yml up -d" -ForegroundColor Gray
