use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

pub struct WebServer {
    port: u16,
    static_dir: String,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            static_dir: ".".to_string(),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");

        println!("🌐 Веб-сервер запущен на http://127.0.0.1:{}", self.port);
        println!("📁 Обслуживает статические файлы из: {}", self.static_dir);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let static_dir = self.static_dir.clone();
                    thread::spawn(move || {
                        Self::handle_connection(stream, static_dir);
                    });
                }
                Err(e) => {
                    eprintln!("Ошибка подключения: {}", e);
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, static_dir: String) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap_or("");

        println!("📥 Запрос: {}", request_line);

        let (status_line, filename, content_type) = Self::parse_request(request_line, &static_dir);

        let response = Self::build_response(status_line, &filename, content_type, &static_dir);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn parse_request(request_line: &str, static_dir: &str) -> (String, String, String) {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        if parts.len() < 2 {
            return (
                "HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_string(),
                "".to_string(),
                "".to_string(),
            );
        }

        let method = parts[0];
        let path = parts[1];

        if method != "GET" {
            return (
                "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n".to_string(),
                "".to_string(),
                "".to_string(),
            );
        }

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
                "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
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
                            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
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
        assert_eq!(WebServer::get_content_type("index.html"), "text/html; charset=utf-8");
        assert_eq!(WebServer::get_content_type("style.css"), "text/css");
        assert_eq!(WebServer::get_content_type("script.js"), "application/javascript");
        assert_eq!(WebServer::get_content_type("image.png"), "image/png");
    }

    #[test]
    fn test_request_parsing() {
        let (status, filename, content_type) = WebServer::parse_request("GET / HTTP/1.1", ".");
        assert_eq!(status, "HTTP/1.1 200 OK\r\n");
        assert_eq!(filename, "index.html");
        assert_eq!(content_type, "text/html; charset=utf-8");

        let (status, filename, _) = WebServer::parse_request("GET /owner_dashboard.html HTTP/1.1", ".");
        assert_eq!(status, "HTTP/1.1 200 OK\r\n");
        assert_eq!(filename, "owner_dashboard.html");
    }
}
