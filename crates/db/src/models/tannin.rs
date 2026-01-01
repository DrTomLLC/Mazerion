use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tannin {
    pub id: i64,
    pub name: String,
    pub tannin_type: String,
    pub source: Option<String>,
    pub manufacturer: Option<String>,

    // Tannin characteristics (TEXT for Decimal precision)
    pub concentration: Option<Decimal>,
    pub typical_dosage_grams_per_gallon: Option<Decimal>,

    // Professional guidance
    pub usage_notes: Option<String>,
    pub flavor_impact: Option<String>,
    pub best_suited_styles: Option<String>,
    pub compatible_styles: Option<String>,
    pub timing: Option<String>,
    pub purpose: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl Tannin {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["grape", "oak", "chestnut", "quebracho", "gallotannin", "other"];
        if !valid_types.contains(&self.tannin_type.as_str()) {
            return Err(format!("Invalid tannin type '{}', must be one of: {}",
                               self.tannin_type, valid_types.join(", ")));
        }

        if let Some(conc) = self.concentration {
            if conc < Decimal::ZERO || conc > Decimal::from(100) {
                return Err("Concentration must be between 0 and 100%".to_string());
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