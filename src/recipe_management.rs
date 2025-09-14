use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Управление рецептами блюд
#[derive(Debug, Clone)]
pub struct RecipeManager {
    recipes: HashMap<String, Recipe>,
    ingredient_calculator: IngredientCalculator,
}

/// Рецепт блюда
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub category: String,
    pub ingredients: Vec<Ingredient>,
    pub portions: u32,
    pub cooking_time_minutes: u32,
    pub difficulty: Difficulty,
    pub cost_per_portion: f64, // в лари
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ингредиент в рецепте
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount_per_portion: f64, // в граммах
    pub unit: String,
    pub cost_per_unit: f64, // в лари за единицу
    pub category: IngredientCategory,
    pub supplier: Option<String>,
    pub shelf_life_days: Option<u32>,
}

/// Категория ингредиента
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IngredientCategory {
    Meat,
    Vegetables,
    Grains,
    Spices,
    Dairy,
    Oils,
    Beverages,
    Other,
}

/// Сложность приготовления
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Калькулятор ингредиентов
#[derive(Debug, Clone)]
pub struct IngredientCalculator {
    waste_factor: f64, // коэффициент потерь при приготовлении (например, 0.1 = 10% потерь)
}

/// План закупки ингредиентов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchasePlan {
    pub id: String,
    pub franchise_id: String,
    pub period: DateRange,
    pub planned_dishes: HashMap<String, u32>, // блюдо -> количество порций
    pub required_ingredients: HashMap<String, f64>, // ингредиент -> количество в кг
    pub estimated_costs: f64,
    pub created_at: DateTime<Utc>,
}

