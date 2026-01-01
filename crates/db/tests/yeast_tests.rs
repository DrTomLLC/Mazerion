use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;

// Import from the db crate
use mazerion_db::models::yeast::YeastStrain;
use mazerion_db::repositories::yeast::YeastRepository;
use mazerion_db::schemas::yeasts::YEAST_SCHEMA;
/// Helper: Create in-memory database with yeast schema
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();

    // Create yeasts table (from schemas/yeasts.rs)
    conn.execute(
        "CREATE TABLE yeasts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                laboratory TEXT NOT NULL,
                product_code TEXT NOT NULL,
                yeast_type TEXT NOT NULL,
                alcohol_tolerance TEXT,
                temperature_range_min TEXT,
                temperature_range_max TEXT,
                attenuation TEXT,
                flocculation TEXT,
                nutrient_requirements TEXT,
                flavor_profile TEXT,
                aroma_profile TEXT,
                best_suited_styles TEXT,
                usage_notes TEXT,
                lag_time_hours TEXT,
                fermentation_duration_days TEXT,
                sensory_notes TEXT,
                requires_rehydration INTEGER NOT NULL DEFAULT 0,
                compatible_ingredients TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
        [],
    ).unwrap();

    // Add indexes for performance
    conn.execute(
        "CREATE INDEX idx_yeasts_type ON yeasts(yeast_type)",
        [],
    ).unwrap();

    conn.execute(
        "CREATE INDEX idx_yeasts_lab ON yeasts(laboratory)",
        [],
    ).unwrap();

    conn
}

