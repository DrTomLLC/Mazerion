use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::syrup::Syrup;
use mazerion_db::repositories::syrup::SyrupRepository;
use mazerion_db::schemas::syrups::SYRUP_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(SYRUP_SCHEMA).unwrap();
    conn
}

fn create_test() -> Syrup {
    Syrup {
        id: 0, name: "Maple Syrup".to_string(), syrup_type: "maple".to_string(),
        manufacturer: None, sugar_content: Some(Decimal::from_str("66.0").unwrap()),
        flavor_profile: None, best_suited_styles: None, usage_notes: None,
        typical_dosage_oz_per_gallon: Some(Decimal::from_str("2.0").unwrap()),
        compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(SyrupRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup(); let id = SyrupRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SyrupRepository::get_by_id(&conn, id).unwrap().name, "Maple Syrup");
}

#[test]
fn test_list() { let conn = setup(); SyrupRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SyrupRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); SyrupRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SyrupRepository::search(&conn, "Maple", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut syr = create_test();
    let id = SyrupRepository::create(&conn, &syr).unwrap();
    syr.id = id; syr.name = "Updated".to_string();
    SyrupRepository::update(&conn, &syr).unwrap();
    assert_eq!(SyrupRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = SyrupRepository::create(&conn, &create_test()).unwrap();
    SyrupRepository::delete(&conn, id).unwrap();
    assert!(SyrupRepository::get_by_id(&conn, id).is_err());
}