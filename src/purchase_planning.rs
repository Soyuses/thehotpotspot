use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use crate::recipe_management::{RecipeManager, DateRange};

/// Планировщик закупок
#[derive(Debug, Clone)]
pub struct PurchasePlanner {
    recipe_manager: RecipeManager,
    demand_forecaster: DemandForecaster,
    supplier_manager: SupplierManager,
}

/// Прогнозировщик спроса
#[derive(Debug, Clone)]
pub struct DemandForecaster {
    historical_data: HashMap<String, Vec<HistoricalDemand>>,
    seasonal_factors: HashMap<String, f64>,
}

/// Исторические данные спроса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDemand {
    pub date: DateTime<Utc>,
    pub dish_id: String,
    pub quantity: u32,
    pub franchise_id: String,
}

/// Менеджер поставщиков
#[derive(Debug, Clone)]
pub struct SupplierManager {
    suppliers: HashMap<String, Supplier>,
    price_history: HashMap<String, Vec<PricePoint>>,
}

/// Поставщик
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub contact_info: ContactInfo,
    pub delivery_time_days: u32,
    pub minimum_order_amount: f64,
    pub payment_terms: String,
    pub reliability_score: f64, // от 0 до 1
}

/// Контактная информация
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub phone: String,
    pub email: String,
    pub address: String,
}

/// Точка цены
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub date: DateTime<Utc>,
    pub ingredient: String,
    pub price_per_unit: f64,
    pub supplier_id: String,
}

/// Детальный план закупки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedPurchasePlan {
    pub id: String,
    pub franchise_id: String,
    pub period: DateRange,
    pub planned_dishes: HashMap<String, u32>,
    pub ingredient_orders: Vec<IngredientOrder>,
    pub total_estimated_cost: f64,
    pub delivery_schedule: Vec<DeliverySchedule>,
    pub created_at: DateTime<Utc>,
}

/// Заказ ингредиента
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientOrder {
    pub ingredient_name: String,
    pub quantity_kg: f64,
    pub supplier_id: String,
    pub unit_price: f64,
    pub total_cost: f64,
    pub delivery_date: DateTime<Utc>,
    pub order_date: DateTime<Utc>,
}

/// График поставок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySchedule {
    pub supplier_id: String,
    pub delivery_date: DateTime<Utc>,
    pub ingredients: Vec<String>,
    pub total_cost: f64,
    pub status: DeliveryStatus,
}

/// Статус поставки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Planned,
    Ordered,
    InTransit,
    Delivered,
    Cancelled,
}

impl PurchasePlanner {
    pub fn new() -> Self {
        let mut planner = Self {
            recipe_manager: RecipeManager::new(),
            demand_forecaster: DemandForecaster::new(),
            supplier_manager: SupplierManager::new(),
        };
        
        // Загружаем базовых поставщиков
        planner.supplier_manager.load_default_suppliers();
        planner
    }

    /// Создает детальный план закупки на 1000 блюд
    pub fn create_detailed_plan_for_1000_dishes(
        &self,
        franchise_id: String,
        dish_quantities: HashMap<String, u32>,
    ) -> DetailedPurchasePlan {
        let period = DateRange {
            start: Utc::now(),
            end: Utc::now() + Duration::days(30),
        };

        // Создаем базовый план
        let base_plan = self.recipe_manager.create_purchase_plan(
            franchise_id.clone(),
            dish_quantities.clone(),
            period.clone(),
        );

        // Создаем детальные заказы по поставщикам
        let mut ingredient_orders = Vec::new();
        let mut delivery_schedule = Vec::new();
        let mut total_cost = 0.0;

        for (ingredient, quantity_kg) in &base_plan.required_ingredients {
            // Выбираем лучшего поставщика для ингредиента
            if let Some(supplier) = self.supplier_manager.get_best_supplier_for_ingredient(ingredient) {
                let current_price = self.supplier_manager.get_current_price(ingredient, &supplier.id);
                let order_cost = quantity_kg * current_price;
                
                let order = IngredientOrder {
                    ingredient_name: ingredient.clone(),
                    quantity_kg: *quantity_kg,
                    supplier_id: supplier.id.clone(),
                    unit_price: current_price,
                    total_cost: order_cost,
                    delivery_date: Utc::now() + Duration::days(supplier.delivery_time_days as i64),
                    order_date: Utc::now(),
                };

                ingredient_orders.push(order.clone());
                total_cost += order_cost;

                // Добавляем в график поставок
                self.add_to_delivery_schedule(
                    &mut delivery_schedule,
                    &supplier.id,
                    order.delivery_date,
                    ingredient.clone(),
                    order_cost,
                );
            }
        }

        DetailedPurchasePlan {
            id: format!("detailed_plan_{}_{}", franchise_id, Utc::now().timestamp()),
            franchise_id,
            period,
            planned_dishes: dish_quantities,
            ingredient_orders,
            total_estimated_cost: total_cost,
            delivery_schedule,
            created_at: Utc::now(),
        }
    }

