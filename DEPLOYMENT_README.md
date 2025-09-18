# Развертывание The Hot Pot Spot на Railway

## Быстрый старт

### 1. Установка Railway CLI

**Windows:**
```bash
npm install -g @railway/cli
```

**Linux/macOS:**
```bash
npm install -g @railway/cli
```

### 2. Авторизация в Railway

```bash
railway login
```

### 3. Автоматическое развертывание

**Windows:**
```bash
deploy-railway.bat
```

**Linux/macOS:**
```bash
chmod +x deploy-railway.sh
./deploy-railway.sh
```

## Ручное развертывание

### 1. Инициализация проекта

```bash
railway init
```

### 2. Сборка проекта

```bash
cargo build --release
```

### 3. Развертывание

```bash
railway up
```

## Настройка переменных окружения

После развертывания настройте переменные окружения в Railway dashboard:

1. Перейдите в Settings → Variables
2. Добавьте переменные из `railway-config.md`
3. Установите соответствующие значения

## Добавление PostgreSQL

1. В Railway dashboard нажмите "New Service"
2. Выберите "PostgreSQL"
3. Скопируйте DATABASE_URL из настроек PostgreSQL
4. Добавьте переменную окружения DATABASE_URL

## Проверка развертывания

После развертывания проверьте:

1. **Главная страница**: `https://your-app.railway.app/`
2. **API health**: `https://your-app.railway.app/api/health`
3. **Меню**: `https://your-app.railway.app/api/menu`

## Мониторинг

- **Логи**: Railway dashboard → Deployments → Logs
- **Метрики**: Порт 9090 (если включен)
- **Статус**: Railway dashboard → Status

## Масштабирование

### Горизонтальное масштабирование
1. Railway dashboard → Settings → Scaling
2. Увеличьте количество реплик
3. Настройте load balancer

### Вертикальное масштабирование
1. Railway dashboard → Settings → Resources
2. Увеличьте CPU/Memory
3. Мониторьте производительность

## Обновления

### Автоматические обновления
Railway автоматически развертывает изменения при push в main ветку.

### Ручные обновления
```bash
git push origin main
railway up
```

## Откат изменений

```bash
railway rollback
```

## Удаление проекта

```bash
railway delete
```

## Поддержка

- **Railway документация**: https://docs.railway.app/
- **Railway Discord**: https://discord.gg/railway
- **GitHub Issues**: Создайте issue в репозитории проекта

## Безопасность

### SSL/TLS
Railway автоматически предоставляет SSL сертификаты.

### Секреты
- Никогда не коммитьте секреты в код
- Используйте переменные окружения
- Регулярно ротируйте ключи

### Мониторинг
- Настройте алерты на ошибки
- Мониторьте логи
- Отслеживайте производительность

## Стоимость

Railway предоставляет:
- **Бесплатный план**: $5 кредитов в месяц
- **Pro план**: $20/месяц за проект
- **Team план**: $99/месяц за команду

Подробнее: https://railway.app/pricing

## Альтернативные платформы

Если Railway не подходит, рассмотрите:

1. **Heroku**: https://heroku.com
2. **DigitalOcean App Platform**: https://www.digitalocean.com/products/app-platform
3. **Fly.io**: https://fly.io
4. **Render**: https://render.com

Для каждой платформы создайте соответствующий Dockerfile и конфигурацию.
