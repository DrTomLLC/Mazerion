use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

const MAX_NAME_LEN: usize = 200;
const MAX_CATEGORY_LEN: usize = 50;
const MAX_DESCRIPTION_LEN: usize = 5000;
const MAX_AUTHOR_LEN: usize = 200;
const MAX_SOURCE_LEN: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub batch_size_l: Decimal,
    pub target_og: Option<Decimal>,
    pub target_fg: Option<Decimal>,
    pub target_abv: Option<Decimal>,
    pub created_at: String,
    pub updated_at: String,
}

impl Recipe {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > MAX_NAME_LEN {
            return Err(format!("Name exceeds {} characters", MAX_NAME_LEN));
        }

        if self.category.is_empty() {
            return Err("Category cannot be empty".to_string());
        }
        if self.category.len() > MAX_CATEGORY_LEN {
            return Err(format!("Category exceeds {} characters", MAX_CATEGORY_LEN));
        }

        if let Some(ref sub) = self.subcategory {
            if sub.len() > MAX_CATEGORY_LEN {
                return Err(format!("Subcategory exceeds {} characters", MAX_CATEGORY_LEN));
            }
        }

        if let Some(ref desc) = self.description {
            if desc.len() > MAX_DESCRIPTION_LEN {
                return Err(format!("Description exceeds {} characters", MAX_DESCRIPTION_LEN));
            }
        }

        if let Some(ref auth) = self.author {
            if auth.len() > MAX_AUTHOR_LEN {
                return Err(format!("Author exceeds {} characters", MAX_AUTHOR_LEN));
            }
        }

        if let Some(ref src) = self.source {
            if src.len() > MAX_SOURCE_LEN {
                return Err(format!("Source exceeds {} characters", MAX_SOURCE_LEN));
            }
        }

        if self.batch_size_l <= Decimal::ZERO {
            return Err("Batch size must be positive".to_string());
        }
        if self.batch_size_l > Decimal::from(10000) {
            return Err("Batch size exceeds maximum (10000L)".to_string());
        }

        if let Some(og) = self.target_og {
            if og < Decimal::new(960, 3) || og > Decimal::new(1200, 3) {
                return Err("OG must be between 0.960 and 1.200".to_string());
            }
        }
        if let Some(fg) = self.target_fg {
            if fg < Decimal::new(960, 3) || fg > Decimal::new(1050, 3) {
                return Err("FG must be between 0.960 and 1.050".to_string());
            }
        }
        if let Some(abv) = self.target_abv {
            if abv < Decimal::ZERO || abv > Decimal::from(25) {
                return Err("ABV must be between 0% and 25%".to_string());
            }
        }

        Ok(())
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - {}L",
            self.name,
            self.category,
            self.batch_size_l
        )
    }
}