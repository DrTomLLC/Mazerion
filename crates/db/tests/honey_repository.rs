use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;

// Import from the db crate
use mazerion_db::models::honey::Honey;
use mazerion_db::repositories::honey::HoneyRepository;
use mazerion_db::schemas::honeys::HONEY_SCHEMA;

/// Helper: Create in-memory database with honey schema
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(HONEY_SCHEMA).unwrap();
    conn
}

/// Helper: Create valid test honey
fn create_test_honey() -> Honey {
    Honey {
        id: 0,
        name: "Orange Blossom".to_string(),
        floral_source: "Citrus sinensis".to_string(),
        origin: Some("California".to_string()),
        color: "white".to_string(),
        moisture_content: Some(Decimal::from_str("17.5").unwrap()),
        fructose_percentage: Some(Decimal::from_str("40.0").unwrap()),
        glucose_percentage: Some(Decimal::from_str("30.0").unwrap()),
        other_sugars_percentage: Some(Decimal::from_str("5.0").unwrap()),
        specific_gravity: Some(Decimal::from_str("1.425").unwrap()),
        ph: Some(Decimal::from_str("3.9").unwrap()),
        flavor_intensity: "mild".to_string(),
        flavor_profile: Some(r#"["citrus","floral","sweet"]"#.to_string()),
        aroma_profile: Some(r#"["orange blossom","honey"]"#.to_string()),
        crystallization_tendency: Some("slow".to_string()),
        best_suited_styles: Some(r#"["traditional mead","melomel"]"#.to_string()),
        usage_notes: Some("Excellent for delicate meads".to_string()),
        sensory_notes: Some("Light, fruity character".to_string()),
        harvest_season: Some("Spring".to_string()),
        is_monofloral: true,
        is_raw: Some(true),
        compatible_yeasts: Some(r#"["wine","mead","champagne"]"#.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

// ===== NORMAL OPERATION TESTS =====

#[test]
fn test_create_honey() {
    let conn = setup_test_db();
    let honey = create_test_honey();

    let id = HoneyRepository::create(&conn, &honey).unwrap();
    assert!(id > 0, "Generated ID should be positive");
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let honey = create_test_honey();

    let id = HoneyRepository::create(&conn, &honey).unwrap();
    let retrieved = HoneyRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, honey.name);
    assert_eq!(retrieved.floral_source, honey.floral_source);
    assert_eq!(retrieved.color, honey.color);
    assert_eq!(retrieved.moisture_content, honey.moisture_content);
}

#[test]
fn test_list_honeys() {
    let conn = setup_test_db();
    let honey1 = create_test_honey();
    let mut honey2 = create_test_honey();
    honey2.name = "Wildflower".to_string();
    honey2.floral_source = "Mixed wildflowers".to_string();
    honey2.is_monofloral = false;

    HoneyRepository::create(&conn, &honey1).unwrap();
    HoneyRepository::create(&conn, &honey2).unwrap();

    let honeys = HoneyRepository::list(&conn, None).unwrap();
    assert_eq!(honeys.len(), 2);
}

#[test]
fn test_list_with_limit() {
    let conn = setup_test_db();

    // Create 5 honeys
    for i in 0..5 {
        let mut honey = create_test_honey();
        honey.name = format!("Honey {}", i);
        HoneyRepository::create(&conn, &honey).unwrap();
    }

    let honeys = HoneyRepository::list(&conn, Some(3)).unwrap();
    assert_eq!(honeys.len(), 3, "Should respect limit");
}

#[test]
fn test_search_by_name() {
    let conn = setup_test_db();
    let honey = create_test_honey();
    HoneyRepository::create(&conn, &honey).unwrap();

    let results = HoneyRepository::search(&conn, "Orange", None).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Orange Blossom");
}

#[test]
fn test_search_by_floral_source() {
    let conn = setup_test_db();
    let honey = create_test_honey();
    HoneyRepository::create(&conn, &honey).unwrap();

    let results = HoneyRepository::search(&conn, "Citrus", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_by_origin() {
    let conn = setup_test_db();
    let honey = create_test_honey();
    HoneyRepository::create(&conn, &honey).unwrap();

    let results = HoneyRepository::search(&conn, "California", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_case_insensitive() {
    let conn = setup_test_db();
    let honey = create_test_honey();
    HoneyRepository::create(&conn, &honey).unwrap();

    let results = HoneyRepository::search(&conn, "orange", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_empty_query() {
    let conn = setup_test_db();
    let honey = create_test_honey();
    HoneyRepository::create(&conn, &honey).unwrap();

    let results = HoneyRepository::search(&conn, "", None).unwrap();
    assert_eq!(results.len(), 0, "Empty query should return no results");
}

#[test]
fn test_get_by_color() {
    let conn = setup_test_db();

    let mut white_honey = create_test_honey();
    white_honey.color = "white".to_string();

    let mut amber_honey = create_test_honey();
    amber_honey.name = "Buckwheat".to_string();
    amber_honey.color = "dark amber".to_string();

    HoneyRepository::create(&conn, &white_honey).unwrap();
    HoneyRepository::create(&conn, &amber_honey).unwrap();

    let white_honeys = HoneyRepository::get_by_color(&conn, "white", None).unwrap();
    assert_eq!(white_honeys.len(), 1);
    assert_eq!(white_honeys[0].color, "white");
}

#[test]
fn test_get_by_intensity() {
    let conn = setup_test_db();

    let mut mild_honey = create_test_honey();
    mild_honey.flavor_intensity = "mild".to_string();

    let mut robust_honey = create_test_honey();
    robust_honey.name = "Buckwheat".to_string();
    robust_honey.flavor_intensity = "robust".to_string();

    HoneyRepository::create(&conn, &mild_honey).unwrap();
    HoneyRepository::create(&conn, &robust_honey).unwrap();

    let mild_honeys = HoneyRepository::get_by_intensity(&conn, "mild", None).unwrap();
    assert_eq!(mild_honeys.len(), 1);
    assert_eq!(mild_honeys[0].flavor_intensity, "mild");
}

#[test]
fn test_get_by_floral_type() {
    let conn = setup_test_db();

    let mut monofloral = create_test_honey();
    monofloral.is_monofloral = true;

    let mut multifloral = create_test_honey();
    multifloral.name = "Wildflower".to_string();
    multifloral.is_monofloral = false;

    HoneyRepository::create(&conn, &monofloral).unwrap();
    HoneyRepository::create(&conn, &multifloral).unwrap();

    let monofloral_honeys = HoneyRepository::get_by_floral_type(&conn, true, None).unwrap();
    assert_eq!(monofloral_honeys.len(), 1);
    assert!(monofloral_honeys[0].is_monofloral);
}

#[test]
fn test_update_honey() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();

    let id = HoneyRepository::create(&conn, &honey).unwrap();

    honey.id = id;
    honey.name = "Updated Honey".to_string();
    honey.moisture_content = Some(Decimal::from_str("18.5").unwrap());
    honey.updated_at = "2025-01-02T00:00:00Z".to_string();

    HoneyRepository::update(&conn, &honey).unwrap();

    let retrieved = HoneyRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated Honey");
    assert_eq!(retrieved.moisture_content, Some(Decimal::from_str("18.5").unwrap()));
}

#[test]
fn test_delete_honey() {
    let conn = setup_test_db();
    let honey = create_test_honey();

    let id = HoneyRepository::create(&conn, &honey).unwrap();
    HoneyRepository::delete(&conn, id).unwrap();

    let result = HoneyRepository::get_by_id(&conn, id);
    assert!(result.is_err(), "Should not find deleted honey");
}

#[test]
fn test_count() {
    let conn = setup_test_db();

    assert_eq!(HoneyRepository::count(&conn).unwrap(), 0);

    HoneyRepository::create(&conn, &create_test_honey()).unwrap();
    assert_eq!(HoneyRepository::count(&conn).unwrap(), 1);

    HoneyRepository::create(&conn, &create_test_honey()).unwrap();
    assert_eq!(HoneyRepository::count(&conn).unwrap(), 2);
}

#[test]
fn test_count_by_color() {
    let conn = setup_test_db();

    let mut white_honey = create_test_honey();
    white_honey.color = "white".to_string();

    let mut amber_honey = create_test_honey();
    amber_honey.name = "Amber Honey".to_string();
    amber_honey.color = "amber".to_string();

    HoneyRepository::create(&conn, &white_honey).unwrap();
    HoneyRepository::create(&conn, &amber_honey).unwrap();
    HoneyRepository::create(&conn, &amber_honey).unwrap();

    assert_eq!(HoneyRepository::count_by_color(&conn, "white").unwrap(), 1);
    assert_eq!(HoneyRepository::count_by_color(&conn, "amber").unwrap(), 2);
}

#[test]
fn test_count_by_intensity() {
    let conn = setup_test_db();

    let mut mild_honey = create_test_honey();
    mild_honey.flavor_intensity = "mild".to_string();

    let mut robust_honey = create_test_honey();
    robust_honey.name = "Robust Honey".to_string();
    robust_honey.flavor_intensity = "robust".to_string();

    HoneyRepository::create(&conn, &mild_honey).unwrap();
    HoneyRepository::create(&conn, &robust_honey).unwrap();
    HoneyRepository::create(&conn, &robust_honey).unwrap();

    assert_eq!(HoneyRepository::count_by_intensity(&conn, "mild").unwrap(), 1);
    assert_eq!(HoneyRepository::count_by_intensity(&conn, "robust").unwrap(), 2);
}

// ===== EDGE CASE TESTS =====

#[test]
fn test_create_with_minimal_fields() {
    let conn = setup_test_db();

    let minimal = Honey {
        id: 0,
        name: "Minimal Honey".to_string(),
        floral_source: "Unknown".to_string(),
        origin: None,
        color: "white".to_string(),
        moisture_content: None,
        fructose_percentage: None,
        glucose_percentage: None,
        other_sugars_percentage: None,
        specific_gravity: None,
        ph: None,
        flavor_intensity: "mild".to_string(),
        flavor_profile: None,
        aroma_profile: None,
        crystallization_tendency: None,
        best_suited_styles: None,
        usage_notes: None,
        sensory_notes: None,
        harvest_season: None,
        is_monofloral: false,
        is_raw: None,
        compatible_yeasts: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = HoneyRepository::create(&conn, &minimal).unwrap();
    let retrieved = HoneyRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, "Minimal Honey");
    assert_eq!(retrieved.moisture_content, None);
}

#[test]
fn test_decimal_precision() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();

    // Test high-precision decimals
    honey.moisture_content = Some(Decimal::from_str("17.567890123456").unwrap());
    honey.fructose_percentage = Some(Decimal::from_str("40.123456789012").unwrap());

    let id = HoneyRepository::create(&conn, &honey).unwrap();
    let retrieved = HoneyRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(
        retrieved.moisture_content,
        Some(Decimal::from_str("17.567890123456").unwrap()),
        "Decimal precision must be preserved"
    );
    assert_eq!(
        retrieved.fructose_percentage,
        Some(Decimal::from_str("40.123456789012").unwrap()),
        "Decimal precision must be preserved"
    );
}

#[test]
fn test_get_nonexistent_id() {
    let conn = setup_test_db();
    let result = HoneyRepository::get_by_id(&conn, 99999);
    assert!(result.is_err(), "Should error on nonexistent ID");
}

#[test]
fn test_update_nonexistent() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.id = 99999;

    let result = HoneyRepository::update(&conn, &honey);
    assert!(result.is_err(), "Should error when updating nonexistent honey");
}

#[test]
fn test_delete_nonexistent() {
    let conn = setup_test_db();
    let result = HoneyRepository::delete(&conn, 99999);
    assert!(result.is_err(), "Should error when deleting nonexistent honey");
}

#[test]
fn test_boundary_moisture_content() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();

    // Test minimum boundary
    honey.moisture_content = Some(Decimal::ZERO);
    HoneyRepository::create(&conn, &honey).unwrap();

    // Test maximum boundary
    honey.name = "Max Moisture".to_string();
    honey.moisture_content = Some(Decimal::from(25));
    HoneyRepository::create(&conn, &honey).unwrap();
}

#[test]
fn test_boundary_sugar_percentages() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();

    // Test edge case with 100% total sugars
    honey.fructose_percentage = Some(Decimal::from(50));
    honey.glucose_percentage = Some(Decimal::from(40));
    honey.other_sugars_percentage = Some(Decimal::from(10));
    HoneyRepository::create(&conn, &honey).unwrap();
}

#[test]
fn test_all_honey_colors() {
    let conn = setup_test_db();
    let colors = vec![
        "extra white", "white", "extra light amber", "light amber",
        "amber", "dark amber", "dark"
    ];

    for (i, color) in colors.iter().enumerate() {
        let mut honey = create_test_honey();
        honey.name = format!("Honey {}", i);
        honey.color = color.to_string();
        HoneyRepository::create(&conn, &honey).unwrap();
    }

    assert_eq!(HoneyRepository::count(&conn).unwrap(), 7);
}

#[test]
fn test_all_intensities() {
    let conn = setup_test_db();
    let intensities = vec!["delicate", "mild", "moderate", "strong", "robust"];

    for (i, intensity) in intensities.iter().enumerate() {
        let mut honey = create_test_honey();
        honey.name = format!("Honey {}", i);
        honey.flavor_intensity = intensity.to_string();
        HoneyRepository::create(&conn, &honey).unwrap();
    }

    assert_eq!(HoneyRepository::count(&conn).unwrap(), 5);
}

// ===== INVALID INPUT TESTS =====

#[test]
fn test_create_empty_name() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.name = "".to_string();

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject empty name");
}

#[test]
fn test_create_empty_floral_source() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.floral_source = "".to_string();

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject empty floral source");
}

#[test]
fn test_create_invalid_color() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.color = "purple".to_string();

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject invalid color");
}

#[test]
fn test_create_excessive_moisture() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.moisture_content = Some(Decimal::from(30));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject excessive moisture content");
}

#[test]
fn test_create_negative_moisture() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.moisture_content = Some(Decimal::from(-5));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject negative moisture");
}

#[test]
fn test_create_excessive_fructose() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.fructose_percentage = Some(Decimal::from(110));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject fructose > 100%");
}

