use rusqlite::Connection;
use rust_decimal::Decimal;
use std::str::FromStr;

use mazerion_db::models::hop::Hop;
use mazerion_db::repositories::hop::HopRepository;
use mazerion_db::schemas::hops::HOP_SCHEMA;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(HOP_SCHEMA).unwrap();
    conn
}

fn create_test_hop() -> Hop {
    Hop {
        id: 0,
        name: "Cascade".to_string(),
        origin: "USA".to_string(),
        hop_type: "dual-purpose".to_string(),
        alpha_acid: Some(Decimal::from_str("5.5").unwrap()),
        beta_acid: Some(Decimal::from_str("4.8").unwrap()),
        cohumulone: Some(Decimal::from_str("33.0").unwrap()),
        total_oil: Some(Decimal::from_str("1.5").unwrap()),
        myrcene: Some(Decimal::from_str("50.0").unwrap()),
        humulene: Some(Decimal::from_str("12.0").unwrap()),
        caryophyllene: Some(Decimal::from_str("6.0").unwrap()),
        farnesene: Some(Decimal::from_str("4.0").unwrap()),
        flavor_profile: Some(r#"["citrus","floral","grapefruit"]"#.to_string()),
        aroma_profile: Some(r#"["citrus","pine","floral"]"#.to_string()),
        substitutes: Some(r#"["Centennial","Amarillo"]"#.to_string()),
        best_suited_styles: Some(r#"["IPA","Pale Ale","Porter"]"#.to_string()),
        usage_notes: Some("Excellent dual-purpose hop".to_string()),
        sensory_notes: Some("Distinctive citrus character".to_string()),
        typical_usage: Some("dry hop".to_string()),
        storage_stability: Some("good".to_string()),
        compatible_styles: Some(r#"["American Ale"]"#.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    }
}

// ===== NORMAL OPERATIONS =====

#[test]
fn test_create_hop() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    let id = HopRepository::create(&conn, &hop).unwrap();
    assert!(id > 0);
}

#[test]
fn test_get_by_id() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    let id = HopRepository::create(&conn, &hop).unwrap();
    let retrieved = HopRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.name, hop.name);
    assert_eq!(retrieved.origin, hop.origin);
    assert_eq!(retrieved.alpha_acid, hop.alpha_acid);
}

#[test]
fn test_list_hops() {
    let conn = setup_test_db();
    let mut hop1 = create_test_hop();
    let mut hop2 = create_test_hop();
    hop2.name = "Citra".to_string();

    HopRepository::create(&conn, &hop1).unwrap();
    HopRepository::create(&conn, &hop2).unwrap();

    let hops = HopRepository::list(&conn, None).unwrap();
    assert_eq!(hops.len(), 2);
}

#[test]
fn test_list_with_limit() {
    let conn = setup_test_db();

    for i in 0..5 {
        let mut hop = create_test_hop();
        hop.name = format!("Hop {}", i);
        HopRepository::create(&conn, &hop).unwrap();
    }

    let hops = HopRepository::list(&conn, Some(3)).unwrap();
    assert_eq!(hops.len(), 3);
}

#[test]
fn test_search_by_name() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    HopRepository::create(&conn, &hop).unwrap();

    let results = HopRepository::search(&conn, "Cascade", None).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Cascade");
}

#[test]
fn test_search_by_origin() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    HopRepository::create(&conn, &hop).unwrap();

    let results = HopRepository::search(&conn, "USA", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_case_insensitive() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    HopRepository::create(&conn, &hop).unwrap();

    let results = HopRepository::search(&conn, "cascade", None).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_empty_query() {
    let conn = setup_test_db();
    let hop = create_test_hop();
    HopRepository::create(&conn, &hop).unwrap();

    let results = HopRepository::search(&conn, "", None).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_get_by_type() {
    let conn = setup_test_db();

    let mut dual = create_test_hop();
    dual.hop_type = "dual-purpose".to_string();

    let mut bittering = create_test_hop();
    bittering.name = "Magnum".to_string();
    bittering.hop_type = "bittering".to_string();

    HopRepository::create(&conn, &dual).unwrap();
    HopRepository::create(&conn, &bittering).unwrap();

    let dual_hops = HopRepository::get_by_type(&conn, "dual-purpose", None).unwrap();
    assert_eq!(dual_hops.len(), 1);
    assert_eq!(dual_hops[0].hop_type, "dual-purpose");
}

#[test]
fn test_get_by_origin() {
    let conn = setup_test_db();

    let mut usa = create_test_hop();
    usa.origin = "USA".to_string();

    let mut germany = create_test_hop();
    germany.name = "Hallertau".to_string();
    germany.origin = "Germany".to_string();

    HopRepository::create(&conn, &usa).unwrap();
    HopRepository::create(&conn, &germany).unwrap();

    let usa_hops = HopRepository::get_by_origin(&conn, "USA", None).unwrap();
    assert_eq!(usa_hops.len(), 1);
    assert_eq!(usa_hops[0].origin, "USA");
}

#[test]
fn test_update_hop() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();

    let id = HopRepository::create(&conn, &hop).unwrap();

    hop.id = id;
    hop.name = "Updated Hop".to_string();
    hop.alpha_acid = Some(Decimal::from_str("7.5").unwrap());
    hop.updated_at = "2025-01-02T00:00:00Z".to_string();

    HopRepository::update(&conn, &hop).unwrap();

    let retrieved = HopRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Updated Hop");
    assert_eq!(retrieved.alpha_acid, Some(Decimal::from_str("7.5").unwrap()));
}

#[test]
fn test_delete_hop() {
    let conn = setup_test_db();
    let hop = create_test_hop();

    let id = HopRepository::create(&conn, &hop).unwrap();
    HopRepository::delete(&conn, id).unwrap();

    let result = HopRepository::get_by_id(&conn, id);
    assert!(result.is_err());
}

#[test]
fn test_count() {
    let conn = setup_test_db();
    assert_eq!(HopRepository::count(&conn).unwrap(), 0);

    HopRepository::create(&conn, &create_test_hop()).unwrap();
    assert_eq!(HopRepository::count(&conn).unwrap(), 1);
}

#[test]
fn test_count_by_type() {
    let conn = setup_test_db();

    let mut dual = create_test_hop();
    dual.hop_type = "dual-purpose".to_string();

    let mut aroma = create_test_hop();
    aroma.name = "Aroma Hop".to_string();
    aroma.hop_type = "aroma".to_string();

    HopRepository::create(&conn, &dual).unwrap();
    HopRepository::create(&conn, &aroma).unwrap();
    HopRepository::create(&conn, &aroma).unwrap();

    assert_eq!(HopRepository::count_by_type(&conn, "dual-purpose").unwrap(), 1);
    assert_eq!(HopRepository::count_by_type(&conn, "aroma").unwrap(), 2);
}

#[test]
fn test_count_by_origin() {
    let conn = setup_test_db();

    let mut usa = create_test_hop();
    usa.origin = "USA".to_string();

    let mut germany = create_test_hop();
    germany.name = "German Hop".to_string();
    germany.origin = "Germany".to_string();

    HopRepository::create(&conn, &usa).unwrap();
    HopRepository::create(&conn, &germany).unwrap();
    HopRepository::create(&conn, &germany).unwrap();

    assert_eq!(HopRepository::count_by_origin(&conn, "USA").unwrap(), 1);
    assert_eq!(HopRepository::count_by_origin(&conn, "Germany").unwrap(), 2);
}

// ===== EDGE CASES =====

#[test]
fn test_create_minimal_fields() {
    let conn = setup_test_db();

    let minimal = Hop {
        id: 0,
        name: "Minimal".to_string(),
        origin: "Unknown".to_string(),
        hop_type: "bittering".to_string(),
        alpha_acid: None,
        beta_acid: None,
        cohumulone: None,
        total_oil: None,
        myrcene: None,
        humulene: None,
        caryophyllene: None,
        farnesene: None,
        flavor_profile: None,
        aroma_profile: None,
        substitutes: None,
        best_suited_styles: None,
        usage_notes: None,
        sensory_notes: None,
        typical_usage: None,
        storage_stability: None,
        compatible_styles: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = HopRepository::create(&conn, &minimal).unwrap();
    let retrieved = HopRepository::get_by_id(&conn, id).unwrap();
    assert_eq!(retrieved.name, "Minimal");
    assert_eq!(retrieved.alpha_acid, None);
}

#[test]
fn test_decimal_precision() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();

    hop.alpha_acid = Some(Decimal::from_str("5.567890123456").unwrap());
    hop.beta_acid = Some(Decimal::from_str("4.123456789012").unwrap());

    let id = HopRepository::create(&conn, &hop).unwrap();
    let retrieved = HopRepository::get_by_id(&conn, id).unwrap();

    assert_eq!(retrieved.alpha_acid, Some(Decimal::from_str("5.567890123456").unwrap()));
    assert_eq!(retrieved.beta_acid, Some(Decimal::from_str("4.123456789012").unwrap()));
}

#[test]
fn test_get_nonexistent_id() {
    let conn = setup_test_db();
    let result = HopRepository::get_by_id(&conn, 99999);
    assert!(result.is_err());
}

#[test]
fn test_update_nonexistent() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.id = 99999;

    let result = HopRepository::update(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent() {
    let conn = setup_test_db();
    let result = HopRepository::delete(&conn, 99999);
    assert!(result.is_err());
}

#[test]
fn test_boundary_alpha_acid() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();

    hop.alpha_acid = Some(Decimal::ZERO);
    HopRepository::create(&conn, &hop).unwrap();

    hop.name = "Max Alpha".to_string();
    hop.alpha_acid = Some(Decimal::from(25));
    HopRepository::create(&conn, &hop).unwrap();
}

#[test]
fn test_all_hop_types() {
    let conn = setup_test_db();
    let types = vec!["bittering", "aroma", "dual-purpose"];

    for (i, hop_type) in types.iter().enumerate() {
        let mut hop = create_test_hop();
        hop.name = format!("Hop {}", i);
        hop.hop_type = hop_type.to_string();
        HopRepository::create(&conn, &hop).unwrap();
    }

    assert_eq!(HopRepository::count(&conn).unwrap(), 3);
}

// ===== INVALID INPUTS =====

#[test]
fn test_create_empty_name() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.name = "".to_string();

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_empty_origin() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.origin = "".to_string();

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_type() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.hop_type = "invalid".to_string();

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_alpha() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.alpha_acid = Some(Decimal::from(30));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_negative_alpha() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.alpha_acid = Some(Decimal::from(-5));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_beta() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.beta_acid = Some(Decimal::from(20));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_cohumulone() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.cohumulone = Some(Decimal::from(110));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_excessive_total_oil() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.total_oil = Some(Decimal::from(10));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_oil_totals_over_100() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.myrcene = Some(Decimal::from(60));
    hop.humulene = Some(Decimal::from(40));
    hop.caryophyllene = Some(Decimal::from(10));
    hop.farnesene = Some(Decimal::from(5));

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_typical_usage() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.typical_usage = Some("invalid".to_string());

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_create_invalid_storage_stability() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.storage_stability = Some("terrible".to_string());

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_name_too_long() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.name = "a".repeat(101);

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}

#[test]
fn test_origin_too_long() {
    let conn = setup_test_db();
    let mut hop = create_test_hop();
    hop.origin = "a".repeat(101);

    let result = HopRepository::create(&conn, &hop);
    assert!(result.is_err());
}