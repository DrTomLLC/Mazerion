use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrient {
    pub id: i64,
    pub name: String,
    pub nutrient_type: String,
    pub manufacturer: Option<String>,

    // Nutrient composition (TEXT for Decimal precision)
    pub nitrogen_content: Option<Decimal>,
    pub phosphorus_content: Option<Decimal>,
    pub potassium_content: Option<Decimal>,
    pub typical_dosage_grams_per_gallon: Option<Decimal>,

    // Usage guidance
    pub usage_notes: Option<String>,
    pub best_suited_styles: Option<String>,
    pub compatible_styles: Option<String>,
    pub timing: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl Nutrient {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["DAP", "fermaid", "yeast_hulls", "urea", "complete", "other"];
        if !valid_types.contains(&self.nutrient_type.as_str()) {
            return Err(format!("Invalid nutrient type '{}', must be one of: {}",
                               self.nutrient_type, valid_types.join(", ")));
        }

        if let Some(n) = self.nitrogen_content {
            if n < Decimal::ZERO || n > Decimal::from(100) {
                return Err("Nitrogen content must be between 0 and 100%".to_string());
            }
        }

        if let Some(p) = self.phosphorus_content {
            if p < Decimal::ZERO || p > Decimal::from(100) {
                return Err("Phosphorus content must be between 0 and 100%".to_string());
            }
        }

        if let Some(k) = self.potassium_content {
            if k < Decimal::ZERO || k > Decimal::from(100) {
                return Err("Potassium content must be between 0 and 100%".to_string());
            }
        }

        if let Some(dose) = self.typical_dosage_grams_per_gallon {
            if dose < Decimal::ZERO || dose > Decimal::from(10) {
                return Err("Typical dosage must be between 0 and 10 g/gal".to_string());
            }
        }

        Ok(())
    }
}