/// Диапазон дат
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl RecipeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            recipes: HashMap::new(),
            ingredient_calculator: IngredientCalculator::new(0.1), // 10% потерь
        };
        
        // Загружаем базовые рецепты
        manager.load_default_recipes();
        manager
    }

    /// Загружает базовые рецепты с реальными данными
    fn load_default_recipes(&mut self) {
        // Плов
        let plov = Recipe {
            id: "plov_001".to_string(),
            name: "Плов с мясом".to_string(),
            category: "Основные блюда".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "Рис".to_string(),
                    amount_per_portion: 200.0, // 200г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.002, // 2 лари за кг
                    category: IngredientCategory::Grains,
                    supplier: Some("РисТрейд".to_string()),
                    shelf_life_days: Some(365),
                },
                Ingredient {
                    name: "Говядина".to_string(),
                    amount_per_portion: 150.0, // 150г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.005, // 5 лари за кг
                    category: IngredientCategory::Meat,
                    supplier: Some("МясоМаркет".to_string()),
                    shelf_life_days: Some(3),
                },
                Ingredient {
                    name: "Лук репчатый".to_string(),
                    amount_per_portion: 50.0, // 50г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.001, // 1 лари за кг
                    category: IngredientCategory::Vegetables,
                    supplier: Some("ОвощиПлюс".to_string()),
                    shelf_life_days: Some(30),
                },
                Ingredient {
                    name: "Морковь".to_string(),
                    amount_per_portion: 50.0, // 50г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.0008, // 0.8 лари за кг
                    category: IngredientCategory::Vegetables,
                    supplier: Some("ОвощиПлюс".to_string()),
                    shelf_life_days: Some(14),
                },
                Ingredient {
                    name: "Подсолнечное масло".to_string(),
                    amount_per_portion: 20.0, // 20г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.003, // 3 лари за кг
                    category: IngredientCategory::Oils,
                    supplier: Some("МаслоТрейд".to_string()),
                    shelf_life_days: Some(180),
                },
                Ingredient {
                    name: "Соль".to_string(),
                    amount_per_portion: 5.0, // 5г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.0005, // 0.5 лари за кг
                    category: IngredientCategory::Spices,
                    supplier: Some("СпецииМир".to_string()),
                    shelf_life_days: None,
                },
                Ingredient {
                    name: "Перец черный".to_string(),
                    amount_per_portion: 1.0, // 1г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.02, // 20 лари за кг
                    category: IngredientCategory::Spices,
                    supplier: Some("СпецииМир".to_string()),
                    shelf_life_days: Some(730),
                },
            ],
            portions: 1,
            cooking_time_minutes: 60,
            difficulty: Difficulty::Medium,
            cost_per_portion: 0.0, // будет рассчитано автоматически
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Хинкали
        let khinkali = Recipe {
            id: "khinkali_001".to_string(),
            name: "Хинкали с мясом".to_string(),
            category: "Основные блюда".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "Мука пшеничная".to_string(),
                    amount_per_portion: 100.0, // 100г на порцию (8-10 штук)
                    unit: "г".to_string(),
                    cost_per_unit: 0.0015, // 1.5 лари за кг
                    category: IngredientCategory::Grains,
                    supplier: Some("МукаТрейд".to_string()),
                    shelf_life_days: Some(180),
                },
                Ingredient {
                    name: "Говядина".to_string(),
                    amount_per_portion: 80.0, // 80г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.005, // 5 лари за кг
                    category: IngredientCategory::Meat,
                    supplier: Some("МясоМаркет".to_string()),
                    shelf_life_days: Some(3),
                },
                Ingredient {
                    name: "Лук репчатый".to_string(),
                    amount_per_portion: 30.0, // 30г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.001, // 1 лари за кг
                    category: IngredientCategory::Vegetables,
                    supplier: Some("ОвощиПлюс".to_string()),
                    shelf_life_days: Some(30),
                },
                Ingredient {
                    name: "Соль".to_string(),
                    amount_per_portion: 3.0, // 3г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.0005, // 0.5 лари за кг
                    category: IngredientCategory::Spices,
                    supplier: Some("СпецииМир".to_string()),
                    shelf_life_days: None,
                },
                Ingredient {
                    name: "Перец черный".to_string(),
                    amount_per_portion: 1.0, // 1г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.02, // 20 лари за кг
                    category: IngredientCategory::Spices,
                    supplier: Some("СпецииМир".to_string()),
                    shelf_life_days: Some(730),
                },
            ],
            portions: 1,
            cooking_time_minutes: 45,
            difficulty: Difficulty::Hard,
            cost_per_portion: 0.0, // будет рассчитано автоматически
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Хачапури
        let khachapuri = Recipe {
            id: "khachapuri_001".to_string(),
            name: "Хачапури по-аджарски".to_string(),
            category: "Основные блюда".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "Мука пшеничная".to_string(),
                    amount_per_portion: 150.0, // 150г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.0015, // 1.5 лари за кг
                    category: IngredientCategory::Grains,
                    supplier: Some("МукаТрейд".to_string()),
                    shelf_life_days: Some(180),
                },
                Ingredient {
                    name: "Сыр сулугуни".to_string(),
                    amount_per_portion: 100.0, // 100г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.008, // 8 лари за кг
                    category: IngredientCategory::Dairy,
                    supplier: Some("СырМаркет".to_string()),
                    shelf_life_days: Some(7),
                },
                Ingredient {
                    name: "Яйца куриные".to_string(),
                    amount_per_portion: 1.0, // 1 яйцо на порцию (60г)
                    unit: "шт".to_string(),
                    cost_per_unit: 0.3, // 0.3 лари за штуку
                    category: IngredientCategory::Dairy,
                    supplier: Some("ЯйцоФерма".to_string()),
                    shelf_life_days: Some(14),
                },
                Ingredient {
                    name: "Сливочное масло".to_string(),
                    amount_per_portion: 20.0, // 20г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.012, // 12 лари за кг
                    category: IngredientCategory::Dairy,
                    supplier: Some("МолочныеПродукты".to_string()),
                    shelf_life_days: Some(30),
                },
                Ingredient {
                    name: "Соль".to_string(),
                    amount_per_portion: 2.0, // 2г на порцию
                    unit: "г".to_string(),
                    cost_per_unit: 0.0005, // 0.5 лари за кг
                    category: IngredientCategory::Spices,
                    supplier: Some("СпецииМир".to_string()),
                    shelf_life_days: None,
                },
            ],
            portions: 1,
            cooking_time_minutes: 30,
            difficulty: Difficulty::Medium,
            cost_per_portion: 0.0, // будет рассчитано автоматически
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Добавляем рецепты
        self.add_recipe(plov);
        self.add_recipe(khinkali);
        self.add_recipe(khachapuri);
    }

    /// Добавляет рецепт
    pub fn add_recipe(&mut self, mut recipe: Recipe) {
        // Рассчитываем стоимость порции
        recipe.cost_per_portion = self.calculate_recipe_cost(&recipe);
        self.recipes.insert(recipe.id.clone(), recipe);
    }

    /// Получает рецепт по ID
    pub fn get_recipe(&self, id: &str) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    /// Получает все рецепты
    pub fn get_all_recipes(&self) -> Vec<&Recipe> {
        self.recipes.values().collect()
    }

    /// Рассчитывает стоимость рецепта
    pub fn calculate_recipe_cost(&self, recipe: &Recipe) -> f64 {
        recipe.ingredients.iter()
            .map(|ingredient| {
                let amount_in_kg = ingredient.amount_per_portion / 1000.0;
                amount_in_kg * ingredient.cost_per_unit
            })
            .sum()
    }

    /// Создает план закупки на основе рецептов и количества порций
    pub fn create_purchase_plan(
        &self,
        franchise_id: String,
        planned_dishes: HashMap<String, u32>,
        period: DateRange,
    ) -> PurchasePlan {
        let mut required_ingredients: HashMap<String, f64> = HashMap::new();
        let mut total_cost = 0.0;

        // Проходим по всем запланированным блюдам
        for (recipe_id, portions) in planned_dishes.iter() {
            if let Some(recipe) = self.get_recipe(recipe_id) {
                // Рассчитываем ингредиенты для данного количества порций
                for ingredient in &recipe.ingredients {
                    let total_amount = ingredient.amount_per_portion * (*portions as f64);
                    let amount_with_waste = total_amount * (1.0 + self.ingredient_calculator.waste_factor);
                    let amount_in_kg = amount_with_waste / 1000.0;

                    // Добавляем к общему количеству
                    *required_ingredients.entry(ingredient.name.clone()).or_insert(0.0) += amount_in_kg;
                    
                    // Рассчитываем стоимость
                    total_cost += amount_in_kg * ingredient.cost_per_unit;
                }
            }
        }

        PurchasePlan {
            id: format!("plan_{}_{}", franchise_id, Utc::now().timestamp()),
            franchise_id,
            period,
            planned_dishes,
            required_ingredients,
            estimated_costs: total_cost,
            created_at: Utc::now(),
        }
    }

    /// Получает рецепты по категории
    pub fn get_recipes_by_category(&self, category: &str) -> Vec<&Recipe> {
        self.recipes.values()
            .filter(|recipe| recipe.category == category)
            .collect()
    }

    /// Обновляет рецепт
    pub fn update_recipe(&mut self, id: &str, mut recipe: Recipe) -> Result<(), String> {
        if !self.recipes.contains_key(id) {
            return Err(format!("Рецепт с ID {} не найден", id));
        }

        recipe.id = id.to_string();
        recipe.cost_per_portion = self.calculate_recipe_cost(&recipe);
        recipe.updated_at = Utc::now();
        
        self.recipes.insert(id.to_string(), recipe);
        Ok(())
    }

    /// Удаляет рецепт
    pub fn delete_recipe(&mut self, id: &str) -> Result<(), String> {
        if self.recipes.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Рецепт с ID {} не найден", id))
        }
    }
}

