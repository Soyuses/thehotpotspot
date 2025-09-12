use blockchain_project::chef_arm::{ChefARM, ChefARMManager, ChefOrder, OrderStatus, OrderItem};
use std::time::{SystemTime, UNIX_EPOCH};

// Импортируем основные структуры блокчейна
// В реальном проекте это были бы импорты из main.rs
// Для тестирования создаем упрощенные версии

#[derive(Debug, Clone)]
struct MockTokenHolder {
    pub address: String,
    pub security_tokens: u128,
    pub utility_tokens: u128,
}

impl MockTokenHolder {
    fn new(address: String) -> Self {
        Self {
            address,
            security_tokens: 0,
            utility_tokens: 0,
        }
    }

    fn add_security_tokens(&mut self, amount: u128) {
        self.security_tokens += amount;
    }

    fn add_utility_tokens(&mut self, amount: u128) {
        self.utility_tokens += amount;
    }
}

#[derive(Debug, Clone)]
struct MockBlockchain {
    pub token_holders: std::collections::HashMap<String, MockTokenHolder>,
    pub chef_arm_manager: ChefARMManager,
    pub main_owner: String,
    pub charity_fund: String,
}

impl MockBlockchain {
    fn new(main_owner: String) -> Self {
        let mut token_holders = std::collections::HashMap::new();
        token_holders.insert(main_owner.clone(), MockTokenHolder::new(main_owner.clone()));
        
        Self {
            token_holders,
            chef_arm_manager: ChefARMManager::new(),
            main_owner: main_owner.clone(),
            charity_fund: "charity_fund".to_string(),
        }
    }

