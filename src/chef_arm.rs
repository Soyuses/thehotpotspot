use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Статусы заказа для повара
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,        // Заказ ожидает подтверждения повара
    Confirmed,      // Повар подтвердил заказ и начал готовить
    InProgress,     // Заказ готовится
    Ready,          // Заказ готов
    Completed,      // Заказ завершен (выдан клиенту)
    Cancelled,      // Заказ отменен
}

/// Информация о заказе для повара
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChefOrder {
    pub order_id: String,
    pub customer_id: String,
    pub food_truck_id: String,
    pub items: Vec<OrderItem>,
    pub total_amount: u128,
    pub status: OrderStatus,
    pub created_at: u64,
    pub confirmed_at: Option<u64>,
    pub started_at: Option<u64>,
    pub ready_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub estimated_time: u32, // В минутах
    pub actual_time: Option<u32>, // В минутах
    pub chef_notes: Option<String>,
    pub customer_notes: Option<String>,
}

/// Элемент заказа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub item_id: String,
    pub name: String,
    pub quantity: u32,
    pub price: u128,
    pub special_instructions: Option<String>,
    pub allergens: Vec<String>,
    pub preparation_time: u32, // В минутах
}

/// ARM повара - система управления заказами
#[derive(Debug, Clone)]
pub struct ChefARM {
    pub chef_id: String,
    pub food_truck_id: String,
    pub orders: HashMap<String, ChefOrder>,
    pub is_active: bool,
    pub current_orders: Vec<String>, // ID активных заказов
    pub completed_orders_today: u32,
    pub total_orders_processed: u32,
    pub average_preparation_time: f32, // В минутах
    pub chef_rating: f32, // Рейтинг повара (1.0 - 5.0)
    pub specializations: Vec<String>, // Специализации повара
    pub working_hours: (u8, u8), // (start_hour, end_hour)
    pub break_schedule: Vec<(u8, u8)>, // Время перерывов
}

impl ChefARM {
    /// Создание нового ARM повара
    pub fn new(chef_id: String, food_truck_id: String) -> Self {
        Self {
            chef_id,
            food_truck_id,
            orders: HashMap::new(),
            is_active: false,
            current_orders: Vec::new(),
            completed_orders_today: 0,
            total_orders_processed: 0,
            average_preparation_time: 0.0,
            chef_rating: 5.0, // Начальный рейтинг
            specializations: Vec::new(),
            working_hours: (8, 20), // 8:00 - 20:00 по умолчанию
            break_schedule: Vec::new(),
        }
    }

    /// Активация ARM повара
    pub fn activate(&mut self) {
        self.is_active = true;
        println!("🍳 ARM повара {} активирован для ноды {}", self.chef_id, self.food_truck_id);
    }

    /// Деактивация ARM повара
    pub fn deactivate(&mut self) {
        self.is_active = false;
        println!("🍳 ARM повара {} деактивирован", self.chef_id);
    }

    /// Получение нового заказа
    pub fn receive_order(&mut self, order: ChefOrder) -> Result<(), String> {
        if !self.is_active {
            return Err("ARM повара не активен".to_string());
        }

        if self.current_orders.len() >= 10 { // Максимум 10 активных заказов
            return Err("Слишком много активных заказов".to_string());
        }

        let order_id = order.order_id.clone();
        self.orders.insert(order_id.clone(), order);
        self.current_orders.push(order_id.clone());

        println!("🍳 Получен новый заказ {} для повара {}", order_id, self.chef_id);
        Ok(())
    }

    /// Подтверждение заказа поваром
    pub fn confirm_order(&mut self, order_id: &str, estimated_time: u32, chef_notes: Option<String>) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            if order.status != OrderStatus::Pending {
                return Err("Заказ уже обработан".to_string());
            }

            order.status = OrderStatus::Confirmed;
            order.confirmed_at = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            order.estimated_time = estimated_time;
            order.chef_notes = chef_notes;

