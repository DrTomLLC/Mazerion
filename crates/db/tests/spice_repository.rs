use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;
use mazerion_db::models::spice::Spice;
use mazerion_db::repositories::spice::SpiceRepository;
use mazerion_db::schemas::spices::SPICE_SCHEMA;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(SPICE_SCHEMA).unwrap();
    conn
}

fn create_test() -> Spice {
    Spice {
        id: 0, name: "Cinnamon".to_string(), scientific_name: Some("Cinnamomum verum".to_string()),
        spice_type: "warming".to_string(), origin: Some("Sri Lanka".to_string()),
        heat_level: Some(Decimal::from_str("0.5").unwrap()), flavor_profile: None, aroma_profile: None,
        best_suited_styles: None, usage_notes: None, sensory_notes: None,
        typical_dosage_oz_per_gallon: Some(Decimal::from_str("0.5").unwrap()),
        preparation_method: None, compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(), updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create() { let conn = setup(); assert!(SpiceRepository::create(&conn, &create_test()).unwrap() > 0); }

#[test]
fn test_get_by_id() {
    let conn = setup(); let id = SpiceRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SpiceRepository::get_by_id(&conn, id).unwrap().name, "Cinnamon");
}

#[test]
fn test_list() { let conn = setup(); SpiceRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SpiceRepository::list(&conn, None).unwrap().len(), 1); }

#[test]
fn test_search() {
    let conn = setup(); SpiceRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SpiceRepository::search(&conn, "Cinn", None).unwrap().len(), 1);
}

#[test]
fn test_get_by_type() {
    let conn = setup(); SpiceRepository::create(&conn, &create_test()).unwrap();
    assert_eq!(SpiceRepository::get_by_type(&conn, "warming", None).unwrap().len(), 1);
}

#[test]
fn test_update() {
    let conn = setup(); let mut sp = create_test();
    let id = SpiceRepository::create(&conn, &sp).unwrap();
    sp.id = id; sp.name = "Updated".to_string();
    SpiceRepository::update(&conn, &sp).unwrap();
    assert_eq!(SpiceRepository::get_by_id(&conn, id).unwrap().name, "Updated");
}

#[test]
fn test_delete() {
    let conn = setup(); let id = SpiceRepository::create(&conn, &create_test()).unwrap();
    SpiceRepository::delete(&conn, id).unwrap();
    assert!(SpiceRepository::get_by_id(&conn, id).is_err());
}

#[test]
fn test_invalid_type() {
    let conn = setup(); let mut sp = create_test(); sp.spice_type = "invalid".to_string();
    assert!(SpiceRepository::create(&conn, &sp).is_err());
}