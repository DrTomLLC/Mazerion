// Inventory repository tests

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
fn test_add_inventory_item() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    let item = InventoryItem {
        id: None,
        item_type: "honey".to_string(),
        item_name: "Orange Blossom Honey".to_string(),
        quantity: Decimal::from_str_exact("5.4").unwrap(),
        unit: "kg".to_string(),
        location: Some("pantry".to_string()),
        purchase_date: Some("2025-01-01".to_string()),
        expiration_date: None,
        cost: Some(Decimal::from(45)),
        notes: Some("From local beekeeper".to_string()),
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.add(&item)?;
    assert!(id > 0);

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.item_name, "Orange Blossom Honey");
    assert_eq!(retrieved.quantity, Decimal::from_str_exact("5.4").unwrap());

    Ok(())
}

#[test]
fn test_update_quantity() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    let item = InventoryItem {
        id: None,
        item_type: "yeast".to_string(),
        item_name: "Lalvin 71B".to_string(),
        quantity: Decimal::from(5),
        unit: "packets".to_string(),
        location: Some("fridge".to_string()),
        purchase_date: None,
        expiration_date: None,
        cost: None,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = repo.add(&item)?;

    repo.update_quantity(id, Decimal::from(3))?;

    let retrieved = repo.get(id)?.unwrap();
    assert_eq!(retrieved.quantity, Decimal::from(3));

    Ok(())
}

#[test]
fn test_search_inventory() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    let items = vec![
        ("Orange Blossom Honey", "honey"),
        ("Wildflower Honey", "honey"),
        ("Cascade Hops", "hops"),
    ];

    for (name, itype) in items {
        let item = InventoryItem {
            id: None,
            item_type: itype.to_string(),
            item_name: name.to_string(),
            quantity: Decimal::from(1),
            unit: "kg".to_string(),
            location: None,
            purchase_date: None,
            expiration_date: None,
            cost: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };
        repo.add(&item)?;
    }

    let results = repo.search("honey", 10)?;
    assert_eq!(results.len(), 2);

    let results = repo.search("cascade", 10)?;
    assert_eq!(results.len(), 1);

    Ok(())
}

#[test]
fn test_list_by_type() -> Result<()> {
    let conn = setup_test_db()?;
    let repo = InventoryRepository::new(&conn);

    for i in 0..5 {
        let item = InventoryItem {
            id: None,
            item_type: if i < 3 { "grain" } else { "hops" }.to_string(),
            item_name: format!("Item {}", i),
            quantity: Decimal::from(1),
            unit: "kg".to_string(),
            location: None,
            purchase_date: None,
            expiration_date: None,
            cost: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };
        repo.add(&item)?;
    }

    let grains = repo.list(Some("grain"), 100)?;
    assert_eq!(grains.len(), 3);

    let hops = repo.list(Some("hops"), 100)?;
    assert_eq!(hops.len(), 2);

    let all = repo.list(None, 100)?;
    assert_eq!(all.len(), 5);

    Ok(())
}