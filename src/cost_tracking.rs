use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Datelike};
use crate::purchase_planning::{PurchasePlanner, DetailedPurchasePlan, IngredientOrder};
use crate::recipe_management::RecipeManager;

/// Отслеживание затрат
#[derive(Debug, Clone)]
pub struct CostTracker {
    purchase_planner: PurchasePlanner,
    actual_purchases: Vec<ActualPurchase>,
    cost_analyzer: CostAnalyzer,
    pub token_calculator: TokenCalculator,
}

/// Фактическая закупка
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualPurchase {
    pub id: String,
    pub franchise_id: String,
    pub ingredient: String,
    pub quantity_kg: f64,
    pub cost: f64,
    pub date: DateTime<Utc>,
    pub supplier: String,
    pub invoice_number: Option<String>,
    pub status: PurchaseStatus,
}

/// Статус закупки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurchaseStatus {
    Ordered,
    Delivered,
    Verified,
    Paid,
    Cancelled,
}

/// Анализатор затрат
#[derive(Debug, Clone)]
pub struct CostAnalyzer {
    cost_categories: HashMap<String, CostCategory>,
    monthly_limits: HashMap<String, f64>,
}

/// Категория затрат
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategory {
    pub id: String,
    pub name: String,
    pub percentage_of_total: f64,
    pub description: String,
}

/// Калькулятор токенов
#[derive(Debug, Clone)]
pub struct TokenCalculator {
    token_per_5_lari: f64, // 1 токен за каждые 5 лари затрат
    minimum_cost_threshold: f64, // минимальный порог для эмиссии токенов
}

/// Отчет о затратах
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    pub id: String,
    pub franchise_id: String,
    pub period: DateRange,
    pub total_costs: f64,
    pub cost_breakdown: HashMap<String, f64>,
    pub tokens_to_emit: u64,
    pub transparency_score: f64,
    pub created_at: DateTime<Utc>,
}

/// Диапазон дат
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Анализ эффективности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyAnalysis {
    pub franchise_id: String,
    pub period: DateRange,
    pub planned_vs_actual: PlannedVsActual,
    pub cost_per_dish: HashMap<String, f64>,
    pub waste_percentage: f64,
    pub supplier_performance: HashMap<String, SupplierPerformance>,
}

/// Планируемые vs фактические затраты
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedVsActual {
    pub planned_cost: f64,
    pub actual_cost: f64,
    pub variance: f64,
    pub variance_percentage: f64,
}

/// Производительность поставщика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierPerformance {
    pub supplier_id: String,
    pub total_orders: u32,
    pub on_time_deliveries: u32,
    pub quality_score: f64,
    pub average_price_variance: f64,
}

impl CostTracker {
    pub fn new() -> Self {
        let mut tracker = Self {
            purchase_planner: PurchasePlanner::new(),
            actual_purchases: Vec::new(),
            cost_analyzer: CostAnalyzer::new(),
            token_calculator: TokenCalculator::new(),
        };
        
        // Инициализируем категории затрат
        tracker.cost_analyzer.initialize_cost_categories();
        tracker
    }

    /// Регистрирует фактическую закупку
    pub fn register_actual_purchase(&mut self, purchase: ActualPurchase) -> Result<(), String> {
        // Валидация
        if purchase.quantity_kg <= 0.0 {
            return Err("Количество должно быть больше 0".to_string());
        }
        
        if purchase.cost <= 0.0 {
            return Err("Стоимость должна быть больше 0".to_string());
        }

        self.actual_purchases.push(purchase);
        Ok(())
    }

