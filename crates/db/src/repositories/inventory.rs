// Inventory repository with proper Decimal handling

use rusqlite::{params, Connection, OptionalExtension, Row};
use rust_decimal::Decimal;
use mazerion_core::{Error, Result};
use crate::models::InventoryItem;
use std::str::FromStr;

pub struct InventoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> InventoryRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn add(&self, item: &InventoryItem) -> Result<i64> {
        item.validate()?;

        self.conn
            .execute(
                "INSERT INTO inventory (item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    &item.item_type,
                    &item.item_name,
                    item.quantity.to_string(),
                    &item.unit,
                    &item.location,
                    &item.purchase_date,
                    &item.expiration_date,
                    item.cost.as_ref().map(|v| v.to_string()),
                    &item.notes,
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to insert: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get(&self, id: i64) -> Result<Option<InventoryItem>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_item(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Failed to get: {}", e)))?;

        Ok(result)
    }

    pub fn list(&self, item_type: Option<&str>, limit: usize) -> Result<Vec<InventoryItem>> {
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(itype) = item_type {
            (
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory WHERE item_type = ?1 ORDER BY item_name ASC LIMIT ?2".to_string(),
                vec![Box::new(itype.to_string()), Box::new(limit as i64)],
            )
        } else {
            (
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory ORDER BY item_name ASC LIMIT ?1".to_string(),
                vec![Box::new(limit as i64)],
            )
        };

        let mut stmt = self
            .conn
            .prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare: {}", e)))?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let items = stmt
            .query_map(params_refs.as_slice(), |row| Self::row_to_item(row))
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(items)
    }

    pub fn update_quantity(&self, id: i64, quantity: Decimal) -> Result<()> {
        if quantity < Decimal::ZERO || quantity > Decimal::from(1000000) {
            return Err(Error::Validation("Invalid quantity".into()));
        }

        let affected = self
            .conn
            .execute(
                "UPDATE inventory SET quantity = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![quantity.to_string(), id],
            )
            .map_err(|e| Error::DatabaseError(format!("Update failed: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Item {} not found", id)));
        }

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM inventory WHERE id = ?1", params![id])
            .map_err(|e| Error::DatabaseError(format!("Delete failed: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Item {} not found", id)));
        }

        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<InventoryItem>> {
        if query.len() > 200 {
            return Err(Error::Validation("Query too long".into()));
        }

        let search_pattern = format!("%{}%", query);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory WHERE item_name LIKE ?1 ORDER BY item_name ASC LIMIT ?2",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let items = stmt
            .query_map(params![search_pattern, limit as i64], |row| Self::row_to_item(row))
            .map_err(|e| Error::DatabaseError(format!("Search failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(items)
    }

    fn row_to_item(row: &Row) -> rusqlite::Result<InventoryItem> {
        let qty_str: String = row.get(3)?;
        let quantity = Decimal::from_str(&qty_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                3,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        let cost_str: Option<String> = row.get(8)?;
        let cost = cost_str
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                8,
                rusqlite::types::Type::Text,
                Box::new(e),
            ))?;

        Ok(InventoryItem {
            id: Some(row.get(0)?),
            item_type: row.get(1)?,
            item_name: row.get(2)?,
            quantity,
            unit: row.get(4)?,
            location: row.get(5)?,
            purchase_date: row.get(6)?,
            expiration_date: row.get(7)?,
            cost,
            notes: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }
}