    fn process_purchase_with_chef(&mut self, customer: String, food_truck: String, amount_subunits: u128, food_items: Vec<String>) -> String {
        // Новая логика распределения токенов с кошельком повара:
        // Нода владельца сети: 48% владелец сети, 3% фонд, 24% повар, 25% покупатель
        // Нода франчайзи: 48% владелец франшизы, 3% фонд, 24% повар, 25% покупатель
        
        let check_id = format!("check_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        // Распределение токенов (упрощенная версия)
        let main_owner_tokens = (amount_subunits * 48) / 100;
        let charity_tokens = (amount_subunits * 3) / 100;
        let chef_tokens = (amount_subunits * 24) / 100;
        let customer_tokens = (amount_subunits * 25) / 100;
        
        // Распределяем токены
        if let Some(holder) = self.token_holders.get_mut(&self.main_owner) {
            holder.add_security_tokens(main_owner_tokens);
        }
        
        if let Some(holder) = self.token_holders.get_mut(&self.charity_fund) {
            holder.add_security_tokens(charity_tokens);
        } else {
            let mut charity_holder = MockTokenHolder::new(self.charity_fund.clone());
            charity_holder.add_security_tokens(charity_tokens);
            self.token_holders.insert(self.charity_fund.clone(), charity_holder);
        }
        
        // Кошелек повара
        let chef_address = format!("chef_{}", food_truck);
        if let Some(holder) = self.token_holders.get_mut(&chef_address) {
            holder.add_security_tokens(chef_tokens);
        } else {
            let mut chef_holder = MockTokenHolder::new(chef_address.clone());
            chef_holder.add_security_tokens(chef_tokens);
            self.token_holders.insert(chef_address, chef_holder);
        }
        
        // Кошелек покупателя
        if let Some(holder) = self.token_holders.get_mut(&customer) {
            holder.add_security_tokens(customer_tokens);
        } else {
            let mut customer_holder = MockTokenHolder::new(customer.clone());
            customer_holder.add_security_tokens(customer_tokens);
            self.token_holders.insert(customer.clone(), customer_holder);
        }
        
        // Создаем заказ для повара
        self.create_chef_order(&customer, &food_truck, amount_subunits, &food_items, &check_id);
        
        check_id
    }

    fn create_chef_order(&mut self, customer: &str, food_truck: &str, amount_subunits: u128, food_items: &[String], check_id: &str) {
        // Проверяем, есть ли повар для этой ноды
        if let Some(chef_arm) = self.chef_arm_manager.get_chef_by_food_truck_mut(food_truck) {
            // Создаем элементы заказа
            let order_items: Vec<OrderItem> = food_items.iter().enumerate().map(|(i, item)| {
                OrderItem {
                    item_id: format!("item_{}_{}", check_id, i),
                    name: item.clone(),
                    quantity: 1,
                    price: amount_subunits / food_items.len() as u128,
                    special_instructions: None,
                    allergens: vec!["глютен".to_string()],
                    preparation_time: 15,
                }
            }).collect();

            // Создаем заказ для повара
            let chef_order = ChefOrder {
                order_id: check_id.to_string(),
                customer_id: customer.to_string(),
                food_truck_id: food_truck.to_string(),
                items: order_items,
                total_amount: amount_subunits,
                status: OrderStatus::Pending,
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                confirmed_at: None,
                started_at: None,
                ready_at: None,
                completed_at: None,
                estimated_time: 0,
                actual_time: None,
                chef_notes: None,
                customer_notes: None,
            };

            // Отправляем заказ повару
            if let Err(e) = chef_arm.receive_order(chef_order) {
                println!("⚠️ Не удалось отправить заказ повару: {}", e);
            } else {
                println!("🍳 Заказ {} отправлен повару ноды {}", check_id, food_truck);
            }
        } else {
            println!("⚠️ Повар не найден для ноды {}", food_truck);
        }
    }

    fn register_chef(&mut self, chef_id: String, food_truck_id: String) -> Result<(), String> {
        self.chef_arm_manager.register_chef(chef_id, food_truck_id)
    }

    fn activate_chef(&mut self, chef_id: &str) -> Result<(), String> {
        if let Some(chef_arm) = self.chef_arm_manager.get_chef_mut(chef_id) {
            chef_arm.activate();
            Ok(())
        } else {
            Err("Повар не найден".to_string())
        }
    }

    fn get_chef_tokens(&self, food_truck: &str) -> u128 {
        let chef_address = format!("chef_{}", food_truck);
        self.token_holders.get(&chef_address)
            .map(|h| h.security_tokens)
            .unwrap_or(0)
    }

    fn get_customer_tokens(&self, customer: &str) -> u128 {
        self.token_holders.get(customer)
            .map(|h| h.security_tokens)
            .unwrap_or(0)
    }

    fn get_owner_tokens(&self) -> u128 {
        self.token_holders.get(&self.main_owner)
            .map(|h| h.security_tokens)
            .unwrap_or(0)
    }

    fn get_charity_tokens(&self) -> u128 {
        self.token_holders.get(&self.charity_fund)
            .map(|h| h.security_tokens)
            .unwrap_or(0)
    }
}

#[test]
fn test_chef_token_distribution() {
    let mut blockchain = MockBlockchain::new("main_owner".to_string());
    
    // Регистрируем повара
    assert!(blockchain.register_chef("chef_001".to_string(), "truck_001".to_string()).is_ok());
    assert!(blockchain.activate_chef("chef_001").is_ok());
    
    // Покупатель делает заказ на 1000 subunits
    let customer = "customer_001".to_string();
    let food_truck = "truck_001".to_string();
    let amount = 1000;
    let food_items = vec!["Бургер".to_string(), "Картошка".to_string()];
    
    let check_id = blockchain.process_purchase_with_chef(
        customer.clone(),
        food_truck.clone(),
        amount,
        food_items,
    );
    
    // Проверяем распределение токенов
    let owner_tokens = blockchain.get_owner_tokens();
    let charity_tokens = blockchain.get_charity_tokens();
    let chef_tokens = blockchain.get_chef_tokens(&food_truck);
    let customer_tokens = blockchain.get_customer_tokens(&customer);
    
    // 48% владельцу = 480
    assert_eq!(owner_tokens, 480);
    // 3% фонду = 30
    assert_eq!(charity_tokens, 30);
    // 24% повару = 240
    assert_eq!(chef_tokens, 240);
    // 25% покупателю = 250
    assert_eq!(customer_tokens, 250);
    
    // Проверяем, что заказ создан для повара
    if let Some(chef_arm) = blockchain.chef_arm_manager.get_chef("chef_001") {
        assert_eq!(chef_arm.current_orders.len(), 1);
        assert_eq!(chef_arm.get_order_status(&check_id), Some(OrderStatus::Pending));
    }
}

#[test]
fn test_chef_order_workflow_integration() {
    let mut blockchain = MockBlockchain::new("main_owner".to_string());
    
    // Регистрируем повара
    assert!(blockchain.register_chef("chef_001".to_string(), "truck_001".to_string()).is_ok());
    assert!(blockchain.activate_chef("chef_001").is_ok());
    
    // Покупатель делает заказ
    let customer = "customer_001".to_string();
    let food_truck = "truck_001".to_string();
    let amount = 1000;
    let food_items = vec!["Бургер".to_string()];
    
    let check_id = blockchain.process_purchase_with_chef(
        customer.clone(),
        food_truck.clone(),
        amount,
        food_items,
    );
    
    // Повар подтверждает заказ
    if let Some(chef_arm) = blockchain.chef_arm_manager.get_chef_mut("chef_001") {
        assert!(chef_arm.confirm_order(&check_id, 15, Some("Без лука".to_string())).is_ok());
        assert_eq!(chef_arm.get_order_status(&check_id), Some(OrderStatus::Confirmed));
        
        // Повар начинает готовить
        assert!(chef_arm.start_cooking(&check_id).is_ok());
        assert_eq!(chef_arm.get_order_status(&check_id), Some(OrderStatus::InProgress));
        
        // Повар отмечает готовность
        assert!(chef_arm.mark_ready(&check_id).is_ok());
        assert_eq!(chef_arm.get_order_status(&check_id), Some(OrderStatus::Ready));
        
        // Повар завершает заказ
        assert!(chef_arm.complete_order(&check_id).is_ok());
        assert_eq!(chef_arm.get_order_status(&check_id), Some(OrderStatus::Completed));
        assert_eq!(chef_arm.current_orders.len(), 0);
        assert_eq!(chef_arm.completed_orders_today, 1);
    }
}

#[test]
fn test_multiple_chefs_different_trucks() {
    let mut blockchain = MockBlockchain::new("main_owner".to_string());
    
    // Регистрируем двух поваров для разных нод
    assert!(blockchain.register_chef("chef_001".to_string(), "truck_001".to_string()).is_ok());
    assert!(blockchain.register_chef("chef_002".to_string(), "truck_002".to_string()).is_ok());
    
    assert!(blockchain.activate_chef("chef_001").is_ok());
    assert!(blockchain.activate_chef("chef_002").is_ok());
    
    // Покупатели делают заказы в разных нодах
    let customer1 = "customer_001".to_string();
    let customer2 = "customer_002".to_string();
    let truck1 = "truck_001".to_string();
    let truck2 = "truck_002".to_string();
    let amount = 1000;
    let food_items = vec!["Бургер".to_string()];
    
    let check_id1 = blockchain.process_purchase_with_chef(
        customer1.clone(),
        truck1.clone(),
        amount,
        food_items.clone(),
    );
    
    let check_id2 = blockchain.process_purchase_with_chef(
        customer2.clone(),
        truck2.clone(),
        amount,
        food_items,
    );
    
    // Проверяем, что каждый повар получил свой заказ
    if let Some(chef1) = blockchain.chef_arm_manager.get_chef("chef_001") {
        assert_eq!(chef1.current_orders.len(), 1);
        assert_eq!(chef1.get_order_status(&check_id1), Some(OrderStatus::Pending));
    }
    
    if let Some(chef2) = blockchain.chef_arm_manager.get_chef("chef_002") {
        assert_eq!(chef2.current_orders.len(), 1);
        assert_eq!(chef2.get_order_status(&check_id2), Some(OrderStatus::Pending));
    }
    
    // Проверяем, что токены распределены правильно для каждого повара
    let chef1_tokens = blockchain.get_chef_tokens(&truck1);
    let chef2_tokens = blockchain.get_chef_tokens(&truck2);
    
    assert_eq!(chef1_tokens, 240); // 24% от 1000
    assert_eq!(chef2_tokens, 240); // 24% от 1000
}

#[test]
fn test_chef_without_registration() {
    let mut blockchain = MockBlockchain::new("main_owner".to_string());
    
    // Покупатель делает заказ в ноде без повара
    let customer = "customer_001".to_string();
    let food_truck = "truck_001".to_string();
    let amount = 1000;
    let food_items = vec!["Бургер".to_string()];
    
    let check_id = blockchain.process_purchase_with_chef(
        customer.clone(),
        food_truck.clone(),
        amount,
        food_items,
    );
    
    // Токены все равно должны распределиться
    let owner_tokens = blockchain.get_owner_tokens();
    let charity_tokens = blockchain.get_charity_tokens();
    let chef_tokens = blockchain.get_chef_tokens(&food_truck);
    let customer_tokens = blockchain.get_customer_tokens(&customer);
    
    assert_eq!(owner_tokens, 480);
    assert_eq!(charity_tokens, 30);
    assert_eq!(chef_tokens, 240); // Повар получает токены, даже если не зарегистрирован
    assert_eq!(customer_tokens, 250);
    
    // Но заказ не должен быть создан для повара
    // (в реальной реализации это должно логироваться как предупреждение)
}

#[test]
fn test_chef_statistics_after_multiple_orders() {
    let mut blockchain = MockBlockchain::new("main_owner".to_string());
    
    // Регистрируем повара
    assert!(blockchain.register_chef("chef_001".to_string(), "truck_001".to_string()).is_ok());
    assert!(blockchain.activate_chef("chef_001").is_ok());
    
    // Повар обрабатывает несколько заказов
    for i in 1..=3 {
        let customer = format!("customer_{:03}", i);
        let food_truck = "truck_001".to_string();
        let amount = 1000;
        let food_items = vec!["Бургер".to_string()];
        
        let check_id = blockchain.process_purchase_with_chef(
            customer,
            food_truck,
            amount,
            food_items,
        );
        
        // Повар обрабатывает заказ
        if let Some(chef_arm) = blockchain.chef_arm_manager.get_chef_mut("chef_001") {
            chef_arm.confirm_order(&check_id, 15, None).unwrap();
            chef_arm.start_cooking(&check_id).unwrap();
            chef_arm.mark_ready(&check_id).unwrap();
            chef_arm.complete_order(&check_id).unwrap();
        }
    }
    
    // Проверяем статистику повара
    if let Some(chef_arm) = blockchain.chef_arm_manager.get_chef("chef_001") {
        let stats = chef_arm.get_statistics();
        assert_eq!(stats.chef_id, "chef_001");
        assert_eq!(stats.total_orders, 3);
        assert_eq!(stats.completed_today, 3);
        assert_eq!(stats.active_orders, 0);
        assert!(stats.average_time > 0.0);
    }
    
    // Проверяем, что повар накопил токены
    let chef_tokens = blockchain.get_chef_tokens("truck_001");
    assert_eq!(chef_tokens, 720); // 24% от 3000 (3 заказа по 1000)
}