    /// Добавляет заказ в график поставок
    fn add_to_delivery_schedule(
        &self,
        schedule: &mut Vec<DeliverySchedule>,
        supplier_id: &str,
        delivery_date: DateTime<Utc>,
        ingredient: String,
        cost: f64,
    ) {
        // Ищем существующую поставку в этот день от этого поставщика
        if let Some(existing_delivery) = schedule.iter_mut()
            .find(|d| d.supplier_id == supplier_id && d.delivery_date.date_naive() == delivery_date.date_naive()) {
            existing_delivery.ingredients.push(ingredient);
            existing_delivery.total_cost += cost;
        } else {
            // Создаем новую поставку
            schedule.push(DeliverySchedule {
                supplier_id: supplier_id.to_string(),
                delivery_date,
                ingredients: vec![ingredient],
                total_cost: cost,
                status: DeliveryStatus::Planned,
            });
        }
    }

    /// Получает план закупки по ID франшизы
    pub fn get_purchase_plan(&self, franchise_id: &str) -> Option<&DetailedPurchasePlan> {
        // В реальной реализации здесь был бы поиск в базе данных
        None
    }

    /// Обновляет статус поставки
    pub fn update_delivery_status(
        &mut self,
        plan_id: &str,
        supplier_id: &str,
        delivery_date: DateTime<Utc>,
        status: DeliveryStatus,
    ) -> Result<(), String> {
        // В реальной реализации здесь было бы обновление в базе данных
        println!("Обновлен статус поставки: {} от {} на {} - {:?}", 
                plan_id, supplier_id, delivery_date, status);
        Ok(())
    }
}

impl DemandForecaster {
    pub fn new() -> Self {
        Self {
            historical_data: HashMap::new(),
            seasonal_factors: HashMap::new(),
        }
    }

    /// Прогнозирует спрос на блюдо
    pub fn forecast_demand(&self, dish_id: &str, days: u32) -> u32 {
        // Простой прогноз на основе исторических данных
        if let Some(history) = self.historical_data.get(dish_id) {
            let avg_daily = history.iter()
                .map(|h| h.quantity)
                .sum::<u32>() as f64 / history.len() as f64;
            
            (avg_daily * days as f64) as u32
        } else {
            // Дефолтные значения для новых блюд
            match dish_id {
                "plov_001" => 50 * days, // 50 порций плова в день
                "khinkali_001" => 30 * days, // 30 порций хинкали в день
                "khachapuri_001" => 40 * days, // 40 порций хачапури в день
                _ => 20 * days, // 20 порций других блюд в день
            }
        }
    }

    /// Добавляет исторические данные
    pub fn add_historical_data(&mut self, data: HistoricalDemand) {
        self.historical_data
            .entry(data.dish_id.clone())
            .or_insert_with(Vec::new)
            .push(data);
    }
}

impl SupplierManager {
    pub fn new() -> Self {
        Self {
            suppliers: HashMap::new(),
            price_history: HashMap::new(),
        }
    }

