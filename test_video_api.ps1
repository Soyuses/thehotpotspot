# Тестирование API системы видеонаблюдения
# PowerShell скрипт для проверки связи фронтенд-бэкенд

Write-Host "🎥 Тестирование API системы видеонаблюдения" -ForegroundColor Green
Write-Host "===============================================" -ForegroundColor Green

$baseUrl = "http://127.0.0.1:8080"

# Функция для отправки HTTP запросов
function Invoke-ApiRequest {
    param(
        [string]$Method,
        [string]$Url,
        [string]$Body = $null,
        [string]$ContentType = "application/json"
    )
    
    try {
        $headers = @{
            "Content-Type" = $ContentType
        }
        
        if ($Body) {
            $response = Invoke-RestMethod -Uri $Url -Method $Method -Body $Body -Headers $headers
        } else {
            $response = Invoke-RestMethod -Uri $Url -Method $Method -Headers $headers
        }
        
        return $response
    }
    catch {
        Write-Host "❌ Ошибка: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

# Тест 1: Запрос согласия на видеозапись
Write-Host "`n📋 Тест 1: Запрос согласия на видеозапись" -ForegroundColor Yellow
$consentRequest = @{
    customer_id = "CUSTOMER_001"
    table_id = "TABLE_001"
    language = "ru"
} | ConvertTo-Json

$response = Invoke-ApiRequest -Method "POST" -Url "$baseUrl/api/video-consent" -Body $consentRequest
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 2: Подтверждение согласия
Write-Host "`n📋 Тест 2: Подтверждение согласия" -ForegroundColor Yellow
$confirmRequest = @{
    customer_id = "CUSTOMER_001"
    anonymization_preference = "replace"
} | ConvertTo-Json

$response = Invoke-ApiRequest -Method "POST" -Url "$baseUrl/api/video-consent/confirm" -Body $confirmRequest
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 3: Начало записи
Write-Host "`n🎥 Тест 3: Начало записи" -ForegroundColor Yellow
$startRecordingRequest = @{
    camera_id = "CAM_TABLE_001"
    customer_id = "CUSTOMER_001"
    table_id = "TABLE_001"
} | ConvertTo-Json

$response = Invoke-ApiRequest -Method "POST" -Url "$baseUrl/api/video-recording/start" -Body $startRecordingRequest
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 4: Получение активных записей
Write-Host "`n📊 Тест 4: Получение активных записей" -ForegroundColor Yellow
$response = Invoke-ApiRequest -Method "GET" -Url "$baseUrl/api/video-recording/active"
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 5: Статистика камер
Write-Host "`n📈 Тест 5: Статистика камер" -ForegroundColor Yellow
$response = Invoke-ApiRequest -Method "GET" -Url "$baseUrl/api/video-cameras/stats"
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 6: Добавление новой камеры
Write-Host "`n📹 Тест 6: Добавление новой камеры" -ForegroundColor Yellow
$addCameraRequest = @{
    camera_id = "CAM_TEST_001"
    camera_type = "customer_table"
    location = "Test Table"
    ip_address = "192.168.1.200"
    port = 8080
    resolution = @(1920, 1080)
    fps = 30
    anonymization_zone = "replace"
    requires_consent = $true
    stream_to_twitch = $true
    stream_to_youtube = $false
} | ConvertTo-Json

$response = Invoke-ApiRequest -Method "POST" -Url "$baseUrl/api/video-cameras" -Body $addCameraRequest
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 7: Остановка записи
Write-Host "`n⏹️ Тест 7: Остановка записи" -ForegroundColor Yellow
$stopRecordingRequest = @{
    recording_id = "REC_12345678"
} | ConvertTo-Json

$response = Invoke-ApiRequest -Method "POST" -Url "$baseUrl/api/video-recording/stop" -Body $stopRecordingRequest
if ($response) {
    Write-Host "✅ Ответ получен:" -ForegroundColor Green
    $response | ConvertTo-Json -Depth 3
} else {
    Write-Host "❌ Запрос не удался" -ForegroundColor Red
}

# Тест 8: Проверка доступности веб-интерфейсов
Write-Host "`n🌐 Тест 8: Проверка доступности веб-интерфейсов" -ForegroundColor Yellow

$interfaces = @(
    @{ Name = "Главная страница"; Url = "/" },
    @{ Name = "Панель управления видео"; Url = "/video_management_dashboard.html" },
    @{ Name = "Интерфейс согласия"; Url = "/video_consent_interface.html" },
    @{ Name = "Дашборд владельца"; Url = "/owner_dashboard.html" },
    @{ Name = "Кошелек клиента"; Url = "/customer_wallet.html" }
)

foreach ($interface in $interfaces) {
    try {
        $response = Invoke-WebRequest -Uri "$baseUrl$($interface.Url)" -Method "GET" -TimeoutSec 5
        if ($response.StatusCode -eq 200) {
            Write-Host "✅ $($interface.Name): Доступен" -ForegroundColor Green
        } else {
            Write-Host "⚠️ $($interface.Name): Статус $($response.StatusCode)" -ForegroundColor Yellow
        }
    }
    catch {
        Write-Host "❌ $($interface.Name): Недоступен - $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`n🎯 Тестирование завершено!" -ForegroundColor Green
Write-Host "===============================================" -ForegroundColor Green
Write-Host "Для полного тестирования:" -ForegroundColor Cyan
Write-Host "1. Запустите сервер: cargo run" -ForegroundColor Cyan
Write-Host "2. Откройте браузер: http://127.0.0.1:8080/video_management_dashboard.html" -ForegroundColor Cyan
Write-Host "3. Протестируйте интерфейс согласия: http://127.0.0.1:8080/video_consent_interface.html" -ForegroundColor Cyan
