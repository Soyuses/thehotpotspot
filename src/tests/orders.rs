use crate::*;

#[test]
fn order_create_confirm_cancel() {
    let mut order = Order::new(
        "wallet1".to_string(),
        vec![OrderItem { menu_item_id: "m1".to_string(), quantity: 2 }],
        20,
    );
    assert!(order.id.starts_with("ORDER_"));
    assert!(matches!(order.status, OrderStatus::Pending));

    order.confirm(3.0);
    assert!(matches!(order.status, OrderStatus::Confirmed));
    assert_eq!(order.tokens_issued, 3.0);
    assert!(order.confirmed_timestamp.is_some());

    // Cancel a new order
    let mut order2 = Order::new("w2".to_string(), vec![], 10);
    order2.cancel("No items".to_string());
    assert!(matches!(order2.status, OrderStatus::Cancelled));
    assert_eq!(order2.cancellation_reason.as_deref(), Some("No items"));
}

#[test]
fn blockchain_create_and_confirm_order_updates_balances_and_availability() {
    let main_owner = "Owner".to_string();
    let mut bc = Blockchain::new(main_owner.clone());

    // Add a detailed menu item owned by main owner
    let ingredients = vec![Ingredient { name: "I".to_string(), amount_grams: 1.0, calories: 1.0 }];
    bc.add_menu_item_with_details(
        "Item1".to_string(),
        "D".to_string(),
        10.0,
        3,
        5,
        10,
        ingredients,
        main_owner.clone(),
    )
    .unwrap();

    let menu_id = bc.menu_items[0].id.clone();
    let order = bc
        .create_order(
            "Customer1".to_string(),
            vec![OrderItem { menu_item_id: menu_id.clone(), quantity: 2 }],
            30,
        )
        .expect("order should be created");
    assert_eq!(order.total_amount, 20.0);

    // Confirm order
    bc.confirm_order(order.id.clone()).expect("confirm should succeed");

    // Tokens issued to customer
    let holder = bc.token_holders.get("Customer1").expect("customer holder exists");
    assert_eq!(holder.security_tokens, 20.0);
    assert_eq!(holder.utility_tokens, 2.0);

    // Availability reduced
    let item = bc.menu_items.iter().find(|m| m.id == menu_id).unwrap();
    assert_eq!(item.availability, 1);
}