    /// Создает отчет о затратах за период
    pub fn create_cost_report(
        &self,
        franchise_id: &str,
        period: DateRange,
    ) -> CostReport {
        let purchases_in_period: Vec<&ActualPurchase> = self.actual_purchases
            .iter()
            .filter(|p| {
                p.franchise_id == franchise_id &&
                p.date >= period.start &&
                p.date <= period.end &&
                matches!(p.status, PurchaseStatus::Paid)
            })
            .collect();

        let total_costs = purchases_in_period.iter()
            .map(|p| p.cost)
            .sum();

        let mut cost_breakdown = HashMap::new();
        for purchase in &purchases_in_period {
            let category = self.cost_analyzer.get_category_for_ingredient(&purchase.ingredient);
            *cost_breakdown.entry(category).or_insert(0.0) += purchase.cost;
        }

        let tokens_to_emit = self.token_calculator.calculate_tokens(total_costs);
        let transparency_score = self.calculate_transparency_score(&purchases_in_period);

        CostReport {
            id: format!("report_{}_{}", franchise_id, Utc::now().timestamp()),
            franchise_id: franchise_id.to_string(),
            period,
            total_costs,
            cost_breakdown,
            tokens_to_emit,
            transparency_score,
            created_at: Utc::now(),
        }
    }

    /// Создает план закупки на 1000 блюд и рассчитывает токены
    pub fn create_plan_and_calculate_tokens(
        &self,
        franchise_id: String,
        dish_quantities: HashMap<String, u32>,
    ) -> (DetailedPurchasePlan, u64) {
        let plan = self.purchase_planner.create_detailed_plan_for_1000_dishes(
            franchise_id,
            dish_quantities,
        );

        let tokens = self.token_calculator.calculate_tokens(plan.total_estimated_cost);
        
        (plan, tokens)
    }

    /// Анализирует эффективность франшизы
    pub fn analyze_efficiency(
        &self,
        franchise_id: &str,
        period: DateRange,
    ) -> EfficiencyAnalysis {
        let purchases_in_period: Vec<&ActualPurchase> = self.actual_purchases
            .iter()
            .filter(|p| {
                p.franchise_id == franchise_id &&
                p.date >= period.start &&
                p.date <= period.end
            })
            .collect();

        let actual_cost = purchases_in_period.iter()
            .map(|p| p.cost)
            .sum();

        // Получаем планируемые затраты (в реальной системе из базы данных)
        let planned_cost = actual_cost * 0.9; // Примерно 90% от фактических затрат
        let variance = actual_cost - planned_cost;
        let variance_percentage = (variance / planned_cost) * 100.0;

        let planned_vs_actual = PlannedVsActual {
            planned_cost,
            actual_cost,
            variance,
            variance_percentage,
        };

        let cost_per_dish = self.calculate_cost_per_dish(&purchases_in_period);
        let waste_percentage = self.calculate_waste_percentage(&purchases_in_period);
        let supplier_performance = self.analyze_supplier_performance(&purchases_in_period);

        EfficiencyAnalysis {
            franchise_id: franchise_id.to_string(),
            period,
            planned_vs_actual,
            cost_per_dish,
            waste_percentage,
            supplier_performance,
        }
    }

    /// Рассчитывает стоимость на блюдо
    fn calculate_cost_per_dish(&self, purchases: &[&ActualPurchase]) -> HashMap<String, f64> {
        let mut dish_costs = HashMap::new();
        
        // Группируем по ингредиентам и рассчитываем стоимость блюд
        // Это упрощенная логика - в реальной системе нужна связь с рецептами
        for purchase in purchases {
            let dish_name = self.get_dish_name_for_ingredient(&purchase.ingredient);
            *dish_costs.entry(dish_name).or_insert(0.0) += purchase.cost;
        }
        
        dish_costs
    }

    /// Получает название блюда для ингредиента
    fn get_dish_name_for_ingredient(&self, ingredient: &str) -> String {
        match ingredient {
            "Рис" | "Говядина" | "Лук репчатый" | "Морковь" | "Подсолнечное масло" | "Соль" | "Перец черный" => "Плов".to_string(),
            "Мука пшеничная" => "Хинкали".to_string(),
            "Сыр сулугуни" | "Яйца куриные" | "Сливочное масло" => "Хачапури".to_string(),
            _ => "Другие блюда".to_string(),
        }
    }

    /// Рассчитывает процент потерь
    fn calculate_waste_percentage(&self, _purchases: &[&ActualPurchase]) -> f64 {
        // В реальной системе здесь был бы расчет на основе фактических данных
        10.0 // 10% потерь по умолчанию
    }

