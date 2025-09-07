 #[cfg(test)]
use std::fmt;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
#[cfg(test)]
use qrcode::{QrCode, render::svg};
use hex;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

mod simple_server;
use simple_server::SimpleServer;

// Utility Token for voting
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UtilityToken {
    symbol: String,
    total_supply: f64,
    voting_power_per_token: f64,
}

impl UtilityToken {
    fn new(symbol: String) -> Self {
        UtilityToken {
            symbol,
            total_supply: 0.0,
            voting_power_per_token: 1.0,
        }
    }

    fn issue_voting_tokens(&mut self, amount: f64) -> f64 {
        self.total_supply += amount;
        amount * self.voting_power_per_token
    }
}

// User Roles based on token holdings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum UserRole {
    Unauthorized,
    Starter,      // > 1% of total supply
    MiddlePlayer, // > 5% of total supply
    BigStack,     // > 10% of total supply
    MainOwner,    // Special role
}

impl UserRole {
    fn from_percentage(percentage: f64) -> Self {
        match percentage {
            p if p > 10.0 => UserRole::BigStack,
            p if p > 5.0 => UserRole::MiddlePlayer,
            p if p > 1.0 => UserRole::Starter,
            _ => UserRole::Unauthorized,
        }
    }
}

// Check structure for account activation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Check {
    check_id: String,
    qr_code: String,
    activation_code: String,
    amount: f64,
    food_items: Vec<String>,
    timestamp: u64,
    is_activated: bool,
    blockchain_account: String,
}

impl Check {
    fn new(amount: f64, food_items: Vec<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let check_id = Self::generate_check_id(amount, &food_items, timestamp);
        let activation_code = Self::generate_activation_code();
        let blockchain_account = Self::generate_blockchain_account();
        
        // Generate QR code data
        let qr_data = format!("{}|{}|{}", check_id, activation_code, blockchain_account);
        let qr_code = Self::generate_qr_code(&qr_data);
        
        Check {
            check_id,
            qr_code,
            activation_code,
            amount,
            food_items,
            timestamp,
            is_activated: false,
            blockchain_account,
        }
    }

    fn generate_check_id(amount: f64, food_items: &[String], timestamp: u64) -> String {
        let data = format!("{}{}{}", amount, food_items.join(""), timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(&hasher.finalize()[..16])
    }

    fn generate_activation_code() -> String {
        let mut rng = fastrand::Rng::new();
        format!("{:06}", rng.u32(100000..999999))
    }

    fn generate_blockchain_account() -> String {
        let mut rng = fastrand::Rng::new();
        let mut account = String::new();
        for _ in 0..8 {
            account.push_str(&format!("{:x}", rng.u32(0..16)));
        }
        format!("0x{}", account)
    }

    fn generate_qr_code(data: &str) -> String {
        // In a real implementation, this would generate an actual QR code
        // For now, we'll return a placeholder
        format!("QR_CODE_{}", data)
    }
}

// Blockchain Account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockchainAccount {
    address: String,
    status: AccountStatus,
    security_tokens: f64,
    utility_tokens: f64,
    personal_data: Option<PersonalData>,
    created_timestamp: u64,
    activated_timestamp: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum AccountStatus {
    Sleep,      // 100% tokens go to owner
    Active,     // Normal operation
    ForSale,    // Account is listed for sale
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonalData {
    name: String,
    email: String,
    phone: String,
    wallet_address: Option<String>,
}

impl BlockchainAccount {
    fn new(address: String) -> Self {
        BlockchainAccount {
            address,
            status: AccountStatus::Sleep,
            security_tokens: 0.0,
            utility_tokens: 0.0,
            personal_data: None,
            created_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            activated_timestamp: None,
        }
    }

    fn activate(&mut self, personal_data: PersonalData) -> Result<(), String> {
        if self.status != AccountStatus::Sleep {
            return Err("Account is not in sleep status".to_string());
        }
        
        self.status = AccountStatus::Active;
        self.personal_data = Some(personal_data);
        self.activated_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(())
    }

    fn list_for_sale(&mut self) -> Result<(), String> {
        if self.status != AccountStatus::Active {
            return Err("Only active accounts can be listed for sale".to_string());
        }
        self.status = AccountStatus::ForSale;
        Ok(())
    }
}

// Enhanced Token Holder with roles
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenHolder {
    address: String,
    security_tokens: f64,
    utility_tokens: f64,
    role: UserRole,
    is_main_owner: bool,
    checks: Vec<Check>,
    blockchain_accounts: HashMap<String, BlockchainAccount>,
}

impl TokenHolder {
    fn new(address: String, is_main_owner: bool) -> Self {
        TokenHolder {
            address,
            security_tokens: 0.0,
            utility_tokens: 0.0,
            role: if is_main_owner { UserRole::MainOwner } else { UserRole::Unauthorized },
            is_main_owner,
            checks: vec![],
            blockchain_accounts: HashMap::new(),
        }
    }

    fn add_security_tokens(&mut self, amount: f64) {
        self.security_tokens += amount;
        self.update_role();
    }

    fn add_utility_tokens(&mut self, amount: f64) {
        self.utility_tokens += amount;
    }

    fn update_role(&mut self) {
        if self.is_main_owner {
            self.role = UserRole::MainOwner;
        } else {
            // Role will be calculated based on percentage of total supply
            // This will be updated by the blockchain
        }
    }

    fn add_check(&mut self, check: Check) {
        self.checks.push(check.clone());
        // Add blockchain account to holder's accounts
        let account = BlockchainAccount::new(check.blockchain_account.clone());
        self.blockchain_accounts.insert(check.blockchain_account.clone(), account);
    }

    fn activate_account(&mut self, check_id: &str, activation_code: &str, personal_data: PersonalData) -> Result<(), String> {
        // Find the check
        if let Some(check) = self.checks.iter_mut().find(|c| c.check_id == check_id) {
            if check.activation_code != activation_code {
                return Err("Invalid activation code".to_string());
            }
            if check.is_activated {
                return Err("Check already activated".to_string());
            }
            
            check.is_activated = true;
            
            // Activate the blockchain account
            if let Some(account) = self.blockchain_accounts.get_mut(&check.blockchain_account) {
                account.activate(personal_data)?;
            }
            
            Ok(())
        } else {
            Err("Check not found".to_string())
        }
    }
}

// Ингредиент с количеством и калориями
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ingredient {
    name: String,
    amount_grams: f64,
    calories: f64,
}

// Позиция меню с полной информацией
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MenuItem {
    id: String,
    name: String,
    description: String,
    price: f64,
    availability: u32, // количество доступных штук
    priority_rank: u32, // приоритетность (1-10, где 10 - высший)
    cooking_time_minutes: u32, // время готовки в минутах
    ingredients: Vec<Ingredient>,
    total_calories: f64,
    suggested_by: String,
    votes_for: f64,
    votes_against: f64,
    status: MenuItemStatus,
    created_timestamp: u64,
    voting_ends: u64,
    is_available_for_voting: bool, // доступно ли для голосования
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum MenuItemStatus {
    Proposed,
    Voting,
    Approved,
    Rejected,
    Active,
}

// Заказ блюда
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    menu_item_id: String,
    quantity: u32,
}

// Статус заказа
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum OrderStatus {
    Pending,      // на рассмотрении
    Confirmed,    // подтвержден
    Cancelled,    // отменен
    Completed,    // выполнен
}

// Заказ
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: String,
    customer_wallet: String,
    items: Vec<OrderItem>,
    total_amount: f64,
    delivery_time_minutes: u32, // когда может приехать курьер
    status: OrderStatus,
    created_timestamp: u64,
    confirmed_timestamp: Option<u64>,
    cancellation_reason: Option<String>,
    tokens_issued: f64, // количество токенов, выданных за заказ
}

#[cfg_attr(test, allow(dead_code))]
impl MenuItem {
    fn new(name: String, description: String, price: f64, suggested_by: String, voting_duration_days: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        MenuItem {
            id: Self::generate_id(&name, &suggested_by, timestamp),
            name,
            description,
            price,
            availability: 0,
            priority_rank: 5, // средний приоритет по умолчанию
            cooking_time_minutes: 15, // 15 минут по умолчанию
            ingredients: vec![],
            total_calories: 0.0,
            suggested_by,
            votes_for: 0.0,
            votes_against: 0.0,
            status: MenuItemStatus::Proposed,
            created_timestamp: timestamp,
            voting_ends: timestamp + (voting_duration_days * 24 * 60 * 60),
            is_available_for_voting: false,
        }
    }

