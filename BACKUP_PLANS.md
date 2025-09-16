# Резервные планы для The Hot Pot Spot

## 🚨 План восстановления после сбоев

### 1. Сбой основного сервера
**Симптомы**: Недоступность API, потеря данных, остановка сервисов

**Действия**:
1. **Немедленные меры** (0-5 минут):
   - Проверить статус сервера: `systemctl status blockchain_project`
   - Перезапустить сервис: `systemctl restart blockchain_project`
   - Проверить логи: `journalctl -u blockchain_project -f`

2. **Если перезапуск не помог** (5-15 минут):
   - Переключиться на резервный сервер
   - Обновить DNS записи на резервный IP
   - Уведомить пользователей о временных неполадках

3. **Восстановление данных** (15-60 минут):
   - Восстановить базу данных из последнего бэкапа
   - Синхронизировать блокчейн с другими нодами
   - Проверить целостность данных

4. **Полное восстановление** (1-4 часа):
   - Развернуть новую ноду с нуля
   - Восстановить все конфигурации
   - Протестировать все функции

### 2. Сбой базы данных
**Симптомы**: Ошибки подключения к БД, потеря транзакций

**Действия**:
1. **Диагностика**:
   ```bash
   # Проверить статус PostgreSQL
   systemctl status postgresql
   
   # Проверить подключения
   psql -h localhost -U blockchain_user -d thehotpotspot -c "SELECT 1;"
   
   # Проверить место на диске
   df -h
   ```

2. **Восстановление**:
   ```bash
   # Остановить приложение
   systemctl stop blockchain_project
   
   # Восстановить из бэкапа
   pg_restore -h localhost -U blockchain_user -d thehotpotspot /backup/latest.dump
   
   # Перезапустить приложение
   systemctl start blockchain_project
   ```

3. **Проверка**:
   - Проверить целостность данных
   - Запустить тесты
   - Уведомить о восстановлении

### 3. Сбой блокчейн сети
**Симптомы**: Ноды не синхронизируются, потеря консенсуса

**Действия**:
1. **Диагностика сети**:
   ```bash
   # Проверить статус нод
   curl http://node1:3000/api/status
   curl http://node2:3000/api/status
   curl http://node3:3000/api/status
   
   # Проверить синхронизацию
   curl http://localhost:3000/api/blockchain/status
   ```

2. **Восстановление консенсуса**:
   - Перезапустить все ноды в правильном порядке
   - Дождаться синхронизации
   - Проверить целостность блокчейна

3. **Если консенсус потерян**:
   - Использовать последний валидный блок
   - Пересоздать блокчейн с нуля
   - Восстановить состояние из бэкапа

## 🔄 План резервного копирования

### 1. Автоматические бэкапы
```bash
#!/bin/bash
# backup_script.sh

# Создание бэкапа базы данных
pg_dump -h localhost -U blockchain_user thehotpotspot > /backup/db_$(date +%Y%m%d_%H%M%S).sql

# Создание бэкапа конфигураций
tar -czf /backup/config_$(date +%Y%m%d_%H%M%S).tar.gz /etc/blockchain_project/

# Создание бэкапа логов
tar -czf /backup/logs_$(date +%Y%m%d_%H%M%S).tar.gz /var/log/blockchain_project/

# Удаление старых бэкапов (старше 30 дней)
find /backup -name "*.sql" -mtime +30 -delete
find /backup -name "*.tar.gz" -mtime +30 -delete

# Отправка уведомления
echo "Backup completed at $(date)" | mail -s "Backup Status" admin@thehotpotspot.com
```

### 2. Расписание бэкапов
```bash
# Добавить в crontab
# Ежедневные бэкапы в 2:00
0 2 * * * /opt/backup_script.sh

# Еженедельные полные бэкапы в воскресенье в 1:00
0 1 * * 0 /opt/full_backup_script.sh

# Ежемесячные архивы в 1-е число в 0:00
0 0 1 * * /opt/monthly_archive_script.sh
```

### 3. Хранение бэкапов
- **Локально**: `/backup/` (7 дней)
- **Облако**: AWS S3, Google Cloud Storage (30 дней)
- **Внешний сервер**: FTP/SFTP (90 дней)
- **Физические носители**: Ежемесячно

## 🌐 План резервного хостинга

### 1. Альтернативные хостинг провайдеры

#### Railway (Основной)
- **Преимущества**: Автоматические деплои, SSL, мониторинг
- **Недостатки**: Ограниченный бесплатный план
- **Резервный план**: Переход на платный план

