use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::repositories::fruit::FruitRepository;
use mazerion_db::models::fruit::Fruit;
use mazerion_db::schemas::fruits::FRUIT_SCHEMA;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(FRUIT_SCHEMA).unwrap();
    conn
}

fn create_test_fruit() -> Fruit {
    Fruit {
        id: 0,
        name: "Raspberry".to_string(),
        scientific_name: Some("Rubus idaeus".to_string()),
        fruit_type: "berry".to_string(),
        origin: Some("Europe".to_string()),
        typical_sugar_content: Some(Decimal::from_str("5.0").unwrap()),
        ph_level: Some(Decimal::from_str("3.5").unwrap()),
        color_contribution: Some("red".to_string()),
        flavor_profile: Some(r#"["tart","sweet","berry"]"#.to_string()),
        aroma_profile: Some(r#"["raspberry","floral"]"#.to_string()),
        best_suited_styles: Some(r#"["melomel","fruit wine"]"#.to_string()),
        usage_notes: Some("Excellent for melomels".to_string()),
        sensory_notes: Some("Bright, tart character".to_string()),
        pounds_per_gallon: Some(Decimal::from_str("3.5").unwrap()),
        preparation_method: Some("Freeze-thaw cycle".to_string()),
        compatible_styles: Some(r#"["mead"]"#.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create_fruit() {
    let conn = setup_test_db();
    let fruit = create_test_fruit();
    let id = FruitRepository::create(&conn, &fruit).unwrap();
    assert!(id > 0);
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let fruit = create_test_fruit();
    let id = FruitRepository::create(&conn, &fruit).unwrap();
    let retrieved = FruitRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, fruit.name);
}

#[test]
fn test_list_fruits() {
    let conn = setup_test_db();
    FruitRepository::create(&conn, &create_test_fruit()).unwrap();
    let mut fruit2 = create_test_fruit();
    fruit2.name = "Strawberry".to_string();
    FruitRepository::create(&conn, &fruit2).unwrap();

    let fruits = FruitRepository::list(&conn, None).unwrap();
    assert_eq!(fruits.len(), 2);
}

#[test]
fn test_search() {
    let conn = setup_test_db();
    FruitRepository::create(&conn, &create_test_fruit()).unwrap();
    let results = FruitRepository::search(&conn, "Rasp", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_get_by_type() {
    let conn = setup_test_db();
    FruitRepository::create(&conn, &create_test_fruit()).unwrap();
    let berries = FruitRepository::get_by_type(&conn, "berry", None).unwrap();
    assert_eq!(berries.len(), 1);
}

#[test]
fn test_update() {
    let conn = setup_test_db();
    let mut fruit = create_test_fruit();
    let id = FruitRepository::create(&conn, &fruit).unwrap();
    fruit.id = id;
    fruit.name = "Updated".to_string();
    FruitRepository::update(&conn, &fruit).unwrap();
    let retrieved = FruitRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup_test_db();
    let fruit = create_test_fruit();
    let id = FruitRepository::create(&conn, &fruit).unwrap();
    FruitRepository::delete(&conn, id).unwrap();
    assert!(FruitRepository::get_by_id(&conn, id).is_err());
}

#[test]
fn test_count() {
    let conn = setup_test_db();
    assert_eq!(FruitRepository::count(&conn).unwrap(), 0);
    FruitRepository::create(&conn, &create_test_fruit()).unwrap();
    assert_eq!(FruitRepository::count(&conn).unwrap(), 1);
}

#[test]
fn test_invalid_empty_name() {
    let conn = setup_test_db();
    let mut fruit = create_test_fruit();
    fruit.name = "".to_string();
    assert!(FruitRepository::create(&conn, &fruit).is_err());
}

#[test]
fn test_invalid_type() {
    let conn = setup_test_db();
    let mut fruit = create_test_fruit();
    fruit.fruit_type = "invalid".to_string();
    assert!(FruitRepository::create(&conn, &fruit).is_err());
}