    fn new_with_details(
        name: String, 
        description: String, 
        price: f64,
        availability: u32,
        priority_rank: u32,
        cooking_time_minutes: u32,
        ingredients: Vec<Ingredient>,
        suggested_by: String, 
        voting_duration_days: u64
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let total_calories: f64 = ingredients.iter().map(|i| i.calories).sum();
        
        MenuItem {
            id: Self::generate_id(&name, &suggested_by, timestamp),
            name,
            description,
            price,
            availability,
            priority_rank,
            cooking_time_minutes,
            ingredients,
            total_calories,
            suggested_by,
            votes_for: 0.0,
            votes_against: 0.0,
            status: MenuItemStatus::Proposed,
            created_timestamp: timestamp,
            voting_ends: timestamp + (voting_duration_days * 24 * 60 * 60),
            is_available_for_voting: false,
        }
    }

    fn generate_id(name: &str, suggested_by: &str, timestamp: u64) -> String {
        let data = format!("{}{}{}", name, suggested_by, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(&hasher.finalize()[..8])
    }

    fn start_voting(&mut self) {
        self.status = MenuItemStatus::Voting;
    }

    fn vote(&mut self, utility_tokens: f64, vote_for: bool) -> Result<(), String> {
        if self.status != MenuItemStatus::Voting {
            return Err("Voting is not active for this item".to_string());
        }
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if current_time > self.voting_ends {
            return Err("Voting period has ended".to_string());
        }
        
        if vote_for {
            self.votes_for += utility_tokens;
        } else {
            self.votes_against += utility_tokens;
        }
        Ok(())
    }

    fn finalize_vote(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if current_time > self.voting_ends {
            if self.votes_for > self.votes_against {
                self.status = MenuItemStatus::Approved;
            } else {
                self.status = MenuItemStatus::Rejected;
            }
        }
    }

    fn make_available_for_voting(&mut self) {
        self.is_available_for_voting = true;
        self.status = MenuItemStatus::Voting;
    }
}

impl Order {
    fn new(customer_wallet: String, items: Vec<OrderItem>, delivery_time_minutes: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let total_amount: f64 = items.iter().map(|item| {
            // Здесь нужно будет получить цену из меню
            item.quantity as f64 * 10.0 // временная заглушка
        }).sum();
        
        Order {
            id: Self::generate_order_id(&customer_wallet, timestamp),
            customer_wallet,
            items,
            total_amount,
            delivery_time_minutes,
            status: OrderStatus::Pending,
            created_timestamp: timestamp,
            confirmed_timestamp: None,
            cancellation_reason: None,
            tokens_issued: 0.0,
        }
    }

    fn generate_order_id(customer_wallet: &str, timestamp: u64) -> String {
        let data = format!("{}{}", customer_wallet, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("ORDER_{}", hex::encode(&hasher.finalize()[..8]))
    }

    fn confirm(&mut self, tokens_issued: f64) {
        self.status = OrderStatus::Confirmed;
        self.confirmed_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        self.tokens_issued = tokens_issued;
    }

    fn cancel(&mut self, reason: String) {
        self.status = OrderStatus::Cancelled;
        self.cancellation_reason = Some(reason);
    }
}

// Transaction with check generation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount: f64,
    food_items: Vec<String>,
    timestamp: u64,
    transaction_id: String,
    check: Option<Check>,
    security_tokens_issued: f64,
    utility_tokens_issued: f64,
}

impl Transaction {
    fn new(from: String, to: String, amount: f64, food_items: Vec<String>, 
           security_tokens: f64, utility_tokens: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let transaction_id = Self::generate_transaction_id(&from, &to, amount, &food_items, timestamp);
        
        // Generate check for the transaction
        let check = Check::new(amount, food_items.clone());
        
        Transaction {
            from,
            to,
            amount,
            food_items,
            timestamp,
            transaction_id,
            check: Some(check),
            security_tokens_issued: security_tokens,
            utility_tokens_issued: utility_tokens,
        }
    }

