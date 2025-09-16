# План развертывания The Hot Pot Spot в продакшене

## 🎯 Цели развертывания
- Поддержка 1-2 точек франшиз
- Репликация блокчейна на 3 нодах
- Препрод и прод среды
- Бесплатное хостинг решение

## 🏗️ Архитектура продакшена

### Блокчейн ноды (3 ноды)
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Нода 1        │    │   Нода 2        │    │   Нода 3        │
│   (Master)      │◄──►│   (Validator)   │◄──►│   (Validator)   │
│   - API Server  │    │   - API Server  │    │   - API Server  │
│   - Database    │    │   - Database    │    │   - Database    │
│   - Video API   │    │   - Video API   │    │   - Video API   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Точки франшиз (1-2 точки)
```
┌─────────────────┐    ┌─────────────────┐
│   Фудтрак 1     │    │   Фудтрак 2     │
│   - POS система │    │   - POS система │
│   - Камеры      │    │   - Камеры      │
│   - Мобильное   │    │   - Мобильное   │
│     приложение  │    │     приложение  │
└─────────────────┘    └─────────────────┘
```

## 🆓 Бесплатные хостинг решения

### 1. Railway (Рекомендуется)
- **Бесплатный план**: $5 кредитов в месяц
- **Поддержка**: Rust, PostgreSQL, Docker
- **Преимущества**: Автоматические деплои, SSL, домены
- **Ограничения**: 500 часов в месяц

**Настройка:**
```bash
# Установка Railway CLI
npm install -g @railway/cli

# Логин
railway login

# Создание проекта
railway init

# Деплой
railway up
```

### 2. Render
- **Бесплатный план**: 750 часов в месяц
- **Поддержка**: Rust, PostgreSQL
- **Преимущества**: Автоматические деплои, SSL
- **Ограничения**: Спит после 15 минут неактивности

### 3. Fly.io
- **Бесплатный план**: 3 приложения, 256MB RAM
- **Поддержка**: Rust, PostgreSQL
- **Преимущества**: Глобальная сеть, быстрый старт
- **Ограничения**: Ограниченные ресурсы

### 4. Heroku (Альтернатива)
- **Платный план**: $7/месяц за приложение
- **Поддержка**: Rust, PostgreSQL
- **Преимущества**: Надежность, масштабируемость

## 🖥️ Требования к оборудованию

### Минимальные требования для 1-2 точек:

#### Сервер (VPS)
- **CPU**: 2 ядра
- **RAM**: 4GB
- **Storage**: 50GB SSD
- **Network**: 100 Mbps
- **OS**: Ubuntu 20.04 LTS

#### Точка франшизы
- **Планшет/ПК**: 4GB RAM, 64GB storage
- **Камера**: USB 2.0, 1080p, 30fps
- **Интернет**: 50 Mbps стабильный
- **Принтер**: Термопринтер для чеков

### Рекомендуемые требования:

#### Сервер (VPS)
- **CPU**: 4 ядра
- **RAM**: 8GB
- **Storage**: 100GB SSD
- **Network**: 1 Gbps
- **OS**: Ubuntu 22.04 LTS

#### Точка франшизы
- **Планшет**: iPad или Android планшет 10"
- **Камера**: USB 3.0, 4K, 60fps
- **Интернет**: 100 Mbps стабильный
- **Принтер**: Термопринтер с WiFi

## 🚀 План развертывания

### Этап 1: Препрод среда
```bash
# 1. Создание препрод окружения
railway create thehotpotspot-preprod

# 2. Настройка базы данных
railway add postgresql

# 3. Настройка переменных окружения
railway variables set DATABASE_URL=$DATABASE_URL
railway variables set BLOCKCHAIN_NETWORK=testnet
railway variables set API_PORT=3000

# 4. Деплой приложения
railway up
```

### Этап 2: Продакшен среда
```bash
# 1. Создание продакшен окружения
railway create thehotpotspot-prod

# 2. Настройка 3 нод
railway create thehotpotspot-node1
railway create thehotpotspot-node2
railway create thehotpotspot-node3

# 3. Настройка базы данных для каждой ноды
railway add postgresql --service thehotpotspot-node1
railway add postgresql --service thehotpotspot-node2
railway add postgresql --service thehotpotspot-node3

# 4. Настройка переменных окружения
railway variables set BLOCKCHAIN_NETWORK=mainnet
railway variables set NODE_ID=1
railway variables set PEER_NODES=node2,node3
```

### Этап 3: Настройка блокчейна
```bash
# 1. Инициализация блокчейна на ноде 1
railway run --service thehotpotspot-node1 cargo run --bin blockchain_init

# 2. Подключение нод 2 и 3
railway run --service thehotpotspot-node2 cargo run --bin blockchain_join
railway run --service thehotpotspot-node3 cargo run --bin blockchain_join

# 3. Проверка синхронизации
railway run --service thehotpotspot-node1 cargo run --bin blockchain_status
```

