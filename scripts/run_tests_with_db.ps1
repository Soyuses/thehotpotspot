# PowerShell скрипт для запуска тестов с PostgreSQL

Write-Host "🚀 Запуск тестов с PostgreSQL..." -ForegroundColor Green

# Проверяем, установлен ли Docker
try {
    docker --version | Out-Null
} catch {
    Write-Host "❌ Docker не установлен. Установите Docker для запуска тестов с БД." -ForegroundColor Red
    exit 1
}

# Проверяем, установлен ли docker-compose
try {
    docker-compose --version | Out-Null
} catch {
    Write-Host "❌ docker-compose не установлен. Установите docker-compose для запуска тестов с БД." -ForegroundColor Red
    exit 1
}

# Запускаем PostgreSQL в Docker
Write-Host "📦 Запуск PostgreSQL в Docker..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yml up -d postgres-test

# Ждем, пока PostgreSQL будет готов
Write-Host "⏳ Ожидание готовности PostgreSQL..." -ForegroundColor Yellow
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

if ($counter -eq $timeout) {
    Write-Host "❌ PostgreSQL не запустился в течение $timeout секунд" -ForegroundColor Red
    docker-compose -f docker-compose.test.yml down
    exit 1
}

# Запускаем тесты
Write-Host "🧪 Запуск тестов..." -ForegroundColor Yellow
$env:DATABASE_URL = "postgresql://postgres:password@localhost:5433/test_blockchain"
cargo test --lib

# Сохраняем код выхода
$testExitCode = $LASTEXITCODE

# Останавливаем PostgreSQL
Write-Host "🛑 Остановка PostgreSQL..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yml down

# Возвращаем код выхода тестов
exit $testExitCode
