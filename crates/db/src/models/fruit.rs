use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fruit {
    pub id: i64,
    pub name: String,
    pub scientific_name: Option<String>,
    pub fruit_type: String,
    pub origin: Option<String>,
    pub typical_sugar_content: Option<Decimal>,
    pub ph_level: Option<Decimal>,
    pub color_contribution: Option<String>,
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

impl Fruit {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Fruit name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Fruit name too long (max 100 characters)".to_string());
        }

        let valid_types = ["berry", "stone fruit", "citrus", "pome", "tropical", "melon", "other"];
        if !valid_types.contains(&self.fruit_type.as_str()) {
            return Err(format!("Invalid fruit type '{}', must be one of: {}",
                               self.fruit_type, valid_types.join(", ")));
        }

        if let Some(sugar) = self.typical_sugar_content {
            if sugar < Decimal::ZERO || sugar > Decimal::from(50) {
                return Err("Sugar content must be between 0% and 50%".to_string());
            }
        }

        if let Some(ph) = self.ph_level {
            if ph < Decimal::from(2) || ph > Decimal::from(7) {
                return Err("pH must be between 2.0 and 7.0".to_string());
            }
        }

        if let Some(ppg) = self.pounds_per_gallon {
            if ppg < Decimal::ZERO || ppg > Decimal::from(20) {
                return Err("Pounds per gallon must be between 0 and 20".to_string());
            }
        }

        Ok(())
    }
}