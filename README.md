# The Hot Pot Spot - Блокчейн сеть фудтраков

## 🍲 Описание проекта

The Hot Pot Spot - это инновационная блокчейн-основанная сеть фудтраков, которая объединяет традиционный ресторанный бизнес с современными технологиями блокчейна, токеномики и мобильных приложений.

## ✨ Основные возможности

### 🔗 Блокчейн интеграция
- **ST токены** для владельцев франшиз
- **UT токены** для пользователей и зрителей стримов
- **DAO управление** системой через голосование
- **KYC/AML** проверки для безопасности

### 📱 Мобильное приложение
- **React Native** приложение
- **QR-коды** для быстрых платежей
- **Анонимные кошельки** для конфиденциальности
- **Интеграция со стримингом** для начисления токенов

### 🏪 Управление франшизами
- **Панель владельца** с аналитикой и управлением
- **Панель франчайзи** с планированием закупок
- **Система рецептов** и расчет стоимости
- **Отслеживание затрат** и эффективности

### 📊 Прозрачность и отчетность
- **Публичные отчеты** по затратам и доходам
- **Аналитика эффективности** франшиз
- **API для интеграции** с внешними системами

## 🚀 Быстрый старт

### 1. Запуск всех серверов
```bash
# Windows
start_servers.bat

# Linux/Mac
./start_servers.sh
```

### 2. Ручной запуск серверов
```bash
# Основной блокчейн сервер (порт 3000)
cargo run --bin blockchain_project

# Check API сервер (порт 8081)
cargo run --example check_api_demo

# Transparency API сервер (порт 8082)
cargo run --example transparency_demo
```

### 3. Открытие веб-интерфейсов
- **Главная страница**: `index.html`
- **Панель владельца**: `owner_dashboard.html`
- **Панель франчайзи**: `franchise_dashboard.html`
- **Панель прозрачности**: `transparency_dashboard.html`

## 📱 Мобильное приложение

### Установка зависимостей
```bash
cd mobile_app
npm install
```

### Запуск приложения
```bash
# Android
npx react-native run-android

# iOS
npx react-native run-ios
```

## 🧪 Тестирование

### Запуск всех тестов
```bash
cargo test
```

### Запуск конкретных тестов
```bash
# Unit тесты
cargo test --lib

# Интеграционные тесты
cargo test --test integration_tests

# Property тесты
cargo test --test property_tests
```

## 📚 API документация

### Основной API (порт 3000)
- **GET** `/api/status` - Статус системы
- **POST** `/api/transactions` - Создание транзакций
- **GET** `/api/balance/{address}` - Баланс кошелька

### Check API (порт 8081)
- **POST** `/api/checks/generate` - Генерация чека
- **POST** `/api/checks/claim` - Получение чека
- **GET** `/api/checks/{id}` - Информация о чеке

### Transparency API (порт 8082)
- **GET** `/api/reports` - Список отчетов
- **POST** `/api/reports/generate` - Генерация отчета
- **GET** `/api/reports/{id}` - Получение отчета

## 🏗️ Архитектура

### Основные модули
- **`src/new_tokenomics.rs`** - Токеномика ST/UT токенов
- **`src/kyc_aml.rs`** - KYC/AML проверки
- **`src/governance_dao.rs`** - DAO управление
- **`src/viewer_arm.rs`** - Интеграция со стримингом
- **`src/check_generation.rs`** - Генерация чеков
- **`src/anonymous_wallets.rs`** - Анонимные кошельки
- **`src/transparency_reporting.rs`** - Отчетность

### Веб-интерфейсы
- **`index.html`** - Главная страница
- **`owner_dashboard.html`** - Панель владельца
- **`franchise_dashboard.html`** - Панель франчайзи
- **`transparency_dashboard.html`** - Панель прозрачности

### Мобильное приложение
- **`mobile_app/App.tsx`** - Главный компонент
- **`mobile_app/src/screens/`** - Экраны приложения
- **`mobile_app/src/contexts/`** - Контексты состояния

## 🔧 Конфигурация

### Переменные окружения
```bash
# База данных
DATABASE_URL=postgresql://user:password@localhost/thehotpotspot

# API ключи
TWITCH_CLIENT_ID=your_twitch_client_id
YOUTUBE_API_KEY=your_youtube_api_key

# Блокчейн
BLOCKCHAIN_NETWORK=testnet
```

## 📈 Мониторинг

### Логи
```bash
# Просмотр логов
tail -f logs/blockchain.log
tail -f logs/api.log
```

### Метрики
- **Производительность**: Время обработки транзакций
- **Безопасность**: Количество KYC проверок
- **Использование**: Активные пользователи и транзакции

## 🤝 Участие в разработке

### Установка зависимостей
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (для мобильного приложения)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
```

### Сборка проекта
```bash
cargo build --release
```

## 📄 Лицензия

MIT License - см. файл [LICENSE](LICENSE)

## 🆘 Поддержка

- **Issues**: [GitHub Issues](https://github.com/Soyuses/thehotpotspot/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Soyuses/thehotpotspot/discussions)
- **Email**: support@thehotpotspot.com

## 🎯 Roadmap

### Версия 1.1
- [ ] Интеграция с дополнительными стриминговыми платформами
- [ ] Расширенная аналитика
- [ ] Мобильные уведомления

### Версия 1.2
- [ ] Интеграция с внешними платежными системами
- [ ] Многоязычная поддержка
- [ ] Расширенные KYC проверки

---

**Версия**: 1.0.0  
**Статус**: Production Ready  
**Последнее обновление**: $(date)