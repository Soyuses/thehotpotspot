use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::{Blockchain, MenuItem, Order};

pub struct SimpleServer {
    blockchain: Arc<Mutex<Blockchain>>,
    port: u16,
}

impl SimpleServer {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Self {
        SimpleServer { blockchain, port }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");
        
        println!("🌐 Simple HTTP Server started on port {}", self.port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let blockchain = Arc::clone(&self.blockchain);
                    thread::spawn(move || {
                        Self::handle_client(stream, blockchain);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 4096];
        
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]);
                let response = Self::process_request(&request, blockchain);
                
                let http_response = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: application/json\r\n\
                     Access-Control-Allow-Origin: *\r\n\
                     Access-Control-Allow-Headers: Content-Type\r\n\
                     Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
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

    fn process_request(request: &str, blockchain: Arc<Mutex<Blockchain>>) -> String {
        // Handle OPTIONS request for CORS
        if request.starts_with("OPTIONS") {
            return "{}".to_string();
        }

        // Extract JSON from POST request
        let json_start = request.find("\r\n\r\n");
        if json_start.is_none() {
            return serde_json::json!({"error": "Invalid request format"}).to_string();
        }

        let json_start = json_start.unwrap() + 4;
        let json_str = &request[json_start..];
        
        // Try to parse as wrapped enum (e.g., {"GetMenu": {}})
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
            if let Some(obj) = value.as_object() {
                for (key, val) in obj {
                    match key.as_str() {
                        "GetMenu" => {
                            let mut bc = blockchain.lock().unwrap();
                            let menu_items = bc.menu_items.clone();
                            return serde_json::json!({"Menu": menu_items}).to_string();
                        }
                        "AddMenuItem" => {
                            if let Ok(menu_item) = serde_json::from_value::<MenuItem>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                bc.menu_items.push(menu_item);
                                return serde_json::json!({"Success": "Menu item added successfully"}).to_string();
                            }
                        }
                        "UpdateMenuItem" => {
                            if let Ok(menu_item) = serde_json::from_value::<MenuItem>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                if let Some(pos) = bc.menu_items.iter().position(|item| item.id == menu_item.id) {
                                    bc.menu_items[pos] = menu_item;
                                }
                                return serde_json::json!({"Success": "Menu item updated successfully"}).to_string();
                            }
                        }
                        "DeleteMenuItem" => {
                            if let Ok(id) = serde_json::from_value::<String>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                bc.menu_items.retain(|item| item.id != id);
                                return serde_json::json!({"Success": "Menu item deleted successfully"}).to_string();
                            }
                        }
                        "GetOrders" => {
                            let mut bc = blockchain.lock().unwrap();
                            let orders = bc.orders.clone();
                            return serde_json::json!({"Orders": orders}).to_string();
                        }
                        "CreateOrder" => {
                            if let Ok(order) = serde_json::from_value::<Order>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                bc.orders.push(order);
                                return serde_json::json!({"Success": "Order created successfully"}).to_string();
                            }
                        }
                        "ConfirmOrder" => {
                            if let Ok(id) = serde_json::from_value::<String>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                match bc.confirm_order(id) {
                                    Ok(_) => return serde_json::json!({"Success": "Order confirmed successfully"}).to_string(),
                                    Err(e) => return serde_json::json!({"Error": e}).to_string(),
                                }
                            }
                        }
                        "CancelOrder" => {
                            if let Ok(id) = serde_json::from_value::<String>(val.clone()) {
                                let mut bc = blockchain.lock().unwrap();
                                match bc.cancel_order(id, "Cancelled via API".to_string()) {
                                    Ok(_) => return serde_json::json!({"Success": "Order cancelled successfully"}).to_string(),
                                    Err(e) => return serde_json::json!({"Error": e}).to_string(),
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        serde_json::json!({"error": "Invalid request"}).to_string()
    }
}
