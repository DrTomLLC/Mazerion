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

    pub fn create_user_recipe(&self, recipe: &Recipe) -> Result<i64> {
        recipe.validate()
            .map_err(|e| Error::Validation(e))?;

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
            .map_err(|e| Error::DatabaseError(format!("Insert recipe: {}", e)))?;

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
            .map_err(|e| Error::DatabaseError(format!("Prepare query: {}", e)))?;

        let result = stmt
            .query_row([id], |row| Self::row_to_user_recipe(row))
            .optional()
            .map_err(|e| Error::DatabaseError(format!("Get recipe: {}", e)))?;

        Ok(result)
    }

    pub fn list_user_recipes(&self, category: Option<&str>, limit: usize) -> Result<Vec<Recipe>> {
        let capped_limit = limit.min(1000);
        let (query, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(cat) = category {
            (
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes WHERE category = ?1 ORDER BY name ASC LIMIT ?2".to_string(),
                vec![Box::new(cat.to_string()), Box::new(capped_limit as i64)],
            )
        } else {
            (
                "SELECT id, name, category, subcategory, description,
                 batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
                 FROM user_recipes ORDER BY name ASC LIMIT ?1".to_string(),
                vec![Box::new(capped_limit as i64)],
            )
        };

        let mut stmt = self.conn.prepare(&query)
            .map_err(|e| Error::DatabaseError(format!("Prepare list: {}", e)))?;

        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&params_ref[..], |row| Self::row_to_user_recipe(row))
            .map_err(|e| Error::DatabaseError(format!("Execute list: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn update_user_recipe(&self, recipe: &Recipe) -> Result<()> {
        recipe.validate()
            .map_err(|e| Error::Validation(e))?;

        let rows_affected = self.conn.execute(
            "UPDATE user_recipes
             SET name = ?1, category = ?2, subcategory = ?3, description = ?4,
                 batch_size_l = ?5, target_og = ?6, target_fg = ?7, target_abv = ?8,
                 updated_at = ?9
             WHERE id = ?10",
            params![
                &recipe.name,
                &recipe.category,
                &recipe.subcategory,
                &recipe.description,
                recipe.batch_size_l.to_string(),
                recipe.target_og.as_ref().map(|v| v.to_string()),
                recipe.target_fg.as_ref().map(|v| v.to_string()),
                recipe.target_abv.as_ref().map(|v| v.to_string()),
                chrono::Utc::now().to_rfc3339(),
                recipe.id,
            ]
        ).map_err(|e| Error::DatabaseError(format!("Update recipe: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Recipe {} not found", recipe.id)));
        }

        Ok(())
    }

    pub fn delete_user_recipe(&self, id: i64) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM user_recipes WHERE id = ?1",
            params![id]
        ).map_err(|e| Error::DatabaseError(format!("Delete recipe: {}", e)))?;

        if rows_affected == 0 {
            return Err(Error::DatabaseError(format!("Recipe {} not found", id)));
        }

        Ok(())
    }

    pub fn search_user_recipes(&self, query: &str, limit: usize) -> Result<Vec<Recipe>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", query.trim());
        let capped_limit = limit.min(1000);

        let mut stmt = self.conn.prepare(
            "SELECT id, name, category, subcategory, description,
             batch_size_l, target_og, target_fg, target_abv, created_at, updated_at
             FROM user_recipes
             WHERE name LIKE ?1 OR description LIKE ?1 OR category LIKE ?1
             ORDER BY name ASC
             LIMIT ?2"
        ).map_err(|e| Error::DatabaseError(format!("Prepare search: {}", e)))?;

        let rows = stmt.query_map(
            params![search_pattern, capped_limit as i64],
            |row| Self::row_to_user_recipe(row)
        ).map_err(|e| Error::DatabaseError(format!("Execute search: {}", e)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result.map_err(|e| Error::DatabaseError(format!("Row error: {}", e)))?);
        }
        Ok(results)
    }

    pub fn count_user_recipes(&self) -> Result<u32> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM user_recipes",
            [],
            |row| row.get(0)
        ).map_err(|e| Error::DatabaseError(format!("Count recipes: {}", e)))?;

        Ok(count as u32)
    }

    fn row_to_user_recipe(row: &Row) -> rusqlite::Result<Recipe> {
        fn parse_decimal(s: Option<String>) -> Option<Decimal> {
            s.and_then(|s| Decimal::from_str(&s).ok())
        }

        Ok(Recipe {
            id: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            subcategory: row.get(3)?,
            description: row.get(4)?,
            author: None,
            source: None,
            batch_size_l: parse_decimal(Some(row.get::<_, String>(5)?)).unwrap(),
            target_og: parse_decimal(row.get(6)?),
            target_fg: parse_decimal(row.get(7)?),
            target_abv: parse_decimal(row.get(8)?),
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }
}