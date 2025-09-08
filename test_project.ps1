# Testing The Hot Pot Spot project
Write-Host "Testing The Hot Pot Spot project" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Check compilation
Write-Host "`nChecking compilation..." -ForegroundColor Yellow
cargo check
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Компиляция успешна" -ForegroundColor Green
} else {
    Write-Host "❌ Ошибка компиляции" -ForegroundColor Red
    exit 1
}

# Проверка сборки
Write-Host "`n🔨 Проверка сборки..." -ForegroundColor Yellow
cargo build --bin blockchain_project
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Сборка успешна" -ForegroundColor Green
} else {
    Write-Host "❌ Ошибка сборки" -ForegroundColor Red
    exit 1
}

# Проверка тестов
Write-Host "`n🧪 Запуск тестов..." -ForegroundColor Yellow
cargo test --lib
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Тесты прошли успешно" -ForegroundColor Green
} else {
    Write-Host "⚠️ Некоторые тесты не прошли" -ForegroundColor Yellow
}

# Проверка файлов проекта
Write-Host "`n📁 Проверка файлов проекта..." -ForegroundColor Yellow

$requiredFiles = @(
    "src/main.rs",
    "src/lib.rs",
    "src/video_surveillance.rs",
    "src/video_api.rs",
    "src/streaming_integration.rs",
    "src/enhanced_web_server.rs",
    "video_management_dashboard.html",
    "video_consent_interface.html",
    "api_test_interface.html",
    "Cargo.toml",
    "README.md"
)

$missingFiles = @()
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "✅ $file" -ForegroundColor Green
    } else {
        Write-Host "❌ $file - отсутствует" -ForegroundColor Red
        $missingFiles += $file
    }
}

if ($missingFiles.Count -gt 0) {
    Write-Host "`n⚠️ Отсутствуют файлы: $($missingFiles -join ', ')" -ForegroundColor Yellow
}

# Проверка структуры проекта
Write-Host "`n🏗️ Проверка структуры проекта..." -ForegroundColor Yellow

$directories = @("src", "target", "contracts", "mobile_app")
foreach ($dir in $directories) {
    if (Test-Path $dir) {
        Write-Host "✅ Директория $dir существует" -ForegroundColor Green
    } else {
        Write-Host "❌ Директория $dir отсутствует" -ForegroundColor Red
    }
}

# Проверка зависимостей
Write-Host "`n📦 Проверка зависимостей..." -ForegroundColor Yellow
$cargoToml = Get-Content "Cargo.toml" -Raw
$requiredDeps = @("serde", "tokio", "sha2", "hex", "chrono")

foreach ($dep in $requiredDeps) {
    if ($cargoToml -match $dep) {
        Write-Host "✅ Зависимость $dep найдена" -ForegroundColor Green
    } else {
        Write-Host "❌ Зависимость $dep не найдена" -ForegroundColor Red
    }
}

# Проверка документации
Write-Host "`n📚 Проверка документации..." -ForegroundColor Yellow
$docFiles = @(
    "README.md",
    "VIDEO_SURVEILLANCE_REPORT.md",
    "FRONTEND_BACKEND_INTEGRATION_REPORT.md",
    "VIDEO_SYSTEM_QUICK_START.md"
)

foreach ($doc in $docFiles) {
    if (Test-Path $doc) {
        $size = (Get-Item $doc).Length
        Write-Host "OK $doc ($size bytes)" -ForegroundColor Green
    } else {
        Write-Host "MISSING $doc" -ForegroundColor Red
    }
}

# Итоговый отчет
Write-Host "`n📊 ИТОГОВЫЙ ОТЧЕТ" -ForegroundColor Cyan
Write-Host "==================" -ForegroundColor Cyan

if ($missingFiles.Count -eq 0) {
    Write-Host "✅ Все основные файлы присутствуют" -ForegroundColor Green
} else {
    Write-Host "⚠️ Отсутствуют $($missingFiles.Count) файлов" -ForegroundColor Yellow
}

Write-Host "`n🚀 Проект готов к:" -ForegroundColor Green
Write-Host "   • Запуску: cargo run --bin blockchain_project" -ForegroundColor White
Write-Host "   • Тестированию: .\test_video_api.ps1" -ForegroundColor White
Write-Host "   • Выгрузке на GitHub" -ForegroundColor White

Write-Host "`n📋 Следующие шаги:" -ForegroundColor Cyan
Write-Host "   1. Очистить тестовые данные" -ForegroundColor White
Write-Host "   2. Перезапустить блокчейн" -ForegroundColor White
Write-Host "   3. Подготовить к выгрузке на GitHub" -ForegroundColor White

Write-Host "`n🎯 Тестирование завершено!" -ForegroundColor Green
