# 🚀 Быстрый старт системы видеонаблюдения

## Обзор

Система видеонаблюдения The Hot Pot Spot включает:
- **4 типа камер** с разными настройками анонимизации
- **Стриминг** на Twitch и YouTube
- **Управление согласиями** согласно законодательству Грузии
- **Веб-интерфейсы** для управления и согласий

## 📹 Типы камер

### 1. Внешние камеры (External)
```rust
CameraConfig {
    camera_id: "CAM_EXT_001",
    camera_type: CameraType::External,
    location: "Main Entrance",
    anonymization_zone: AnonymizationZone::FullFaceBlur,
    requires_consent: false,
    stream_to_twitch: true,
    stream_to_youtube: false,
}
```

### 2. Камеры производства (Production)
```rust
CameraConfig {
    camera_id: "CAM_PROD_001",
    camera_type: CameraType::Production,
    location: "Kitchen Area",
    anonymization_zone: AnonymizationZone::FullFaceBlur,
    requires_consent: false,
    stream_to_twitch: true,
    stream_to_youtube: true,
}
```

### 3. Камеры откровений (Confession)
```rust
CameraConfig {
    camera_id: "CAM_CONF_001",
    camera_type: CameraType::Confession,
    location: "Feedback Zone",
    anonymization_zone: AnonymizationZone::FaceReplacement,
    requires_consent: true,
    max_recording_duration: Some(Duration::from_secs(30 * 60)),
}
```

### 4. Камеры за столами (Customer Table)
```rust
CameraConfig {
    camera_id: "CAM_TABLE_001",
    camera_type: CameraType::CustomerTable,
    location: "Table 1",
    anonymization_zone: AnonymizationZone::FaceReplacement,
    requires_consent: true,
    max_recording_duration: Some(Duration::from_secs(30 * 60)),
    stream_to_twitch: true,
    stream_to_youtube: true,
}
```

## 🔒 Уровни анонимизации

### 1. Полное размытие лиц (FullFaceBlur)
- Автоматическое обнаружение и размытие всех лиц
- Используется для внешних камер и производства
- Не требует согласия клиента

### 2. Замена лиц на аватар (FaceReplacement)
- Обнаружение лиц и замена на выбранное изображение
- Используется для камер за столами
- Требует согласия клиента

### 3. Без анонимизации (NoAnonymization)
- Лица остаются видимыми
- Только с явного согласия клиента
- Ограниченное время записи (30 минут)

## 📡 Стриминг

### Настройка Twitch
```rust
let twitch_config = TwitchConfig {
    client_id: "your_twitch_client_id",
    client_secret: "your_twitch_client_secret",
    access_token: "your_twitch_access_token",
    channel_name: "hotpotspot_georgia",
    stream_key: "your_twitch_stream_key",
};
```

### Настройка YouTube
```rust
let youtube_config = YouTubeConfig {
    client_id: "your_youtube_client_id",
    client_secret: "your_youtube_client_secret",
    channel_id: "your_youtube_channel_id",
    stream_key: "your_youtube_stream_key",
    api_key: "your_youtube_api_key",
};
```

### Качество стрима
- **Low**: 480p, 1.5 Mbps
- **Medium**: 720p, 3 Mbps (по умолчанию)
- **High**: 1080p, 6 Mbps

## 🖥️ Веб-интерфейсы

### 1. Интерфейс согласия
**URL**: `video_consent_interface.html`

**Функции:**
- Многоязычность (русский, грузинский, английский)
- Выбор уровня анонимизации
- Информация о правах клиента
- Юридические уведомления

**Использование:**
```
http://localhost:8080/video_consent_interface.html?customer_id=CUSTOMER_001&table_id=TABLE_001
```

### 2. Панель управления
**URL**: `video_management_dashboard.html`

**Разделы:**
- **Обзор**: Статистика системы
- **Камеры**: Управление камерами
- **Записи**: Мониторинг записей
- **Стриминг**: Управление стримами
- **Согласия**: Просмотр согласий
- **Настройки**: Конфигурация системы

## 🔧 API эндпоинты

### Управление согласиями
```bash
# Запрос согласия
curl -X POST http://localhost:8080/api/video-consent \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "CUSTOMER_001",
    "table_id": "TABLE_001",
    "language": "ru"
  }'

# Подтверждение согласия
curl -X POST http://localhost:8080/api/video-consent/confirm \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "CUSTOMER_001",
    "anonymization_preference": "replace"
  }'
```

