#!/bin/bash

# Скрипт для запуска тестов с PostgreSQL

echo "🚀 Запуск тестов с PostgreSQL..."

# Проверяем, установлен ли Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker не установлен. Установите Docker для запуска тестов с БД."
    exit 1
fi

# Проверяем, установлен ли docker-compose
if ! command -v docker-compose &> /dev/null; then
    echo "❌ docker-compose не установлен. Установите docker-compose для запуска тестов с БД."
    exit 1
fi

# Запускаем PostgreSQL в Docker
echo "📦 Запуск PostgreSQL в Docker..."
docker-compose -f docker-compose.test.yml up -d postgres-test

# Ждем, пока PostgreSQL будет готов
echo "⏳ Ожидание готовности PostgreSQL..."
timeout=60
counter=0
while [ $counter -lt $timeout ]; do
    if docker-compose -f docker-compose.test.yml exec -T postgres-test pg_isready -U postgres -d test_blockchain &> /dev/null; then
        echo "✅ PostgreSQL готов!"
        break
    fi
    sleep 1
    counter=$((counter + 1))
done

if [ $counter -eq $timeout ]; then
    echo "❌ PostgreSQL не запустился в течение $timeout секунд"
    docker-compose -f docker-compose.test.yml down
    exit 1
fi

# Запускаем тесты
echo "🧪 Запуск тестов..."
export DATABASE_URL="postgresql://postgres:password@localhost:5433/test_blockchain"
cargo test --lib

# Сохраняем код выхода
test_exit_code=$?

# Останавливаем PostgreSQL
echo "🛑 Остановка PostgreSQL..."
docker-compose -f docker-compose.test.yml down

# Возвращаем код выхода тестов
exit $test_exit_code
