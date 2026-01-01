use rusqlite::Connection;
use mazerion_db::repositories::extract::ExtractRepository;
use mazerion_db::models::Extract;
use mazerion_db::schemas;
use rust_decimal::Decimal;

fn setup_test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS extracts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            extract_type TEXT NOT NULL,
            manufacturer TEXT,
            flavor_profile TEXT,
            aroma_profile TEXT,
            best_suited_styles TEXT,
            usage_notes TEXT,
            typical_dosage_oz_per_gallon NUMERIC,
            alcohol_based INTEGER,
            compatible_styles TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn create_test_extract() -> Extract {
    Extract {
        id: 0,
        name: "Vanilla Extract".to_string(),
        extract_type: "vanilla".to_string(),
        manufacturer: Some("McCormick".to_string()),
        flavor_profile: Some("Sweet, creamy vanilla".to_string()),
        aroma_profile: Some("Strong vanilla bean".to_string()),
        best_suited_styles: Some("Meads, stouts, porters".to_string()),
        usage_notes: Some("Add at end of fermentation".to_string()),
        typical_dosage_oz_per_gallon: Some(Decimal::new(25, 2)),
        alcohol_based: true,
        compatible_styles: Some("Sweet styles".to_string()),
        created_at: String::new(),
        updated_at: String::new(),
        concentration: (),
        notes: (),
    }
}

#[test]
fn test_create() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let extract = create_test_extract();
    let id = ExtractRepository::create(&conn, &extract)?;
    assert!(id > 0);
    let retrieved = ExtractRepository::get_by_id(&conn, id)?;
    assert_eq!(retrieved.name, extract.name);
    Ok(())
}

#[test]
fn test_get_by_id() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let extract = create_test_extract();
    let id = ExtractRepository::create(&conn, &extract)?;
    let retrieved = ExtractRepository::get_by_id(&conn, id)?;
    assert_eq!(retrieved.extract_type, "vanilla");
    Ok(())
}

#[test]
fn test_list() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let extract = create_test_extract();
    ExtractRepository::create(&conn, &extract)?;
    let results = ExtractRepository::list(&conn, None)?;
    assert_eq!(results.len(), 1);
    Ok(())
}

#[test]
fn test_search() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let extract = create_test_extract();
    ExtractRepository::create(&conn, &extract)?;
    let results = ExtractRepository::search(&conn, "Vanilla", None)?;
    assert_eq!(results.len(), 1);
    Ok(())
}

#[test]
fn test_update() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let mut extract = create_test_extract();
    let id = ExtractRepository::create(&conn, &extract)?;
    extract.id = id;
    extract.name = "Almond Extract".to_string();
    extract.extract_type = "almond".to_string();
    ExtractRepository::update(&conn, &extract)?;
    let updated = ExtractRepository::get_by_id(&conn, id)?;
    assert_eq!(updated.name, "Almond Extract");
    Ok(())
}

#[test]
fn test_delete() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let extract = create_test_extract();
    let id = ExtractRepository::create(&conn, &extract)?;
    ExtractRepository::delete(&conn, id)?;
    let result = ExtractRepository::get_by_id(&conn, id);
    assert!(result.is_err());
    Ok(())
}