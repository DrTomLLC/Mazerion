// Recipe repository - handles both master pack recipes and user recipes

use rusqlite::{params, Connection, OptionalExtension, Row};
use rust_decimal::Decimal;
use mazerion_core::{Error, Result};
use crate::models::Recipe;
use std::str::FromStr;

pub struct RecipeRepository<'a> {
    conn: &'a Connection,
}

impl<'a> RecipeRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ══════════════════════════════════════════════════════════════════════
    // USER RECIPES (writable in user.db)
    // ══════════════════════════════════════════════════════════════════════

    pub fn create_user_recipe(&self, recipe: &Recipe) -> Result<i64> {
        recipe.validate()?;

        self.conn
            .execute(
                "INSERT INTO user_recipes (name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    &recipe.name,
                    &recipe.category,
                    &recipe.subcategory,
                    &recipe.description,
                    recipe.batch_size_l.to_string(),
                    recipe.target_og.as_ref().map(|v| v.to_string()),
                    recipe.target_fg.as_ref().map(|v| v.to_string()),
                    recipe.target_abv.as_ref().map(|v| v.to_string()),
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Insert failed: {}", e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_user_recipe(&self, id: i64) -> Result<Option<Recipe>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_user_recipe(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?;

        Ok(result)
    }

    pub fn list_user_recipes(&self, category: Option<&str>, limit: usize) -> Result<Vec<Recipe>> {
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(cat) = category {
            (
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes WHERE category = ?1 ORDER BY name ASC LIMIT ?2".to_string(),
                vec![Box::new(cat.to_string()), Box::new(limit as i64)],
            )
        } else {
            (
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes ORDER BY name ASC LIMIT ?1".to_string(),
                vec![Box::new(limit as i64)],
            )
        };

        let mut stmt = self
            .conn
            .prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let recipes = stmt
            .query_map(params_refs.as_slice(), |row| Self::row_to_user_recipe(row))
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(recipes)
    }

    pub fn update_user_recipe(&self, recipe: &Recipe) -> Result<()> {
        let id = recipe.id.ok_or_else(|| Error::Validation("Recipe ID required".into()))?;
        recipe.validate()?;

        let affected = self
            .conn
            .execute(
                "UPDATE user_recipes SET name = ?1, category = ?2, subcategory = ?3,
                 description = ?4, batch_size_l = ?5, target_og = ?6, target_fg = ?7,
                 target_abv = ?8, updated_at = CURRENT_TIMESTAMP WHERE id = ?9",
                params![
                    &recipe.name,
                    &recipe.category,
                    &recipe.subcategory,
                    &recipe.description,
                    recipe.batch_size_l.to_string(),
                    recipe.target_og.as_ref().map(|v| v.to_string()),
                    recipe.target_fg.as_ref().map(|v| v.to_string()),
                    recipe.target_abv.as_ref().map(|v| v.to_string()),
                    id,
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Update failed: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Recipe {} not found", id)));
        }

        Ok(())
    }

    pub fn delete_user_recipe(&self, id: i64) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM user_recipes WHERE id = ?1", params![id])
            .map_err(|e| Error::DatabaseError(format!("Delete failed: {}", e)))?;

        if affected == 0 {
            return Err(Error::Validation(format!("Recipe {} not found", id)));
        }

        Ok(())
    }

    pub fn search_user_recipes(&self, query: &str, limit: usize) -> Result<Vec<Recipe>> {
        if query.len() > 200 {
            return Err(Error::Validation("Query too long".into()));
        }

        let search_pattern = format!("%{}%", query);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes WHERE name LIKE ?1 OR description LIKE ?1
                 ORDER BY name ASC LIMIT ?2",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let recipes = stmt
            .query_map(params![search_pattern, limit as i64], |row| {
                Self::row_to_user_recipe(row)
            })
            .map_err(|e| Error::DatabaseError(format!("Search failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(recipes)
    }

    // ══════════════════════════════════════════════════════════════════════
    // MASTER RECIPES (read-only from packs)
    // ══════════════════════════════════════════════════════════════════════

    pub fn get_master_recipe(&self, id: i64) -> Result<Option<Recipe>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, category, subcategory, description, author, source, difficulty,
                 batch_size_l, target_og, target_fg, target_abv
                 FROM recipes WHERE id = ?1",
            )
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_master_recipe(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?;

        Ok(result)
    }

    pub fn list_master_recipes(&self, category: Option<&str>, limit: usize) -> Result<Vec<Recipe>> {
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(cat) = category {
            (
                "SELECT id, name, category, subcategory, description, author, source, difficulty,
                 batch_size_l, target_og, target_fg, target_abv
                 FROM recipes WHERE category = ?1 ORDER BY name ASC LIMIT ?2".to_string(),
                vec![Box::new(cat.to_string()), Box::new(limit as i64)],
            )
        } else {
            (
                "SELECT id, name, category, subcategory, description, author, source, difficulty,
                 batch_size_l, target_og, target_fg, target_abv
                 FROM recipes ORDER BY name ASC LIMIT ?1".to_string(),
                vec![Box::new(limit as i64)],
            )
        };

        let mut stmt = self
            .conn
            .prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare failed: {}", e)))?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let recipes = stmt
            .query_map(params_refs.as_slice(), |row| Self::row_to_master_recipe(row))
            .map_err(|e| Error::DatabaseError(format!("Query failed: {}", e)))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Parse failed: {}", e)))?;

        Ok(recipes)
    }

    // ══════════════════════════════════════════════════════════════════════
    // HELPER FUNCTIONS
    // ══════════════════════════════════════════════════════════════════════

    fn row_to_user_recipe(row: &Row) -> rusqlite::Result<Recipe> {
        let batch_size_str: String = row.get(5)?;
        let batch_size_l = Decimal::from_str(&batch_size_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e))
        })?;

        let target_og: Option<String> = row.get(6)?;
        let target_og = target_og
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let target_fg: Option<String> = row.get(7)?;
        let target_fg = target_fg
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    7,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let target_abv: Option<String> = row.get(8)?;
        let target_abv = target_abv
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    8,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        Ok(Recipe {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            category: row.get(2)?,
            subcategory: row.get(3)?,
            description: row.get(4)?,
            author: None,
            source: None,
            difficulty: None,
            batch_size_l,
            target_og,
            target_fg,
            target_abv,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }

    fn row_to_master_recipe(row: &Row) -> rusqlite::Result<Recipe> {
        let batch_size_str: String = row.get(8)?;
        let batch_size_l = Decimal::from_str(&batch_size_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e))
        })?;

        let target_og: Option<String> = row.get(9)?;
        let target_og = target_og
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    9,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let target_fg: Option<String> = row.get(10)?;
        let target_fg = target_fg
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    10,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let target_abv: Option<String> = row.get(11)?;
        let target_abv = target_abv
            .map(|s| Decimal::from_str(&s))
            .transpose()
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    11,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        Ok(Recipe {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            category: row.get(2)?,
            subcategory: row.get(3)?,
            description: row.get(4)?,
            author: row.get(5)?,
            source: row.get(6)?,
            difficulty: row.get(7)?,
            batch_size_l,
            target_og,
            target_fg,
            target_abv,
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}