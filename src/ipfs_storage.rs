use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::franchise_network::{FranchiseNetwork, Sale, SaleItem};

// IPFS интеграция для децентрализованного хранения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSStorage {
    pub gateway_url: String,
    pub local_cache: HashMap<String, String>, // hash -> content
    pub pinned_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredData {
    pub hash: String,
    pub content_type: String,
    pub size: usize,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuData {
    pub items: Vec<MenuItem>,
    pub categories: Vec<String>,
    pub last_updated: u64,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_subunits: u128, // Цена в subunits (1/100 GEL)
    pub category: String,
    pub ingredients: Vec<String>,
    pub image_hash: Option<String>, // IPFS hash изображения
    pub nutritional_info: NutritionalInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalInfo {
    pub calories: u32,
    pub protein: f64,
    pub carbs: f64,
    pub fat: f64,
    pub fiber: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesReport {
    pub node_id: u64,
    pub period_start: u64,
    pub period_end: u64,
    pub total_sales: u32,
    pub total_revenue: f64,
    pub top_items: Vec<TopItem>,
    pub daily_breakdown: Vec<DailySales>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopItem {
    pub item_id: String,
    pub name: String,
    pub quantity_sold: u32,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySales {
    pub date: String,
    pub sales_count: u32,
    pub revenue: f64,
}

impl IPFSStorage {
    pub fn new(gateway_url: String) -> Self {
        Self {
            gateway_url,
            local_cache: HashMap::new(),
            pinned_hashes: Vec::new(),
        }
    }

    // Сохранение данных в IPFS (симуляция)
    pub fn store_data(&mut self, content: &str, content_type: &str) -> Result<String, String> {
        // В реальной реализации здесь был бы вызов IPFS API
        let hash = self.generate_hash(content);
        
        let stored_data = StoredData {
            hash: hash.clone(),
            content_type: content_type.to_string(),
            size: content.len(),
            created_at: chrono::Utc::now().timestamp() as u64,
            metadata: HashMap::new(),
        };
        
        // Сохраняем в локальный кеш
        self.local_cache.insert(hash.clone(), content.to_string());
        self.pinned_hashes.push(hash.clone());
        
        println!("📦 Stored data in IPFS: {}", hash);
        Ok(hash)
    }

    // Получение данных из IPFS
    pub fn retrieve_data(&self, hash: &str) -> Result<String, String> {
        // Сначала проверяем локальный кеш
        if let Some(content) = self.local_cache.get(hash) {
            return Ok(content.clone());
        }
        
        // В реальной реализации здесь был бы запрос к IPFS gateway
        println!("🌐 Retrieving data from IPFS: {}", hash);
        
        // Симуляция получения данных
        Ok(format!("Retrieved data for hash: {}", hash))
    }

    // Сохранение меню в IPFS
    pub fn store_menu(&mut self, menu_data: &MenuData) -> Result<String, String> {
        let content = serde_json::to_string(menu_data)
            .map_err(|e| format!("Failed to serialize menu: {}", e))?;
        
        self.store_data(&content, "application/json")
    }

    // Получение меню из IPFS
    pub fn retrieve_menu(&self, hash: &str) -> Result<MenuData, String> {
        let content = self.retrieve_data(hash)?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize menu: {}", e))
    }

    // Сохранение отчета о продажах
    pub fn store_sales_report(&mut self, report: &SalesReport) -> Result<String, String> {
        let content = serde_json::to_string(report)
            .map_err(|e| format!("Failed to serialize sales report: {}", e))?;
        
        self.store_data(&content, "application/json")
    }

    // Получение отчета о продажах
    pub fn retrieve_sales_report(&self, hash: &str) -> Result<SalesReport, String> {
        let content = self.retrieve_data(hash)?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize sales report: {}", e))
    }

    // Сохранение изображения (симуляция)
    pub fn store_image(&mut self, image_data: &[u8], filename: &str) -> Result<String, String> {
        // В реальной реализации здесь был бы вызов IPFS API для изображений
        let hash = self.generate_hash(&format!("{:?}", image_data));
        
        println!("🖼️  Stored image in IPFS: {} ({} bytes)", hash, image_data.len());
        Ok(hash)
    }

    // Получение изображения
    pub fn retrieve_image(&self, hash: &str) -> Result<Vec<u8>, String> {
        // Симуляция получения изображения
        println!("🖼️  Retrieving image from IPFS: {}", hash);
        Ok(vec![0u8; 1024]) // Заглушка
    }

    // Генерация хеша (упрощенная версия)
    fn generate_hash(&self, content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("Qm{}", hex::encode(&hasher.finalize()[..20]))
    }

    // Синхронизация с IPFS сетью
    pub fn sync_with_network(&mut self) -> Result<Vec<String>, String> {
        println!("🔄 Syncing with IPFS network...");
        
        // В реальной реализации здесь была бы синхронизация с IPFS
        let new_hashes = vec![
            "QmNewHash1".to_string(),
            "QmNewHash2".to_string(),
        ];
        
        for hash in &new_hashes {
            self.pinned_hashes.push(hash.clone());
        }
        
        Ok(new_hashes)
    }

    // Получение статистики хранилища
    pub fn get_storage_stats(&self) -> StorageStats {
        StorageStats {
            total_files: self.local_cache.len(),
            total_size: self.local_cache.values().map(|v| v.len()).sum(),
            pinned_hashes: self.pinned_hashes.len(),
            gateway_url: self.gateway_url.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_files: usize,
    pub total_size: usize,
    pub pinned_hashes: usize,
    pub gateway_url: String,
}

// Интеграция с франшизной сетью
impl IPFSStorage {
    // Сохранение всех данных франшизной сети в IPFS
    pub fn backup_franchise_network(&mut self, network: &FranchiseNetwork) -> Result<String, String> {
        let backup_data = serde_json::to_string(network)
            .map_err(|e| format!("Failed to serialize network: {}", e))?;
        
        self.store_data(&backup_data, "application/json")
    }

    // Создание отчета о продажах для ноды
    pub fn create_sales_report(&self, node_id: u64, network: &FranchiseNetwork, 
                              period_days: u32) -> Result<SalesReport, String> {
        let end_time = chrono::Utc::now().timestamp() as u64;
        let start_time = end_time - (period_days as u64 * 86400);
        
        // Фильтруем продажи по ноде и периоду
        let node_sales: Vec<_> = network.sales.iter()
            .filter(|s| s.node_id == node_id && s.timestamp >= start_time && s.timestamp <= end_time)
            .collect();
        
        let total_sales = node_sales.len() as u32;
        let total_revenue: f64 = node_sales.iter().map(|s| s.price_subunits as f64 / 100.0).sum();
        
        // Анализируем топ товары
        let mut item_counts: HashMap<String, (u32, f64)> = HashMap::new();
        for sale in &node_sales {
            for item in &sale.items {
                let entry = item_counts.entry(item.item_id.clone()).or_insert((0, 0.0));
                entry.0 += item.quantity;
                entry.1 += (item.price_subunits * item.quantity as u128) as f64 / 100.0;
            }
        }
        
        let mut top_items: Vec<TopItem> = item_counts.into_iter()
            .map(|(item_id, (quantity, revenue))| TopItem {
                item_id: item_id.clone(),
                name: format!("Item {}", item_id), // В реальности получали бы из меню
                quantity_sold: quantity,
                revenue,
            })
            .collect();
        
        top_items.sort_by(|a, b| b.quantity_sold.cmp(&a.quantity_sold));
        top_items.truncate(10); // Топ 10
        
        // Дневная разбивка
        let mut daily_sales: HashMap<String, (u32, f64)> = HashMap::new();
        for sale in &node_sales {
            let date = chrono::DateTime::from_timestamp(sale.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            
            let entry = daily_sales.entry(date).or_insert((0, 0.0));
            entry.0 += 1;
            entry.1 += sale.price_subunits as f64 / 100.0;
        }
        
        let daily_breakdown: Vec<DailySales> = daily_sales.into_iter()
            .map(|(date, (sales_count, revenue))| DailySales {
                date,
                sales_count,
                revenue,
            })
            .collect();
        
        Ok(SalesReport {
            node_id,
            period_start: start_time,
            period_end: end_time,
            total_sales,
            total_revenue,
            top_items,
            daily_breakdown,
        })
    }

    // Создание глобального отчета по всей сети
    pub fn create_network_report(&self, network: &FranchiseNetwork) -> Result<NetworkReport, String> {
        let total_nodes = network.nodes.len();
        let active_nodes = network.nodes.values().filter(|n| n.active).count();
        let total_sales = network.sales.len();
        let total_revenue: f64 = network.sales.iter().map(|s| s.price_subunits as f64 / 100.0).sum();
        
        // Анализ по городам
        let mut city_stats: HashMap<String, (u32, f64)> = HashMap::new();
        for sale in &network.sales {
            if let Some(node) = network.nodes.get(&sale.node_id) {
                let entry = city_stats.entry(node.city.clone()).or_insert((0, 0.0));
                entry.0 += 1;
                entry.1 += sale.price_subunits as f64 / 100.0;
            }
        }
        
        let city_breakdown: Vec<CityStats> = city_stats.into_iter()
            .map(|(city, (sales_count, revenue))| CityStats {
                city: city.clone(),
                sales_count,
                revenue,
                node_count: network.nodes.values().filter(|n| n.city == city).count(),
            })
            .collect();
        
        Ok(NetworkReport {
            total_nodes,
            active_nodes,
            total_sales,
            total_revenue,
            total_tokens_minted: network.total_supply,
            city_breakdown,
            generated_at: chrono::Utc::now().timestamp() as u64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkReport {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_sales: usize,
    pub total_revenue: f64,
    pub total_tokens_minted: u64,
    pub city_breakdown: Vec<CityStats>,
    pub generated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityStats {
    pub city: String,
    pub sales_count: u32,
    pub revenue: f64,
    pub node_count: usize,
}