impl IngredientCalculator {
    pub fn new(waste_factor: f64) -> Self {
        Self { waste_factor }
    }

    /// Рассчитывает количество ингредиента с учетом потерь
    pub fn calculate_with_waste(&self, base_amount: f64) -> f64 {
        base_amount * (1.0 + self.waste_factor)
    }
}

impl Default for RecipeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_recipe_creation() {
        let manager = RecipeManager::new();
        let recipes = manager.get_all_recipes();
        
        assert!(!recipes.is_empty());
        assert!(recipes.iter().any(|r| r.name == "Плов с мясом"));
    }

    #[test]
    fn test_purchase_plan_creation() {
        let manager = RecipeManager::new();
        let mut planned_dishes = HashMap::new();
        planned_dishes.insert("plov_001".to_string(), 1000); // 1000 порций плова

        let period = DateRange {
            start: Utc::now(),
            end: Utc::now() + chrono::Duration::days(30),
        };

        let plan = manager.create_purchase_plan(
            "franchise_001".to_string(),
            planned_dishes,
            period,
        );

        // Проверяем, что план создан корректно
        assert_eq!(plan.planned_dishes.get("plov_001"), Some(&1000));
        assert!(plan.required_ingredients.contains_key("Рис"));
        assert!(plan.required_ingredients.contains_key("Говядина"));
        
        // Проверяем расчеты для 1000 порций плова
        // Рис: 200г * 1000 * 1.1 (waste) = 220кг
        let rice_amount = plan.required_ingredients.get("Рис").unwrap();
        assert!(*rice_amount >= 200.0 && *rice_amount <= 250.0); // с учетом потерь
    }

    #[test]
    fn test_cost_calculation() {
        let manager = RecipeManager::new();
        let plov = manager.get_recipe("plov_001").unwrap();
        
        // Проверяем, что стоимость рассчитана
        assert!(plov.cost_per_portion > 0.0);
        
        // Примерная стоимость плова должна быть около 1.5-2 лари
        assert!(plov.cost_per_portion >= 1.0 && plov.cost_per_portion <= 3.0);
    }
}