#### Render (Резервный #1)
- **Настройка**:
  ```bash
  # Создать новый проект
  render create blockchain-project-backup
  
  # Настроить переменные окружения
  render env set DATABASE_URL=$DATABASE_URL
  render env set BLOCKCHAIN_NETWORK=mainnet
  
  # Деплой
  git push render main
  ```

#### Fly.io (Резервный #2)
- **Настройка**:
  ```bash
  # Установить flyctl
  curl -L https://fly.io/install.sh | sh
  
  # Создать приложение
  fly launch
  
  # Настроить базу данных
  fly postgres create
  
  # Деплой
  fly deploy
  ```

#### DigitalOcean (Резервный #3)
- **Настройка**:
  ```bash
  # Создать дроплет
  doctl compute droplet create blockchain-backup \
    --image ubuntu-22-04-x64 \
    --size s-2vcpu-4gb \
    --region nyc1
  
  # Настроить сервер
  ssh root@<droplet-ip>
  apt update && apt install -y docker.io docker-compose
  
  # Развернуть приложение
  git clone https://github.com/Soyuses/thehotpotspot.git
  cd thehotpotspot
  docker-compose up -d
  ```

### 2. План переключения на резервный хостинг

#### Автоматическое переключение
```bash
#!/bin/bash
# failover_script.sh

# Проверить доступность основного сервера
if ! curl -f http://main-server:3000/api/status; then
    echo "Main server is down, switching to backup"
    
    # Обновить DNS записи
    aws route53 change-resource-record-sets \
        --hosted-zone-id Z123456789 \
        --change-batch file://dns-change.json
    
    # Уведомить команду
    curl -X POST https://hooks.slack.com/services/... \
        -d '{"text":"Switched to backup server due to main server failure"}'
    
    # Запустить резервный сервер
    ssh backup-server "systemctl start blockchain_project"
fi
```

#### Ручное переключение
1. **Обновить DNS записи**:
   - Изменить A-запись с основного IP на резервный
   - TTL установить в 300 секунд для быстрого переключения

2. **Запустить резервный сервер**:
   ```bash
   # На резервном сервере
   systemctl start blockchain_project
   systemctl start postgresql
   systemctl start nginx
   ```

3. **Проверить работоспособность**:
   - Тестировать все API эндпоинты
   - Проверить синхронизацию блокчейна
   - Уведомить пользователей

## 🔧 План восстановления оборудования

### 1. Сбой сервера
**Действия**:
1. **Диагностика**:
   - Проверить питание и подключение
   - Проверить статус жестких дисков
   - Проверить температуру процессора

2. **Временное решение**:
   - Переключиться на резервный сервер
   - Использовать облачные ресурсы
   - Масштабировать существующие серверы

3. **Замена оборудования**:
   - Заказать новое оборудование
   - Настроить с нуля
   - Восстановить данные из бэкапа

### 2. Сбой точки франшизы
**Действия**:
1. **Диагностика**:
   - Проверить подключение к интернету
   - Проверить работу камеры
   - Проверить работу принтера

2. **Временное решение**:
   - Использовать мобильное приложение
   - Ручной ввод заказов
   - Временное отключение от сети

3. **Восстановление**:
   - Заменить неисправное оборудование
   - Переустановить ПО
   - Синхронизировать с сетью

## 📱 План восстановления мобильного приложения

### 1. Сбой приложения
**Действия**:
1. **Диагностика**:
   - Проверить версию приложения
   - Проверить подключение к API
   - Проверить логи приложения

2. **Восстановление**:
   - Переустановить приложение
   - Очистить кэш и данные
   - Обновить до последней версии

3. **Резервные варианты**:
   - Использовать веб-версию
   - Использовать альтернативное приложение
   - Ручной ввод данных

### 2. Сбой API для мобильного приложения
**Действия**:
1. **Переключение на резервный API**:
   ```javascript
   // В мобильном приложении
   const API_ENDPOINTS = [
       'https://api1.thehotpotspot.com',
       'https://api2.thehotpotspot.com',
       'https://api3.thehotpotspot.com'
   ];
   
   async function callAPI(endpoint, data) {
       for (const url of API_ENDPOINTS) {
           try {
               const response = await fetch(url + endpoint, data);
               if (response.ok) return response;
           } catch (error) {
               console.log(`API ${url} failed, trying next...`);
           }
       }
       throw new Error('All API endpoints failed');
   }
   ```

2. **Офлайн режим**:
   - Кэшировать данные локально
   - Синхронизировать при восстановлении
   - Уведомить пользователя о режиме

## 🚨 План уведомлений

