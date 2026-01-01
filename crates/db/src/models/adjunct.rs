use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adjunct {
    pub id: i64,
    pub name: String,
    pub adjunct_type: String,
    pub manufacturer: Option<String>,
    pub fermentability: Option<Decimal>,
    pub flavor_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub typical_percentage: Option<Decimal>,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Adjunct {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() { return Err("Name cannot be empty".to_string()); }
        if self.name.len() > 100 { return Err("Name too long".to_string()); }
        let valid = ["sugar", "honey", "grain", "other"];
        if !valid.contains(&self.adjunct_type.as_str()) {
            return Err(format!("Invalid type '{}'", self.adjunct_type));
        }
        if let Some(ferm) = self.fermentability {
            if ferm < Decimal::ZERO || ferm > Decimal::from(100) {
                return Err("Fermentability must be 0-100%".to_string());
            }
        }
        Ok(())
    }
}