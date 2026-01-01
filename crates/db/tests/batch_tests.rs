use rusqlite::Connection;
use rust_decimal::Decimal;

use mazerion_db::models::{Batch, BatchReading, BatchStatus};
use mazerion_db::repositories::batch::BatchRepository;
use mazerion_db::schemas::create_user_schema;

fn setup_test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_user_schema(&conn)?;
    Ok(conn)
}

#[test]
fn test_create_batch() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: 0,
        name: "Traditional Mead Batch 1".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: Some(Decimal::new(1100, 3)),
        target_fg: Some(Decimal::new(1010, 3)),
        target_abv: Some(Decimal::new(120, 1)),
        status: BatchStatus::Planning,
        notes: Some("First batch of the year".to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.create(&batch)?;
    assert!(id > 0);

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.name, batch.name);
    assert_eq!(retrieved.batch_size_l, batch.batch_size_l);
    Ok(())
}

#[test]
fn test_batch_validation() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let mut batch = Batch {
        id: 0,
        name: "Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Planning,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    batch.name = "".to_string();
    assert!(repo.create(&batch).is_err());

    batch.name = "Valid Name".to_string();
    batch.batch_size_l = Decimal::ZERO;
    assert!(repo.create(&batch).is_err());

    Ok(())
}

#[test]
fn test_update_batch_status() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: 0,
        name: "Status Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Planning,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.create(&batch)?;
    repo.update_status(id, BatchStatus::Fermenting)?;

    let retrieved = repo.get(id)?.unwrap();
    assert!(matches!(retrieved.status, BatchStatus::Fermenting));

    Ok(())
}

#[test]
fn test_add_readings() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: 0,
        name: "Readings Test Batch".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: Some(Decimal::new(1100, 3)),
        target_fg: Some(Decimal::new(1010, 3)),
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let batch_id = repo.create(&batch)?;

    let reading1 = BatchReading {
        id: 0,
        batch_id,
        reading_date: "2025-01-02".to_string(),
        gravity: Decimal::new(1090, 3),
        temperature_c: Some(Decimal::new(20, 0)),
        ph: Some(Decimal::new(38, 1)),
        notes: Some("First reading".to_string()),
        source: "manual".to_string(),
    };

    let reading2 = BatchReading {
        id: 0,
        batch_id,
        reading_date: "2025-01-05".to_string(),
        gravity: Decimal::new(1050, 3),
        temperature_c: Some(Decimal::new(20, 0)),
        ph: Some(Decimal::new(36, 1)),
        notes: Some("Second reading".to_string()),
        source: "tilt".to_string(),
    };

    repo.add_reading(&reading1)?;
    repo.add_reading(&reading2)?;

    let readings = repo.get_readings(batch_id)?;
    assert_eq!(readings.len(), 2);
    assert_eq!(readings[0].gravity, Decimal::new(1050, 3));

    Ok(())
}

#[test]
fn test_reading_validation() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: 0,
        name: "Validation Test".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let batch_id = repo.create(&batch)?;

    let reading = BatchReading {
        id: 0,
        batch_id,
        reading_date: "2025-01-02".to_string(),
        gravity: Decimal::new(2000, 3),
        temperature_c: None,
        ph: None,
        notes: None,
        source: "manual".to_string(),
    };
    assert!(repo.add_reading(&reading).is_err());

    let reading = BatchReading {
        id: 0,
        batch_id,
        reading_date: "2025-01-02".to_string(),
        gravity: Decimal::new(1050, 3),
        temperature_c: Some(Decimal::from(100)),
        ph: None,
        notes: None,
        source: "manual".to_string(),
    };
    assert!(repo.add_reading(&reading).is_err());

    Ok(())
}

#[test]
fn test_list_batches_with_filter() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    for i in 0..5 {
        let batch = Batch {
            id: 0,
            name: format!("Batch {}", i),
            recipe_id: None,
            category: "mead".to_string(),
            batch_size_l: Decimal::new(190, 1),
            brew_date: "2025-01-01".to_string(),
            target_og: None,
            target_fg: None,
            target_abv: None,
            status: if i < 2 { BatchStatus::Fermenting } else { BatchStatus::Complete },
            notes: None,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        };
        repo.create(&batch)?;
    }

    let all = repo.list(None, 100)?;
    assert_eq!(all.len(), 5);

    let fermenting = repo.list(Some(BatchStatus::Fermenting), 100)?;
    assert_eq!(fermenting.len(), 2);

    let complete = repo.list(Some(BatchStatus::Complete), 100)?;
    assert_eq!(complete.len(), 3);

    Ok(())
}

#[test]
fn test_delete_batch_cascades_readings() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = BatchRepository::new(&conn);

    let batch = Batch {
        id: 0,
        name: "Delete Test".to_string(),
        recipe_id: None,
        category: "mead".to_string(),
        batch_size_l: Decimal::new(190, 1),
        brew_date: "2025-01-01".to_string(),
        target_og: None,
        target_fg: None,
        target_abv: None,
        status: BatchStatus::Fermenting,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let batch_id = repo.create(&batch)?;

    for i in 0..3 {
        let reading = BatchReading {
            id: 0,
            batch_id,
            reading_date: format!("2025-01-{:02}", i + 1),
            gravity: Decimal::new(1050, 3),
            temperature_c: Some(Decimal::from(20)),
            ph: None,
            notes: None,
            source: "manual".to_string(),
        };
        repo.add_reading(&reading)?;
    }

    let readings_before = repo.get_readings(batch_id)?;
    assert_eq!(readings_before.len(), 3);

    repo.delete(batch_id)?;

    let batch_result = repo.get(batch_id)?;
    assert!(batch_result.is_none());

    let readings_after = repo.get_readings(batch_id)?;
    assert_eq!(readings_after.len(), 0);

    Ok(())
}