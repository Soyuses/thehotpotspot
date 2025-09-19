@echo off
setlocal enabledelayedexpansion

echo 🚀 Начинаем развертывание TypeScript фронтенда на Railway...

REM Проверяем, что мы в правильной директории
if not exist "package.json" (
    echo ❌ Ошибка: package.json не найден. Убедитесь, что вы находитесь в директории frontend.
    exit /b 1
)

REM Проверяем, что Railway CLI установлен
railway --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Railway CLI не установлен. Установите его с помощью:
    echo npm install -g @railway/cli
    exit /b 1
)

REM Проверяем, что пользователь авторизован в Railway
railway whoami >nul 2>&1
if errorlevel 1 (
    echo ❌ Вы не авторизованы в Railway. Выполните:
    echo railway login
    exit /b 1
)

echo ✅ Проверки пройдены

REM Устанавливаем зависимости
echo 📦 Устанавливаем зависимости...
npm install

REM Проверяем типы TypeScript
echo 🔍 Проверяем типы TypeScript...
npm run type-check

REM Запускаем линтер
echo 🔧 Проверяем код линтером...
npm run lint

REM Собираем проект
echo 🔨 Собираем проект...
npm run build

if errorlevel 1 (
    echo ❌ Ошибка сборки проекта
    exit /b 1
)

echo ✅ Проект успешно собран

REM Инициализируем Railway проект (если еще не инициализирован)
if not exist "railway.toml" (
    echo 🚂 Инициализируем Railway проект...
    railway init
)

REM Проверяем статус проекта
echo 📊 Проверяем статус Railway проекта...
railway status

REM Развертываем проект
echo 🚀 Развертываем проект на Railway...
railway up

if errorlevel 1 (
    echo ❌ Ошибка развертывания
    exit /b 1
)

echo ✅ TypeScript фронтенд успешно развернут на Railway!
echo 🌐 URL приложения:
railway domain
echo.
echo 📋 Следующие шаги:
echo 1. Настройте переменные окружения в Railway dashboard
echo 2. Убедитесь, что API сервер доступен
echo 3. Проверьте работу приложения
echo.
echo 📖 Подробная инструкция: README.md

pause
