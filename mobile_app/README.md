# The Hot Pot Spot Mobile Application

Мобильное приложение для The Hot Pot Spot - платформы для управления токенами и участия в DAO.

## Возможности

- **Аутентификация**: Вход по номеру телефона и регистрация с KYC требованиями
- **Кошелек**: Управление Security (ST) и Utility (UT) токенами
- **Сканирование QR-кодов**: Активация чеков для получения токенов
- **Меню**: Просмотр блюд и оформление заказов
- **Голосование**: Участие в DAO голосованиях
- **Профиль**: Управление личной информацией и KYC статусом

## Технологии

- **React Native 0.72.6**: Кроссплатформенная разработка
- **React Navigation**: Навигация между экранами
- **React Native Paper**: Material Design компоненты
- **React Query**: Управление состоянием и кэширование
- **AsyncStorage**: Локальное хранение данных
- **React Native Camera**: Сканирование QR-кодов

## Установка

### Предварительные требования

- Node.js 18+
- React Native CLI
- Android Studio (для Android)
- Xcode (для iOS)

### Установка зависимостей

```bash
cd mobile_app
npm install
```

### Android

```bash
# Запуск Metro bundler
npm start

# Запуск на Android
npm run android
```

### iOS

```bash
# Установка iOS зависимостей
cd ios && pod install && cd ..

# Запуск на iOS
npm run ios
```

## Структура проекта

```
mobile_app/
├── src/
│   ├── screens/           # Экраны приложения
│   │   ├── LoginScreen.tsx
│   │   ├── RegisterScreen.tsx
│   │   ├── QRScannerScreen.tsx
│   │   ├── WalletScreen.tsx
│   │   ├── MenuScreen.tsx
│   │   ├── VotingScreen.tsx
│   │   ├── ProfileScreen.tsx
│   │   └── CheckClaimScreen.tsx
│   ├── contexts/          # React Contexts
│   │   ├── AuthContext.tsx
│   │   ├── WalletContext.tsx
│   │   └── ThemeContext.tsx
│   ├── services/          # API сервисы
│   │   └── api.ts
│   └── theme/             # Тема приложения
│       └── theme.ts
├── App.tsx                # Главный компонент
├── package.json           # Зависимости
└── README.md             # Документация
```

## API Endpoints

Приложение взаимодействует с следующими API endpoints:

### Аутентификация
- `POST /auth/login` - Вход в систему
- `POST /auth/register` - Регистрация
- `POST /auth/verify-phone` - Подтверждение телефона
- `GET /auth/kyc-status` - Статус KYC

### Кошелек
- `GET /wallet/balance` - Баланс токенов
- `GET /wallet/transactions` - История транзакций
- `POST /wallet/claim-check` - Активация чека
- `POST /wallet/transfer` - Перевод токенов

### Меню
- `GET /menu` - Список блюд
- `GET /menu/{id}` - Детали блюда
- `POST /orders` - Оформление заказа

### Голосование
- `GET /voting/proposals` - Список предложений
- `GET /voting/proposals/{id}` - Детали предложения
- `POST /voting/vote` - Голосование
- `GET /voting/history` - История голосований

### Стриминг
- `GET /streaming/streams` - Список стримов
- `POST /streaming/sessions` - Начало сессии
- `DELETE /streaming/sessions/{id}` - Завершение сессии
- `POST /streaming/activity` - Запись активности

## Конфигурация

### API URL

Измените `API_BASE_URL` в `src/services/api.ts`:

```typescript
const API_BASE_URL = 'https://your-api-domain.com';
```

### Тема

Настройте цвета и стили в `src/theme/theme.ts`:

```typescript
export const theme = {
  colors: {
    primary: '#ff6b6b',
    secondary: '#4facfe',
    // ... другие цвета
  },
};
```

## Сборка

### Android

```bash
npm run build:android
```

### iOS

```bash
npm run build:ios
```

## Тестирование

```bash
# Запуск тестов
npm test

# Линтинг
npm run lint
```

## Развертывание

### Android (Google Play Store)

1. Создайте подписанный APK:
```bash
cd android
./gradlew assembleRelease
```

2. Загрузите APK в Google Play Console

### iOS (App Store)

1. Откройте проект в Xcode:
```bash
open ios/TheHotPotSpot.xcworkspace
```

2. Настройте подписание и загрузите в App Store Connect

## Безопасность

- Все API запросы используют HTTPS
- Токены аутентификации хранятся в AsyncStorage
- KYC данные шифруются перед отправкой
- QR-коды валидируются на клиенте и сервере

## Поддержка

Для получения поддержки:
- Email: support@thehotpotspot.com
- Telegram: @thehotpotspot_support
- GitHub Issues: [Создать issue](https://github.com/Soyuses/thehotpotspot/issues)

## Лицензия

MIT License - см. файл LICENSE для деталей.