    /// Загружает базовых поставщиков
    pub fn load_default_suppliers(&mut self) {
        // РисТрейд
        let rice_trader = Supplier {
            id: "rice_trader_001".to_string(),
            name: "РисТрейд".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 123 4567".to_string(),
                email: "orders@ricetrader.ge".to_string(),
                address: "Тбилиси, ул. Агмашенебели, 123".to_string(),
            },
            delivery_time_days: 2,
            minimum_order_amount: 100.0,
            payment_terms: "Предоплата 50%".to_string(),
            reliability_score: 0.95,
        };

        // МясоМаркет
        let meat_market = Supplier {
            id: "meat_market_001".to_string(),
            name: "МясоМаркет".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 234 5678".to_string(),
                email: "orders@meatmarket.ge".to_string(),
                address: "Тбилиси, ул. Руставели, 456".to_string(),
            },
            delivery_time_days: 1,
            minimum_order_amount: 200.0,
            payment_terms: "Оплата при получении".to_string(),
            reliability_score: 0.90,
        };

        // ОвощиПлюс
        let vegetables_plus = Supplier {
            id: "vegetables_plus_001".to_string(),
            name: "ОвощиПлюс".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 345 6789".to_string(),
                email: "orders@vegetablesplus.ge".to_string(),
                address: "Тбилиси, ул. Плеханова, 789".to_string(),
            },
            delivery_time_days: 1,
            minimum_order_amount: 50.0,
            payment_terms: "Оплата при получении".to_string(),
            reliability_score: 0.85,
        };

        // СпецииМир
        let spices_world = Supplier {
            id: "spices_world_001".to_string(),
            name: "СпецииМир".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 456 7890".to_string(),
                email: "orders@spicesworld.ge".to_string(),
                address: "Тбилиси, ул. Чавчавадзе, 321".to_string(),
            },
            delivery_time_days: 3,
            minimum_order_amount: 20.0,
            payment_terms: "Предоплата 100%".to_string(),
            reliability_score: 0.88,
        };

        // МаслоТрейд
        let oil_trader = Supplier {
            id: "oil_trader_001".to_string(),
            name: "МаслоТрейд".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 567 8901".to_string(),
                email: "orders@oiltrader.ge".to_string(),
                address: "Тбилиси, ул. Казбеги, 654".to_string(),
            },
            delivery_time_days: 2,
            minimum_order_amount: 30.0,
            payment_terms: "Оплата при получении".to_string(),
            reliability_score: 0.92,
        };

        // СырМаркет
        let cheese_market = Supplier {
            id: "cheese_market_001".to_string(),
            name: "СырМаркет".to_string(),
            contact_info: ContactInfo {
                phone: "+995 32 678 9012".to_string(),
                email: "orders@cheesemarket.ge".to_string(),
                address: "Тбилиси, ул. Вере, 987".to_string(),
            },
            delivery_time_days: 1,
            minimum_order_amount: 50.0,
            payment_terms: "Оплата при получении".to_string(),
            reliability_score: 0.87,
        };

        // Добавляем поставщиков
        self.suppliers.insert(rice_trader.id.clone(), rice_trader);
        self.suppliers.insert(meat_market.id.clone(), meat_market);
        self.suppliers.insert(vegetables_plus.id.clone(), vegetables_plus);
        self.suppliers.insert(spices_world.id.clone(), spices_world);
        self.suppliers.insert(oil_trader.id.clone(), oil_trader);
        self.suppliers.insert(cheese_market.id.clone(), cheese_market);

        // Загружаем историю цен
        self.load_price_history();
    }

    /// Загружает историю цен
    fn load_price_history(&mut self) {
        let current_date = Utc::now();
        
        // Цены на рис
        self.add_price_point("Рис", "rice_trader_001", 0.002, current_date);
        
        // Цены на мясо
        self.add_price_point("Говядина", "meat_market_001", 0.005, current_date);
        
        // Цены на овощи
        self.add_price_point("Лук репчатый", "vegetables_plus_001", 0.001, current_date);
        self.add_price_point("Морковь", "vegetables_plus_001", 0.0008, current_date);
        
        // Цены на специи
        self.add_price_point("Соль", "spices_world_001", 0.0005, current_date);
        self.add_price_point("Перец черный", "spices_world_001", 0.02, current_date);
        
        // Цены на масло
        self.add_price_point("Подсолнечное масло", "oil_trader_001", 0.003, current_date);
        
        // Цены на сыр
        self.add_price_point("Сыр сулугуни", "cheese_market_001", 0.008, current_date);
    }

    /// Добавляет точку цены
    fn add_price_point(&mut self, ingredient: &str, supplier_id: &str, price: f64, date: DateTime<Utc>) {
        let key = format!("{}:{}", ingredient, supplier_id);
        self.price_history
            .entry(key)
            .or_insert_with(Vec::new)
            .push(PricePoint {
                date,
                ingredient: ingredient.to_string(),
                price_per_unit: price,
                supplier_id: supplier_id.to_string(),
            });
    }

    /// Получает лучшего поставщика для ингредиента
    pub fn get_best_supplier_for_ingredient(&self, ingredient: &str) -> Option<&Supplier> {
        // Простая логика выбора поставщика
        match ingredient {
            "Рис" => self.suppliers.get("rice_trader_001"),
            "Говядина" => self.suppliers.get("meat_market_001"),
            "Лук репчатый" | "Морковь" => self.suppliers.get("vegetables_plus_001"),
            "Соль" | "Перец черный" => self.suppliers.get("spices_world_001"),
            "Подсолнечное масло" => self.suppliers.get("oil_trader_001"),
            "Сыр сулугуни" => self.suppliers.get("cheese_market_001"),
            _ => None,
        }
    }

    /// Получает текущую цену ингредиента от поставщика
    pub fn get_current_price(&self, ingredient: &str, supplier_id: &str) -> f64 {
        let key = format!("{}:{}", ingredient, supplier_id);
        if let Some(prices) = self.price_history.get(&key) {
            if let Some(latest_price) = prices.last() {
                return latest_price.price_per_unit;
            }
        }
        
        // Дефолтные цены если нет истории
        match ingredient {
            "Рис" => 0.002,
            "Говядина" => 0.005,
            "Лук репчатый" => 0.001,
            "Морковь" => 0.0008,
            "Соль" => 0.0005,
            "Перец черный" => 0.02,
            "Подсолнечное масло" => 0.003,
            "Сыр сулугуни" => 0.008,
            _ => 0.001, // дефолтная цена
        }
    }
}

