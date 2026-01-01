use rusqlite::Connection;
use rust_decimal::Decimal;

use mazerion_db::models::InventoryItem;
use mazerion_db::repositories::inventory::InventoryRepository;
use mazerion_db::schemas::create_user_schema;

fn setup_test_db() -> anyhow::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_user_schema(&conn)?;
    Ok(conn)
}

#[test]
fn test_add_inventory_item() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    let item = InventoryItem {
        id: 0,
        item_type: "honey".to_string(),
        item_name: "Wildflower Honey".to_string(),
        quantity: Decimal::new(50, 1),
        unit: "kg".to_string(),
        location: Some("Storage Room A".to_string()),
        purchase_date: Some("2025-01-01".to_string()),
        expiration_date: None,
        cost: Some(Decimal::from(150)),
        notes: Some("Premium quality".to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.add(&item)?;
    assert!(id > 0);

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.item_name, item.item_name);
    assert_eq!(retrieved.quantity, item.quantity);

    Ok(())
}

#[test]
fn test_update_quantity() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    let item = InventoryItem {
        id: 0,
        item_type: "yeast".to_string(),
        item_name: "Lalvin 71B".to_string(),
        quantity: Decimal::from(10),
        unit: "packets".to_string(),
        location: None,
        purchase_date: None,
        expiration_date: None,
        cost: None,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };

    let id = repo.add(&item)?;
    repo.update_quantity(id, Decimal::from(5))?;

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.quantity, Decimal::from(5));

    Ok(())
}

#[test]
fn test_search_inventory() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    for i in 0..3 {
        let item = InventoryItem {
            id: 0,
            item_type: "honey".to_string(),
            item_name: format!("Honey Type {}", i),
            quantity: Decimal::from(10),
            unit: "kg".to_string(),
            location: Some(format!("Location {}", i)),
            purchase_date: None,
            expiration_date: None,
            cost: None,
            notes: None,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        };
        repo.add(&item)?;
    }

    let results = repo.search("Type 1", 100)?;
    assert_eq!(results.len(), 1);

    let results = repo.search("honey", 100)?;
    assert_eq!(results.len(), 3);

    Ok(())
}

#[test]
fn test_list_by_type() -> anyhow::Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    for i in 0..2 {
        let item = InventoryItem {
            id: 0,
            item_type: "honey".to_string(),
            item_name: format!("Honey {}", i),
            quantity: Decimal::from(10),
            unit: "kg".to_string(),
            location: None,
            purchase_date: None,
            expiration_date: None,
            cost: None,
            notes: None,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
        };
        repo.add(&item)?;
    }

    let item = InventoryItem {
        id: 0,
        item_type: "yeast".to_string(),
        item_name: "Yeast 1".to_string(),
        quantity: Decimal::from(5),
        unit: "packets".to_string(),
        location: None,
        purchase_date: None,
        expiration_date: None,
        cost: None,
        notes: None,
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
    };
    repo.add(&item)?;

    let honey_items = repo.list(Some("honey"), 100)?;
    assert_eq!(honey_items.len(), 2);

    let all_items = repo.list(None, 100)?;
    assert_eq!(all_items.len(), 3);

    Ok(())
}