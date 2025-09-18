#!/bin/bash

# Скрипт для развертывания The Hot Pot Spot на Railway

set -e

echo "🚀 Начинаем развертывание The Hot Pot Spot на Railway..."

# Проверяем, что мы в правильной директории
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Ошибка: Cargo.toml не найден. Убедитесь, что вы находитесь в корневой директории проекта."
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

# Собираем проект локально для проверки
echo "🔨 Собираем проект..."
cargo build --release

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
    echo "✅ Проект успешно развернут на Railway!"
    echo "🌐 URL приложения:"
    railway domain
    echo ""
    echo "📋 Следующие шаги:"
    echo "1. Настройте переменные окружения в Railway dashboard"
    echo "2. Добавьте PostgreSQL сервис"
    echo "3. Проверьте работу приложения"
    echo ""
    echo "📖 Подробная инструкция: railway-config.md"
else
    echo "❌ Ошибка развертывания"
    exit 1
fi
