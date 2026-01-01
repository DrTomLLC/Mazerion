use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::extract::Extract;
use mazerion_db::repositories::extract::ExtractRepository;
use mazerion_db::schemas::extracts::EXTRACT_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(EXTRACT_SCHEMA).unwrap();
    conn
}

fn create_test() -> Extract {
    Extract {
        id: 0, name: "Vanilla Extract".to_string(), extract_type: "vanilla".to_string(),
        manufacturer: Some("Nielsen-Massey".to_string()), flavor_profile: None, aroma_profile: None,
        best_suited_styles: None, usage_notes: None,
        typical_dosage_oz_per_gallon: Some(Decimal::from_str("0.5").unwrap()),
        alcohol_based: true, compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(ExtractRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup(); let id = ExtractRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(ExtractRepository::get_by_id(&conn, id).unwrap().name, "Vanilla Extract");
}

#[test]
fn test_list() { let conn = setup(); ExtractRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(ExtractRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); ExtractRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(ExtractRepository::search(&conn, "Vanilla", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut ext = create_test();
    let id = ExtractRepository::create(&conn, &ext).unwrap();
    ext.id = id; ext.name = "Updated".to_string();
    ExtractRepository::update(&conn, &ext).unwrap();
    assert_eq!(ExtractRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = ExtractRepository::create(&conn, &create_test()).unwrap();
    ExtractRepository::delete(&conn, id).unwrap();
    assert!(ExtractRepository::get_by_id(&conn, id).is_err());
}