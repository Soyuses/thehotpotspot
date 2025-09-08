# 🏪 FranchiseToken Smart Contract

Смарт-контракт для франшизной сети фудтраков в Грузии на блокчейне Polygon.

## 📋 Описание

FranchiseToken - это ERC20 токен с автоматической эмиссией при продажах и распределением токенов между участниками сети согласно бизнес-логике франшизы.

## 🎯 Основные функции

### 1. **Токеномика**
- **SCALE = 100** - 1 токен = 100 subunits
- **OWNER точки**: 51% владельцу, 49% покупателю
- **FRANCHISE точки**: 48% франчайзи, 3% роялти, 49% покупателю
- **Генезис**: 1 токен (100 subunits) для основателя

### 2. **Управление нодами**
```solidity
function registerNode(address nodeOwner, NodeType nodeType, string memory city)
function setNodeActive(uint256 nodeId, bool active)
function getNodeInfo(uint256 nodeId)
```

### 3. **Запись продаж**
```solidity
function recordSale(
    uint256 nodeId,
    string memory saleId,
    uint256 priceGEL,
    address buyer,
    string memory buyerMeta,
    string memory posId
)
```

### 4. **Управление POS системами**
```solidity
function whitelistPOS(address posAddress, bool whitelisted)
```

### 5. **Чек-адреса**
```solidity
function createCheckAddress(string memory saleId)
```

## 🚀 Развертывание

### Требования
- Node.js 16+
- Hardhat
- MetaMask
- Polygon RPC URL

### Установка
```bash
npm install
npm install @openzeppelin/contracts
```

### Компиляция
```bash
npx hardhat compile
```

### Тестирование
```bash
npx hardhat test
```

### Развертывание на Polygon
```bash
# Настройте .env файл с PRIVATE_KEY и POLYGON_RPC_URL
npx hardhat run scripts/deploy.js --network polygon
```

## 📊 API методы

### Основные функции
- `registerNode()` - Регистрация новой ноды
- `recordSale()` - Запись продажи с автоматической эмиссией
- `whitelistPOS()` - Управление POS системами
- `createCheckAddress()` - Создание чек-адреса

### Информационные функции
- `getNodeInfo()` - Информация о ноде
- `getNetworkStats()` - Статистика сети
- `getWalletBalance()` - Баланс кошелька
- `totalSupply()` - Общее количество токенов

## 🔒 Безопасность

### Модификаторы
- `onlyOwner` - Только владелец контракта
- `onlyWhitelistedPOS` - Только whitelisted POS системы
- `whenNotPaused` - Контракт не на паузе
- `nonReentrant` - Защита от reentrancy атак

### Функции безопасности
- `pause()` - Экстренная остановка
- `unpause()` - Возобновление работы
- `setNodeActive()` - Управление активностью нод

## 📈 События

```solidity
event NodeRegistered(uint256 indexed nodeId, address indexed owner, NodeType nodeType, string city);
event SaleRecorded(uint256 indexed nodeId, string indexed saleId, address indexed buyer, uint256 priceGEL);
event TokensMinted(string indexed saleId, uint256 mintedUnits, uint256 ownerUnits, uint256 buyerUnits, uint256 royaltyUnits);
event CheckAddressCreated(string indexed saleId, address indexed checkAddress);
event POSWhitelisted(address indexed posAddress, bool whitelisted);
```

## 🔄 Интеграция с POS

### Пример интеграции
```javascript
// 1. Регистрация POS системы
await franchiseToken.whitelistPOS(posAddress, true);

// 2. Запись продажи
await franchiseToken.recordSale(
    nodeId,
    saleId,
    priceGEL,
    buyerAddress,
    buyerMeta,
    posId
);

// 3. Проверка баланса
const balance = await franchiseToken.getWalletBalance(walletAddress);
```

## 📋 Структуры данных

### Node
```solidity
struct Node {
    address owner;
    NodeType nodeType;
    string city;
    bool active;
    uint256 registeredAt;
    uint256 salesCount;
    uint256 totalRevenue;
}
```

### Sale
```solidity
struct Sale {
    uint256 nodeId;
    string saleId;
    uint256 timestamp;
    uint256 priceGEL;
    address buyer;
    string buyerMeta;
    string posId;
    bool processed;
}
```

### TokenMinting
```solidity
struct TokenMinting {
    uint256 saleId;
    uint256 mintedUnits;
    uint256 ownerUnits;
    uint256 buyerUnits;
    uint256 royaltyUnits;
    uint256 timestamp;
}
```

## 🌐 Сеть Polygon

### Преимущества Polygon
- **Низкие комиссии** - ~$0.01 за транзакцию
- **Быстрые транзакции** - ~2 секунды
- **Совместимость с Ethereum** - те же инструменты
- **Экологичность** - Proof of Stake

### Настройка сети
```javascript
// hardhat.config.js
module.exports = {
  networks: {
    polygon: {
      url: process.env.POLYGON_RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
      gasPrice: 30000000000, // 30 gwei
    }
  }
};
```

## 📊 Мониторинг

### Индексация событий
Используйте The Graph для индексации событий:
```graphql
query {
  sales(first: 10, orderBy: timestamp, orderDirection: desc) {
    id
    nodeId
    saleId
    priceGEL
    buyer
    timestamp
  }
}
```

### Аналитика
- **Total Sales** - общее количество продаж
- **Total Revenue** - общий доход в GEL
- **Token Distribution** - распределение токенов
- **Node Performance** - производительность нод

## 🔧 Обновления

### Proxy Pattern
Контракт поддерживает обновления через OpenZeppelin Proxy:
```solidity
// Развертывание через Proxy
const proxy = await upgrades.deployProxy(FranchiseToken, []);
```

### Версионирование
- **v1.0** - Базовая функциональность
- **v1.1** - Добавление аналитики
- **v2.0** - Интеграция с IPFS

## 📞 Поддержка

Для вопросов и поддержки:
- **GitHub Issues** - баг-репорты и предложения
- **Discord** - сообщество разработчиков
- **Email** - support@thehotpotspot.ge

## 📄 Лицензия

MIT License - см. файл LICENSE для деталей.