### Управление записями
```bash
# Начать запись
curl -X POST http://localhost:8080/api/video-recording/start \
  -H "Content-Type: application/json" \
  -d '{
    "camera_id": "CAM_TABLE_001",
    "customer_id": "CUSTOMER_001",
    "table_id": "TABLE_001"
  }'

# Остановить запись
curl -X POST http://localhost:8080/api/video-recording/stop \
  -H "Content-Type: application/json" \
  -d '{
    "recording_id": "REC_12345678"
  }'
```

### Управление камерами
```bash
# Добавить камеру
curl -X POST http://localhost:8080/api/video-cameras \
  -H "Content-Type: application/json" \
  -d '{
    "camera_id": "CAM_NEW_001",
    "camera_type": "customer_table",
    "location": "Table 2",
    "ip_address": "192.168.1.103",
    "port": 8080,
    "resolution": [1920, 1080],
    "fps": 30,
    "anonymization_zone": "replace",
    "requires_consent": true,
    "stream_to_twitch": true,
    "stream_to_youtube": true
  }'

# Получить статистику камер
curl -X GET http://localhost:8080/api/video-cameras/stats
```

## ⚖️ Правовые требования

### Обязательные уведомления
Разместите в заведении:
```
📹 ВЕДЕТСЯ ВИДЕОНАБЛЮДЕНИЕ
📹 VIDEO SURVEILLANCE IN PROGRESS
📹 ვიდეო მონიტორინგი მიმდინარეობს

Цель: Безопасность и улучшение качества обслуживания
Срок хранения: 30 дней
Права: Доступ, исправление, удаление данных
```

### Текст согласия (многоязычный)
```
მე ვეთანხმები ჩემი ვიდეო ჩაწერას The Hot Pot Spot-ში ჩემი ვიზიტის დროს. 
ვიდეო შეიძლება გამოყენებულ იქნას უსაფრთხოების მიზნებისთვის და შეიძლება 
გადაცემული იქნეს Twitch და YouTube-ზე.

Я соглашаюсь на видеозапись моего пребывания в The Hot Pot Spot. 
Видео может использоваться в целях безопасности и может транслироваться 
на Twitch и YouTube.

I agree to video recording of my visit to The Hot Pot Spot. 
Video may be used for security purposes and may be streamed 
on Twitch and YouTube.
```

## 🚀 Запуск системы

### 1. Запуск основного приложения
```bash
cargo run
```

### 2. Открытие веб-интерфейсов
- **Панель управления**: `video_management_dashboard.html`
- **Интерфейс согласия**: `video_consent_interface.html`

### 3. Настройка камер
1. Откройте панель управления
2. Перейдите в раздел "Камеры"
3. Нажмите "Добавить камеру"
4. Заполните конфигурацию
5. Сохраните настройки

### 4. Тестирование согласий
1. Откройте интерфейс согласия
2. Выберите уровень анонимизации
3. Нажмите "Согласиться"
4. Проверьте в панели управления

## 📊 Мониторинг

### Ключевые метрики
- Количество активных камер
- Активные записи
- Активные стримы
- Зрители онлайн
- Статистика согласий

### Алерты
- Сбои камер
- Превышение лимитов
- Нарушения согласий
- Технические проблемы

## 🔧 Настройка

### Конфигурация стриминга
```toml
[streaming]
max_concurrent_streams = 5
default_quality = "medium"
max_bitrate = 6000
```

### Конфигурация анонимизации
```toml
[anonymization]
default_zone = "replace"
max_recording_duration = 1800  # 30 минут
face_detection_threshold = 0.8
```

## 🆘 Поддержка

### Частые проблемы

**Камера не подключается:**
- Проверьте IP адрес и порт
- Убедитесь в доступности сети
- Проверьте настройки firewall

**Согласие не сохраняется:**
- Проверьте подключение к API
- Убедитесь в корректности данных
- Проверьте логи системы

**Стрим не запускается:**
- Проверьте настройки Twitch/YouTube
- Убедитесь в корректности ключей
- Проверьте качество интернет-соединения

### Контакты
- **Техническая поддержка**: support@hotpotspot.ge
- **Юридические вопросы**: legal@hotpotspot.ge
- **Экстренные случаи**: +995 XXX XXX XXX

---

*Документ обновлен: Декабрь 2024*
*Версия системы: 1.0.0*