impl Default for PurchasePlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_detailed_plan_creation() {
        let planner = PurchasePlanner::new();
        let mut dish_quantities = HashMap::new();
        dish_quantities.insert("plov_001".to_string(), 1000); // 1000 порций плова

        let plan = planner.create_detailed_plan_for_1000_dishes(
            "franchise_001".to_string(),
            dish_quantities,
        );

        // Проверяем, что план создан
        assert_eq!(plan.planned_dishes.get("plov_001"), Some(&1000));
        assert!(!plan.ingredient_orders.is_empty());
        assert!(plan.total_estimated_cost > 0.0);

        // Проверяем, что есть заказы на основные ингредиенты плова
        let has_rice = plan.ingredient_orders.iter().any(|o| o.ingredient_name == "Рис");
        let has_meat = plan.ingredient_orders.iter().any(|o| o.ingredient_name == "Говядина");
        
        assert!(has_rice);
        assert!(has_meat);
    }

    #[test]
    fn test_supplier_selection() {
        let planner = PurchasePlanner::new();
        let supplier_manager = &planner.supplier_manager;

        // Проверяем выбор поставщика для риса
        let rice_supplier = supplier_manager.get_best_supplier_for_ingredient("Рис");
        assert!(rice_supplier.is_some());
        assert_eq!(rice_supplier.unwrap().name, "РисТрейд");

        // Проверяем цену
        let rice_price = supplier_manager.get_current_price("Рис", "rice_trader_001");
        assert_eq!(rice_price, 0.002);
    }
}
