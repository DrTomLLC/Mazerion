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
        item.validate()
            .map_err(|e| Error::Validation(e))?;

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
            .map_err(|e| Error::DatabaseError(format!("Insert inventory: {}", e)))?;

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
            .map_err(|e| Error::DatabaseError(format!("Prepare query: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_item(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Get inventory: {}", e)))?;

        Ok(result)
    }

    pub fn list(&self, item_type: Option<&str>, limit: usize) -> Result<Vec<InventoryItem>> {
        let capped_limit = limit.min(1000);
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(itype) = item_type {
            (
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory WHERE item_type = ?1 ORDER BY item_name ASC LIMIT ?2".to_string(),
                vec![Box::new(itype.to_string()), Box::new(capped_limit as i64)],
            )
        } else {
            (
                "SELECT id, item_type, item_name, quantity, unit, location,
                 purchase_date, expiration_date, cost, notes, created_at, updated_at
                 FROM inventory ORDER BY item_name ASC LIMIT ?1".to_string(),
                vec![Box::new(capped_limit as i64)],
            )
        };

        let mut stmt = self.conn.prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare list: {}", e)))?;

        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&params_ref[..], |row| Self::row_to_item(row))
            .map_err(|e| Error::DatabaseError(format!("Execute list: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<InventoryItem>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let capped_limit = limit.min(1000);

        let mut stmt = self.conn.prepare(
            "SELECT id, item_type, item_name, quantity, unit, location,
             purchase_date, expiration_date, cost, notes, created_at, updated_at
             FROM inventory
             WHERE item_name LIKE ?1 OR item_type LIKE ?1 OR location LIKE ?1
             ORDER BY item_name ASC
             LIMIT ?2"
        ).map_err(|e| Error::DatabaseError(format!("Prepare search: {}", e)))?;

        let rows = stmt.query_map(
            params![search_pattern, capped_limit as i64],
            |row| Self::row_to_item(row)
        ).map_err(|e| Error::DatabaseError(format!("Execute search: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn update_quantity(&self, id: i64, quantity: Decimal) -> Result<()> {
        if quantity < Decimal::ZERO {
            return Err(Error::Validation("Quantity cannot be negative".to_string()));
        }

        let updated_at = chrono::Utc::now().to_rfc3339();

        let rows_affected = self.conn.execute(
            "UPDATE inventory SET quantity = ?1, updated_at = ?2 WHERE id = ?3",
            params![quantity.to_string(), updated_at, id]
        ).map_err(|e| Error::DatabaseError(format!("Update quantity: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Inventory item {} not found", id)));
        }

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM inventory WHERE id = ?1",
            params![id]
        ).map_err(|e| Error::DatabaseError(format!("Delete inventory: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Inventory item {} not found", id)));
        }

        Ok(())
    }

    pub fn count(&self) -> Result<u32> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM inventory",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count inventory: {}", e)))?;

        Ok(count as u32)
    }

    fn row_to_item(row: &Row) -> rusqlite::Result<InventoryItem> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(InventoryItem {
            id: row.get(0)?,
            item_type: row.get(1)?,
            item_name: row.get(2)?,
            quantity: parse_decimal(Some(row.get::<_, String>(3)?)).unwrap(),
            unit: row.get(4)?,
            location: row.get(5)?,
            purchase_date: row.get(6)?,
            expiration_date: row.get(7)?,
            cost: parse_decimal(row.get(8)?),
            notes: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }
}