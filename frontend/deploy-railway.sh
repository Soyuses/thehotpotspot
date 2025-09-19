#!/bin/bash

# Скрипт для развертывания TypeScript фронтенда на Railway

set -e

echo "🚀 Начинаем развертывание TypeScript фронтенда на Railway..."

# Проверяем, что мы в правильной директории
if [ ! -f "package.json" ]; then
    echo "❌ Ошибка: package.json не найден. Убедитесь, что вы находитесь в директории frontend."
    exit 1
fi

# Проверяем, что Railway CLI установлен
if ! command -v railway &> /dev/null; then
    echo "❌ Railway CLI не установлен. Установите его с помощью:"
    echo "npm install -g @railway/cli"
    exit 1
fi

# Проверяем, что пользователь авторизован в Railway
if ! railway whoami &> /dev/null; then
    echo "❌ Вы не авторизованы в Railway. Выполните:"
    echo "railway login"
    exit 1
fi

echo "✅ Проверки пройдены"

# Устанавливаем зависимости
echo "📦 Устанавливаем зависимости..."
npm install

# Проверяем типы TypeScript
echo "🔍 Проверяем типы TypeScript..."
npm run type-check

# Запускаем линтер
echo "🔧 Проверяем код линтером..."
npm run lint

# Собираем проект
echo "🔨 Собираем проект..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Ошибка сборки проекта"
    exit 1
fi

echo "✅ Проект успешно собран"

# Инициализируем Railway проект (если еще не инициализирован)
if [ ! -f "railway.toml" ]; then
    echo "🚂 Инициализируем Railway проект..."
    railway init
fi

# Проверяем статус проекта
echo "📊 Проверяем статус Railway проекта..."
railway status

# Развертываем проект
echo "🚀 Развертываем проект на Railway..."
railway up

if [ $? -eq 0 ]; then
    echo "✅ TypeScript фронтенд успешно развернут на Railway!"
    echo "🌐 URL приложения:"
    railway domain
    echo ""
    echo "📋 Следующие шаги:"
    echo "1. Настройте переменные окружения в Railway dashboard"
    echo "2. Убедитесь, что API сервер доступен"
    echo "3. Проверьте работу приложения"
    echo ""
    echo "📖 Подробная инструкция: README.md"
else
    echo "❌ Ошибка развертывания"
    exit 1
fi