            println!("✅ Повар {} подтвердил заказ {} (время приготовления: {} мин)", 
                     self.chef_id, order_id, estimated_time);
            Ok(())
        } else {
            Err("Заказ не найден".to_string())
        }
    }

    /// Начало приготовления заказа
    pub fn start_cooking(&mut self, order_id: &str) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            if order.status != OrderStatus::Confirmed {
                return Err("Заказ должен быть подтвержден перед началом приготовления".to_string());
            }

            order.status = OrderStatus::InProgress;
            order.started_at = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

            println!("👨‍🍳 Повар {} начал готовить заказ {}", self.chef_id, order_id);
            Ok(())
        } else {
            Err("Заказ не найден".to_string())
        }
    }

    /// Заказ готов
    pub fn mark_ready(&mut self, order_id: &str) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            if order.status != OrderStatus::InProgress {
                return Err("Заказ должен быть в процессе приготовления".to_string());
            }

            order.status = OrderStatus::Ready;
            order.ready_at = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

            // Вычисляем фактическое время приготовления
            if let (Some(started), Some(ready)) = (order.started_at, order.ready_at) {
                let actual_time = (ready - started) / 60; // В минутах
                order.actual_time = Some(actual_time as u32);
                
                // Обновляем среднее время приготовления
                self.update_average_preparation_time(actual_time as u32);
            }

            println!("🍽️ Заказ {} готов! Повар: {}", order_id, self.chef_id);
            Ok(())
        } else {
            Err("Заказ не найден".to_string())
        }
    }

    /// Завершение заказа (выдан клиенту)
    pub fn complete_order(&mut self, order_id: &str) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            if order.status != OrderStatus::Ready {
                return Err("Заказ должен быть готов перед завершением".to_string());
            }

            order.status = OrderStatus::Completed;
            order.completed_at = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

            // Удаляем из активных заказов
            self.current_orders.retain(|id| id != order_id);
            self.completed_orders_today += 1;
            self.total_orders_processed += 1;

            println!("✅ Заказ {} завершен поваром {}", order_id, self.chef_id);
            Ok(())
        } else {
            Err("Заказ не найден".to_string())
        }
    }

    /// Отмена заказа
    pub fn cancel_order(&mut self, order_id: &str, reason: String) -> Result<(), String> {
        if let Some(order) = self.orders.get_mut(order_id) {
            if order.status == OrderStatus::Completed {
                return Err("Завершенный заказ нельзя отменить".to_string());
            }

            order.status = OrderStatus::Cancelled;
            order.chef_notes = Some(format!("Отменен: {}", reason));

            // Удаляем из активных заказов
            self.current_orders.retain(|id| id != order_id);

            println!("❌ Заказ {} отменен поваром {}: {}", order_id, self.chef_id, reason);
            Ok(())
        } else {
            Err("Заказ не найден".to_string())
        }
    }

    /// Получение статуса заказа
    pub fn get_order_status(&self, order_id: &str) -> Option<OrderStatus> {
        self.orders.get(order_id).map(|order| order.status.clone())
    }

    /// Получение всех активных заказов
    pub fn get_active_orders(&self) -> Vec<&ChefOrder> {
        self.current_orders.iter()
            .filter_map(|id| self.orders.get(id))
            .collect()
    }

    /// Получение заказов по статусу
    pub fn get_orders_by_status(&self, status: OrderStatus) -> Vec<&ChefOrder> {
        self.orders.values()
            .filter(|order| order.status == status)
            .collect()
    }

    /// Обновление среднего времени приготовления
    fn update_average_preparation_time(&mut self, new_time: u32) {
        if self.total_orders_processed == 0 {
            self.average_preparation_time = new_time as f32;
        } else {
            let total_time = self.average_preparation_time * (self.total_orders_processed - 1) as f32 + new_time as f32;
            self.average_preparation_time = total_time / self.total_orders_processed as f32;
        }
    }

    /// Обновление рейтинга повара
    pub fn update_rating(&mut self, new_rating: f32) {
        if new_rating >= 1.0 && new_rating <= 5.0 {
            self.chef_rating = new_rating;
            println!("⭐ Рейтинг повара {} обновлен: {:.1}", self.chef_id, new_rating);
        }
    }

    /// Добавление специализации
    pub fn add_specialization(&mut self, specialization: String) {
        if !self.specializations.contains(&specialization) {
            self.specializations.push(specialization.clone());
            println!("🎯 Добавлена специализация повара {}: {}", self.chef_id, specialization);
        }
    }

    /// Настройка рабочего времени
    pub fn set_working_hours(&mut self, start_hour: u8, end_hour: u8) {
        if start_hour < 24 && end_hour < 24 && start_hour < end_hour {
            self.working_hours = (start_hour, end_hour);
            println!("⏰ Рабочее время повара {}: {}:00 - {}:00", 
                     self.chef_id, start_hour, end_hour);
        }
    }

    /// Добавление времени перерыва
    pub fn add_break(&mut self, start_hour: u8, end_hour: u8) {
        if start_hour < 24 && end_hour < 24 && start_hour < end_hour {
            self.break_schedule.push((start_hour, end_hour));
            println!("☕ Перерыв повара {}: {}:00 - {}:00", 
                     self.chef_id, start_hour, end_hour);
        }
    }

    /// Проверка, работает ли повар в данное время
    pub fn is_working_now(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let hour = (now / 3600) % 24;
        
        // Проверяем рабочее время
        if hour < self.working_hours.0 as u64 || hour >= self.working_hours.1 as u64 {
            return false;
        }

        // Проверяем перерывы
        for (break_start, break_end) in &self.break_schedule {
            if hour >= *break_start as u64 && hour < *break_end as u64 {
                return false;
            }
        }

        true
    }

    /// Получение статистики повара
    pub fn get_statistics(&self) -> ChefStatistics {
        ChefStatistics {
            chef_id: self.chef_id.clone(),
            total_orders: self.total_orders_processed,
            completed_today: self.completed_orders_today,
            active_orders: self.current_orders.len(),
            average_time: self.average_preparation_time,
            rating: self.chef_rating,
            is_working: self.is_working_now(),
        }
    }

    /// Сброс дневной статистики
    pub fn reset_daily_stats(&mut self) {
        self.completed_orders_today = 0;
        println!("📊 Дневная статистика повара {} сброшена", self.chef_id);
    }
}

