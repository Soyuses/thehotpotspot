use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

fn main() {
    println!("🚀 Запуск простого веб-сервера Food Truck Network...");
    
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Не удалось привязать к адресу");

    println!("🌐 Веб-сервер запущен на http://127.0.0.1:8080");
    println!("📁 Обслуживает статические файлы из текущей директории");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Ошибка подключения: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");

    println!("📥 Запрос: {}", request_line);

    let (status_line, filename, content_type) = parse_request(request_line);

    let response = build_response(status_line, &filename, content_type);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(request_line: &str) -> (String, String, String) {
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

    // Проверяем существование файла
    if !Path::new(filename).exists() {
        return (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
            "404.html".to_string(),
            "text/html".to_string(),
        );
    }

    // Определяем тип контента
    let content_type = get_content_type(filename);

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

fn build_response(status_line: String, filename: &str, content_type: String) -> String {
    // Читаем содержимое файла
    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            return "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n".to_string();
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