    /// Анализирует производительность поставщиков
    fn analyze_supplier_performance(&self, purchases: &[&ActualPurchase]) -> HashMap<String, SupplierPerformance> {
        let mut supplier_stats: HashMap<String, Vec<&ActualPurchase>> = HashMap::new();
        
        for purchase in purchases {
            supplier_stats.entry(purchase.supplier.clone())
                .or_insert_with(Vec::new)
                .push(purchase);
        }

        let mut performance = HashMap::new();
        for (supplier_id, supplier_purchases) in supplier_stats {
            let total_orders = supplier_purchases.len() as u32;
            let on_time_deliveries = supplier_purchases.iter()
                .filter(|p| matches!(p.status, PurchaseStatus::Delivered | PurchaseStatus::Paid))
                .count() as u32;
            
            let quality_score = if total_orders > 0 {
                on_time_deliveries as f64 / total_orders as f64
            } else {
                0.0
            };

            performance.insert(supplier_id.clone(), SupplierPerformance {
                supplier_id,
                total_orders,
                on_time_deliveries,
                quality_score,
                average_price_variance: 0.0, // Упрощенно
            });
        }

        performance
    }

    /// Рассчитывает оценку прозрачности
    fn calculate_transparency_score(&self, purchases: &[&ActualPurchase]) -> f64 {
        if purchases.is_empty() {
            return 0.0;
        }

        let total_purchases = purchases.len();
        let verified_purchases = purchases.iter()
            .filter(|p| p.invoice_number.is_some())
            .count();

        (verified_purchases as f64 / total_purchases as f64) * 100.0
    }

    /// Получает все закупки франшизы
    pub fn get_franchise_purchases(&self, franchise_id: &str) -> Vec<&ActualPurchase> {
        self.actual_purchases
            .iter()
            .filter(|p| p.franchise_id == franchise_id)
            .collect()
    }

    /// Получает статистику по токенам
    pub fn get_token_statistics(&self, franchise_id: &str) -> TokenStatistics {
        let purchases = self.get_franchise_purchases(franchise_id);
        let total_cost = purchases.iter()
            .filter(|p| matches!(p.status, PurchaseStatus::Paid))
            .map(|p| p.cost)
            .sum();

        let total_tokens = self.token_calculator.calculate_tokens(total_cost);
        let monthly_tokens = self.calculate_monthly_tokens(&purchases);

        TokenStatistics {
            franchise_id: franchise_id.to_string(),
            total_cost,
            total_tokens,
            monthly_tokens,
            last_calculation: Utc::now(),
        }
    }

    /// Рассчитывает токены по месяцам
    fn calculate_monthly_tokens(&self, purchases: &[&ActualPurchase]) -> HashMap<String, u64> {
        let mut monthly_costs: HashMap<String, f64> = HashMap::new();
        
        for purchase in purchases {
            if matches!(purchase.status, PurchaseStatus::Paid) {
                let month_key = format!("{}-{:02}", 
                    purchase.date.year(), 
                    purchase.date.month()
                );
                *monthly_costs.entry(month_key).or_insert(0.0) += purchase.cost;
            }
        }

        monthly_costs.iter()
            .map(|(month, cost)| (month.clone(), self.token_calculator.calculate_tokens(*cost)))
            .collect()
    }
}

impl CostAnalyzer {
    pub fn new() -> Self {
        Self {
            cost_categories: HashMap::new(),
            monthly_limits: HashMap::new(),
        }
    }

    /// Инициализирует категории затрат
    pub fn initialize_cost_categories(&mut self) {
        let categories = vec![
            CostCategory {
                id: "ingredients".to_string(),
                name: "Сырье и ингредиенты".to_string(),
                percentage_of_total: 40.0,
                description: "Затраты на продукты и ингредиенты".to_string(),
            },
            CostCategory {
                id: "rent".to_string(),
                name: "Аренда и коммунальные услуги".to_string(),
                percentage_of_total: 20.0,
                description: "Аренда помещения и коммунальные платежи".to_string(),
            },
            CostCategory {
                id: "salary".to_string(),
                name: "Зарплата персонала".to_string(),
                percentage_of_total: 25.0,
                description: "Зарплаты сотрудников".to_string(),
            },
            CostCategory {
                id: "marketing".to_string(),
                name: "Маркетинг и реклама".to_string(),
                percentage_of_total: 10.0,
                description: "Реклама и маркетинговые мероприятия".to_string(),
            },
            CostCategory {
                id: "other".to_string(),
                name: "Прочие расходы".to_string(),
                percentage_of_total: 5.0,
                description: "Прочие операционные расходы".to_string(),
            },
        ];

        for category in categories {
            self.cost_categories.insert(category.id.clone(), category);
        }
    }

