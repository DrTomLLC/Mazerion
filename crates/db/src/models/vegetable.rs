use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vegetable {
    pub id: i64,
    pub name: String,
    pub scientific_name: Option<String>,
    pub vegetable_type: String,
    pub origin: Option<String>,
    pub typical_sugar_content: Option<Decimal>,
    pub ph_level: Option<Decimal>,
    pub flavor_profile: Option<String>,
    pub aroma_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub sensory_notes: Option<String>,
    pub pounds_per_gallon: Option<Decimal>,
    pub preparation_method: Option<String>,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Vegetable {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Name too long (max 100 characters)".to_string());
        }

        let valid_types = ["root", "gourd", "pepper", "leafy", "other"];
        if !valid_types.contains(&self.vegetable_type.as_str()) {
            return Err(format!("Invalid type '{}', must be one of: {}",
                               self.vegetable_type, valid_types.join(", ")));
        }

        if let Some(sugar) = self.typical_sugar_content {
            if sugar < Decimal::ZERO || sugar > Decimal::from(50) {
                return Err("Sugar content must be between 0% and 50%".to_string());
            }
        }

        if let Some(ph) = self.ph_level {
            if ph < Decimal::from(2) || ph > Decimal::from(8) {
                return Err("pH must be between 2.0 and 8.0".to_string());
            }
        }

        Ok(())
    }
}