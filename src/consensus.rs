use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::franchise_network::{FranchiseNetwork, FranchiseNode, NodeType};

// Алгоритм консенсуса: Proof of Sales + Reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusAlgorithm {
    pub minimum_validators: usize,
    pub maximum_validators: usize,
    pub reputation_weights: ReputationWeights,
    pub geographic_distribution: GeographicDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationWeights {
    pub sales_weight: f64,        // 0.4 - вес продаж
    pub reputation_weight: f64,   // 0.3 - вес репутации
    pub geographic_weight: f64,   // 0.2 - вес географического распределения
    pub stake_weight: f64,        // 0.1 - вес токенов
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicDistribution {
    pub max_nodes_per_city: u32,
    pub preferred_cities: Vec<String>,
    pub city_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorScore {
    pub node_id: u64,
    pub total_score: f64,
    pub sales_score: f64,
    pub reputation_score: f64,
    pub geographic_score: f64,
    pub stake_score: f64,
    pub is_selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub selected_validators: Vec<u64>,
    pub validator_scores: Vec<ValidatorScore>,
    pub consensus_timestamp: u64,
    pub block_height: u64,
}

impl Default for ConsensusAlgorithm {
    fn default() -> Self {
        Self {
            minimum_validators: 3,
            maximum_validators: 7,
            reputation_weights: ReputationWeights {
                sales_weight: 0.4,
                reputation_weight: 0.3,
                geographic_weight: 0.2,
                stake_weight: 0.1,
            },
            geographic_distribution: GeographicDistribution {
                max_nodes_per_city: 2,
                preferred_cities: vec![
                    "Tbilisi".to_string(),
                    "Batumi".to_string(),
                    "Kutaisi".to_string(),
                    "Rustavi".to_string(),
                    "Gori".to_string(),
                ],
                city_weights: HashMap::from([
                    ("Tbilisi".to_string(), 1.0),
                    ("Batumi".to_string(), 0.9),
                    ("Kutaisi".to_string(), 0.8),
                    ("Rustavi".to_string(), 0.7),
                    ("Gori".to_string(), 0.6),
                ]),
            },
        }
    }
}

impl ConsensusAlgorithm {
    pub fn new() -> Self {
        Self::default()
    }

    // Основная функция выбора валидаторов
    pub fn select_validators(&self, network: &FranchiseNetwork, block_height: u64) -> ConsensusResult {
        let mut candidates = Vec::new();
        
        // Рассчитываем score для каждой ноды
        for (node_id, node) in &network.nodes {
            if !node.active {
                continue;
            }

            let sales_score = self.calculate_sales_score(*node_id, network);
            let reputation_score = self.calculate_reputation_score(*node_id, network);
            let geographic_score = self.calculate_geographic_score(node, network);
            let stake_score = self.calculate_stake_score(&node.owner_address, network);

            let total_score = sales_score * self.reputation_weights.sales_weight +
                            reputation_score * self.reputation_weights.reputation_weight +
                            geographic_score * self.reputation_weights.geographic_weight +
                            stake_score * self.reputation_weights.stake_weight;

            // Минимальный порог для участия в консенсусе
            if total_score > 0.3 {
                candidates.push(ValidatorScore {
                    node_id: *node_id,
                    total_score,
                    sales_score,
                    reputation_score,
                    geographic_score,
                    stake_score,
                    is_selected: false,
                });
            }
        }

        // Сортируем по общему score
        candidates.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

        // Применяем географическое распределение
        let selected_validators = self.apply_geographic_distribution(candidates.clone(), network);

        ConsensusResult {
            selected_validators: selected_validators.clone(),
            validator_scores: candidates,
            consensus_timestamp: chrono::Utc::now().timestamp() as u64,
            block_height,
        }
    }

    // Расчет score по продажам
    fn calculate_sales_score(&self, node_id: u64, network: &FranchiseNetwork) -> f64 {
        let sales_count = network.sales.iter()
            .filter(|s| s.node_id == node_id)
            .count();

        let total_sales = network.sales.len();
        if total_sales == 0 {
            return 0.0;
        }

        // Нормализуем: доля от общих продаж
        let sales_ratio = sales_count as f64 / total_sales as f64;
        
        // Применяем логарифмическую шкалу для сглаживания
        if sales_ratio > 0.0 {
            (sales_ratio * 10.0).ln().max(0.0) / 2.3 // ln(10) ≈ 2.3
        } else {
            0.0
        }
    }

    // Расчет репутационного score
    fn calculate_reputation_score(&self, node_id: u64, network: &FranchiseNetwork) -> f64 {
        let node = match network.nodes.get(&node_id) {
            Some(n) => n,
            None => return 0.0,
        };

        let current_time = chrono::Utc::now().timestamp() as u64;
        let days_active = (current_time - node.registered_at) / 86400;

        // Базовый score по времени работы
        let time_score = (days_active as f64 / 365.0).min(1.0);

        // Дополнительные факторы репутации
        let consistency_score = self.calculate_consistency_score(node_id, network);
        let quality_score = self.calculate_quality_score(node_id, network);

        // Комбинированный репутационный score
        time_score * 0.5 + consistency_score * 0.3 + quality_score * 0.2
    }

    // Расчет консистентности (регулярность продаж)
    fn calculate_consistency_score(&self, node_id: u64, network: &FranchiseNetwork) -> f64 {
        let node_sales: Vec<_> = network.sales.iter()
            .filter(|s| s.node_id == node_id)
            .collect();

        if node_sales.len() < 2 {
            return 0.5; // Нейтральный score для новых нод
        }

        // Группируем продажи по дням
        let mut daily_sales: HashMap<u64, u32> = HashMap::new();
        for sale in &node_sales {
            let day = sale.timestamp / 86400;
            *daily_sales.entry(day).or_insert(0) += 1;
        }

        // Рассчитываем коэффициент вариации
        let sales_per_day: Vec<f64> = daily_sales.values().map(|&x| x as f64).collect();
        let mean = sales_per_day.iter().sum::<f64>() / sales_per_day.len() as f64;
        
        if mean == 0.0 {
            return 0.5;
        }

        let variance = sales_per_day.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / sales_per_day.len() as f64;
        
        let std_dev = variance.sqrt();
        let coefficient_of_variation = std_dev / mean;

        // Низкий коэффициент вариации = высокая консистентность
        (1.0 - coefficient_of_variation.min(1.0)).max(0.0)
    }

    // Расчет качества (средний чек, разнообразие товаров)
    fn calculate_quality_score(&self, node_id: u64, network: &FranchiseNetwork) -> f64 {
        let node_sales: Vec<_> = network.sales.iter()
            .filter(|s| s.node_id == node_id)
            .collect();

        if node_sales.is_empty() {
            return 0.5;
        }

        // Средний чек
        let total_revenue: f64 = node_sales.iter().map(|s| s.price_gel).sum();
        let avg_check = total_revenue / node_sales.len() as f64;

        // Разнообразие товаров
        let mut unique_items = std::collections::HashSet::new();
        for sale in &node_sales {
            for item in &sale.items {
                unique_items.insert(&item.item_id);
            }
        }

        let diversity_score = (unique_items.len() as f64 / 10.0).min(1.0); // Нормализуем к 10 товарам
        let revenue_score = (avg_check / 50.0).min(1.0); // Нормализуем к 50 GEL

        (diversity_score + revenue_score) / 2.0
    }

    // Расчет географического score
    fn calculate_geographic_score(&self, node: &FranchiseNode, network: &FranchiseNetwork) -> f64 {
        // Базовый вес города
        let city_weight = self.geographic_distribution.city_weights
            .get(&node.city)
            .copied()
            .unwrap_or(0.5);

        // Поощряем разнообразие городов
        let nodes_in_city = network.nodes.values()
            .filter(|n| n.city == node.city && n.active)
            .count();

        let diversity_bonus = if nodes_in_city <= self.geographic_distribution.max_nodes_per_city as usize {
            1.0
        } else {
            0.5 // Штраф за перенаселение города
        };

        city_weight * diversity_bonus
    }

    // Расчет stake score
    fn calculate_stake_score(&self, owner_address: &str, network: &FranchiseNetwork) -> f64 {
        let balance = network.get_wallet_balance(owner_address);
        let total_supply = network.total_supply;

        if total_supply == 0 {
            return 0.0;
        }

        // Доля от общего количества токенов
        balance as f64 / total_supply as f64
    }

    // Применение географического распределения
    fn apply_geographic_distribution(&self, candidates: Vec<ValidatorScore>, network: &FranchiseNetwork) -> Vec<u64> {
        let mut selected = Vec::new();
        let mut city_counts: HashMap<String, u32> = HashMap::new();

        // Сначала выбираем лучших кандидатов с учетом географического распределения
        for candidate in &candidates {
            let node = network.nodes.get(&candidate.node_id).unwrap();
            let current_city_count = city_counts.get(&node.city).copied().unwrap_or(0);

            // Проверяем лимит по городу
            if current_city_count < self.geographic_distribution.max_nodes_per_city {
                selected.push(candidate.node_id);
                city_counts.insert(node.city.clone(), current_city_count + 1);

                if selected.len() >= self.maximum_validators {
                    break;
                }
            }
        }

        // Если не набрали достаточно валидаторов, добавляем лучших оставшихся
        if selected.len() < self.minimum_validators {
            for candidate in &candidates {
                if !selected.contains(&candidate.node_id) {
                    selected.push(candidate.node_id);
                    if selected.len() >= self.minimum_validators {
                        break;
                    }
                }
            }
        }

        selected
    }

    // Валидация блока
    pub fn validate_block(&self, block: &Block, validators: &[u64], network: &FranchiseNetwork) -> bool {
        // Проверяем, что блок подписан достаточным количеством валидаторов
        let required_signatures = (validators.len() * 2 / 3) + 1; // 2/3 + 1
        let actual_signatures = block.signatures.len();

        if actual_signatures < required_signatures {
            return false;
        }

        // Проверяем, что все подписи от валидных валидаторов
        for signature in &block.signatures {
            if !validators.contains(&signature.validator_id) {
                return false;
            }
        }

        // Дополнительные проверки блока
        self.validate_block_content(block, network)
    }

    // Валидация содержимого блока
    fn validate_block_content(&self, block: &Block, network: &FranchiseNetwork) -> bool {
        // Проверяем хеш блока
        let calculated_hash = block.calculate_hash();
        if calculated_hash != block.hash {
            return false;
        }

        // Проверяем транзакции
        for transaction in &block.transactions {
            if !self.validate_transaction(transaction, network) {
                return false;
            }
        }

        true
    }

    // Валидация транзакции
    fn validate_transaction(&self, transaction: &Transaction, network: &FranchiseNetwork) -> bool {
        // Проверяем, что нода существует и активна
        if let Some(node) = network.nodes.get(&transaction.node_id) {
            if !node.active {
                return false;
            }
        } else {
            return false;
        }

        // Проверяем подпись транзакции
        transaction.verify_signature()
    }
}

// Структуры для блокчейна
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub signatures: Vec<BlockSignature>,
    pub validator_rewards: HashMap<u64, u64>, // validator_id -> reward
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub node_id: u64,
    pub transaction_type: TransactionType,
    pub data: serde_json::Value,
    pub signature: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Sale,
    NodeRegistration,
    TokenTransfer,
    Governance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSignature {
    pub validator_id: u64,
    pub signature: String,
    pub timestamp: u64,
}

impl Block {
    pub fn new(height: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut block = Self {
            height,
            timestamp,
            previous_hash,
            hash: String::new(),
            transactions,
            signatures: Vec::new(),
            validator_rewards: HashMap::new(),
        };
        
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(self.height.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        
        for transaction in &self.transactions {
            hasher.update(transaction.id.as_bytes());
        }
        
        hex::encode(hasher.finalize())
    }

    pub fn add_signature(&mut self, validator_id: u64, signature: String) {
        self.signatures.push(BlockSignature {
            validator_id,
            signature,
            timestamp: chrono::Utc::now().timestamp() as u64,
        });
    }
}

impl Transaction {
    pub fn new(node_id: u64, transaction_type: TransactionType, data: serde_json::Value) -> Self {
        let id = format!("tx_{}_{}", node_id, chrono::Utc::now().timestamp());
        Self {
            id,
            node_id,
            transaction_type,
            data,
            signature: String::new(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn sign(&mut self, private_key: &str) {
        // Упрощенная подпись (в реальности используйте криптографические библиотеки)
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(private_key.as_bytes());
        self.signature = hex::encode(hasher.finalize());
    }

    pub fn verify_signature(&self) -> bool {
        // Упрощенная проверка подписи
        !self.signature.is_empty()
    }
}
