// Comprehensive batch repository tests

use mazerion_db::*;
use mazerion_core::Result;
use rusqlite::Connection;
use rust_decimal::Decimal;

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()
        .map_err(|e| mazerion_core::Error::DatabaseError(format!("Failed to open: {}", e)))?;

    create_user_schema(&conn)?;
    Ok(conn)
}

#[test]
fn test_create_batch() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: None,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: Some(Decimal::from_str_exact("1.100").unwrap()),
        target_fg: Some(Decimal::from_str_exact("1.010").unwrap()),
        target_abv: Some(Decimal::from(14)),
        status: BatchStatus::Planning,
        notes: Some("Test notes".to_string()),
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.create(&batch)?;
    assert!(id > 0);

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.name, "Test Batch");
    assert_eq!(retrieved.category, "mead");
    assert_eq!(retrieved.batch_size_l, Decimal::from(19));

    Ok(())
}

#[test]
fn test_batch_validation() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    // Empty name
    let mut batch = Batch {
        id: None,
        name: "".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Planning,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    assert!(repo.create(&batch).is_err());

    // Negative batch size
    batch.name = "Valid Name".to_string();
    batch.batch_size_l = Decimal::from(-1);
    assert!(repo.create(&batch).is_err());

    // Batch size too large
    batch.batch_size_l = Decimal::from(20000);
    assert!(repo.create(&batch).is_err());

    Ok(())
}

#[test]
fn test_update_batch_status() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: None,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Planning,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.create(&batch)?;

    repo.update_status(id, BatchStatus::Fermenting)?;

    let retrieved = repo.get(id)?.unwrap();
    assert!(matches!(retrieved.status, BatchStatus::Fermenting));

    Ok(())
}

#[test]
fn test_add_readings() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: None,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: Some(Decimal::from_str_exact("1.100").unwrap()),
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let batch_id = repo.create(&batch)?;

    let reading1 = BatchReading {
        id: None,
        batch_id,
        reading_date: "2025-01-01".to_string(),
        gravity: Decimal::from_str_exact("1.100").unwrap(),
        temperature_c: Some(Decimal::from(20)),
        ph: Some(Decimal::from_str_exact("3.5").unwrap()),
        notes: Some("Initial reading".to_string()),
        source: "manual".to_string(),
    };

    let reading2 = BatchReading {
        id: None,
        batch_id,
        reading_date: "2025-01-08".to_string(),
        gravity: Decimal::from_str_exact("1.050").unwrap(),
        temperature_c: Some(Decimal::from(21)),
        ph: None,
        notes: None,
        source: "tilt".to_string(),
    };

    repo.add_reading(&reading1)?;
    repo.add_reading(&reading2)?;

    let readings = repo.get_readings(batch_id)?;
    assert_eq!(readings.len(), 2);
    assert_eq!(readings[0].gravity, Decimal::from_str_exact("1.050").unwrap());
    assert_eq!(readings[1].gravity, Decimal::from_str_exact("1.100").unwrap());

    Ok(())
}

#[test]
fn test_reading_validation() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: None,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let batch_id = repo.create(&batch)?;

    // Invalid gravity (too low)
    let reading = BatchReading {
        id: None,
        batch_id,
        reading_date: "2025-01-01".to_string(),
        gravity: Decimal::from_str_exact("0.800").unwrap(),
        temperature_c: None,
        ph: None,
        notes: None,
        source: "manual".to_string(),
    };

    assert!(repo.add_reading(&reading).is_err());

    // Invalid pH
    let reading = BatchReading {
        id: None,
        batch_id,
        reading_date: "2025-01-01".to_string(),
        gravity: Decimal::from_str_exact("1.050").unwrap(),
        temperature_c: None,
        ph: Some(Decimal::from(15)),
        notes: None,
        source: "manual".to_string(),
    };

    assert!(repo.add_reading(&reading).is_err());

    Ok(())
}

#[test]
fn test_list_batches_with_filter() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    // Create batches with different statuses
    for i in 0..3 {
        let batch = Batch {
            id: None,
            name: format!("Batch {}", i),
            recipe_id: None,
            category: "mead".to_string(),
            batch_size_l: Decimal::from(19),
            brew_date: "2025-01-01".to_string(),
            target_og: None,
            target_fg: None,
            target_abv: None,
            status: if i < 2 { BatchStatus::Fermenting } else { BatchStatus::Complete },
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };
        repo.create(&batch)?;
    }

    let all = repo.list(None, 100)?;
    assert_eq!(all.len(), 3);

    let fermenting = repo.list(Some(BatchStatus::Fermenting), 100)?;
    assert_eq!(fermenting.len(), 2);

    let complete = repo.list(Some(BatchStatus::Complete), 100)?;
    assert_eq!(complete.len(), 1);

    Ok(())
}

#[test]
fn test_delete_batch_cascades_readings() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: None,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::from(19),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let batch_id = repo.create(&batch)?;

    // Add readings
    for i in 0..3 {
        let reading = BatchReading {
            id: None,
            batch_id,
            reading_date: format!("2025-01-{:02}", i + 1),
            gravity: Decimal::from_str_exact("1.050").unwrap(),
            temperature_c: None,
            ph: None,
            notes: None,
            source: "manual".to_string(),
        };
        repo.add_reading(&reading)?;
    }

    assert_eq!(repo.get_readings(batch_id)?.len(), 3);

    // Delete batch
    repo.delete(batch_id)?;

    // Verify batch and readings are gone
    assert!(repo.get(batch_id)?.is_none());
    assert_eq!(repo.get_readings(batch_id)?.len(), 0);

    Ok(())
}