    fn generate_transaction_id(from: &str, to: &str, amount: f64, food_items: &[String], timestamp: u64) -> String {
        let data = format!("{}{}{}{}{}", from, to, amount, food_items.join(""), timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}

// Enhanced Blockchain
#[derive(Clone)]
struct Blockchain {
    chain: Vec<Block>,
    token_holders: HashMap<String, TokenHolder>,
    pending_transactions: Vec<Transaction>,
    utility_token: UtilityToken,
    menu_items: Vec<MenuItem>,
    orders: Vec<Order>,
    #[allow(dead_code)]
    smart_contracts: Vec<SmartContract>,
    voting_history: Vec<VotingRecord>,
    blockchain_history: Vec<BlockchainOrderRecord>,
    main_owner: String,
    difficulty: usize,
    min_stake: f64,
    block_reward: f64,
}

#[cfg_attr(test, allow(dead_code))]
impl Blockchain {
    fn new(main_owner: String) -> Self {
        let genesis_block = Block::new(
            0,
            vec![],
            "0".to_string(),
            "Genesis".to_string(),
            0.0,
        );
        
        let utility_token = UtilityToken::new("VOTE".to_string());
        
        let mut token_holders = HashMap::new();
        token_holders.insert(main_owner.clone(), TokenHolder::new(main_owner.clone(), true));
        
        Blockchain {
            chain: vec![genesis_block],
            token_holders,
            pending_transactions: vec![],
            utility_token,
            menu_items: vec![],
            orders: vec![],
            smart_contracts: vec![],
            voting_history: vec![],
            blockchain_history: vec![],
            main_owner,
            difficulty: 4,
            min_stake: 10.0,
            block_reward: 5.0,
        }
    }

    fn process_purchase(&mut self, customer: String, food_truck: String, amount: f64, food_items: Vec<String>) -> Check {
        // In sleep status, 100% of security tokens go to main owner
        let security_tokens = amount; // 1:1 ratio
        let utility_tokens = amount * 0.1; // 10% of purchase for voting
        
        // Create transaction with check
        let transaction = Transaction::new(
            customer.clone(),
            food_truck,
            amount,
            food_items.clone(),
            security_tokens,
            utility_tokens,
        );
        
        let check = transaction.check.as_ref().unwrap().clone();
        
        // Add security tokens to main owner (100% in sleep status)
        if let Some(owner_holder) = self.token_holders.get_mut(&self.main_owner) {
            owner_holder.add_security_tokens(security_tokens);
            owner_holder.add_check(check.clone());
        }
        
        // Issue utility tokens for voting
        let voting_power = self.utility_token.issue_voting_tokens(utility_tokens);
        
        // Add utility tokens to main owner (until account is activated)
        if let Some(owner_holder) = self.token_holders.get_mut(&self.main_owner) {
            owner_holder.add_utility_tokens(voting_power);
        }
        
        self.add_transaction(transaction);
        check
    }

    fn activate_account(&mut self, check_id: &str, activation_code: &str, personal_data: PersonalData) -> Result<(), String> {
        // Find the holder who has this check
        for holder in self.token_holders.values_mut() {
            if let Ok(()) = holder.activate_account(check_id, activation_code, personal_data.clone()) {
                // In a real system tokens would be transferred here. For now, success is enough.
                    return Ok(());
            }
        }
        Err("Check not found or invalid activation code".to_string())
    }

    fn suggest_menu_item(&mut self, name: String, description: String, price: f64, suggested_by: String) -> Result<(), String> {
        // Only main owner and big stacks can suggest menu items
        if let Some(holder) = self.token_holders.get(&suggested_by) {
            if holder.role != UserRole::MainOwner && holder.role != UserRole::BigStack {
                return Err("Only main owner and big stacks can suggest menu items".to_string());
            }
        } else {
            return Err("Invalid suggester address".to_string());
        }
        
        let menu_item = MenuItem::new(name, description, price, suggested_by, 7); // 7 days voting
        self.menu_items.push(menu_item);
        Ok(())
    }

    fn add_menu_item_with_details(&mut self, name: String, description: String, price: f64, 
                                 availability: u32, priority_rank: u32, cooking_time_minutes: u32,
                                 ingredients: Vec<Ingredient>, suggested_by: String) -> Result<(), String> {
        // Only main owner can add detailed menu items
        if let Some(holder) = self.token_holders.get(&suggested_by) {
            if holder.role != UserRole::MainOwner {
                return Err("Only main owner can add detailed menu items".to_string());
            }
        } else {
            return Err("Invalid suggester address".to_string());
        }
        
        let menu_item = MenuItem::new_with_details(
            name, description, price, availability, priority_rank, 
            cooking_time_minutes, ingredients, suggested_by, 7
        );
        self.menu_items.push(menu_item);
        Ok(())
    }

    fn make_menu_item_available_for_voting(&mut self, menu_item_id: String) -> Result<(), String> {
        if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == menu_item_id) {
            menu_item.make_available_for_voting();
            Ok(())
        } else {
            Err("Menu item not found".to_string())
        }
    }

    fn create_order(&mut self, customer_wallet: String, items: Vec<OrderItem>, delivery_time_minutes: u32) -> Result<Order, String> {
        // Проверяем доступность товаров
        for order_item in &items {
            if let Some(menu_item) = self.menu_items.iter().find(|item| item.id == order_item.menu_item_id) {
                if menu_item.availability < order_item.quantity {
                    return Err(format!("Not enough {} available. Requested: {}, Available: {}", 
                        menu_item.name, order_item.quantity, menu_item.availability));
                }
            } else {
                return Err(format!("Menu item {} not found", order_item.menu_item_id));
            }
        }

        let mut order = Order::new(customer_wallet, items, delivery_time_minutes);
        
        // Рассчитываем правильную сумму заказа
        let mut total_amount = 0.0;
        for order_item in &order.items {
            if let Some(menu_item) = self.menu_items.iter().find(|item| item.id == order_item.menu_item_id) {
                total_amount += menu_item.price * order_item.quantity as f64;
            }
        }
        order.total_amount = total_amount;
        
        self.orders.push(order.clone());
        Ok(order)
    }

    fn confirm_order(&mut self, order_id: String) -> Result<(), String> {
        let idx = self.orders.iter().position(|o| o.id == order_id).ok_or("Order not found".to_string())?;
        let (security_tokens, utility_tokens, customer_wallet, items_clone);
        {
            let order = &self.orders[idx];
            if order.status != OrderStatus::Pending {
                return Err("Order is not pending".to_string());
            }
            security_tokens = order.total_amount;
            utility_tokens = order.total_amount * 0.1;
            customer_wallet = order.customer_wallet.clone();
            items_clone = order.items.clone();
        }

        // update balances
        if let Some(holder) = self.token_holders.get_mut(&customer_wallet) {
                holder.add_security_tokens(security_tokens);
                holder.add_utility_tokens(utility_tokens);
            } else {
            let mut new_holder = TokenHolder::new(customer_wallet.clone(), false);
                new_holder.add_security_tokens(security_tokens);
                new_holder.add_utility_tokens(utility_tokens);
            self.token_holders.insert(customer_wallet.clone(), new_holder);
            }

        // update availability
        for order_item in &items_clone {
                if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == order_item.menu_item_id) {
                    menu_item.availability -= order_item.quantity;
                }
            }

        let total_tokens = security_tokens + utility_tokens;
        let (order_id_clone, order_created_ts, customer_wallet_clone, total_amount_clone, tokens_issued_clone, status_clone);
        {
            let order_mut = &mut self.orders[idx];
            order_mut.confirm(total_tokens);
            order_id_clone = order_mut.id.clone();
            order_created_ts = order_mut.created_timestamp;
            customer_wallet_clone = order_mut.customer_wallet.clone();
            total_amount_clone = order_mut.total_amount;
            tokens_issued_clone = order_mut.tokens_issued;
            status_clone = match order_mut.status {
                OrderStatus::Pending => "Pending".to_string(),
                OrderStatus::Confirmed => "Confirmed".to_string(),
                OrderStatus::Cancelled => "Cancelled".to_string(),
                OrderStatus::Completed => "Completed".to_string(),
            };
        }
        self.blockchain_history.push(BlockchainOrderRecord {
            order_id: order_id_clone,
            customer_wallet: customer_wallet_clone,
            total_amount: total_amount_clone,
            tokens_issued: tokens_issued_clone,
            timestamp: order_created_ts,
            status: status_clone,
        });
            Ok(())
    }

    fn cancel_order(&mut self, order_id: String, reason: String) -> Result<(), String> {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return Err("Order is not pending".to_string());
            }
            order.cancel(reason);
            Ok(())
        } else {
            Err("Order not found".to_string())
        }
    }

    // Децентрализованные смарт-контракты
    fn create_purchase_contract(&mut self, customer: String, amount: f64) -> Result<String, String> {
        let conditions = ContractConditions {
            min_tokens_required: 0.0,
            expiration_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 3600 // 1 час
            ),
            required_participants: vec![customer.clone(), self.main_owner.clone()],
            auto_execute: true,
        };

        let contract = SmartContract::new(
            ContractType::PurchaseContract,
            customer.clone(),
            conditions
        );

