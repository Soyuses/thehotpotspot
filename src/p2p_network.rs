use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::franchise_network::FranchiseNetwork;
use crate::consensus::{ConsensusAlgorithm, ConsensusResult, Block, Transaction};

// P2P сообщения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P2PMessage {
    // Обнаружение сети
    Ping { node_id: u64, timestamp: u64 },
    Pong { node_id: u64, timestamp: u64 },
    
    // Синхронизация данных
    SyncRequest { from_height: u64 },
    SyncResponse { blocks: Vec<Block> },
    
    // Транзакции
    NewTransaction { transaction: Transaction },
    TransactionBroadcast { transaction: Transaction },
    
    // Консенсус
    ConsensusRequest { block_height: u64 },
    ConsensusResponse { result: ConsensusResult },
    
    // Блоки
    NewBlock { block: Block },
    BlockRequest { block_height: u64 },
    BlockResponse { block: Block },
    
    // Статус ноды
    NodeStatus { node_id: u64, status: NodeStatus },
    NetworkStats { stats: NetworkStats },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub node_id: u64,
    pub is_active: bool,
    pub last_seen: u64,
    pub block_height: u64,
    pub peer_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_blocks: u64,
    pub network_hashrate: f64,
}

// P2P узел
pub struct P2PNode {
    pub node_id: u64,
    pub address: SocketAddr,
    pub peers: Arc<Mutex<HashMap<u64, PeerInfo>>>,
    pub franchise_network: Arc<Mutex<FranchiseNetwork>>,
    pub consensus: Arc<Mutex<ConsensusAlgorithm>>,
    pub blockchain: Arc<Mutex<Vec<Block>>>,
    pub pending_transactions: Arc<Mutex<Vec<Transaction>>>,
    pub is_running: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub node_id: u64,
    pub address: SocketAddr,
    pub last_ping: u64,
    pub is_connected: bool,
    pub block_height: u64,
}