#[test]
fn test_create_excessive_glucose() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.glucose_percentage = Some(Decimal::from(110));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject glucose > 100%");
}

#[test]
fn test_create_sugar_totals_over_100() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.fructose_percentage = Some(Decimal::from(60));
    honey.glucose_percentage = Some(Decimal::from(50));
    honey.other_sugars_percentage = Some(Decimal::from(10));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject sugar total > 100%");
}

#[test]
fn test_create_invalid_specific_gravity_low() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.specific_gravity = Some(Decimal::from_str("0.900").unwrap());

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject SG < 1.000");
}

#[test]
fn test_create_invalid_specific_gravity_high() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.specific_gravity = Some(Decimal::from_str("1.700").unwrap());

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject SG > 1.600");
}

#[test]
fn test_create_invalid_ph_low() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.ph = Some(Decimal::from(1));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject pH < 2.0");
}

#[test]
fn test_create_invalid_ph_high() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.ph = Some(Decimal::from(8));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject pH > 7.0");
}

#[test]
fn test_create_invalid_intensity() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.flavor_intensity = "extreme".to_string();

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject invalid flavor intensity");
}

#[test]
fn test_create_invalid_crystallization() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.crystallization_tendency = Some("instant".to_string());

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject invalid crystallization tendency");
}

#[test]
fn test_name_too_long() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.name = "a".repeat(201);

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject name > 200 characters");
}

#[test]
fn test_floral_source_too_long() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.floral_source = "a".repeat(201);

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject floral source > 200 characters");
}

#[test]
fn test_origin_too_long() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.origin = Some("a".repeat(101));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject origin > 100 characters");
}

#[test]
fn test_harvest_season_too_long() {
    let conn = setup_test_db();
    let mut honey = create_test_honey();
    honey.harvest_season = Some("a".repeat(51));

    let result = HoneyRepository::create(&conn, &honey);
    assert!(result.is_err(), "Should reject harvest season > 50 characters");
}