# The Hot Pot Spot - Frontend

Современный TypeScript фронтенд для ресторанной сети "The Hot Pot Spot" с блокчейн-технологиями.

## 🚀 Технологии

- **TypeScript** - типизированный JavaScript
- **Vite** - быстрый сборщик и dev-сервер
- **Axios** - HTTP клиент для API
- **CSS3** - современные стили с CSS Grid и Flexbox
- **ESLint** - линтер для TypeScript

## 📁 Структура проекта

```
frontend/
├── src/
│   ├── components/          # React-подобные компоненты
│   │   ├── MainPage.ts      # Главная страница
│   │   ├── OwnerDashboard.ts # Панель владельца
│   │   └── FranchiseDashboard.ts # Панель франчайзи
│   ├── services/            # API сервисы
│   │   └── api.ts          # Типизированный API клиент
│   ├── types/              # TypeScript типы
│   │   └── api.ts          # API типы
│   ├── utils/              # Утилиты
│   │   ├── format.ts       # Форматирование данных
│   │   └── validation.ts   # Валидация
│   ├── styles/             # CSS стили
│   │   ├── main.css        # Основные стили
│   │   ├── owner-dashboard.css
│   │   ├── franchise-dashboard.css
│   │   ├── transparency-dashboard.css
│   │   ├── video-dashboard.css
│   │   └── tablet-dashboard.css
│   ├── main.ts             # Точка входа
│   ├── owner.ts            # Панель владельца
│   ├── franchise.ts        # Панель франчайзи
│   ├── transparency.ts     # Панель прозрачности
│   ├── video.ts            # Видеонаблюдение
│   └── tablet.ts           # Планшет за столом
├── index.html              # Главная страница
├── owner.html              # Панель владельца
├── franchise.html          # Панель франчайзи
├── transparency.html       # Панель прозрачности
├── video.html              # Видеонаблюдение
├── tablet.html             # Планшет за столом
├── package.json            # Зависимости
├── tsconfig.json           # Конфигурация TypeScript
├── vite.config.ts          # Конфигурация Vite
└── .eslintrc.json          # Конфигурация ESLint
```

## 🛠️ Установка и запуск

### Предварительные требования

- Node.js 18+ 
- npm или yarn

### Установка зависимостей

```bash
cd frontend
npm install
```

### Разработка

```bash
# Запуск dev-сервера
npm run dev

# Проверка типов
npm run type-check

# Линтинг
npm run lint

# Исправление ошибок линтера
npm run lint:fix
```

### Сборка для продакшна

```bash
# Сборка проекта
npm run build

# Предварительный просмотр сборки
npm run preview
```

## 🎯 Основные функции

### 1. Главная страница (`index.html`)
- Обзор проекта и возможностей
- Статистика сети
- Навигация по панелям управления
- Превью меню

### 2. Панель владельца (`owner.html`)
- Мониторинг токеномики
- Управление франшизной сетью
- Конвертация UT в ST
- Учет затрат
- Управление стримами
- Управление изображениями
- Распределение невостребованных токенов

### 3. Панель франчайзи (`franchise.html`)
- Управление заказами
- Управление меню
- Статистика продаж
- Настройки ресторана

### 4. Панель прозрачности (`transparency.html`)
- Отчеты по эмиссии токенов
- Данные о держателях
- Финансовые отчеты
- Экспорт данных

### 5. Видеонаблюдение (`video.html`)
- Управление видеопотоками
- Интеграция с камерами
- YouTube стриминг
- Мониторинг кухни

### 6. Планшет за столом (`tablet.html`)
- Заказ еды
- Просмотр меню
- Прямая трансляция из кухни
- Информация о SPOT токенах
- Корзина заказов

## 🔧 API Интеграция

### Типизированный API клиент

```typescript
import { apiClient } from '@/services/api';

// Получение меню
const menu = await apiClient.getMenu();

// Создание заказа
const order = await apiClient.createOrder({
  customer_wallet: '0x1234...',
  items: [{ menu_item_id: '1', quantity: 2, price_subunits: 1500 }],
  delivery_time_minutes: 30
});

// Получение баланса кошелька
const balance = await apiClient.getWalletBalance('0x1234...');
```