/// Статистика повара
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChefStatistics {
    pub chef_id: String,
    pub total_orders: u32,
    pub completed_today: u32,
    pub active_orders: usize,
    pub average_time: f32,
    pub rating: f32,
    pub is_working: bool,
}

/// Менеджер ARM поваров
#[derive(Debug, Clone)]
pub struct ChefARMManager {
    pub chefs: HashMap<String, ChefARM>,
    pub food_truck_chefs: HashMap<String, String>, // food_truck_id -> chef_id
}

impl ChefARMManager {
    pub fn new() -> Self {
        Self {
            chefs: HashMap::new(),
            food_truck_chefs: HashMap::new(),
        }
    }

    /// Регистрация нового повара
    pub fn register_chef(&mut self, chef_id: String, food_truck_id: String) -> Result<(), String> {
        if self.chefs.contains_key(&chef_id) {
            return Err("Повар уже зарегистрирован".to_string());
        }

        if self.food_truck_chefs.contains_key(&food_truck_id) {
            return Err("У этой ноды уже есть повар".to_string());
        }

        let chef_arm = ChefARM::new(chef_id.clone(), food_truck_id.clone());
        self.chefs.insert(chef_id.clone(), chef_arm);
        self.food_truck_chefs.insert(food_truck_id.clone(), chef_id.clone());

        println!("👨‍🍳 Зарегистрирован новый повар: {} для ноды {}", chef_id, food_truck_id);
        Ok(())
    }

