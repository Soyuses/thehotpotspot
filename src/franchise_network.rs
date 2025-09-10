use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;

// Константы токеномики
pub const SCALE: u64 = 100; // 1 токен = 100 subunits
pub const OWNER_OWNER_SHARE: u64 = 51; // 0.51 токена владельцу собственной точки
pub const OWNER_BUYER_SHARE: u64 = 49; // 0.49 токена покупателю
pub const FRANCHISE_OWNER_SHARE: u64 = 48; // 0.48 токена франчайзи
pub const FRANCHISE_ROYALTY_SHARE: u64 = 3; // 0.03 токена основателю (роялти)
pub const FRANCHISE_BUYER_SHARE: u64 = 49; // 0.49 токена покупателю

// Типы нод
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    OWNER,    // Собственная точка основателя
    FRANCHISE, // Франшизная точка
}

// Структура ноды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranchiseNode {
    pub node_id: u64,
    pub owner_address: String,
    pub node_type: NodeType,
    pub city: String,
    pub active: bool,
    pub registered_at: u64,
    pub pos_systems: Vec<String>, // Whitelisted POS systems
}

// Структура продажи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub sale_id: String,
    pub node_id: u64,
    pub timestamp: u64,
    pub price_subunits: u128, // Цена в subunits (1/100 GEL)
    pub check_address: String,
    pub buyer_meta: String,
    pub pos_id: String,
    pub items: Vec<SaleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItem {
    pub item_id: String,
    pub quantity: u32,
    pub price_subunits: u128, // Цена в subunits (1/100 GEL)
}

// Структура эмиссии токенов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMinting {
    pub mint_id: String,
    pub sale_id: String,
    pub minted_units: u64,
    pub owner_units: u64,
    pub buyer_units: u64,
    pub royalty_units: u64,
}

// Структура кошелька
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub owner_type: String, // "master", "franchise", "buyer"
    pub owner_id: String,
    pub created_at: u64,
    pub balance: u64, // в subunits
}

// Основная сеть франшиз
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranchiseNetwork {
    pub master_owner: String,
    pub nodes: HashMap<u64, FranchiseNode>,
    pub sales: Vec<Sale>,
    pub token_mintings: Vec<TokenMinting>,
    pub wallets: HashMap<String, Wallet>,
    pub whitelisted_pos: HashMap<String, bool>,
    pub total_supply: u64,
    pub next_node_id: u64,
}

impl FranchiseNetwork {
    pub fn new(master_owner: String) -> Self {
        let mut network = Self {
            master_owner: master_owner.clone(),
            nodes: HashMap::new(),
            sales: Vec::new(),
            token_mintings: Vec::new(),
            wallets: HashMap::new(),
            whitelisted_pos: HashMap::new(),
            total_supply: SCALE, // Генезис: 1 токен
            next_node_id: 1,
        };
        
        // Создаем генезис кошелек для master owner
        network.wallets.insert(master_owner.clone(), Wallet {
            address: master_owner.clone(),
            owner_type: "master".to_string(),
            owner_id: "master".to_string(),
            created_at: 0,
            balance: SCALE, // Генезис: 1 токен
        });
        
        network
    }

    // Регистрация новой ноды
    pub fn register_node(&mut self, owner_address: String, node_type: NodeType, city: String) -> Result<u64, String> {
        let node_id = self.next_node_id;
        self.next_node_id += 1;

        let node = FranchiseNode {
            node_id,
            owner_address: owner_address.clone(),
            node_type,
            city,
            active: true,
            registered_at: chrono::Utc::now().timestamp() as u64,
            pos_systems: Vec::new(),
        };

        self.nodes.insert(node_id, node);

        // Создаем кошелек для владельца ноды, если его нет
        if !self.wallets.contains_key(&owner_address) {
            self.wallets.insert(owner_address.clone(), Wallet {
                address: owner_address,
                owner_type: "franchise".to_string(),
                owner_id: node_id.to_string(),
                created_at: chrono::Utc::now().timestamp() as u64,
                balance: 0,
            });
        }

        Ok(node_id)
    }

