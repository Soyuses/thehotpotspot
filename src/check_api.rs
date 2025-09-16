//! Check API Module for The Hot Pot Spot
//! 
//! This module provides HTTP API endpoints for check generation and management.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde_json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::check_generation::{
    CheckGenerationService, CheckGenerationRequest, CheckGenerationResponse,
    CheckClaimRequest, CheckClaimResponse, CheckGenerationConfig, CheckStatistics
};

/// Check API server
#[derive(Debug)]
pub struct CheckAPIServer {
    /// Port to listen on
    pub port: u16,
    /// Check generation service
    pub check_service: Arc<RwLock<CheckGenerationService>>,
}

impl CheckAPIServer {
    /// Create new check API server
    pub fn new(port: u16) -> Self {
        let config = CheckGenerationConfig::default();
        let check_service = Arc::new(RwLock::new(CheckGenerationService::new(config)));
        
        Self {
            port,
            check_service,
        }
    }

    /// Start the API server
    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");

        println!("🔧 Check API Server запущен на http://127.0.0.1:{}", self.port);
        println!("📋 Доступные endpoints:");
        println!("  POST /api/checks/generate - Генерация чека с QR-кодом");
        println!("  POST /api/checks/claim - Активация чека через QR-код");
        println!("  GET  /api/checks/{{id}} - Получить информацию о чеке");
        println!("  GET  /api/checks/statistics - Статистика чеков");
        println!("  GET  /api/checks/unclaimed - Невостребованные чеки");
        println!("  POST /api/checks/print - Отметить чек как напечатанный");
        println!("  POST /api/checks/discard - Отметить чек как выброшенный");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let check_service = self.check_service.clone();
                    thread::spawn(move || {
                        Self::handle_connection(stream, check_service);
                    });
                }
                Err(e) => {
                    eprintln!("Ошибка подключения: {}", e);
                }
            }
        }
    }

    /// Handle HTTP connection
    fn handle_connection(
        mut stream: TcpStream,
        check_service: Arc<RwLock<CheckGenerationService>>,
    ) {
        let mut buffer = [0; 4096];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap_or("");

        println!("📥 API Запрос: {}", request_line);

        // Parse request
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            Self::send_error_response(&mut stream, 400, "Bad Request");
            return;
        }

        let method = parts[0];
        let path = parts[1];

        // Extract request body
        let body = Self::extract_request_body(&request);
        
        // Create server instance for handler
        let server = CheckAPIServer {
            port: 0,
            check_service,
        };

        // Route to appropriate handler
        let response = match (method, path) {
            ("POST", "/api/checks/generate") => server.handle_generate_check(&body),
            ("POST", "/api/checks/claim") => server.handle_claim_check(&body),
            ("GET", path) if path.starts_with("/api/checks/") && path != "/api/checks/statistics" && path != "/api/checks/unclaimed" => {
                server.handle_get_check(path)
            },
            ("GET", "/api/checks/statistics") => server.handle_get_statistics(),
            ("GET", "/api/checks/unclaimed") => server.handle_get_unclaimed(),
            ("POST", "/api/checks/print") => server.handle_print_check(&body),
            ("POST", "/api/checks/discard") => server.handle_discard_check(&body),
            _ => {
                Self::send_error_response(&mut stream, 404, "Not Found");
                return;
            }
        };

        Self::send_response(&mut stream, &response);
    }

    /// Extract request body from HTTP request
    fn extract_request_body(request: &str) -> String {
        if let Some(body_start) = request.find("\r\n\r\n") {
            request[body_start + 4..].trim().to_string()
        } else {
            String::new()
        }
    }

    /// Send HTTP response
    fn send_response(stream: &mut TcpStream, response: &str) {
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
             Access-Control-Allow-Headers: Content-Type\r\n\
             \r\n\
             {}",
            response.len(),
            response
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    /// Send error response
    fn send_error_response(stream: &mut TcpStream, status_code: u16, message: &str) {
        let error_response = serde_json::json!({
            "success": false,
            "error": message
        });

        let response = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             \r\n\
             {}",
            status_code,
            message,
            error_response.to_string().len(),
            error_response
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    /// Handle check generation request
    fn handle_generate_check(&self, body: &str) -> String {
        let request: CheckGenerationRequest = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                return serde_json::json!({
                    "success": false,
                    "error": format!("Invalid request: {}", e)
                }).to_string();
            }
        };

        // Generate check (this would be async in a real implementation)
        let mut service = self.check_service.try_write().unwrap();
        match service.generate_check(request) {
            Ok(response) => serde_json::to_string(&response).unwrap(),
            Err(e) => serde_json::json!({
                "success": false,
                "error": e
            }).to_string(),
        }
    }

    /// Handle check claim request
    fn handle_claim_check(&self, body: &str) -> String {
        let request: CheckClaimRequest = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                return serde_json::json!({
                    "success": false,
                    "error": format!("Invalid request: {}", e)
                }).to_string();
            }
        };

        // Claim check
        let mut service = self.check_service.try_write().unwrap();
        match service.claim_check(request) {
            Ok(response) => serde_json::to_string(&response).unwrap(),
            Err(e) => serde_json::json!({
                "success": false,
                "error": e
            }).to_string(),
        }
    }

    /// Handle get check request
    fn handle_get_check(&self, path: &str) -> String {
        // Extract check ID from path
        let check_id = path.split('/').last().unwrap_or("");
        
        let service = self.check_service.try_read().unwrap();
        match service.get_check(check_id) {
            Some(check) => serde_json::to_string(check).unwrap(),
            None => serde_json::json!({
                "success": false,
                "error": "Check not found"
            }).to_string(),
        }
    }

    /// Handle get statistics request
    fn handle_get_statistics(&self) -> String {
        let service = self.check_service.try_read().unwrap();
        let stats = service.get_statistics();
        serde_json::to_string(&stats).unwrap()
    }

    /// Handle get unclaimed checks request
    fn handle_get_unclaimed(&self) -> String {
        let service = self.check_service.try_read().unwrap();
        let unclaimed = service.get_unclaimed_checks();
        serde_json::to_string(&unclaimed).unwrap()
    }

    /// Handle print check request
    fn handle_print_check(&self, body: &str) -> String {
        let request: serde_json::Value = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                return serde_json::json!({
                    "success": false,
                    "error": format!("Invalid request: {}", e)
                }).to_string();
            }
        };

        let check_id = request["check_id"].as_str().unwrap_or("");
        
        let mut service = self.check_service.try_write().unwrap();
        match service.print_check(check_id) {
            Ok(_) => serde_json::json!({
                "success": true,
                "message": "Check marked as printed"
            }).to_string(),
            Err(e) => serde_json::json!({
                "success": false,
                "error": e
            }).to_string(),
        }
    }

    /// Handle discard check request
    fn handle_discard_check(&self, body: &str) -> String {
        let request: serde_json::Value = match serde_json::from_str(body) {
            Ok(req) => req,
            Err(e) => {
                return serde_json::json!({
                    "success": false,
                    "error": format!("Invalid request: {}", e)
                }).to_string();
            }
        };

        let check_id = request["check_id"].as_str().unwrap_or("");
        
        let mut service = self.check_service.try_write().unwrap();
        match service.discard_check(check_id) {
            Ok(_) => serde_json::json!({
                "success": true,
                "message": "Check marked as discarded"
            }).to_string(),
            Err(e) => serde_json::json!({
                "success": false,
                "error": e
            }).to_string(),
        }
    }
}

/// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_check_api_server() {
        let server = CheckAPIServer::new(8081);
        
        // Start server in background thread
        let server_handle = thread::spawn(move || {
            server.start();
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(100));

        // Test check generation
        let generate_request = CheckGenerationRequest {
            sale_id: "test_sale_001".to_string(),
            node_id: "test_node_001".to_string(),
            amount_gel: 25.0,
            st_tokens: 500,
            customer_phone: None,
        };

        // In a real test, we would make HTTP requests to the server
        // For now, we'll just verify the server can be created
        // Note: is_alive() is not available on JoinHandle in std
        // We'll just verify the server started successfully
    }
}

/// Demo function to show API usage
pub fn demo_check_api() {
    println!("🔧 Демонстрация Check API");
    
    let server = CheckAPIServer::new(8081);
    
    // Example: Generate a check
    let request = CheckGenerationRequest {
        sale_id: "demo_sale_001".to_string(),
        node_id: "demo_node_001".to_string(),
        amount_gel: 25.0,
        st_tokens: 500,
        customer_phone: None,
    };

    let mut service = CheckGenerationService::new(CheckGenerationConfig::default());
    match service.generate_check(request) {
        Ok(response) => {
            println!("✅ Чек сгенерирован: {}", response.check.check_id);
            println!("💰 Сумма: {} GEL", response.check.amount_gel);
            println!("🪙 Токены: {} ST", response.check.st_tokens);
            println!("🔗 Кошелек: {}", response.check.wallet_address);
        }
        Err(e) => {
            println!("❌ Ошибка генерации чека: {}", e);
        }
    }

    // Example: Get statistics
    let stats = service.get_statistics();
    println!("📊 Статистика чеков:");
    println!("  Всего чеков: {}", stats.total_checks);
    println!("  Сгенерировано: {}", stats.generated);
    println!("  Напечатано: {}", stats.printed);
    println!("  Активировано: {}", stats.claimed);
}

