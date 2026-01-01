use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extract {
    pub id: i64,
    pub name: String,
    pub extract_type: String,
    pub manufacturer: Option<String>,
    pub flavor_profile: Option<String>,
    pub aroma_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub typical_dosage_oz_per_gallon: Option<Decimal>,
    pub alcohol_based: bool,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Extract {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() { return Err("Name cannot be empty".to_string()); }
        if self.name.len() > 100 { return Err("Name too long".to_string()); }
        let valid = ["vanilla", "almond", "fruit", "coffee", "chocolate", "other"];
        if !valid.contains(&self.extract_type.as_str()) {
            return Err(format!("Invalid type '{}'", self.extract_type));
        }
        Ok(())
    }
}