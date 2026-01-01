use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bacteria {
    pub id: i64,
    pub name: String,
    pub bacteria_type: String,
    pub laboratory: Option<String>,
    pub product_code: Option<String>,

    // Fermentation characteristics (TEXT for Decimal precision)
    pub optimal_temperature_min: Option<Decimal>,
    pub optimal_temperature_max: Option<Decimal>,
    pub optimal_ph_min: Option<Decimal>,
    pub optimal_ph_max: Option<Decimal>,
    pub typical_dosage_grams_per_gallon: Option<Decimal>,

    // Professional guidance
    pub usage_notes: Option<String>,
    pub flavor_profile: Option<String>,
    pub best_suited_styles: Option<String>,
    pub compatible_styles: Option<String>,
    pub timing: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl Bacteria {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["lactobacillus", "pediococcus", "acetobacter", "other"];
        if !valid_types.contains(&self.bacteria_type.as_str()) {
            return Err(format!("Invalid bacteria type '{}', must be one of: {}",
                               self.bacteria_type, valid_types.join(", ")));
        }

        if let Some(temp_min) = self.optimal_temperature_min {
            if temp_min < Decimal::from(32) || temp_min > Decimal::from(212) {
                return Err("Optimal temperature min must be between 32°F and 212°F".to_string());
            }
        }

        if let Some(temp_max) = self.optimal_temperature_max {
            if temp_max < Decimal::from(32) || temp_max > Decimal::from(212) {
                return Err("Optimal temperature max must be between 32°F and 212°F".to_string());
            }
        }

        if let Some(ph_min) = self.optimal_ph_min {
            if ph_min < Decimal::from(1) || ph_min > Decimal::from(14) {
                return Err("Optimal pH min must be between 1.0 and 14.0".to_string());
            }
        }

        if let Some(ph_max) = self.optimal_ph_max {
            if ph_max < Decimal::from(1) || ph_max > Decimal::from(14) {
                return Err("Optimal pH max must be between 1.0 and 14.0".to_string());
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