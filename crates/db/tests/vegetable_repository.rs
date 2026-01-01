use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::vegetable::Vegetable;
use mazerion_db::repositories::vegetable::VegetableRepository;
use mazerion_db::schemas::vegetables::VEGETABLE_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(VEGETABLE_SCHEMA).unwrap();
    conn
}

fn create_test() -> Vegetable {
    Vegetable {
        id: 0, name: "Pumpkin".to_string(), scientific_name: Some("Cucurbita pepo".to_string()),
        vegetable_type: "gourd".to_string(), origin: Some("Americas".to_string()),
        typical_sugar_content: Some(Decimal::from_str("7.0").unwrap()),
        ph_level: Some(Decimal::from_str("6.0").unwrap()), flavor_profile: None, aroma_profile: None,
        best_suited_styles: None, usage_notes: None, sensory_notes: None,
        pounds_per_gallon: Some(Decimal::from_str("5.0").unwrap()),
        preparation_method: None, compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(VegetableRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup();
    let id = VegetableRepository::create(&conn, &create_test()).unwrap();
    let retrieved = VegetableRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Pumpkin");
}

#[test]
fn test_list() { let conn = setup(); VegetableRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(VegetableRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); VegetableRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(VegetableRepository::search(&conn, "Pump", None).unwrap().len(), 1);
}

#[test]
fn test_get_by_type() {
    let conn = setup(); VegetableRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(VegetableRepository::get_by_type(&conn, "gourd", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut veg = create_test();
    let id = VegetableRepository::create(&conn, &veg).unwrap();
    veg.id = id; veg.name = "Updated".to_string();
    VegetableRepository::update(&conn, &veg).unwrap();
    assert_eq!(VegetableRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = VegetableRepository::create(&conn, &create_test()).unwrap();
    VegetableRepository::delete(&conn, id).unwrap();
    assert!(VegetableRepository::get_by_id(&conn, id).is_err());
}

#[test]
fn test_count() { let conn = setup(); assert_eq!(VegetableRepository::count(&conn).unwrap(), 0); }

#[test]
fn test_invalid_empty_name() {
    let conn = setup(); let mut veg = create_test(); veg.name = "".to_string();
    assert!(VegetableRepository::create(&conn, &veg).is_err());
}

#[test]
fn test_invalid_type() {
    let conn = setup(); let mut veg = create_test(); veg.vegetable_type = "invalid".to_string();
    assert!(VegetableRepository::create(&conn, &veg).is_err());
}