        let contract_id = contract.contract_id.clone();
        self.smart_contracts.push(contract);
        Ok(contract_id)
    }

    fn create_voting_contract(&mut self, voter: String, menu_item_id: String) -> Result<String, String> {
        let conditions = ContractConditions {
            min_tokens_required: 1.0, // Минимум 1 utility токен
            expiration_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 86400 // 24 часа
            ),
            required_participants: vec![voter.clone()],
            auto_execute: false,
        };

        let contract = SmartContract::new(
            ContractType::VotingContract,
            voter.clone(),
            conditions
        );

        let contract_id = contract.contract_id.clone();
        self.smart_contracts.push(contract);
        Ok(contract_id)
    }

    fn execute_voting_contract(&mut self, contract_id: String, voter: String, vote_for: bool) -> Result<(), String> {
        if let Some(contract) = self.smart_contracts.iter_mut().find(|c| c.contract_id == contract_id) {
            // Проверяем баланс utility токенов
            if let Some(holder) = self.token_holders.get(&voter) {
                if holder.utility_tokens < contract.conditions.min_tokens_required {
                    return Err("Insufficient utility tokens for voting".to_string());
                }
            } else {
                return Err("Voter not found".to_string());
            }

            // Выполняем контракт
            let action = if vote_for { "vote_for" } else { "vote_against" };
            let tokens_used = contract.conditions.min_tokens_required;
            
            contract.execute(voter.clone(), action.to_string(), tokens_used)?;

            // Записываем в историю голосований
            let voting_record = VotingRecord {
                voter_wallet: voter.clone(),
                menu_item_id: "menu_item_from_contract".to_string(), // В реальной реализации это будет из контракта
                menu_item_name: "Menu Item".to_string(),
                vote_weight: tokens_used,
                vote_for,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            self.voting_history.push(voting_record);
            Ok(())
        } else {
            Err("Contract not found".to_string())
        }
    }

    // Методы для получения истории
    fn get_blockchain_history(&self, limit: Option<u32>) -> Vec<BlockchainOrderRecord> {
        let limit = limit.unwrap_or(1000);
        let mut history = self.blockchain_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history.truncate(limit as usize);
        history
    }

    fn get_voting_history(&self) -> Vec<VotingRecord> {
        let mut history = self.voting_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history
    }

    fn add_to_blockchain_history(&mut self, order: &Order) {
        let record = BlockchainOrderRecord {
            order_id: order.id.clone(),
            customer_wallet: order.customer_wallet.clone(),
            total_amount: order.total_amount,
            tokens_issued: order.tokens_issued,
            timestamp: order.created_timestamp,
            status: match order.status {
                OrderStatus::Pending => "Pending".to_string(),
                OrderStatus::Confirmed => "Confirmed".to_string(),
                OrderStatus::Cancelled => "Cancelled".to_string(),
                OrderStatus::Completed => "Completed".to_string(),
            },
        };
        self.blockchain_history.push(record);
    }

    fn vote_on_menu_item(&mut self, voter: String, menu_item_id: String, vote_for: bool) -> Result<(), String> {
        let voting_power = if let Some(holder) = self.token_holders.get(&voter) {
            holder.utility_tokens
        } else {
            return Err("Voter not found".to_string());
        };
        
        if voting_power <= 0.0 {
            return Err("No voting power available".to_string());
        }
        
        if let Some(menu_item) = self.menu_items.iter_mut().find(|item| item.id == menu_item_id) {
            menu_item.vote(voting_power, vote_for)
        } else {
            Err("Menu item not found".to_string())
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn select_validator(&self) -> Option<String> {
        if self.token_holders.is_empty() {
            return None;
        }

        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        if total_security_tokens == 0.0 {
            return None;
        }
        
        let mut rng = fastrand::Rng::new();
        let random_value = rng.f64() * total_security_tokens;
        
        let mut current_sum = 0.0;
        for (address, holder) in &self.token_holders {
            current_sum += holder.security_tokens;
            if random_value <= current_sum {
                return Some(address.clone());
            }
        }
        
        None
    }

    fn mine_block(&mut self) -> Result<(), String> {
        if self.pending_transactions.is_empty() {
            return Err("No pending transactions to mine".to_string());
        }

        let validator_address = self.select_validator()
            .ok_or("No validators available")?;
        
        let validator = self.token_holders.get(&validator_address)
            .ok_or("Validator not found")?;
        
        if validator.security_tokens < self.min_stake {
            return Err("Validator security tokens too low".to_string());
        }

        let transactions = self.pending_transactions.drain(..).collect();
        let prev_hash = self.chain.last().unwrap().hash.clone();
        
        let mut new_block = Block::new(
            self.chain.len() as u32,
            transactions,
            prev_hash,
            validator_address.clone(),
            validator.security_tokens,
        );

        // Add block reward before mining so hash includes it
        let reward_transaction = Transaction::new(
            "Blockchain".to_string(),
            validator_address.clone(),
            self.block_reward,
            vec!["Block Reward".to_string()],
            0.0,
            0.0,
        );
        new_block.transactions.push(reward_transaction);

        new_block.mine(self.difficulty);

        // Update validator rewards
        if let Some(validator) = self.token_holders.get_mut(&validator_address) {
            validator.add_security_tokens(self.block_reward);
        }

        self.chain.push(new_block);
        Ok(())
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.hash != current.calculate_hash() {
                return false;
            }
            
            if current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }

    fn update_roles(&mut self) {
        let total_security_tokens: f64 = self.token_holders.values().map(|v| v.security_tokens).sum();
        
        for holder in self.token_holders.values_mut() {
            if !holder.is_main_owner {
                let percentage = (holder.security_tokens / total_security_tokens) * 100.0;
                holder.role = UserRole::from_percentage(percentage);
            }
        }
    }
}

// Block structure (simplified for this example)
#[derive(Debug, Clone)]
struct Block {
    index: u32,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    timestamp: u64,
    validator: String,
    stake_used: f64,
    nonce: u64,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, prev_hash: String, validator: String, stake_used: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            index,
            transactions,
            prev_hash,
            hash: String::new(),
            timestamp,
            validator,
            stake_used,
            nonce: 0,
        };
        
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let tx_concat: String = self.transactions.iter().map(|t| t.transaction_id.as_str()).collect();
        let data = format!(
            "{}{}{}{}{}{}{}",
            self.index,
            tx_concat,
            self.prev_hash,
            self.timestamp,
            self.validator,
            self.stake_used,
            self.nonce
        );
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

// API Request/Response structures
#[derive(Debug, Serialize, Deserialize)]
enum ApiRequest {
    GetMenu,
    GetMenuItem { id: String },
    CreateOrder { 
        customer_wallet: String, 
        items: Vec<OrderItem>, 
        delivery_time_minutes: u32 
    },
    CancelOrder { 
        order_id: String, 
        reason: String, 
        customer_wallet: String 
    },
    GetOrderStatus { order_id: String },
    GetWalletBalance { wallet: String },
    GetBlockchainHistory { limit: Option<u32> },
    GetVotingHistory,
    VoteOnMenuItem { 
        voter_wallet: String, 
        menu_item_id: String, 
        vote_for: bool 
    },
    AddMenuItem {
        name: String,
        description: String,
        price: f64,
        availability: u32,
        priority_rank: u32,
        cooking_time_minutes: u32,
        ingredients: Vec<Ingredient>,
        suggested_by: String,
    },
    MakeItemAvailableForVoting { menu_item_id: String },
    ConfirmOrder { order_id: String },
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    Menu { items: Vec<MenuItem> },
    MenuItem { item: MenuItem },
    OrderCreated { order: Order },
    OrderCancelled { success: bool },
    OrderStatus { order: Order },
    WalletBalance { 
        wallet: String, 
        security_tokens: f64, 
        utility_tokens: f64 
    },
    BlockchainHistory { 
        orders: Vec<BlockchainOrderRecord> 
    },
    VotingHistory { 
        votes: Vec<VotingRecord> 
    },
    VoteResult { success: bool },
    MenuItemAdded { success: bool },
    ItemAvailableForVoting { success: bool },
    OrderConfirmed { success: bool },
    Error { message: String },
}

// Структуры для истории
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockchainOrderRecord {
    order_id: String,
    customer_wallet: String,
    total_amount: f64,
    tokens_issued: f64,
    timestamp: u64,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VotingRecord {
    voter_wallet: String,
    menu_item_id: String,
    menu_item_name: String,
    vote_weight: f64,
    vote_for: bool,
    timestamp: u64,
}

// Децентрализованные смарт-контракты
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SmartContract {
    contract_id: String,
    contract_type: ContractType,
    creator: String,
    created_timestamp: u64,
    status: ContractStatus,
    participants: Vec<String>,
    conditions: ContractConditions,
    execution_history: Vec<ContractExecution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ContractType {
    PurchaseContract,
    VotingContract,
    OrderContract,
    TokenContract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ContractStatus {
    Active,
    Executed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContractConditions {
    min_tokens_required: f64,
    expiration_time: Option<u64>,
    required_participants: Vec<String>,
    auto_execute: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContractExecution {
    executor: String,
    action: String,
    timestamp: u64,
    result: String,
    tokens_used: f64,
}

#[cfg_attr(test, allow(dead_code))]
impl SmartContract {
    fn new(contract_type: ContractType, creator: String, conditions: ContractConditions) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let contract_id = Self::generate_contract_id(&creator, timestamp);
        
        SmartContract {
            contract_id,
            contract_type,
            creator: creator.clone(),
            created_timestamp: timestamp,
            status: ContractStatus::Active,
            participants: vec![creator],
            conditions,
            execution_history: vec![],
        }
    }

    fn generate_contract_id(creator: &str, timestamp: u64) -> String {
        let data = format!("{}{}", creator, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("CONTRACT_{}", hex::encode(&hasher.finalize()[..8]))
    }

    fn execute(&mut self, executor: String, action: String, tokens_used: f64) -> Result<String, String> {
        if self.status != ContractStatus::Active {
            return Err("Contract is not active".to_string());
        }

        // Проверяем условия выполнения
        if !self.participants.contains(&executor) {
            return Err("Executor is not a participant".to_string());
        }

        let result = format!("Action '{}' executed by {}", action, executor);
        
        let execution = ContractExecution {
            executor: executor.clone(),
            action,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            result: result.clone(),
            tokens_used,
        };

        self.execution_history.push(execution);
        
        // Если контракт настроен на автоматическое выполнение
        if self.conditions.auto_execute {
            self.status = ContractStatus::Executed;
        }

        Ok(result)
    }
}

// API Server
struct ApiServer {
    blockchain: Arc<Mutex<Blockchain>>,
    port: u16,
}

impl ApiServer {
    fn new(blockchain: Arc<Mutex<Blockchain>>, port: u16) -> Self {
        ApiServer { blockchain, port }
    }

    fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect("Failed to bind to address");
        
        println!("🌐 API Server started on port {}", self.port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let blockchain_clone = Arc::clone(&self.blockchain);
                    thread::spawn(move || {
                        Self::handle_client(stream, blockchain_clone);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 8192];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let req_raw = String::from_utf8_lossy(&buffer[..size]).to_string();

                // Very basic HTTP parsing
                let headers_end = req_raw.find("\r\n\r\n").unwrap_or(req_raw.len());
                let (head, body_part) = req_raw.split_at(headers_end);
                let body = if body_part.starts_with("\r\n\r\n") { &body_part[4..] } else { body_part };

                let method = if let Some(space) = head.find(' ') { &head[..space] } else { "" };

                // CORS preflight
                if method == "OPTIONS" {
                    let resp = "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                    let _ = stream.write_all(resp.as_bytes());
                    return;
                }

                // Expect POST with JSON body containing our ApiRequest enum
                let api_result = match serde_json::from_str::<ApiRequest>(body) {
                    Ok(req) => Self::process_request(req, blockchain),
                    Err(_) => {
                        // Fallback compatibility: accept {"Variant": {...}} and unit variants as {"Variant": {}}
                        match serde_json::from_str::<serde_json::Value>(body) {
                            Ok(val) => {
                                if let Some(obj) = val.as_object() {
                                    if obj.contains_key("GetMenu") {
                                        Self::process_request(ApiRequest::GetMenu, blockchain)
                                    } else if let Some(params) = obj.get("GetBlockchainHistory") {
                                        let limit = params.get("limit").and_then(|v| v.as_u64()).map(|v| v as u32);
                                        Self::process_request(ApiRequest::GetBlockchainHistory { limit }, blockchain)
                                    } else if obj.contains_key("GetVotingHistory") {
                                        Self::process_request(ApiRequest::GetVotingHistory, blockchain)
                                    } else if let Some(params) = obj.get("MakeItemAvailableForVoting") {
                                        if let Some(menu_item_id) = params.get("menu_item_id").and_then(|v| v.as_str()) {
                                            Self::process_request(ApiRequest::MakeItemAvailableForVoting { menu_item_id: menu_item_id.to_string() }, blockchain)
                                        } else {
                                            ApiResponse::Error { message: "Invalid MakeItemAvailableForVoting payload".to_string() }
                                        }
                                    } else if let Some(params) = obj.get("ConfirmOrder") {
                                        if let Some(order_id) = params.get("order_id").and_then(|v| v.as_str()) {
                                            Self::process_request(ApiRequest::ConfirmOrder { order_id: order_id.to_string() }, blockchain)
                                        } else {
                                            ApiResponse::Error { message: "Invalid ConfirmOrder payload".to_string() }
                                        }
                                    } else if let Some(params) = obj.get("CancelOrder") {
                                        let order_id = params.get("order_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let reason = params.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let customer_wallet = params.get("customer_wallet").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        if order_id.is_empty() || customer_wallet.is_empty() {
                                            ApiResponse::Error { message: "Invalid CancelOrder payload".to_string() }
                                        } else {
                                            Self::process_request(ApiRequest::CancelOrder { order_id, reason, customer_wallet }, blockchain)
                                        }
                                    } else if let Some(params) = obj.get("AddMenuItem") {
                                        // Map incoming ingredient fields amount -> amount_grams
                                        let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let description = params.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let price = params.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let availability = params.get("availability").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let priority_rank = params.get("priority_rank").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let cooking_time_minutes = params.get("cooking_time_minutes").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                                        let suggested_by = params.get("suggested_by").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let ingredients = params.get("ingredients").and_then(|v| v.as_array()).map(|arr| {
                                            arr.iter().filter_map(|ing| {
                                                let name = ing.get("name").and_then(|v| v.as_str())?.to_string();
                                                let amount = ing.get("amount").or_else(|| ing.get("amount_grams")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                let calories = ing.get("calories").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                                Some(Ingredient { name, amount_grams: amount, calories })
                                            }).collect::<Vec<_>>()
                                        }).unwrap_or_default();

                                        Self::process_request(
                                            ApiRequest::AddMenuItem {
                                                name, description, price, availability, priority_rank, cooking_time_minutes, ingredients, suggested_by
                                            },
                                            blockchain,
                                        )
                                    } else if let Some(params) = obj.get("VoteOnMenuItem") {
                                        let voter_wallet = params.get("voter_wallet").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let menu_item_id = params.get("menu_item_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        let vote_for = params.get("vote_for").and_then(|v| v.as_bool()).unwrap_or(true);
                                        if voter_wallet.is_empty() || menu_item_id.is_empty() {
                                            ApiResponse::Error { message: "Invalid VoteOnMenuItem payload".to_string() }
                                        } else {
                                            Self::process_request(ApiRequest::VoteOnMenuItem { voter_wallet, menu_item_id, vote_for }, blockchain)
                                        }
                                    } else {
                                        ApiResponse::Error { message: "Unknown API request".to_string() }
                                    }
                                } else {
                                    ApiResponse::Error { message: "Invalid request payload".to_string() }
                                }
                            }
                            Err(e) => ApiResponse::Error { message: format!("Invalid request format: {}", e) },
                        }
                    }
                };

                let response_json = serde_json::to_string(&api_result)
                    .unwrap_or_else(|_| r#"{"Error": {"message": "Serialization error"}}"#.to_string());

                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\n\r\n{}",
                    response_json.len(),
                    response_json
                );
                let _ = stream.write_all(resp.as_bytes());
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
            }
        }
    }

    fn process_request(request: ApiRequest, blockchain: Arc<Mutex<Blockchain>>) -> ApiResponse {
        let mut blockchain_guard = blockchain.lock().unwrap();
        
        match request {
            ApiRequest::GetMenu => {
                let items = blockchain_guard.menu_items.clone();
                ApiResponse::Menu { items }
            }
            
            ApiRequest::GetMenuItem { id } => {
                if let Some(item) = blockchain_guard.menu_items.iter().find(|item| item.id == id) {
                    ApiResponse::MenuItem { item: item.clone() }
                } else {
                    ApiResponse::Error { message: "Menu item not found".to_string() }
                }
            }
            
            ApiRequest::CreateOrder { customer_wallet, items, delivery_time_minutes } => {
                match blockchain_guard.create_order(customer_wallet, items, delivery_time_minutes) {
                    Ok(order) => ApiResponse::OrderCreated { order },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::CancelOrder { order_id, reason, customer_wallet } => {
                // Проверяем, что заказ принадлежит этому кошельку
                if let Some(order) = blockchain_guard.orders.iter().find(|o| o.id == order_id) {
                    if order.customer_wallet == customer_wallet {
                        match blockchain_guard.cancel_order(order_id, reason) {
                            Ok(()) => ApiResponse::OrderCancelled { success: true },
                            Err(e) => ApiResponse::Error { message: e },
                        }
                    } else {
                        ApiResponse::Error { message: "Order does not belong to this wallet".to_string() }
                    }
                } else {
                    ApiResponse::Error { message: "Order not found".to_string() }
                }
            }
            
            ApiRequest::GetOrderStatus { order_id } => {
                if let Some(order) = blockchain_guard.orders.iter().find(|o| o.id == order_id) {
                    ApiResponse::OrderStatus { order: order.clone() }
                } else {
                    ApiResponse::Error { message: "Order not found".to_string() }
                }
            }
            
            ApiRequest::GetWalletBalance { wallet } => {
                if let Some(holder) = blockchain_guard.token_holders.get(&wallet) {
                    ApiResponse::WalletBalance {
                        wallet,
                        security_tokens: holder.security_tokens,
                        utility_tokens: holder.utility_tokens,
                    }
                } else {
                    ApiResponse::WalletBalance {
                        wallet,
                        security_tokens: 0.0,
                        utility_tokens: 0.0,
                    }
                }
            }
            
            ApiRequest::GetBlockchainHistory { limit } => {
                let orders = blockchain_guard.get_blockchain_history(limit);
                ApiResponse::BlockchainHistory { orders }
            }
            
            ApiRequest::GetVotingHistory => {
                let votes = blockchain_guard.get_voting_history();
                ApiResponse::VotingHistory { votes }
            }
            
            ApiRequest::VoteOnMenuItem { voter_wallet, menu_item_id, vote_for } => {
                match blockchain_guard.vote_on_menu_item(voter_wallet, menu_item_id, vote_for) {
                    Ok(()) => ApiResponse::VoteResult { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::AddMenuItem { name, description, price, availability, priority_rank, cooking_time_minutes, ingredients, suggested_by } => {
                match blockchain_guard.add_menu_item_with_details(
                    name, description, price, availability, priority_rank, 
                    cooking_time_minutes, ingredients, suggested_by
                ) {
                    Ok(()) => ApiResponse::MenuItemAdded { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::MakeItemAvailableForVoting { menu_item_id } => {
                match blockchain_guard.make_menu_item_available_for_voting(menu_item_id) {
                    Ok(()) => ApiResponse::ItemAvailableForVoting { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
            
            ApiRequest::ConfirmOrder { order_id } => {
                match blockchain_guard.confirm_order(order_id) {
                    Ok(()) => ApiResponse::OrderConfirmed { success: true },
                    Err(e) => ApiResponse::Error { message: e },
                }
            }
        }
    }
}

// UI System
struct UI {
    current_user: Option<String>,
    blockchain: Blockchain,
}

#[cfg_attr(test, allow(dead_code))]
impl UI {
    fn new(blockchain: Blockchain) -> Self {
        UI {
            current_user: None,
            blockchain,
        }
    }

    fn show_main_menu(&self) {
        println!("\n🍔 Fast Food Truck Blockchain UI 🍔");
        println!("=====================================");
        
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                println!("Logged in as: {}", user);
                println!("Role: {:?}", holder.role);
                println!("Security Tokens: {:.2}", holder.security_tokens);
                println!("Utility Tokens: {:.2}", holder.utility_tokens);
                println!("Checks: {}", holder.checks.len());
                println!("Blockchain Accounts: {}", holder.blockchain_accounts.len());
            }
        } else {
            println!("Not logged in");
        }
        
        println!("\nOptions:");
        println!("1. Login with QR Code");
        println!("2. Login with Check Number");
        println!("3. View Menu Items");
        println!("4. Suggest Menu Item");
        println!("5. Add Detailed Menu Item");
        println!("6. Make Item Available for Voting");
        println!("7. Vote on Menu Items");
        println!("8. View Orders");
        println!("9. Confirm Order");
        println!("10. View My Checks");
        println!("11. Activate Account");
        println!("12. List Account for Sale");
        println!("13. View Blockchain Status");
        println!("14. Start API Server");
        println!("0. Exit");
    }

    fn show_unauthorized_ui(&self) {
        println!("\n👤 Unauthorized User Interface");
        println!("===============================");
        println!("You don't have any tokens yet.");
        println!("Make a purchase to get started!");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Make Purchase (Demo)");
        println!("3. Back to Main Menu");
    }

    fn show_starter_ui(&self) {
        println!("\n🌟 Starter User Interface");
        println!("==========================");
        println!("You have 1-5% of security tokens!");
        println!("You can vote on menu items.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Vote on Menu Items");
        println!("3. View My Checks");
        println!("4. Activate Account");
        println!("5. Back to Main Menu");
    }

    fn show_middle_player_ui(&self) {
        println!("\n🎯 Middle Player Interface");
        println!("===========================");
        println!("You have 5-10% of security tokens!");
        println!("You can vote and have more influence.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Vote on Menu Items");
        println!("3. View My Checks");
        println!("4. Activate Account");
        println!("5. List Account for Sale");
        println!("6. Back to Main Menu");
    }

    fn show_big_stack_ui(&self) {
        println!("\n💎 Big Stack Interface");
        println!("=======================");
        println!("You have 10%+ of security tokens!");
        println!("You can suggest menu items and vote.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Suggest Menu Item");
        println!("3. Vote on Menu Items");
        println!("4. View My Checks");
        println!("5. Activate Account");
        println!("6. List Account for Sale");
        println!("7. Back to Main Menu");
    }

    fn show_main_owner_ui(&self) {
        println!("\n👑 Main Owner Interface");
        println!("========================");
        println!("You are the main owner!");
        println!("You control the blockchain and can suggest menu items.");
        println!("\nOptions:");
        println!("1. View Menu Items");
        println!("2. Suggest Menu Item");
        println!("3. Vote on Menu Items");
        println!("4. View All Checks");
        println!("5. View Blockchain Status");
        println!("6. Mine Block");
        println!("7. Back to Main Menu");
    }

    fn login_with_qr(&mut self) {
        println!("\n📱 Login with QR Code");
        println!("=====================");
        println!("Scan the QR code from your check:");
        
        // Simulate QR code scanning
        let qr_data = "check_id_123|123456|0xabcd1234";
        println!("QR Data: {}", qr_data);
        
        // Parse QR data
        let parts: Vec<&str> = qr_data.split('|').collect();
        if parts.len() == 3 {
            let check_id = parts[0];
            let activation_code = parts[1];
            let account = parts[2];
            
            println!("Check ID: {}", check_id);
            println!("Activation Code: {}", activation_code);
            println!("Account: {}", account);
            
            // Find the holder with this check
            for (address, holder) in &self.blockchain.token_holders {
                if holder.checks.iter().any(|c| c.check_id == check_id) {
                    self.current_user = Some(address.clone());
                    println!("✅ Login successful!");
                    return;
                }
            }
        }
        
        println!("❌ Invalid QR code or check not found");
    }

    fn login_with_check(&mut self) {
        println!("\n🧾 Login with Check Number");
        println!("==========================");
        println!("Enter your check ID:");
        
        // Simulate user input
        let check_id = "check_id_123";
        println!("Check ID: {}", check_id);
        
        // Find the holder with this check
        for (address, holder) in &self.blockchain.token_holders {
            if holder.checks.iter().any(|c| c.check_id == check_id) {
                self.current_user = Some(address.clone());
                println!("✅ Login successful!");
                return;
            }
        }
        
        println!("❌ Check not found");
    }

    fn view_menu_items(&self) {
        println!("\n🍽️ Menu Items");
        println!("=============");
        
        if self.blockchain.menu_items.is_empty() {
            println!("No menu items available.");
            return;
        }
        
        for (i, item) in self.blockchain.menu_items.iter().enumerate() {
            let status_str = match item.status {
                MenuItemStatus::Proposed => "Proposed",
                MenuItemStatus::Voting => "Voting",
                MenuItemStatus::Approved => "Approved",
                MenuItemStatus::Rejected => "Rejected",
                MenuItemStatus::Active => "Active",
            };
            
            println!("{}. {} - {}", i + 1, item.name, item.description);
            println!("   Status: {}", status_str);
            println!("   Suggested by: {}", item.suggested_by);
            println!("   Votes: For {:.2}, Against {:.2}", item.votes_for, item.votes_against);
            println!();
        }
    }

    fn suggest_menu_item(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner && holder.role != UserRole::BigStack {
                    println!("❌ Only main owners and big stacks can suggest menu items");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n💡 Suggest Menu Item");
        println!("====================");
        
        // Simulate user input
        let name = "Vegan Pizza";
        let description = "Plant-based pizza with vegan cheese";
        let price = 15.99;
        
        println!("Name: {}", name);
        println!("Description: {}", description);
        println!("Price: ${:.2}", price);
        
        if let Some(user) = &self.current_user {
            match self.blockchain.suggest_menu_item(name.to_string(), description.to_string(), price, user.clone()) {
                Ok(()) => println!("✅ Menu item suggested successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn add_detailed_menu_item(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can add detailed menu items");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🍽️ Add Detailed Menu Item");
        println!("==========================");
        
        // Simulate detailed menu item creation
        let name = "Gourmet Burger";
        let description = "Premium beef burger with special sauce";
        let price = 18.50;
        let availability = 20;
        let priority_rank = 8;
        let cooking_time_minutes = 12;
        
        let ingredients = vec![
            Ingredient { name: "Beef Patty".to_string(), amount_grams: 200.0, calories: 350.0 },
            Ingredient { name: "Bun".to_string(), amount_grams: 80.0, calories: 200.0 },
            Ingredient { name: "Cheese".to_string(), amount_grams: 30.0, calories: 120.0 },
            Ingredient { name: "Lettuce".to_string(), amount_grams: 20.0, calories: 5.0 },
            Ingredient { name: "Tomato".to_string(), amount_grams: 25.0, calories: 10.0 },
        ];
        
        println!("Name: {}", name);
        println!("Description: {}", description);
        println!("Price: ${:.2}", price);
        println!("Availability: {} pieces", availability);
        println!("Priority Rank: {}/10", priority_rank);
        println!("Cooking Time: {} minutes", cooking_time_minutes);
        println!("Ingredients: {} items", ingredients.len());
        
        if let Some(user) = &self.current_user {
            match self.blockchain.add_menu_item_with_details(
                name.to_string(), 
                description.to_string(), 
                price, 
                availability, 
                priority_rank, 
                cooking_time_minutes, 
                ingredients, 
                user.clone()
            ) {
                Ok(()) => println!("✅ Detailed menu item added successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn make_item_available_for_voting(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can make items available for voting");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🗳️ Make Item Available for Voting");
        println!("==================================");
        
        // Show items that can be made available for voting
        let proposed_items: Vec<_> = self.blockchain.menu_items.iter()
            .filter(|item| item.status == MenuItemStatus::Proposed && !item.is_available_for_voting)
            .collect();
        
        if proposed_items.is_empty() {
            println!("No items available for voting.");
            return;
        }
        
        for (i, item) in proposed_items.iter().enumerate() {
            println!("{}. {} - {}", i + 1, item.name, item.description);
        }
        
        // Simulate selecting first item
        if let Some(item) = proposed_items.first() {
            println!("Making {} available for voting...", item.name);
            match self.blockchain.make_menu_item_available_for_voting(item.id.clone()) {
                Ok(()) => println!("✅ Item is now available for voting!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn view_orders(&self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can view all orders");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n📋 All Orders");
        println!("=============");
        
        if self.blockchain.orders.is_empty() {
            println!("No orders found.");
            return;
        }
        
        for (i, order) in self.blockchain.orders.iter().enumerate() {
            let status_str = match order.status {
                OrderStatus::Pending => "Pending",
                OrderStatus::Confirmed => "Confirmed",
                OrderStatus::Cancelled => "Cancelled",
                OrderStatus::Completed => "Completed",
            };
            
            println!("{}. Order ID: {}", i + 1, order.id);
            println!("   Customer: {}", order.customer_wallet);
            println!("   Amount: ${:.2}", order.total_amount);
            println!("   Status: {}", status_str);
            println!("   Delivery Time: {} minutes", order.delivery_time_minutes);
            println!("   Items: {} items", order.items.len());
            if let Some(reason) = &order.cancellation_reason {
                println!("   Cancellation Reason: {}", reason);
            }
            println!();
        }
    }

    fn confirm_order(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.role != UserRole::MainOwner {
                    println!("❌ Only main owner can confirm orders");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n✅ Confirm Order");
        println!("================");
        
        let pending_orders: Vec<_> = self.blockchain.orders.iter()
            .filter(|order| order.status == OrderStatus::Pending)
            .collect();
        
        if pending_orders.is_empty() {
            println!("No pending orders found.");
            return;
        }
        
        for (i, order) in pending_orders.iter().enumerate() {
            println!("{}. Order ID: {} - Customer: {} - Amount: ${:.2}", 
                i + 1, order.id, order.customer_wallet, order.total_amount);
        }
        
        // Simulate confirming first order
        if let Some(order) = pending_orders.first() {
            println!("Confirming order: {}", order.id);
            match self.blockchain.confirm_order(order.id.clone()) {
                Ok(()) => println!("✅ Order confirmed successfully! Tokens issued to customer."),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
    }

    fn vote_on_menu_items(&mut self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                if holder.utility_tokens <= 0.0 {
                    println!("❌ You don't have any voting power");
                    return;
                }
            }
        } else {
            println!("❌ Please login first");
            return;
        }
        
        println!("\n🗳️ Vote on Menu Items");
        println!("====================");
        
        let voting_items: Vec<_> = self.blockchain.menu_items.iter()
            .filter(|item| item.status == MenuItemStatus::Voting)
            .collect();
        
        if voting_items.is_empty() {
            println!("No items currently open for voting.");
            return;
        }
        
        for (i, item) in voting_items.iter().enumerate() {
            println!("{}. {} - {}", i + 1, item.name, item.description);
        }
        
        // Simulate voting on first item
        if let Some(item) = voting_items.first() {
            if let Some(user) = &self.current_user {
                match self.blockchain.vote_on_menu_item(user.clone(), item.id.clone(), true) {
                    Ok(()) => println!("✅ Vote cast successfully!"),
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
        }
    }

    fn view_my_checks(&self) {
        if let Some(user) = &self.current_user {
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                println!("\n🧾 My Checks");
                println!("============");
                
                if holder.checks.is_empty() {
                    println!("No checks found.");
                    return;
                }
                
                for (i, check) in holder.checks.iter().enumerate() {
                    println!("{}. Check ID: {}", i + 1, check.check_id);
                    println!("   Amount: ${:.2}", check.amount);
                    println!("   Food Items: {}", check.food_items.join(", "));
                    println!("   Activation Code: {}", check.activation_code);
                    println!("   Status: {}", if check.is_activated { "Activated" } else { "Not Activated" });
                    println!("   QR Code: {}", check.qr_code);
                    println!();
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn activate_account(&mut self) {
        if let Some(user) = &self.current_user {
            println!("\n🔓 Activate Account");
            println!("==================");
            
            if let Some(holder) = self.blockchain.token_holders.get(user) {
                let inactive_checks: Vec<_> = holder.checks.iter()
                    .filter(|c| !c.is_activated)
                    .collect();
                
                if inactive_checks.is_empty() {
                    println!("No inactive checks found.");
                    return;
                }
                
                // Simulate activating first check
                if let Some(check) = inactive_checks.first() {
                    let personal_data = PersonalData {
                        name: "John Doe".to_string(),
                        email: "john@example.com".to_string(),
                        phone: "+1234567890".to_string(),
                        wallet_address: Some("0x1234567890abcdef".to_string()),
                    };
                    
                    println!("Activating check: {}", check.check_id);
                    println!("Personal data: {:?}", personal_data);
                    
                    let check_id = check.check_id.clone();
                    let activation_code = check.activation_code.clone();
                    match self.blockchain.activate_account(&check_id, &activation_code, personal_data) {
                        Ok(()) => println!("✅ Account activated successfully!"),
                        Err(e) => println!("❌ Error: {}", e),
                    }
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn list_account_for_sale(&mut self) {
        if let Some(user) = &self.current_user {
            println!("\n💰 List Account for Sale");
            println!("=======================");
            
            if let Some(holder) = self.blockchain.token_holders.get_mut(user) {
                let active_addresses: Vec<String> = holder.blockchain_accounts
                    .values()
                    .filter(|acc| acc.status == AccountStatus::Active)
                    .map(|acc| acc.address.clone())
                    .collect();
                
                if active_addresses.is_empty() {
                    println!("No active accounts to list for sale.");
                    return;
                }
                
                // Simulate listing first account
                let addr = &active_addresses[0];
                println!("Listing account: {}", addr);
                    
                if let Some(acc) = holder.blockchain_accounts.get_mut(addr) {
                        match acc.list_for_sale() {
                            Ok(()) => println!("✅ Account listed for sale!"),
                            Err(e) => println!("❌ Error: {}", e),
                    }
                }
            }
        } else {
            println!("❌ Please login first");
        }
    }

    fn view_blockchain_status(&self) {
        println!("\n🔗 Blockchain Status");
        println!("===================");
        println!("Chain valid: {}", self.blockchain.is_chain_valid());
        println!("Total blocks: {}", self.blockchain.chain.len());
        println!("Total security tokens: {:.2}", 
            self.blockchain.token_holders.values().map(|h| h.security_tokens).sum::<f64>());
        println!("Total utility tokens: {:.2}", self.blockchain.utility_token.total_supply);
        println!("Main owner: {}", self.blockchain.main_owner);
        
        println!("\nToken Holders:");
        for (address, holder) in &self.blockchain.token_holders {
            println!("  {}: {:?} - Security: {:.2}, Utility: {:.2}", 
                address, holder.role, holder.security_tokens, holder.utility_tokens);
        }
    }

    fn mine_block(&mut self) {
        println!("\n⛏️ Mining Block");
        println!("==============");
        
        match self.blockchain.mine_block() {
            Ok(()) => println!("✅ Block mined successfully!"),
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    fn run(&mut self) {
        loop {
            self.show_main_menu();
            
            // Simulate user choice
            let choice = 1; // This would normally be user input
            
            match choice {
                1 => self.login_with_qr(),
                2 => self.login_with_check(),
                3 => self.view_menu_items(),
                4 => self.suggest_menu_item(),
                5 => self.add_detailed_menu_item(),
                6 => self.make_item_available_for_voting(),
                7 => self.vote_on_menu_items(),
                8 => self.view_orders(),
                9 => self.confirm_order(),
                10 => self.view_my_checks(),
                11 => self.activate_account(),
                12 => self.list_account_for_sale(),
                13 => self.view_blockchain_status(),
                14 => {
                    println!("🌐 Starting API Server...");
                    let blockchain_arc = Arc::new(Mutex::new(self.blockchain.clone()));
                    let api_server = SimpleServer::new(blockchain_arc, 3000);
                    api_server.start();
                },
                0 => {
                    println!("👋 Goodbye!");
                    break;
                }
                _ => println!("❌ Invalid choice"),
            }
            
            // Show role-specific UI
            if let Some(user) = &self.current_user {
                if let Some(holder) = self.blockchain.token_holders.get(user) {
                    match holder.role {
                        UserRole::Unauthorized => self.show_unauthorized_ui(),
                        UserRole::Starter => self.show_starter_ui(),
                        UserRole::MiddlePlayer => self.show_middle_player_ui(),
                        UserRole::BigStack => self.show_big_stack_ui(),
                        UserRole::MainOwner => self.show_main_owner_ui(),
                    }
                }
            }
        }
    }
}

fn main() {
    println!("🍔 Fast Food Truck Blockchain with Security Tokens & Voting 🍔\n");
    
    // Initialize blockchain with main owner
    let main_owner = "MainOwner_Alice".to_string();
    let mut blockchain = Blockchain::new(main_owner.clone());
    
    // Добавляем примеры меню с полной информацией
    println!("Adding sample menu items...");
    
    let burger_ingredients = vec![
        Ingredient { name: "Beef Patty".to_string(), amount_grams: 200.0, calories: 350.0 },
        Ingredient { name: "Bun".to_string(), amount_grams: 80.0, calories: 200.0 },
        Ingredient { name: "Cheese".to_string(), amount_grams: 30.0, calories: 120.0 },
        Ingredient { name: "Lettuce".to_string(), amount_grams: 20.0, calories: 5.0 },
        Ingredient { name: "Tomato".to_string(), amount_grams: 25.0, calories: 10.0 },
    ];
    
    let pizza_ingredients = vec![
        Ingredient { name: "Pizza Dough".to_string(), amount_grams: 150.0, calories: 300.0 },
        Ingredient { name: "Tomato Sauce".to_string(), amount_grams: 50.0, calories: 25.0 },
        Ingredient { name: "Mozzarella".to_string(), amount_grams: 80.0, calories: 200.0 },
        Ingredient { name: "Pepperoni".to_string(), amount_grams: 40.0, calories: 150.0 },
    ];
    
    let _ = blockchain.add_menu_item_with_details(
        "Classic Burger".to_string(),
        "Traditional beef burger with fresh ingredients".to_string(),
        12.99,
        15,
        8,
        10,
        burger_ingredients,
        main_owner.clone()
    );
    
    let _ = blockchain.add_menu_item_with_details(
        "Pepperoni Pizza".to_string(),
        "Classic pepperoni pizza with mozzarella cheese".to_string(),
        16.99,
        8,
        9,
        15,
        pizza_ingredients,
        main_owner.clone()
    );
    
    // Simulate some purchases to generate checks
    println!("Processing food purchases and generating checks...");
    let purchases = vec![
        ("Customer_John".to_string(), "Truck_Alice".to_string(), 12.50, vec!["Burger Combo".to_string()]),
        ("Customer_Sarah".to_string(), "Truck_Bob".to_string(), 8.75, vec!["Taco Plate".to_string()]),
        ("Customer_Mike".to_string(), "Truck_Charlie".to_string(), 15.00, vec!["Pizza Slice".to_string()]),
        ("Customer_Lisa".to_string(), "Truck_Alice".to_string(), 6.25, vec!["Hot Dog".to_string()]),
    ];
    
    for (customer, truck, amount, food_items) in purchases {
        let check = blockchain.process_purchase(customer, truck, amount, food_items);
        println!("Generated check: {} for ${:.2}", check.check_id, check.amount);
    }
    
    // Создаем пример заказа
    println!("Creating sample order...");
    let order_items = vec![
        OrderItem { menu_item_id: blockchain.menu_items[0].id.clone(), quantity: 2 },
        OrderItem { menu_item_id: blockchain.menu_items[1].id.clone(), quantity: 1 },
    ];
    
    match blockchain.create_order("Customer_John".to_string(), order_items, 30) {
        Ok(order) => println!("Created order: {} for ${:.2}", order.id, order.total_amount),
        Err(e) => println!("Failed to create order: {}", e),
    }
    
    // Mine some blocks
    println!("\nMining blocks...");
    for i in 0..2 {
        match blockchain.mine_block() {
            Ok(()) => println!("Block {} mined successfully!", i + 1),
            Err(e) => println!("Failed to mine block {}: {}", i + 1, e),
        }
    }
    
    // Update roles
    blockchain.update_roles();

    // Optional: start API server only (no interactive UI) when API_ONLY=1
    if env::var("API_ONLY").map(|v| v == "1").unwrap_or(false) {
        println!("🌐 Starting API Server (API_ONLY mode) on port 3000...");
        let blockchain_arc = Arc::new(Mutex::new(blockchain.clone()));
        let api_server = SimpleServer::new(blockchain_arc, 3000);
        api_server.start();
        return;
    }
    
    // Start UI
    let mut ui = UI::new(blockchain);
    ui.run();
}

#[cfg(test)]
mod tests {
    mod core;
    mod menu;
    mod orders;
    mod blockchain;
    mod api;
}