    /// Получает категорию для ингредиента
    pub fn get_category_for_ingredient(&self, ingredient: &str) -> String {
        match ingredient {
            "Рис" | "Говядина" | "Лук репчатый" | "Морковь" | 
            "Подсолнечное масло" | "Соль" | "Перец черный" |
            "Мука пшеничная" | "Сыр сулугуни" | "Яйца куриные" | 
            "Сливочное масло" => "Сырье и ингредиенты".to_string(),
            _ => "Прочие расходы".to_string(),
        }
    }
}

impl TokenCalculator {
    pub fn new() -> Self {
        Self {
            token_per_5_lari: 1.0, // 1 токен за каждые 5 лари
            minimum_cost_threshold: 5.0, // минимум 5 лари для эмиссии токена
        }
    }

    /// Рассчитывает количество токенов на основе затрат
    pub fn calculate_tokens(&self, total_cost: f64) -> u64 {
        if total_cost < self.minimum_cost_threshold {
            return 0;
        }
        
        (total_cost / 5.0) as u64
    }

    /// Получает стоимость одного токена в лари
    pub fn get_token_cost_in_lari(&self) -> f64 {
        5.0
    }
}

/// Статистика токенов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatistics {
    pub franchise_id: String,
    pub total_cost: f64,
    pub total_tokens: u64,
    pub monthly_tokens: HashMap<String, u64>,
    pub last_calculation: DateTime<Utc>,
}

impl Default for CostTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_token_calculation() {
        let calculator = TokenCalculator::new();
        
        // 25 лари затрат = 5 токенов
        assert_eq!(calculator.calculate_tokens(25.0), 5);
        
        // 4 лари затрат = 0 токенов (меньше минимума)
        assert_eq!(calculator.calculate_tokens(4.0), 0);
        
        // 100 лари затрат = 20 токенов
        assert_eq!(calculator.calculate_tokens(100.0), 20);
    }

    #[test]
    fn test_cost_report_creation() {
        let mut tracker = CostTracker::new();
        
        // Добавляем тестовую закупку
        let purchase = ActualPurchase {
            id: "test_001".to_string(),
            franchise_id: "franchise_001".to_string(),
            ingredient: "Рис".to_string(),
            quantity_kg: 100.0,
            cost: 200.0, // 200 лари
            date: Utc::now(),
            supplier: "rice_trader_001".to_string(),
            invoice_number: Some("INV-001".to_string()),
            status: PurchaseStatus::Paid,
        };
        
        tracker.register_actual_purchase(purchase).unwrap();
        
        let period = DateRange {
            start: Utc::now() - chrono::Duration::days(30),
            end: Utc::now(),
        };
        
        let report = tracker.create_cost_report("franchise_001", period);
        
        assert_eq!(report.total_costs, 200.0);
        assert_eq!(report.tokens_to_emit, 40); // 200 лари / 5 = 40 токенов
        assert!(report.transparency_score > 0.0);
    }

    #[test]
    fn test_plan_creation_for_1000_dishes() {
        let tracker = CostTracker::new();
        let mut dish_quantities = HashMap::new();
        dish_quantities.insert("plov_001".to_string(), 1000);

        let (plan, tokens) = tracker.create_plan_and_calculate_tokens(
            "franchise_001".to_string(),
            dish_quantities,
        );

        assert_eq!(plan.planned_dishes.get("plov_001"), Some(&1000));
        assert!(tokens > 0);
        assert!(plan.total_estimated_cost > 0.0);
    }
}
