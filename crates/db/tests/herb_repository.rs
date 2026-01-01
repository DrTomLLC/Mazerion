use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::herb::Herb;
use mazerion_db::repositories::herb::HerbRepository;
use mazerion_db::schemas::herbs::HERB_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(HERB_SCHEMA).unwrap();
    conn
}

fn create_test() -> Herb {
    Herb {
        id: 0, name: "Lavender".to_string(), scientific_name: Some("Lavandula angustifolia".to_string()),
        herb_type: "aromatic".to_string(), origin: Some("Mediterranean".to_string()),
        flavor_profile: None, aroma_profile: None, best_suited_styles: None,
        usage_notes: None, sensory_notes: None,
        typical_dosage_oz_per_gallon: Some(Decimal::from_str("0.25").unwrap()),
        preparation_method: None, compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(HerbRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup(); let id = HerbRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(HerbRepository::get_by_id(&conn, id).unwrap().name, "Lavender");
}

#[test]
fn test_list() { let conn = setup(); HerbRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(HerbRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); HerbRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(HerbRepository::search(&conn, "Lav", None).unwrap().len(), 1);
}

#[test]
fn test_get_by_type() {
    let conn = setup(); HerbRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(HerbRepository::get_by_type(&conn, "aromatic", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut herb = create_test();
    let id = HerbRepository::create(&conn, &herb).unwrap();
    herb.id = id; herb.name = "Updated".to_string();
    HerbRepository::update(&conn, &herb).unwrap();
    assert_eq!(HerbRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = HerbRepository::create(&conn, &create_test()).unwrap();
    HerbRepository::delete(&conn, id).unwrap();
    assert!(HerbRepository::get_by_id(&conn, id).is_err());
}