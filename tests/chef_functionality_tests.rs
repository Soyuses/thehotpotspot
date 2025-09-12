use blockchain_project::chef_arm::{
    ChefARM, ChefARMManager, ChefOrder, OrderStatus, OrderItem, ChefStatistics
};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_chef_arm_creation() {
    let chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    assert_eq!(chef_arm.chef_id, "chef_001");
    assert_eq!(chef_arm.food_truck_id, "truck_001");
    assert!(!chef_arm.is_active);
    assert_eq!(chef_arm.current_orders.len(), 0);
    assert_eq!(chef_arm.completed_orders_today, 0);
    assert_eq!(chef_arm.total_orders_processed, 0);
    assert_eq!(chef_arm.chef_rating, 5.0);
}

#[test]
fn test_chef_activation() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    assert!(!chef_arm.is_active);
    
    chef_arm.activate();
    assert!(chef_arm.is_active);
    
    chef_arm.deactivate();
    assert!(!chef_arm.is_active);
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
    assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::Pending));

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
    assert_eq!(chef_arm.total_orders_processed, 1);
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
    
    // Регистрация второго повара
    assert!(manager.register_chef("chef_002".to_string(), "truck_002".to_string()).is_ok());
    assert_eq!(manager.get_all_chefs().len(), 2);
}

#[test]
fn test_order_cancellation() {
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

    // Отмена заказа
    assert!(chef_arm.cancel_order("order_001", "Нет ингредиентов".to_string()).is_ok());
    assert_eq!(chef_arm.get_order_status("order_001"), Some(OrderStatus::Cancelled));
    assert_eq!(chef_arm.current_orders.len(), 0);
}

#[test]
fn test_chef_statistics() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    chef_arm.activate();

    // Создаем несколько заказов
    for i in 1..=5 {
        let order = ChefOrder {
            order_id: format!("order_{:03}", i),
            customer_id: format!("customer_{:03}", i),
            food_truck_id: "truck_001".to_string(),
            items: vec![OrderItem {
                item_id: format!("item_{:03}", i),
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

        chef_arm.receive_order(order).unwrap();
        chef_arm.confirm_order(&format!("order_{:03}", i), 15, None).unwrap();
        chef_arm.start_cooking(&format!("order_{:03}", i)).unwrap();
        chef_arm.mark_ready(&format!("order_{:03}", i)).unwrap();
        chef_arm.complete_order(&format!("order_{:03}", i)).unwrap();
    }

    let stats = chef_arm.get_statistics();
    assert_eq!(stats.chef_id, "chef_001");
    assert_eq!(stats.total_orders, 5);
    assert_eq!(stats.completed_today, 5);
    assert_eq!(stats.active_orders, 0);
    assert!(stats.average_time > 0.0);
    assert_eq!(stats.rating, 5.0);
}

#[test]
fn test_chef_specializations() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    
    assert_eq!(chef_arm.specializations.len(), 0);
    
    chef_arm.add_specialization("Бургеры".to_string());
    chef_arm.add_specialization("Пицца".to_string());
    chef_arm.add_specialization("Бургеры".to_string()); // Дубликат
    
    assert_eq!(chef_arm.specializations.len(), 2);
    assert!(chef_arm.specializations.contains(&"Бургеры".to_string()));
    assert!(chef_arm.specializations.contains(&"Пицца".to_string()));
}

#[test]
fn test_chef_working_hours() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    
    // Устанавливаем рабочее время
    chef_arm.set_working_hours(9, 18); // 9:00 - 18:00
    assert_eq!(chef_arm.working_hours, (9, 18));
    
    // Добавляем перерыв
    chef_arm.add_break(13, 14); // 13:00 - 14:00
    assert_eq!(chef_arm.break_schedule.len(), 1);
    assert_eq!(chef_arm.break_schedule[0], (13, 14));
}

#[test]
fn test_chef_rating_update() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    
    assert_eq!(chef_arm.chef_rating, 5.0);
    
    // Обновляем рейтинг
    chef_arm.update_rating(4.5);
    assert_eq!(chef_arm.chef_rating, 4.5);
    
    // Попытка установить недопустимый рейтинг
    chef_arm.update_rating(6.0); // > 5.0
    assert_eq!(chef_arm.chef_rating, 4.5); // Не изменился
    
    chef_arm.update_rating(0.5); // < 1.0
    assert_eq!(chef_arm.chef_rating, 4.5); // Не изменился
}

#[test]
fn test_multiple_orders_management() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    chef_arm.activate();

    // Создаем несколько заказов
    for i in 1..=3 {
        let order = ChefOrder {
            order_id: format!("order_{:03}", i),
            customer_id: format!("customer_{:03}", i),
            food_truck_id: "truck_001".to_string(),
            items: vec![OrderItem {
                item_id: format!("item_{:03}", i),
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

        chef_arm.receive_order(order).unwrap();
    }

    assert_eq!(chef_arm.current_orders.len(), 3);
    assert_eq!(chef_arm.get_active_orders().len(), 3);

    // Подтверждаем все заказы
    for i in 1..=3 {
        chef_arm.confirm_order(&format!("order_{:03}", i), 15, None).unwrap();
    }

    // Проверяем заказы по статусу
    let confirmed_orders = chef_arm.get_orders_by_status(OrderStatus::Confirmed);
    assert_eq!(confirmed_orders.len(), 3);

    // Завершаем один заказ
    chef_arm.start_cooking("order_001").unwrap();
    chef_arm.mark_ready("order_001").unwrap();
    chef_arm.complete_order("order_001").unwrap();

    assert_eq!(chef_arm.current_orders.len(), 2);
    assert_eq!(chef_arm.completed_orders_today, 1);
}

#[test]
fn test_chef_inactive_state() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    // Не активируем повара

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

    // Попытка получить заказ неактивным поваром
    assert!(chef_arm.receive_order(order).is_err());
    assert_eq!(chef_arm.current_orders.len(), 0);
}

#[test]
fn test_chef_max_orders_limit() {
    let mut chef_arm = ChefARM::new("chef_001".to_string(), "truck_001".to_string());
    chef_arm.activate();

    // Создаем максимальное количество заказов (10)
    for i in 1..=10 {
        let order = ChefOrder {
            order_id: format!("order_{:03}", i),
            customer_id: format!("customer_{:03}", i),
            food_truck_id: "truck_001".to_string(),
            items: vec![OrderItem {
                item_id: format!("item_{:03}", i),
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

        assert!(chef_arm.receive_order(order).is_ok());
    }

    assert_eq!(chef_arm.current_orders.len(), 10);

    // Попытка создать 11-й заказ
    let order_11 = ChefOrder {
        order_id: "order_011".to_string(),
        customer_id: "customer_011".to_string(),
        food_truck_id: "truck_001".to_string(),
        items: vec![OrderItem {
            item_id: "item_011".to_string(),
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

    assert!(chef_arm.receive_order(order_11).is_err());
    assert_eq!(chef_arm.current_orders.len(), 10); // Остается 10
}

