use crate::*;

#[test]
fn menu_item_details_and_voting_flow() {
    let ingredients = vec![
        Ingredient { name: "A".to_string(), amount_grams: 10.0, calories: 30.0 },
        Ingredient { name: "B".to_string(), amount_grams: 20.0, calories: 70.0 },
    ];
    let mut item = MenuItem::new_with_details(
        "Dish".to_string(),
        "Desc".to_string(),
        9.99,
        5,
        7,
        15,
        ingredients,
        "Sugg".to_string(),
        7,
    );
    assert_eq!(item.total_calories, 100.0);
    assert!(matches!(item.status, MenuItemStatus::Proposed));
    item.start_voting();
    assert!(matches!(item.status, MenuItemStatus::Voting));
    item.vote(2.5, true).expect("vote allowed while voting active");
    assert_eq!(item.votes_for, 2.5);
}


