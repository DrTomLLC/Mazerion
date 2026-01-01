use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Syrup {
    pub id: i64,
    pub name: String,
    pub syrup_type: String,
    pub manufacturer: Option<String>,
    pub sugar_content: Option<Decimal>,
    pub flavor_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub typical_dosage_oz_per_gallon: Option<Decimal>,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Syrup {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() { return Err("Name cannot be empty".to_string()); }
        if self.name.len() > 100 { return Err("Name too long".to_string()); }
        let valid = ["maple", "fruit", "flavored", "invert", "other"];
        if !valid.contains(&self.syrup_type.as_str()) {
            return Err(format!("Invalid type '{}'", self.syrup_type));
        }
        if let Some(sugar) = self.sugar_content {
            if sugar < Decimal::ZERO || sugar > Decimal::from(100) {
                return Err("Sugar must be 0-100%".to_string());
            }
        }
        Ok(())
    }
}