### Типы данных

```typescript
import type { MenuItem, Order, UTHolder } from '@/types/api';

// Все API типы полностью типизированы
const menuItem: MenuItem = {
  id: '1',
  name: 'Хот-пот с говядиной',
  description: 'Острое блюдо с говядиной и овощами',
  price_subunits: 2500,
  // ... другие поля
};
```

## 🎨 Стилизация

### CSS переменные

```css
:root {
  --primary-color: #ff6b6b;
  --secondary-color: #4ecdc4;
  --accent-color: #45b7d1;
  --success-color: #96ceb4;
  --warning-color: #feca57;
  --error-color: #ff9ff3;
  --dark-color: #2c3e50;
  --light-color: #ecf0f1;
  --text-color: #2c3e50;
  --text-light: #7f8c8d;
  --border-color: #bdc3c7;
  --shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  --border-radius: 8px;
  --transition: all 0.3s ease;
}
```

### Адаптивный дизайн

- **Desktop**: Полнофункциональный интерфейс
- **Tablet**: Оптимизированная навигация
- **Mobile**: Упрощенный интерфейс

## 🔍 Утилиты

### Форматирование

```typescript
import { formatCurrency, formatTokens, formatDate } from '@/utils/format';

formatCurrency(25.50); // "25.50 GEL"
formatTokens(1000, 'ST'); // "1,000 THP"
formatDate(new Date()); // "15 декабря 2024, 14:30"
```

### Валидация

```typescript
import { isValidEmail, isValidWalletAddress, validateCreateOrder } from '@/utils/validation';

isValidEmail('user@example.com'); // true
isValidWalletAddress('0x1234...5678'); // true
validateCreateOrder(orderData); // string[]
```

## 🚀 Развертывание

### Локальная разработка

```bash
npm run dev
# Открыть http://localhost:3001
```

### Продакшн сборка

```bash
npm run build
# Файлы в папке dist/
```

### Интеграция с бэкендом

Фронтенд автоматически подключается к API серверу на `http://localhost:3000`. Для изменения URL:

```typescript
// В src/services/api.ts
const apiClient = new ApiClient('https://your-api-domain.com');
```

## 📱 Особенности планшета

### Планшет за столом

- **Интуитивный интерфейс** для заказа еды
- **Прямая трансляция** из кухни
- **Информация о SPOT токенах** с модальным окном
- **Корзина заказов** с возможностью изменения количества
- **Адаптивный дизайн** для разных размеров экранов

### SPOT токены

Планшет включает информационную систему о SPOT токенах:

- Как заработать SPOT
- Что с ними делать
- Участие в распределении невостребованных чеков

## 🔧 Разработка

### Добавление нового компонента

1. Создайте файл в `src/components/`
2. Экспортируйте класс компонента
3. Импортируйте в нужном HTML файле
4. Добавьте стили в `src/styles/`

### Добавление нового API метода

1. Добавьте тип в `src/types/api.ts`
2. Добавьте метод в `src/services/api.ts`
3. Используйте в компонентах

### Стилизация

- Используйте CSS переменные из `:root`
- Следуйте принципам адаптивного дизайна
- Добавляйте hover эффекты и переходы

## 🐛 Отладка

### Проверка типов

```bash
npm run type-check
```

### Линтинг

```bash
npm run lint
```

### DevTools

- Откройте DevTools в браузере
- Проверьте консоль на ошибки
- Используйте Network tab для отладки API

## 📄 Лицензия

MIT License - см. файл LICENSE для деталей.

## 🤝 Вклад в проект

1. Форкните репозиторий
2. Создайте ветку для фичи (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'Add amazing feature'`)
4. Отправьте в ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## 📞 Поддержка

При возникновении проблем:

1. Проверьте консоль браузера на ошибки
2. Убедитесь, что API сервер запущен
3. Проверьте типы TypeScript
4. Создайте issue в репозитории
