# 🚀 ПЛАН ОПТИМИЗАЦИИ THEHOTPOTSPOT

## 📊 АНАЛИЗ ТЕКУЩЕГО СОСТОЯНИЯ

### ✅ Что уже есть:
- **Backend на Rust** с P2P, API, веб-интерфейсами
- **HTML интерфейсы:** owner/franchise/wallet
- **Solidity контракты** в папке contracts/
- **Токеномика 1:1** (недавно обновлена)
- **Демо-режимы** и тестовые сценарии
- **Система видеонаблюдения** интегрирована

### ⚠️ Критические проблемы:
- **Валюта не определена** - используется f64 без указания GEL
- **Float арифметика** - нет integer subunits
- **Нет регуляторных реестров** для GSSS/CSD
- **Приватные ключи в чеках** - небезопасно
- **Нет persistence** - только демо-режим
- **Нет KYC/AML** интеграции

---

## 🎯 ПЛАН ОПТИМИЗАЦИИ (по приоритетам)

### 🔥 **ПРИОРИТЕТ 1: Регуляторная и финансовая правка (КРИТИЧНО)**

#### A. Конфигурация и константы
- [ ] Добавить константу `INITIAL_THP_PRICE_GEL = 5.0`
- [ ] Заменить все `price: f64` на `price_gel: u128` (subunits)
- [ ] Добавить `currency: "GEL"` в метаданные
- [ ] Создать `SCALE = 100` для subunits

#### B. Токеномика и проспект
- [ ] Обновить все расчеты на integer subunits
- [ ] Создать проспект эмиссии с ценой 5 GEL
- [ ] Добавить экспорт реестров (CSV/JSON)
- [ ] Интегрировать с GSSS/CSD форматами

### 🔧 **ПРИОРИТЕТ 2: Архитектура токенов и смарт-контрактов**

#### C. Persistence и DB миграции
- [ ] Добавить PostgreSQL вместо демо-режима
- [ ] Создать миграции для nodes, sales, wallets
- [ ] Добавить схемы для mintings, transfers

#### D. Relayer и idempotency
- [ ] Создать relayer сервис (Rust)
- [ ] Добавить Redis для idempotency
- [ ] Реализовать anti-replay protection
- [ ] Настроить gas strategy для L2

### 🔐 **ПРИОРИТЕТ 3: Безопасность и доступы**

#### E. HD-wallet/Checkwallet
- [ ] Заменить приватные ключи на HD-деривацию
- [ ] Создать HSM/secure vault для seed
- [ ] Добавить API для проверки принадлежности
- [ ] Реализовать CheckWallet_{node}_{seq}

#### F. KYC/AML и roles
- [ ] Интеграция с KYC провайдером (mock)
- [ ] Identity registry в базе
- [ ] RBAC + ACL: MASTER_OWNER, REGISTRAR, POS_RELAY, AUDITOR
- [ ] Блокировка mint/transfer до KYC

### 🏗️ **ПРИОРИТЕТ 4: Надежность и масштабирование**

#### G. Smart-contracts (Solidity -> target chain)
- [ ] Обновить контракты на integer subunits
- [ ] Добавить EVM-совместимость (Moonbeam/Polygon)
- [ ] Создать bridge для Substrate/CosmWasm
- [ ] Перенести критическую логику on-chain

#### H. Tests, CI, audit
- [ ] Unit/Integration tests
- [ ] CI: Github Actions с cargo test, clippy, fmt
- [ ] Security scan
- [ ] Prometheus/Grafana метрики

#### I. Reports for regulator
- [ ] Экспорт реестров в CSV/JSON
- [ ] Фискальные чеки с sale_id, check_address
- [ ] OpenAPI/Swagger документация
- [ ] Версионирование API

---

## 📋 TODO CHECKLIST ДЛЯ CURSOR

### 🔥 **НЕМЕДЛЕННО (сегодня):**

1. **Создать константы валюты:**
   ```rust
   // src/config.rs
   pub const INITIAL_THP_PRICE_GEL: u128 = 500; // 5.00 GEL в subunits
   pub const SCALE: u128 = 100;
   pub const CURRENCY: &str = "GEL";
   ```

2. **Заменить float на integer:**
   - Найти все `f64` в токеномике
   - Заменить на `u128` subunits
   - Обновить арифметику

3. **Добавить метаданные валюты:**
   - В структуры Order, Transaction, Check
   - В API ответы
   - В веб-интерфейсы

### 🔧 **НА ЭТОЙ НЕДЕЛЕ:**

4. **Создать PostgreSQL схему:**
   ```sql
   -- migrations/001_initial.sql
   CREATE TABLE nodes (...);
   CREATE TABLE sales (...);
   CREATE TABLE wallets (...);
   ```

5. **Обновить смарт-контракты:**
   - Заменить float на uint256
   - Добавить GEL метаданные
   - Обновить токеномику

6. **Создать relayer сервис:**
   - Axum web server
   - Redis для idempotency
   - HSM интеграция

### 🎯 **В ТЕЧЕНИЕ МЕСЯЦА:**

7. **HD-wallet система:**
   - BIP32/BIP44 деривация
   - Secure seed storage
   - Check address generation

8. **KYC/AML интеграция:**
   - Identity registry
   - RBAC система
   - Compliance checks

9. **Полная документация:**
   - OpenAPI spec
   - Регуляторные реестры
   - Deployment guide

---

## 🚨 КРИТИЧЕСКИЕ РИСКИ И МИТИГАЦИИ

### ⚠️ **Риск 1: Регуляторное переквалифицирование**
- **Проблема:** Utility токены могут стать security
- **Митигация:** Четкое разделение, KYC, документы

### ⚠️ **Риск 2: Утечка ключей чек-адресов**
- **Проблема:** Приватные ключи в чеках
- **Митигация:** HD-wallet + HSM, только публичные адреса

### ⚠️ **Риск 3: Форк/чарджбэки**
- **Проблема:** Дублирование транзакций
- **Митигация:** Idempotency, on-chain finality

---

## 📊 МЕТРИКИ УСПЕХА

### ✅ **Критерии готовности:**
- [ ] Все цены в GEL subunits (u128)
- [ ] Нет float арифметики в критических местах
- [ ] PostgreSQL persistence работает
- [ ] HD-wallet система активна
- [ ] KYC интеграция готова
- [ ] Регуляторные реестры экспортируются
- [ ] CI/CD pipeline работает
- [ ] Security audit пройден

### 📈 **KPI:**
- **Время отклика API:** < 100ms
- **Uptime:** > 99.9%
- **Security score:** A+
- **Compliance:** 100% с грузинским законодательством

---

## 🎉 ОЖИДАЕМЫЙ РЕЗУЛЬТАТ

После завершения оптимизации проект будет:
- ✅ **Соответствовать** грузинскому законодательству
- ✅ **Готов к продакшену** с промышленным уровнем надежности
- ✅ **Масштабируемым** для роста сети
- ✅ **Безопасным** с enterprise-grade защитой
- ✅ **Прозрачным** для регуляторов и инвесторов

**🚀 ГОТОВ К ЗАПУСКУ В ПРОДАКШЕН!**
