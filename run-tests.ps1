# Скрипт для запуска всех тестов проекта

Write-Host "🚀 Запуск тестов проекта TheHotPotSpot" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Green

# Функция для проверки результата команды
function Test-CommandResult {
    param($Result, $TestName)
    if ($Result -eq 0) {
        Write-Host "✅ $TestName - УСПЕШНО" -ForegroundColor Green
    } else {
        Write-Host "❌ $TestName - ОШИБКА" -ForegroundColor Red
        return $false
    }
    return $true
}

# Проверяем, что мы в корневой директории проекта
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Ошибка: Cargo.toml не найден. Запустите скрипт из корневой директории проекта." -ForegroundColor Red
    exit 1
}

# Очищаем предыдущие сборки
Write-Host "🧹 Очистка предыдущих сборок..." -ForegroundColor Yellow
cargo clean
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️ Предупреждение: Не удалось очистить предыдущие сборки" -ForegroundColor Yellow
}

# Проверяем форматирование кода
Write-Host "📝 Проверка форматирования кода..." -ForegroundColor Cyan
$formatResult = cargo fmt --all -- --check
Test-CommandResult $LASTEXITCODE "Проверка форматирования"

# Запускаем clippy
Write-Host "🔍 Запуск clippy..." -ForegroundColor Cyan
$clippyResult = cargo clippy --all-targets --all-features -- -D warnings
Test-CommandResult $LASTEXITCODE "Clippy проверка"

# Запускаем unit тесты
Write-Host "🧪 Запуск unit тестов..." -ForegroundColor Cyan
$unitTestResult = cargo test --lib -- --nocapture
Test-CommandResult $LASTEXITCODE "Unit тесты"

# Запускаем интеграционные тесты
Write-Host "🔗 Запуск интеграционных тестов..." -ForegroundColor Cyan
$integrationTestResult = cargo test --test integration -- --nocapture
Test-CommandResult $LASTEXITCODE "Интеграционные тесты"

# Запускаем тесты безопасности
Write-Host "🛡️ Запуск тестов безопасности..." -ForegroundColor Cyan
$securityTestResult = cargo test --test security -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты безопасности"

# Запускаем нагрузочные тесты
Write-Host "⚡ Запуск нагрузочных тестов..." -ForegroundColor Cyan
$loadTestResult = cargo test --test load_testing -- --nocapture
Test-CommandResult $LASTEXITCODE "Нагрузочные тесты"

# Запускаем тесты атак
Write-Host "🎯 Запуск тестов атак..." -ForegroundColor Cyan
$attackTestResult = cargo test --test attack_scenarios -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты атак"

# Запускаем тесты API
Write-Host "🔌 Запуск тестов API..." -ForegroundColor Cyan
$apiTestResult = cargo test --test api -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты API"

# Запускаем тесты блокчейна
Write-Host "⛓️ Запуск тестов блокчейна..." -ForegroundColor Cyan
$blockchainTestResult = cargo test --test blockchain -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты блокчейна"

# Запускаем тесты ядра
Write-Host "🔧 Запуск тестов ядра..." -ForegroundColor Cyan
$coreTestResult = cargo test --test core -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты ядра"

# Запускаем тесты меню
Write-Host "🍔 Запуск тестов меню..." -ForegroundColor Cyan
$menuTestResult = cargo test --test menu -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты меню"

# Запускаем тесты заказов
Write-Host "📋 Запуск тестов заказов..." -ForegroundColor Cyan
$ordersTestResult = cargo test --test orders -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты заказов"

# Запускаем тесты веб-интерфейса
Write-Host "🌐 Запуск тестов веб-интерфейса..." -ForegroundColor Cyan
$webInterfaceTestResult = cargo test --test web_interface -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты веб-интерфейса"

# Запускаем тесты мобильного приложения
Write-Host "📱 Запуск тестов мобильного приложения..." -ForegroundColor Cyan
$mobileAppTestResult = cargo test --test mobile_app -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты мобильного приложения"

# Запускаем тесты новых токенов
Write-Host "🪙 Запуск тестов новых токенов..." -ForegroundColor Cyan
$newTokenTestResult = cargo test --test new_token_distribution -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты новых токенов"

# Запускаем тесты невостребованных токенов
Write-Host "💎 Запуск тестов невостребованных токенов..." -ForegroundColor Cyan
$unclaimedTokenTestResult = cargo test --test unclaimed_tokens_distribution -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты невостребованных токенов"

# Запускаем тесты анализа пакета управления
Write-Host "📊 Запуск тестов анализа пакета управления..." -ForegroundColor Cyan
$controlPackageTestResult = cargo test --test control_package_analysis -- --nocapture
Test-CommandResult $LASTEXITCODE "Тесты анализа пакета управления"

# Собираем проект
Write-Host "🔨 Сборка проекта..." -ForegroundColor Cyan
$buildResult = cargo build --release
Test-CommandResult $LASTEXITCODE "Сборка проекта"

# Проверяем, что все тесты прошли успешно
$allTestsPassed = $true
if ($formatResult -ne 0) { $allTestsPassed = $false }
if ($clippyResult -ne 0) { $allTestsPassed = $false }
if ($unitTestResult -ne 0) { $allTestsPassed = $false }
if ($integrationTestResult -ne 0) { $allTestsPassed = $false }
if ($securityTestResult -ne 0) { $allTestsPassed = $false }
if ($loadTestResult -ne 0) { $allTestsPassed = $false }
if ($attackTestResult -ne 0) { $allTestsPassed = $false }
if ($apiTestResult -ne 0) { $allTestsPassed = $false }
if ($blockchainTestResult -ne 0) { $allTestsPassed = $false }
if ($coreTestResult -ne 0) { $allTestsPassed = $false }
if ($menuTestResult -ne 0) { $allTestsPassed = $false }
if ($ordersTestResult -ne 0) { $allTestsPassed = $false }
if ($webInterfaceTestResult -ne 0) { $allTestsPassed = $false }
if ($mobileAppTestResult -ne 0) { $allTestsPassed = $false }
if ($newTokenTestResult -ne 0) { $allTestsPassed = $false }
if ($unclaimedTokenTestResult -ne 0) { $allTestsPassed = $false }
if ($controlPackageTestResult -ne 0) { $allTestsPassed = $false }
if ($buildResult -ne 0) { $allTestsPassed = $false }

# Выводим итоговый результат
Write-Host "================================================" -ForegroundColor Green
if ($allTestsPassed) {
    Write-Host "🎉 ВСЕ ТЕСТЫ ПРОШЛИ УСПЕШНО!" -ForegroundColor Green
    Write-Host "✅ Проект готов к деплою" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ НЕКОТОРЫЕ ТЕСТЫ НЕ ПРОШЛИ" -ForegroundColor Red
    Write-Host "⚠️ Проект не готов к деплою" -ForegroundColor Yellow
    exit 1
}
