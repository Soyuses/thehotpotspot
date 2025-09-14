# Техническое задание: Исправление GitHub Actions Security Scanning

## 🎯 **Цель**
Исправить GitHub Actions workflow для корректной работы сканирования безопасности и настроить уведомления на email `gildcgir@gmail.com`.

## 🚨 **Проблемы**

### 1. **Workflow не отображается в Actions**
- Workflow `security.yml` не запускается корректно
- Отсутствует в списке Actions на GitHub
- Возможные причины:
  - Неправильная конфигурация триггеров
  - Ошибки в синтаксисе YAML
  - Проблемы с правами доступа

### 2. **Отсутствие уведомлений**
- Нет настройки email уведомлений
- Workflow не отправляет результаты на `gildcgir@gmail.com`

## 🔧 **Техническое решение**

### **Этап 1: Исправление workflow конфигурации**

#### 1.1 **Обновить триггеры в `.github/workflows/security.yml`**
```yaml
on:
  schedule:
    # Запускаем сканирование безопасности каждый день в 2:00 UTC
    - cron: '0 2 * * *'
  push:
    branches: [ master, main, develop ]  # Добавить master
  pull_request:
    branches: [ master, main, develop ]  # Добавить master
  workflow_dispatch:
```

#### 1.2 **Исправить проблемы с инструментами**
- Добавить проверки существования инструментов
- Использовать более стабильные версии
- Добавить fallback для отсутствующих инструментов

#### 1.3 **Упростить workflow для начального тестирования**
```yaml
# Создать упрощенную версию для тестирования
name: Security Scan (Simplified)

on:
  push:
    branches: [ master ]
  workflow_dispatch:

jobs:
  basic-security-check:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Basic security check
      run: |
        echo "🔍 Running basic security checks..."
        cargo check --release
        echo "✅ Basic security check completed"
```

### **Этап 2: Настройка уведомлений**

#### 2.1 **Добавить email уведомления в workflow**
```yaml
  # Добавить в конец workflow
  notify-on-completion:
    name: Send Email Notification
    runs-on: ubuntu-latest
    needs: [security-report]
    if: always()
    steps:
    - name: Send notification email
      uses: dawidd6/action-send-mail@v3
      with:
        server_address: smtp.gmail.com
        server_port: 587
        username: ${{ secrets.EMAIL_USERNAME }}
        password: ${{ secrets.EMAIL_PASSWORD }}
        subject: "Security Scan Results - TheHotPotSpot"
        to: gildcgir@gmail.com
        from: GitHub Actions
        body: |
          Security scan completed for commit ${{ github.sha }}
          
          Results: ${{ needs.security-report.result }}
          
          View details: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}
```

#### 2.2 **Настроить GitHub Secrets**
Добавить в настройки репозитория:
- `EMAIL_USERNAME`: email для отправки
- `EMAIL_PASSWORD`: пароль приложения Gmail

### **Этап 3: Создание конфигурационных файлов**

#### 3.1 **Создать `deny.toml` для cargo-deny**
```toml
[advisories]
# Проверка на уязвимости
vulnerability = "deny"
unmaintained = "warn"
unsound = "deny"
notice = "warn"

[licenses]
# Разрешенные лицензии
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
    "Unlicense"
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0"
]
```

#### 3.2 **Создать `.github/dependabot.yml`**
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
```

## 📋 **План реализации**

### **Шаг 1: Создание упрощенного workflow**
1. Создать `security-simple.yml` для тестирования
2. Проверить запуск на push в master
3. Убедиться, что workflow появляется в Actions

### **Шаг 2: Постепенное добавление функций**
1. Добавить базовые проверки безопасности
2. Добавить проверку зависимостей
3. Добавить статический анализ кода

### **Шаг 3: Настройка уведомлений**
1. Настроить Gmail App Password
2. Добавить secrets в GitHub
3. Протестировать отправку email

### **Шаг 4: Полная конфигурация**
1. Восстановить полный workflow
2. Добавить все инструменты безопасности
3. Настроить детальные отчеты

## 🛠️ **Команды для выполнения**

### **1. Создать упрощенный workflow**
```bash
# Создать файл .github/workflows/security-simple.yml
```

### **2. Настроить secrets в GitHub**
```bash
# Через веб-интерфейс GitHub:
# Settings → Secrets and variables → Actions → New repository secret
```

### **3. Тестирование**
```bash
# Сделать тестовый коммит
git add .
git commit -m "test: trigger security workflow"
git push origin master
```

## 📧 **Настройка Gmail для уведомлений**

### **1. Включить 2FA в Gmail**
- Настройки → Безопасность → Двухэтапная аутентификация

### **2. Создать App Password**
- Настройки → Безопасность → Пароли приложений
- Создать пароль для "GitHub Actions"

### **3. Добавить в GitHub Secrets**
- `EMAIL_USERNAME`: `gildcgir@gmail.com`
- `EMAIL_PASSWORD`: `<app-password>`

## 🎯 **Ожидаемый результат**

### **После исправления:**
1. ✅ Workflow `Security Scanning` отображается в Actions
2. ✅ Автоматический запуск при push в master
3. ✅ Email уведомления на `gildcgir@gmail.com`
4. ✅ Детальные отчеты о безопасности
5. ✅ Интеграция с GitHub Security tab

### **Уведомления будут содержать:**
- Статус выполнения (успех/ошибка)
- Ссылку на детальный отчет
- Краткое резюме найденных проблем
- Рекомендации по исправлению

## 🔍 **Диагностика проблем**

### **Если workflow не запускается:**
1. Проверить синтаксис YAML
2. Убедиться, что файл в правильной директории
3. Проверить права доступа к репозиторию

### **Если нет email уведомлений:**
1. Проверить настройки Gmail
2. Убедиться в корректности secrets
3. Проверить логи workflow

## 📊 **Метрики успеха**

- [ ] Workflow запускается автоматически
- [ ] Email уведомления приходят на `gildcgir@gmail.com`
- [ ] Отчеты содержат полезную информацию
- [ ] Интеграция с GitHub Security работает
- [ ] Время выполнения workflow < 10 минут

---

**Приоритет:** Высокий  
**Срок выполнения:** 1-2 дня  
**Ответственный:** DevOps/Backend Developer  
**Email для уведомлений:** gildcgir@gmail.com
