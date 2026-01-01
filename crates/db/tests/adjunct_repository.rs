use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::adjunct::Adjunct;
use mazerion_db::repositories::adjunct::AdjunctRepository;
use mazerion_db::schemas::adjuncts::ADJUNCT_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(ADJUNCT_SCHEMA).unwrap();
    conn
}

fn create_test() -> Adjunct {
    Adjunct {
        id: 0, name: "Corn Sugar".to_string(), adjunct_type: "sugar".to_string(),
        manufacturer: None, fermentability: Some(Decimal::from_str("100.0").unwrap()),
        flavor_profile: None, best_suited_styles: None, usage_notes: None,
        typical_percentage: Some(Decimal::from_str("10.0").unwrap()),
        compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(AdjunctRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup(); let id = AdjunctRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(AdjunctRepository::get_by_id(&conn, id).unwrap().name, "Corn Sugar");
}

#[test]
fn test_list() { let conn = setup(); AdjunctRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(AdjunctRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); AdjunctRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(AdjunctRepository::search(&conn, "Corn", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut adj = create_test();
    let id = AdjunctRepository::create(&conn, &adj).unwrap();
    adj.id = id; adj.name = "Updated".to_string();
    AdjunctRepository::update(&conn, &adj).unwrap();
    assert_eq!(AdjunctRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = AdjunctRepository::create(&conn, &create_test()).unwrap();
    AdjunctRepository::delete(&conn, id).unwrap();
    assert!(AdjunctRepository::get_by_id(&conn, id).is_err());
}