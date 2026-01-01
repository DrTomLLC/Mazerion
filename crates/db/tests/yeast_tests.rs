use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;

use mazerion_db::models::yeast::Yeast;
use mazerion_db::repositories::yeast::YeastRepository;
use mazerion_db::schemas::yeasts::YEAST_SCHEMA;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(YEAST_SCHEMA).unwrap();
    conn
}

fn create_test_yeast() -> Yeast {
    Yeast {
        id: None,
        name: "Lalvin 71B-1122".to_string(),
        laboratory: "Lalvin".to_string(),
        product_code: "71B".to_string(),
        yeast_type: "wine".to_string(),
        yeast_form: "dry".to_string(),
        alcohol_tolerance: Some(Decimal::from_str("14.0").unwrap()),
        temperature_range_min: Some(Decimal::from_str("59.0").unwrap()),
        temperature_range_max: Some(Decimal::from_str("86.0").unwrap()),
        attenuation: Some(Decimal::from_str("80.0").unwrap()),
        flocculation: "medium".to_string(),
        nutrient_requirements: "moderate".to_string(),
        flavor_profile: r#"["fruity","ester"]"#.to_string(),
        aroma_profile: r#"["tropical","citrus"]"#.to_string(),
        best_suited_styles: r#"["Traditional Mead","Cyser"]"#.to_string(),
        usage_notes: "Excellent for fruit meads".to_string(),
        lag_time_hours: Some(Decimal::from_str("12.0").unwrap()),
        fermentation_duration_days: Some(Decimal::from_str("14.0").unwrap()),
        sensory_notes: "Enhances fruit character".to_string(),
        notes: "General notes".to_string(),
        requires_rehydration: 1,
        compatible_ingredients: r#"["honey","fruit","spices"]"#.to_string(),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn test_create_yeast() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    let id = YeastRepository::create(&conn, &yeast).unwrap();
    assert!(id > 0);
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();
    let id = YeastRepository::create(&conn, &yeast).unwrap();
    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, yeast.name);
    assert_eq!(retrieved.laboratory, yeast.laboratory);
    assert_eq!(retrieved.alcohol_tolerance, yeast.alcohol_tolerance);
}

#[test]
fn test_list_yeasts() {
    let conn = setup_test_db();
    let yeast1 = create_test_yeast();
    let mut yeast2 = create_test_yeast();
    yeast2.name = "Lalvin D47".to_string();
    yeast2.product_code = "D47".to_string();

    YeastRepository::create(&conn, &yeast1).unwrap();
    YeastRepository::create(&conn, &yeast2).unwrap();

    let yeasts = YeastRepository::list(&conn, None).unwrap();
    assert_eq!(yeasts.len(), 2);
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
fn test_get_by_type() {
    let conn = setup_test_db();
    let wine_yeast = create_test_yeast();

    let mut beer_yeast = create_test_yeast();
    beer_yeast.name = "Safale US-05".to_string();
    beer_yeast.yeast_type = "beer".to_string();
    beer_yeast.product_code = "US-05".to_string();

    YeastRepository::create(&conn, &wine_yeast).unwrap();
    YeastRepository::create(&conn, &beer_yeast).unwrap();

    let wine_yeasts = YeastRepository::get_by_type(&conn, "wine", None).unwrap();
    assert_eq!(wine_yeasts.len(), 1);
    assert_eq!(wine_yeasts[0].yeast_type, "wine");
}

#[test]
fn test_update_yeast() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();

    yeast.id = Some(id);
    yeast.name = "Updated Yeast".to_string();
    yeast.alcohol_tolerance = Some(Decimal::from_str("15.0").unwrap());
    yeast.updated_at = "2025-01-02T00:00:00Z".to_string();

    YeastRepository::update(&conn, &yeast).unwrap();

    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated Yeast");
    assert_eq!(retrieved.alcohol_tolerance, Some(Decimal::from_str("15.0").unwrap()));
}

#[test]
fn test_delete_yeast() {
    let conn = setup_test_db();
    let yeast = create_test_yeast();

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    YeastRepository::delete(&conn, id).unwrap();

    let result = YeastRepository::get_by_id(&conn, id);
    assert!(result.is_err());
}

#[test]
fn test_count() {
    let conn = setup_test_db();
    assert_eq!(YeastRepository::count(&conn).unwrap(), 0);

    YeastRepository::create(&conn, &create_test_yeast()).unwrap();
    assert_eq!(YeastRepository::count(&conn).unwrap(), 1);
}

#[test]
fn test_create_empty_name() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.name = "".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_type() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.yeast_type = "invalid".to_string();

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_alcohol_tolerance() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();
    yeast.alcohol_tolerance = Some(Decimal::from(30));

    let result = YeastRepository::create(&conn, &yeast);
    assert!(result.is_err());
}

#[test]
fn test_boundary_temperature() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    yeast.temperature_range_min = Some(Decimal::from(32));
    yeast.temperature_range_max = Some(Decimal::from(120));
    assert!(YeastRepository::create(&conn, &yeast).is_ok());
}

#[test]
fn test_all_yeast_types() {
    let conn = setup_test_db();
    let types = vec!["wine", "beer", "mead", "champagne", "distillers", "wild"];

    for (i, yeast_type) in types.iter().enumerate() {
        let mut yeast = create_test_yeast();
        yeast.name = format!("Yeast {}", i);
        yeast.product_code = format!("CODE{}", i);
        yeast.yeast_type = yeast_type.to_string();
        YeastRepository::create(&conn, &yeast).unwrap();
    }

    assert_eq!(YeastRepository::count(&conn).unwrap(), 6);
}

#[test]
fn test_decimal_precision() {
    let conn = setup_test_db();
    let mut yeast = create_test_yeast();

    yeast.alcohol_tolerance = Some(Decimal::from_str("14.567890123456").unwrap());
    yeast.attenuation = Some(Decimal::from_str("78.123456789012").unwrap());

    let id = YeastRepository::create(&conn, &yeast).unwrap();
    let retrieved = YeastRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.alcohol_tolerance, Some(Decimal::from_str("14.567890123456").unwrap()));
    assert_eq!(retrieved.attenuation, Some(Decimal::from_str("78.123456789012").unwrap()));
}