    /// Получение ARM повара по ID
    pub fn get_chef(&self, chef_id: &str) -> Option<&ChefARM> {
        self.chefs.get(chef_id)
    }

    /// Получение ARM повара по ID ноды
    pub fn get_chef_by_food_truck(&self, food_truck_id: &str) -> Option<&ChefARM> {
        if let Some(chef_id) = self.food_truck_chefs.get(food_truck_id) {
            self.chefs.get(chef_id)
        } else {
            None
        }
    }

    /// Получение мутабельного ARM повара по ID
    pub fn get_chef_mut(&mut self, chef_id: &str) -> Option<&mut ChefARM> {
        self.chefs.get_mut(chef_id)
    }

    /// Получение мутабельного ARM повара по ID ноды
    pub fn get_chef_by_food_truck_mut(&mut self, food_truck_id: &str) -> Option<&mut ChefARM> {
        if let Some(chef_id) = self.food_truck_chefs.get(food_truck_id) {
            self.chefs.get_mut(chef_id)
        } else {
            None
        }
    }

    /// Получение всех поваров
    pub fn get_all_chefs(&self) -> Vec<&ChefARM> {
        self.chefs.values().collect()
    }

    /// Получение статистики всех поваров
    pub fn get_all_statistics(&self) -> Vec<ChefStatistics> {
        self.chefs.values().map(|chef| chef.get_statistics()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chef_arm_creation() {
        let chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
        assert_eq!(chef_arm.chef_id, "chef_001");
        assert_eq!(chef_arm.food_truck_id, "truck_001");
        assert!(!chef_arm.is_active);
        assert_eq!(chef_arm.current_orders.len(), 0);
    }

    #[test]
    fn test_order_workflow() {
        let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
        chef_arm.activate();

        let order = ChefOrder {
            order_id: "order_001".to_string(),
            customer_id: "customer_001".to_string(),
            food_truck_id: "truck_001".to_string(),
            items: vec![OrderItem {
                item_id: "item_001".to_string(),
                name: "Бургер".to_string(),
                quantity: 1,
                price: 1000,
                special_instructions: None,
                allergens: vec!["глютен".to_string()],
                preparation_time: 15,
            }],
            total_amount: 1000,
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

        // Получение заказа
        assert!(chef_arm.receive_order(order).is_ok());
        assert_eq!(chef_arm.current_orders.len(), 1);

        // Подтверждение заказа
        assert!(chef_arm.confirm_order("order_001", 15, Some("Без лука".to_string())).is_ok());
        assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::Confirmed));

        // Начало приготовления
        assert!(chef_arm.start_cooking("order_001").is_ok());
        assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::InProgress));

        // Заказ готов
        assert!(chef_arm.mark_ready("order_001").is_ok());
        assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::Ready));

        // Завершение заказа
        assert!(chef_arm.complete_order("order_001").is_ok());
        assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::Completed));
        assert_eq!(chef_arm.current_orders.len(), 0);
        assert_eq!(chef_arm.completed_orders_today, 1);
    }

    #[test]
    fn test_chef_manager() {
        let mut manager = ChefARMManager::new();
        
        // Регистрация повара
        assert!(manager.register_chef("chef_001".to_string(), "truck_001".to_string()).is_ok());
        
        // Попытка зарегистрировать того же повара
        assert!(manager.register_chef("chef_001".to_string(), "truck_002".to_string()).is_err());
        
        // Попытка зарегистрировать повара для той же ноды
        assert!(manager.register_chef("chef_002".to_string(), "truck_001".to_string()).is_err());
        
        // Получение повара
        assert!(manager.get_chef("chef_001").is_some());
        assert!(manager.get_chef_by_food_truck("truck_001").is_some());
        assert!(manager.get_chef_by_food_truck("truck_002").is_none());
    }
}
