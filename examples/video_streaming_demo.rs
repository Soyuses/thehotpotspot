use blockchain_project::video_streaming::{
    VideoStreamingManager, StreamConfig, StreamQuality, OverlayConfig
};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎥 The Hot Pot Spot - Демонстрация видеопотоков");
    println!("{}", "=".repeat(50));

    // Создаем менеджер видеопотоков
    let manager = Arc::new(RwLock::new(VideoStreamingManager::new()));

    // Получаем список доступных камер
    println!("\n📹 Поиск доступных камер...");
    let cameras = manager.read().await.get_available_cameras().await;
    
    if cameras.is_empty() {
        println!("❌ Камеры не найдены");
    } else {
        println!("✅ Найдено камер: {}", cameras.len());
        for camera in &cameras {
            println!("   - {} ({}): {}", camera.name, camera.resolution, camera.device_id);
        }
    }

    // Создаем поток с камеры
    if !cameras.is_empty() {
        println!("\n🎬 Создание потока с камеры...");
        
        let config = StreamConfig {
            auto_start: false,
            loop_video: false,
            quality: StreamQuality::High,
            overlay: Some(OverlayConfig {
                show_timestamp: true,
                show_logo: true,
                logo_path: Some("logo.png".to_string()),
                text_overlay: Some("The Hot Pot Spot".to_string()),
            }),
        };

        let camera_stream_id = manager.write().await.create_camera_stream(
            cameras[0].device_id.clone(),
            cameras[0].resolution.clone(),
            "Основная камера фудтрака".to_string(),
            config,
        ).await?;

        println!("✅ Поток с камеры создан: {}", camera_stream_id);

        // Запускаем поток с камеры
        println!("▶️ Запуск потока с камеры...");
        manager.write().await.start_stream(&camera_stream_id).await?;
        println!("✅ Поток с камеры запущен");

        // Ждем немного
        sleep(Duration::from_secs(3)).await;

        // Останавливаем поток с камеры
        println!("⏹️ Остановка потока с камеры...");
        manager.write().await.stop_stream(&camera_stream_id).await?;
        println!("✅ Поток с камеры остановлен");
    }

    // Создаем поток с YouTube
    println!("\n📺 Создание потока с YouTube...");
    
    let youtube_url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let video_id = manager.read().await.validate_youtube_url(youtube_url)?;
    
    let youtube_config = StreamConfig {
        auto_start: false,
        loop_video: true,
        quality: StreamQuality::High,
        overlay: None,
    };

    let youtube_stream_id = manager.write().await.create_youtube_stream(
        video_id,
        youtube_url.to_string(),
        "Фоновое видео для фудтрака".to_string(),
        youtube_config,
    ).await?;

    println!("✅ YouTube поток создан: {}", youtube_stream_id);

    // Запускаем YouTube поток
    println!("▶️ Запуск YouTube потока...");
    manager.write().await.start_stream(&youtube_stream_id).await?;
    println!("✅ YouTube поток запущен");

    // Ждем немного
    sleep(Duration::from_secs(5)).await;

    // Получаем информацию о потоках
    println!("\n📋 Информация о потоках:");
    let all_streams = manager.read().await.get_all_streams().await;
    
    for stream in &all_streams {
        println!("\n🎥 Поток: {}", stream.metadata.title);
        println!("   ID: {}", stream.stream_id);
        println!("   Тип: {:?}", stream.stream_type);
        println!("   Статус: {:?}", stream.status);
        println!("   Создан: {}", stream.created_at);
        println!("   Обновлен: {}", stream.updated_at);
        
        if let Some(fps) = stream.metadata.fps {
            println!("   FPS: {}", fps);
        }
        if let Some(bitrate) = stream.metadata.bitrate {
            println!("   Битрейт: {} kbps", bitrate);
        }
        if let Some(resolution) = &stream.metadata.resolution {
            println!("   Разрешение: {}", resolution);
        }
    }

    // Останавливаем YouTube поток
    println!("\n⏹️ Остановка YouTube потока...");
    manager.write().await.stop_stream(&youtube_stream_id).await?;
    println!("✅ YouTube поток остановлен");

    // Демонстрация валидации YouTube URL
    println!("\n🔍 Тестирование валидации YouTube URL:");
    
    let test_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://youtu.be/dQw4w9WgXcQ",
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=10s",
        "https://invalid-url.com",
        "not-a-url",
    ];

    for url in test_urls {
        match manager.read().await.validate_youtube_url(url) {
            Ok(video_id) => {
                println!("✅ {} -> {}", url, video_id);
            }
            Err(e) => {
                println!("❌ {} -> {}", url, e);
            }
        }
    }

    println!("\n🎉 Демонстрация видеопотоков завершена!");
    println!("{}", "=".repeat(50));

    Ok(())
}
