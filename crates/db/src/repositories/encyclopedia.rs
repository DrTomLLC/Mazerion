// Encyclopedia repository - read-only access to yeast, hops, etc

use rusqlite::{params, Connection, OptionalExtension, Row};
use rust_decimal::Decimal;
use mazerion_core::{Error, Result};
use crate::models::{YeastStrain, HopVariety};
use std::str::FromStr;

pub struct EncyclopediaRepository<'a> {
    conn: &'a Connection,
}

impl<'a> EncyclopediaRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ══════════════════════════════════════════════════════════════════════
    // YEAST OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn get_yeast(&self, id: i64) -> Result<Option<YeastStrain>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, laboratory, attenuation_min, attenuation_max,
                 temp_min_c, temp_max_c, alcohol_tolerance, flocculation, description
                 FROM yeast_strains WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_yeast(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?;

        Ok(result)
    }

    pub fn list_yeasts(&self, limit: usize) -> Result<Vec<YeastStrain>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, laboratory, attenuation_min, attenuation_max,
                 temp_min_c, temp_max_c, alcohol_tolerance, flocculation, description
                 FROM yeast_strains ORDER BY name ASC LIMIT ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let yeasts = stmt
            .query_map([limit as i64], |row| Self::row_to_yeast(row))
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(yeasts)
    }

    pub fn search_yeasts(&self, query: &str, limit: usize) -> Result<Vec<YeastStrain>> {
        if query.len() > 200 {
            return Err(Error::Validation("Query too long".into()));
        }

        let search_pattern = format!("%{}%", query);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, laboratory, attenuation_min, attenuation_max,
                 temp_min_c, temp_max_c, alcohol_tolerance, flocculation, description
                 FROM yeast_strains WHERE name LIKE ?1 OR laboratory LIKE ?1
                 ORDER BY name ASC LIMIT ?2",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let yeasts = stmt
            .query_map(params![search_pattern, limit as i64], |row| Self::row_to_yeast(row))
            .map_err(|e| Error::DatabaseError(format!("Search failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(yeasts)
    }

    // ══════════════════════════════════════════════════════════════════════
    // HOP OPERATIONS
    // ══════════════════════════════════════════════════════════════════════

    pub fn get_hop(&self, id: i64) -> Result<Option<HopVariety>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, alpha_acid_min, alpha_acid_max, beta_acid_min, beta_acid_max,
                 cohumulone, aroma_profile, description
                 FROM hop_varieties WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_hop(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?;

        Ok(result)
    }

    pub fn list_hops(&self, limit: usize) -> Result<Vec<HopVariety>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, alpha_acid_min, alpha_acid_max, beta_acid_min, beta_acid_max,
                 cohumulone, aroma_profile, description
                 FROM hop_varieties ORDER BY name ASC LIMIT ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let hops = stmt
            .query_map([limit as i64], |row| Self::row_to_hop(row))
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(hops)
    }

    pub fn search_hops(&self, query: &str, limit: usize) -> Result<Vec<HopVariety>> {
        if query.len() > 200 {
            return Err(Error::Validation("Query too long".into()));
        }

        let search_pattern = format!("%{}%", query);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, alpha_acid_min, alpha_acid_max, beta_acid_min, beta_acid_max,
                 cohumulone, aroma_profile, description
                 FROM hop_varieties WHERE name LIKE ?1 OR aroma_profile LIKE ?1
                 ORDER BY name ASC LIMIT ?2",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let hops = stmt
            .query_map(params![search_pattern, limit as i64], |row| Self::row_to_hop(row))
            .map_err(|e| Error::DatabaseError(format!("Search failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(hops)
    }

    // ══════════════════════════════════════════════════════════════════════
    // HELPER FUNCTIONS
    // ══════════════════════════════════════════════════════════════════════

    fn row_to_yeast(row: &Row) -> rusqlite::Result<YeastStrain> {
        let atten_min: Option<String> = row.get(3)?;
        let atten_max: Option<String> = row.get(4)?;
        let temp_min: Option<String> = row.get(5)?;
        let temp_max: Option<String> = row.get(6)?;
        let alcohol_tol: Option<String> = row.get(7)?;

        Ok(YeastStrain {
            id: row.get(0)?,
            name: row.get(1)?,
            laboratory: row.get(2)?,
            attenuation_min: atten_min.and_then(|s| Decimal::from_str(&s).ok()),
            attenuation_max: atten_max.and_then(|s| Decimal::from_str(&s).ok()),
            temp_min_c: temp_min.and_then(|s| Decimal::from_str(&s).ok()),
            temp_max_c: temp_max.and_then(|s| Decimal::from_str(&s).ok()),
            alcohol_tolerance: alcohol_tol.and_then(|s| Decimal::from_str(&s).ok()),
            flocculation: row.get(8)?,
            description: row.get(9)?,
        })
    }

    fn row_to_hop(row: &Row) -> rusqlite::Result<HopVariety> {
        let alpha_min: Option<String> = row.get(2)?;
        let alpha_max: Option<String> = row.get(3)?;
        let beta_min: Option<String> = row.get(4)?;
        let beta_max: Option<String> = row.get(5)?;
        let cohum: Option<String> = row.get(6)?;

        Ok(HopVariety {
            id: row.get(0)?,
            name: row.get(1)?,
            alpha_acid_min: alpha_min.and_then(|s| Decimal::from_str(&s).ok()),
            alpha_acid_max: alpha_max.and_then(|s| Decimal::from_str(&s).ok()),
            beta_acid_min: beta_min.and_then(|s| Decimal::from_str(&s).ok()),
            beta_acid_max: beta_max.and_then(|s| Decimal::from_str(&s).ok()),
            cohumulone: cohum.and_then(|s| Decimal::from_str(&s).ok()),
            aroma_profile: row.get(7)?,
            description: row.get(8)?,
        })
    }
}