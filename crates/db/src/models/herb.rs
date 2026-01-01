use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Herb {
    pub id: i64,
    pub name: String,
    pub scientific_name: Option<String>,
    pub herb_type: String,
    pub origin: Option<String>,
    pub flavor_profile: Option<String>,
    pub aroma_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub sensory_notes: Option<String>,
    pub typical_dosage_oz_per_gallon: Option<Decimal>,
    pub preparation_method: Option<String>,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Herb {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Name too long (max 100 characters)".to_string());
        }

        let valid_types = ["culinary", "medicinal", "aromatic", "other"];
        if !valid_types.contains(&self.herb_type.as_str()) {
            return Err(format!("Invalid type '{}'", self.herb_type));
        }

        if let Some(dosage) = self.typical_dosage_oz_per_gallon {
            if dosage < Decimal::ZERO || dosage > Decimal::from(10) {
                return Err("Dosage must be between 0 and 10 oz/gal".to_string());
            }
        }

        Ok(())
    }
}