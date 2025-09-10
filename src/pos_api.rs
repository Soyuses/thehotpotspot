use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::{Serialize, Deserialize};
use crate::franchise_network::{FranchiseNetwork, NodeType, SaleItem};

// API запросы для POS систем
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterNodeRequest {
    pub owner_address: String,
    pub node_type: String, // "OWNER" or "FRANCHISE"
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordSaleRequest {
    pub node_id: u64,
    pub sale_id: String,
    pub price_subunits: u128, // Цена в subunits (1/100 GEL)
    pub buyer_meta: String,
    pub pos_id: String,
    pub items: Vec<SaleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistPosRequest {
    pub pos_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// POS API сервер
pub struct PosApiServer {
    franchise_network: Arc<Mutex<FranchiseNetwork>>,
    port: u16,
}

impl PosApiServer {
    pub fn new(franchise_network: Arc<Mutex<FranchiseNetwork>>, port: u16) -> Self {
        PosApiServer { franchise_network, port }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");
        
        println!("🏪 POS API Server started on port {}", self.port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let franchise_network = Arc::clone(&self.franchise_network);
                    thread::spawn(move || {
                        Self::handle_client(stream, franchise_network);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, franchise_network: Arc<Mutex<FranchiseNetwork>>) {
        let mut buffer = [0; 4096];
        
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]);
                let response = Self::process_request(&request, franchise_network);
                
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

    fn process_request(request: &str, franchise_network: Arc<Mutex<FranchiseNetwork>>) -> String {
        // Handle OPTIONS request for CORS
        if request.starts_with("OPTIONS") {
            return "{}".to_string();
        }

        // Extract JSON from POST request
        let json_start = request.find("\r\n\r\n");
        if json_start.is_none() {
            return serde_json::json!({"success": false, "error": "Invalid request format"}).to_string();
        }

        let json_start = json_start.unwrap() + 4;
        let json_str = &request[json_start..];
        
        // Try to parse as wrapped enum (e.g., {"RegisterNode": {...}})
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
            if let Some(obj) = value.as_object() {
                for (key, val) in obj {
                    match key.as_str() {
                        "RegisterNode" => {
                            if let Ok(req) = serde_json::from_value::<RegisterNodeRequest>(val.clone()) {
                                let mut network = franchise_network.lock().unwrap();
                                let node_type = match req.node_type.as_str() {
                                    "OWNER" => NodeType::OWNER,
                                    "FRANCHISE" => NodeType::FRANCHISE,
                                    _ => {
                                        return serde_json::json!({
                                            "success": false,
                                            "error": "Invalid node type. Use 'OWNER' or 'FRANCHISE'"
                                        }).to_string();
                                    }
                                };
                                
                                match network.register_node(req.owner_address, node_type, req.city) {
                                    Ok(node_id) => {
                                        return serde_json::json!({
                                            "success": true,
                                            "data": {"node_id": node_id}
                                        }).to_string();
                                    }
                                    Err(e) => {
                                        return serde_json::json!({
                                            "success": false,
                                            "error": e
                                        }).to_string();
                                    }
                                }
                            }
                        }
                        "RecordSale" => {
                            if let Ok(req) = serde_json::from_value::<RecordSaleRequest>(val.clone()) {
                                let mut network = franchise_network.lock().unwrap();
                                match network.record_sale(
                                    req.node_id,
                                    req.sale_id,
                                    req.price_subunits,
                                    req.buyer_meta,
                                    req.pos_id,
                                    req.items
                                ) {
                                    Ok(minting) => {
                                        return serde_json::json!({
                                            "success": true,
                                            "data": minting
                                        }).to_string();
                                    }
                                    Err(e) => {
                                        return serde_json::json!({
                                            "success": false,
                                            "error": e
                                        }).to_string();
                                    }
                                }
                            }
                        }
                        "WhitelistPos" => {
                            if let Ok(req) = serde_json::from_value::<WhitelistPosRequest>(val.clone()) {
                                let mut network = franchise_network.lock().unwrap();
                                network.whitelist_pos(req.pos_id);
                                return serde_json::json!({
                                    "success": true,
                                    "data": {"message": "POS system whitelisted"}
                                }).to_string();
                            }
                        }
                        "GetNetworkStats" => {
                            let network = franchise_network.lock().unwrap();
                            let stats = network.get_network_stats();
                            return serde_json::json!({
                                "success": true,
                                "data": stats
                            }).to_string();
                        }
                        "GetWalletBalance" => {
                            if let Ok(address) = serde_json::from_value::<String>(val.clone()) {
                                let network = franchise_network.lock().unwrap();
                                let balance = network.get_wallet_balance(&address);
                                return serde_json::json!({
                                    "success": true,
                                    "data": {"address": address, "balance": balance}
                                }).to_string();
                            }
                        }
                        "GetNodeInfo" => {
                            if let Ok(node_id) = serde_json::from_value::<u64>(val.clone()) {
                                let network = franchise_network.lock().unwrap();
                                if let Some(node_info) = network.get_node_info(node_id) {
                                    return serde_json::json!({
                                        "success": true,
                                        "data": node_info
                                    }).to_string();
                                } else {
                                    return serde_json::json!({
                                        "success": false,
                                        "error": "Node not found"
                                    }).to_string();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        
        serde_json::json!({"success": false, "error": "Invalid request"}).to_string()
    }
}