## 📱 Настройка точек франшиз

### 1. Установка POS системы
```bash
# Скачивание мобильного приложения
wget https://github.com/Soyuses/thehotpotspot/releases/latest/download/mobile-app.apk

# Установка на Android устройство
adb install mobile-app.apk
```

### 2. Настройка камер
```bash
# Проверка доступных камер
ls /dev/video*

# Настройка разрешения
v4l2-ctl --device=/dev/video0 --set-fmt-video=width=1920,height=1080,pixelformat=YUYV
```

### 3. Настройка принтера
```bash
# Установка драйверов термопринтера
sudo apt-get install cups cups-client

# Добавление принтера
lpadmin -p "ThermalPrinter" -E -v "usb://Thermal/Printer" -m "raw"
```

## 🔧 Конфигурация

### Переменные окружения
```bash
# Основные настройки
DATABASE_URL=postgresql://user:password@localhost/thehotpotspot
BLOCKCHAIN_NETWORK=mainnet
API_PORT=3000
VIDEO_API_PORT=8083
CHECK_API_PORT=8081
TRANSPARENCY_API_PORT=8082

# Блокчейн настройки
NODE_ID=1
PEER_NODES=node2,node3
CONSENSUS_ALGORITHM=proof_of_stake
BLOCK_TIME=10

# Безопасность
JWT_SECRET=your_jwt_secret_here
ENCRYPTION_KEY=your_encryption_key_here
KYC_PROVIDER_API_KEY=your_kyc_provider_key

# Стриминг
TWITCH_CLIENT_ID=your_twitch_client_id
YOUTUBE_API_KEY=your_youtube_api_key
STREAM_QUALITY=high
STREAM_BITRATE=5000

# Уведомления
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASS=your_app_password
```

### Docker конфигурация
```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blockchain_project /usr/local/bin/
EXPOSE 3000 8081 8082 8083
CMD ["blockchain_project"]
```

## 📊 Мониторинг и логирование

### 1. Логирование
```bash
# Настройка логов
railway variables set LOG_LEVEL=info
railway variables set LOG_FORMAT=json

# Просмотр логов
railway logs --service thehotpotspot-node1
```

### 2. Мониторинг
```bash
# Установка Prometheus
railway add prometheus

# Настройка метрик
railway variables set ENABLE_METRICS=true
railway variables set METRICS_PORT=9090
```

### 3. Алерты
```bash
# Настройка уведомлений
railway variables set ALERT_EMAIL=admin@thehotpotspot.com
railway variables set ALERT_SLACK_WEBHOOK=https://hooks.slack.com/...
```

## 🔒 Безопасность

### 1. SSL сертификаты
```bash
# Автоматические SSL сертификаты через Railway
railway domains add thehotpotspot.com
```

### 2. Firewall
```bash
# Настройка UFW
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

### 3. Резервное копирование
```bash
# Автоматические бэкапы базы данных
railway variables set BACKUP_SCHEDULE="0 2 * * *"
railway variables set BACKUP_RETENTION_DAYS=30
```

## 💰 Стоимость

### Бесплатный план (Railway)
- **3 ноды**: $0 (в рамках бесплатного плана)
- **База данных**: $0 (в рамках бесплатного плана)
- **Домены**: $0 (поддомены)
- **SSL**: $0 (автоматические сертификаты)

### Платный план (при необходимости)
- **3 ноды**: $15/месяц
- **База данных**: $10/месяц
- **Домены**: $12/год
- **Мониторинг**: $5/месяц
- **Итого**: ~$30/месяц

## 🎯 Рекомендации

### 1. Начать с бесплатного плана
- Использовать Railway бесплатный план
- Начать с 1 ноды, добавить остальные при необходимости
- Мониторить использование ресурсов

### 2. Постепенное масштабирование
- Начать с 1 точки франшизы
- Добавить вторую точку после стабилизации
- Масштабировать по мере роста

### 3. Резервные планы
- Подготовить альтернативные хостинг решения
- Настроить автоматические бэкапы
- Подготовить план восстановления

## 📋 Чек-лист развертывания

### Препрод
- [ ] Создать Railway проект
- [ ] Настроить базу данных
- [ ] Деплоить приложение
- [ ] Протестировать все функции
- [ ] Настроить мониторинг

### Продакшен
- [ ] Создать 3 ноды
- [ ] Настроить блокчейн
- [ ] Настроить SSL
- [ ] Настроить мониторинг
- [ ] Настроить бэкапы

### Точки франшиз
- [ ] Установить POS систему
- [ ] Настроить камеры
- [ ] Настроить принтер
- [ ] Протестировать интеграцию
- [ ] Обучить персонал

---

**Дата создания**: $(date)  
**Версия**: 1.0.0  
**Статус**: Готов к реализации