impl P2PNode {
    pub fn new(node_id: u64, address: SocketAddr, franchise_network: Arc<Mutex<FranchiseNetwork>>) -> Self {
        Self {
            node_id,
            address,
            peers: Arc::new(Mutex::new(HashMap::new())),
            franchise_network,
            consensus: Arc::new(Mutex::new(ConsensusAlgorithm::new())),
            blockchain: Arc::new(Mutex::new(Vec::new())),
            pending_transactions: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    // Запуск P2P узла
    pub fn start(&self) {
        println!("🌐 Starting P2P Node {} on {}", self.node_id, self.address);
        
        *self.is_running.lock().unwrap() = true;
        
        // Запускаем сервер для входящих соединений
        let server_handle = self.start_server();
        
        // Запускаем клиент для исходящих соединений
        let client_handle = self.start_client();
        
        // Запускаем консенсус
        let consensus_handle = self.start_consensus();
        
        // Запускаем синхронизацию
        let sync_handle = self.start_sync();
        
        // Ждем завершения
        server_handle.join().unwrap();
        client_handle.join().unwrap();
        consensus_handle.join().unwrap();
        sync_handle.join().unwrap();
    }

    // Сервер для входящих соединений
    fn start_server(&self) -> thread::JoinHandle<()> {
        let node_id = self.node_id;
        let peers = Arc::clone(&self.peers);
        let franchise_network = Arc::clone(&self.franchise_network);
        let blockchain = Arc::clone(&self.blockchain);
        let pending_transactions = Arc::clone(&self.pending_transactions);
        let is_running = Arc::clone(&self.is_running);
        
        thread::spawn(move || {
            let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind server");
            println!("🔗 P2P Server listening on 0.0.0.0:8080");
            
            for stream in listener.incoming() {
                if !*is_running.lock().unwrap() {
                    break;
                }
                
                match stream {
                    Ok(stream) => {
                        let peers = Arc::clone(&peers);
                        let franchise_network = Arc::clone(&franchise_network);
                        let blockchain = Arc::clone(&blockchain);
                        let pending_transactions = Arc::clone(&pending_transactions);
                        
                        thread::spawn(move || {
                            Self::handle_incoming_connection(stream, node_id, peers, franchise_network, blockchain, pending_transactions);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        })
    }

    // Клиент для исходящих соединений
    fn start_client(&self) -> thread::JoinHandle<()> {
        let node_id = self.node_id;
        let peers = Arc::clone(&self.peers);
        let is_running = Arc::clone(&self.is_running);
        
        thread::spawn(move || {
            // Список известных пиров (в реальности это может быть из конфига или DNS)
            let known_peers = vec![
                "127.0.0.1:8081".parse::<SocketAddr>().unwrap(),
                "127.0.0.1:8082".parse::<SocketAddr>().unwrap(),
            ];
            
            while *is_running.lock().unwrap() {
                for peer_addr in &known_peers {
                    if let Ok(stream) = TcpStream::connect(peer_addr) {
                        let peers = Arc::clone(&peers);
                        thread::spawn(move || {
                            Self::handle_outgoing_connection(stream, node_id, peers);
                        });
                    }
                }
                
                thread::sleep(Duration::from_secs(30)); // Переподключение каждые 30 секунд
            }
        })
    }

    // Консенсус
    fn start_consensus(&self) -> thread::JoinHandle<()> {
        let node_id = self.node_id;
        let peers = Arc::clone(&self.peers);
        let franchise_network = Arc::clone(&self.franchise_network);
        let consensus = Arc::clone(&self.consensus);
        let blockchain = Arc::clone(&self.blockchain);
        let pending_transactions = Arc::clone(&self.pending_transactions);
        let is_running = Arc::clone(&self.is_running);
        
        thread::spawn(move || {
            while *is_running.lock().unwrap() {
                // Ждем накопления транзакций или таймаут
                thread::sleep(Duration::from_secs(10));
                
                let pending_count = pending_transactions.lock().unwrap().len();
                if pending_count == 0 {
                    continue;
                }
                
                // Выбираем валидаторов
                let network = franchise_network.lock().unwrap();
                let consensus_alg = consensus.lock().unwrap();
                let current_height = blockchain.lock().unwrap().len() as u64;
                
                let consensus_result = consensus_alg.select_validators(&network, current_height);
                drop(network);
                drop(consensus_alg);
                
                // Проверяем, являемся ли мы валидатором
                if consensus_result.selected_validators.contains(&node_id) {
                    println!("🎯 Node {} selected as validator for block {}", node_id, current_height);
                    
                    // Создаем новый блок
                    let mut transactions = pending_transactions.lock().unwrap();
                    let block_transactions = transactions.drain(..).collect();
                    drop(transactions);
                    
                    let blockchain_guard = blockchain.lock().unwrap();
                    let previous_hash = blockchain_guard.last()
                        .map(|b| b.hash.clone())
                        .unwrap_or_else(|| "genesis".to_string());
                    drop(blockchain_guard);
                    
                    let mut new_block = Block::new(current_height, previous_hash, block_transactions);
                    
                    // Подписываем блок
                    new_block.add_signature(node_id, format!("signature_{}_{}", node_id, current_height));
                    
                    // Добавляем блок в блокчейн
                    blockchain.lock().unwrap().push(new_block.clone());
                    
                    // Рассылаем блок другим узлам
                    Self::broadcast_message(&peers, P2PMessage::NewBlock { block: new_block });
                }
            }
        })
    }

    // Синхронизация
    fn start_sync(&self) -> thread::JoinHandle<()> {
        let node_id = self.node_id;
        let peers = Arc::clone(&self.peers);
        let blockchain = Arc::clone(&self.blockchain);
        let is_running = Arc::clone(&self.is_running);
        
        thread::spawn(move || {
            while *is_running.lock().unwrap() {
                thread::sleep(Duration::from_secs(60)); // Синхронизация каждую минуту
                
                let current_height = blockchain.lock().unwrap().len() as u64;
                
                // Запрашиваем синхронизацию у пиров
                Self::broadcast_message(&peers, P2PMessage::SyncRequest { from_height: current_height });
            }
        })
    }

    // Обработка входящего соединения
    fn handle_incoming_connection(
        mut stream: TcpStream,
        node_id: u64,
        peers: Arc<Mutex<HashMap<u64, PeerInfo>>>,
        franchise_network: Arc<Mutex<FranchiseNetwork>>,
        blockchain: Arc<Mutex<Vec<Block>>>,
        pending_transactions: Arc<Mutex<Vec<Transaction>>>,
    ) {
        let peer_addr = stream.peer_addr().unwrap();
        println!("📡 Incoming connection from {}", peer_addr);
        
        let reader = BufReader::new(stream.try_clone().unwrap());
        
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(message) = serde_json::from_str::<P2PMessage>(&line) {
                    Self::handle_message(
                        message,
                        &mut stream,
                        node_id,
                        &peers,
                        &franchise_network,
                        &blockchain,
                        &pending_transactions,
                    );
                }
            }
        }
    }

    // Обработка исходящего соединения
    fn handle_outgoing_connection(
        mut stream: TcpStream,
        node_id: u64,
        peers: Arc<Mutex<HashMap<u64, PeerInfo>>>,
    ) {
        let peer_addr = stream.peer_addr().unwrap();
        println!("🔗 Connected to peer {}", peer_addr);
        
        // Отправляем ping
        let ping = P2PMessage::Ping {
            node_id,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        
        if let Ok(json) = serde_json::to_string(&ping) {
            let _ = writeln!(stream, "{}", json);
        }
        
        // Добавляем пира
        let mut peers_guard = peers.lock().unwrap();
        peers_guard.insert(node_id, PeerInfo {
            node_id,
            address: peer_addr,
            last_ping: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            is_connected: true,
            block_height: 0,
        });
    }

    // Обработка сообщений
    fn handle_message(
        message: P2PMessage,
        stream: &mut TcpStream,
        node_id: u64,
        peers: &Arc<Mutex<HashMap<u64, PeerInfo>>>,
        franchise_network: &Arc<Mutex<FranchiseNetwork>>,
        blockchain: &Arc<Mutex<Vec<Block>>>,
        pending_transactions: &Arc<Mutex<Vec<Transaction>>>,
    ) {
        match message {
            P2PMessage::Ping { node_id: peer_id, timestamp } => {
                println!("📡 Ping from node {}", peer_id);
                
                let pong = P2PMessage::Pong {
                    node_id,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };
                
                if let Ok(json) = serde_json::to_string(&pong) {
                    let _ = writeln!(stream, "{}", json);
                }
            }
            
            P2PMessage::Pong { node_id: peer_id, timestamp: _ } => {
                println!("📡 Pong from node {}", peer_id);
                
                // Обновляем информацию о пире
                let mut peers_guard = peers.lock().unwrap();
                if let Some(peer) = peers_guard.get_mut(&peer_id) {
                    peer.last_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                }
            }
            
            P2PMessage::SyncRequest { from_height } => {
                println!("📡 Sync request from height {}", from_height);
                
                let blockchain_guard = blockchain.lock().unwrap();
                let blocks_to_send: Vec<Block> = blockchain_guard
                    .iter()
                    .skip(from_height as usize)
                    .cloned()
                    .collect();
                drop(blockchain_guard);
                
                let sync_response = P2PMessage::SyncResponse { blocks: blocks_to_send };
                if let Ok(json) = serde_json::to_string(&sync_response) {
                    let _ = writeln!(stream, "{}", json);
                }
            }
            
            P2PMessage::SyncResponse { blocks } => {
                println!("📡 Received {} blocks in sync response", blocks.len());
                
                let mut blockchain_guard = blockchain.lock().unwrap();
                for block in blocks {
                    // Проверяем, что блок еще не существует
                    if blockchain_guard.len() <= block.height as usize {
                        blockchain_guard.push(block);
                    }
                }
            }
            
            P2PMessage::NewTransaction { transaction } => {
                println!("📡 New transaction: {}", transaction.id);
                
                let mut pending = pending_transactions.lock().unwrap();
                pending.push(transaction);
            }
            
            P2PMessage::NewBlock { block } => {
                println!("📡 New block: height {}", block.height);
                
                let mut blockchain_guard = blockchain.lock().unwrap();
                if blockchain_guard.len() <= block.height as usize {
                    blockchain_guard.push(block);
                }
            }
            
            _ => {
                println!("📡 Unhandled message type");
            }
        }
    }

    // Рассылка сообщений всем пирам
    fn broadcast_message(peers: &Arc<Mutex<HashMap<u64, PeerInfo>>>, message: P2PMessage) {
        let peers_guard = peers.lock().unwrap();
        for peer in peers_guard.values() {
            if peer.is_connected {
                if let Ok(mut stream) = TcpStream::connect(peer.address) {
                    if let Ok(json) = serde_json::to_string(&message) {
                        let _ = writeln!(stream, "{}", json);
                    }
                }
            }
        }
    }

    // Остановка узла
    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
        println!("🛑 P2P Node {} stopped", self.node_id);
    }

    // Получение статистики сети
    pub fn get_network_stats(&self) -> NetworkStats {
        let peers_guard = self.peers.lock().unwrap();
        let blockchain_guard = self.blockchain.lock().unwrap();
        
        NetworkStats {
            total_nodes: peers_guard.len() + 1, // +1 для себя
            active_nodes: peers_guard.values().filter(|p| p.is_connected).count() + 1,
            total_blocks: blockchain_guard.len() as u64,
            network_hashrate: 0.0, // Упрощенная версия
        }
    }
}
