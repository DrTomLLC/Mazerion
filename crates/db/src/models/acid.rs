use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acid {
    pub id: i64,
    pub name: String,
    pub acid_type: String,
    pub chemical_formula: Option<String>,
    pub manufacturer: Option<String>,

    // Acid characteristics (TEXT for Decimal precision)
    pub concentration: Option<Decimal>,
    pub ph_adjustment_per_ml: Option<Decimal>,
    pub typical_dosage_ml_per_gallon: Option<Decimal>,

    // Professional guidance
    pub usage_notes: Option<String>,
    pub flavor_impact: Option<String>,
    pub best_suited_styles: Option<String>,
    pub compatible_styles: Option<String>,
    pub safety_notes: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl Acid {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["lactic", "phosphoric", "citric", "tartaric", "malic", "other"];
        if !valid_types.contains(&self.acid_type.as_str()) {
            return Err(format!("Invalid acid type '{}', must be one of: {}",
                               self.acid_type, valid_types.join(", ")));
        }

        if let Some(conc) = self.concentration {
            if conc < Decimal::ZERO || conc > Decimal::from(100) {
                return Err("Concentration must be between 0 and 100%".to_string());
            }
        }

        if let Some(ph_adj) = self.ph_adjustment_per_ml {
            if ph_adj < Decimal::ZERO || ph_adj > Decimal::from(2) {
                return Err("pH adjustment must be between 0 and 2.0 per ml".to_string());
            }
        }

        if let Some(dose) = self.typical_dosage_ml_per_gallon {
            if dose < Decimal::ZERO || dose > Decimal::from(20) {
                return Err("Typical dosage must be between 0 and 20 ml/gal".to_string());
            }
        }

        Ok(())
    }
}