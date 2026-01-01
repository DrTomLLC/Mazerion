use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;

use mazerion_db::models::malt::Malt;
use mazerion_db::repositories::malt::MaltRepository;
use mazerion_db::schemas::malts::MALT_SCHEMA;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(MALT_SCHEMA).unwrap();
    conn
}

fn create_test_malt() -> Malt {
    Malt {
        id: 0,
        name: "Pale Malt 2-Row".to_string(),
        maltster: "Briess".to_string(),
        origin: "USA".to_string(),
        grain_type: "base".to_string(),
        color_lovibond: Some(Decimal::from_str("2.0").unwrap()),
        max_percentage: Some(Decimal::from_str("100.0").unwrap()),
        extract_potential: Some(Decimal::from_str("1.037").unwrap()),
        diastatic_power: Some(Decimal::from_str("140.0").unwrap()),
        moisture_content: Some(Decimal::from_str("4.0").unwrap()),
        protein_content: Some(Decimal::from_str("11.5").unwrap()),
        flavor_profile: Some(r#"["malty","sweet","clean"]"#.to_string()),
        aroma_profile: Some(r#"["grain","bread"]"#.to_string()),
        typical_usage: Some("Base malt for all styles".to_string()),
        substitutes: Some(r#"["Maris Otter","Pilsner"]"#.to_string()),
        best_suited_styles: Some(r#"["Ale","Lager","IPA"]"#.to_string()),
        usage_notes: Some("Workhorse base malt".to_string()),
        sensory_notes: Some("Clean, sweet malt character".to_string()),
        requires_mashing: true,
        compatible_styles: Some(r#"["American Ale"]"#.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create_malt() {
    let conn = setup_test_db();
    let malt = create_test_malt();
    let id = MaltRepository::create(&conn, &malt).unwrap();
    assert!(id > 0);
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let malt = create_test_malt();
    let id = MaltRepository::create(&conn, &malt).unwrap();
    let retrieved = MaltRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, malt.name);
    assert_eq!(retrieved.maltster, malt.maltster);
    assert_eq!(retrieved.color_lovibond, malt.color_lovibond);
}

#[test]
fn test_list_malts() {
    let conn = setup_test_db();
    let mut malt1 = create_test_malt();
    let mut malt2 = create_test_malt();
    malt2.name = "Caramel 60L".to_string();

    MaltRepository::create(&conn, &malt1).unwrap();
    MaltRepository::create(&conn, &malt2).unwrap();

    let malts = MaltRepository::list(&conn, None).unwrap();
    assert_eq!(malts.len(), 2);
}

#[test]
fn test_list_with_limit() {
    let conn = setup_test_db();

    for i in 0..5 {
        let mut malt = create_test_malt();
        malt.name = format!("Malt {}", i);
        MaltRepository::create(&conn, &malt).unwrap();
    }

    let malts = MaltRepository::list(&conn, Some(3)).unwrap();
    assert_eq!(malts.len(), 3);
}

#[test]
fn test_search_by_name() {
    let conn = setup_test_db();
    let malt = create_test_malt();
    MaltRepository::create(&conn, &malt).unwrap();

    let results = MaltRepository::search(&conn, "Pale", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_by_maltster() {
    let conn = setup_test_db();
    let malt = create_test_malt();
    MaltRepository::create(&conn, &malt).unwrap();

    let results = MaltRepository::search(&conn, "Briess", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_empty_query() {
    let conn = setup_test_db();
    let malt = create_test_malt();
    MaltRepository::create(&conn, &malt).unwrap();

    let results = MaltRepository::search(&conn, "", None).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_get_by_type() {
    let conn = setup_test_db();

    let mut base = create_test_malt();
    base.grain_type = "base".to_string();

    let mut specialty = create_test_malt();
    specialty.name = "Crystal 60L".to_string();
    specialty.grain_type = "specialty".to_string();

    MaltRepository::create(&conn, &base).unwrap();
    MaltRepository::create(&conn, &specialty).unwrap();

    let base_malts = MaltRepository::get_by_type(&conn, "base", None).unwrap();
    assert_eq!(base_malts.len(), 1);
}

#[test]
fn test_get_by_maltster() {
    let conn = setup_test_db();

    let mut briess1 = create_test_malt();
    let mut briess2 = create_test_malt();
    briess2.name = "Munich Malt".to_string();

    let mut weyermann = create_test_malt();
    weyermann.name = "Pilsner".to_string();
    weyermann.maltster = "Weyermann".to_string();

    MaltRepository::create(&conn, &briess1).unwrap();
    MaltRepository::create(&conn, &briess2).unwrap();
    MaltRepository::create(&conn, &weyermann).unwrap();

    let briess_malts = MaltRepository::get_by_maltster(&conn, "Briess", None).unwrap();
    assert_eq!(briess_malts.len(), 2);
}

#[test]
fn test_update_malt() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();

    let id = MaltRepository::create(&conn, &malt).unwrap();

    malt.id = id;
    malt.name = "Updated Malt".to_string();
    malt.color_lovibond = Some(Decimal::from_str("5.0").unwrap());

    MaltRepository::update(&conn, &malt).unwrap();

    let retrieved = MaltRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated Malt");
    assert_eq!(retrieved.color_lovibond, Some(Decimal::from_str("5.0").unwrap()));
}

#[test]
fn test_delete_malt() {
    let conn = setup_test_db();
    let malt = create_test_malt();

    let id = MaltRepository::create(&conn, &malt).unwrap();
    MaltRepository::delete(&conn, id).unwrap();

    let result = MaltRepository::get_by_id(&conn, id);
    assert!(result.is_err());
}

#[test]
fn test_count() {
    let conn = setup_test_db();
    assert_eq!(MaltRepository::count(&conn).unwrap(), 0);

    MaltRepository::create(&conn, &create_test_malt()).unwrap();
    assert_eq!(MaltRepository::count(&conn).unwrap(), 1);
}

#[test]
fn test_count_by_type() {
    let conn = setup_test_db();

    let mut base = create_test_malt();
    base.grain_type = "base".to_string();

    let mut specialty = create_test_malt();
    specialty.name = "Specialty".to_string();
    specialty.grain_type = "specialty".to_string();

    MaltRepository::create(&conn, &base).unwrap();
    MaltRepository::create(&conn, &specialty).unwrap();
    MaltRepository::create(&conn, &specialty).unwrap();

    assert_eq!(MaltRepository::count_by_type(&conn, "base").unwrap(), 1);
    assert_eq!(MaltRepository::count_by_type(&conn, "specialty").unwrap(), 2);
}

#[test]
fn test_count_by_maltster() {
    let conn = setup_test_db();

    let mut briess = create_test_malt();
    briess.maltster = "Briess".to_string();

    let mut weyermann = create_test_malt();
    weyermann.name = "Wey Malt".to_string();
    weyermann.maltster = "Weyermann".to_string();

    MaltRepository::create(&conn, &briess).unwrap();
    MaltRepository::create(&conn, &weyermann).unwrap();
    MaltRepository::create(&conn, &weyermann).unwrap();

    assert_eq!(MaltRepository::count_by_maltster(&conn, "Briess").unwrap(), 1);
    assert_eq!(MaltRepository::count_by_maltster(&conn, "Weyermann").unwrap(), 2);
}

#[test]
fn test_decimal_precision() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();

    malt.color_lovibond = Some(Decimal::from_str("2.567890123456").unwrap());
    malt.extract_potential = Some(Decimal::from_str("1.037123456789").unwrap());

    let id = MaltRepository::create(&conn, &malt).unwrap();
    let retrieved = MaltRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.color_lovibond, Some(Decimal::from_str("2.567890123456").unwrap()));
    assert_eq!(retrieved.extract_potential, Some(Decimal::from_str("1.037123456789").unwrap()));
}

#[test]
fn test_get_nonexistent_id() {
    let conn = setup_test_db();
    let result = MaltRepository::get_by_id(&conn, 99999);
    assert!(result.is_err());
}

#[test]
fn test_update_nonexistent() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.id = 99999;

    let result = MaltRepository::update(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent() {
    let conn = setup_test_db();
    let result = MaltRepository::delete(&conn, 99999);
    assert!(result.is_err());
}

#[test]
fn test_create_empty_name() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.name = "".to_string();

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_empty_maltster() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.maltster = "".to_string();

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_empty_origin() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.origin = "".to_string();

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_type() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.grain_type = "invalid".to_string();

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_color() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.color_lovibond = Some(Decimal::from(800));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_negative_color() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.color_lovibond = Some(Decimal::from(-5));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_max_percentage() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.max_percentage = Some(Decimal::from(110));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_extract() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.extract_potential = Some(Decimal::from_str("0.900").unwrap());

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_diastatic_power() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.diastatic_power = Some(Decimal::from(250));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_moisture() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.moisture_content = Some(Decimal::from(15));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_protein() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.protein_content = Some(Decimal::from(25));

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_name_too_long() {
    let conn = setup_test_db();
    let mut malt = create_test_malt();
    malt.name = "a".repeat(101);

    let result = MaltRepository::create(&conn, &malt);
    assert!(result.is_err());
}

#[test]
fn test_all_grain_types() {
    let conn = setup_test_db();
    let types = vec!["base", "specialty", "adjunct"];

    for (i, grain_type) in types.iter().enumerate() {
        let mut malt = create_test_malt();
        malt.name = format!("Malt {}", i);
        malt.grain_type = grain_type.to_string();
        MaltRepository::create(&conn, &malt).unwrap();
    }

    assert_eq!(MaltRepository::count(&conn).unwrap(), 3);
}