// Simple HTTP Server for blockchain API
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct SimpleServer {
    port: u16,
}

impl SimpleServer {
    pub fn new(port: u16) -> Self {
        SimpleServer { port }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");
        
        println!("🌐 Simple HTTP Server started on port {}", self.port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        Self::handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 4096];
        
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]);
                let response = Self::process_request(&request);
                
                let http_response = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: application/json\r\n\
                     Access-Control-Allow-Origin: *\r\n\
                     Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
                     Access-Control-Allow-Headers: Content-Type\r\n\
                     Content-Length: {}\r\n\
                     \r\n\
                     {}",
                    response.len(),
                    response
                );
                
                if let Err(e) = stream.write_all(http_response.as_bytes()) {
                    eprintln!("Error writing response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading request: {}", e);
            }
        }
    }

    fn process_request(request: &str) -> String {
        // Handle OPTIONS request for CORS
        if request.starts_with("OPTIONS") {
            return "{}".to_string();
        }

        // Simple response for now
        serde_json::json!({
            "message": "Simple server is running",
            "status": "ok",
            "note": "Full blockchain integration will be added later"
        }).to_string()
    }
}