    // Создание детерминированного чек-адреса
    pub fn create_check_address(&mut self, sale_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(sale_id.as_bytes());
        hasher.update(b"check_address");
        let hash = hasher.finalize();
        let address = format!("CheckWallet_{}", hex::encode(&hash[..8]));
        
        // Создаем кошелек для чек-адреса
        if !self.wallets.contains_key(&address) {
            self.wallets.insert(address.clone(), Wallet {
                address: address.clone(),
                owner_type: "buyer".to_string(),
                owner_id: sale_id.to_string(),
                created_at: chrono::Utc::now().timestamp() as u64,
                balance: 0,
            });
        }
        
        address
    }

    // Запись продажи и эмиссия токенов
    pub fn record_sale(&mut self, node_id: u64, sale_id: String, price_subunits: u128, 
                      buyer_meta: String, pos_id: String, items: Vec<SaleItem>) -> Result<TokenMinting, String> {
        
        // Проверяем, что нода существует и активна
        let node = self.nodes.get(&node_id)
            .ok_or("Node not found")?;
        
        if !node.active {
            return Err("Node is not active".to_string());
        }

        // Проверяем, что POS система в whitelist
        if !self.whitelisted_pos.get(&pos_id).unwrap_or(&false) {
            return Err("POS system not whitelisted".to_string());
        }

        // Создаем чек-адрес
        let check_address = self.create_check_address(&sale_id);

        // Создаем запись о продаже
        let sale = Sale {
            sale_id: sale_id.clone(),
            node_id,
            timestamp: chrono::Utc::now().timestamp() as u64,
            price_subunits,
            check_address: check_address.clone(),
            buyer_meta,
            pos_id,
            items,
        };

        self.sales.push(sale);

        // Эмитируем и распределяем токены
        let minting = self.mint_and_distribute(node_id, &check_address)?;
        self.token_mintings.push(minting.clone());

        Ok(minting)
    }

    // Эмиссия и распределение токенов
    fn mint_and_distribute(&mut self, node_id: u64, buyer_address: &str) -> Result<TokenMinting, String> {
        let node = self.nodes.get(&node_id)
            .ok_or("Node not found")?;

        let minted_units = SCALE; // 1 токен = 100 subunits
        self.total_supply += minted_units;

        let (owner_units, buyer_units, royalty_units) = match node.node_type {
            NodeType::OWNER => {
                // Собственная точка: 51% владельцу, 49% покупателю
                (OWNER_OWNER_SHARE, OWNER_BUYER_SHARE, 0)
            },
            NodeType::FRANCHISE => {
                // Франшиза: 48% франчайзи, 3% роялти, 49% покупателю
                (FRANCHISE_OWNER_SHARE, FRANCHISE_BUYER_SHARE, FRANCHISE_ROYALTY_SHARE)
            }
        };

        // Обновляем балансы
        if let Some(owner_wallet) = self.wallets.get_mut(&node.owner_address) {
            owner_wallet.balance += owner_units;
        }

        if let Some(buyer_wallet) = self.wallets.get_mut(buyer_address) {
            buyer_wallet.balance += buyer_units;
        }

        if royalty_units > 0 {
            if let Some(master_wallet) = self.wallets.get_mut(&self.master_owner) {
                master_wallet.balance += royalty_units;
            }
        }

        let minting = TokenMinting {
            mint_id: format!("mint_{}_{}", node_id, chrono::Utc::now().timestamp()),
            sale_id: format!("sale_{}", node_id),
            minted_units,
            owner_units,
            buyer_units,
            royalty_units,
        };

        Ok(minting)
    }

    // Добавление POS системы в whitelist
    pub fn whitelist_pos(&mut self, pos_id: String) {
        self.whitelisted_pos.insert(pos_id, true);
    }

    // Получение баланса кошелька
    pub fn get_wallet_balance(&self, address: &str) -> u64 {
        self.wallets.get(address).map(|w| w.balance).unwrap_or(0)
    }

    // Получение информации о ноде
    pub fn get_node_info(&self, node_id: u64) -> Option<&FranchiseNode> {
        self.nodes.get(&node_id)
    }

    // Получение статистики сети
    pub fn get_network_stats(&self) -> NetworkStats {
        let total_nodes = self.nodes.len();
        let active_nodes = self.nodes.values().filter(|n| n.active).count();
        let total_sales = self.sales.len();
        let total_tokens_minted = self.total_supply;

        NetworkStats {
            total_nodes,
            active_nodes,
            total_sales,
            total_tokens_minted,
            master_owner_balance: self.get_wallet_balance(&self.master_owner),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_sales: usize,
    pub total_tokens_minted: u64,
    pub master_owner_balance: u64,
}
