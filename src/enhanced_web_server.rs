use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};
use serde_json;
use crate::video_api::VideoHTTPHandler;
use crate::video_surveillance::VideoSurveillanceSystem;

pub struct EnhancedWebServer {
    port: u16,
    static_dir: String,
    video_handler: Option<Arc<VideoHTTPHandler>>,
}

impl EnhancedWebServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            static_dir: ".".to_string(),
            video_handler: None,
        }
    }

    pub fn with_video_handler(mut self, video_handler: Arc<VideoHTTPHandler>) -> Self {
        self.video_handler = Some(video_handler);
        self
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");

        println!("🌐 Enhanced веб-сервер запущен на http://127.0.0.1:{}", self.port);
        println!("📁 Обслуживает статические файлы из: {}", self.static_dir);
        println!("🎥 API видеонаблюдения: {}", if self.video_handler.is_some() { "активно" } else { "неактивно" });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let static_dir = self.static_dir.clone();
                    let video_handler = self.video_handler.clone();
                    thread::spawn(move || {
                        Self::handle_connection(stream, static_dir, video_handler);
                    });
                }
                Err(e) => {
                    eprintln!("Ошибка подключения: {}", e);
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, static_dir: String, video_handler: Option<Arc<VideoHTTPHandler>>) {
        let mut buffer = [0; 4096]; // Увеличиваем буфер для больших запросов
        let bytes_read = stream.read(&mut buffer).unwrap_or(0);
        
        if bytes_read == 0 {
            return;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let request_lines: Vec<&str> = request.lines().collect();
        
        if request_lines.is_empty() {
            return;
        }

        let request_line = request_lines[0];
        println!("📥 Запрос: {}", request_line);

        // Парсим HTTP заголовки
        let mut headers = HashMap::new();
        let mut body_start = 0;
        
        for (i, line) in request_lines.iter().enumerate() {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            if i > 0 && line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    headers.insert(parts[0].trim().to_lowercase(), parts[1].trim().to_string());
                }
            }
        }

        // Извлекаем тело запроса
        let body = if body_start < request_lines.len() {
            request_lines[body_start..].join("\n")
        } else {
            String::new()
        };

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            Self::send_error_response(&mut stream, 400, "Bad Request");
            return;
        }

        let method = parts[0];
        let path = parts[1];

        // Обрабатываем CORS preflight запросы
        if method == "OPTIONS" {
            Self::send_cors_response(&mut stream);
            return;
        }

        // Проверяем, является ли это API запросом
        if path.starts_with("/api/") {
            if let Some(ref handler) = video_handler {
                Self::handle_api_request(&mut stream, method, path, &body, handler);
            } else {
                Self::send_error_response(&mut stream, 503, "Video API not available");
            }
            return;
        }

        // Обрабатываем статические файлы
        Self::handle_static_request(&mut stream, method, path, &static_dir);
    }

    fn handle_api_request(
        stream: &mut TcpStream,
        method: &str,
        path: &str,
        body: &str,
        video_handler: &VideoHTTPHandler,
    ) {
        // Создаем простые заглушки для API, так как async обработка требует tokio runtime
        let response = match (method, path) {
            ("POST", "/api/video-consent") => {
                // Заглушка для запроса согласия
                r#"{"type": "ConsentRequested", "consent_id": "CONSENT_001", "consent_text": "Согласие на видеозапись", "max_duration_minutes": 30, "anonymization_options": ["blur", "replace", "none"]}"#.to_string()
            },
            ("POST", "/api/video-consent/confirm") => {
                r#"{"type": "Success", "data": {"message": "Consent confirmed successfully"}}"#.to_string()
            },
            ("POST", "/api/video-recording/start") => {
                r#"{"type": "RecordingStarted", "recording_id": "REC_12345678", "camera_id": "CAM_TABLE_001", "estimated_end_time": 1734567890}"#.to_string()
            },
            ("POST", "/api/video-recording/stop") => {
                r#"{"type": "RecordingStopped", "recording_id": "REC_12345678", "duration_seconds": 1800}"#.to_string()
            },
            ("GET", "/api/video-recording/active") => {
                r#"{"type": "ActiveRecordings", "recordings": [{"recording_id": "REC_12345678", "camera_id": "CAM_TABLE_001", "customer_id": "CUSTOMER_001", "start_time": 1734566090, "status": "recording"}]}"#.to_string()
            },
            ("GET", "/api/video-cameras/stats") => {
                r#"{"type": "CameraStats", "stats": {"CAM_EXT_001": {"camera_id": "CAM_EXT_001", "active_recordings": 0, "total_recordings": 5}, "CAM_PROD_001": {"camera_id": "CAM_PROD_001", "active_recordings": 1, "total_recordings": 10}, "CAM_TABLE_001": {"camera_id": "CAM_TABLE_001", "active_recordings": 1, "total_recordings": 3}}}"#.to_string()
            },
            ("POST", "/api/video-cameras") => {
                r#"{"type": "Success", "data": {"message": "Camera added successfully", "camera_id": "CAM_NEW_001"}}"#.to_string()
            },
            _ => {
                format!(r#"{{"type": "Error", "message": "API endpoint not found", "code": "NOT_FOUND"}}"#)
            }
        };

        let response_body = response;
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
             Access-Control-Allow-Headers: Content-Type\r\n\
             \r\n\
             {}",
            response_body.len(),
            response_body
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Ошибка отправки API ответа: {}", e);
        }
        stream.flush().unwrap();
    }

    fn handle_static_request(stream: &mut TcpStream, method: &str, path: &str, static_dir: &str) {
        if method != "GET" {
            Self::send_error_response(stream, 405, "Method Not Allowed");
            return;
        }

        let (status_line, filename, content_type) = Self::parse_request(path, static_dir);
        let response = Self::build_response(status_line, &filename, content_type, static_dir);
        
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Ошибка отправки статического файла: {}", e);
        }
        stream.flush().unwrap();
    }

    fn send_cors_response(stream: &mut TcpStream) {
        let response = "HTTP/1.1 200 OK\r\n\
                       Access-Control-Allow-Origin: *\r\n\
                       Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                       Access-Control-Allow-Headers: Content-Type\r\n\
                       \r\n";
        
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Ошибка отправки CORS ответа: {}", e);
        }
        stream.flush().unwrap();
    }

    fn send_error_response(stream: &mut TcpStream, status_code: u16, message: &str) {
        let status_line = match status_code {
            400 => "HTTP/1.1 400 BAD REQUEST",
            404 => "HTTP/1.1 404 NOT FOUND",
            405 => "HTTP/1.1 405 METHOD NOT ALLOWED",
            500 => "HTTP/1.1 500 INTERNAL SERVER ERROR",
            503 => "HTTP/1.1 503 SERVICE UNAVAILABLE",
            _ => "HTTP/1.1 500 INTERNAL SERVER ERROR",
        };

        let error_html = format!(
            "<!DOCTYPE html>
<html>
<head>
    <title>Error {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; text-align: center; padding: 50px; }}
        .error {{ color: #d32f2f; }}
    </style>
</head>
<body>
    <h1 class=\"error\">Error {}</h1>
    <p>{}</p>
</body>
</html>",
            status_code, status_code, message
        );

        let response = format!(
            "{}\r\n\
             Content-Type: text/html\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             \r\n\
             {}",
            status_line,
            error_html.len(),
            error_html
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Ошибка отправки ошибки: {}", e);
        }
        stream.flush().unwrap();
    }

    fn parse_request(path: &str, static_dir: &str) -> (String, String, String) {
        // Определяем файл для обслуживания
        let filename = if path == "/" || path == "/index.html" {
            "index.html"
        } else if path.starts_with("/") {
            &path[1..] // Убираем ведущий слеш
        } else {
            path
        };

        let full_path = format!("{}/{}", static_dir, filename);
        
        // Проверяем существование файла
        if !Path::new(&full_path).exists() {
            return (
                "HTTP/1.1 404 NOT FOUND\r\n".to_string(),
                "404.html".to_string(),
                "text/html".to_string(),
            );
        }

        // Определяем тип контента
        let content_type = Self::get_content_type(filename);

        (
            "HTTP/1.1 200 OK\r\n".to_string(),
            filename.to_string(),
            content_type,
        )
    }

    fn get_content_type(filename: &str) -> String {
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match extension {
            "html" => "text/html; charset=utf-8",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "pdf" => "application/pdf",
            "txt" => "text/plain",
            _ => "application/octet-stream",
        }.to_string()
    }

    fn build_response(status_line: String, filename: &str, content_type: String, static_dir: &str) -> String {
        let full_path = format!("{}/{}", static_dir, filename);
        
        // Читаем содержимое файла
        let contents = match fs::read_to_string(&full_path) {
            Ok(content) => content,
            Err(_) => {
                // Если не удалось прочитать как текст, попробуем как бинарный файл
                match fs::read(&full_path) {
                    Ok(bytes) => {
                        // Для бинарных файлов возвращаем специальный ответ
                        return format!(
                            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
                            status_line,
                            content_type,
                            bytes.len()
                        );
                    }
                    Err(_) => {
                        return "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n".to_string();
                    }
                }
            }
        };

        let length = contents.len();
        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n{}",
            status_line,
            content_type,
            length,
            contents
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_detection() {
        assert_eq!(EnhancedWebServer::get_content_type("index.html"), "text/html; charset=utf-8");
        assert_eq!(EnhancedWebServer::get_content_type("style.css"), "text/css");
        assert_eq!(EnhancedWebServer::get_content_type("script.js"), "application/javascript");
        assert_eq!(EnhancedWebServer::get_content_type("image.png"), "image/png");
    }

    #[test]
    fn test_request_parsing() {
        let (status, filename, content_type) = EnhancedWebServer::parse_request("/", ".");
        assert_eq!(status, "HTTP/1.1 200 OK\r\n");
        assert_eq!(filename, "index.html");
        assert_eq!(content_type, "text/html; charset=utf-8");

        let (status, filename, _) = EnhancedWebServer::parse_request("/owner_dashboard.html", ".");
        assert_eq!(status, "HTTP/1.1 200 OK\r\n");
        assert_eq!(filename, "owner_dashboard.html");
    }

    #[test]
    fn test_api_path_detection() {
        assert!("/api/video-consent".starts_with("/api/"));
        assert!("/api/video-recording/start".starts_with("/api/"));
        assert!(!"/index.html".starts_with("/api/"));
    }
}
