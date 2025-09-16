@echo off
echo Запуск серверов The Hot Pot Spot...

echo.
echo 1. Запуск основного блокчейн сервера на порту 3000...
start "Blockchain Server" cmd /k "cargo run --bin blockchain_project"

echo.
echo 2. Запуск Check API сервера на порту 8081...
start "Check API Server" cmd /k "cargo run --example check_api_demo"

echo.
echo 3. Запуск Transparency API сервера на порту 8082...
start "Transparency API Server" cmd /k "cargo run --example transparency_demo"

echo.
echo 4. Запуск Video Streaming API сервера на порту 8083...
start "Video Streaming API Server" cmd /k "cargo run --example video_streaming_server"

echo.
echo 5. Открытие веб-интерфейсов...
start index.html
start owner_dashboard.html
start franchise_dashboard.html
start transparency_dashboard.html
start video_streaming_dashboard.html

echo.
echo Все серверы запущены!
echo.
echo Доступные интерфейсы:
echo - Главная страница: index.html
echo - Панель владельца: owner_dashboard.html
echo - Панель франчайзи: franchise_dashboard.html
echo - Панель прозрачности: transparency_dashboard.html
echo - Панель видеопотоков: video_streaming_dashboard.html
echo.
echo API серверы:
echo - Основной API: http://localhost:3000
echo - Check API: http://localhost:8081
echo - Transparency API: http://localhost:8082
echo - Video Streaming API: http://localhost:8083
echo.
pause