/// Helper: Create valid test yeast
fn create_test_yeast() -> YeastStrain {
    YeastStrain {
        id: 0, // Will be set by database
        name: "Lalvin 71B-1122".to_string(),
        laboratory: "Lalvin".to_string(),
        product_code: "71B".to_string(),
        yeast_type: "wine".to_string(),
        alcohol_tolerance: Some(Decimal::from_str("14.0").unwrap()),
        temperature_range_min: Some(Decimal::from_str("59.0").unwrap()),
        temperature_range_max: Some(Decimal::from_str("86.0").unwrap()),
        attenuation: Some(Decimal::from_str("80.0").unwrap()),
        flocculation: Some("medium".to_string()),
        nutrient_requirements: Some("moderate".to_string()),
        flavor_profile: Some(r#"["fruity","floral","tropical"]"#.to_string()),
        aroma_profile: Some(r#"["stone fruit","citrus"]"#.to_string()),
        best_suited_styles: Some(r#"["mead","wine","fruit wine"]"#.to_string()),
        usage_notes: Some("Excellent for melomels, reduces harsh notes".to_string()),
        lag_time_hours: Some(Decimal::from_str("12.0").unwrap()),
        fermentation_duration_days: Some(Decimal::from_str("14.0").unwrap()),
        sensory_notes: Some("Enhances fruit character, produces soft mouthfeel".to_string()),
        requires_rehydration: true,
        compatible_ingredients: Some(r#"["raspberry","strawberry","cherry"]"#.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

// ===== NORMAL OPERATION TESTS =====

#[test]
fn test_create_yeast() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    assert!(id > 0, "Generated ID should be positive");
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, yeast.name);
    assert_eq!(retrieved.laboratory, yeast.laboratory);
    assert_eq!(retrieved.product_code, yeast.product_code);
    assert_eq!(retrieved.yeast_type, yeast.yeast_type);
    assert_eq!(retrieved.alcohol_tolerance, yeast.alcohol_tolerance);
}

#[test]
fn test_list_yeasts() {
    let conn = setup_test_db();
    let yeast1 = create_test_yeast();
    let mut yeast2 = create_test_yeast();
    yeast2.name = "Wyeast 1056".to_string();
    yeast2.laboratory = "Wyeast".to_string();
    yeast2.product_code = "1056".to_string();
    yeast2.yeast_type = "beer".to_string();

    YeastRepository::create(&conn, &yeast1).unwrap();
    YeastRepository::create(&conn, &yeast2).unwrap();

    let yeasts = YeastRepository::list(&conn, None).unwrap();
    assert_eq!(yeasts.len(), 2);
}

#[test]
fn test_list_with_limit() {
    let conn = setup_test_db();

    // Create 5 yeasts
    for i in 0..5 {
        let mut yeast = create_test_yeast();
        yeast.name = format!("Yeast {}", i);
        YeastRepository::create(&conn, &yeast).unwrap();
    }

    let yeasts = YeastRepository::list(&conn, Some(3)).unwrap();
    assert_eq!(yeasts.len(), 3, "Should respect limit");
}

#[test]
fn test_search_by_name() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    YeastRepository::create(&conn, &yeast).unwrap();

    let results = YeastRepository::search(&conn, "71B", None).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Lalvin 71B-1122");
}

#[test]
fn test_search_by_laboratory() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    YeastRepository::create(&conn, &yeast).unwrap();

    let results = YeastRepository::search(&conn, "Lalvin", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_case_insensitive() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    YeastRepository::create(&conn, &yeast).unwrap();

    let results = YeastRepository::search(&conn, "lalvin", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_empty_query() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    YeastRepository::create(&conn, &yeast).unwrap();

    let results = YeastRepository::search(&conn, "", None).unwrap();
    assert_eq!(results.len(), 0, "Empty query should return no results");
}

#[test]
fn test_get_by_type() {
    let conn = setup_test_db();

    let mut wine_yeast = create_test_yeast();
    wine_yeast.yeast_type = "wine".to_string();

    let mut beer_yeast = create_test_yeast();
    beer_yeast.name = "Beer Yeast".to_string();
    beer_yeast.yeast_type = "beer".to_string();

    YeastRepository::create(&conn, &wine_yeast).unwrap();
    YeastRepository::create(&conn, &beer_yeast).unwrap();

    let wine_yeasts = YeastRepository::get_by_type(&conn, "wine", None).unwrap();
    assert_eq!(wine_yeasts.len(), 1);
    assert_eq!(wine_yeasts[0].yeast_type, "wine");
}

#[test]
fn test_get_by_laboratory() {
    let conn = setup_test_db();

    let mut lalvin1 = create_test_yeast();
    lalvin1.name = "Lalvin 71B".to_string();

    let mut lalvin2 = create_test_yeast();
    lalvin2.name = "Lalvin D47".to_string();
    lalvin2.product_code = "D47".to_string();

    let mut wyeast = create_test_yeast();
    wyeast.name = "Wyeast 1056".to_string();
    wyeast.laboratory = "Wyeast".to_string();

    YeastRepository::create(&conn, &lalvin1).unwrap();
    YeastRepository::create(&conn, &lalvin2).unwrap();
    YeastRepository::create(&conn, &wyeast).unwrap();

    let lalvin_yeasts = YeastRepository::get_by_laboratory(&conn, "Lalvin", None).unwrap();
    assert_eq!(lalvin_yeasts.len(), 2);
}

#[test]
fn test_update_yeast() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();

    yeast.id = id;
    yeast.name = "Updated Yeast".to_string();
    yeast.alcohol_tolerance = Some(Decimal::from_str("18.0").unwrap());
    yeast.updated_at = "2025-01-02T00:00:00Z".to_string();

    YeastRepository::update(&conn, &yeast).unwrap();

    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated Yeast");
    assert_eq!(retrieved.alcohol_tolerance, Some(Decimal::from_str("18.0").unwrap()));
}

#[test]
fn test_delete_yeast() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    YeastRepository::delete(&conn, id).unwrap();

    let result = YeastRepository::get_by_id(&conn, id);
    assert!(result.is_err(), "Should not find deleted yeast");
}

#[test]
fn test_count() {
    let conn = setup_test_db();

    assert_eq!(YeastRepository::count(&conn).unwrap(), 0);

    YeastRepository::create(&conn, &create_test_yeast()).unwrap();
    assert_eq!(YeastRepository::count(&conn).unwrap(), 1);

    YeastRepository::create(&conn, &create_test_yeast()).unwrap();
    assert_eq!(YeastRepository::count(&conn).unwrap(), 2);
}

#[test]
fn test_count_by_type() {
    let conn = setup_test_db();

    let mut wine_yeast = create_test_yeast();
    wine_yeast.yeast_type = "wine".to_string();

    let mut beer_yeast = create_test_yeast();
    beer_yeast.name = "Beer Yeast".to_string();
    beer_yeast.yeast_type = "beer".to_string();

    YeastRepository::create(&conn, &wine_yeast).unwrap();
    YeastRepository::create(&conn, &beer_yeast).unwrap();
    YeastRepository::create(&conn, &beer_yeast).unwrap();

    assert_eq!(YeastRepository::count_by_type(&conn, "wine").unwrap(), 1);
    assert_eq!(YeastRepository::count_by_type(&conn, "beer").unwrap(), 2);
}

// ===== EDGE CASE TESTS =====

#[test]
fn test_create_with_minimal_fields() {
    let conn = setup_test_db();

    let minimal = YeastStrain {
        id: 0,
        name: "Minimal Yeast".to_string(),
        laboratory: "TestLab".to_string(),
        product_code: "MIN1".to_string(),
        yeast_type: "wine".to_string(),
        alcohol_tolerance: None,
        temperature_range_min: None,
        temperature_range_max: None,
        attenuation: None,
        flocculation: None,
        nutrient_requirements: None,
        flavor_profile: None,
        aroma_profile: None,
        best_suited_styles: None,
        usage_notes: None,
        lag_time_hours: None,
        fermentation_duration_days: None,
        sensory_notes: None,
        requires_rehydration: false,
        compatible_ingredients: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = YeastRepository::create(&conn, &minimal).unwrap();
    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, "Minimal Yeast");
    assert_eq!(retrieved.alcohol_tolerance, None);
}

#[test]
fn test_decimal_precision() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    // Test high-precision decimal
    yeast.alcohol_tolerance = Some(Decimal::from_str("14.567890123456").unwrap());

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(
        retrieved.alcohol_tolerance,
        Some(Decimal::from_str("14.567890123456").unwrap()),
        "Decimal precision must be preserved"
    );
}

#[test]
fn test_get_nonexistent_id() {
    let conn = setup_test_db();
    let result = YeastRepository::get_by_id(&conn, 99999);
    assert!(result.is_err(), "Should error on nonexistent ID");
}

#[test]
fn test_update_nonexistent() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.id = 99999;

    let result = YeastRepository::update(&conn, &yeast);
    assert!(result.is_err(), "Should error when updating nonexistent yeast");
}

