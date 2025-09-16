use crate::video_streaming::{
    VideoStreamingManager, StreamResponse, StreamConfig, StreamQuality
};
use warp::Filter;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn create_video_streaming_routes(
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let manager_filter = warp::any().map(move || manager.clone());

    // Создать поток с камеры
    let create_camera_stream = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::path("camera"))
        .and(warp::post())
        .and(warp::body::json())
        .and(manager_filter.clone())
        .and_then(create_camera_stream_handler);

    // Создать поток с YouTube
    let create_youtube_stream = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::path("youtube"))
        .and(warp::post())
        .and(warp::body::json())
        .and(manager_filter.clone())
        .and_then(create_youtube_stream_handler);

    // Запустить поток
    let start_stream = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::path("start"))
        .and(warp::path::param::<String>())
        .and(warp::post())
        .and(manager_filter.clone())
        .and_then(start_stream_handler);

    // Остановить поток
    let stop_stream = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::path("stop"))
        .and(warp::path::param::<String>())
        .and(warp::post())
        .and(manager_filter.clone())
        .and_then(stop_stream_handler);

    // Получить информацию о потоке
    let get_stream = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(manager_filter.clone())
        .and_then(get_stream_handler);

    // Получить все потоки
    let get_all_streams = warp::path("api")
        .and(warp::path("streams"))
        .and(warp::get())
        .and(manager_filter.clone())
        .and_then(get_all_streams_handler);

    // Получить доступные камеры
    let get_cameras = warp::path("api")
        .and(warp::path("cameras"))
        .and(warp::get())
        .and(manager_filter.clone())
        .and_then(get_cameras_handler);

    // Валидация YouTube URL
    let validate_youtube = warp::path("api")
        .and(warp::path("youtube"))
        .and(warp::path("validate"))
        .and(warp::post())
        .and(warp::body::json())
        .and(manager_filter.clone())
        .and_then(validate_youtube_handler);

    // Статус сервера
    let status = warp::path("api")
        .and(warp::path("status"))
        .and(warp::get())
        .map(|| warp::reply::json(&json!({
            "status": "running",
            "service": "video_streaming",
            "version": "1.0.0"
        })));

    create_camera_stream
        .or(create_youtube_stream)
        .or(start_stream)
        .or(stop_stream)
        .or(get_stream)
        .or(get_all_streams)
        .or(get_cameras)
        .or(validate_youtube)
        .or(status)
}

async fn create_camera_stream_handler(
    request: serde_json::Value,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let device_id = request["device_id"].as_str()
        .ok_or_else(|| warp::reject::custom(ApiError::InvalidRequest))?
        .to_string();
    
    let resolution = request["resolution"].as_str()
        .unwrap_or("1920x1080")
        .to_string();
    
    let title = request["title"].as_str()
        .ok_or_else(|| warp::reject::custom(ApiError::InvalidRequest))?
        .to_string();

    let config = StreamConfig {
        auto_start: request["auto_start"].as_bool().unwrap_or(false),
        loop_video: request["loop_video"].as_bool().unwrap_or(false),
        quality: match request["quality"].as_str().unwrap_or("medium") {
            "low" => StreamQuality::Low,
            "medium" => StreamQuality::Medium,
            "high" => StreamQuality::High,
            "ultra" => StreamQuality::Ultra,
            _ => StreamQuality::Medium,
        },
        overlay: None,
    };

    let manager = manager.read().await;
    match manager.create_camera_stream(device_id, resolution, title, config).await {
        Ok(stream_id) => {
            Ok(warp::reply::json(&StreamResponse {
                stream_id,
                status: "created".to_string(),
                message: "Camera stream created successfully".to_string(),
            }))
        }
        Err(e) => {
            Ok(warp::reply::json(&StreamResponse {
                stream_id: "".to_string(),
                status: "error".to_string(),
                message: e,
            }))
        }
    }
}

async fn create_youtube_stream_handler(
    request: serde_json::Value,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = request["url"].as_str()
        .ok_or_else(|| warp::reject::custom(ApiError::InvalidRequest))?
        .to_string();
    
    let title = request["title"].as_str()
        .ok_or_else(|| warp::reject::custom(ApiError::InvalidRequest))?
        .to_string();

    let manager = manager.read().await;
    
    // Валидируем YouTube URL
    let video_id = manager.validate_youtube_url(&url)
        .map_err(|e| warp::reject::custom(ApiError::InvalidRequest))?;

    let config = StreamConfig {
        auto_start: request["auto_start"].as_bool().unwrap_or(false),
        loop_video: request["loop_video"].as_bool().unwrap_or(true),
        quality: match request["quality"].as_str().unwrap_or("high") {
            "low" => StreamQuality::Low,
            "medium" => StreamQuality::Medium,
            "high" => StreamQuality::High,
            "ultra" => StreamQuality::Ultra,
            _ => StreamQuality::High,
        },
        overlay: None,
    };

    match manager.create_youtube_stream(video_id, url, title, config).await {
        Ok(stream_id) => {
            Ok(warp::reply::json(&StreamResponse {
                stream_id,
                status: "created".to_string(),
                message: "YouTube stream created successfully".to_string(),
            }))
        }
        Err(e) => {
            Ok(warp::reply::json(&StreamResponse {
                stream_id: "".to_string(),
                status: "error".to_string(),
                message: e,
            }))
        }
    }
}

async fn start_stream_handler(
    stream_id: String,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let manager = manager.read().await;
    match manager.start_stream(&stream_id).await {
        Ok(_) => {
            Ok(warp::reply::json(&json!({
                "status": "success",
                "message": "Stream started successfully"
            })))
        }
        Err(e) => {
            Ok(warp::reply::json(&json!({
                "status": "error",
                "message": e
            })))
        }
    }
}

async fn stop_stream_handler(
    stream_id: String,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let manager = manager.read().await;
    match manager.stop_stream(&stream_id).await {
        Ok(_) => {
            Ok(warp::reply::json(&json!({
                "status": "success",
                "message": "Stream stopped successfully"
            })))
        }
        Err(e) => {
            Ok(warp::reply::json(&json!({
                "status": "error",
                "message": e
            })))
        }
    }
}

async fn get_stream_handler(
    stream_id: String,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let manager = manager.read().await;
    match manager.get_stream(&stream_id).await {
        Some(stream) => {
            Ok(warp::reply::json(&stream))
        }
        None => {
            Ok(warp::reply::json(&json!({
                "error": "Stream not found"
            })))
        }
    }
}

async fn get_all_streams_handler(
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let manager = manager.read().await;
    let streams = manager.get_all_streams().await;
    Ok(warp::reply::json(&streams))
}

async fn get_cameras_handler(
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let manager = manager.read().await;
    let cameras = manager.get_available_cameras().await;
    Ok(warp::reply::json(&cameras))
}

async fn validate_youtube_handler(
    request: serde_json::Value,
    manager: Arc<RwLock<VideoStreamingManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = request["url"].as_str()
        .ok_or_else(|| warp::reject::custom(ApiError::InvalidRequest))?
        .to_string();

    let manager = manager.read().await;
    match manager.validate_youtube_url(&url) {
        Ok(video_id) => {
            Ok(warp::reply::json(&json!({
                "valid": true,
                "video_id": video_id,
                "message": "Valid YouTube URL"
            })))
        }
        Err(e) => {
            Ok(warp::reply::json(&json!({
                "valid": false,
                "error": e
            })))
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    InvalidRequest,
    StreamNotFound,
    InternalError,
}

impl warp::reject::Reject for ApiError {}