### 1. Автоматические уведомления
```bash
#!/bin/bash
# notification_script.sh

# Slack уведомления
send_slack_notification() {
    curl -X POST https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX \
        -H 'Content-type: application/json' \
        --data "{\"text\":\"$1\"}"
}

# Email уведомления
send_email_notification() {
    echo "$1" | mail -s "The Hot Pot Spot Alert" admin@thehotpotspot.com
}

# SMS уведомления (через Twilio)
send_sms_notification() {
    curl -X POST https://api.twilio.com/2010-04-01/Accounts/$TWILIO_ACCOUNT_SID/Messages.json \
        --data-urlencode "From=+1234567890" \
        --data-urlencode "To=+0987654321" \
        --data-urlencode "Body=$1" \
        -u $TWILIO_ACCOUNT_SID:$TWILIO_AUTH_TOKEN
}
```

### 2. Эскалация уведомлений
- **Уровень 1** (0-5 минут): Slack уведомление команде
- **Уровень 2** (5-15 минут): Email уведомление администратору
- **Уровень 3** (15-30 минут): SMS уведомление ответственному
- **Уровень 4** (30+ минут): Звонок ответственному

## 📊 План мониторинга

### 1. Мониторинг серверов
```bash
#!/bin/bash
# monitoring_script.sh

# Проверка доступности API
check_api() {
    if ! curl -f http://localhost:3000/api/status; then
        send_notification "API server is down"
        return 1
    fi
    return 0
}

# Проверка использования ресурсов
check_resources() {
    CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    MEMORY_USAGE=$(free | grep Mem | awk '{printf("%.2f"), $3/$2 * 100.0}')
    DISK_USAGE=$(df -h / | awk 'NR==2{print $5}' | cut -d'%' -f1)
    
    if (( $(echo "$CPU_USAGE > 80" | bc -l) )); then
        send_notification "High CPU usage: ${CPU_USAGE}%"
    fi
    
    if (( $(echo "$MEMORY_USAGE > 80" | bc -l) )); then
        send_notification "High memory usage: ${MEMORY_USAGE}%"
    fi
    
    if [ "$DISK_USAGE" -gt 80 ]; then
        send_notification "High disk usage: ${DISK_USAGE}%"
    fi
}

# Проверка базы данных
check_database() {
    if ! psql -h localhost -U blockchain_user -d thehotpotspot -c "SELECT 1;" > /dev/null 2>&1; then
        send_notification "Database connection failed"
        return 1
    fi
    return 0
}

# Основной цикл мониторинга
while true; do
    check_api
    check_resources
    check_database
    sleep 60
done
```

### 2. Мониторинг приложения
- **Uptime**: Проверка каждые 30 секунд
- **Response time**: Измерение времени ответа API
- **Error rate**: Подсчет ошибок в логах
- **User activity**: Мониторинг активных пользователей

## 🔐 План безопасности

### 1. Инциденты безопасности
**Действия**:
1. **Немедленные меры**:
   - Изолировать затронутые системы
   - Сменить пароли и ключи
   - Заблокировать подозрительные IP

2. **Расследование**:
   - Анализ логов
   - Определение масштаба инцидента
   - Документирование событий

3. **Восстановление**:
   - Устранение уязвимостей
   - Восстановление из чистых бэкапов
   - Тестирование системы

### 2. Резервные ключи и сертификаты
- **SSL сертификаты**: Резервные копии в безопасном месте
- **API ключи**: Ротация каждые 90 дней
- **Пароли**: Хранение в зашифрованном виде
- **Ключи шифрования**: Резервные копии в разных местах

## 📋 Чек-лист восстановления

### Быстрое восстановление (0-30 минут)
- [ ] Проверить статус всех сервисов
- [ ] Перезапустить неработающие сервисы
- [ ] Проверить подключение к базе данных
- [ ] Проверить доступность API
- [ ] Уведомить команду о статусе

### Полное восстановление (30 минут - 4 часа)
- [ ] Переключиться на резервный сервер
- [ ] Восстановить базу данных из бэкапа
- [ ] Синхронизировать блокчейн
- [ ] Протестировать все функции
- [ ] Уведомить пользователей о восстановлении

### Восстановление после катастрофы (4+ часов)
- [ ] Развернуть новую инфраструктуру
- [ ] Восстановить все данные
- [ ] Настроить мониторинг
- [ ] Провести полное тестирование
- [ ] Обновить документацию

---

**Дата создания**: $(date)  
**Версия**: 1.0.0  
**Статус**: Готов к использованию  
**Следующий пересмотр**: Через 3 месяца