#[test]
fn test_delete_nonexistent() {
    let conn = setup_test_db();
    let result = YeastRepository::delete(&conn, 99999);
    assert!(result.is_err(), "Should error when deleting nonexistent yeast");
}

#[test]
fn test_boundary_alcohol_tolerance() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    // Test minimum boundary
    yeast.alcohol_tolerance = Some(Decimal::ZERO);
    YeastRepository::create(&conn, &yeast).unwrap();

    // Test maximum boundary
    yeast.name = "Max Tolerance".to_string();
    yeast.alcohol_tolerance = Some(Decimal::from(25));
    YeastRepository::create(&conn, &yeast).unwrap();
}

#[test]
fn test_boundary_attenuation() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    // Test minimum
    yeast.attenuation = Some(Decimal::ZERO);
    YeastRepository::create(&conn, &yeast).unwrap();

    // Test maximum
    yeast.name = "Max Attenuation".to_string();
    yeast.attenuation = Some(Decimal::from(100));
    YeastRepository::create(&conn, &yeast).unwrap();
}

// ===== INVALID INPUT TESTS =====

#[test]
fn test_create_empty_name() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.name = "".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject empty name");
}

#[test]
fn test_create_empty_laboratory() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.laboratory = "".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject empty laboratory");
}

#[test]
fn test_create_empty_product_code() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.product_code = "".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject empty product code");
}

#[test]
fn test_create_invalid_type() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.yeast_type = "invalid_type".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject invalid yeast type");
}

#[test]
fn test_create_negative_alcohol_tolerance() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.alcohol_tolerance = Some(Decimal::from(-5));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject negative alcohol tolerance");
}

#[test]
fn test_create_excessive_alcohol_tolerance() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.alcohol_tolerance = Some(Decimal::from(30));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject excessive alcohol tolerance");
}

#[test]
fn test_create_invalid_temperature_range() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.temperature_range_min = Some(Decimal::from(90));
    yeast.temperature_range_max = Some(Decimal::from(60));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject inverted temperature range");
}

#[test]
fn test_create_temperature_too_low() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.temperature_range_min = Some(Decimal::from(20));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject temperature below 32°F");
}

#[test]
fn test_create_temperature_too_high() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.temperature_range_max = Some(Decimal::from(150));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject temperature above 120°F");
}

#[test]
fn test_create_negative_attenuation() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.attenuation = Some(Decimal::from(-10));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject negative attenuation");
}

#[test]
fn test_create_excessive_attenuation() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.attenuation = Some(Decimal::from(110));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject attenuation > 100%");
}

#[test]
fn test_create_invalid_flocculation() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.flocculation = Some("super_high".to_string());

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject invalid flocculation level");
}

#[test]
fn test_create_invalid_nutrient_requirements() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.nutrient_requirements = Some("extreme".to_string());

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject invalid nutrient requirement");
}

#[test]
fn test_create_excessive_lag_time() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.lag_time_hours = Some(Decimal::from(200));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject lag time > 168 hours");
}

#[test]
fn test_create_excessive_fermentation_duration() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.fermentation_duration_days = Some(Decimal::from(400));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject duration > 365 days");
}

#[test]
fn test_name_too_long() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.name = "a".repeat(201);

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject name > 200 characters");
}

#[test]
fn test_laboratory_too_long() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.laboratory = "a".repeat(101);

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject laboratory > 100 characters");
}

#[test]
fn test_product_code_too_long() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.product_code = "a".repeat(51);

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err(), "Should reject product code > 50 characters");
}