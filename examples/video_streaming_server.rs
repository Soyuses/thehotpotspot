use blockchain_project::video_streaming::VideoStreamingManager;
use blockchain_project::video_streaming_api::create_video_streaming_routes;
use warp::Filter;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    println!("🎥 Запуск сервера видеопотоков The Hot Pot Spot");
    println!("=" .repeat(50));

    // Создаем менеджер видеопотоков
    let manager = Arc::new(RwLock::new(VideoStreamingManager::new()));

    // Создаем маршруты
    let routes = create_video_streaming_routes(manager)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]));

    // Запускаем сервер
    let port = 8083;
    println!("🚀 Сервер запущен на порту {}", port);
    println!("📱 Веб-интерфейс: http://localhost:{}/video_streaming_dashboard.html", port);
    println!("🔗 API документация:");
    println!("   GET  /api/status - Статус сервера");
    println!("   GET  /api/cameras - Список камер");
    println!("   POST /api/streams/camera - Создать поток с камеры");
    println!("   POST /api/streams/youtube - Создать поток с YouTube");
    println!("   POST /api/streams/start/{id} - Запустить поток");
    println!("   POST /api/streams/stop/{id} - Остановить поток");
    println!("   GET  /api/streams - Список всех потоков");
    println!("   GET  /api/streams/{id} - Информация о потоке");
    println!("   POST /api/youtube/validate - Валидация YouTube URL");
    println!("=" .repeat(50));

    // Добавляем статические файлы
    let static_files = warp::path("video_streaming_dashboard.html")
        .and(warp::fs::file("./video_streaming_dashboard.html"));

    let all_routes = routes.or(static_files);

    warp::serve(all_routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
